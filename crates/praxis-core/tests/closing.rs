//! `TS.260820.07` — refuse to close an iteration that is about to drop scope in silence.
//!
//! C3 is the one that matters. Every other check here is defeated by editing the claim
//! list, which is exactly how scope is dropped in a Markdown checklist today.

use praxis_core::{Ask, Closing, Corpus, Schema, close_iteration, parse};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug"  each="1"
            field "claim" each="0..n"
        }
        entity "iteration" {
            field "on-slice" each="1"
            field "state"    each="1"
            field "claim"    each="0..n"
            field "finding"  each="0..n"
        }
        rule "no-silent-drop" refuses="a closed iteration carrying an unaccounted claim"
        rule "claim-dropped-from-the-slice" refuses="an iteration carrying a claim its slice no longer declares"
    }
}
"##;

fn ask() -> Ask {
    Ask {
        signer: "human:someone".to_owned(),
        at: "2026-08-21T09:00:00Z".to_owned(),
        by: "agent:praxis".to_owned(),
    }
}

fn closing(record: &str, iteration: &str) -> Closing {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let docs = vec![schema_doc, record_doc];
    let corpus = Corpus::from_documents(&docs, &schema);
    close_iteration(iteration, &corpus, &ask(), &[])
}

/// A slice with two claims, and an iteration that met one of them.
const HALF_DONE: &str = r##"
thin-slice "TS.a" {
    slug "a"
    claim "C1"
    claim "C2"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "working"
    claim "C1" from-slice="TS.a" state="met"
    claim "C2" from-slice="TS.a" state="pending"
}
"##;

#[test]
fn c1_closing_with_an_unmet_claim_and_no_finding_is_refused_naming_the_claim() {
    let Closing::Refused(record) = closing(HALF_DONE, "ITER.1") else {
        panic!("a close with an unaccounted claim must be refused");
    };
    assert_eq!(record.failed.len(), 1);
    assert_eq!(record.failed[0].0, "C2", "the claim is named");
    assert!(record.failed[0].1.contains("no finding carries it"));
    assert!(record.kdl.contains("claim-C2-is-unaccounted"));
    assert!(record.kdl.contains(r#"on-iteration "ITER.1""#));
}

#[test]
fn c1_the_refusal_leaves_the_iteration_open() {
    let Closing::Refused(record) = closing(HALF_DONE, "ITER.1") else { panic!("refused") };
    assert!(!record.kdl.contains(r#"state "closed""#), "nothing closes");
    assert!(record.id.starts_with("REF."));
}

#[test]
fn c2_closing_with_an_unmet_claim_and_a_recorded_finding_succeeds() {
    let result = closing(
        r##"
thin-slice "TS.a" {
    slug "a"
    claim "C1"
    claim "C2"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "working"
    claim "C1" from-slice="TS.a" state="met"
    claim "C2" from-slice="TS.a" state="carried"
    finding "F1" carries="C2" text="the residue, and where it is owed"
}
"##,
        "ITER.1",
    );
    let Closing::Accepted { accounting, all_met, .. } = result else {
        panic!("a recorded shortfall is enough to close: {result:?}");
    };
    assert!(!all_met, "and the close says so rather than rounding up");
    assert_eq!(accounting.len(), 2);
    assert_eq!(accounting[1], ("C2".to_owned(), "carried by F1".to_owned()));
}

#[test]
fn c3_deleting_a_claim_to_make_a_close_succeed_is_itself_refused() {
    // The slice has been edited mid-iteration: C2 is gone from it. The iteration froze
    // C2 at open, so the deletion does not remove the claim — it makes the two disagree.
    let result = closing(
        r##"
thin-slice "TS.a" {
    slug "a"
    claim "C1"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "working"
    claim "C1" from-slice="TS.a" state="met"
    claim "C2" from-slice="TS.a" state="pending"
}
"##,
        "ITER.1",
    );
    let Closing::Refused(record) = result else {
        panic!("deleting a claim must not be a way to close: {result:?}");
    };
    assert_eq!(record.failed[0].0, "C2");
    assert!(
        record.failed[0].1.contains("no longer declares it"),
        "and it says what happened: {:?}",
        record.failed[0].1
    );
    assert!(record.failed[0].1.contains("TS.a"));
}

#[test]
fn a_finding_carrying_a_claim_the_iteration_does_not_hold_is_refused() {
    // Accounting that refers to nothing is not accounting.
    let result = closing(
        r##"
thin-slice "TS.a" {
    slug "a"
    claim "C1"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "working"
    claim "C1" from-slice="TS.a" state="met"
    finding "F1" carries="C9" text="carries something nobody claimed"
}
"##,
        "ITER.1",
    );
    let Closing::Refused(record) = result else { panic!("expected a refusal: {result:?}") };
    assert!(record.failed.iter().any(|(c, w)| c == "C9" && w.contains("does not hold")));
}

#[test]
fn every_claim_met_closes_clean() {
    let result = closing(
        r##"
thin-slice "TS.a" {
    slug "a"
    claim "C1"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "working"
    claim "C1" from-slice="TS.a" state="met"
}
"##,
        "ITER.1",
    );
    let Closing::Accepted { all_met, accounting, .. } = result else { panic!("{result:?}") };
    assert!(all_met);
    assert_eq!(accounting, vec![("C1".to_owned(), "met".to_owned())]);
}

#[test]
fn a_closed_iteration_is_history_and_does_not_reopen() {
    let result = closing(
        r##"
thin-slice "TS.a" {
    slug "a"
    claim "C1"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    claim "C1" from-slice="TS.a" state="met"
}
"##,
        "ITER.1",
    );
    let Closing::NotOpen(why) = result else { panic!("expected a refusal to reopen: {result:?}") };
    assert!(why.contains("a new iteration"), "{why}");
}

#[test]
fn a_claim_settled_for_another_slice_is_not_measured_against_this_one() {
    // ITER.260821.04 settled a claim belonging to TS.260820.04 while working on
    // TS.260820.05. That is how a residue gets settled, and C3 must not fire on it.
    let result = closing(
        r##"
thin-slice "TS.a" {
    slug "a"
    claim "C1"
}
thin-slice "TS.b" {
    slug "b"
    claim "C1"
}
iteration "ITER.1" {
    on-slice "TS.b"
    state "working"
    claim "C1" from-slice="TS.b" state="met"
    claim "C1" from-slice="TS.a" state="met"
}
"##,
        "ITER.1",
    );
    assert!(matches!(result, Closing::Accepted { .. }), "{result:?}");
}
