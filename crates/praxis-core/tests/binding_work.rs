//! `TS.260820.16` — attach finished iterations to a version, and refuse anything that did
//! not close.
//!
//! What a version contains becomes a fact about the record rather than a list somebody
//! keeps alongside it. A hand-kept list and a derived one look identical right up until
//! they disagree, and afterwards nobody can tell which was right.

use praxis_core::{Ask, Binding, Corpus, Schema, bind, parse, propose, unbind};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "iteration" {
            field "on-slice"    each="1"
            field "state"       each="1"
            field "claim"       each="0..n"
            field "contributes" each="0..n"
        }
        entity "release" {
            field "version" each="1"
            field "state"   each="1"
            field "binds"   each="0..n"
        }
    }
}
"##;

/// The bump rules as this repository binds them. Praxis is pre-1.0 and bumps MINOR for a
/// breaking change — a hardcoded semver rule would misreport its own author's releases.
const CONFIG: &str = r##"
config {
    versioning {
        scheme "semver"
        bump-proposal {
            breaking-seam-contract "minor"
            capability-added       "minor"
            slice-outcome-added    "minor"
            gap-remediation-only   "patch"
            doctrine-only          "patch"
        }
    }
}
"##;

fn ask() -> Ask {
    Ask {
        signer: "human:someone".to_owned(),
        at: "2026-08-21T09:00:00Z".to_owned(),
        by: "agent:praxis".to_owned(), attested_by: None,
    }
}

fn corpus_of(record: &str) -> Corpus {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let config_doc = parse(CONFIG).expect("config parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    Corpus::from_documents(&[schema_doc, config_doc, record_doc], &schema)
}

const CLOSED: &str = r##"
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    claim "C1" from-slice="TS.a" state="met"
    contributes "slice-outcome-added"
}
"##;

#[test]
fn c1_binding_an_unclosed_iteration_is_refused_naming_it_and_its_unsettled_claims() {
    let corpus = corpus_of(
        r##"
iteration "ITER.1" {
    on-slice "TS.a"
    state "working"
    claim "C1" from-slice="TS.a" state="met"
    claim "C2" from-slice="TS.a" state="pending"
}
"##,
    );
    let Binding::Refused(record) = bind("ITER.1", "0.9.0", &corpus, &ask(), &[]) else {
        panic!("an unclosed iteration must not bind");
    };
    assert_eq!(record.failed[0].0, "ITER.1", "the iteration is named");
    assert!(record.failed[0].1.contains("C2"), "and its unsettled claim: {:?}", record.failed[0].1);
    assert!(!record.failed[0].1.contains("C1"), "and not the settled one");
    assert!(record.kdl.contains("only-closed-iterations-bind"));
}

#[test]
fn c2_an_iteration_already_bound_elsewhere_is_refused() {
    let corpus = corpus_of(&format!(
        r##"
{CLOSED}
release "REL.0.8.0" {{
    version "0.8.0"
    state "planned"
    binds "ITER.1"
}}
"##
    ));
    let Binding::Refused(record) = bind("ITER.1", "0.9.0", &corpus, &ask(), &[]) else {
        panic!("the same work must not be counted in two releases");
    };
    assert!(record.failed[0].1.contains("0.8.0"), "and it says WHERE: {:?}", record.failed[0].1);
    assert!(record.kdl.contains("an-iteration-binds-exactly-once"));
}

#[test]
fn c3_the_proposed_bump_follows_the_configured_rules() {
    // One case per rule the config declares. The mapping is a BINDING — this project
    // bumps minor for a breaking change, and the engine must not know better.
    for (kind, expected) in [
        ("breaking-seam-contract", "minor"),
        ("capability-added", "minor"),
        ("slice-outcome-added", "minor"),
        ("gap-remediation-only", "patch"),
        ("doctrine-only", "patch"),
    ] {
        let corpus = corpus_of(&format!(
            r##"
iteration "ITER.1" {{
    on-slice "TS.a"
    state "closed"
    contributes {kind:?}
}}
"##
        ));
        let proposal = propose(&["ITER.1".to_owned()], &corpus);
        assert_eq!(proposal.position, expected, "{kind} proposes {expected}");
        assert!(proposal.because.contains(kind));
    }
}

#[test]
fn c3_the_highest_bump_among_what_is_bound_wins() {
    let corpus = corpus_of(
        r##"
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    contributes "doctrine-only"
}
iteration "ITER.2" {
    on-slice "TS.b"
    state "closed"
    contributes "capability-added"
}
"##,
    );
    let proposal = propose(&["ITER.1".to_owned(), "ITER.2".to_owned()], &corpus);
    assert_eq!(proposal.position, "minor");
    assert!(proposal.because.contains("ITER.2"), "and it names which one decided it");
}

#[test]
fn an_iteration_declaring_no_contribution_is_named_rather_than_assumed() {
    // A proposal computed from half the work is not a proposal, so the half that
    // informed nothing is on the record beside it.
    let corpus = corpus_of(
        r##"
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    contributes "doctrine-only"
}
iteration "ITER.2" {
    on-slice "TS.b"
    state "closed"
}
"##,
    );
    let proposal = propose(&["ITER.1".to_owned(), "ITER.2".to_owned()], &corpus);
    assert_eq!(proposal.position, "patch");
    assert_eq!(proposal.silent, vec!["ITER.2".to_owned()]);

    let Binding::Bound { release, .. } = bind("ITER.2", "0.9.0", &corpus, &ask(), &[]) else {
        panic!("it still binds — informing nothing is not a refusal")
    };
    assert!(release.kdl.contains("proposal-incomplete-for"));
}

#[test]
fn c4_unbinding_is_possible_before_cut() {
    let corpus = corpus_of(&format!(
        r##"
{CLOSED}
release "REL.0.9.0" {{
    version "0.9.0"
    state "planned"
    binds "ITER.1"
}}
"##
    ));
    let Binding::Bound { release, .. } = unbind("ITER.1", "0.9.0", &corpus, &ask(), &[]) else {
        panic!("binding is reversible until the release is cut");
    };
    assert!(!release.kdl.contains(r#"binds "ITER.1""#));
    assert!(release.kdl.contains("Nothing bound"), "and the emptiness is stated, not blank");
}

#[test]
fn c4_unbinding_after_cut_is_refused() {
    let corpus = corpus_of(&format!(
        r##"
{CLOSED}
release "REL.0.9.0" {{
    version "0.9.0"
    state "released"
    binds "ITER.1"
}}
"##
    ));
    let Binding::Refused(record) = unbind("ITER.1", "0.9.0", &corpus, &ask(), &[]) else {
        panic!("a cut release does not change");
    };
    assert!(record.failed[0].1.contains("cutting is not"));
}

#[test]
fn c4_binding_into_a_cut_release_is_refused_too() {
    let corpus = corpus_of(&format!(
        r##"
{CLOSED}
iteration "ITER.2" {{
    on-slice "TS.b"
    state "closed"
    contributes "doctrine-only"
}}
release "REL.0.9.0" {{
    version "0.9.0"
    state "released"
    binds "ITER.1"
}}
"##
    ));
    let Binding::Refused(record) = bind("ITER.2", "0.9.0", &corpus, &ask(), &[]) else {
        panic!("a cut release does not change, in either direction");
    };
    assert!(record.kdl.contains("a-cut-release-does-not-change"));
}

#[test]
fn a_scheme_the_engine_does_not_know_proposes_nothing_rather_than_guessing() {
    let schema_doc = parse(SCHEMA).expect("parses");
    let config_doc = parse(
        r##"
config {
    versioning {
        scheme "calver"
        bump-proposal {
            doctrine-only "patch"
        }
    }
}
"##,
    )
    .expect("parses");
    let record_doc = parse(
        r##"
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    contributes "doctrine-only"
}
"##,
    )
    .expect("parses");
    let schema = Schema::from_document(&schema_doc);
    let corpus = Corpus::from_documents(&[schema_doc, config_doc, record_doc], &schema);
    let proposal = propose(&["ITER.1".to_owned()], &corpus);
    assert_eq!(
        proposal.position, "patch",
        "the RULE still applies — it is the ranking between positions that calver does not share"
    );
}

#[test]
fn binding_composes_the_release_whole_rather_than_appending() {
    let corpus = corpus_of(&format!(
        r##"
{CLOSED}
iteration "ITER.2" {{
    on-slice "TS.b"
    state "closed"
    contributes "doctrine-only"
}}
release "REL.0.9.0" {{
    version "0.9.0"
    state "planned"
    binds "ITER.1"
}}
"##
    ));
    let Binding::Bound { release, .. } = bind("ITER.2", "0.9.0", &corpus, &ask(), &[]) else {
        panic!("expected a bind")
    };
    assert!(release.kdl.contains(r#"binds "ITER.1""#), "what was already bound survives");
    assert!(release.kdl.contains(r#"binds "ITER.2""#));
    assert!(release.kdl.starts_with("//"), "and the whole file is composed, header included");
}
