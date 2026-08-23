//! `TS.260823.02` — a claim settles on a run the tool watched.
//!
//! Before this, the strongest thing the record could say about a met claim was that
//! somebody had typed a sentence beside it. The validation review closed an iteration with
//! three evidenced layers and a met claim in a repository containing no application code,
//! and nothing in the method objected — which is `S2` reproduced inside the mechanism built
//! to compute it away.
//!
//! What is checked here is the CORE: what a run means, what settles, and what is refused
//! before anything is executed. Running is I/O and belongs to the shell.

use praxis_core::evidence::{Unwitnessed, Verification, Witnessed, command_for, digest, verification};
use praxis_core::parse;

fn run(exit: i32, output: &str) -> Witnessed {
    Witnessed {
        command: "cargo test --quiet".into(),
        exit,
        digest: digest(output),
        at: "2026-08-23T00:00:00Z".into(),
    }
}

/// C1. The digest is of what the run PRINTED, and it is a value the agent does not know
/// until the run has happened — which is the property prose never had.
#[test]
fn a_run_is_identified_by_what_it_printed() {
    assert_ne!(
        run(0, "5 passed").digest,
        run(0, "0 passed").digest,
        "a different run must digest differently, or the digest witnesses nothing"
    );
    assert_eq!(run(0, "5 passed").digest, run(0, "5 passed").digest);
}

/// C2. A red run is a fact. It is not evidence the claim holds.
#[test]
fn a_non_zero_exit_does_not_settle() {
    assert!(run(0, "ok").settles());
    assert!(!run(1, "failed").settles());
    assert!(!run(101, "panicked").settles());
}

/// C3. A repository that declares no verification is a legitimate state, not an error —
/// it keeps prose evidence, and the absence is what marks the claim reported rather than
/// witnessed.
#[test]
fn a_repository_may_declare_no_verification() {
    let doc = parse(r#"config { repository "acme/checkout" }"#).expect("parses");
    assert_eq!(verification(&[doc]), None, "declaring none is an answer, not a failure");
}

#[test]
fn a_declared_verification_is_read_from_the_config() {
    let doc = parse(
        r#"
config {
    repository "acme/checkout"
    verification {
        runner "cargo test --quiet"
        select-with "--test"
    }
}
"#,
    )
    .expect("parses");
    assert_eq!(
        verification(&[doc]),
        Some(Verification {
            runner: "cargo test --quiet".into(),
            select_with: Some("--test".into()),
        })
    );
}

/// The command comes from the RECORD. Nothing the caller passes becomes a program to run —
/// an engine that ran an arbitrary string would be a command runner driven by the agent it
/// exists to check, and that agent could name `true` and settle everything.
#[test]
fn the_selector_never_becomes_the_command() {
    let v = Verification {
        runner: "cargo test --quiet".into(),
        select_with: Some("--test".into()),
    };
    let argv = command_for(&v, Some("rm -rf /")).expect("builds");
    assert_eq!(argv[0], "cargo", "the program is the runner's, always");
    assert_eq!(
        argv.last().unwrap(),
        "rm -rf /",
        "a selector stays ONE argument — it is data handed to the runner, never re-split into a command"
    );
}

/// A selector silently dropped runs the whole suite and reports it as the one test.
#[test]
fn a_selector_a_runner_cannot_take_is_refused_not_dropped() {
    let v = Verification { runner: "make verify".into(), select_with: None };
    assert!(matches!(
        command_for(&v, Some("one-test")),
        Err(Unwitnessed::RunnerTakesNoSelector { .. })
    ));
    assert!(command_for(&v, None).is_ok(), "the whole suite is still runnable");
}
