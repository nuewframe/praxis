//! `TS.260821.07` — the doctrine a version shipped, derived rather than scraped.
//!
//! `gen-coverage-matrix.sh` learned the language coverage by grepping every probe's
//! `--include` list. `gen-doctrine-index.sh` scraped frontmatter into a table. Both described
//! the working tree, so neither could answer "what did 0.8.0 ship" — a different question
//! wearing the same words.

use praxis_core::view::ReadModel;
use praxis_core::{Corpus, Publication, Schema, parse, publish};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "event-storm" {
            field "read-model" each="0..n"
        }
        entity "thin-slice" {
            field "slug" each="1"
        }
        entity "iteration" {
            field "on-slice" each="1..n"
            field "state"    each="1"
        }
        entity "release" {
            field "version" each="1"
            field "state"   each="1"
            field "binds"   each="0..n"
        }
        entity "invariant" {
            field "protects"   each="1"
            field "severity"   each="1"
            field "language"   each="0..n"
            field "structural" each="0..1"
        }
        entity "doctrine-surface" states="current retired" {
            field "path"   each="1"
            field "kind"   each="1"
            field "serves" each="1..n"
            field "state"  each="0..1"
        }
    }
}
"##;

const RECORD: &str = r##"
event-storm "ES.test" {
    read-model "what-this-plugin-ships" answers="what shipped?" publishable=#true \
        publishes-to="docs/releases/<version>/doctrine/"
}
thin-slice "TS.old" {
    slug "shipped-earlier"
}
thin-slice "TS.new" {
    slug "shipped-later"
}
iteration "ITER.old" {
    on-slice "TS.old"
    state "closed"
}
iteration "ITER.new" {
    on-slice "TS.new"
    state "closed"
}
release "REL.0.1.0" {
    version "0.1.0"
    state "cut"
    binds "ITER.old"
}
release "REL.0.2.0" {
    version "0.2.0"
    state "cut"
    binds "ITER.new"
}
invariant "gated" {
    protects "something that fails closed"
    severity "refuse"
    language "python"
}
invariant "noticed" {
    protects "something that only reports"
    severity "report"
    structural "reads structure"
}
doctrine-surface "surface.old" {
    path "skills/old/SKILL.md"
    kind "skill"
    serves "TS.old"
}
doctrine-surface "surface.new" {
    path "skills/new/SKILL.md"
    kind "skill"
    serves "TS.new"
}
doctrine-surface "surface.probe" {
    path "scripts/check-gated.sh"
    kind "probe"
    serves "gated"
}
doctrine-surface "surface.gone" {
    path "skills/gone/SKILL.md"
    kind "skill"
    serves "TS.old"
    state "retired"
}
"##;

fn published(version: &str) -> ReadModel {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(RECORD).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let corpus = Corpus::from_documents(&[schema_doc, record_doc], &schema);
    match publish(version, &corpus) {
        Publication::Ready { documents, .. } => documents
            .into_iter()
            .find(|d| d.file.contains("what-this-plugin-ships"))
            .expect("the view is declared publishable, so the publisher must have a composer")
            .model,
        Publication::Refused(why) => panic!("publish refused: {why:?}"),
    }
}

/// Every cell of one section, flattened. Asserting on the MODEL rather than on rendered
/// Markdown is the same distinction the slice is about: the document is a projection, and a
/// test that reads the projection is a test that could pass on a broken record.
fn cells(model: &ReadModel, section: &str) -> Vec<String> {
    model
        .sections
        .iter()
        .filter(|s| s.name == section)
        .flat_map(|s| s.rows.iter().flatten().cloned())
        .collect()
}

/// C1 — the set is derived from the record and regenerates identically.
#[test]
fn the_shipped_set_is_derived_and_regenerates_identically() {
    let once = published("0.2.0");
    let twice = published("0.2.0");
    assert_eq!(cells(&once, "doctrine shipped"), cells(&twice, "doctrine shipped"));
    assert_eq!(once.defines.iter().find(|(k, _)| k == "depicts").map(|(_, v)| v.as_str()), Some("0.2.0"));
}

/// C4 — the set published for an older version still describes that version.
///
/// This is the claim the retired generators could not make at all: both read the working
/// tree, so both answered the same way whichever version you asked about.
#[test]
fn an_older_version_is_not_described_by_the_current_tree() {
    let shipped = cells(&published("0.1.0"), "doctrine shipped");

    // `shipped in` is DERIVED: serves names a slice, an iteration covers it, that iteration
    // bound to a release. Nothing stores a version on a surface.
    assert!(shipped.contains(&"skills/old/SKILL.md".to_owned()));
    assert!(
        shipped.contains(&"0.1.0".to_owned()) && shipped.contains(&"0.2.0".to_owned()),
        "each surface carries the release derived from what IT serves, not the release being \
         published — which is the whole of what the scraped generators could not do: they \
         read the tree, so they answered the same way whichever version you asked about. \
         Found: {shipped:?}"
    );
    // The version being published is on the document, not smeared across its rows.
    assert_eq!(published("0.1.0").defines.iter().find(|(k, _)| k == "depicts").map(|(_, v)| v.as_str()), Some("0.1.0"));
}

/// C2 — severity is published, because it is the only thing about a probe an adopter must
/// not get wrong.
#[test]
fn a_gate_and_a_notice_are_published_differently() {
    let guarantees = cells(&published("0.2.0"), "what it guarantees");

    assert!(guarantees.contains(&"fails closed".to_owned()), "{guarantees:?}");
    assert!(
        guarantees.contains(&"reports".to_owned()),
        "the reporting invariant must not claim to be a gate: {guarantees:?}"
    );
    // A structural probe says so rather than being listed against nine languages it does
    // not scan — which is exactly what the grepped coverage matrix used to do.
    assert!(guarantees.contains(&"every language — it reads structure, not text".to_owned()));
    assert!(guarantees.contains(&"python".to_owned()));
}

/// A retired surface is not shipped, and is not silently dropped either. Where it stands is
/// what makes removing it recoverable by name.
#[test]
fn a_retired_surface_is_listed_apart_rather_than_dropped() {
    let doc = published("0.2.0");
    let gone = "skills/gone/SKILL.md".to_owned();

    assert!(
        !cells(&doc, "doctrine shipped").contains(&gone),
        "retired doctrine is not shipped doctrine"
    );
    assert!(
        cells(&doc, "retired, and where it still stands").contains(&gone),
        "and it is not dropped from the document either — where it stands is what makes \
         removing it recoverable by name"
    );
}
