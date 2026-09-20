//! `TS.260823.09` — the least a repository must declare, computed from the schema.
//!
//! Twenty-seven kinds, a hundred and fifty-eight field names and fifty-nine rules is what an
//! adopter meets. `adopt-the-method` opened by teaching what a repository may never REDEFINE
//! — the right rule, and an answer to a question nobody has yet. The question they DO have is
//! *what is the least I must declare before this thing will check*, and it had no answer
//! anywhere, so the rational move was to copy an existing record and mutate it. That is
//! transcription, which this method spends its longest section arguing against, and the
//! vocabulary's size is what made it the sensible choice.

use praxis_core::{Schema, parse};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "config" {
            field "repository" each="1"
        }
        entity "thin-slice" {
            field "realizes"   each="1"   references="capability"
            field "from-storm" each="1"   references="event-storm"
            field "attacks"    each="0..n" references="symptom"
        }
        entity "capability" {
            field "derived-from" each="1" references="event-storm"
        }
        entity "event-storm" {
            field "frame" each="1" references="frame"
        }
        entity "frame" {
            field "slug" each="1"
        }
        entity "adr" when="when a decision will outlive the iteration that made it" {
            field "body" each="1"
        }
        entity "walkthrough" {
            field "slug" each="1"
        }
    }
}
"##;

fn schema() -> Schema {
    Schema::from_document(&parse(SCHEMA).expect("the schema parses"))
}

/// C1. Computed by following required references from the unit of work, never listed.
#[test]
fn the_minimum_is_the_required_closure_from_a_slice() {
    assert_eq!(
        schema().minimum_to_start(),
        vec!["capability", "config", "event-storm", "frame", "thin-slice"],
        "a slice must name a capability, a capability the storm it came from, a storm its \
         frame — plus the config that binds the repository to the method"
    );
}

/// An OPTIONAL reference is a thing you may add, not a thing without which nothing checks.
/// `attacks` points at a symptom and is `0..n`, so a symptom is not in the minimum.
#[test]
fn an_optional_reference_is_not_part_of_the_minimum() {
    assert!(
        !schema().minimum_to_start().iter().any(|k| k == "symptom"),
        "an optional edge does not make its target required"
    );
}

/// C1 again, and the reason it is computed: a kind gaining a required edge changes the
/// answer with nothing to update. A hand-written minimum is a second copy of the schema.
#[test]
fn a_new_required_edge_enters_the_minimum_with_no_other_change() {
    let grown = SCHEMA.replace(
        r#"            field "attacks"    each="0..n" references="symptom""#,
        r#"            field "attacks"    each="1..n" references="symptom"
            field "under"      each="1"    references="strategy""#,
    ) + "";
    let doc = parse(&grown.replace(
        r#"        entity "walkthrough" {"#,
        r#"        entity "strategy" { field "serves" each="1" }
        entity "symptom" { field "text" each="1" }
        entity "walkthrough" {"#,
    ))
    .expect("parses");
    let minimum = Schema::from_document(&doc).minimum_to_start();
    assert!(minimum.iter().any(|k| k == "symptom"), "{minimum:?}");
    assert!(minimum.iter().any(|k| k == "strategy"), "{minimum:?}");
}

/// C3. A kind outside the minimum says when it arrives, or is reported.
///
/// An adopter meeting a kind with no stated trigger re-derives what it is for — every time,
/// in every repository, and never into the record.
#[test]
fn a_kind_outside_the_minimum_names_the_moment_that_calls_for_it() {
    let vocabulary = schema().vocabulary();
    let stated: Vec<&str> = vocabulary
        .iter()
        .filter(|(_, _, when)| when.is_some())
        .map(|(kind, _, _)| kind.as_str())
        .collect();
    assert_eq!(stated, vec!["adr"], "the one kind that declares a trigger");

    let unstated: Vec<&str> = vocabulary
        .iter()
        .filter(|(kind, _, when)| when.is_none() && kind.as_str() == "walkthrough")
        .map(|(kind, _, _)| kind.as_str())
        .collect();
    assert_eq!(
        unstated,
        vec!["walkthrough"],
        "and one that does not, which the view reports rather than guessing at"
    );
}
