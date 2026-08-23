//! `TS.260820.03` — ask the record what this repository already knows.
//!
//! This is `S7`: the agent who arrives with none of the previous one's context and
//! reconstructs it from prose. The answer is computed on every ask, and it says what it
//! does not cover, so silence is not read as absence.

use praxis_core::{Corpus, Schema, parse, what_is_currently_true};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug"     each="1"
            field "realizes" each="1"
            field "claim"    each="0..n"
        }
        entity "iteration" {
            field "on-slice" each="1"
            field "state"    each="1"
            field "claim"    each="0..n"
            field "finding"  each="0..n"
        }
        entity "release" {
            field "version" each="1"
            field "state"   each="1"
            field "binds"   each="0..n"
        }
        entity "capability" id-prefix="CAP." {
            field "state"      each="1"
            field "owns-event" each="0..n"
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

const KNOWN: &str = r##"
thin-slice "TS.a" {
    slug "a-thing"
    realizes "CAP.alpha"
    claim "C1"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    claim "C1" from-slice="TS.a" state="met"
    finding "F1" carries="C1"
}
capability "alpha" {
    state "sought"
    owns-event "One" "Two" "Three"
}
release "REL.0.1.0" {
    version "0.1.0"
    state "planned"
    binds "ITER.1"
}
"##;

fn section<'a>(model: &'a praxis_core::ReadModel, name: &str) -> &'a praxis_core::Section {
    model.sections.iter().find(|s| s.name == name).unwrap_or_else(|| panic!("a {name} section"))
}

#[test]
fn c1_the_answer_changes_with_the_record_and_has_no_regeneration_step() {
    let before = what_is_currently_true(&corpus_of(KNOWN), "now");
    assert_eq!(section(&before, "slices delivered").rows.len(), 1);

    // Mutate the record — the iteration no longer meets its claim — and ask again. There
    // is no cache to invalidate and no summary to regenerate, because the only input is
    // the record itself.
    let moved = KNOWN.replace(r#"claim "C1" from-slice="TS.a" state="met""#, r#"claim "C1" from-slice="TS.a" state="carried""#);
    let after = what_is_currently_true(&corpus_of(&moved), "now");
    assert_eq!(section(&after, "slices delivered").rows.len(), 0);
    assert_eq!(section(&after, "slices outstanding").rows.len(), 1);
}

#[test]
fn c2_an_empty_area_says_nothing_recorded_rather_than_returning_nothing() {
    let empty = what_is_currently_true(&corpus_of("// a record that holds nothing yet\n"), "now");
    assert!(empty.flaws().is_empty(), "{:?}", empty.flaws());
    for name in ["capabilities", "releases", "slices delivered", "what is owed"] {
        let section = section(&empty, name);
        assert!(section.is_empty());
        let why = section.empty_because.as_deref().expect("an empty section states why");
        assert!(why.contains("nothing recorded"), "{name}: {why}");
    }
}

#[test]
fn the_answer_says_what_it_does_not_cover() {
    // A view that does not state its edges is one a reader will over-trust — the
    // difference between an answer and an impression.
    let model = what_is_currently_true(&corpus_of(KNOWN), "now");
    let excluded = section(&model, "not covered by this answer");
    assert!(!excluded.is_empty());
    assert!(excluded.rows.iter().any(|r| r[0].contains("in flight")));
    assert!(excluded.rows.iter().any(|r| r[1].contains("praxis ready")));
}

#[test]
fn a_capability_owning_several_events_reports_all_of_them() {
    // `owns-event "One" "Two" "Three"` is ONE node carrying three values. Reading only
    // the first reported every capability in the real record as owning exactly one event,
    // which the checker never noticed because it counts values and this counted nodes.
    let model = what_is_currently_true(&corpus_of(KNOWN), "now");
    let capabilities = section(&model, "capabilities");
    assert_eq!(capabilities.rows[0][2], "3");
}

#[test]
fn what_is_owed_carries_findings_from_closed_iterations() {
    let model = what_is_currently_true(&corpus_of(KNOWN), "now");
    let owed = section(&model, "what is owed");
    assert_eq!(owed.rows.len(), 1);
    assert_eq!(owed.rows[0], vec!["ITER.1", "F1", "C1"]);
}

#[test]
fn an_open_iteration_contributes_nothing_to_what_is_owed() {
    // Work in flight is another question, and this view says so rather than half-answering.
    let record = KNOWN.replace(r#"    state "closed""#, r#"    state "working""#);
    let model = what_is_currently_true(&corpus_of(&record), "now");
    assert!(section(&model, "what is owed").is_empty());
}

#[test]
fn this_view_carries_a_moment_because_its_value_is_being_current() {
    // The exact opposite of an archival result, which names the version it depicts and no
    // clock at all. Same type, opposite requirement.
    let model = what_is_currently_true(&corpus_of(KNOWN), "2026-08-21T09:00:00Z");
    assert_eq!(model.as_of, "2026-08-21T09:00:00Z");
    assert!(!model.publishable, "and it is never committed");
}
