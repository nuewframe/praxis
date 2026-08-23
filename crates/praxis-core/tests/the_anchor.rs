//! `TS.260821.16` — hold what the product is for, above the problem it attacks.
//!
//! Everything this record held descended from a frame, and a frame is a problem. `root-cause`
//! says why the PROBLEM exists; nothing said why the PRODUCT does. So a published release
//! could tell a reader HOW, because usage is in the record, and could not tell them WHY.
//!
//! Vision and mission are not checked for truth. They are what truth is checked against, and
//! the check is the one this record makes everywhere else: that nothing is orphaned.

use praxis_core::{Corpus, Known, Publication, Schema, check_corpus, check_node, index_all, parse, publish};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "vision" {
            field "is" each="1"
        }
        entity "mission" {
            field "is"             each="1"
            field "toward"         each="1" references="vision"
            field "delivered-when" each="1"
        }
        entity "strategy" {
            field "serves" each="1" references="mission"
            field "step"   each="1..n"
        }
        entity "frame" {
            field "title" each="1"
            field "under" each="0..1" references="strategy"
        }
        entity "event-storm" {
            field "read-model" each="0..n"
        }
        entity "release" {
            field "version" each="1"
            field "state"   each="1"
            field "binds"   each="0..n"
        }
        entity "iteration" {
            field "on-slice" each="1..n"
            field "state"    each="1"
        }
        entity "thin-slice" {
            field "slug" each="1"
        }
        rule "a-frame-is-worked-under-a-strategy" reports="a frame naming no strategy"
    }
}
"##;

const ANCHOR: &str = r#"
vision "trustworthy" {
    is "a team can trust what an agent produced without re-deriving it"
}
mission "computed" {
    is "make fidelity a property computed from the record"
    toward "trustworthy"
    delivered-when "somebody who did not do the work can ask the record and act on the answer"
}
strategy "person-first" {
    serves "computed"
    step "1" is="purpose"
    step "2" is="audience"
}
"#;

fn violations(record: &str) -> Vec<praxis_core::Violation> {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let mut known = Known::default();
    index_all(&schema_doc, &mut known);
    index_all(&record_doc, &mut known);
    let mut all: Vec<_> = record_doc
        .nodes()
        .iter()
        .flat_map(|n| check_node(n, &schema, &known))
        .collect();
    all.extend(check_corpus(&[schema_doc, record_doc], &schema));
    all
}

/// C1 — the mission carries what would count as delivered, and one without it is refused.
///
/// The VISION carries none: a world state cannot be tested, and asking whether it is true is
/// the category error that kept all three of these out of the record.
#[test]
fn a_mission_with_no_test_is_refused_and_a_vision_needs_none() {
    assert!(violations(ANCHOR).is_empty(), "{:?}", violations(ANCHOR));

    let untested = ANCHOR.replace(
        "    delivered-when \"somebody who did not do the work can ask the record and act on the answer\"\n",
        "",
    );
    assert!(
        violations(&untested).iter().any(|v| v.refusal.message().contains("delivered-when")),
        "a mission nobody could be wrong about is a slogan: {:?}",
        violations(&untested)
    );
}

/// C2 — the chain frame → strategy → mission → vision is followed by dangling-relationship,
/// with no rule of its own.
#[test]
fn the_chain_upward_is_checked_by_the_edges_it_already_has() {
    for (broken, what) in [
        (ANCHOR.replace("toward \"trustworthy\"", "toward \"no-such-vision\""), "mission → vision"),
        (ANCHOR.replace("serves \"computed\"", "serves \"no-such-mission\""), "strategy → mission"),
    ] {
        let found: Vec<_> = violations(&broken)
            .into_iter()
            .filter(|v| v.refusal.rule() == "dangling-relationship")
            .collect();
        assert_eq!(found.len(), 1, "{what} dangles without a rule of its own: {found:?}");
    }

    let orphan_frame = format!("{ANCHOR}\nframe \"FRAME.1\" {{\n    title \"a problem\"\n    under \"no-such-strategy\"\n}}\n");
    assert!(
        violations(&orphan_frame)
            .iter()
            .any(|v| v.refusal.rule() == "dangling-relationship")
    );
}

/// C2, the other half — a frame naming no strategy is REPORTED, not refused.
///
/// One frame exists and it predates all three kinds. Every enforcement in this frame that
/// failed closed on arrival had to be walked back.
#[test]
fn a_frame_under_no_strategy_is_reported_and_not_refused() {
    let record = format!("{ANCHOR}\nframe \"FRAME.1\" {{\n    title \"a problem\"\n}}\n");
    let found: Vec<_> = violations(&record)
        .into_iter()
        .filter(|v| v.refusal.rule() == "a-frame-is-worked-under-a-strategy")
        .collect();

    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0].severity(), praxis_core::Severity::Report);

    let anchored = format!("{ANCHOR}\nframe \"FRAME.1\" {{\n    title \"a problem\"\n    under \"person-first\"\n}}\n");
    assert!(
        !violations(&anchored)
            .iter()
            .any(|v| v.refusal.rule() == "a-frame-is-worked-under-a-strategy")
    );
}

/// C4 — the release opens with why, before how.
///
/// A reader told what a tool does and never why it exists has been handed a manual for a
/// decision they have not made yet.
#[test]
fn the_published_concepts_open_with_why() {
    let record = format!(
        "{ANCHOR}\nthin-slice \"TS.1\" {{\n    slug \"a-slice\"\n}}\n\
         iteration \"ITER.1\" {{\n    on-slice \"TS.1\"\n    state \"closed\"\n}}\n\
         release \"REL.0.1.0\" {{\n    version \"0.1.0\"\n    state \"planned\"\n    binds \"ITER.1\"\n}}\n\
         event-storm \"ES.1\" {{\n    read-model \"what-this-product-means\" \
         answers=\"what does it mean?\" publishable=#true publishes-to=\"docs/releases/<v>/concepts/\"\n}}\n"
    );
    let schema_doc = parse(SCHEMA).expect("parses");
    let record_doc = parse(&record).expect("parses");
    let schema = Schema::from_document(&schema_doc);
    let corpus = Corpus::from_documents(&[schema_doc, record_doc], &schema);

    let documents = match publish("0.1.0", &corpus) {
        Publication::Ready { documents, .. } => documents,
        Publication::Refused(why) => panic!("publish refused: {why:?}"),
    };
    let model = &documents
        .iter()
        .find(|d| d.model.view == "what-this-product-means")
        .expect("the concepts")
        .model;

    assert_eq!(
        model.sections.first().map(|s| s.name.as_str()),
        Some("why this exists"),
        "why comes first, before the problem and before the glossary"
    );
    let why = format!("{:?}", model.sections[0].rows);
    assert!(why.contains("a team can trust what an agent produced"));
    assert!(why.contains("what would count as delivered"));
}
