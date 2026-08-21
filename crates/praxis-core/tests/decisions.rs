//! `TS.260820.14` — bind a decision to the work that forced it, and correct it only by
//! appending.
//!
//! A decision with no alternatives and nothing that would show it wrong is a preference,
//! and a preference recorded as a decision is the hardest kind to argue with later: there
//! is nothing to argue against.

use praxis_core::{Corpus, Publication, Schema, check_corpus, cut::seal, parse, publish, refused};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "iteration" {
            field "on-slice" each="1"
            field "state"    each="1"
            field "decision" each="0..n" holds="decision"
            field "finding"  each="0..n"
        }
        entity "decision" {
        }
        entity "event-storm" {
            field "read-model" each="0..n" holds="read-model"
        }
        entity "read-model" {
            field "answers" each="0..1"
        }
        entity "release" {
            field "version" each="1"
            field "state"   each="1"
            field "binds"   each="0..n"
        }
        rule "a-decision-names-what-it-rejected" refuses="a decision naming no alternative"
        rule "a-decision-names-its-falsifier" refuses="a decision naming no falsifier"
        rule "a-decision-is-bound-to-an-iteration" refuses="an unbound decision"
        rule "an-accepted-decision-is-append-only" refuses="a rewritten decision"
    }
}
"##;

fn violations(record: &str) -> Vec<praxis_core::Violation> {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    check_corpus(&[schema_doc, record_doc], &schema)
}

fn corpus_of(record: &str) -> Corpus {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    Corpus::from_documents(&[schema_doc, record_doc], &schema)
}

/// The material the seal covers: title, chose, over, because, falsified-by. Amendments are
/// deliberately outside it, so appending one is not an edit.
fn body(title: &str, chose: &str, over: &str, because: &str, falsifier: &str) -> String {
    const UNIT: char = '\u{1f}';
    format!("{title}{UNIT}{chose}{UNIT}{over}{UNIT}{because}{UNIT}{falsifier}")
}

#[test]
fn c2_a_decision_with_no_alternatives_is_refused() {
    let found = violations(
        r##"
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    decision "we did the obvious thing" chose="the obvious thing" falsified-by="something"
}
"##,
    );
    assert!(refused(&found));
    let message = found[0].refusal.message();
    assert!(message.contains("we did the obvious thing"), "the decision is NAMED: {message}");
    assert!(message.contains("is a preference"), "{message}");
}

#[test]
fn c2_a_decision_with_no_falsifier_is_refused() {
    let found = violations(
        r##"
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    decision "a real choice" chose="this" over="that" because="reasons"
}
"##,
    );
    assert!(refused(&found));
    assert!(found[0].refusal.message().contains("falsified-by"));
}

#[test]
fn c3_a_decision_bound_to_no_iteration_is_refused() {
    let found = violations(
        r##"
decision "filed beside the work" chose="this" over="that" falsified-by="something"
"##,
    );
    assert!(refused(&found));
    assert!(found[0].refusal.message().contains("belongs to no iteration"));
    assert!(found[0].refusal.message().contains("what forced it"));
}

#[test]
fn c1_editing_the_body_of_an_accepted_decision_is_refused() {
    // Sealed with one `chose`, then the body says another. The seal is over the body and
    // the body moved.
    let stale = seal(&body("a choice", "the original", "the other", "why", "what would break it"), "", &[], &[]);
    let found = violations(&format!(
        r##"
iteration "ITER.1" {{
    on-slice "TS.a"
    state "closed"
    decision "a choice" chose="something else entirely" over="the other" because="why" \
        falsified-by="what would break it" state="accepted" seal={stale:?}
}}
"##
    ));
    assert!(refused(&found));
    let message = found[0].refusal.message();
    assert!(message.contains("no longer matches what was accepted"), "{message}");
    assert!(message.contains("APPENDING an amendment"), "and it says what to do instead: {message}");
}

#[test]
fn c1_appending_an_amendment_is_not_an_edit() {
    // The same decision, same body, with a correction appended. The seal must still hold —
    // otherwise the only way to correct a decision would be to break its seal, and
    // append-only would be unusable.
    let intact = seal(&body("a choice", "the original", "the other", "why", "what would break it"), "", &[], &[]);
    let found = violations(&format!(
        r##"
iteration "ITER.1" {{
    on-slice "TS.a"
    state "closed"
    decision "a choice" chose="the original" over="the other" because="why" \
        falsified-by="what would break it" state="accepted" seal={intact:?} {{
        amendment "narrowed in 0.9.0 — the original still stands for the case it named"
    }}
}}
"##
    ));
    assert!(found.is_empty(), "appending must not read as a rewrite: {found:?}");
}

#[test]
fn a_decision_still_proposed_is_not_yet_sealed_against() {
    let found = violations(
        r##"
iteration "ITER.1" {
    on-slice "TS.a"
    state "working"
    decision "still arguing" chose="this, for now" over="that" falsified-by="something" \
        state="proposed" seal="fnv1a64:0000000000000000"
}
"##,
    );
    assert!(
        !found.iter().any(|v| matches!(v.refusal, praxis_core::Refusal::RewrittenDecision { .. })),
        "append-only starts at acceptance, not before"
    );
}

const RECORDED: &str = r##"
event-storm "ES.1" {
    read-model "the-decisions-that-shaped-this" publishable=#true \
        because="a decision is frozen by nature" \
        publishes-to="docs/releases/<version>/decisions/"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    decision "the machine does half" chose="half, honestly" over="all of it, as a formality" \
        because="one command cannot reproduce two moments" \
        falsified-by="the second check never catching anything"
    finding "F1" tests="the machine does half" text="what happened when it ran"
}
release "REL.0.9.0" {
    version "0.9.0"
    state "planned"
    binds "ITER.1"
}
"##;

#[test]
fn c4_findings_that_tested_a_decision_are_reachable_from_it() {
    let Publication::Ready { documents, .. } = publish("0.9.0", &corpus_of(RECORDED)) else {
        panic!("expected a publication")
    };
    let model = &documents.iter().find(|d| d.model.view == "the-decisions-that-shaped-this").expect("the decisions document").model;
    let tested = model.sections.iter().find(|s| s.name == "what tested it").expect("a section");
    assert_eq!(tested.rows.len(), 1);
    assert_eq!(tested.rows[0], vec!["the machine does half", "F1", "ITER.1"]);
}

#[test]
fn a_decision_carries_the_iteration_that_forced_it() {
    let corpus = corpus_of(RECORDED);
    assert_eq!(corpus.decisions.len(), 1);
    assert_eq!(corpus.decisions[0].iteration, "ITER.1", "bound to what forced it");
    assert_eq!(corpus.decisions[0].over, vec!["all of it, as a formality"]);
}

#[test]
fn the_alternatives_appear_in_the_published_document() {
    let Publication::Ready { documents, .. } = publish("0.9.0", &corpus_of(RECORDED)) else {
        panic!("expected a publication")
    };
    let model = &documents.iter().find(|d| d.model.view == "the-decisions-that-shaped-this").expect("a document").model;
    let rejected = model.sections.iter().find(|s| s.name == "what it rejected").expect("a section");
    assert_eq!(rejected.rows[0][1], "all of it, as a formality");
    assert!(model.flaws().is_empty(), "{:?}", model.flaws());
}
