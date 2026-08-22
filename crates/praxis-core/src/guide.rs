//! `how-to-use-a-capability`. `TS.260820.15`: write usage while building, and refuse to
//! publish a guide for truth that never shipped.
//!
//! A guide describing behaviour that never shipped is **worse than no guide** — it is a
//! document that is confidently wrong, and a reader has no way to tell. So a capability's
//! usage enters version N's guide only if that capability's truth was promoted for N.
//!
//! The prose lives in the record, attached to the capability, written while the capability
//! is being built. Not assembled at release time, which is when it is least accurate and
//! most rushed, and not in a file the record points at, which is a second thing to keep in
//! step.

use crate::admission::Corpus;
use crate::view::{ReadModel, Section};

/// Why a guide could not be produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoGuide {
    NoSuchCapability(String),
    /// The capability exists and this release did not promote its truth.
    NotPromoted { capability: String, version: String },
    /// Promoted, and nobody wrote how to use it.
    NoUsage { capability: String },
}

impl NoGuide {
    pub fn message(&self) -> String {
        match self {
            Self::NoSuchCapability(id) => format!("{id} is no capability the record holds"),
            Self::NotPromoted { capability, version } => format!(
                "{version} did not promote {capability}'s truth, so a guide for it would describe \
                 behaviour that never shipped at that version. That is worse than no guide — a \
                 reader cannot tell a confidently wrong document from a right one"
            ),
            Self::NoUsage { capability } => format!(
                "{capability} shipped a usable surface and nobody wrote how to use it. Reported \
                 rather than omitted: a guide that is silently missing and one that was never \
                 needed look identical"
            ),
        }
    }
}

/// The guide for one capability at one version, or why there is none.
pub fn guide_for<'a>(
    capability: &str,
    version: &str,
    corpus: &'a Corpus,
) -> Result<&'a [String], NoGuide> {
    let name = capability.trim_start_matches("CAP.");
    let Some(record) = corpus.capabilities.iter().find(|c| c.id == name) else {
        return Err(NoGuide::NoSuchCapability(capability.to_owned()));
    };
    if !record.usable_at(version) {
        return Err(NoGuide::NotPromoted {
            capability: name.to_owned(),
            version: version.to_owned(),
        });
    }
    if record.usage.is_empty() {
        return Err(NoGuide::NoUsage { capability: name.to_owned() });
    }
    Ok(&record.usage)
}

/// `how-to-use-a-capability` for a whole release.
pub fn guides(version: &str, corpus: &Corpus) -> ReadModel {
    // What this version BINDS, not what it promoted. Publishing happens before the cut and
    // promotion after it, so a guide gated on promoted truth is empty on every release ever
    // published — by construction, not by omission (WALK.260822.02/AS8).
    let shipped_here: Vec<&str> = corpus
        .release(version)
        .map(|r| r.binds.as_slice())
        .unwrap_or_default()
        .iter()
        .filter_map(|id| corpus.attempts.iter().find(|a| &a.id == id))
        .flat_map(|a| a.on_slices.iter())
        .filter_map(|id| corpus.slice(id))
        .map(|s| s.realizes.trim_start_matches("CAP.").to_owned())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
        .leak()
        .iter()
        .map(String::as_str)
        .collect();

    // One column, and the invocations defined beside it. `read-model@v1` constraint 2:
    // prose is REFERENCED, not inlined — and a usage line is `praxis bind <ITER> <VERSION>`,
    // which is markup in a cell by the seam's own test. Inlining it would mean rewriting the
    // record's words to fit a table.
    let mut written = Section::new("how to use what shipped", &["capability"])
        .empty_because(
            "this version binds no slice, so no capability gained a surface anybody could be \
             told how to use",
        );
    let mut gaps = Section::new("shipped without a guide", &["capability", "shipped in"])
        .empty_because("every capability this version bound work for carries usage prose");
    let mut waiting = Section::new("written, not yet shipped", &["capability", "why it is not here"])
        .empty_because("no capability carries usage for a surface this release did not promote");

    let mut how: Vec<(String, String)> = Vec::new();
    for capability in &corpus.capabilities {
        if shipped_here.contains(&capability.id.as_str()) {
            if capability.usage.is_empty() {
                gaps.push(vec![capability.id.clone(), version.to_owned()]);
                continue;
            }
            written.push(vec![capability.id.clone()]);
            how.push((capability.id.clone(), capability.usage.join("\n\n")));
        } else if !capability.usage.is_empty() {
            // Usage written before the truth shipped. Named rather than dropped, so the
            // absence reads as timing instead of as an oversight.
            waiting.push(vec![
                capability.id.clone(),
                format!("{version} bound no slice realizing it"),
            ]);
        }
    }

    let mut model = ReadModel::new(
        "how-to-use-a-capability",
        format!("how do I use what {version} can do?"),
        version,
    );
    model.publishable = true;
    let mut model = model.section(written).section(gaps).section(waiting);
    for (capability, usage) in how {
        model = model.define(capability, usage);
    }
    model
}
