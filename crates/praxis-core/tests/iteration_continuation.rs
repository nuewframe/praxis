//! `TS.260823.10` — a completeness question is answered from a slice's whole history, not
//! from the current iteration alone.
//!
//! Three findings, one shape: `ITER.260823.08/F1` re-owed four already-witnessed claims,
//! `ITER.260823.08/F2` passed `implement-follows-an-approach` by holding no design at all,
//! and `ITER.260823.06/F4` refused the ordinary in-flight state of any iteration that
//! designs before it builds.

use praxis_core::{
    Ask, Closing, Conditions, Continuation, Corpus, Known, Pickup, Schema, Severity, Violation,
    assess, check_corpus, check_node, close_iteration, index_all, parse, pick_up,
};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug"     each="1"
            field "kind"     each="1"
            field "realizes" each="1"
            field "claim"    each="0..n"
        }
        entity "iteration" {
            field "on-slice"  each="1..n" references="thin-slice"
            field "continues" each="0..1" references="iteration"
            field "state"     each="1"
            field "claim"     each="0..n"
            field "finding"   each="0..n"
            field "phase"     each="0..n" holds="phase"
        }
        entity "phase" states="pending active complete skipped" {
            field "state"    each="1" one-of="pending" "active" "complete" "skipped"
            field "approach" each="0..n" holds="approach"
            field "followed" each="0..n"
        }
        entity "approach" {
            field "over"    each="1..n"
            field "because" each="1"
        }
        rule "a-continuation-shares-its-slice" refuses="a continuation over a different slice"
        rule "a-continuation-states-why"       reports="a continuation with no stated reason"
        rule "implement-follows-an-approach"   refuses="an implement that did not read its plan"
        rule "the-plan-and-the-code-agree"     refuses="an approach the record says was taken and was not, or the reverse"
    }
    admits "an iteration on a slice" {
        condition "slice-is-shaped"        check="slice-is-shaped"
        condition "dependencies-delivered" check="dependencies-delivered"
        condition "no-iteration-in-flight" check="no-iteration-in-flight"
        condition "nothing-left-to-admit"  check="nothing-left-to-admit"
    }
}
"##;

fn violations(record: &str) -> Vec<Violation> {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let mut known = Known::default();
    index_all(&schema_doc, &mut known);
    index_all(&record_doc, &mut known);
    let mut all: Vec<_> =
        record_doc.nodes().iter().flat_map(|n| check_node(n, &schema, &known)).collect();
    all.extend(check_corpus(&[schema_doc, record_doc], &schema));
    all
}

/// C1 — `continues` naming an iteration with no slice in common is refused.
#[test]
fn c1_continuing_an_iteration_on_a_different_slice_is_refused() {
    let record = r#"
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
}
iteration "ITER.2" {
    on-slice "TS.b"
    continues "ITER.1" because="borrows the name, not the slice"
    state "open"
}
"#;
    let found = violations(record);
    assert!(
        found.iter().any(|v| v.refusal.rule() == "a-continuation-shares-its-slice"
            && v.severity() == Severity::Refuse),
        "{found:?}"
    );
}

/// C1 — `continues` naming nothing the record holds is refused, by the generic reference
/// check `continues` inherits from `references="iteration"`.
#[test]
fn c1_continuing_an_iteration_the_record_does_not_hold_is_refused() {
    let record = r#"
iteration "ITER.2" {
    on-slice "TS.a"
    continues "ITER.nowhere" because="stands on nothing the record holds"
    state "open"
}
"#;
    let found = violations(record);
    assert!(
        found.iter().any(|v| v.refusal.rule() == "dangling-relationship"),
        "{found:?}"
    );
}

/// C1 — sharing a slice with what it continues is not refused.
#[test]
fn c1_continuing_an_iteration_on_the_same_slice_is_not_refused() {
    let record = r#"
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
}
iteration "ITER.2" {
    on-slice "TS.a"
    continues "ITER.1" because="the first left C2 unmet"
    state "open"
}
"#;
    let found = violations(record);
    assert!(
        !found.iter().any(|v| v.refusal.rule() == "a-continuation-shares-its-slice"),
        "{found:?}"
    );
}

/// C6 — a continuation naming no reason is reported, not refused.
#[test]
fn c6_a_continuation_with_no_stated_reason_is_reported() {
    let record = r#"
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
}
iteration "ITER.2" {
    on-slice "TS.a"
    continues "ITER.1"
    state "open"
}
"#;
    let found = violations(record);
    let hit = found.iter().find(|v| v.refusal.rule() == "a-continuation-states-why");
    assert!(hit.is_some(), "{found:?}");
    assert_eq!(hit.unwrap().severity(), Severity::Report);
}

/// C4 — an iteration with no design of its own satisfies `implement-follows-an-approach`
/// by naming the approach the iteration it continues produced. `ITER.260823.08/F2`: this
/// passed before by holding no design at all, which is the wrong route to the right
/// outcome.
#[test]
fn c4_naming_a_continued_approach_satisfies_implement_follows_an_approach() {
    let base = r#"
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    phase "design-system" state="complete" {
        approach "form-agnostic reading" {
            over "a declared form per field"
            because "nothing needs to insist on a form"
        }
    }
}
iteration "ITER.2" {
    on-slice "TS.a"
    continues "ITER.1" because="carries D1 out; C2 was the only claim left"
    state "closed"
    phase "implement" state="complete" %s
}
"#;
    let silent = base.replacen("%s", "", 1);
    let found = violations(&silent);
    assert!(
        found.iter().any(|v| v.refusal.rule() == "implement-follows-an-approach"),
        "an implement naming nothing is still refused: {found:?}"
    );

    let named = base.replacen("%s", "followed=\"form-agnostic reading\"", 1);
    let found = violations(&named);
    assert!(
        !found.iter().any(|v| v.refusal.rule() == "implement-follows-an-approach"),
        "naming the continued iteration's approach satisfies it: {found:?}"
    );
}

/// C5 — an approach whose implement is still in flight is reported, never refused.
/// `ITER.260823.06/F4`: this is the ordinary state of any iteration that designs before it
/// builds, and refusing it defeated the very sequencing that produced the design.
#[test]
fn c5_an_approach_with_an_in_flight_implement_is_reported_not_refused() {
    let planned = |implement: &str| -> String {
        format!(
            r#"
iteration "ITER.1" {{
    on-slice "TS.a"
    state "working"
    phase "design-system" state="complete" {{
        approach "one way" {{
            over "another way"
            because "it is better"
        }}
    }}
{implement}
}}
"#
        )
    };

    // No implement phase at all yet — the moment right after design closes.
    let not_started = planned("");
    let found = violations(&not_started);
    assert!(
        !found.iter().any(|v| v.refusal.rule() == "the-plan-and-the-code-agree"
            && v.severity() == Severity::Refuse),
        "an approach not yet built is not refused: {found:?}"
    );
    assert!(
        found.iter().any(|v| v.refusal.rule() == "the-plan-and-the-code-agree"
            && v.severity() == Severity::Report),
        "and it is visible rather than silent: {found:?}"
    );

    // Implement under way and not yet complete.
    let active = planned("    phase \"implement\" state=\"active\"");
    let found = violations(&active);
    assert!(
        !found.iter().any(|v| v.refusal.rule() == "the-plan-and-the-code-agree"
            && v.severity() == Severity::Refuse),
        "{found:?}"
    );

    // Complete, and still not followed or abandoned — THIS is refused.
    let finished = planned("    phase \"implement\" state=\"complete\"");
    let found = violations(&finished);
    assert!(
        found.iter().any(|v| v.refusal.rule() == "the-plan-and-the-code-agree"
            && v.severity() == Severity::Refuse),
        "a complete implement that never followed or abandoned the plan is refused: {found:?}"
    );
}

fn ask() -> Ask {
    Ask {
        signer: "human:someone".to_owned(),
        at: "2026-08-23T09:00:00Z".to_owned(),
        by: "agent:praxis".to_owned(),
        attested_by: None,
    }
}

/// C2 — a claim the continued iteration already met arrives CARRIED, citing that
/// iteration, and one it left unmet still arrives pending. `ITER.260823.08/F1`: today both
/// arrive pending and the already-witnessed ones are re-owed.
#[test]
fn c2_a_claim_the_continued_iteration_already_met_arrives_carried() {
    let record = r#"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
    claim "C1"
    claim "C2"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    claim "C1" from-slice="TS.a" state="met"
    claim "C2" from-slice="TS.a" state="pending"
    finding "F1" carries="C2" text="the residue"
}
"#;
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let conditions = Conditions::from_document(&schema_doc);
    let docs = vec![schema_doc, record_doc];
    let corpus = Corpus::from_documents(&docs, &schema);
    let assessment = assess(&corpus, &conditions, &ask().at);
    let continuation = Continuation { iteration: "ITER.1", because: Some("C2 was left open") };

    let Pickup::Opened(opened) =
        pick_up(&["TS.a".to_owned()], &corpus, &assessment, &ask(), &["ITER.1".to_owned()], Some(&continuation))
    else {
        panic!("expected an admission");
    };

    assert!(
        opened.kdl.contains(r#"claim "C1" from-slice="TS.a" state="carried" carried-from="ITER.1""#),
        "{}",
        opened.kdl
    );
    assert!(
        opened.kdl.contains(r#"claim "C2" from-slice="TS.a" state="pending""#),
        "C2 was never met, so it arrives pending like any first attempt: {}",
        opened.kdl
    );
    assert!(opened.kdl.contains(r#"continues "ITER.1" because="C2 was left open""#), "{}", opened.kdl);
}

/// C3 — a carried claim, once re-witnessed, settles on the new run rather than staying a
/// citation. Modelled at the accounting layer: a claim citing a prior iteration is
/// accounted "carried from"; the same claim once its own state is `met` is accounted
/// "met", regardless of the citation left on it.
#[test]
fn c3_a_carried_claim_may_be_re_witnessed_and_then_reads_as_met() {
    let closing = |record: &str| -> Closing {
        let schema_doc = parse(SCHEMA).expect("schema parses");
        let record_doc = parse(record).expect("record parses");
        let schema = Schema::from_document(&schema_doc);
        let corpus = Corpus::from_documents(&[schema_doc, record_doc], &schema);
        close_iteration(
            "ITER.2",
            &corpus,
            &Ask {
                signer: "human:someone".to_owned(),
                at: ask().at,
                by: "agent:praxis".to_owned(),
                attested_by: Some("human:reviewer".to_owned()),
            },
            &[],
        )
    };

    let still_carried = r#"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
    claim "C1"
}
iteration "ITER.2" {
    on-slice "TS.a"
    continues "ITER.1" because="second attempt"
    state "working"
    claim "C1" from-slice="TS.a" state="carried" carried-from="ITER.1"
}
"#;
    let Closing::Accepted { accounting, .. } = closing(still_carried) else {
        panic!("a carried claim, on its own, is already accounted for");
    };
    assert_eq!(accounting, vec![("C1".to_owned(), "carried from ITER.1".to_owned())]);

    // Re-witnessed against the tree as it now stands — the tool rewrites `state` to `met`
    // and leaves the citation, which is now inert history rather than the accounting.
    let re_witnessed = still_carried.replace(r#"state="carried" carried-from="ITER.1""#, r#"state="met" carried-from="ITER.1""#);
    let Closing::Accepted { accounting, all_met, .. } = closing(&re_witnessed) else {
        panic!("a re-witnessed claim settles")
    };
    assert_eq!(accounting, vec![("C1".to_owned(), "met".to_owned())]);
    assert!(all_met, "and the close can say every claim is genuinely met, not merely accounted for");
}
