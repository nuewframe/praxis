//! `TS.260821.11` — answer a question without knowing what the record is about.
//!
//! Cut from a usage log: eight questions in one session were asked with `grep` because the
//! CLI could not answer them, and every one traversed a graph the engine already held.
//!
//! The shape is forced by A4. `praxis unbound-iterations` would encode a kind and a field the
//! record declares, and would answer nothing in a repository whose kinds are `cohort` and
//! `experiment` — which is what the first test here is.

use praxis_core::ask::{Answer, Question, ask};
use praxis_core::{Schema, parse};

/// A schema with nothing this repository has ever declared.
const ELSEWHERE: &str = r##"
notional-architecture "NA.acme" {
    schema {
        entity "cohort" {
            field "slug"    each="1"
            field "channel" each="1"
            field "size"    each="0..1"
        }
        entity "experiment" {
            field "slug"      each="1"
            field "on-cohort" each="0..n"
        }
        entity "receipt" {
            field "line" each="0..n"
        }
        entity "line" {
            field "sku" each="1"
        }
    }
}
"##;

const RECORD: &str = r#"
cohort "COH.1" {
    slug "weekday-mornings"
    channel "email"
    size "1200"
}
cohort "COH.2" {
    slug "weekend-evenings"
    channel "push"
}
cohort "COH.3" {
    slug "never-tested"
    channel "email"
}
experiment "EXP.1" {
    slug "subject-line"
    on-cohort "COH.1"
    on-cohort "COH.2"
}
receipt "REC.1" {
    line "L.1" {
        sku "abc"
    }
}
"#;

fn answer(question: Question) -> Answer {
    let schema_doc = parse(ELSEWHERE).expect("schema parses");
    let record_doc = parse(RECORD).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    ask(&[schema_doc, record_doc], &schema, &question)
}

fn about(kind: &str) -> Question {
    Question { kind: kind.to_owned(), ..Question::default() }
}

/// C1 — the same code path answers over kinds this engine has never heard of.
#[test]
fn it_answers_over_a_vocabulary_this_repository_does_not_have() {
    let Answer::Rows { columns, rows } = answer(Question {
        where_: vec![("channel".to_owned(), "email".to_owned())],
        show: vec!["slug".to_owned()],
        ..about("cohort")
    }) else {
        panic!("cohort is declared by the record, which is all the engine needs")
    };

    assert_eq!(columns, ["cohort", "slug"]);
    assert_eq!(rows, [["COH.1", "weekday-mornings"], ["COH.3", "never-tested"]]);
}

/// C2 — an undeclared kind or field is refused, and the refusal says what IS declared.
///
/// Refused rather than answered empty: a filter on a field that does not exist matches
/// nothing, and looks exactly like a filter that matched nothing.
#[test]
fn an_undeclared_kind_or_field_is_refused_and_names_what_is_declared() {
    let Answer::Refused(why) = answer(about("iteration")) else {
        panic!("`iteration` is this repository's word, not acme's")
    };
    assert!(why.contains("cohort"), "it names what the record does hold: {why}");

    let Answer::Refused(why) = answer(Question {
        where_: vec![("colour".to_owned(), "blue".to_owned())],
        ..about("cohort")
    }) else {
        panic!("a filter on a field that does not exist must refuse")
    };
    assert!(why.contains("channel"), "it names the fields that exist: {why}");
}

/// C3 — the count is the row count of the same question, and nothing else.
#[test]
fn the_count_is_the_rows_of_the_same_question() {
    let filtered = Question {
        where_: vec![("channel".to_owned(), "email".to_owned())],
        ..about("cohort")
    };
    assert_eq!(answer(filtered.clone()).count(), 2);
    assert_eq!(answer(about("cohort")).count(), 3);
    // A predicate that legitimately matches nothing answers zero rather than refusing.
    assert_eq!(
        answer(Question {
            where_: vec![("channel".to_owned(), "carrier-pigeon".to_owned())],
            ..about("cohort")
        })
        .count(),
        0
    );
}

/// C4 — the anti-join, through an edge the schema declares. This is the session's own grep:
/// closed iterations no release binds.
#[test]
fn unreferenced_by_follows_a_declared_edge() {
    let Answer::Rows { rows, .. } = answer(Question {
        unreferenced_by: Some(("experiment".to_owned(), "on-cohort".to_owned())),
        ..about("cohort")
    }) else {
        panic!("experiment.on-cohort is declared")
    };
    assert_eq!(rows, [["COH.3"]], "only the cohort no experiment points at");

    // An edge the schema does not name is the engine guessing what relates to what.
    let Answer::Refused(why) = answer(Question {
        unreferenced_by: Some(("experiment".to_owned(), "invented".to_owned())),
        ..about("cohort")
    }) else {
        panic!("an undeclared edge has nothing to follow")
    };
    assert!(why.contains("not an edge this record declares"), "{why}");
}

/// A kind that exists only nested is refused rather than answered zero.
///
/// Nested traversal is excluded by this slice. Answering `0` as though it were the count is
/// not: it is an empty answer that reads as the truth, which is what refusing an undeclared
/// field exists to prevent.
#[test]
fn a_kind_that_is_only_ever_nested_says_so_instead_of_answering_zero() {
    let Answer::Refused(why) = answer(about("line")) else {
        panic!("`line` appears only inside a receipt")
    };
    assert!(why.contains("only nested"), "{why}");
    assert!(why.contains("cannot see them"), "and says what 0 would have meant: {why}");

    // A root kind with no records at all is a real zero, not a blind spot.
    let empty = parse("notional-architecture \"NA.empty\" {\n    schema {\n        entity \"cohort\" {\n            field \"slug\" each=\"1\"\n        }\n    }\n}\n").expect("parses");
    let schema = Schema::from_document(&empty);
    assert_eq!(ask(&[empty], &schema, &about("cohort")).count(), 0);
}
