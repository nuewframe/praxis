//! The one gate. `TS.260820.05`: take a slice, and have the refusal recorded when it is
//! refused.
//!
//! Everything here is pure. The gate decides and composes a record; the shell writes it.
//! That split is what lets every condition be tested without a filesystem, and it is also
//! `change-set@v1`: one command produces one change set, applied whole or not at all.
//!
//! The refusal is the point. A gate that was never invoked and a gate that refused look
//! identical afterwards — which is the whole of `S3` — so a refused pick-up writes a
//! record naming the condition that failed, and opens nothing.

use crate::admission::{Assessment, Corpus, Verdict};

/// What a pick-up produced. Exactly one of these, never both and never neither.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pickup {
    /// The gate admitted. An iteration record, ready to be written.
    Opened(Record),
    /// The gate refused, naming every condition that failed. Nothing opens.
    Refused(Record),
    /// The ask names no slice the record holds. Nothing is decided, because there was
    /// nothing to decide about.
    NoSuchSlice(String),
}

/// One file the gate produced: what to call it, and what is in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub id: String,
    pub file: String,
    pub kdl: String,
    /// The conditions that failed, each with the specific thing that failed it. Empty on
    /// an admission.
    pub failed: Vec<(String, String)>,
}

/// Who asked, and when. The signer is a human or the record refuses it — an agent-signed
/// approval is the trust-transfer problem expressed as a signature.
#[derive(Debug, Clone)]
pub struct Ask {
    pub signer: String,
    pub at: String,
    /// What RAN. The tool, always — `agent:praxis`.
    pub by: String,
    /// Who ATTESTS, named by whoever ran the command and never defaulted (`TS.260821.09`).
    ///
    /// `by` and this are different facts, and conflating them is the defect that shipped:
    /// `an-attestation-is-not-self-issued` compared `by` — the tool — against the identity
    /// that worked a phase, which are two things that can never be equal. The rule passed
    /// four tests and could not fire (ITER.260822.05/AP3).
    ///
    /// `None` is a close nobody attested, and is refused. A default here would prove that
    /// the tool ran, which nobody doubted.
    pub attested_by: Option<String>,
}

/// Evaluate the gate for one slice and compose whichever record it produced.
///
/// `taken` is every id already in use, so a new one does not collide. The gate never
/// reads the disk: which ids exist is a fact handed to it, like every other.
/// Take one or more slices through the gate, as ONE commitment.
///
/// Every slice is vetted separately and the commitment is admitted only if all of them
/// are: a commitment that admits its easy half is not one thing, and the ask was for one
/// thing. A refusal names which slice failed which condition.
pub fn pick_up(
    slice_ids: &[String],
    corpus: &Corpus,
    assessment: &Assessment,
    ask: &Ask,
    taken: &[String],
) -> Pickup {
    if slice_ids.is_empty() {
        return Pickup::NoSuchSlice("no slice was named, so there is nothing to admit".to_owned());
    }

    let mut slices = Vec::new();
    for id in slice_ids {
        let Some(slice) = corpus.slice(id) else {
            return Pickup::NoSuchSlice(format!(
                "{id} is no slice the record holds, so there is nothing to admit or refuse"
            ));
        };
        if assessment.verdicts.get(id).is_none() {
            return Pickup::NoSuchSlice(format!("{id} was never assessed"));
        }
        slices.push(slice);
    }

    let mut failed = Vec::new();
    let mut undecided = Vec::new();
    let mut per_slice = Vec::new();
    let committed = slices.len();
    for slice in &slices {
        let verdicts = &assessment.verdicts[&slice.id];
        for (condition, verdict) in verdicts {
            if verdict.blocks() {
                // Named per slice only when there are several: with one, the suffix is
                // noise; with four, `dependencies-delivered` alone does not say which of
                // them is waiting.
                let named = if committed == 1 {
                    condition.clone()
                } else {
                    format!("{condition} ({})", slice.id)
                };
                failed.push((named, verdict.detail().to_owned()));
            }
            if matches!(verdict, Verdict::Uncomputed(_) | Verdict::Judgement)
                && !undecided.iter().any(|(c, _): &(String, String)| c == condition)
            {
                undecided.push((condition.clone(), verdict.detail().to_owned()));
            }
        }
        per_slice.push((slice.id.clone(), verdicts.clone()));
    }

    // The file is named for the commitment, which is the first slice's slug when there is
    // one and the count when there are several — a filename cannot carry four slugs.
    let name = if slices.len() == 1 {
        slices[0].slug.clone()
    } else {
        format!("{}-and-{}-more", slices[0].slug, slices.len() - 1)
    };

    if failed.is_empty() {
        let id = next_id("ITER", &ask.at, taken);
        let claims: Vec<(String, String)> = slices
            .iter()
            .flat_map(|s| s.claims.iter().map(|c| (s.id.clone(), c.clone())))
            .collect();
        let kdl = iteration_kdl(&id, &name, &claims, ask, &per_slice, &undecided);
        Pickup::Opened(Record { file: format!("iterations/{id}.{name}.kdl"), id, kdl, failed })
    } else {
        let id = next_id("REF", &ask.at, taken);
        let kdl = refusal_kdl(&id, slice_ids, ask, &failed, &undecided);
        Pickup::Refused(Record { file: format!("iterations/{id}.{name}.kdl"), id, kdl, failed })
    }
}

/// `PREFIX.YYMMDD.NN`, the first NN not already taken. Two agents picking up on one day
/// get distinct files and never contend — the layout rule ADR.260819.01 gives for
/// iterations, applied to what a refused pick-up produces as well.
pub(crate) fn next_id(prefix: &str, at: &str, taken: &[String]) -> String {
    let day: String = at.chars().filter(|c| c.is_ascii_digit()).take(8).collect();
    let day = if day.len() == 8 { day[2..].to_owned() } else { day };
    (1..100)
        .map(|n| format!("{prefix}.{day}.{n:02}"))
        .find(|id| !taken.iter().any(|t| t == id))
        .unwrap_or_else(|| format!("{prefix}.{day}.99"))
}

fn iteration_kdl(
    id: &str,
    name: &str,
    claims: &[(String, String)],
    ask: &Ask,
    per_slice: &[(String, Vec<(String, Verdict)>)],
    undecided: &[(String, String)],
) -> String {
    let mut out = String::new();
    out.push_str("// Opened by `praxis pick-up`. Every condition below was evaluated against the\n");
    out.push_str(&format!("// tree as it stood at {}, and the verdicts are recorded\n", ask.at));
    out.push_str("// rather than summarised: a gate whose reasoning is not on the record is a gate\n");
    out.push_str("// you have to trust.\n");
    if per_slice.len() > 1 {
        out.push_str("//\n");
        out.push_str(&format!(
            "// One commitment over {} slices. They were vetted separately and admitted\n",
            per_slice.len()
        ));
        out.push_str("// together — a commitment that admits its easy half is not one thing.\n");
    }
    out.push('\n');
    out.push_str(&format!("iteration {id:?} {{\n"));
    // The slug names the COMMITMENT, and the gate cannot know what it will be about —
    // only which slices it covers. It seeds from them and expects a human to rename it.
    out.push_str(&format!("    slug {name:?}\n"));
    for (slice, _) in per_slice {
        out.push_str(&format!("    on-slice {slice:?}\n"));
    }
    out.push_str("    state \"open\"\n");
    out.push_str(&format!("    opened-at {:?}\n", ask.at));
    out.push_str(&format!("    opened-by {:?}\n", ask.by));
    out.push_str("\n    approval \"admission\" {\n");
    out.push_str("        status \"signed\"\n");
    out.push_str(&format!("        signer {:?}\n", ask.signer));
    out.push_str(&format!("        at {:?}\n", ask.at));
    out.push_str(&format!(
        "        signed-by \"the ask to pick up {}\"\n",
        if per_slice.len() == 1 { "this slice" } else { "these slices, as one commitment" }
    ));
    out.push_str("    }\n");

    for (slice, verdicts) in per_slice {
        out.push_str(&format!(
            "\n    vet \"create\" state=\"done\" at={:?} on-slice={slice:?} {{\n",
            ask.at
        ));
        for (condition, verdict) in verdicts {
            let (word, detail) = match verdict {
                Verdict::Admits => ("passed", String::new()),
                Verdict::Blocks(why) => ("failed", why.clone()),
                Verdict::Uncomputed(why) => ("not-computed", why.clone()),
                Verdict::Judgement => ("left-to-the-maintainer", String::new()),
            };
            out.push_str(&format!("        condition {condition:?} verdict={word:?}"));
            if !detail.is_empty() {
                out.push_str(&format!(" \\\n            found={:?}", squash(&detail)));
            }
            out.push('\n');
        }
        out.push_str("    }\n");
    }

    if !undecided.is_empty() {
        out.push_str(&format!(
            "\n    // {} of the declared conditions were not decided by the gate. They are\n",
            undecided.len()
        ));
        out.push_str("    // recorded above rather than omitted: an admission that does not say what\n");
        out.push_str("    // it left undecided claims more than it checked.\n");
    }

    // The claims, pinned at open, each carrying the slice that declared it. A claim list
    // that can be edited mid-iteration is a close you can always make succeed by dropping
    // what you did not reach (TS.260820.07/C3).
    if !claims.is_empty() {
        out.push_str("\n    // Frozen at open, from the slices. Removing one from a slice now\n");
        out.push_str("    // does not remove it from here — it makes the two disagree, and that is\n");
        out.push_str("    // refused.\n");
        for (slice, claim) in claims {
            out.push_str(&format!(
                "    claim {claim:?} from-slice={slice:?} state=\"pending\"\n"
            ));
        }
    }

    out.push_str("\n    trail {\n");
    out.push_str(&format!(
        "        entry at={:?} by={:?} action=\"created\" \\\n            note=\"opened by the gate, on the ask of {}\"\n",
        ask.at, ask.by, ask.signer
    ));
    out.push_str("    }\n}\n");
    out
}

fn refusal_kdl(
    id: &str,
    slice_ids: &[String],
    ask: &Ask,
    failed: &[(String, String)],
    undecided: &[(String, String)],
) -> String {
    let mut out = String::new();
    out.push_str(
        "// Written by `praxis pick-up`, which refused. It sits beside the iterations because\n\
         // an admission and a refusal are the two outcomes of one command, and a folder is\n\
         // named for what its contents ARE to the owner (ADR.260819.01). Nothing was opened.\n\n",
    );
    out.push_str(&format!("refusal {id:?} {{\n"));
    // One node per slice. Joining them into a single value made `on-slice` name a
    // thin-slice the record does not hold — the refusal path had never been run with more
    // than one slice, which is AK1's family: the commitment rewire touched every admission
    // site and the refusal writer beside it kept compiling.
    for slice_id in slice_ids {
        out.push_str(&format!("    on-slice {slice_id:?}\n"));
    }
    out.push_str(&format!("    at {:?}\n", ask.at));
    out.push_str(&format!("    by {:?}\n", ask.by));
    out.push_str(&format!("    asked-by {:?}\n", ask.signer));
    for (condition, detail) in failed {
        out.push_str(&format!(
            "    condition {condition:?} failed=#true \\\n        found={:?}\n",
            squash(detail)
        ));
    }
    for (condition, detail) in undecided {
        out.push_str(&format!(
            "    condition {condition:?} failed=#false \\\n        found={:?}\n",
            squash(&format!("not decided — {detail}"))
        ));
    }
    out.push_str("\n    trail {\n");
    out.push_str(&format!(
        "        entry at={:?} by={:?} action=\"created\" \\\n            note=\"the gate refused. Nothing opened\"\n",
        ask.at, ask.by
    ));
    out.push_str("    }\n}\n");
    out
}

/// A recorded reason is one value. Authored line breaks belong to the author.
fn squash(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
