//! `TS.260820.18` — close a symptom by naming a release that demonstrably attacked it.
//!
//! The sharpest check in the set: it makes progress against a problem **computable from
//! the record** rather than declared by whoever wants to close it. It is also the one most
//! likely to refuse something the maintainer believes, which is the point.

use praxis_core::{Schema, check_corpus, parse, refused};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "frame" {
            field "symptom" each="0..n" holds="symptom"
        }
        entity "symptom" {
        }
        entity "thin-slice" {
            field "slug"    each="1"
            field "attacks" each="0..n"
        }
        entity "iteration" {
            field "on-slice" each="1"
            field "state"    each="1"
        }
        entity "release" {
            field "version" each="1"
            field "state"   each="1"
            field "binds"   each="0..n"
        }
        rule "resolution-names-a-release" refuses="a resolution the record cannot compute"
    }
}
"##;

fn violations(record: &str) -> Vec<praxis_core::Violation> {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    check_corpus(&[schema_doc, record_doc], &schema)
}

/// A release that actually attacked S1: a cut version binding an iteration on a slice
/// whose `attacks` names it.
const ATTACKED: &str = r##"
frame "F.1" {
    symptom "S1" state="resolved" resolved-by="0.1.0" text="the thing that was wrong"
}
thin-slice "TS.a" {
    slug "a-thing"
    attacks "S1"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
}
release "REL.0.1.0" {
    version "0.1.0"
    state "released"
    binds "ITER.1"
}
"##;

#[test]
fn a_resolution_the_record_can_compute_is_accepted() {
    assert!(violations(ATTACKED).is_empty(), "{:?}", violations(ATTACKED));
}

#[test]
fn c1_resolving_against_a_release_that_does_not_exist_is_refused() {
    let found = violations(&ATTACKED.replace(r#"resolved-by="0.1.0""#, r#"resolved-by="0.5.0""#));
    assert!(refused(&found));
    let message = found[0].refusal.message();
    assert!(message.contains("S1") && message.contains("0.5.0"), "{message}");
    assert!(message.contains("never by an opinion"), "{message}");
}

#[test]
fn c1_a_release_that_is_only_planned_does_not_resolve_anything() {
    // Binding work to a version is not shipping it. Until it is cut, the symptom is not
    // resolved by it — the frame shrinks at the cut, not at the intent.
    let found = violations(&ATTACKED.replace(r#"state "released""#, r#"state "planned""#));
    assert!(refused(&found));
}

#[test]
fn c2_a_release_that_bound_no_attacking_slice_is_refused() {
    let found = violations(&ATTACKED.replace(r#"    attacks "S1""#, r#"    attacks "S9""#));
    assert!(refused(&found));
    let message = found[0].refusal.message();
    assert!(message.contains("bound no slice whose `attacks` names S1"), "{message}");
    assert!(message.contains("COMPUTED from what shipped"), "{message}");
}

#[test]
fn c2_an_attacking_slice_in_a_different_release_does_not_count() {
    // The attack has to be in THIS release. A slice that attacks S1 and shipped somewhere
    // else says nothing about what 0.1.0 did.
    let record = ATTACKED.replace(
        r##"release "REL.0.1.0" {
    version "0.1.0"
    state "released"
    binds "ITER.1"
}"##,
        r##"release "REL.0.1.0" {
    version "0.1.0"
    state "released"
}
release "REL.0.2.0" {
    version "0.2.0"
    state "released"
    binds "ITER.1"
}"##,
    );
    let found = violations(&record);
    assert!(refused(&found), "0.1.0 bound nothing, so it attacked nothing");
}

#[test]
fn a_partially_resolved_symptom_is_held_to_the_same_standard() {
    // `partially-resolved` is a state, not a hedge. It names a release like any other
    // resolution and is computed like any other.
    let found = violations(
        &ATTACKED
            .replace(r#"state="resolved""#, r#"state="partially-resolved""#)
            .replace(r#"    attacks "S1""#, r#"    attacks "S9""#),
    );
    assert!(refused(&found), "a partial claim is still a claim");
}

#[test]
fn a_symptom_still_present_names_no_release_and_is_not_checked() {
    let found = violations(
        r##"
frame "F.1" {
    symptom "S1" state="present" text="still wrong"
}
"##,
    );
    assert!(found.is_empty(), "nothing to compute until something is claimed");
}

#[test]
fn a_withdrawn_claim_leaves_the_symptom_present_and_the_record_clean() {
    // What this repository's own S1 had to do: the claim was not false, it was
    // UNCOMPUTABLE — 0.5.0 predates the record. Withdrawing it is what the rule asks for,
    // and grandfathering it would have been the root cause with a nicer surface.
    let found = violations(
        r##"
frame "F.1" {
    symptom "S1" state="present" text="still wrong" \
        previously-claimed="partially-resolved by 0.5.0" \
        withdrawn-because="the record holds no cut release at that version"
}
"##,
    );
    assert!(found.is_empty());
}
