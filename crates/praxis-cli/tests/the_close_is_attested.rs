//! `TS.260821.09` — an attestation the tool does not supply.
//!
//! **This test runs the binary.** That is not a stylistic choice: `TS.260821.08` shipped
//! `an-attestation-is-not-self-issued` with four passing core tests and no reachable path,
//! because `praxis close` compares identities the core never sees it choose
//! (`ITER.260822.05/AP3`). `AK1` was the same defect one command over, and the smoke test
//! written for it only checks that arguments BUILD.
//!
//! A rule that passes its tests and cannot fire is worse than no rule: it reads as a
//! guarantee.

use std::path::{Path, PathBuf};
use std::process::Command;

const SCHEMA: &str = r##"
notional-architecture "NA.e2e" {
    slug "the-shape"
    title "a schema for one iteration"
    frame "FRAME.e2e"
    state "held"
    trail {
        entry at="2026-08-22T00:00:00Z" by="human:test" action="created"
    }
    schema {
        entity "frame" {
            field "title" each="1"
        }
        entity "thin-slice" {
            field "slug"  each="1"
            field "claim" each="1..n"
        }
        entity "iteration" states="open working closed" {
            field "slug"        each="1"
            field "on-slice"    each="1..n"
            field "state"       each="1"
            field "phase"       each="0..n"
            field "claim"       each="0..n"
            field "closed-at"   each="0..1"
            field "outcome"     each="0..1"
            field "close"       each="0..1"
            field "trail"       each="1"
        }
        entity "role" {
            field "is"            each="1"
            field "owns-phase"    each="1..n"
            field "never-for-own" each="0..n"
        }
        rule "a-close-names-its-attester"        refuses="a close nobody attested"
        rule "an-attestation-is-not-self-issued" refuses="an attester vouching for its own work"
    }
}
"##;

const RECORD: &str = r#"
frame "FRAME.e2e" {
    title "a problem"
}
role "principal-engineer" {
    is "architect, implementer and reviewer"
    owns-phase "implement"
    never-for-own "implement"
}
thin-slice "TS.900" {
    slug "a-slice"
    claim "C1" text="something" settled-by="a test"
}
iteration "ITER.900" {
    slug "an-attempt"
    on-slice "TS.900"
    state "working"
    phase "implement" state="complete" worked-by="agent:principal-engineer"
    claim "C1" from-slice="TS.900" state="met"
    trail {
        entry at="2026-08-22T00:00:00Z" by="agent:praxis" action="created"
    }
}
"#;

/// A state root with one iteration, ready to close. Rebuilt per test so a close in one does
/// not decide the outcome of another.
fn root(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("praxis-attest-{name}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("the scratch root is creatable");
    std::fs::write(dir.join("NA.e2e.kdl"), SCHEMA).expect("schema written");
    std::fs::write(dir.join("ITER.900.kdl"), RECORD).expect("record written");
    dir
}

fn close(root: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_praxis"))
        .arg("close")
        .arg("ITER.900")
        .args(args)
        .arg(root)
        .env("NO_COLOR", "1")
        .output()
        .expect("praxis runs");
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    (out.status.success(), text)
}

/// C1 — a close with no named attester is refused, through the binary.
#[test]
fn a_close_nobody_attested_is_refused() {
    let root = root("unattested");
    let (ok, text) = close(&root, &[]);

    assert!(!ok, "an unattested close must not succeed: {text}");
    assert!(
        text.contains("--attested-by"),
        "and the refusal must say how to fix it, not name a rule: {text}"
    );
    assert!(
        !std::fs::read_to_string(root.join("ITER.900.kdl")).unwrap().contains("closed"),
        "nothing closes"
    );
}

/// C2 — the rule fires on the NAMED attester. This is the claim `TS.260821.08` could not
/// make: it compared `by`, which is the tool, against a phase worker, and those can never be
/// equal.
#[test]
fn an_agent_attesting_its_own_work_is_refused() {
    let root = root("self");
    let (ok, text) = close(&root, &["--attested-by", "agent:principal-engineer"]);

    assert!(!ok, "the attester worked the implement phase its role reserves: {text}");
    assert!(text.contains("implement"), "the refusal names the phase: {text}");
    assert!(
        !std::fs::read_to_string(root.join("ITER.900.kdl")).unwrap().contains("closed-at"),
        "nothing closes"
    );
}

/// C2, the other half — a distinct attester closes. Working the phase was never the problem.
#[test]
fn a_distinct_attester_closes() {
    let root = root("distinct");
    let (ok, text) = close(&root, &["--attested-by", "human:someone-else"]);

    assert!(ok, "a handoff is exactly what the rule exists to require: {text}");

    // C3 — what RAN and who ATTESTED stay separate facts in the record.
    let written = std::fs::read_to_string(root.join("ITER.900.kdl")).unwrap();
    assert!(written.contains("attested-by \"human:someone-else\""), "{written}");
    assert!(
        written.contains("by=\"agent:praxis\""),
        "the trail still records the tool, because conflating the two is the defect: {written}"
    );
}

/// An identity occupying no declared role gets no licence to vouch for itself. The licence
/// is what a role IS, so its absence cannot be permission.
#[test]
fn an_identity_with_no_role_cannot_attest_its_own_work() {
    let root = root("roleless");
    std::fs::write(
        root.join("ITER.900.kdl"),
        RECORD.replace("worked-by=\"agent:principal-engineer\"", "worked-by=\"agent:nobody\""),
    )
    .expect("record written");

    let (ok, text) = close(&root, &["--attested-by", "agent:nobody"]);
    assert!(!ok, "no role means no phase is exempt, not that every phase is: {text}");
}
