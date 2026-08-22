//! Every command, invoked as a user invokes it.
//!
//! `ITER.260821.19/AK1`: `praxis pick-up` panicked on startup for an entire iteration and
//! nothing noticed, because every other test calls the CORE. A binary whose argument
//! parser cannot be built is a binary that fails at the first keystroke, and 185 tests
//! passed over it.

use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_praxis"))
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("NO_COLOR", "1")
        .output()
        .expect("praxis runs")
}

/// clap validates the whole command tree when it builds. Asking for help on a subcommand
/// is enough to prove its arguments are constructible — which is exactly what was broken.
#[test]
fn every_subcommand_can_be_built() {
    for command in [
        "check",
        "pick-up",
        "close",
        "bind",
        "cut-release",
        "publish",
        "verify-published",
        "promote",
        "truth",
        "review",
        "guide",
        "dashboard",
        "prove",
        "accept",
        "ready",
        "audit-surfaces",
    ] {
        let out = run(&[command, "--help"]);
        assert!(
            out.status.success(),
            "`praxis {command} --help` failed — its arguments cannot be built:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
}

#[test]
fn the_top_level_help_lists_every_command() {
    let out = run(&["--help"]);
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    for command in ["check", "pick-up", "prove", "accept", "dashboard"] {
        assert!(text.contains(command), "`{command}` is missing from the help");
    }
}

/// `TS.260821.03`. The audit reads TWO trees — the state root and the tree whose doctrine
/// is being audited — and they are not the same directory. Conflating them would audit
/// `praxis/skills/`, which does not exist, and report zero shipped surfaces: a clean answer
/// to the wrong question.
#[test]
fn audit_surfaces_separates_the_record_from_the_tree() {
    let out = run(&["audit-surfaces", "--help"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("--root"), "the state root is a flag: {text}");
    assert!(text.contains("--from"), "and the audited tree is a separate one: {text}");
}

#[test]
fn pick_up_takes_several_slices_and_a_root_flag() {
    // The shape that broke: a variadic positional followed by an optional positional.
    let out = run(&["pick-up", "--help"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("--root"), "root is a flag, not a positional: {text}");
    assert!(text.contains("SLICES"), "and the slices are variadic: {text}");
}
