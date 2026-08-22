//! `TS.260821.12` — publish what a version lets somebody do.
//!
//! The record was more human-readable than its own projection. Every slice declares `title`,
//! `trigger`, `outcome` and `useful-alone` — sentences written by hand, for people — and the
//! `Slice` the corpus built held none of them, so the release notes listed iteration ids
//! with one bump category repeated on every row.

use praxis_core::view::ReadModel;
use praxis_core::{Corpus, Publication, Schema, parse, publish};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "frame" {
            field "title"      each="1"
            field "root-cause" each="1"
            field "principle"  each="1"
            field "symptom"    each="0..n" holds="symptom"
        }
        entity "symptom" {
            field "state" each="1"
        }
        entity "event-storm" {
            field "read-model" each="0..n"
        }
        entity "thin-slice" is="an atomic vertical slice, cut so it can exercise a capability" {
            field "slug"         each="1"
            field "title"        each="1"
            field "trigger"      each="1"
            field "outcome"      each="1"
            field "useful-alone" each="0..1"
            field "command"      each="0..1"
            field "realizes"     each="1"
        }
        entity "capability" is="a permanent doing the system must have" {
            field "state"            each="1"
            field "facet"            each="0..1"
            field "doing"            each="1"
            field "not"              each="1"
            field "keeps-consistent" each="0..n"
        }
        entity "iteration" {
            field "on-slice" each="1..n"
            field "state"    each="1"
            field "finding"  each="0..n"
        }
        entity "release" {
            field "version" each="1"
            field "state"   each="1"
            field "binds"   each="0..n"
        }
    }
}
"##;

const RECORD: &str = r##"
frame "FRAME.test" {
    title "Trust in an artifact is unearned"
    root-cause "fidelity is invisible in the artifact"
    principle "fidelity is computed from the record, never claimed by whoever produced it"
    symptom "S1" state="present"
}
event-storm "ES.test" {
    read-model "the-published-set-for-a-release" answers="what can you do now?" publishable=#true \
        publishes-to="docs/releases/<version>/"
    read-model "what-this-product-means" answers="what does it mean?" publishable=#true \
        publishes-to="docs/releases/<version>/concepts/"
    read-model "how-it-fits-together" answers="how does it fit?" publishable=#true \
        publishes-to="docs/releases/<version>/architecture/"
}
capability "the-gate" {
    state "active"
    facet "product"
    doing "admit work, or refuse it in writing"
    not "it never decides whether the work is worth doing"
    keeps-consistent "at most one open iteration per slice"
}
thin-slice "TS.a" {
    slug "pick-up-a-slice"
    title "Take a slice, and have the refusal recorded when it is refused"
    trigger "somebody wants to start work"
    outcome "the gate admits, or refuses and says which condition failed"
    useful-alone "a refusal stops being invisible. Worth having with nothing else changed"
    command "pick-up"
    realizes "CAP.the-gate"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    finding "F1" text="disjointness is declared and never computed" carries="C3"
}
release "REL.0.1.0" {
    version "0.1.0"
    state "planned"
    binds "ITER.1"
}
"##;

/// Publishing happens BEFORE the cut — the set has to exist in the commit the index names —
/// so the fixture's release is planned. That ordering is also why the usage guide gated on
/// promoted truth was empty on every release ever published (`WALK.260822.02/AS8`).
fn published(view: &str) -> ReadModel {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(RECORD).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let corpus = Corpus::from_documents(&[schema_doc, record_doc], &schema);
    match publish("0.1.0", &corpus) {
        Publication::Ready { documents, .. } => documents
            .into_iter()
            .find(|d| d.model.view == view)
            .unwrap_or_else(|| panic!("{view} is declared publishable"))
            .model,
        Publication::Refused(why) => panic!("publish refused: {why:?}"),
    }
}

fn rows(model: &ReadModel, section: &str) -> Vec<Vec<String>> {
    model
        .sections
        .iter()
        .find(|s| s.name == section)
        .unwrap_or_else(|| panic!("a {section:?} section"))
        .rows
        .clone()
}

fn defined(model: &ReadModel, key: &str) -> String {
    model
        .defines
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| panic!("{key:?} defined, in {:?}", model.defines))
}

/// C1 — what the version lets somebody do, in the record's own sentences.
#[test]
fn what_you_can_do_comes_from_the_slice_and_not_from_its_id() {
    let notes = published("the-published-set-for-a-release");
    let gained = rows(&notes, "what you can do now");

    assert_eq!(gained, [["Take a slice, and have the refusal recorded when it is refused"]]);
    assert!(
        !format!("{gained:?}").contains("ITER."),
        "no iteration id appears above the reader's own question: {gained:?}"
    );
    // The long sentence is REFERENCED, not inlined — `read-model@v1` counts markup in a cell
    // as a violation, and rewriting the record's words to fit would make the publisher a
    // second author.
    assert!(
        defined(&notes, "Take a slice, and have the refusal recorded when it is refused")
            .starts_with("a refusal stops being invisible")
    );
}

/// C2 — how to start: the commands the shipped slices declare.
#[test]
fn how_to_start_is_the_command_and_the_moment_it_is_for() {
    let notes = published("the-published-set-for-a-release");
    assert_eq!(
        rows(&notes, "how to start"),
        [["praxis pick-up", "somebody wants to start work"]]
    );
}

/// C4 — a release that hides its shortfalls is the artifact this frame distrusts.
#[test]
fn what_is_known_missing_is_published_with_what_it_says() {
    let notes = published("the-published-set-for-a-release");
    assert_eq!(rows(&notes, "what is known to be missing"), [["F1", "C3"]]);
    assert_eq!(defined(&notes, "F1"), "disjointness is declared and never computed");
}

/// C5 — the accounting survives, and is last. It is what an auditor wants after the reader
/// has gone.
#[test]
fn the_ledger_is_kept_and_is_the_last_section() {
    let notes = published("the-published-set-for-a-release");
    assert_eq!(rows(&notes, "appendix — what bound to this version"), [["ITER.1", "TS.a"]]);
    assert_eq!(
        notes.sections.last().map(|s| s.name.as_str()),
        Some("appendix — what bound to this version")
    );
}

/// C6 — the problem, the principle, and what each word means. The glossary is generated from
/// whatever the schema declares, so a kind an adopting project adds appears in ITS release.
#[test]
fn the_concepts_carry_the_problem_and_the_vocabulary() {
    let concepts = published("what-this-product-means");
    let problem = rows(&concepts, "the problem this exists for");

    assert!(format!("{problem:?}").contains("Trust in an artifact is unearned"));
    assert!(format!("{problem:?}").contains("fidelity is computed from the record"));

    let glossary = rows(&concepts, "what the words mean");
    assert!(
        glossary.iter().any(|r| r[0] == "capability"
            && r[1] == "a permanent doing the system must have"),
        "the schema's own `is=` sentence, verbatim: {glossary:?}"
    );
}

/// C3 and C7 — what a capability refuses, published beside what it does.
#[test]
fn the_architecture_publishes_what_each_part_refuses_to_do() {
    let architecture = published("how-it-fits-together");
    let product = rows(&architecture, "what it does");

    assert_eq!(
        product,
        [[
            "the-gate",
            "admit work, or refuse it in writing",
            "it never decides whether the work is worth doing"
        ]],
        "a description that omits what a thing cannot do is the artifact this frame distrusts"
    );
    assert_eq!(
        rows(&architecture, "what each keeps true"),
        [["the-gate", "at most one open iteration per slice"]]
    );
}
