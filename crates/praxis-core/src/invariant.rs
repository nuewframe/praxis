//! `TS.260821.05`: hold what the plugin guarantees, and what keeps each guarantee.
//!
//! `praxis/config.kdl` has said `enable-all-fail-closed #true` since the binding was written,
//! and omitted three probes by name. Neither the enabling nor the omitting referred to
//! anything the record held — the ids were strings, and what enforced them was ten shell
//! scripts and their header comments.
//!
//! An invariant is what a probe is FOR. The probe is how. Nothing connected the two, which
//! is why twenty-five doctrine surfaces had no anchor: not because they were stale, but
//! because the record had no word for what they did.

use crate::admission::Corpus;

/// One property the plugin guarantees about code it is loaded into.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invariant {
    pub id: String,
    pub protects: String,
    /// `refuse` or `report`. The difference between a gate and a notice is the whole of what
    /// an adopter needs to know, and describing a probe as stronger than it is was already a
    /// named failure in this repository's own README.
    pub severity: String,
    pub languages: Vec<String>,
    /// A probe that reads structure rather than text applies to every language. Saying so is
    /// a different claim from listing nine of them, and the coverage matrix has to keep them
    /// apart or it overstates both.
    pub structural: Option<String>,
}

impl Invariant {
    pub fn fails_closed(&self) -> bool {
        self.severity == "refuse"
    }
}

/// What the check concluded about one invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Enforcement {
    /// Enabled by the config, and a surface enforces it.
    Kept { invariant: String, by: Vec<String> },
    /// Enabled, and nothing enforces it. The plugin guarantees something nothing keeps.
    Unkept { invariant: String },
    /// Omitted by this repository's profile. Not a failure — a binding, and the record has
    /// to hold both or "we do not need this" and "we did not do this" read alike.
    Omitted { invariant: String, because: String },
}

impl Enforcement {
    pub fn invariant(&self) -> &str {
        match self {
            Self::Kept { invariant, .. }
            | Self::Unkept { invariant }
            | Self::Omitted { invariant, .. } => invariant,
        }
    }

    pub fn unkept(&self) -> bool {
        matches!(self, Self::Unkept { .. })
    }
}

/// Check every invariant the record declares against what the config enables and what the
/// shipped surfaces enforce.
///
/// Total over the declared set: an invariant that is neither enabled nor omitted still
/// appears, as `Unkept`, because a guarantee nobody decided about is not the same as one
/// nobody needed.
pub fn check_invariants(corpus: &Corpus) -> Vec<Enforcement> {
    corpus
        .invariants
        .iter()
        .map(|invariant| {
            if corpus.config.omitted.iter().any(|o| o == &invariant.id) {
                return Enforcement::Omitted {
                    invariant: invariant.id.clone(),
                    because: "omitted by this repository's profile".to_owned(),
                };
            }
            let by: Vec<String> = corpus
                .surfaces
                .iter()
                .filter(|s| !s.retired() && s.serves.iter().any(|t| t == &invariant.id))
                .map(|s| s.path.clone())
                .collect();
            if by.is_empty() {
                Enforcement::Unkept { invariant: invariant.id.clone() }
            } else {
                Enforcement::Kept { invariant: invariant.id.clone(), by }
            }
        })
        .collect()
}

/// The guarantees nothing keeps. This is the answer to "what does `enable-all-fail-closed`
/// actually mean here", which has never been answerable.
pub fn unkept(corpus: &Corpus) -> Vec<Enforcement> {
    check_invariants(corpus).into_iter().filter(Enforcement::unkept).collect()
}
