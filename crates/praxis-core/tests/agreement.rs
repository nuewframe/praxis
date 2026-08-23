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
/// Both forms record every value (`TS.260823.04`/`D1`).
///
/// This test used to assert the opposite: that repeated `followed="…"` properties collapse
/// to one, and that the collapse is *caught rather than silently accepted*. The premise was
/// half right — `node.get()` returns one of them — but the document keeps every entry, and
/// `all_props` reads them all. So the collapse was never in KDL; it was in the one accessor
/// the rule happened to use.
///
/// `D1` decided that a field is read in whichever form it was written, over declaring a form
/// per field and refusing a mismatch. Catching the property form is what that decision
/// rejects: the author who writes four properties means four, and a method that refuses them
/// is teaching syntax it has no reason to care about. `BB1` — an iteration that claimed four
/// approaches and held one — is fixed by reading all four, not by refusing the line.
#[test]
fn followed_records_every_value_in_either_form() {
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
    assert!(
        agreement(&as_properties).is_empty(),
        "four properties mean four approaches, the same as four child nodes — the author who \
         writes one form should not be refused for not writing the other: {:?}",
        agreement(&as_properties)
    );
}
