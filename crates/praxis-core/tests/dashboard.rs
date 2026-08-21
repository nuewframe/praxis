//! `TS.260820.13` — see where the product stands right now, without committing a document
//! that ages.
//!
//! `E19`'s test: a DOCUMENT composes several read models; a READ MODEL still has exactly
//! one owner. And `E17`'s: this is the view the publication test rejects by name.

use praxis_core::{Composed, Conditions, Corpus, Schema, dashboard, parse};

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
        }
        entity "release" {
            field "version" each="1"
            field "state"   each="1"
            field "binds"   each="0..n"
        }
        entity "capability" {
            field "state" each="1"
        }
    }
    admits "an iteration on a slice" {
        condition "slice-is-shaped" check="slice-is-shaped" because="a refused slice is a defect"
        condition "the-best-available" judgement=#true because="the tool does not rank"
    }
}
"##;

const RECORD: &str = r##"
thin-slice "TS.a" {
    slug "a-thing"
    realizes "CAP.alpha"
    claim "C1"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    claim "C1" from-slice="TS.a" state="met"
}
capability "alpha" {
    state "sought"
}
release "REL.0.1.0" {
    version "0.1.0"
    state "planned"
    binds "ITER.1"
}
"##;

fn compose(record: &str, as_of: &str) -> Composed {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let conditions = Conditions::from_document(&schema_doc);
    let corpus = Corpus::from_documents(&[schema_doc, record_doc], &schema);
    dashboard(&corpus, &conditions, as_of)
}

#[test]
fn c1_three_results_from_three_capabilities_compose_with_no_renderer_change() {
    let document = compose(RECORD, "now");
    assert_eq!(document.parts.len(), 3);
    // Composition is ordering, not interpretation: every part is the same type, and the
    // document knows only who owns each one.
    for part in &document.parts {
        assert!(part.owner.starts_with("CAP."));
        assert!(part.model.flaws().is_empty(), "{:?}", part.model.flaws());
    }
}

#[test]
fn c1_no_owner_appears_in_more_than_one_result() {
    // E19: a composite document would otherwise need a cross-capability owner, and the
    // one-owner rule would break.
    let document = compose(RECORD, "now");
    assert!(document.owners_are_distinct());
    let owners: Vec<&str> = document.parts.iter().map(|p| p.owner.as_str()).collect();
    assert_eq!(
        owners,
        vec!["CAP.delivery-record", "CAP.work-admission", "CAP.release-binding"]
    );
}

#[test]
fn c2_the_dashboard_is_marked_as_of_a_moment_never_as_of_a_release() {
    let document = compose(RECORD, "2026-08-21T09:00:00Z");
    assert_eq!(document.as_of, "2026-08-21T09:00:00Z");
    for part in &document.parts {
        assert_eq!(part.model.as_of, "2026-08-21T09:00:00Z", "{} carries the moment", part.owner);
        assert!(!part.model.publishable, "{} is not publishable", part.owner);
    }
}

#[test]
fn c2_the_release_part_carries_the_moment_even_though_published_it_would_carry_the_version() {
    // The same content, the same seam, a different lifetime. Composed on demand it is
    // current; published into a release directory it depicts a version and carries no
    // clock. That the SAME result can do both is what makes read-model@v1 a seam.
    let document = compose(RECORD, "now");
    let release_part = document
        .parts
        .iter()
        .find(|p| p.owner == "CAP.release-binding")
        .expect("the release part");
    assert_eq!(release_part.model.as_of, "now");
    assert_eq!(release_part.model.view, "the-published-set-for-a-release");
}

#[test]
fn c3_asking_twice_across_a_state_change_gives_two_different_answers() {
    let before = compose(RECORD, "now");
    let moved = RECORD.replace(
        r#"claim "C1" from-slice="TS.a" state="met""#,
        r#"claim "C1" from-slice="TS.a" state="carried""#,
    );
    let after = compose(&moved, "now");
    assert_ne!(before, after, "no regeneration step — the only input is the record");
}

#[test]
fn c4_composing_produces_no_file_at_all() {
    // Structural, like the preview: `dashboard` returns values. There is no path in the
    // type, so nothing under docs/releases/ can be reached from here.
    let document = compose(RECORD, "now");
    for part in &document.parts {
        for section in &part.model.sections {
            for row in &section.rows {
                for cell in row {
                    assert!(!cell.contains("docs/releases/"), "{cell}");
                }
            }
        }
    }
}

#[test]
fn the_readiness_part_is_omitted_when_the_record_declares_no_gate() {
    // An empty gate is not an open one. Rather than compute readiness from an assumed
    // set of conditions, the part is left out — and its absence is visible because the
    // owner is missing from the document.
    let schema_doc = parse(SCHEMA.replace("admits \"an iteration on a slice\"", "not-admits").as_str())
        .expect("parses");
    let record_doc = parse(RECORD).expect("parses");
    let schema = Schema::from_document(&schema_doc);
    let corpus = Corpus::from_documents(&[schema_doc, record_doc], &schema);
    let document = dashboard(&corpus, &Conditions::default(), "now");
    assert!(!document.parts.iter().any(|p| p.owner == "CAP.work-admission"));
    assert_eq!(document.parts.len(), 2);
}

#[test]
fn a_record_with_no_release_composes_the_parts_it_has() {
    let record = RECORD.replace(
        r##"release "REL.0.1.0" {
    version "0.1.0"
    state "planned"
    binds "ITER.1"
}"##,
        "",
    );
    let document = compose(&record, "now");
    assert_eq!(document.parts.len(), 2);
    assert!(document.owners_are_distinct());
}

#[test]
fn a_document_whose_owners_repeat_is_detectable() {
    // The check is on the DOCUMENT rather than on any model, because that is where the
    // rule can be broken.
    let mut document = compose(RECORD, "now");
    let first = document.parts[0].clone();
    document.parts.push(first);
    assert!(!document.owners_are_distinct());
}
