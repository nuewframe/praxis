//! `TS.260821.13`: take a cut back in one command.
//!
//! The index points at a commit and never mirrors it, which is what makes cutting cheap.
//! Promotion breaks the symmetry by writing derived truth into records the cut does not own
//! — legitimate under `a-derived-field-is-recomputed`, and the cost lands on whoever moves
//! the pointer back. A derived field is only as cheap as its recomputation.
//!
//! Withdrawing `0.8.0` by hand meant three edits across seven files, and `praxis check` was
//! inconsistent in between (`WALK.260822.02/AS6`).

use crate::admission::Corpus;

/// What a withdrawal produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Withdrawal {
    /// The release returns to a marker, and these records were recomputed with it.
    Taken { version: String, unwound: Vec<String> },
    /// Refused, and why. Nothing moves.
    Refused(String),
}

/// Withdraw a cut release.
///
/// Total in one step: the pointer and everything the cut wrote elsewhere move together, or
/// neither does. A withdrawal that leaves five capability records asserting a release which
/// no longer stands has moved the problem rather than undone it.
pub fn withdraw(version: &str, because: Option<&str>, corpus: &Corpus) -> Withdrawal {
    let Some(release) = corpus.release(version) else {
        return Withdrawal::Refused(format!("{version} is no release the record holds"));
    };
    if release.state != "released" {
        return Withdrawal::Refused(format!(
            "{version} is {:?}, and only a cut release can be taken back. A version that was \
             never cut has no pointer to move",
            release.state
        ));
    }
    // A withdrawal nobody explained is a cut nobody can account for. Required rather than
    // defaulted, for the same reason `--attested-by` is (TS.260821.09).
    let Some(because) = because.filter(|b| !b.trim().is_empty()) else {
        return Withdrawal::Refused(
            "a withdrawal says why, in the same place the release said what it cut. Name it with \
             `--because <reason>`"
                .to_owned(),
        );
    };
    let _ = because;

    // Every capability whose promoted truth came from this release. Recomputed away here, in
    // the same operation, or `promoted-truth-is-derived` refuses the tree afterwards.
    let unwound = corpus
        .capabilities
        .iter()
        .filter(|c| c.shipped.iter().any(|s| s.version == version))
        .map(|c| c.id.clone())
        .collect();

    Withdrawal::Taken { version: version.to_owned(), unwound }
}
