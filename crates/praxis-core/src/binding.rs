//! Binding work to a version. `TS.260820.16`: attach finished iterations to a version, and
//! refuse anything that did not close.
//!
//! What a version contains becomes a fact about the record rather than a list somebody
//! maintains alongside it — which is `S5` at release granularity, and the same argument as
//! every other rule here: a hand-kept list and a derived one look identical right up until
//! they disagree.
//!
//! The release record is **machine-owned**. Every field in it is derived from what was
//! bound, so it is composed whole each time rather than edited — unlike an iteration,
//! which is a human's record that a command may only add to.

use crate::admission::Corpus;
use crate::pickup::{Ask, Record};

/// What an ask to bind produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Binding {
    /// Bound. The release record, composed whole.
    Bound { release: Record, proposal: Proposal },
    /// Refused, naming what failed. Nothing binds.
    Refused(Record),
    /// Nothing to bind, or nowhere to bind it.
    NotFound(String),
}

/// The bump the bound work implies, and what the record could not decide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    /// The position to bump — from the configured rules, never from an assumption about
    /// what semver means to somebody else's project.
    pub position: String,
    /// Which rule decided it, and which iteration matched that rule.
    pub because: String,
    /// Iterations that declared no `contributes`, so they informed nothing. Named rather
    /// than skipped: a proposal computed from half the work is not a proposal.
    pub silent: Vec<String>,
}

/// Why a bind was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rejected {
    /// The iteration has not closed. C1.
    NotClosed { iteration: String, state: String, unsettled: Vec<String> },
    /// Already bound somewhere. C2.
    AlreadyBound { iteration: String, version: String },
    /// The release has been cut, so its content is no longer a question. C4.
    AlreadyCut { version: String },
}

impl Rejected {
    pub fn message(&self) -> String {
        match self {
            Self::NotClosed { iteration, state, unsettled } => {
                let claims = if unsettled.is_empty() {
                    String::new()
                } else {
                    format!(" — unsettled: {}", unsettled.join(", "))
                };
                format!(
                    "{iteration} is {state:?} and only a closed iteration may bind{claims}. A \
                     version whose content includes work still in flight has no definite content"
                )
            }
            Self::AlreadyBound { iteration, version } => format!(
                "{iteration} is already bound to {version} — an iteration binds exactly once, or \
                 the same work is counted in two releases"
            ),
            Self::AlreadyCut { version } => format!(
                "{version} is cut. Binding is reversible until the release is cut; cutting is not"
            ),
        }
    }
}

/// Bind one closed iteration to a version.
pub fn bind(
    iteration_id: &str,
    version: &str,
    corpus: &Corpus,
    ask: &Ask,
    taken: &[String],
) -> Binding {
    let Some(attempt) = corpus.attempts.iter().find(|a| a.id == iteration_id) else {
        return Binding::NotFound(format!("{iteration_id} is no iteration the record holds"));
    };

    let existing = corpus.release(version);
    if let Some(release) = existing
        && release.cut()
    {
        return refuse(&[Rejected::AlreadyCut { version: version.to_owned() }], ask, taken, version);
    }

    if attempt.state != "closed" {
        let unsettled: Vec<String> = attempt
            .claims
            .iter()
            .filter(|c| c.state != "met")
            .map(|c| c.id.clone())
            .collect();
        return refuse(
            &[Rejected::NotClosed {
                iteration: iteration_id.to_owned(),
                state: attempt.state.clone(),
                unsettled,
            }],
            ask,
            taken,
            version,
        );
    }

    if let Some(bound) = corpus.bound_to(iteration_id) {
        return refuse(
            &[Rejected::AlreadyBound {
                iteration: iteration_id.to_owned(),
                version: bound.version.clone(),
            }],
            ask,
            taken,
            version,
        );
    }

    let mut binds: Vec<String> =
        existing.map(|r| r.binds.clone()).unwrap_or_default();
    binds.push(iteration_id.to_owned());
    let id = existing.map_or_else(|| format!("REL.{version}"), |r| r.id.clone());
    let proposal = propose(&binds, corpus);
    let kdl = release_kdl(&id, version, &binds, &proposal, ask);

    Binding::Bound {
        release: Record {
            file: format!("releases/{id}.kdl"),
            id,
            kdl,
            failed: Vec::new(),
        },
        proposal,
    }
}

/// Unbind, which is allowed only while the release is still planned.
pub fn unbind(
    iteration_id: &str,
    version: &str,
    corpus: &Corpus,
    ask: &Ask,
    taken: &[String],
) -> Binding {
    let Some(release) = corpus.release(version) else {
        return Binding::NotFound(format!("{version} is no release the record holds"));
    };
    if release.cut() {
        return refuse(&[Rejected::AlreadyCut { version: version.to_owned() }], ask, taken, version);
    }
    if !release.binds.iter().any(|b| b == iteration_id) {
        return Binding::NotFound(format!("{iteration_id} is not bound to {version}"));
    }

    let binds: Vec<String> =
        release.binds.iter().filter(|b| *b != iteration_id).cloned().collect();
    let proposal = propose(&binds, corpus);
    let kdl = release_kdl(&release.id, version, &binds, &proposal, ask);
    Binding::Bound {
        release: Record {
            file: format!("releases/{}.kdl", release.id),
            id: release.id.clone(),
            kdl,
            failed: Vec::new(),
        },
        proposal,
    }
}

/// The bump the bound work implies, from the rules the config declares.
///
/// Which change kind maps to which position is a BINDING, read from the record. What the
/// positions rank as is what the scheme means, and a scheme the engine does not know
/// yields no proposal rather than a guess.
pub fn propose(binds: &[String], corpus: &Corpus) -> Proposal {
    let rank = |position: &str| match (corpus.config.scheme.as_str(), position) {
        ("semver", "major") => 3,
        ("semver", "minor") => 2,
        ("semver", "patch") => 1,
        _ => 0,
    };

    let mut best: Option<(u8, String, String)> = None;
    let mut silent = Vec::new();
    for id in binds {
        let Some(attempt) = corpus.attempts.iter().find(|a| &a.id == id) else { continue };
        if attempt.contributes.is_empty() {
            silent.push(id.clone());
            continue;
        }
        for kind in &attempt.contributes {
            let Some((_, position)) = corpus.config.bump_rules.iter().find(|(k, _)| k == kind)
            else {
                continue;
            };
            let score = rank(position);
            if best.as_ref().is_none_or(|(b, _, _)| score > *b) {
                best = Some((score, position.clone(), format!("{kind} ({id})")));
            }
        }
    }

    match best {
        Some((_, position, because)) => Proposal { position, because, silent },
        None if corpus.config.scheme.is_empty() => Proposal {
            position: "none".to_owned(),
            because: "the record declares no versioning scheme, so no position can be ranked"
                .to_owned(),
            silent,
        },
        None => Proposal {
            position: "none".to_owned(),
            because: "nothing bound matches a configured bump rule".to_owned(),
            silent,
        },
    }
}

fn refuse(rejected: &[Rejected], ask: &Ask, taken: &[String], version: &str) -> Binding {
    let id = crate::pickup::next_id("REF", &ask.at, taken);
    let mut kdl = String::new();
    kdl.push_str("// Written by `praxis bind`, which refused. Nothing was bound, and the\n");
    kdl.push_str("// version's content is unchanged.\n\n");
    kdl.push_str(&format!("refusal {id:?} {{\n"));
    kdl.push_str(&format!("    on-version {version:?}\n"));
    kdl.push_str(&format!("    at {:?}\n", ask.at));
    kdl.push_str(&format!("    by {:?}\n", ask.by));
    kdl.push_str(&format!("    asked-by {:?}\n", ask.signer));
    for item in rejected {
        let name = match item {
            Rejected::NotClosed { .. } => "only-closed-iterations-bind",
            Rejected::AlreadyBound { .. } => "an-iteration-binds-exactly-once",
            Rejected::AlreadyCut { .. } => "a-cut-release-does-not-change",
        };
        kdl.push_str(&format!(
            "    condition {name:?} failed=#true \\\n        found={:?}\n",
            squash(&item.message())
        ));
    }
    kdl.push_str("\n    trail {\n");
    kdl.push_str(&format!(
        "        entry at={:?} by={:?} action=\"created\" \\\n            note=\"the bind was refused. Nothing was bound\"\n",
        ask.at, ask.by
    ));
    kdl.push_str("    }\n}\n");
    Binding::Refused(Record {
        file: format!("releases/{id}.bind-refused.kdl"),
        id,
        kdl,
        failed: rejected.iter().map(|r| (name_of(r), r.message())).collect(),
    })
}

fn name_of(rejected: &Rejected) -> String {
    match rejected {
        Rejected::NotClosed { iteration, .. } | Rejected::AlreadyBound { iteration, .. } => {
            iteration.clone()
        }
        Rejected::AlreadyCut { version } => version.clone(),
    }
}

fn release_kdl(
    id: &str,
    version: &str,
    binds: &[String],
    proposal: &Proposal,
    ask: &Ask,
) -> String {
    let mut out = String::new();
    out.push_str("// Machine-owned. Every field here is derived from what is bound, so this file\n");
    out.push_str("// is composed whole on each bind rather than edited — there is nothing in it a\n");
    out.push_str("// human wrote that a rewrite could lose. An index into history, never a copy\n");
    out.push_str("// of it.\n\n");
    out.push_str(&format!("release {id:?} {{\n"));
    out.push_str(&format!("    version {version:?}\n"));
    out.push_str("    state \"planned\"\n");
    if binds.is_empty() {
        out.push_str("\n    // Nothing bound. Stated rather than left blank: a version with no\n");
        out.push_str("    // content and a version nobody has computed are not the same thing.\n");
    }
    for iteration in binds {
        out.push_str(&format!("    binds {iteration:?}\n"));
    }
    out.push_str(&format!(
        "\n    proposed-bump {:?} \\\n        because={:?}\n",
        proposal.position,
        squash(&proposal.because)
    ));
    if !proposal.silent.is_empty() {
        out.push_str(&format!(
            "    proposal-incomplete-for {} \\\n        because=\"these declare no `contributes`, so they informed the proposal with nothing\"\n",
            proposal.silent.iter().map(|s| format!("{s:?}")).collect::<Vec<_>>().join(" ")
        ));
    }
    out.push_str("    confirmation \"required\" by=\"human\" \\\n");
    out.push_str("        note=\"the record PROPOSES. Choosing the version is the maintainer's, at cut\"\n");
    out.push_str("\n    trail {\n");
    out.push_str(&format!(
        "        entry at={:?} by={:?} action=\"bound\" \\\n            note=\"composed from what is bound. Rewritten whole on every bind\"\n",
        ask.at, ask.by
    ));
    out.push_str("    }\n}\n");
    out
}

fn squash(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
