//! `TS.260821.20` — refuse an approach the record says was taken and was not.
//!
//! `implement-follows-an-approach` checked that implement NAMES an approach. It did not check
//! that the named one exists, so `followed="an approach nobody recorded"` passed — and a
//! design-system that produced three approaches while implement followed one left two
//! decisions the record claimed were taken and were not.
//!
//! `AZ2` asked whether the plan was written before the code. That is unverifiable and was
//! also the wrong question: the code IS the decision, the plan is the informed act before it,
//! and what matters is that the two agree.

use praxis_core::{Schema, check_corpus, parse};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "iteration" {
            field "on-slice" each="1..n"
            field "state"    each="1"
            field "phase"    each="0..n" holds="phase"
        }
        entity "phase" {
            field "state"    each="1"
            field "approach" each="0..n" holds="approach"
            field "followed" each="0..n"
        }
        entity "approach" {
            field "over"      each="1..n"
            field "because"   each="1"
            field "abandoned" each="0..1"
        }
        rule "the-plan-and-the-code-agree" refuses="a plan describing work nobody did"
    }
}
"##;

fn agreement(record: &str) -> Vec<String> {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    check_corpus(&[schema_doc, record_doc], &schema)
        .into_iter()
        .filter(|v| v.refusal.rule() == "the-plan-and-the-code-agree")
        .map(|v| v.refusal.message())
        .collect()
}

fn iteration(approaches: &str, implement: &str) -> String {
    format!(
        r#"
iteration "ITER.1" {{
    on-slice "TS.1"
    state "closed"
    phase "design-system" state="complete" {{
{approaches}
    }}
{implement}
}}
"#
    )
}

const ONE: &str = r#"        approach "the one that was written down" {
            over "something else"
            because "it seemed better"
        }"#;

/// C1 — a `followed` naming an approach the iteration never produced is refused.
///
/// This is the case that passed before the slice: an account of a choice nobody made, which a
/// reader has no reason to distrust.
#[test]
fn following_an_approach_nobody_recorded_is_refused() {
    let found = agreement(&iteration(
        ONE,
        "    phase \"implement\" state=\"complete\" followed=\"an approach nobody recorded\"",
    ));

    assert_eq!(found.len(), 2, "the invented one, and the real one left untaken: {found:?}");
    assert!(
        found.iter().any(|m| m.contains("an approach nobody recorded")
            && m.contains("worse than no plan")),
        "{found:?}"
    );
}

/// C2 — an approach neither followed nor abandoned is refused, by name.
#[test]
fn an_approach_that_was_planned_and_not_taken_is_named() {
    let two = format!(
        "{ONE}\n        approach \"the one that was quietly dropped\" {{\n\
         \x20           over \"the first one\"\n            because \"it looked cheaper\"\n        }}"
    );
    let found = agreement(&iteration(
        &two,
        "    phase \"implement\" state=\"complete\" {\n        followed \"the one that was written down\"\n    }",
    ));

    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("the one that was quietly dropped"), "{found:?}");
    assert!(found[0].contains("`abandoned`"), "and it says how to fix it: {found:?}");
}

/// C3 — abandoning is a recorded act, and the approach stays in the record.
///
/// Building teaches things a plan cannot know. A rule with no exit would teach people to
/// record only what they had already built, which is the behaviour `TS.260821.19` exists to
/// fix — so the exit is cheap, and the alternative considered stays where it is worth
/// something.
#[test]
fn an_abandoned_approach_satisfies_the_rule_and_remains() {
    let two = format!(
        "{ONE}\n        approach \"the one that did not survive contact\" {{\n\
         \x20           over \"the first one\"\n            because \"it looked cheaper\"\n\
         \x20           abandoned \"the schema could not express it without a second kind\"\n        }}"
    );
    let record = iteration(
        &two,
        "    phase \"implement\" state=\"complete\" {\n        followed \"the one that was written down\"\n    }",
    );
    assert!(agreement(&record).is_empty(), "{:?}", agreement(&record));
    assert!(
        record.contains("the one that did not survive contact"),
        "an abandoned approach is not deleted — the alternative considered is why the plan was \
         worth writing"
    );
}

/// Several approaches, all followed, is silent — and `followed` must be child nodes.
///
/// KDL keeps only the LAST of a repeated property, so four `followed="…"` properties on one
/// node record one approach and lose three. `ITER.260822.14` claimed four and held one
/// (`BB1`).
#[test]
fn followed_is_a_repeatable_child_and_not_a_repeated_property() {
    let two = format!(
        "{ONE}\n        approach \"the second one\" {{\n\
         \x20           over \"doing it all at once\"\n            because \"it splits cleanly\"\n        }}"
    );
    let as_children = iteration(
        &two,
        "    phase \"implement\" state=\"complete\" {\n\
         \x20       followed \"the one that was written down\"\n\
         \x20       followed \"the second one\"\n    }",
    );
    assert!(agreement(&as_children).is_empty(), "{:?}", agreement(&as_children));

    let as_properties = iteration(
        &two,
        "    phase \"implement\" state=\"complete\" \\\n\
         \x20       followed=\"the one that was written down\" \\\n\
         \x20       followed=\"the second one\"",
    );
    assert_eq!(
        agreement(&as_properties).len(),
        1,
        "the collapsed one is caught rather than silently accepted — which is the whole reason \
         it is worth catching: {:?}",
        agreement(&as_properties)
    );
}
