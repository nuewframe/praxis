//! `TS.260821.13` — take a cut back in one command.
//!
//! The index points at a commit and never mirrors it, which is what makes cutting cheap.
//! Promotion breaks the symmetry by writing derived truth into records the cut does not own,
//! and the cost lands on whoever moves the pointer back: withdrawing `0.8.0` by hand meant
//! three edits across seven files, with `praxis check` inconsistent in between.

use praxis_core::withdraw::{Withdrawal, withdraw};
use praxis_core::{Corpus, Schema, parse};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "release" states="planned released withdrawn" {
            field "version" each="1"
            field "state"   each="1" one-of="planned" "released" "withdrawn"
            field "binds"   each="0..n"
        }
        entity "capability" {
            field "state"   each="1"
            field "shipped" each="0..n"
        }
        rule "a-state-vocabulary-is-declared-once" refuses="a kind declaring its states twice, differently"
    }
}
"##;

const RECORD: &str = r#"
release "REL.0.1.0" {
    version "0.1.0"
    state "released"
    binds "ITER.1"
}
release "REL.0.2.0" {
    version "0.2.0"
    state "released"
    binds "ITER.2"
}
capability "promoted-from-both" {
    state "active"
    shipped "0.1.0" by="ITER.1" slice="TS.a"
    shipped "0.2.0" by="ITER.2" slice="TS.b"
}
capability "promoted-from-neither" {
    state "sought"
}
"#;

fn corpus() -> Corpus {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(RECORD).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    Corpus::from_documents(&[schema_doc, record_doc], &schema)
}

/// C2 — the promoted truth a withdrawn release produced comes back, and only that one's.
#[test]
fn withdrawing_unwinds_only_its_own_promotion() {
    let Withdrawal::Taken { version, unwound } =
        withdraw("0.2.0", Some("the artifacts it froze were not worth freezing"), &corpus())
    else {
        panic!("a cut release with a reason can be taken back");
    };

    assert_eq!(version, "0.2.0");
    assert_eq!(
        unwound,
        ["promoted-from-both"],
        "the capability promoted from 0.2.0 is unwound; the one promoted from nothing is not, \
         and entries from OTHER cut releases have to survive or withdrawing one version \
         rewrites the history of every other"
    );
}

/// C3 — a withdrawal with no reason is refused.
#[test]
fn a_withdrawal_with_no_reason_is_refused() {
    for reason in [None, Some(""), Some("   ")] {
        let outcome = withdraw("0.2.0", reason, &corpus());
        assert!(
            matches!(outcome, Withdrawal::Refused(_)),
            "a withdrawal nobody explained is a cut nobody can account for: {reason:?}"
        );
    }
    // And the refusal says how to fix it, rather than naming a rule.
    let Withdrawal::Refused(why) = withdraw("0.2.0", None, &corpus()) else { panic!() };
    assert!(why.contains("--because"), "{why}");
}

/// Only a cut release has a pointer to move. A planned one was never cut, and the two must
/// not read alike afterwards.
#[test]
fn a_version_that_was_never_cut_cannot_be_withdrawn() {
    let record = RECORD.replace("state \"released\"\n    binds \"ITER.2\"", "state \"planned\"");
    let schema_doc = parse(SCHEMA).expect("parses");
    let record_doc = parse(&record).expect("parses");
    let schema = Schema::from_document(&schema_doc);
    let corpus = Corpus::from_documents(&[schema_doc, record_doc], &schema);

    let Withdrawal::Refused(why) = withdraw("0.2.0", Some("a reason"), &corpus) else {
        panic!("a planned release has no pointer to take back");
    };
    assert!(why.contains("planned"), "{why}");

    let Withdrawal::Refused(why) = withdraw("9.9.9", Some("a reason"), &corpus) else { panic!() };
    assert!(why.contains("no release the record holds"), "{why}");
}

/// C5 — a kind declares its states once, or it refuses what it also permits.
///
/// `withdrawn` had to be added to `release` in two places and the first edit changed
/// nothing, because `states=` was declared and never read (`WALK.260822.02/AS7`).
#[test]
fn a_kind_declaring_its_states_twice_differently_is_refused() {
    let split = r##"
notional-architecture "NA.split" {
    schema {
        entity "thing" states="one two" {
            field "state" each="1" one-of="one" "three"
        }
    }
}
"##;
    let doc = parse(split).expect("parses");
    let schema = Schema::from_document(&doc);
    let found = praxis_core::check_corpus(&[doc], &schema);
    let split_vocab: Vec<_> = found
        .iter()
        .filter(|v| v.refusal.rule() == "a-state-vocabulary-is-declared-once")
        .collect();

    assert_eq!(split_vocab.len(), 1, "{found:?}");
    let message = split_vocab[0].refusal.message();
    assert!(message.contains("two"), "it names what is only in states=: {message}");
    assert!(message.contains("three"), "and what is only in the field: {message}");

    // Agreement is silent, and a kind declaring states in only one place is not split.
    let agreed = split.replace(r#"one-of="one" "three""#, r#"one-of="one" "two""#);
    let doc = parse(&agreed).expect("parses");
    let schema = Schema::from_document(&doc);
    assert!(
        praxis_core::check_corpus(&[doc], &schema)
            .iter()
            .all(|v| v.refusal.rule() != "a-state-vocabulary-is-declared-once")
    );
}
