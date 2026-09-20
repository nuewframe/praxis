//! `TS.260821.18` — record a value that changed as a change.
//!
//! A product matures by ADDING a value, REMOVING one that did not work, and CHANGING one for
//! something better. This record held two of the three: removal is strong — twenty-six
//! retired surfaces name the tag they still stand at — and addition is a new node. Change was
//! rewritten in place, so afterwards "we changed our mind" and "we were always right" looked
//! identical.

use praxis_core::{Known, Schema, check_corpus, check_node, index_all, parse};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "capability" {
            field "doing"   each="1"
            field "matured" each="0..n" holds="matured"
        }
        entity "matured" {
            field "from"      each="1"
            field "to"        each="1"
            field "at"        each="1"
            field "because"   each="1"
            field "taught-by" each="1"
        }
        rule "a-value-agrees-with-its-maturation" refuses="a value contradicting its own change"
    }
}
"##;

fn violations(record: &str) -> Vec<praxis_core::Violation> {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let mut known = Known::default();
    index_all(&schema_doc, &mut known);
    index_all(&record_doc, &mut known);
    let mut all: Vec<_> = record_doc
        .nodes()
        .iter()
        .flat_map(|n| check_node(n, &schema, &known))
        .collect();
    all.extend(check_corpus(&[schema_doc, record_doc], &schema));
    all
}

const MATURED: &str = r#"
capability "narrowed" {
    doing "keep every guarantee paired with something that enforces it"
    matured "doing" {
        from "owns five families of script"
        to "keep every guarantee paired with something that enforces it"
        at "2026-08-22"
        because "the prior claim was a list rather than a consistency boundary"
        taught-by "ITER.260822.03"
    }
}
"#;

/// C1 — all four facts are readable from the record alone, with no file outside it named.
#[test]
fn a_matured_value_carries_what_it_was_and_what_taught_it() {
    assert!(violations(MATURED).is_empty(), "{:?}", violations(MATURED));

    let doc = parse(MATURED).expect("parses");
    let entry = doc.nodes()[0]
        .children()
        .and_then(|b| b.nodes().iter().find(|n| n.name().value() == "matured").cloned())
        .expect("the maturation");
    let body = entry.children().expect("a body");
    for field in ["from", "to", "at", "because", "taught-by"] {
        assert!(
            body.nodes().iter().any(|n| n.name().value() == field),
            "{field} is carried IN the record — nothing here points at a file that can be deleted"
        );
    }
    assert_eq!(
        entry.entries().first().and_then(|e| e.value().as_string()),
        Some("doing"),
        "the maturation is identified by the field it is about"
    );
}

/// C2 — a value disagreeing with its own maturation is refused, naming both.
///
/// Worse than not recording the change at all, because it reads as an account of what
/// happened.
#[test]
fn a_value_contradicting_its_own_maturation_is_refused() {
    let contradicted = MATURED.replace(
        r#"    doing "keep every guarantee paired with something that enforces it""#,
        r#"    doing "something else entirely""#,
    );
    let found: Vec<_> = violations(&contradicted)
        .into_iter()
        .filter(|v| v.refusal.rule() == "a-value-agrees-with-its-maturation")
        .collect();

    assert_eq!(found.len(), 1, "{found:?}");
    let message = found[0].refusal.message();
    assert!(message.contains("something else entirely"), "it names what is held: {message}");
    assert!(message.contains("keep every guarantee"), "and what was claimed: {message}");
}

/// C3 — a maturation naming nothing that taught it is refused.
///
/// A change citing nothing is a rewrite, which is the discipline a strategy revision and an
/// emergent persona are already held to.
#[test]
fn a_maturation_citing_nothing_is_refused() {
    let untaught = MATURED.replace("        taught-by \"ITER.260822.03\"\n", "");
    let found: Vec<_> = violations(&untaught)
        .into_iter()
        .filter(|v| v.refusal.message().contains("taught-by"))
        .collect();
    assert_eq!(found.len(), 1, "{found:?}");

    // And `from` is required for the same reason: a change that does not say what it was is
    // a new value wearing a change's clothes.
    let no_prior = MATURED.replace("        from \"owns five families of script\"\n", "");
    assert!(
        violations(&no_prior).iter().any(|v| v.refusal.message().contains("from")),
        "the half that gets lost is the half that makes the change legible"
    );
}

/// Several maturations of one field are legitimate — that is what iterating IS.
///
/// Only the latest is checked against the current value. A field that matured twice has a
/// stale first `to` by construction, and refusing it would make a chain of improvements look
/// like a defect. The earlier entries are history and history is the point.
#[test]
fn a_field_may_mature_more_than_once() {
    let twice = r#"
capability "twice" {
    doing "the third thing"
    matured "doing" {
        from "the first thing"
        to "the second thing"
        at "2026-08-20"
        because "the first was wrong"
        taught-by "ITER.1"
    }
    matured "doing" {
        from "the second thing"
        to "the third thing"
        at "2026-08-22"
        because "the second was not enough"
        taught-by "ITER.2"
    }
}
"#;
    let found: Vec<_> = violations(twice)
        .into_iter()
        .filter(|v| v.refusal.rule() == "a-value-agrees-with-its-maturation")
        .collect();
    assert!(
        found.is_empty(),
        "the chain is the record maturing forward, and only its head is checked: {found:?}"
    );

    // And the head is still checked: break the last `to` and it refuses.
    let head_wrong = twice.replace(r#"        to "the third thing""#, r#"        to "something else""#);
    assert_eq!(
        violations(&head_wrong)
            .iter()
            .filter(|v| v.refusal.rule() == "a-value-agrees-with-its-maturation")
            .count(),
        1
    );
}
