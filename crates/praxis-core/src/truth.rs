//! `what-is-currently-true`. `TS.260820.03`: ask the record what this repository already
//! knows, instead of re-deriving it.
//!
//! This attacks `S7` directly — the agent that arrives with none of the previous one's
//! context and reconstructs it from prose. The answer is computed from state on every ask,
//! never from a summary somebody wrote, and it says what it does **not** cover so silence
//! is not read as absence.
//!
//! It is deliberately pre-projection: the content stands on its own before anything
//! renders it, which is what makes `read-model@v1` a seam rather than a formatting choice.

use crate::admission::Corpus;
use crate::view::{ReadModel, Section};

/// Compose the answer. `as_of` is a moment because this view's value IS being current —
/// the opposite of an archival result, which names the version it depicts and no clock.
pub fn what_is_currently_true(corpus: &Corpus, as_of: &str) -> ReadModel {
    let mut capabilities =
        Section::new("capabilities", &["capability", "state", "events owned", "shipped in"])
            .empty_because("nothing recorded — no capability has been named yet");
    for capability in &corpus.capabilities {
        let shipped = if capability.shipped.is_empty() {
            "nothing yet".to_owned()
        } else {
            capability
                .shipped
                .iter()
                .map(|s| s.version.clone())
                .collect::<Vec<_>>()
                .join(" · ")
        };
        capabilities.push(vec![
            capability.id.clone(),
            capability.state.clone(),
            capability.owns.len().to_string(),
            shipped,
        ]);
    }

    let mut releases = Section::new("releases", &["version", "state", "binds"])
        .empty_because("nothing recorded — no version has been bound yet");
    for release in &corpus.releases {
        releases.push(vec![
            release.version.clone(),
            release.state.clone(),
            release.binds.len().to_string(),
        ]);
    }

    let mut delivered = Section::new("slices delivered", &["slice", "slug", "capability"])
        .empty_because("nothing recorded — no slice has met every claim it declares");
    let mut outstanding = Section::new("slices outstanding", &["slice", "slug", "claims"])
        .empty_because("nothing recorded — every slice the record holds is delivered");
    for slice in &corpus.slices {
        if corpus.delivered(&slice.id) {
            delivered.push(vec![slice.id.clone(), slice.slug.clone(), slice.realizes.clone()]);
        } else {
            outstanding.push(vec![
                slice.id.clone(),
                slice.slug.clone(),
                slice.claims.len().to_string(),
            ]);
        }
    }

    // What is owed. A finding that carries a claim is a shortfall somebody recorded; the
    // rest are things learned. Both are owed, and neither should need looking for.
    let mut owed = Section::new("what is owed", &["from", "finding", "carries a claim"])
        .empty_because("nothing recorded — no closed iteration carried a finding");
    for attempt in corpus.attempts.iter().filter(|a| a.state == "closed") {
        for finding in &attempt.findings {
            owed.push(vec![
                attempt.id.clone(),
                finding.id.clone(),
                finding.carries.clone().unwrap_or_else(|| "no".to_owned()),
            ]);
        }
    }

    // What this plugin ships as instruction, and what asks for it. The next agent's first
    // question is "which of these skills is still the method?" — and before TS.260821.03
    // the only way to answer it was to open forty-seven files and guess.
    let mut doctrine = Section::new("doctrine shipped", &["surface", "kind", "serves", "state"])
        .empty_because(
            "nothing recorded — this record declares no doctrine-surface, so what the plugin \
             ships is not something it can be asked about",
        );
    for surface in &corpus.surfaces {
        doctrine.push(vec![
            surface.path.clone(),
            surface.kind.clone(),
            surface.serves.join(" · "),
            surface.state.clone(),
        ]);
    }

    // A view that does not say what it leaves out is one a reader will over-trust. This
    // section is the difference between an answer and an impression.
    let mut excluded = Section::new("not covered by this answer", &["question", "ask instead"])
        .empty_because("this answer covers everything the record holds");
    excluded.push(vec![
        "what is in flight right now".to_owned(),
        "the review view — an open iteration's claims and evidence".to_owned(),
    ]);
    excluded.push(vec![
        "what could be started right now".to_owned(),
        "`praxis ready` — this answer does not evaluate admission".to_owned(),
    ]);
    excluded.push(vec![
        "why any of it was decided".to_owned(),
        "the decisions the record holds, which this does not summarise".to_owned(),
    ]);
    excluded.push(vec![
        "which shipped files have NO anchor".to_owned(),
        "`praxis audit-surfaces` — this lists what the record declares, never what the tree \
         holds, because a read model that reads a directory answers a different question \
         depending on where it ran"
            .to_owned(),
    ]);

    ReadModel::new(
        "what-is-currently-true",
        "what does this repository already know, so I do not re-derive it?",
        as_of,
    )
    .section(capabilities)
    .section(releases)
    .section(delivered)
    .section(outstanding)
    .section(owed)
    .section(doctrine)
    .section(excluded)
}
