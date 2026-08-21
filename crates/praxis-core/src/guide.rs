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
    let mut written = Section::new("how to use what shipped", &["capability", "usage"])
        .empty_because(
            "nothing recorded — this release promoted no capability's truth, so there is no \
             shipped surface to describe",
        );
    let mut gaps = Section::new("shipped without a guide", &["capability", "shipped in"])
        .empty_because("every capability this release promoted carries usage prose");
    let mut waiting = Section::new("written, not yet shipped", &["capability", "why it is not here"])
        .empty_because("no capability carries usage for a surface this release did not promote");

    for capability in &corpus.capabilities {
        if capability.usable_at(version) {
            if capability.usage.is_empty() {
                gaps.push(vec![capability.id.clone(), version.to_owned()]);
                continue;
            }
            for line in &capability.usage {
                written.push(vec![capability.id.clone(), line.clone()]);
            }
        } else if !capability.usage.is_empty() {
            // Usage written before the truth shipped. Named rather than dropped, so the
            // absence reads as timing instead of as an oversight.
            waiting.push(vec![
                capability.id.clone(),
                format!("{version} did not promote its truth"),
            ]);
        }
    }

    let mut model = ReadModel::new(
        "how-to-use-a-capability",
        format!("how do I use what {version} can do?"),
        version,
    );
    model.publishable = true;
    model.section(written).section(gaps).section(waiting)
}
