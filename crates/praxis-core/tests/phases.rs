//! `TS.260821.19` — make a phase produce something, before the next one may start.
//!
//! `phase` carried its kinds as a PROSE STRING and declared no fields, so every phase in this
//! frame was a `produced=` sentence written at close by whoever did the work. Read them and
//! every `design-system` is a past-tense summary of what was built — including one that was
//! wrong in exactly the way a design phase exists to catch.

use praxis_core::{Known, Schema, Severity, check_corpus, check_node, index_all, parse};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "persona" {
            field "is" each="1"
        }
        entity "iteration" {
            field "on-slice" each="1..n"
            field "state"    each="1"
            field "phase"    each="0..n" holds="phase"
        }
        entity "phase" states="pending active complete skipped" {
            field "state"    each="1" one-of="pending" "active" "complete" "skipped"
            field "approach" each="0..n" holds="approach"
            field "followed" each="0..n"
            field "taught"   each="0..n" references="persona"
            field "produced" each="0..1"
        }
        entity "approach" {
            field "over"    each="1..n"
            field "because" each="1"
        }
        rule "implement-follows-an-approach" refuses="an implement that did not read its plan"
        rule "teach-reaches-an-end-user"     reports="a teach naming no reader"
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

fn planned(implement: &str, teach: &str) -> String {
    format!(
        r#"
persona "the-reader" {{
    is "somebody who has read nothing else"
}}
iteration "ITER.1" {{
    on-slice "TS.1"
    state "closed"
    phase "design-system" state="complete" {{
        approach "carry the plan in the iteration" {{
            over "a design document per slice"
            because "the approach belongs where the claims it will settle already are"
        }}
    }}
{implement}
{teach}
}}
"#
    )
}

/// C1 — an approach names what it was chosen over. A choice with nothing rejected is a
/// description of the only thing anybody thought of.
#[test]
fn an_approach_with_no_alternative_is_refused() {
    let record = planned("", "").replace(
        "            over \"a design document per slice\"\n",
        "",
    );
    assert!(
        violations(&record).iter().any(|v| v.refusal.message().contains("over")),
        "{:?}",
        violations(&record)
    );
}

/// C2 — implement complete against a design-system that produced an approach it does not
/// name is refused.
#[test]
fn implementing_without_naming_the_approach_is_refused() {
    let record = planned("    phase \"implement\" state=\"complete\"", "");
    let found: Vec<_> = violations(&record)
        .into_iter()
        .filter(|v| v.refusal.rule() == "implement-follows-an-approach")
        .collect();

    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0].severity(), Severity::Refuse);

    // Naming it is enough — this refuses building against no plan, not building without
    // permission. Who may ATTEST is a different rule.
    let followed = planned(
        "    phase \"implement\" state=\"complete\" followed=\"carry the plan in the iteration\"",
        "",
    );
    assert!(
        !violations(&followed)
            .iter()
            .any(|v| v.refusal.rule() == "implement-follows-an-approach")
    );

    // And a design-system that produced no approach does not force one: a slice whose layers
    // do not need the phase skips it, with a reason.
    let unplanned = r#"
iteration "ITER.2" {
    on-slice "TS.1"
    state "closed"
    phase "implement" state="complete"
}
"#;
    assert!(
        !violations(unplanned)
            .iter()
            .any(|v| v.refusal.rule() == "implement-follows-an-approach")
    );
}

/// C4 — a teach phase names who it reached.
///
/// It names the READER rather than sniffing the path. `produced` containing `skills/` was a
/// heuristic on a string, and would have called a correct teach phase wrong the moment
/// somebody wrote a guide somewhere else.
#[test]
fn a_teach_phase_that_names_no_reader_is_reported() {
    let record = planned(
        "    phase \"implement\" state=\"complete\" followed=\"carry the plan in the iteration\"",
        "    phase \"teach\" state=\"complete\" produced=\"a skill\"",
    );
    let found: Vec<_> = violations(&record)
        .into_iter()
        .filter(|v| v.refusal.rule() == "teach-reaches-an-end-user")
        .collect();

    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(
        found[0].severity(),
        Severity::Report,
        "every teach phase in the record named a skill on the day this landed; a rule failing \
         closed on arrival names thirty-three"
    );

    let named = record.replace(
        "phase \"teach\" state=\"complete\" produced=\"a skill\"",
        "phase \"teach\" state=\"complete\" produced=\"a skill\" taught=\"the-reader\"",
    );
    assert!(
        !violations(&named)
            .iter()
            .any(|v| v.refusal.rule() == "teach-reaches-an-end-user")
    );
}

/// C5 — the phase vocabulary is checked rather than described.
///
/// `state` was carried as a PROPERTY on every phase in the record, and the schema could only
/// describe child nodes — which is why `phase` was called "deliberately shapeless". It was
/// not a choice about phases; it was the schema having no way to describe them.
#[test]
fn a_phase_is_shaped_whether_its_fields_are_nodes_or_properties() {
    let as_property = r#"
iteration "ITER.3" {
    on-slice "TS.1"
    state "closed"
    phase "implement" state="complete"
}
"#;
    // Filtered to the phase rules: this fixture declares no persona, and
    // `a-record-names-somebody-it-is-for` correctly reports that — a different subject.
    let about_phases: Vec<_> = violations(as_property)
        .into_iter()
        .filter(|v| v.entity_kind == "phase" || v.refusal.message().contains("phase"))
        .collect();
    assert!(about_phases.is_empty(), "{about_phases:?}");

    let no_state = r#"
iteration "ITER.4" {
    on-slice "TS.1"
    state "closed"
    phase "implement"
}
"#;
    assert!(
        violations(no_state).iter().any(|v| v.refusal.message().contains("state")),
        "a phase with no state is now refusable, which it was not before: {:?}",
        violations(no_state)
    );
}
