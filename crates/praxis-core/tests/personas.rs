//! `TS.260821.15` — hold who the product is for, so value is claimed from their side.
//!
//! `needed-by` has been on every read model since the storm was written and named a role with
//! nothing behind it: no statement of who a reader IS, what they came for, or how they would
//! know they got it. So `useful-alone` — the field that literally says what you get — has
//! always had an unstated subject.

use praxis_core::{Known, Schema, Severity, check_corpus, check_node, index_all, parse};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "persona" states="primary emergent" {
            field "is"        each="1"
            field "came-for"  each="1"
            field "judges-by" each="1"
            field "state"     each="0..1" one-of="primary" "emergent"
            field "found-by"  each="1" when-state="emergent"
        }
        entity "walkthrough" {
            field "slug" each="1"
        }
        entity "event-storm" {
            field "read-model" each="0..n"
        }
        rule "a-record-names-somebody-it-is-for" reports="a record naming nobody"
    }
}
"##;

const PRIMARY: &str = r#"
persona "the-team" {
    is "engineers, designers and product managers working through an agent"
    came-for "delivery they can trust without re-reading everything"
    judges-by "they can pick up work somebody else left, without asking them"
    state "primary"
}
"#;

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

/// C1 — a persona is a claim, not a description. `judges-by` is the field that makes it one.
#[test]
fn a_persona_nobody_could_be_wrong_about_is_refused() {
    let no_test = r#"
persona "the-team" {
    is "engineers and designers"
    came-for "delivery they can trust"
    state "primary"
}
"#;
    let found = violations(no_test);
    assert!(
        found.iter().any(|v| v.refusal.rule() == "entity-without-a-required-field"
            && v.refusal.message().contains("judges-by")),
        "a persona nobody can be wrong about is a description: {found:?}"
    );
}

/// C5 — one primary persona is required and the rest are not. Emergence is the normal case,
/// not a shortfall.
#[test]
fn one_persona_is_enough_and_none_is_reported() {
    let with_one: Vec<_> = violations(PRIMARY)
        .into_iter()
        .filter(|v| v.refusal.rule() == "a-record-names-somebody-it-is-for")
        .collect();
    assert!(with_one.is_empty(), "one primary persona is a complete answer: {with_one:?}");

    let none: Vec<_> = violations("walkthrough \"WALK.1\" {\n    slug \"a-walk\"\n}\n")
        .into_iter()
        .filter(|v| v.refusal.rule() == "a-record-names-somebody-it-is-for")
        .collect();
    assert_eq!(none.len(), 1, "a record naming nobody is reported");
    assert_eq!(
        none[0].severity(),
        Severity::Report,
        "reported, never refused — enumerating personas upfront is a week spent on people \
         nobody has met, and this must not push anyone toward doing that"
    );
}

/// C6 — an emergent persona names what surfaced it. Discovered means discovered BY something.
#[test]
fn an_emergent_persona_with_no_origin_is_refused() {
    let from_nowhere = format!(
        "{PRIMARY}\npersona \"appeared\" {{\n    is \"somebody\"\n    came-for \"something\"\n\
         \x20   judges-by \"some way of telling\"\n    state \"emergent\"\n}}\n"
    );
    let found = violations(&from_nowhere);
    assert!(
        found.iter().any(|v| v.refusal.rule() == "entity-without-a-required-field"
            && v.refusal.message().contains("found-by")),
        "a persona added later with no origin is indistinguishable from one somebody invented \
         to justify a decision: {found:?}"
    );

    // With an origin it is clean — and required only WHEN emergent, so the primary one above
    // needs none.
    let sourced = from_nowhere.replace(
        "    state \"emergent\"",
        "    state \"emergent\"\n    found-by \"WALK.1\"",
    );
    let sourced = format!("walkthrough \"WALK.1\" {{\n    slug \"a-walk\"\n}}\n{sourced}");
    assert!(
        !violations(&sourced)
            .iter()
            .any(|v| v.refusal.message().contains("found-by")),
        "an emergent persona citing a walkthrough is complete"
    );
}

/// C2 — `needed-by` names a persona the record holds, or it dangles. No second rule.
#[test]
fn a_view_needed_by_nobody_the_record_holds_dangles() {
    const REFERENCING: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "persona" {
            field "is"        each="1"
            field "came-for"  each="1"
            field "judges-by" each="1"
        }
        entity "event-storm" {
            field "read-model" each="0..n" holds="read-model"
        }
        entity "read-model" {
            field "needed-by" each="0..n" references="persona"
        }
    }
}
"##;
    let record = format!(
        "{PRIMARY}\nevent-storm \"ES.1\" {{\n    read-model \"a-view\" {{\n\
         \x20       needed-by \"nobody-declared\"\n    }}\n}}\n"
    );
    let schema_doc = parse(REFERENCING).expect("parses");
    let record_doc = parse(&record).expect("parses");
    let schema = Schema::from_document(&schema_doc);
    let mut known = Known::default();
    index_all(&schema_doc, &mut known);
    index_all(&record_doc, &mut known);

    let found: Vec<_> = record_doc
        .nodes()
        .iter()
        .flat_map(|n| check_node(n, &schema, &known))
        .filter(|v| v.refusal.rule() == "dangling-relationship")
        .collect();

    assert_eq!(found.len(), 1, "the label is now an edge, and an edge to nobody dangles: {found:?}");
}
