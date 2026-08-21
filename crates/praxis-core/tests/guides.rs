//! `TS.260820.15` — write usage while building, and refuse to publish a guide for truth
//! that never shipped.
//!
//! A guide describing behaviour that never shipped is worse than no guide: it is a
//! document that is confidently wrong, and a reader has no way to tell.

use praxis_core::{Corpus, NoGuide, Schema, guide_for, guides, parse};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "capability" {
            field "state"   each="1"
            field "usage"   each="0..n"
            field "shipped" each="0..n"
        }
    }
}
"##;

fn corpus_of(record: &str) -> Corpus {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    Corpus::from_documents(&[schema_doc, record_doc], &schema)
}

const RECORD: &str = r##"
capability "shipped-and-documented" {
    state "active"
    usage "`praxis do-the-thing` — does the thing"
    shipped "0.1.0" by="ITER.1" slice="TS.a"
}
capability "shipped-and-undocumented" {
    state "active"
    shipped "0.1.0" by="ITER.2" slice="TS.b"
}
capability "documented-but-not-shipped" {
    state "sought"
    usage "`praxis do-the-other-thing` — will do the other thing"
}
"##;

fn section<'a>(model: &'a praxis_core::ReadModel, name: &str) -> &'a praxis_core::Section {
    model.sections.iter().find(|s| s.name == name).unwrap_or_else(|| panic!("a {name} section"))
}

#[test]
fn c1_a_guide_for_a_version_that_did_not_ship_it_is_refused() {
    let err = guide_for("documented-but-not-shipped", "0.1.0", &corpus_of(RECORD))
        .expect_err("usage exists, and 0.1.0 never promoted it");
    assert_eq!(
        err,
        NoGuide::NotPromoted {
            capability: "documented-but-not-shipped".to_owned(),
            version: "0.1.0".to_owned()
        }
    );
    let message = err.message();
    assert!(message.contains("documented-but-not-shipped"), "the capability is NAMED: {message}");
    assert!(message.contains("worse than no guide"), "{message}");
}

#[test]
fn c1_the_composed_guide_carries_only_what_the_release_promoted() {
    let model = guides("0.1.0", &corpus_of(RECORD));
    let written = section(&model, "how to use what shipped");
    assert_eq!(written.rows.len(), 1);
    assert_eq!(written.rows[0][0], "shipped-and-documented");
    assert!(
        !written.rows.iter().any(|r| r[0] == "documented-but-not-shipped"),
        "prose written ahead of the shipping is not this version's guide"
    );
}

#[test]
fn c1_prose_written_before_its_truth_shipped_is_named_as_timing_not_dropped() {
    // The absence has to read as "not yet" rather than as an oversight, or the next
    // person writes it again.
    let model = guides("0.1.0", &corpus_of(RECORD));
    let waiting = section(&model, "written, not yet shipped");
    assert_eq!(waiting.rows.len(), 1);
    assert_eq!(waiting.rows[0][0], "documented-but-not-shipped");
    assert!(waiting.rows[0][1].contains("did not promote its truth"));
}

#[test]
fn c2_usage_prose_comes_from_the_record_and_nowhere_else() {
    // `guides` takes a Corpus and nothing else. There is no path in the signature, so
    // there is nowhere to read a file from — the prose is data, not a pointer to data.
    let model = guides("0.1.0", &corpus_of(RECORD));
    let written = section(&model, "how to use what shipped");
    assert_eq!(written.rows[0][1], "`praxis do-the-thing` — does the thing");
    for row in &written.rows {
        for cell in row {
            assert!(!cell.contains(".md"), "no cell points at a file: {cell}");
        }
    }
}

#[test]
fn c3_a_shipped_capability_with_no_usage_is_reported_not_omitted() {
    let model = guides("0.1.0", &corpus_of(RECORD));
    let gaps = section(&model, "shipped without a guide");
    assert_eq!(gaps.rows.len(), 1);
    assert_eq!(gaps.rows[0][0], "shipped-and-undocumented");

    // And asking for it directly says the same thing rather than returning nothing.
    let err = guide_for("shipped-and-undocumented", "0.1.0", &corpus_of(RECORD))
        .expect_err("shipped, and nobody wrote how to use it");
    assert!(err.message().contains("silently missing"));
}

#[test]
fn a_capability_prefix_is_accepted_because_slices_write_it_that_way() {
    // Slices say `realizes "CAP.delivery-record"`; capability records are named without
    // the prefix. Accepting both is not sloppiness — it is one fact spelled two ways in
    // the record, and refusing one spelling would only move the problem.
    assert!(guide_for("CAP.shipped-and-documented", "0.1.0", &corpus_of(RECORD)).is_ok());
    assert!(guide_for("shipped-and-documented", "0.1.0", &corpus_of(RECORD)).is_ok());
}

#[test]
fn a_release_that_promoted_nothing_says_so_rather_than_publishing_an_empty_guide() {
    let model = guides("9.9.9", &corpus_of(RECORD));
    assert!(model.flaws().is_empty(), "{:?}", model.flaws());
    let written = section(&model, "how to use what shipped");
    assert!(written.is_empty());
    assert!(
        written.empty_because.as_deref().is_some_and(|w| w.contains("no shipped surface")),
        "an empty guide states why it is empty"
    );
}

#[test]
fn asking_about_a_capability_the_record_does_not_hold_answers_plainly() {
    let err = guide_for("nowhere", "0.1.0", &corpus_of(RECORD)).expect_err("no such capability");
    assert_eq!(err, NoGuide::NoSuchCapability("nowhere".to_owned()));
}
