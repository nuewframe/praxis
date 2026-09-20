//! `TS.260820.12` — decide what belongs in the published set, and refuse a choice made
//! without a reason.
//!
//! The publication test is one question: *if a reader finds this two years from now,
//! stamped with an old release, does it still do its job?* It is a judgement about
//! perishability, and an unjustified judgement is indistinguishable from an oversight.

use praxis_core::{Corpus, Publication, Schema, check_corpus, parse, publish, refused};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "event-storm" {
            field "read-model" each="0..n" holds="read-model"
        }
        entity "read-model" {
            field "answers" each="0..1"
        }
        entity "iteration" {
            field "on-slice" each="1"
            field "state"    each="1"
        }
        entity "release" {
            field "version" each="1"
            field "state"   each="1"
            field "binds"   each="0..n"
        }
        rule "publish-only-what-survives-freezing" refuses="an undeclared or unjustified lifetime"
    }
}
"##;

fn violations(record: &str) -> Vec<praxis_core::Violation> {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    check_corpus(&[schema_doc, record_doc], &schema)
}

fn corpus_of(record: &str) -> Corpus {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    Corpus::from_documents(&[schema_doc, record_doc], &schema)
}

#[test]
fn c1_a_read_model_with_no_declared_lifetime_is_refused() {
    let found = violations(
        r##"
event-storm "ES.1" {
    read-model "a-view-nobody-decided-about" answers="something"
}
"##,
    );
    assert!(refused(&found));
    let message = found[0].refusal.message();
    assert!(message.contains("a-view-nobody-decided-about"), "the view is NAMED: {message}");
    assert!(message.contains("no repository-wide default"), "{message}");
}

#[test]
fn c2_a_publishable_declaration_with_no_reason_is_refused() {
    let found = violations(
        r##"
event-storm "ES.1" {
    read-model "a-view" publishable=#true publishes-to="docs/releases/<version>/a.md"
}
"##,
    );
    assert!(refused(&found));
    assert!(found[0].refusal.message().contains("unjustified judgement"));
}

#[test]
fn an_on_demand_declaration_needs_a_reason_too() {
    // The judgement is the same judgement in both directions. Deciding NOT to publish
    // something is as much a decision about perishability as deciding to.
    let found = violations(
        r##"
event-storm "ES.1" {
    read-model "a-view" publishable=#false
}
"##,
    );
    assert!(refused(&found), "on-demand is a decision, not an absence of one");
}

#[test]
fn a_publishable_view_must_say_where_it_lands() {
    let found = violations(
        r##"
event-storm "ES.1" {
    read-model "a-view" publishable=#true because="it is useful frozen"
}
"##,
    );
    assert!(refused(&found));
    assert!(found[0].refusal.message().contains("publishes-to"));
}

#[test]
fn a_fully_declared_pair_of_views_raises_nothing() {
    let found = violations(
        r##"
event-storm "ES.1" {
    read-model "kept" publishable=#true because="useful frozen" publishes-to="docs/releases/<version>/kept.md"
    read-model "current" publishable=#false because="its value is being current"
}
"##,
    );
    assert!(found.is_empty(), "{found:?}");
}

const DECLARED: &str = r##"
event-storm "ES.1" {
    read-model "the-published-set-for-a-release" publishable=#true \
        because="a release note is useful only frozen" \
        publishes-to="docs/releases/<version>/release-notes.md"
    read-model "capabilities-and-what-they-own" publishable=#true \
        because="what the system could do at N is what a reader pinned to N needs" \
        publishes-to="docs/releases/<version>/capabilities/"
    read-model "what-is-ready-to-pick-up" publishable=#false \
        because="`right now` is in the question"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
}
release "REL.0.9.0" {
    version "0.9.0"
    state "planned"
    binds "ITER.1"
}
"##;

#[test]
fn c3_the_published_set_equals_exactly_the_publishable_declarations() {
    let Publication::Ready { documents, .. } = publish("0.9.0", &corpus_of(DECLARED)) else {
        panic!("expected a publication")
    };
    assert_eq!(documents.len(), 2, "two publishable declarations, two documents");
    let files: Vec<&str> = documents.iter().map(|d| d.file.as_str()).collect();
    assert!(files.contains(&"docs/releases/0.9.0/release-notes.md"));
    assert!(
        files.contains(&"docs/releases/0.9.0/capabilities/capabilities-and-what-they-own.md"),
        "a path ending in `/` is a directory, and the document inside is named for the view: {files:?}"
    );
    assert!(
        !files.iter().any(|f| f.contains("ready-to-pick-up")),
        "an on-demand view is not in the set"
    );
}

#[test]
fn c3_the_paths_come_from_the_record_not_from_the_engine() {
    // Before this slice the engine composed `what-shipped.md` and `capabilities.md`. The
    // record had said `release-notes.md` and `capabilities/` all along, and nothing read
    // it (ITER.260821.14/AD1).
    let Publication::Ready { documents, .. } = publish("0.9.0", &corpus_of(DECLARED)) else {
        panic!("expected a publication")
    };
    assert!(!documents.iter().any(|d| d.file.ends_with("what-shipped.md")));
}

#[test]
fn c3_a_declared_view_the_engine_cannot_compose_refuses_the_whole_publish() {
    // The set must EQUAL the declarations. A shorter set that quietly omits one is a
    // hand-listed set with extra steps.
    let record = DECLARED.replace(
        r#"    read-model "what-is-ready-to-pick-up" publishable=#false \
        because="`right now` is in the question""#,
        r#"    read-model "a-view-nobody-built" publishable=#true \
        because="declared, and unimplemented" \
        publishes-to="docs/releases/<version>/nothing.md""#,
    );
    let Publication::Refused(why) = publish("0.9.0", &corpus_of(&record)) else {
        panic!("a set that does not equal the declarations must not be published")
    };
    assert!(why[0].contains("a-view-nobody-built"));
    assert!(why[0].contains("rather than a shorter set"));
}

#[test]
fn a_record_declaring_nothing_publishable_refuses_rather_than_publishing_an_empty_set() {
    let record = DECLARED
        .replace("publishable=#true", "publishable=#false")
        .replace(r#" publishes-to="docs/releases/<version>/release-notes.md""#, "")
        .replace(r#" publishes-to="docs/releases/<version>/capabilities/""#, "");
    let Publication::Refused(why) = publish("0.9.0", &corpus_of(&record)) else {
        panic!("expected a refusal")
    };
    assert!(why[0].contains("gap in the declarations"), "{why:?}");
}
