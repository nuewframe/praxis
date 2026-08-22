//! `TS.260821.08` — refuse a close whose reviewer is the agent that did the work.
//!
//! "The same engineer cannot self-approve" has been in the router and in the engineer
//! persona since before the delivery graph, and was enforced by nothing at all. It sat in
//! the bottom row of the enforcement table — agent-attested, trusted, not compelled — which
//! is `S2` exactly: an artifact looks identical whether the reviewer was a different mind or
//! the same one wearing a second hat.

use praxis_core::close::{Closing, close_iteration};
use praxis_core::pickup::Ask;
use praxis_core::{Corpus, Schema, parse};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug"  each="1"
            field "claim" each="1..n"
        }
        entity "iteration" states="open working closed" {
            field "slug"     each="1"
            field "on-slice" each="1..n"
            field "state"    each="1"
            field "phase"    each="0..n"
            field "claim"    each="0..n"
        }
        entity "role" {
            field "is"            each="1"
            field "owns-phase"    each="1..n"
            field "never-for-own" each="0..n"
        }
    }
}
"##;

const ROLES: &str = r#"
role "principal-engineer" {
    is "architect, implementer and reviewer"
    owns-phase "implement"
    owns-phase "learn"
    never-for-own "implement"
}
thin-slice "TS.900" {
    slug "a-slice"
    claim "C1" text="something"
}
"#;

fn closing(iteration: &str, by: &str) -> Closing {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(&format!("{ROLES}\n{iteration}")).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let corpus = Corpus::from_documents(&[schema_doc, record_doc], &schema);
    let ask = Ask {
        signer: "human:someone".to_owned(),
        at: "2026-08-22T00:00:00Z".to_owned(),
        by: by.to_owned(),
    };
    close_iteration("ITER.900", &corpus, &ask, &[])
}

const WORKED_BY_ONE: &str = r#"
iteration "ITER.900" {
    slug "an-attempt"
    on-slice "TS.900"
    state "working"
    phase "implement" state="complete" worked-by="agent:principal-engineer"
    claim "C1" from-slice="TS.900" state="met"
}
"#;

/// C1 — the closer worked a phase their own role may not attest.
#[test]
fn an_agent_closing_work_it_did_itself_is_refused() {
    let Closing::Refused(record) = closing(WORKED_BY_ONE, "agent:principal-engineer") else {
        panic!("a close attested by the identity that worked implement must be refused");
    };

    assert_eq!(record.failed.len(), 1);
    assert_eq!(record.failed[0].0, "an-attestation-is-not-self-issued");
    assert!(
        record.failed[0].1.contains("implement"),
        "the refusal names the PHASE, so the fix is obvious: {}",
        record.failed[0].1
    );
    // And it refuses before any claim is examined. C1 is met here — an iteration whose
    // accounting is perfect and whose reviewer is its author is not a review that came out
    // clean, it is not a review.
    assert!(!record.failed[0].1.contains("C1"));
}

/// C2 — working two phases is normal. Attesting your own work is the act refused.
#[test]
fn a_different_identity_closing_the_same_work_is_accepted() {
    let closing = closing(WORKED_BY_ONE, "human:someone-else");

    assert!(
        matches!(closing, Closing::Accepted { .. }),
        "doing the design and the implementation is normal and often better; a human closing \
         work an agent did is exactly the handoff this rule exists to require"
    );
}

/// A phase with no recorded worker is not a violation. Refusing on absence would make every
/// iteration written before this rule unclosable, which is punishing history for not having
/// anticipated it.
#[test]
fn a_phase_whose_worker_nobody_recorded_does_not_refuse() {
    let no_worker = r#"
iteration "ITER.900" {
    slug "an-attempt"
    on-slice "TS.900"
    state "working"
    phase "implement" state="complete"
    claim "C1" from-slice="TS.900" state="met"
}
"#;
    assert!(matches!(
        closing(no_worker, "agent:principal-engineer"),
        Closing::Accepted { .. }
    ));
}

/// A role only forbids attesting the phases it names. Working `learn` and closing is the
/// reviewer doing its job.
#[test]
fn a_phase_the_role_does_not_reserve_is_not_refused() {
    let learn_only = r#"
iteration "ITER.900" {
    slug "an-attempt"
    on-slice "TS.900"
    state "working"
    phase "learn" state="complete" worked-by="agent:principal-engineer"
    claim "C1" from-slice="TS.900" state="met"
}
"#;
    assert!(
        matches!(closing(learn_only, "agent:principal-engineer"), Closing::Accepted { .. }),
        "`learn` is the reviewer's own phase — never-for-own names implement, and nothing else"
    );
}
