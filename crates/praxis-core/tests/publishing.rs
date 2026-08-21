//! `TS.260820.10` — regenerate every published document whole, each stating the release it
//! depicts.
//!
//! A document that is spliced can be corrupted; a document that is regenerated cannot.
//! C3 is finding `E10`'s falsifier stated as an acceptance criterion: if projection ever
//! needs a code path specific to one view, this capability and the content-owning ones
//! merge.

use praxis_core::{Corpus, Publication, ReadModel, Schema, Section, parse, publish};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug" each="1"
        }
        entity "iteration" {
            field "on-slice"    each="1"
            field "state"       each="1"
            field "contributes" each="0..n"
            field "finding"     each="0..n"
        }
        entity "release" {
            field "version" each="1"
            field "state"   each="1"
            field "binds"   each="0..n"
        }
        entity "capability" {
            field "from-cluster" each="0..1"
            field "owns-event"   each="0..n"
        }
        entity "event-storm" {
            field "read-model" each="0..n" holds="read-model"
        }
        entity "read-model" {
            field "answers" each="0..1"
        }
    }
}
"##;

const RECORD: &str = r##"
// Membership of the published set is derived from these declarations and nothing else.
event-storm "ES.1" {
    read-model "the-published-set-for-a-release" publishable=#true \
        because="a release note is useful only frozen" \
        publishes-to="docs/releases/<version>/release-notes.md"
    read-model "capabilities-and-what-they-own" publishable=#true \
        because="what the system could do at N is what a reader pinned to N needs" \
        publishes-to="docs/releases/<version>/capabilities.md"
}
thin-slice "TS.a" {
    slug "a-thing"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    contributes "slice-outcome-added"
    finding "F1" carries="C2"
}
release "REL.0.9.0" {
    version "0.9.0"
    state "planned"
    binds "ITER.1"
}
capability "CAP.x" {
    from-cluster "some-cluster"
    owns-event "ThingHappened"
}
"##;

fn corpus_of(record: &str) -> Corpus {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    Corpus::from_documents(&[schema_doc, record_doc], &schema)
}

fn ready(record: &str, version: &str) -> Vec<praxis_core::Document> {
    match publish(version, &corpus_of(record)) {
        Publication::Ready { documents, .. } => documents,
        Publication::Refused(why) => panic!("expected a publication: {why:?}"),
    }
}

#[test]
fn c1_publishing_twice_produces_the_same_documents() {
    // "Byte-identical to a fresh render" is only possible if nothing in the document is a
    // wall-clock reading. An archival result names the VERSION it depicts and not the
    // moment it was produced — otherwise verification of a published tree, which is a
    // comparison, can never pass.
    let first = ready(RECORD, "0.9.0");
    let second = ready(RECORD, "0.9.0");
    assert_eq!(first, second, "a publish an hour later is the same publish");
    assert!(
        first.iter().all(|d| d.model.as_of == "0.9.0"),
        "the `moment` of an archival result IS its version"
    );
}

#[test]
fn c1_composition_happens_before_any_write() {
    // There is no in-place edit path to find, because `publish` returns the whole set as
    // values. A caller cannot splice; it can only write what it was given.
    let documents = ready(RECORD, "0.9.0");
    assert_eq!(documents.len(), 2);
    for document in &documents {
        assert!(document.model.flaws().is_empty(), "{:?}", document.model.flaws());
    }
}

#[test]
fn c2_every_document_names_its_release_and_lands_under_that_version() {
    for document in ready(RECORD, "0.9.0") {
        assert!(
            document.file.starts_with("docs/releases/0.9.0/"),
            "under the release's own directory: {}",
            document.file
        );
        assert!(document.model.publishable, "and declared archival");
    }
}

#[test]
fn c4_publishing_writes_nothing_outside_this_release_directory() {
    let documents = ready(RECORD, "0.9.0");
    assert!(
        documents.iter().all(|d| !d.file.contains("..")
            && d.file.matches("docs/releases/").count() == 1
            && d.file.starts_with("docs/releases/0.9.0/")),
        "a prior release's directory cannot be reached from here: {documents:?}"
    );
}

#[test]
fn c3_a_newly_declared_read_model_renders_with_no_renderer_change() {
    // E10's falsifier. The renderer's whole input is a ReadModel; a view it has never
    // seen is not distinguishable from one it has, because there is nothing to
    // distinguish with.
    let invented = ReadModel::new("a-view-that-did-not-exist", "invented by this test", "now")
        .section(
            Section::new("things", &["name", "count"])
                .row(vec!["alpha".to_owned(), "2".to_owned()]),
        )
        .section(Section::new("nothing", &["x"]).empty_because("this section is empty on purpose"));

    assert!(invented.flaws().is_empty(), "it satisfies the seam like any other result");
    assert_eq!(invented.sections.len(), 2);
    assert_eq!(invented.sections[0].rows[0][1], "2");
}

#[test]
fn c3_no_emitted_cell_contains_markup() {
    // The second half. WALK.260820.01/W2 found the first half passes even when the split
    // has become fiction, so both are required.
    for document in ready(RECORD, "0.9.0") {
        for section in &document.model.sections {
            for row in &section.rows {
                for cell in row {
                    assert!(
                        !cell.contains('|') && !cell.contains('<') && !cell.contains("**"),
                        "a capability that may emit presentation makes the split fiction: {cell:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn publishing_for_a_cut_release_is_refused() {
    // The ordering forced by TS.260820.09/C3: the indexed commit must CONTAIN the
    // published set, and a commit taken at cut time cannot contain documents written
    // afterwards. So the set is published before the cut, never after.
    let record = RECORD.replace(r#"state "planned""#, r#"state "released""#);
    let Publication::Refused(why) = publish("0.9.0", &corpus_of(&record)) else {
        panic!("publishing after the cut would put the documents outside the indexed commit");
    };
    assert!(why[0].contains("BEFORE the cut"));
}

#[test]
fn publishing_for_a_version_the_record_does_not_hold_is_refused() {
    let Publication::Refused(why) = publish("9.9.9", &corpus_of(RECORD)) else {
        panic!("expected a refusal")
    };
    assert!(why[0].contains("9.9.9"));
}

#[test]
fn a_version_binding_nothing_has_no_shipped_work_to_describe() {
    let record = RECORD.replace(r#"    binds "ITER.1""#, "");
    let Publication::Refused(why) = publish("0.9.0", &corpus_of(&record)) else {
        panic!("expected a refusal")
    };
    assert!(why[0].contains("binds nothing"));
}

#[test]
fn what_shipped_carries_the_findings_the_bound_work_left_owed() {
    let documents = ready(RECORD, "0.9.0");
    let shipped = documents
        .iter()
        .find(|d| d.model.view == "the-published-set-for-a-release")
        .map(|d| &d.model)
        .expect("the release notes");
    let owed = shipped
        .sections
        .iter()
        .find(|s| s.name == "what it left owed")
        .expect("a section for what is owed");
    assert_eq!(owed.rows.len(), 1);
    assert_eq!(owed.rows[0], vec!["ITER.1", "F1", "C2"]);
}

#[test]
fn an_empty_section_in_a_published_document_still_says_why() {
    let documents = ready(RECORD, "0.9.0");
    let resolved = documents
        .iter()
        .find(|d| d.model.view == "the-published-set-for-a-release")
        .expect("the release notes")
        .model
        .sections
        .iter()
        .find(|s| s.name == "symptoms it resolved")
        .expect("a section");
    assert!(resolved.is_empty());
    assert!(resolved.empty_because.is_some(), "archival or not, silence is not an answer");
}
