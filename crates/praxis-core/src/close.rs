//! Closing an iteration. `TS.260820.07`: refuse to close one that is about to drop scope
//! in silence.
//!
//! An unmet claim that closes quietly and a met claim look identical afterwards. The
//! refusal is the only thing that makes them different, which is `S5` — the symptom with
//! the most day-to-day cost, and the one every Markdown checklist has.
//!
//! `C3` is the check that matters. Every other rule here is defeated by editing the claim
//! list, so the claims are pinned into the iteration when the gate opens it: removing one
//! from the slice afterwards does not remove it from the iteration, it makes the two
//! disagree, and the disagreement is refused.

use crate::admission::Corpus;
use crate::pickup::{Ask, Record};

/// What an ask to close produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Closing {
    /// Every claim is accounted for. The iteration may close.
    Accepted {
        id: String,
        /// Each claim and how it was accounted: met, or carried by a named finding.
        accounting: Vec<(String, String)>,
        all_met: bool,
    },
    /// Refused, naming every claim that is neither met nor carried. Nothing closes.
    Refused(Record),
    /// Nothing to close.
    NoSuchIteration(String),
    /// Already closed. A later attempt is a new iteration, never a reopening.
    NotOpen(String),
}

/// Why one claim was not accounted for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unaccounted {
    /// Not met, and no finding carries it. This is the silent drop.
    NeitherMetNorCarried { claim: String, state: String },
    /// Carried by a finding the iteration does not hold.
    CarriedByNothing { claim: String, finding: String },
    /// The slice no longer declares a claim this iteration froze at open. C3.
    DroppedFromTheSlice { claim: String, slice: String },
}

impl Unaccounted {
    pub fn claim(&self) -> &str {
        match self {
            Self::NeitherMetNorCarried { claim, .. }
            | Self::CarriedByNothing { claim, .. }
            | Self::DroppedFromTheSlice { claim, .. } => claim,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::NeitherMetNorCarried { claim, state } => format!(
                "{claim} is {} and no finding carries it — record a finding that names it, or \
                 settle it. A shortfall nobody wrote down and a claim that was met look identical \
                 afterwards",
                if state.is_empty() { "unsettled".to_owned() } else { format!("{state:?}") }
            ),
            Self::CarriedByNothing { claim, finding } => format!(
                "{claim} names finding {finding}, which this iteration does not hold"
            ),
            Self::DroppedFromTheSlice { claim, slice } => format!(
                "{claim} was frozen into this iteration at open and {slice} no longer declares it. \
                 Deleting a claim is not a way to settle it — that is the edit this rule exists to \
                 refuse"
            ),
        }
    }
}

/// Evaluate the ask to close. Pure: it reads the corpus and composes whichever record the
/// answer calls for, and writes nothing.
pub fn close_iteration(iteration_id: &str, corpus: &Corpus, ask: &Ask, taken: &[String]) -> Closing {
    let Some(attempt) = corpus.attempts.iter().find(|a| a.id == iteration_id) else {
        return Closing::NoSuchIteration(format!(
            "{iteration_id} is no iteration the record holds"
        ));
    };
    if !attempt.in_flight() {
        return Closing::NotOpen(format!(
            "{iteration_id} is {:?}. A closed iteration is history — a later attempt is a new \
             iteration, never a reopening",
            attempt.state
        ));
    }

    let declared: Vec<String> = corpus
        .slice(&attempt.on_slice)
        .map(|s| s.claims.clone())
        .unwrap_or_default();

    let mut unaccounted = Vec::new();
    let mut accounting = Vec::new();
    for claim in &attempt.claims {
        // C3 first: a claim frozen here that the slice has since stopped declaring.
        if claim.from == attempt.on_slice && !declared.is_empty() && !declared.contains(&claim.id) {
            unaccounted.push(Unaccounted::DroppedFromTheSlice {
                claim: claim.id.clone(),
                slice: attempt.on_slice.clone(),
            });
            continue;
        }
        if claim.state == "met" {
            accounting.push((claim.id.clone(), "met".to_owned()));
            continue;
        }
        match attempt.findings.iter().find(|f| f.carries.as_deref() == Some(claim.id.as_str())) {
            Some(finding) => {
                accounting.push((claim.id.clone(), format!("carried by {}", finding.id)));
            }
            None => unaccounted.push(Unaccounted::NeitherMetNorCarried {
                claim: claim.id.clone(),
                state: claim.state.clone(),
            }),
        }
    }

    // A finding that says it carries a claim this iteration does not hold is an accounting
    // that refers to nothing.
    for finding in &attempt.findings {
        if let Some(claim) = &finding.carries
            && !attempt.claims.iter().any(|c| &c.id == claim)
        {
            unaccounted.push(Unaccounted::CarriedByNothing {
                claim: claim.clone(),
                finding: finding.id.clone(),
            });
        }
    }

    if unaccounted.is_empty() {
        let all_met = accounting.iter().all(|(_, how)| how == "met");
        return Closing::Accepted { id: iteration_id.to_owned(), accounting, all_met };
    }

    let id = crate::pickup::next_id("REF", &ask.at, taken);
    let kdl = refusal_kdl(&id, iteration_id, &attempt.on_slice, ask, &unaccounted);
    Closing::Refused(Record {
        file: format!("iterations/{id}.close-refused.kdl"),
        id,
        kdl,
        failed: unaccounted
            .iter()
            .map(|u| (u.claim().to_owned(), u.message()))
            .collect(),
    })
}

fn refusal_kdl(
    id: &str,
    iteration_id: &str,
    slice: &str,
    ask: &Ask,
    unaccounted: &[Unaccounted],
) -> String {
    let mut out = String::new();
    out.push_str("// Written by `praxis close`, which refused. The iteration stays open, and\n");
    out.push_str("// every claim it could not account for is named below. Nothing was closed.\n\n");
    out.push_str(&format!("refusal {id:?} {{\n"));
    out.push_str(&format!("    on-slice {slice:?}\n"));
    out.push_str(&format!("    at {:?}\n", ask.at));
    out.push_str(&format!("    by {:?}\n", ask.by));
    out.push_str(&format!("    asked-by {:?}\n", ask.signer));
    out.push_str(&format!("    on-iteration {iteration_id:?}\n"));
    for item in unaccounted {
        out.push_str(&format!(
            "    condition {:?} failed=#true \\\n        found={:?}\n",
            format!("claim-{}-is-unaccounted", item.claim()),
            squash(&item.message())
        ));
    }
    out.push_str("\n    trail {\n");
    out.push_str(&format!(
        "        entry at={:?} by={:?} action=\"created\" \\\n            note=\"the close was refused. The iteration stays open\"\n",
        ask.at, ask.by
    ));
    out.push_str("    }\n}\n");
    out
}

fn squash(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
