//! `TS.260820.09` — cut the version, and record the index that points into git.
//!
//! A release without an index is a release nothing can be verified against. Everything
//! downstream — publication, verification, symptom resolution — pins to this point.

use praxis_core::{Ask, Corpus, Cut, Schema, cut, parse, seal};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "iteration" {
            field "on-slice"    each="1"
            field "state"       each="1"
            field "contributes" each="0..n"
        }
        entity "release" {
            field "version"  each="1"
            field "state"    each="1"
            field "binds"    each="0..n"
            field "resolves" each="0..n"
            field "index"    each="0..1"
            field "seal"     each="0..1"
        }
        entity "frame" {
            field "symptom" each="0..n" holds="symptom"
        }
        entity "symptom" {
        }
    }
}
"##;

const CONFIG: &str = r##"
config {
    versioning {
        scheme "semver"
        bump-proposal {
            slice-outcome-added "minor"
        }
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

fn corpus_of(record: &str) -> Corpus {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let config_doc = parse(CONFIG).expect("config parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    Corpus::from_documents(&[schema_doc, config_doc, record_doc], &schema)
}

const READY_TO_CUT: &str = r##"
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    contributes "slice-outcome-added"
}
release "REL.0.9.0" {
    version "0.9.0"
    state "planned"
    binds "ITER.1"
}
"##;

const COMMIT: &str = "9f1c0a7e2b3d4f5061728394a5b6c7d8e9f0a1b2";

#[test]
fn c1_cutting_with_an_unclosed_bound_iteration_is_refused_naming_it() {
    let corpus = corpus_of(
        r##"
iteration "ITER.1" {
    on-slice "TS.a"
    state "working"
}
release "REL.0.9.0" {
    version "0.9.0"
    state "planned"
    binds "ITER.1"
}
"##,
    );
    let Cut::Refused(record) = cut("0.9.0", COMMIT, true, &corpus, &ask(), &[]) else {
        panic!("work still in flight must not be cut into a release");
    };
    assert!(record.failed.iter().any(|(c, w)| c == "only-closed-work-is-cut" && w.contains("ITER.1")));
    assert!(!record.kdl.contains(r#"state "released""#), "and nothing is cut");
}

#[test]
fn c2_the_cut_and_the_index_are_one_record() {
    // Atomicity by construction: the whole record is composed before anything is written,
    // so there IS no half-cut state. A failure to write leaves the planned record intact
    // because the planned record was never modified.
    let Cut::Made(record) = cut("0.9.0", COMMIT, true, &corpus_of(READY_TO_CUT), &ask(), &[]) else {
        panic!("expected a cut")
    };
    let doc = parse(&record.kdl).expect("valid KDL");
    let node = doc.nodes().iter().find(|n| n.name().value() == "release").expect("a release");

    let state = node.iter_children().find(|c| c.name().value() == "state");
    let index = node.iter_children().find(|c| c.name().value() == "index");
    assert!(state.is_some() && index.is_some(), "neither exists without the other");
    assert!(
        record.kdl.matches("release \"REL.0.9.0\"").count() == 1,
        "one record, composed whole"
    );

    let index = index.expect("an index");
    for required in ["tag", "commit", "cut-at", "cut-by", "confirmed-bump"] {
        assert!(
            index.iter_children().any(|c| c.name().value() == required),
            "the index carries `{required}`"
        );
    }
}

#[test]
fn c3_the_index_records_the_commit_it_was_cut_at() {
    // The half that is settleable here. That the commit CONTAINS the release's published
    // directory cannot be shown until publishing exists — TS.260820.10, which depends on
    // this slice. Carried as X1.
    let Cut::Made(record) = cut("0.9.0", COMMIT, true, &corpus_of(READY_TO_CUT), &ask(), &[]) else {
        panic!("expected a cut")
    };
    assert!(record.kdl.contains(&format!("commit {COMMIT:?}")));
    assert!(record.kdl.contains(r#"tag "v0.9.0""#), "the tag is derived from the version");
    assert!(
        !record.kdl.contains("message") && !record.kdl.contains("diff"),
        "the index POINTS at a commit and never mirrors what git already holds"
    );
}

#[test]
fn c4_editing_a_cut_release_changes_its_seal() {
    let Cut::Made(record) = cut("0.9.0", COMMIT, true, &corpus_of(READY_TO_CUT), &ask(), &[]) else {
        panic!("expected a cut")
    };
    let written = seal("0.9.0", COMMIT, &["ITER.1".to_owned()], &[]);
    assert!(record.kdl.contains(&written), "the seal is over the content as cut");

    // Every field the seal covers moves it.
    assert_ne!(written, seal("0.9.1", COMMIT, &["ITER.1".to_owned()], &[]));
    assert_ne!(written, seal("0.9.0", "0000000", &["ITER.1".to_owned()], &[]));
    assert_ne!(written, seal("0.9.0", COMMIT, &["ITER.2".to_owned()], &[]));
    assert_ne!(
        written,
        seal("0.9.0", COMMIT, &["ITER.1".to_owned(), "ITER.2".to_owned()], &[]),
        "adding a binding after the cut is exactly what this catches"
    );
    assert_ne!(written, seal("0.9.0", COMMIT, &["ITER.1".to_owned()], &["S1".to_owned()]));
}

#[test]
fn the_seal_does_not_depend_on_the_order_things_were_bound_in() {
    // Otherwise a rewrite that reorders — which the bind command is free to do, since it
    // composes the record whole — would read as tampering.
    let a = seal("0.9.0", COMMIT, &["ITER.1".to_owned(), "ITER.2".to_owned()], &[]);
    let b = seal("0.9.0", COMMIT, &["ITER.2".to_owned(), "ITER.1".to_owned()], &[]);
    assert_eq!(a, b);
}

#[test]
fn a_release_is_cut_once() {
    let corpus = corpus_of(
        r##"
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
}
release "REL.0.9.0" {
    version "0.9.0"
    state "released"
    binds "ITER.1"
}
"##,
    );
    let Cut::Refused(record) = cut("0.9.0", COMMIT, true, &corpus, &ask(), &[]) else {
        panic!("a release is a point, not a range")
    };
    assert!(record.failed.iter().any(|(c, _)| c == "a-release-is-cut-once"));
}

#[test]
fn a_version_binding_nothing_is_not_a_release() {
    let corpus = corpus_of(
        r##"
release "REL.0.9.0" {
    version "0.9.0"
    state "planned"
}
"##,
    );
    let Cut::Refused(record) = cut("0.9.0", COMMIT, true, &corpus, &ask(), &[]) else {
        panic!("a point on the line that means nothing is not worth having")
    };
    assert!(record.failed.iter().any(|(c, _)| c == "a-release-has-content"));
}

#[test]
fn the_record_proposes_and_a_human_confirms() {
    let Cut::Refused(record) = cut("0.9.0", COMMIT, false, &corpus_of(READY_TO_CUT), &ask(), &[])
    else {
        panic!("an unconfirmed bump must not be cut on the record's own say-so")
    };
    assert!(record.failed[0].1.contains("minor"), "and it says what it proposed");
    assert!(record.failed.iter().any(|(c, _)| c == "the-maintainer-confirms-the-version"));
}

#[test]
fn what_a_release_resolves_is_derived_from_the_symptoms_that_name_it() {
    let corpus = corpus_of(
        r##"
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
}
frame "F.1" {
    symptom "S3" state="resolved" resolved-by="0.9.0"
    symptom "S4" state="resolved" resolved-by="0.9.1"
}
release "REL.0.9.0" {
    version "0.9.0"
    state "planned"
    binds "ITER.1"
}
"##,
    );
    let Cut::Made(record) = cut("0.9.0", COMMIT, true, &corpus, &ask(), &[]) else {
        panic!("expected a cut")
    };
    assert!(record.kdl.contains(r#"resolves "S3""#));
    assert!(!record.kdl.contains(r#"resolves "S4""#), "a symptom naming another version is not ours");
}

#[test]
fn a_release_resolving_nothing_says_so() {
    let Cut::Made(record) = cut("0.9.0", COMMIT, true, &corpus_of(READY_TO_CUT), &ask(), &[]) else {
        panic!("expected a cut")
    };
    assert!(
        record.kdl.contains("Resolves nothing"),
        "a release that fixed no symptom and one nobody checked are not the same thing"
    );
}
