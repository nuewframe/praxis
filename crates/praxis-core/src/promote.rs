//! Promoting what shipped. `TS.260820.17`: fold what a release contains into what each
//! capability says it is, in one operation.
//!
//! Nothing here is hand-written. What moves is **derived** from the bound work of cut
//! releases — which is what makes `C2` checkable at all: a hand-edit to a capability's
//! current truth is not discouraged, it is *recomputable*, and the rule
//! `promoted-truth-is-derived` refuses any capability whose promoted block is not what the
//! record would produce.
//!
//! That is also the answer to `ITER.260821.07/W7`. A derived field written at one moment
//! and never rechecked is a cache. A derived field written at one moment and *recomputed
//! by a rule* is a projection with a proof attached.

use std::collections::BTreeMap;

use crate::admission::Corpus;

/// One line of promoted truth: this capability gained this, in this release.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Shipped {
    pub version: String,
    pub iteration: String,
    pub slice: String,
}

/// What one capability's record must become.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub capability: String,
    pub shipped: Vec<Shipped>,
    /// Whether this promotion moves the capability out of `sought`. A capability with
    /// shipped work is no longer something the system is looking for.
    pub becomes_active: bool,
}

/// What an ask to promote produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Promotion {
    /// The whole change set, computed before anything is applied. A promotion that cannot
    /// be computed in full is not applied in part.
    Ready { version: String, changes: Vec<Change> },
    /// Already promoted. Not an error: promoting twice is a no-op, never a double
    /// application (`C3`).
    AlreadyPromoted { version: String },
    Refused(Vec<String>),
}

/// Every capability's promoted truth, as the record would derive it from cut releases.
///
/// This is the single definition. `promote` writes it and `promoted-truth-is-derived`
/// recomputes it — so the command and the check cannot disagree about what truth is.
pub fn derive(corpus: &Corpus) -> BTreeMap<String, Vec<Shipped>> {
    let mut out: BTreeMap<String, Vec<Shipped>> = BTreeMap::new();
    for release in corpus.releases.iter().filter(|r| r.cut()) {
        for iteration in &release.binds {
            let Some(attempt) = corpus.attempts.iter().find(|a| &a.id == iteration) else {
                continue;
            };
            for slice in attempt.on_slices.iter().filter_map(|id| corpus.slice(id)) {
                if slice.realizes.is_empty() {
                    continue;
                }
                let capability = slice.realizes.trim_start_matches("CAP.").to_owned();
                out.entry(capability).or_default().push(Shipped {
                    version: release.version.clone(),
                    iteration: attempt.id.clone(),
                    slice: slice.id.clone(),
                });
            }
        }
    }
    for shipped in out.values_mut() {
        shipped.sort();
        shipped.dedup();
    }
    out
}

/// Compute the promotion for one cut release.
pub fn promote(version: &str, corpus: &Corpus) -> Promotion {
    let Some(release) = corpus.release(version) else {
        return Promotion::Refused(vec![format!("{version} is no release the record holds")]);
    };
    if !release.cut() {
        return Promotion::Refused(vec![format!(
            "{version} is not cut. Promotion is part of releasing, not a task that follows it — \
             there is nothing to fold in until the release is a point on the line"
        )]);
    }

    let derived = derive(corpus);
    let mut changes = Vec::new();
    for (capability, shipped) in &derived {
        // Only capabilities this release touched are moved by promoting it, but what is
        // WRITTEN is the whole derivation — so a promotion is idempotent and a promotion
        // that runs after an earlier one was interrupted repairs it.
        if !shipped.iter().any(|s| s.version == version) {
            continue;
        }
        let record = corpus.capabilities.iter().find(|c| &c.id == capability);
        let already = record.is_some_and(|c| c.shipped == *shipped);
        let becomes_active = record.is_some_and(|c| c.state == "sought");
        if already && !becomes_active {
            continue;
        }
        changes.push(Change {
            capability: capability.clone(),
            shipped: shipped.clone(),
            becomes_active,
        });
    }

    if changes.is_empty() {
        return Promotion::AlreadyPromoted { version: version.to_owned() };
    }
    Promotion::Ready { version: version.to_owned(), changes }
}

/// Capabilities whose promoted truth is not what the record derives, and what it should
/// have been. `promoted-truth-is-derived` refuses each of these.
pub fn undeclared_promotions(corpus: &Corpus) -> Vec<(String, Vec<Shipped>, Vec<Shipped>)> {
    let derived = derive(corpus);
    let mut out = Vec::new();
    for capability in &corpus.capabilities {
        let expected = derived.get(&capability.id).cloned().unwrap_or_default();
        if capability.shipped != expected {
            out.push((capability.id.clone(), capability.shipped.clone(), expected));
        }
    }
    out
}
