//! `TS.260820.08` — review what an iteration has actually shown, writing nothing.
//!
//! This attacks `S2`: a reviewer judging an artifact by how finished it looks. What makes
//! that possible is that "what was promised" and "what was shown" arrive separately and
//! get reconciled in someone's head. Here they arrive together with the gaps named.

use praxis_core::{Corpus, Schema, parse, review};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug"  each="1"
            field "layer" each="0..n"
            field "claim" each="0..n"
        }
        entity "iteration" {
            field "on-slice" each="1"
            field "state"    each="1"
            field "claim"    each="0..n"
            field "layer"    each="0..n"
            field "phase"    each="0..n"
            field "finding"  each="0..n"
        }
    }
}
"##;

const IN_FLIGHT: &str = r##"
thin-slice "TS.a" {
    slug "a-thing"
    layer "doctrine"
    layer "enforcement"
    claim "C1"
    claim "C2"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "working"
    claim "C1" from-slice="TS.a" state="met"
    claim "C2" from-slice="TS.a" state="pending"
    layer "doctrine" state="evidenced" evidence="a skill that exists"
    phase "implement" state="active" produced="the half that is done"
}
"##;

fn corpus_of(record: &str) -> Corpus {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    Corpus::from_documents(&[schema_doc, record_doc], &schema)
}

fn section<'a>(model: &'a praxis_core::ReadModel, name: &str) -> &'a praxis_core::Section {
    model.sections.iter().find(|s| s.name == name).unwrap_or_else(|| panic!("a {name} section"))
}

#[test]
fn c1_a_preview_declares_itself_a_preview() {
    let model = review("ITER.1", &corpus_of(IN_FLIGHT), "now").expect("a preview");
    assert!(model.answers.starts_with("PREVIEW of ITER.1"), "{}", model.answers);
    assert!(!model.publishable, "and it can never be published");
    assert!(
        model.definition("this is a preview").is_some_and(|d| d.contains("not a record")),
        "the header says which it is, in the result rather than only in the renderer"
    );
}

#[test]
fn c2_composing_a_preview_produces_no_file_at_all() {
    // Structural: `review` returns a ReadModel and nothing else. There is no path in the
    // type, so a preview cannot name a place in the published tree to land.
    let model = review("ITER.1", &corpus_of(IN_FLIGHT), "now").expect("a preview");
    assert!(model.flaws().is_empty(), "{:?}", model.flaws());
    for section in &model.sections {
        for row in &section.rows {
            for cell in row {
                assert!(!cell.contains("docs/releases/"), "nothing here points into the published tree");
            }
        }
    }
}

#[test]
fn c3_a_preview_reflects_the_moment_it_was_asked() {
    let first = review("ITER.1", &corpus_of(IN_FLIGHT), "now").expect("a preview");
    let promised = section(&first, "what it promised");
    assert_eq!(promised.rows[1][3], "nothing yet", "C2 has shown nothing");

    // The work moves. Ask again — no regeneration step, because there is nothing to
    // regenerate.
    let moved = IN_FLIGHT.replace(
        r#"claim "C2" from-slice="TS.a" state="pending""#,
        r#"claim "C2" from-slice="TS.a" state="met""#,
    );
    let second = review("ITER.1", &corpus_of(&moved), "now").expect("a preview");
    assert_ne!(first, second);
    assert_eq!(section(&second, "what it promised").rows[1][3], "settled");
}

#[test]
fn u3_a_declared_layer_with_no_evidence_is_listed_not_omitted() {
    // The residue ITER.260821.05 could not settle: the review view must LIST a layer the
    // slice declared and the iteration has not reached.
    let model = review("ITER.1", &corpus_of(IN_FLIGHT), "now").expect("a preview");
    let reached = section(&model, "what it has reached");
    assert_eq!(reached.rows.len(), 2, "both declared layers appear");
    assert_eq!(reached.rows[0][1], "evidenced");
    assert_eq!(reached.rows[1][0], "enforcement");
    assert_eq!(reached.rows[1][1], "unevidenced");
    assert!(reached.rows[1][2].contains("not reached"), "and it says so in words");
}

#[test]
fn a_layer_the_slice_never_declared_is_shown_as_undeclared() {
    // The other half of evidence-names-its-layer, surfaced where a human reads it rather
    // than only where the checker refuses it.
    let record = IN_FLIGHT.replace(
        r#"    layer "doctrine" state="evidenced" evidence="a skill that exists""#,
        "    layer \"doctrine\" state=\"evidenced\" evidence=\"a skill\"\n    layer \"harness\" state=\"evidenced\" evidence=\"a workflow\"",
    );
    let model = review("ITER.1", &corpus_of(&record), "now").expect("a preview");
    let reached = section(&model, "what it has reached");
    assert!(
        reached.rows.iter().any(|r| r[0] == "harness" && r[1].contains("NOT DECLARED")),
        "{reached:?}"
    );
}

#[test]
fn a_carried_claim_shows_the_finding_that_carries_it() {
    let record = IN_FLIGHT.replace(
        r#"    phase "implement" state="active" produced="the half that is done""#,
        "    phase \"implement\" state=\"active\" produced=\"the half that is done\"\n    finding \"F1\" carries=\"C2\"",
    );
    let model = review("ITER.1", &corpus_of(&record), "now").expect("a preview");
    assert_eq!(section(&model, "what it promised").rows[1][3], "carried by F1");
    assert_eq!(section(&model, "what is still open").rows[0], vec!["F1", "C2"]);
}

#[test]
fn every_empty_section_of_a_preview_still_says_why() {
    let record = r##"
thin-slice "TS.a" {
    slug "a-thing"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "open"
}
"##;
    let model = review("ITER.1", &corpus_of(record), "now").expect("a preview");
    assert!(model.flaws().is_empty(), "{:?}", model.flaws());
    assert!(model.sections.iter().all(|s| !s.is_empty() || s.empty_because.is_some()));
}

#[test]
fn reviewing_an_iteration_the_record_does_not_hold_answers_nothing() {
    assert!(review("ITER.9", &corpus_of(IN_FLIGHT), "now").is_none());
}
