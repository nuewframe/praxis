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
        entity "persona" {
            field "wants" each="1"
        }
        rule "a-frame-is-worked-under-a-strategy" reports="a frame naming no strategy"
        rule "work-follows-the-declared-sequence" reports="work at a declared step while an earlier step is satisfied by nothing"
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
    step "1" is="purpose" {
        satisfied-by "mission"
    }
    step "2" is="audience" {
        satisfied-by "persona"
    }
    step "3" is="work" {
        satisfied-by "thin-slice"
    }
}
"#;

/// The one persona that satisfies step 2. Kept apart from `ANCHOR` so a test can leave it
/// out and watch the sequence report fire.
const AUDIENCE: &str = r#"
persona "the-maintainer" {
    wants "to know what happened without asking the person who did it"
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
    // With the audience, because a-record-names-somebody-it-is-for reports a record holding
    // no persona at all — and that report is about a different fact than this one.
    let anchored = format!("{ANCHOR}{AUDIENCE}");
    assert!(violations(&anchored).is_empty(), "{:?}", violations(&anchored));

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
        .find(|d| d.model().view == "what-this-product-means")
        .expect("the concepts")
        .model();

    assert_eq!(
        model.sections.first().map(|s| s.name.as_str()),
        Some("why this exists"),
        "why comes first, before the problem and before the glossary"
    );
    let why = format!("{:?}", model.sections[0].rows);
    assert!(why.contains("a team can trust what an agent produced"));
    assert!(why.contains("what would count as delivered"));
}

/// C5 — a strategy is an ORDERING, and work out of sequence is reported.
///
/// This is what a strategy earns instead of a falsifier. It carries none because it is not a
/// claim about delivery — and that argument only holds if the ordering is checked, which for
/// a day it was not.
///
/// Computed from what the record HOLDS. A slice exists, nobody has been named, and the
/// strategy says audience comes before work.
#[test]
fn work_before_the_step_it_depends_on_is_reported() {
    let early = format!(
        "{ANCHOR}\nthin-slice \"TS.1\" {{\n    slug \"cut-before-anybody-was-named\"\n}}\n"
    );
    let found: Vec<_> = violations(&early)
        .into_iter()
        .filter(|v| v.refusal.rule() == "work-follows-the-declared-sequence")
        .collect();

    assert_eq!(found.len(), 1, "one gap, one report: {found:?}");
    assert_eq!(
        found[0].severity(),
        praxis_core::Severity::Report,
        "being out of order is a fact about a project's state, not a malformed record"
    );
    let said = found[0].refusal.message();
    assert!(said.contains("persona"), "the report names what the empty step wants: {said}");
    assert!(said.contains("\"3\""), "and which later step is already occupied: {said}");
}

/// C5, the other half — naming the person clears it.
///
/// The whole reason to compute this from state rather than from trail timestamps: a report
/// derived from when things were WRITTEN could never be cleared, and a report nobody can act
/// on is noise.
#[test]
fn naming_the_audience_clears_the_sequence_report() {
    let in_order = format!(
        "{ANCHOR}{AUDIENCE}\nthin-slice \"TS.1\" {{\n    slug \"cut-after-somebody-was-named\"\n}}\n"
    );
    assert!(
        !violations(&in_order)
            .iter()
            .any(|v| v.refusal.rule() == "work-follows-the-declared-sequence"),
        "writing the missing record clears it: {:?}",
        violations(&in_order)
    );
}

/// C5 — a strategy alone reports nothing, however little the record holds.
///
/// The rule fires on work at a LATER step, never on a step being empty. A greenfield project
/// that has stated its purpose and nothing else is at the start of the sequence rather than
/// out of it, and a tool that failed it there would be failing it during the hour it is most
/// needed.
#[test]
fn a_record_at_the_start_of_the_sequence_is_not_out_of_it() {
    assert!(
        !violations(ANCHOR)
            .iter()
            .any(|v| v.refusal.rule() == "work-follows-the-declared-sequence"),
        "nothing later is occupied, so there is no gap: {:?}",
        violations(ANCHOR)
    );
}

/// C5 — a step naming a kind the schema does not declare is unreachable by construction.
///
/// The strategy declares what satisfies each step and the engine computes the order. The
/// price of that is a mapping that can be wrong, and a step pointing at a kind nothing can
/// ever be is worse than an unmapped one: it reads as checked.
#[test]
fn a_step_satisfied_by_no_declared_kind_is_reported() {
    let typo = ANCHOR.replace("satisfied-by \"persona\"", "satisfied-by \"personna\"");
    let found: Vec<_> = violations(&typo)
        .into_iter()
        .filter(|v| matches!(v.refusal, praxis_core::Refusal::StepNamesNoKind { .. }))
        .collect();
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].refusal.message().contains("personna"));
}

/// C5 — an unmapped step is not a gap.
///
/// A strategy is prose somebody writes first and maps to kinds later. Requiring the mapping
/// upfront would make declaring one a schema exercise, which is how a strategy ends up
/// unwritten.
#[test]
fn a_step_that_names_no_kind_is_not_checked() {
    let unmapped = ANCHOR.replace("    step \"2\" is=\"audience\" {\n        satisfied-by \"persona\"\n    }", "    step \"2\" is=\"audience\"");
    let record = format!("{unmapped}\nthin-slice \"TS.1\" {{\n    slug \"a-slice\"\n}}\n");
    assert!(
        !violations(&record)
            .iter()
            .any(|v| v.refusal.rule() == "work-follows-the-declared-sequence"),
        "a step the author has not mapped declares nothing to check: {:?}",
        violations(&record)
    );
}
