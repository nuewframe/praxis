//! `TS.260823.01` — the admission names who asked, and the tool does not decide who.
//!
//! `praxis pick-up` read `git config user.email`, stripped the domain, prefixed `human:`
//! and wrote it into a record that said `status "signed"`. Nothing was signed and nobody
//! was asked by name. The ask through a prompt IS the permission — that was never in
//! doubt — but the identity in the record came from the tree, which knows whose machine
//! this is and nothing about who decided.
//!
//! `close --attested-by` had already made this argument in its own help text: "an identity
//! the tool supplied would prove the tool ran, which nobody doubted." It was applied to
//! close by `TS.260821.08` and never applied to pick-up, so the gate at the START of an
//! iteration was the one place in the method where the tool signed on a human's behalf.
//!
//! The four rules guarding that gate — `iteration-without-signed-admission`,
//! `approval-signed-by-agent`, `unknown-approval-kind`, `adr-accepted-without-approval` —
//! were declared by the method and implemented NOWHERE. Until this slice the only code in
//! the engine that touched an approval was the code that wrote it.

use praxis_core::check::check_corpus;
use praxis_core::{Schema, parse};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "iteration" states="open working closed" {
            field "slug"     each="1"
            field "on-slice" each="1..n"
            field "state"    each="1"
            field "approval" each="0..n"
        }
        rule "iteration-without-signed-admission" refuses="work beginning on an iteration whose admission is unsigned"
        rule "approval-signed-by-agent"           refuses="a signature outside the human namespace"
    }
}
"##;

fn refusals(record: &str) -> Vec<String> {
    let schema_doc = parse(SCHEMA).expect("the schema parses");
    let doc = parse(record).expect("the record parses");
    let schema = Schema::from_document(&schema_doc);
    check_corpus(&[schema_doc.clone(), doc], &schema)
        .into_iter()
        .map(|v| v.refusal.message())
        .collect()
}

/// C4. An agent admitting its own work is the whole subject of this frame.
#[test]
fn an_agent_may_not_admit_its_own_work() {
    let found = refusals(
        r#"
iteration "ITER.1" {
    slug "s"
    on-slice "TS.1"
    state "open"
    approval "admission" {
        status "asked"
        asked-by "agent:praxis"
    }
}
"#,
    );
    assert!(
        found.iter().any(|m| m.contains("agent:praxis") && m.contains("agent")),
        "an agent-admitted iteration must be refused, and the refusal must name who: {found:?}"
    );
}

/// An admission that records a status and no asker says a gate was passed without saying
/// who opened it — the half that cannot be reconstructed afterwards.
#[test]
fn an_admission_with_no_asker_is_refused() {
    let found = refusals(
        r#"
iteration "ITER.1" {
    slug "s"
    on-slice "TS.1"
    state "open"
    approval "admission" { status "asked" }
}
"#,
    );
    assert!(
        found.iter().any(|m| m.contains("names nobody who asked")),
        "an admission with no asker must be refused: {found:?}"
    );
}

/// The ask is the gate — it is recorded or it did not happen.
#[test]
fn an_iteration_with_no_admission_is_refused() {
    let found = refusals(
        r#"
iteration "ITER.1" {
    slug "s"
    on-slice "TS.1"
    state "open"
}
"#,
    );
    assert!(
        found.iter().any(|m| m.contains("carries no admission")),
        "an iteration nobody asked for must be refused: {found:?}"
    );
}

/// The old form still READS. Refusing it would refuse every iteration this record already
/// holds, and those records are not malformed — the fault was that the tool supplied the
/// name in them, which is fixed at the writing end, not by invalidating history.
#[test]
fn the_old_signer_form_still_reads() {
    let found = refusals(
        r#"
iteration "ITER.1" {
    slug "s"
    on-slice "TS.1"
    state "open"
    approval "admission" {
        status "signed"
        signer "human:wael"
    }
}
"#,
    );
    assert!(found.is_empty(), "an iteration written before this slice must still check: {found:?}");
}

/// A named human passes, in either form.
#[test]
fn a_named_human_admits() {
    let found = refusals(
        r#"
iteration "ITER.1" {
    slug "s"
    on-slice "TS.1"
    state "open"
    approval "admission" {
        status "asked"
        asked-by "human:wael"
        via "the ask to pick up this slice"
    }
}
"#,
    );
    assert!(found.is_empty(), "a named human asker must check clean: {found:?}");
}
