//! `what-this-iteration-claims-and-has-shown`. `TS.260820.08`: review what an iteration
//! has actually shown, writing nothing.
//!
//! This attacks `S2` — a reviewer judging an artifact by how finished it *looks*. What
//! makes that possible is that "what was promised" and "what was shown" arrive in
//! different places and get reconciled in someone's head. Here they arrive together, in
//! one answer, with the gaps named.
//!
//! It is the second `E10` test: the content is owned by claim-settlement and the mechanism
//! by document-projection. If rendering this needs a code path specific to this view, the
//! capability split is fake.

use crate::admission::Phase;
use crate::admission::Corpus;
use crate::view::{ReadModel, Section};

/// Compose the preview for one iteration. `None` when the record holds no such iteration.
pub fn review(iteration_id: &str, corpus: &Corpus, as_of: &str) -> Option<ReadModel> {
    let attempt = corpus.attempts.iter().find(|a| a.id == iteration_id)?;
    // Layers come from the SLICES, so a layer none of them declared is visible as
    // undeclared and one none of them reached is visible as unevidenced.
    let declared: Vec<String> = attempt
        .on_slices
        .iter()
        .filter_map(|id| corpus.slice(id))
        .flat_map(|s| s.layers.clone())
        .fold(Vec::new(), |mut acc, layer| {
            if !acc.contains(&layer) {
                acc.push(layer);
            }
            acc
        });

    // What it promised against what it has shown, in one place. A claim with no evidence
    // is listed with the gap stated rather than left out of the table.
    let mut claims = Section::new("what it promised", &["claim", "from", "state", "evidence"])
        .empty_because("nothing recorded — this iteration carries no claim");
    for claim in &attempt.claims {
        let carried = attempt
            .findings
            .iter()
            .find(|f| f.carries.as_deref() == Some(claim.id.as_str()))
            .map(|f| format!("carried by {}", f.id));
        let shown = match (claim.state.as_str(), carried) {
            ("met", _) => "settled".to_owned(),
            (_, Some(by)) => by,
            (state, None) if state.is_empty() => "nothing yet".to_owned(),
            (_, None) => "nothing yet".to_owned(),
        };
        claims.push(vec![claim.id.clone(), claim.from.clone(), claim.state.clone(), shown]);
    }

    // U3, owed by ITER.260821.05: a layer the SLICE declared and the iteration has not
    // evidenced must appear here as unevidenced, not be omitted. A layer that vanishes
    // from the review was never reached AND never refused.
    let mut layers = Section::new("what it has reached", &["layer", "state", "evidence"])
        .empty_because("nothing recorded — the slice declares no layer");
    for layer in &declared {
        match attempt.layers.iter().find(|(name, _, _)| name == layer) {
            Some((_, state, evidence)) => layers.push(vec![
                layer.clone(),
                if state.is_empty() { "declared".to_owned() } else { state.clone() },
                if evidence.is_empty() { "none named".to_owned() } else { evidence.clone() },
            ]),
            None => layers.push(vec![
                layer.clone(),
                "unevidenced".to_owned(),
                "nothing yet — declared by the slice and not reached".to_owned(),
            ]),
        }
    }
    // And a layer the iteration claims that the slice never declared, which is the other
    // half of evidence-names-its-layer showing up where a human will read it.
    for (name, state, evidence) in &attempt.layers {
        if !declared.contains(name) {
            layers.push(vec![
                name.clone(),
                format!("{state} — NOT DECLARED by {}", attempt.slices()),
                evidence.clone(),
            ]);
        }
    }

    let mut phases = Section::new("how far it has got", &["phase", "state", "produced"])
        .empty_because("nothing recorded — no phase has been opened");
    for Phase { kind, state, produced, .. } in &attempt.phases {
        phases.push(vec![
            kind.clone(),
            state.clone(),
            if produced.is_empty() { "nothing recorded".to_owned() } else { produced.clone() },
        ]);
    }

    let mut open = Section::new("what is still open", &["finding", "carries a claim", "size"])
        .empty_because("nothing recorded — this iteration has raised no finding");
    for finding in &attempt.findings {
        open.push(vec![
            finding.id.clone(),
            finding.carries.clone().unwrap_or_else(|| "no".to_owned()),
            String::new(),
        ]);
    }
    // The size column is not in the corpus; rather than invent it, the column is dropped.
    let open = Section {
        columns: vec!["finding".to_owned(), "carries a claim".to_owned()],
        rows: open.rows.into_iter().map(|mut r| { r.pop(); r }).collect(),
        ..open
    };

    let mut model = ReadModel::new(
        "what-this-iteration-claims-and-has-shown",
        format!(
            "PREVIEW of {iteration_id} on {} — what did it promise, what has it evidenced, and \
             what is still open?",
            attempt.slices()
        ),
        as_of,
    );
    // Never publishable. Its value is being CURRENT, and a preview committed to the tree
    // is a record of a version that does not exist.
    model.publishable = false;
    Some(
        model
            .section(claims)
            .section(layers)
            .section(phases)
            .section(open)
            .define(
                "this is a preview",
                format!(
                    "composed from the record as it stands, for an iteration whose state is {:?}. \
                     It is not a record of anything and nothing was written to produce it",
                    attempt.state
                ),
            ),
    )
}
