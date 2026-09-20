//! `TS.260823.07` — one version across every harness.
//!
//! Five manifests described a method two versions old and nothing could tell. Version and
//! description are projections of the record; everything else about a manifest stays
//! hand-authored, and a package that ships doctrine instructing an agent to run `praxis
//! check` had better ship `praxis` to run.

use praxis_core::check::{Facts, Severity, check_corpus_given};
use praxis_core::manifest::{PATHS, Projection, derive};
use praxis_core::{Schema, parse};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "release" {
            field "version" each="1"
            field "state"   each="1"
        }
        rule "a-manifest-is-derived-from-the-record" refuses="a manifest whose version or description does not match the record"
        rule "the-package-ships-what-it-tells-you-to-run" refuses="a package that omits the engine or the method file"
        rule "surface-teaches-a-retired-kind" refuses="doctrine instructing a retired word"

        retired "the wave-era vocabulary" were="the retired frame" {
            word "wave" instead="a frame"
        }
    }
}
release "REL.current" {
    version "1.2.0"
    state "planned"
}
mission "test" {
    is "fidelity is computed from the record"
}
"##;

const MISSION: &str = "fidelity is computed from the record";

fn manifest_text(version: &str, description: &str) -> String {
    format!(r#"{{"name": "praxis", "version": {version:?}, "description": {description:?}}}"#)
}

fn violations(text: Vec<(String, String)>) -> Vec<praxis_core::Violation> {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let schema = Schema::from_document(&schema_doc);
    let facts = Facts { shipped: vec![], owed: vec![], text };
    check_corpus_given(&[schema_doc], &schema, &facts)
}

/// C1 — every manifest `derive()` produces for the record's version carries it, and none
/// of its other fields move.
#[test]
fn c1_deriving_a_manifest_carries_the_records_version_and_nothing_else() {
    for path in PATHS {
        // Exercised by the other four; its derived fields live inside `plugins`, which
        // this generic fixture does not have.
        if *path == ".claude-plugin/marketplace.json" {
            continue;
        }
        let existing = r#"{"name": "praxis", "version": "0.1.0", "description": "stale", "skills": "./skills/"}"#;
        let out = derive(path, existing, Projection { version: "1.2.0", description: Some(MISSION) })
            .unwrap_or_else(|e| panic!("{path}: {e}"));
        let entry: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(entry["version"], "1.2.0", "{path}");
        assert_eq!(entry["description"], MISSION, "{path}");
        assert_eq!(entry["skills"], "./skills/", "{path} — not touched by the projection");
    }
}

/// C2 — a manifest matching the record's version and description is not drift; one that
/// does not is, by name and by field.
#[test]
fn c2_a_hand_edited_manifest_is_named_by_field() {
    let fresh = violations(vec![(PATHS[0].to_owned(), manifest_text("1.2.0", MISSION))]);
    assert!(
        !fresh.iter().any(|v| v.refusal.rule() == "a-manifest-is-derived-from-the-record"),
        "a manifest carrying the record's own version and description is not drift: {fresh:?}"
    );

    let stale = violations(vec![(PATHS[0].to_owned(), manifest_text("0.7.1", MISSION))]);
    assert!(
        stale.iter().any(|v| v.refusal.rule() == "a-manifest-is-derived-from-the-record"
            && v.refusal.field() == "version"
            && v.severity() == Severity::Refuse),
        "{stale:?}"
    );

    let reworded = violations(vec![(PATHS[0].to_owned(), manifest_text("1.2.0", "hand-written prose"))]);
    assert!(
        reworded.iter().any(|v| v.refusal.rule() == "a-manifest-is-derived-from-the-record"
            && v.refusal.field() == "description"),
        "{reworded:?}"
    );
}

/// C3 — a package shipping doctrine that instructs `praxis check` without shipping the
/// engine or the method file is named, by what it omits.
#[test]
fn c3_a_package_missing_the_engine_or_the_method_is_named() {
    let bare = violations(vec![(
        "package.json".to_owned(),
        r#"{"name": "praxis", "files": ["skills", "agents"]}"#.to_owned(),
    )]);
    assert!(
        bare.iter().any(|v| v.refusal.rule() == "the-package-ships-what-it-tells-you-to-run"
            && v.refusal.message().contains("crates")),
        "{bare:?}"
    );
    assert!(
        bare.iter().any(|v| v.refusal.rule() == "the-package-ships-what-it-tells-you-to-run"
            && v.refusal.message().contains("praxis")),
        "{bare:?}"
    );

    let complete = violations(vec![(
        "package.json".to_owned(),
        r#"{"name": "praxis", "files": ["skills", "agents", "crates", "praxis"]}"#.to_owned(),
    )]);
    assert!(
        !complete.iter().any(|v| v.refusal.rule() == "the-package-ships-what-it-tells-you-to-run"),
        "{complete:?}"
    );
}

/// C4 — a manifest is shipped text like any other, so a retired word inside one is caught
/// by the same rule that catches it in a skill.
#[test]
fn c4_a_manifest_naming_a_retired_word_is_caught_by_the_shared_rule() {
    let found = violations(vec![(
        PATHS[0].to_owned(),
        r#"{"name": "praxis", "version": "1.2.0", "keywords": ["wave-methodology"]}"#.to_owned(),
    )]);
    assert!(
        found.iter().any(|v| v.refusal.rule() == "surface-teaches-a-retired-kind"),
        "{found:?}"
    );
}
