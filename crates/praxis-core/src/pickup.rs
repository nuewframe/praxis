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
    pub by: String,
}

/// Evaluate the gate for one slice and compose whichever record it produced.
///
/// `taken` is every id already in use, so a new one does not collide. The gate never
/// reads the disk: which ids exist is a fact handed to it, like every other.
pub fn pick_up(slice_id: &str, corpus: &Corpus, assessment: &Assessment, ask: &Ask, taken: &[String]) -> Pickup {
    let Some(slice) = corpus.slice(slice_id) else {
        return Pickup::NoSuchSlice(format!(
            "{slice_id} is no slice the record holds, so there is nothing to admit or refuse"
        ));
    };
    let Some(verdicts) = assessment.verdicts.get(slice_id) else {
        return Pickup::NoSuchSlice(format!("{slice_id} was never assessed"));
    };

    let failed: Vec<(String, String)> = verdicts
        .iter()
        .filter(|(_, v)| v.blocks())
        .map(|(c, v)| (c.clone(), v.detail().to_owned()))
        .collect();

    // Declared, undecided, and carried into the record either way. An admission that does
    // not say what it left undecided is an admission of less than it appears to be.
    let undecided: Vec<(String, String)> = verdicts
        .iter()
        .filter(|(_, v)| matches!(v, Verdict::Uncomputed(_) | Verdict::Judgement))
        .map(|(c, v)| (c.clone(), v.detail().to_owned()))
        .collect();

    if failed.is_empty() {
        let id = next_id("ITER", &ask.at, taken);
        let kdl = iteration_kdl(&id, slice_id, &slice.slug, ask, verdicts, &undecided);
        Pickup::Opened(Record {
            file: format!("iterations/{id}.{}.kdl", slice.slug),
            id,
            kdl,
            failed,
        })
    } else {
        let id = next_id("REF", &ask.at, taken);
        let kdl = refusal_kdl(&id, slice_id, ask, &failed, &undecided);
        Pickup::Refused(Record {
            file: format!("iterations/{id}.{}.kdl", slice.slug),
            id,
            kdl,
            failed,
        })
    }
}

/// `PREFIX.YYMMDD.NN`, the first NN not already taken. Two agents picking up on one day
/// get distinct files and never contend — the layout rule ADR.260819.01 gives for
/// iterations, applied to what a refused pick-up produces as well.
fn next_id(prefix: &str, at: &str, taken: &[String]) -> String {
    let day: String = at.chars().filter(|c| c.is_ascii_digit()).take(8).collect();
    let day = if day.len() == 8 { day[2..].to_owned() } else { day };
    (1..100)
        .map(|n| format!("{prefix}.{day}.{n:02}"))
        .find(|id| !taken.iter().any(|t| t == id))
        .unwrap_or_else(|| format!("{prefix}.{day}.99"))
}

fn iteration_kdl(
    id: &str,
    slice_id: &str,
    slug: &str,
    ask: &Ask,
    verdicts: &[(String, Verdict)],
    undecided: &[(String, String)],
) -> String {
    let mut out = String::new();
    out.push_str("// Opened by `praxis pick-up`. Every condition below was evaluated against the\n");
    out.push_str(&format!("// tree as it stood at {}, and the verdicts are recorded\n", ask.at));
    out.push_str("// rather than summarised: a gate whose reasoning is not on the record is a gate\n");
    out.push_str("// you have to trust.\n\n");
    out.push_str(&format!("iteration {id:?} {{\n"));
    // The slug names the ATTEMPT, and the gate cannot know what this attempt will be
    // about — only which slice it is on. So it seeds from the slice and expects a human
    // to rename it once there is something to name.
    out.push_str(&format!("    slug {slug:?}\n"));
    out.push_str(&format!("    on-slice {slice_id:?}\n"));
    out.push_str("    state \"open\"\n");
    out.push_str(&format!("    opened-at {:?}\n", ask.at));
    out.push_str(&format!("    opened-by {:?}\n", ask.by));
    out.push_str("\n    approval \"admission\" {\n");
    out.push_str("        status \"signed\"\n");
    out.push_str(&format!("        signer {:?}\n", ask.signer));
    out.push_str(&format!("        at {:?}\n", ask.at));
    out.push_str("        signed-by \"the ask to pick up this slice\"\n");
    out.push_str("    }\n");
    out.push_str(&format!(
        "\n    vet \"create\" state=\"done\" at={:?} {{\n",
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
    if !undecided.is_empty() {
        out.push_str(&format!(
            "\n    // {} of the declared conditions were not decided by the gate. They are\n",
            undecided.len()
        ));
        out.push_str("    // recorded above rather than omitted: an admission that does not say what\n");
        out.push_str("    // it left undecided claims more than it checked.\n");
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
    slice_id: &str,
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
    out.push_str(&format!("    on-slice {slice_id:?}\n"));
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
