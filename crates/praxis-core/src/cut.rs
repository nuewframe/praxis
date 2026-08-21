//! Cutting a version. `TS.260820.09`: a point exists on the version line, indexed to the
//! commit it was cut at.
//!
//! A release without an index is a release nothing can be verified against. The index is
//! what makes going back a lookup rather than an archaeology exercise — and it POINTS at
//! a commit rather than copying anything git already holds.
//!
//! Atomicity here is by construction rather than by cleanup: the whole record is composed
//! before anything is written, so there is no half-cut state to roll back from. The cut
//! and the index node cannot come apart because they were never two operations.

use crate::admission::Corpus;
use crate::pickup::{Ask, Record};

/// What an ask to cut produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cut {
    /// Cut. The release record, composed whole, with its index node and seal.
    Made(Record),
    /// Refused, naming what stopped it. Nothing is cut.
    Refused(Record),
    NotFound(String),
}

/// Why a cut was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Blocked {
    /// A bound iteration did not close. C1.
    BoundButUnclosed { iteration: String, state: String },
    /// Already cut. A release is a point, not a range.
    AlreadyCut { version: String },
    /// Nothing bound. A version with no content is not a release.
    NothingBound { version: String },
    /// The bump was never confirmed. The record proposes; a human chooses.
    Unconfirmed { version: String, proposed: String },
}

impl Blocked {
    pub fn name(&self) -> &'static str {
        match self {
            Self::BoundButUnclosed { .. } => "only-closed-work-is-cut",
            Self::AlreadyCut { .. } => "a-release-is-cut-once",
            Self::NothingBound { .. } => "a-release-has-content",
            Self::Unconfirmed { .. } => "the-maintainer-confirms-the-version",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::BoundButUnclosed { iteration, state } => format!(
                "{iteration} is bound to this version and is {state:?}. A release containing work \
                 still in flight is a point on the line that nothing can be pinned to"
            ),
            Self::AlreadyCut { version } => format!(
                "{version} is already cut. A release is a point, and nothing rebinds to it"
            ),
            Self::NothingBound { version } => format!(
                "{version} binds nothing. A version with no content is not a release, and cutting \
                 one would put a point on the line that means nothing"
            ),
            Self::Unconfirmed { version, proposed } => format!(
                "{version} proposes a {proposed} bump and nobody confirmed it. The record proposes; \
                 choosing the version is the maintainer's, which is what `--confirm` says"
            ),
        }
    }
}

/// Cut a planned release. `commit` is where it is being cut — the shell resolves it,
/// because what HEAD is is a fact about the tree rather than about the record.
pub fn cut(
    version: &str,
    commit: &str,
    confirmed: bool,
    corpus: &Corpus,
    ask: &Ask,
    taken: &[String],
) -> Cut {
    let Some(release) = corpus.release(version) else {
        return Cut::NotFound(format!(
            "{version} is no release the record holds — bind work to it first"
        ));
    };

    let mut blocked = Vec::new();
    if release.cut() {
        blocked.push(Blocked::AlreadyCut { version: version.to_owned() });
    }
    if release.binds.is_empty() {
        blocked.push(Blocked::NothingBound { version: version.to_owned() });
    }
    for id in &release.binds {
        if let Some(attempt) = corpus.attempts.iter().find(|a| &a.id == id)
            && attempt.state != "closed"
        {
            blocked.push(Blocked::BoundButUnclosed {
                iteration: id.clone(),
                state: attempt.state.clone(),
            });
        }
    }
    let proposal = crate::binding::propose(&release.binds, corpus);
    if !confirmed && blocked.is_empty() {
        blocked.push(Blocked::Unconfirmed {
            version: version.to_owned(),
            proposed: proposal.position.clone(),
        });
    }

    if !blocked.is_empty() {
        let id = crate::pickup::next_id("REF", &ask.at, taken);
        return Cut::Refused(Record {
            file: format!("releases/{id}.cut-refused.kdl"),
            kdl: refusal_kdl(&id, version, ask, &blocked),
            id,
            failed: blocked.iter().map(|b| (b.name().to_owned(), b.message())).collect(),
        });
    }

    // Symptoms this release resolves. Derived from the symptoms that NAME it, never
    // stored beside them — the same argument as the index itself, one scale down.
    let resolves: Vec<String> = corpus
        .resolved_by(version)
        .into_iter()
        .map(str::to_owned)
        .collect();

    let kdl = release_kdl(release, version, commit, &proposal.position, &resolves, ask);
    Cut::Made(Record {
        file: format!("releases/{}.kdl", release.id),
        id: release.id.clone(),
        kdl,
        failed: Vec::new(),
    })
}

/// The seal over a cut release's content.
///
/// It makes an edit VISIBLE, which is all it claims. It is not a signature and it does not
/// make forgery hard: anyone who can edit the record can run the same function. What it
/// catches is the edit nobody meant to make and nobody would otherwise notice, which is
/// the failure this frame is actually about.
pub fn seal(version: &str, commit: &str, binds: &[String], resolves: &[String]) -> String {
    let mut sorted: Vec<&str> = binds.iter().map(String::as_str).collect();
    sorted.sort_unstable();
    let mut resolved: Vec<&str> = resolves.iter().map(String::as_str).collect();
    resolved.sort_unstable();

    let material = format!("{version}\u{1f}{commit}\u{1f}{}\u{1f}{}", sorted.join(","), resolved.join(","));
    // FNV-1a, 64-bit. Chosen because it needs no dependency and the property wanted is
    // "a different record produces a different seal", not secrecy.
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in material.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x1000_0000_01b3);
    }
    format!("fnv1a64:{hash:016x}")
}

fn release_kdl(
    release: &crate::admission::Release,
    version: &str,
    commit: &str,
    bump: &str,
    resolves: &[String],
    ask: &Ask,
) -> String {
    let tag = format!("v{version}");
    let mut out = String::new();
    out.push_str("// CUT. This record is immutable: nothing rebinds to a cut release and the index\n");
    out.push_str("// node is never edited. The `seal` below is computed from the content — change\n");
    out.push_str("// any of it and `praxis check` says so.\n");
    out.push_str("//\n");
    out.push_str("// The index POINTS at a commit. It never mirrors what git already holds.\n\n");
    out.push_str(&format!("release {:?} {{\n", release.id));
    out.push_str(&format!("    version {version:?}\n"));
    out.push_str("    state \"released\"\n");
    for iteration in &release.binds {
        out.push_str(&format!("    binds {iteration:?}\n"));
    }
    if resolves.is_empty() {
        out.push_str("\n    // Resolves nothing. Stated rather than omitted: a release that fixed\n");
        out.push_str("    // no symptom and one nobody checked are not the same thing.\n");
    }
    for symptom in resolves {
        out.push_str(&format!("    resolves {symptom:?}\n"));
    }
    out.push_str(&format!("\n    index {{\n        tag {tag:?}\n        commit {commit:?}\n"));
    out.push_str(&format!("        cut-at {:?}\n", ask.at));
    out.push_str(&format!("        cut-by {:?}\n", ask.signer));
    out.push_str(&format!("        confirmed-bump {bump:?}\n"));
    out.push_str("    }\n");
    out.push_str(&format!(
        "    seal {:?} \\\n        note=\"computed over version, commit, what it binds and what it resolves. It makes an edit visible; it is not a signature\"\n",
        seal(version, commit, &release.binds, resolves)
    ));
    out.push_str("\n    trail {\n");
    out.push_str(&format!(
        "        entry at={:?} by={:?} action=\"cut\" \\\n            note=\"the release and its index written in one operation. Neither exists without the other\"\n",
        ask.at, ask.by
    ));
    out.push_str("    }\n}\n");
    out
}

fn refusal_kdl(id: &str, version: &str, ask: &Ask, blocked: &[Blocked]) -> String {
    let mut out = String::new();
    out.push_str("// Written by `praxis cut`, which refused. Nothing was cut, and the version\n");
    out.push_str("// is still planned.\n\n");
    out.push_str(&format!("refusal {id:?} {{\n"));
    out.push_str(&format!("    on-version {version:?}\n"));
    out.push_str(&format!("    at {:?}\n", ask.at));
    out.push_str(&format!("    by {:?}\n", ask.by));
    out.push_str(&format!("    asked-by {:?}\n", ask.signer));
    for item in blocked {
        out.push_str(&format!(
            "    condition {:?} failed=#true \\\n        found={:?}\n",
            item.name(),
            item.message().split_whitespace().collect::<Vec<_>>().join(" ")
        ));
    }
    out.push_str("\n    trail {\n");
    out.push_str(&format!(
        "        entry at={:?} by={:?} action=\"created\" \\\n            note=\"the cut was refused. The version is still planned\"\n",
        ask.at, ask.by
    ));
    out.push_str("    }\n}\n");
    out
}
