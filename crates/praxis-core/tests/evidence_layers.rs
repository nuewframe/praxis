//! `TS.260820.06` — back a layer's claim with an artifact instead of the agent's word.
//!
//! The slice sets the granularity when it is cut. `evidence-names-its-layer` is what makes
//! "how much of this was actually reached" a question the record can answer, and it has
//! been a policy since the storm was held with nothing enforcing it.

use praxis_core::{Known, Schema, check_corpus, index_all, parse, refused};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug"  each="1"
            field "layer" each="1..n"
        }
        entity "iteration" {
            field "on-slice"  each="1"
            field "state"     each="1"
            field "layer"     each="0..n"
            field "layer-set" each="0..1"
        }
        rule "evidence-names-its-layer" refuses="an iteration evidencing a layer its slice never declared"
    }
}
"##;

/// The same schema with the rule left out, to prove the engine applies it because the
/// record declares it — not because the engine has it.
const SCHEMA_WITHOUT_THE_RULE: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug"  each="1"
            field "layer" each="1..n"
        }
        entity "iteration" {
            field "on-slice"  each="1"
            field "state"     each="1"
            field "layer"     each="0..n"
            field "layer-set" each="0..1"
        }
    }
}
"##;

fn violations(schema_src: &str, record: &str) -> Vec<praxis_core::Violation> {
    let schema_doc = parse(schema_src).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let mut known = Known::default();
    index_all(&schema_doc, &mut known);
    index_all(&record_doc, &mut known);
    check_corpus(&[schema_doc, record_doc], &schema)
}

const UNDECLARED: &str = r##"
thin-slice "TS.a" {
    slug "a"
    layer "doctrine"
    layer "enforcement"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    layer "doctrine" state="evidenced"
    layer "harness" state="evidenced"
    layer-set sealed=#true
}
"##;

#[test]
fn c1_evidence_naming_an_undeclared_layer_is_refused() {
    let found = violations(SCHEMA, UNDECLARED);
    let layer = found
        .iter()
        .find(|v| matches!(v.refusal, praxis_core::Refusal::UndeclaredLayer { .. }))
        .expect("the undeclared layer is refused");
    assert!(refused(&found), "and it fails closed");
    assert_eq!(layer.entity_id.as_deref(), Some("ITER.1"), "naming the iteration, not the slice");

    let message = layer.refusal.message();
    assert!(message.contains("harness"), "it names the layer: {message}");
    assert!(message.contains("TS.a"), "and the slice that did not declare it: {message}");
    assert!(
        message.contains("doctrine") && message.contains("enforcement"),
        "and what the slice DID declare, because the next question is which one was meant: {message}"
    );
}

#[test]
fn c2_a_declared_layer_with_no_evidence_is_reported_not_omitted() {
    let found = violations(SCHEMA, UNDECLARED);
    let unevidenced = found
        .iter()
        .find(|v| matches!(v.refusal, praxis_core::Refusal::UnevidencedLayer { .. }))
        .expect("`enforcement` is declared, evidenced by nothing, and still visible");
    assert_eq!(
        unevidenced.severity(),
        praxis_core::Severity::Report,
        "an iteration may close having reached less than it hoped — not silently"
    );
    assert!(unevidenced.refusal.message().contains("enforcement"));
}

#[test]
fn an_iteration_reaching_everything_it_declared_raises_nothing() {
    let found = violations(
        SCHEMA,
        r##"
thin-slice "TS.a" {
    slug "a"
    layer "doctrine"
    layer "enforcement"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    layer "doctrine" state="evidenced"
    layer "enforcement" state="evidenced"
    layer-set sealed=#true
}
"##,
    );
    assert!(found.is_empty(), "{found:?}");
}

#[test]
fn an_unsealed_iteration_is_still_deciding_what_it_will_reach() {
    // A layer set that is not sealed is not yet a promise, so an absent layer is not yet
    // a gap. This is what lets `praxis pick-up` open an iteration with no layers at all.
    let found = violations(
        SCHEMA,
        r##"
thin-slice "TS.a" {
    slug "a"
    layer "doctrine"
    layer "enforcement"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "open"
}
"##,
    );
    assert!(found.is_empty(), "nothing is owed before the set is sealed: {found:?}");
}

#[test]
fn the_rule_applies_because_the_record_declares_it() {
    // A4: the engine may not hold a rule the record does not state. Take the declaration
    // away and the same record raises nothing — with no change to the engine.
    let with = violations(SCHEMA, UNDECLARED);
    let without = violations(SCHEMA_WITHOUT_THE_RULE, UNDECLARED);
    assert!(!with.is_empty(), "declared, and enforced");
    assert!(without.is_empty(), "undeclared, and silent: {without:?}");
}

#[test]
fn an_iteration_on_a_slice_the_record_does_not_hold_is_not_judged_on_layers() {
    // The dangling `on-slice` is somebody else's refusal. This rule does not invent a
    // second one for the same defect.
    let found = violations(
        SCHEMA,
        r##"
iteration "ITER.1" {
    on-slice "TS.nowhere"
    state "closed"
    layer "doctrine" state="evidenced"
    layer-set sealed=#true
}
"##,
    );
    assert!(
        !found.iter().any(|v| matches!(
            v.refusal,
            praxis_core::Refusal::UndeclaredLayer { .. } | praxis_core::Refusal::UnevidencedLayer { .. }
        )),
        "{found:?}"
    );
}
