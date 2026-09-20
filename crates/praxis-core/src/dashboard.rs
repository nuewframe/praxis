//! The dashboard. `TS.260820.13`: see where the product stands right now, without
//! committing a document that ages.
//!
//! This is `E19`'s test. A **document** composes several read models; a **read model**
//! still has exactly one owner. Nothing here reaches across an ownership boundary — each
//! part is emitted by the capability that owns its content, and composition only puts them
//! in order.
//!
//! It is also the view the publication test rejects **by name** (`E17`): a dashboard's
//! entire value is being current, and a committed dashboard is a claim that starts
//! decaying the moment it is written.

use crate::admission::{Conditions, Corpus, assess};
use crate::view::ReadModel;

/// One part of a composed document, and the capability whose content it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    pub owner: String,
    pub model: ReadModel,
}

/// A document composed from several read models, each still singly owned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Composed {
    pub title: String,
    pub as_of: String,
    pub parts: Vec<Part>,
    /// Views this composition wanted and could not attribute. See `orphaned()`.
    pub orphaned: Vec<String>,
}

impl Composed {
    /// Whether any capability contributed more than one part. A composite document with a
    /// cross-capability owner would break the one-owner rule, so the check is on the
    /// document rather than on the model.
    /// Views the dashboard could not attribute, because no capability declares owning them.
    ///
    /// Named rather than defaulted. A part with an invented owner reads as attributed and is
    /// not, which is the case `TS.260823.03` exists to remove.
    #[must_use]
    pub fn orphaned(&self) -> &[String] {
        &self.orphaned
    }

    pub fn owners_are_distinct(&self) -> bool {
        let mut seen: Vec<&str> = Vec::new();
        for part in &self.parts {
            if seen.contains(&part.owner.as_str()) {
                return false;
            }
            seen.push(&part.owner);
        }
        true
    }
}

/// Compose the dashboard as of now.
///
/// `conditions` come from the record, as everywhere else; without them the readiness part
/// is omitted rather than computed from an assumed gate — an empty gate is not an open one.
pub fn dashboard(corpus: &Corpus, conditions: &Conditions, as_of: &str) -> Composed {
    // Every owner is ASKED of the record. Until `TS.260823.03` the three below were
    // literals — `CAP.delivery-record`, `CAP.work-admission`, `CAP.release-binding` — which
    // are this repository's capability names, compiled into the tool every repository runs.
    // A dashboard composed in a repository whose only capability was `payment` attributed
    // its parts to three capabilities that repository had never held.
    //
    // A view nobody owns yields NO part rather than a part with an invented owner. The gap
    // is the honest output: `owners_are_held` is what says so, and `a-value-nothing-claims`
    // already reports an unclaimed value one level down.
    let mut parts = Vec::new();
    let mut orphaned = Vec::new();
    let push = |name: &str, model, parts: &mut Vec<Part>, orphaned: &mut Vec<String>| {
        match corpus.owner_of_view(name) {
            Some(owner) => parts.push(Part { owner, model }),
            None => orphaned.push(name.to_owned()),
        }
    };

    push(
        "what-is-currently-true",
        crate::truth::what_is_currently_true(corpus, as_of),
        &mut parts,
        &mut orphaned,
    );

    if !conditions.is_empty() {
        let assessment = assess(corpus, conditions, as_of);
        push(
            "what-is-ready-to-pick-up",
            crate::admission::project(&assessment, corpus),
            &mut parts,
            &mut orphaned,
        );
    }

    // The newest version the record holds. A dashboard shows where the product IS, which
    // is the release being worked toward rather than the last one finished.
    if let Some(release) = corpus.releases.iter().max_by(|a, b| a.version.cmp(&b.version)) {
        let mut model = crate::publish::published_set(&release.version, corpus);
        // Composed on demand, so it carries the moment. The same content published into a
        // release directory carries the version instead — the seam is the same and the
        // lifetime is not.
        model.as_of = as_of.to_owned();
        model.publishable = false;
        push("the-published-set-for-a-release", model, &mut parts, &mut orphaned);
    }

    Composed {
        title: "where the product stands".to_owned(),
        as_of: as_of.to_owned(),
        parts,
        orphaned,
    }
}
