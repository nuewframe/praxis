//! `TS.260820.17` — fold what shipped into what each capability says it is, in one
//! operation.
//!
//! Nothing promoted is hand-written. That is not a style preference: it is what makes
//! `C2` checkable at all. A hand-edit is not discouraged here, it is *recomputable*.

use praxis_core::{Corpus, Promotion, Schema, derive, parse, promote, undeclared_promotions};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug"     each="1"
            field "realizes" each="1"
        }
        entity "iteration" {
            field "on-slice" each="1"
            field "state"    each="1"
        }
        entity "release" {
            field "version" each="1"
            field "state"   each="1"
            field "binds"   each="0..n"
        }
        entity "capability" {
            field "state"   each="1"
            field "shipped" each="0..n"
        }
        rule "promoted-truth-is-derived" refuses="a capability whose promoted truth is not derived"
    }
}
"##;

const CUT: &str = r##"
thin-slice "TS.a" {
    slug "a-thing"
    realizes "CAP.alpha"
}
thin-slice "TS.b" {
    slug "b-thing"
    realizes "CAP.beta"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
}
iteration "ITER.2" {
    on-slice "TS.b"
    state "closed"
}
release "REL.0.1.0" {
    version "0.1.0"
    state "released"
    binds "ITER.1"
    binds "ITER.2"
}
capability "alpha" {
    state "sought"
}
capability "beta" {
    state "sought"
}
"##;

fn corpus_of(record: &str) -> Corpus {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    Corpus::from_documents(&[schema_doc, record_doc], &schema)
}

#[test]
fn c1_the_whole_change_set_is_computed_before_anything_is_applied() {
    // Atomicity starts here: `promote` returns every change as values, so there is no
    // shape in which half of them exist. A half-promoted record is worse than an
    // unpromoted one — it is wrong in a way nobody can see.
    let Promotion::Ready { changes, .. } = promote("0.1.0", &corpus_of(CUT)) else {
        panic!("expected a promotion")
    };
    assert_eq!(changes.len(), 2, "both capabilities move, or the caller has neither");
    assert!(changes.iter().all(|c| c.becomes_active), "both leave `sought`");
}

#[test]
fn c2_promoted_truth_is_derived_from_bound_work() {
    let derived = derive(&corpus_of(CUT));
    assert_eq!(derived["alpha"].len(), 1);
    assert_eq!(derived["alpha"][0].version, "0.1.0");
    assert_eq!(derived["alpha"][0].iteration, "ITER.1");
    assert_eq!(derived["alpha"][0].slice, "TS.a", "traced through the slice it realizes");
}

#[test]
fn c2_a_hand_edited_capability_is_refused() {
    // Somebody wrote a shipped line by hand. Nothing bound produces it.
    let record = CUT.replace(
        r#"capability "alpha" {
    state "sought"
}"#,
        r#"capability "alpha" {
    state "active"
    shipped "0.1.0" by="ITER.1" slice="TS.a"
    shipped "0.0.9" by="ITER.9" slice="TS.z"
}"#,
    );
    let wrong = undeclared_promotions(&corpus_of(&record));
    let alpha = wrong.iter().find(|(id, _, _)| id == "alpha").expect("alpha is refused");
    assert_eq!(alpha.1.len(), 2, "what the record claims");
    assert_eq!(alpha.2.len(), 1, "what it derives");
    // beta is in the list too, for the opposite reason — it has not been promoted at all.
    assert!(wrong.iter().any(|(id, found, _)| id == "beta" && found.is_empty()));
}

#[test]
fn c2_an_unpromoted_capability_is_also_a_mismatch() {
    // The rule runs in both directions: a capability that should have moved and did not
    // is as much a disagreement as one that moved further than it should.
    let wrong = undeclared_promotions(&corpus_of(CUT));
    assert_eq!(wrong.len(), 2, "neither has been promoted yet");
    assert!(wrong.iter().all(|(_, found, derived)| found.is_empty() && derived.len() == 1));
}

#[test]
fn c3_promoting_twice_is_a_no_op() {
    let record = CUT
        .replace(
            r#"capability "alpha" {
    state "sought"
}"#,
            r#"capability "alpha" {
    state "active"
    shipped "0.1.0" by="ITER.1" slice="TS.a"
}"#,
        )
        .replace(
            r#"capability "beta" {
    state "sought"
}"#,
            r#"capability "beta" {
    state "active"
    shipped "0.1.0" by="ITER.2" slice="TS.b"
}"#,
        );
    let corpus = corpus_of(&record);
    assert!(undeclared_promotions(&corpus).is_empty(), "the record already agrees");
    assert_eq!(
        promote("0.1.0", &corpus),
        Promotion::AlreadyPromoted { version: "0.1.0".to_owned() },
        "not a double application, and not a refusal either"
    );
}

#[test]
fn c3_an_interrupted_promotion_is_repaired_by_running_it_again() {
    // alpha moved, beta did not. Promoting again writes the WHOLE derivation, so the
    // second run finishes what the first started rather than compounding it.
    let record = CUT.replace(
        r#"capability "alpha" {
    state "sought"
}"#,
        r#"capability "alpha" {
    state "active"
    shipped "0.1.0" by="ITER.1" slice="TS.a"
}"#,
    );
    let Promotion::Ready { changes, .. } = promote("0.1.0", &corpus_of(&record)) else {
        panic!("expected the remaining change")
    };
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].capability, "beta");
}

#[test]
fn promoting_a_release_that_is_not_cut_is_refused() {
    let record = CUT.replace(r#"state "released""#, r#"state "planned""#);
    let Promotion::Refused(why) = promote("0.1.0", &corpus_of(&record)) else {
        panic!("promotion is part of releasing, not a follow-up")
    };
    assert!(why[0].contains("not cut"));
}

#[test]
fn only_cut_releases_contribute_to_derived_truth() {
    let record = format!(
        "{CUT}\n{}",
        r##"
thin-slice "TS.c" {
    slug "c-thing"
    realizes "CAP.alpha"
}
iteration "ITER.3" {
    on-slice "TS.c"
    state "closed"
}
release "REL.0.2.0" {
    version "0.2.0"
    state "planned"
    binds "ITER.3"
}
"##
    );
    let derived = derive(&corpus_of(&record));
    assert_eq!(derived["alpha"].len(), 1, "0.2.0 is planned, so it has shipped nothing");
}

#[test]
fn a_capability_nothing_shipped_for_stays_where_it_was() {
    let record = format!("{CUT}\ncapability \"gamma\" {{\n    state \"sought\"\n}}\n");
    let Promotion::Ready { changes, .. } = promote("0.1.0", &corpus_of(&record)) else {
        panic!("expected a promotion")
    };
    assert!(
        !changes.iter().any(|c| c.capability == "gamma"),
        "promotion moves what shipped, and nothing shipped for gamma"
    );
}

#[test]
fn the_command_and_the_check_share_one_definition_of_truth() {
    // They call the same function. There is no second place to disagree from.
    let corpus = corpus_of(CUT);
    let derived = derive(&corpus);
    let Promotion::Ready { changes, .. } = promote("0.1.0", &corpus) else { panic!("expected") };
    for change in &changes {
        assert_eq!(&change.shipped, &derived[&change.capability]);
    }
}
