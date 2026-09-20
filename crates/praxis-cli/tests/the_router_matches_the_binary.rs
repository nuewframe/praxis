//! `TS.260821.04`/C4 — the router names only commands the binary has.
//!
//! The spine in `skills/using-praxis/SKILL.md` described `create-wave`, `start-thin-slice`,
//! `create-sprint`, `intake` and `close-sprint` for eighteen iterations after the delivery
//! graph replaced all five. Nothing objected, because nothing compared the document to the
//! thing it documents.
//!
//! This is the generalisable half of that slice: **a document whose instructions are checked
//! against the thing they instruct cannot go stale in silence.**

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    // crates/praxis-cli → crates → repo
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("the crate sits two levels below the repository root")
        .to_path_buf()
}

fn subcommands() -> Vec<String> {
    let out = Command::new(env!("CARGO_BIN_EXE_praxis"))
        .arg("--help")
        .env("NO_COLOR", "1")
        .output()
        .expect("praxis runs");
    let text = String::from_utf8_lossy(&out.stdout);
    text.lines()
        .skip_while(|l| !l.starts_with("Commands:"))
        .skip(1)
        .take_while(|l| l.starts_with("  ") && !l.trim().is_empty())
        .filter_map(|l| l.split_whitespace().next())
        .map(str::to_owned)
        .filter(|c| c != "help")
        .collect()
}

#[test]
fn every_praxis_command_the_router_names_exists() {
    let router = std::fs::read_to_string(repo_root().join("skills/using-praxis/SKILL.md"))
        .expect("the router is where the router has always been");
    let known = subcommands();
    assert!(!known.is_empty(), "the binary reported no subcommands, so this test proves nothing");

    // Only what the document tells an agent to RUN: a backticked `praxis <verb>`, or a line
    // inside a fence. Matching bare "praxis " in prose would catch "praxis sets defaults"
    // and turn this test into a list of exceptions, which is how a check stops meaning
    // anything.
    let mut named = Vec::new();
    let fenced: Vec<&str> =
        router.lines().map(str::trim).filter(|l| l.starts_with("praxis ")).collect();
    let backticked: Vec<&str> =
        router.match_indices("`praxis ").map(|(i, _)| &router[i + 1..]).collect();
    for text in fenced.into_iter().chain(backticked) {
        let verb: String = text["praxis ".len()..]
            .chars()
            .take_while(|c| c.is_ascii_lowercase() || *c == '-')
            .collect();
        // A flag is not a command: `praxis --help` is a legitimate thing to tell someone
        // to run and there is no subcommand called `--help`.
        if verb.starts_with(|c: char| c.is_ascii_lowercase()) && !named.contains(&verb) {
            named.push(verb);
        }
    }
    assert!(!named.is_empty(), "the router names no commands at all, which cannot be right");

    let missing: Vec<_> = named.iter().filter(|v| !known.contains(v)).collect();

    assert!(
        missing.is_empty(),
        "the router tells an agent to run {missing:?}, and the binary has no such command. \
         The spine described five commands that did not exist for eighteen iterations \
         (ITER.260821.19/AK3) — this test is the thing that would have said so.\nThe binary \
         has: {known:?}"
    );
}

/// The retired spine, by name. A regression here means the old workflow came back into the
/// first thing an arriving agent reads.
#[test]
fn the_router_does_not_describe_the_retired_spine() {
    let router = std::fs::read_to_string(repo_root().join("skills/using-praxis/SKILL.md"))
        .expect("the router is readable");

    for retired in [
        "`create-wave`",
        "`start-thin-slice`",
        "`create-sprint`",
        "`close-sprint`",
        "`intake-code-contribution`",
        "`create-adr`",
        "`author-user-docs`",
    ] {
        assert!(
            !router.contains(retired),
            "{retired} is retired and the router still routes to it. It stands at v0.7.1; \
             recovery is by name, not by leaving the door open"
        );
    }
}
