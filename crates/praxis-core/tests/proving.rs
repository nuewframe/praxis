//! `TS.260821.02` — report any declared rule that nothing demonstrates refusing.
//!
//! A rule that has never been shown to refuse is indistinguishable from one that cannot.
//! `ITER.260821.17/AG1` proved it the expensive way: a correct rule reported nothing
//! because its refusals were computed and silently dropped.

use praxis_core::{
    NO_WITNESS, Proof, Schema, WITNESS_REFUSED_NOTHING, WITNESS_UNPARSEABLE, parse, prove,
    unwitnessed,
};

/// A schema whose rules are declared with and without witnesses. The rules come from the
/// record here exactly as they do in the tree.
fn schema_of(rules: &str) -> Schema {
    let doc = parse(&format!(
        r##"
notional-architecture "NA.test" {{
    schema {{
        entity "capability" {{
            field "state"      each="1"
            field "owns-event" each="0..n" unique-in="capability"
        }}
        entity "iteration" {{
            field "on-slice" each="1..n"
            field "state"    each="1"
            field "decision" each="0..n" holds="decision"
            field "claim"    each="0..n"
        }}
        entity "decision" {{
        }}
{rules}
    }}
}}
"##
    ))
    .expect("schema parses");
    Schema::from_document(&doc)
}

#[test]
fn c1_a_rule_whose_witness_refuses_nothing_is_reported_by_name() {
    let schema = schema_of(
        r##"        rule "a-value-claimed-twice" refuses="two records claiming one value" \
            witness=#"""
                capability "only-one" {
                    state "sought"
                    owns-event "Uncontested"
                }
                """#
"##,
    );
    let found = unwitnessed(&schema);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].rule(), "a-value-claimed-twice", "the rule is NAMED");
    let Proof::Unwitnessed { why, .. } = &found[0] else { panic!("expected unwitnessed") };
    assert_eq!(*why, WITNESS_REFUSED_NOTHING);
}

#[test]
fn c1_a_rule_with_no_witness_at_all_reads_differently() {
    // `never demonstrated` and `demonstrated and nothing happened` are different problems.
    let schema = schema_of(r##"        rule "a-value-claimed-twice" refuses="two records claiming one value"
"##);
    let found = unwitnessed(&schema);
    let Proof::Unwitnessed { why, .. } = &found[0] else { panic!("expected unwitnessed") };
    assert_eq!(*why, NO_WITNESS);
    assert_ne!(*why, WITNESS_REFUSED_NOTHING, "the two must not read alike");
}

#[test]
fn c2_a_witness_proves_a_rule_by_producing_a_refusal_the_engine_computed() {
    let schema = schema_of(
        r##"        rule "a-value-claimed-twice" refuses="two records claiming one value" \
            witness=#"""
                capability "one" {
                    state "sought"
                    owns-event "Contested"
                }
                capability "two" {
                    state "sought"
                    owns-event "Contested"
                }
                """#
"##,
    );
    let proofs = prove(&schema);
    assert_eq!(proofs.len(), 1);
    let Proof::Witnessed { rule, refusals } = &proofs[0] else {
        panic!("the witness violates the rule: {proofs:?}")
    };
    assert_eq!(rule, "a-value-claimed-twice");
    assert!(*refusals >= 1, "the count is of refusals the checker actually produced");
}

#[test]
fn c2_a_refusal_enforcing_a_different_rule_does_not_witness_this_one() {
    // The strong half of C2. A witness that produces SOME refusal but not this rule's
    // would otherwise pass, and every rule could be witnessed by one malformed record.
    let schema = schema_of(
        r##"        rule "a-value-claimed-twice" refuses="two records claiming one value" \
            witness=#"""
                capability "missing-its-state" {
                    owns-event "Uncontested"
                }
                """#
"##,
    );
    // That witness DOES refuse — for entity-without-a-required-field — and still leaves
    // a-value-claimed-twice unwitnessed.
    let found = unwitnessed(&schema);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].rule(), "a-value-claimed-twice");
}

#[test]
fn c2_a_witness_the_codec_cannot_read_is_reported_as_such() {
    let schema = schema_of(
        r##"        rule "a-value-claimed-twice" refuses="two records claiming one value" \
            witness="capability \"unclosed\" {"
"##,
    );
    let found = unwitnessed(&schema);
    let Proof::Unwitnessed { why, .. } = &found[0] else { panic!("expected unwitnessed") };
    assert_eq!(*why, WITNESS_UNPARSEABLE, "not the same as refusing nothing");
}

#[test]
fn c3_a_rule_whose_refusals_are_computed_and_dropped_has_no_witness() {
    // AG1, reintroduced. Its refusals were about a NESTED entity — a decision inside an
    // iteration — and attribution searched root nodes only, so they were computed and
    // lost. The prover runs the witness through the same path the tree goes through, so a
    // rule that loses its refusals on the way out loses its witness with them.
    let schema = schema_of(
        r##"        rule "a-decision-is-bound-to-an-iteration" refuses="an unbound decision" \
            witness=#"""
                iteration "ITER.witness" {
                    on-slice "TS.witness"
                    state "closed"
                    decision "nested, not root" chose="this" over="that" falsified-by="something"
                }
                """#
"##,
    );
    // The witness nests the decision, so the rule that refuses ROOT-level decisions finds
    // nothing — and says so rather than passing.
    let found = unwitnessed(&schema);
    assert_eq!(found.len(), 1);
    let Proof::Unwitnessed { why, .. } = &found[0] else { panic!("expected unwitnessed") };
    assert_eq!(*why, WITNESS_REFUSED_NOTHING);
}

#[test]
fn c3_the_same_rule_with_a_witness_that_reaches_it_is_proven() {
    let schema = schema_of(
        r##"        rule "a-decision-is-bound-to-an-iteration" refuses="an unbound decision" \
            witness=#"""
                decision "filed beside the work" chose="this" over="that" falsified-by="something"
                """#
"##,
    );
    assert!(prove(&schema)[0].proven(), "the rule can fire, and now something shows it");
}

#[test]
fn c4_an_immutable_record_with_no_seal_is_refused() {
    let schema = schema_of(
        r##"        rule "an-immutable-record-carries-a-seal" refuses="an immutable record with no seal" \
            witness=#"""
                iteration "ITER.witness" {
                    on-slice "TS.witness"
                    state "closed"
                    decision "accepted and sealed by nothing" chose="this" over="that" \
                        falsified-by="something" state="accepted"
                }
                """#
"##,
    );
    assert!(prove(&schema)[0].proven());
}

#[test]
fn c4_a_sealed_decision_raises_nothing() {
    // The rule refuses ABSENCE. Whether the seal still matches is
    // an-accepted-decision-is-append-only's question, and they are different rules.
    let schema = schema_of(
        r##"        rule "an-immutable-record-carries-a-seal" refuses="an immutable record with no seal" \
            witness=#"""
                iteration "ITER.witness" {
                    on-slice "TS.witness"
                    state "closed"
                    decision "sealed" chose="this" over="that" falsified-by="something" \
                        state="accepted" seal="fnv1a64:1234567890abcdef"
                }
                """#
"##,
    );
    let found = unwitnessed(&schema);
    assert_eq!(found.len(), 1, "nothing to refuse, so nothing witnesses it");
}

#[test]
fn a_duplicate_claim_is_refused_naming_the_claim_and_its_slice() {
    // W6 / AI1: the gate pins a claim at open and the working agent settles it. Writing
    // the settled one BESIDE the pinned one leaves both, and the close reads whichever
    // comes first. It happened twice, the second time inside the iteration that declared
    // the rule against it.
    let schema = schema_of(
        r##"        rule "a-claim-id-is-unique-in-its-iteration" refuses="two claims with one id" \
            witness=#"""
                iteration "ITER.witness" {
                    on-slice "TS.witness"
                    state "working"
                    claim "C1" from-slice="TS.witness" state="pending"
                    claim "C1" from-slice="TS.witness" state="met"
                }
                """#
"##,
    );
    assert!(prove(&schema)[0].proven());
}

#[test]
fn one_claim_per_slice_is_not_a_duplicate() {
    // A commitment covering several slices may carry the same claim ID from each of them,
    // and that is not a duplicate — it is two slices that both called their first claim C1.
    let schema = schema_of(
        r##"        rule "a-claim-id-is-unique-in-its-iteration" refuses="two claims with one id" \
            witness=#"""
                iteration "ITER.witness" {
                    on-slice "TS.one"
                    on-slice "TS.two"
                    state "working"
                    claim "C1" from-slice="TS.one" state="pending"
                    claim "C1" from-slice="TS.two" state="pending"
                }
                """#
"##,
    );
    let found = unwitnessed(&schema);
    assert_eq!(found.len(), 1, "the same id from two slices is not a duplicate");
}

#[test]
fn a_commitment_that_drops_a_slices_claim_is_refused() {
    // AJ6, found by falling into it. TS.260820.07/C3 refuses deleting a claim from the
    // SLICE to make a close succeed; this is the same move one level over — leave the
    // slice alone and drop the claim from the iteration. The close accounts for what the
    // iteration HOLDS, so nothing objected.
    let doc = parse(
        r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug"  each="1"
            field "claim" each="0..n"
        }
        entity "iteration" {
            field "on-slice" each="1..n"
            field "state"    each="1"
            field "claim"    each="0..n"
        }
        rule "a-commitment-carries-every-claim" refuses="a dropped claim" \
            witness=#"""
                thin-slice "TS.witness" {
                    slug "witnessed"
                    claim "C1"
                    claim "C2"
                }
                iteration "ITER.witness" {
                    on-slice "TS.witness"
                    state "closed"
                    claim "C1" from-slice="TS.witness" state="met"
                }
                """#
    }
}
"##,
    )
    .expect("schema parses");
    let schema = Schema::from_document(&doc);
    assert!(prove(&schema)[0].proven(), "the dropped C2 is refused");
}

#[test]
fn a_commitment_carrying_everything_its_slices_declare_raises_nothing() {
    let doc = parse(
        r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug"  each="1"
            field "claim" each="0..n"
        }
        entity "iteration" {
            field "on-slice" each="1..n"
            field "state"    each="1"
            field "claim"    each="0..n"
        }
        rule "a-commitment-carries-every-claim" refuses="a dropped claim" \
            witness=#"""
                thin-slice "TS.witness" {
                    slug "witnessed"
                    claim "C1"
                }
                iteration "ITER.witness" {
                    on-slice "TS.witness"
                    state "closed"
                    claim "C1" from-slice="TS.witness" state="met"
                }
                """#
    }
}
"##,
    )
    .expect("schema parses");
    let schema = Schema::from_document(&doc);
    assert_eq!(unwitnessed(&schema).len(), 1, "nothing to refuse, so nothing witnesses it");
}
