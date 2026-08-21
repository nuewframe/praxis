//! `TS.260820.04` — see which slices could be started right now, and what would refuse
//! each of the rest.
//!
//! C1 (the settleable half): the view computes no condition of its own. Every verdict it
//! shows came from `assess`, which is what the gate at `TS.260820.05` will call. The
//! residue — that the gate does call it — is owed against that slice and recorded as S1
//! in ITER.260821.03, not asserted here.
//!
//! C2: every excluded slice names its blocking condition, and none has an empty reason.

use praxis_core::{Conditions, Corpus, Schema, Verdict, assess, parse, project};

/// A schema minimal enough to shape a slice and an iteration, and nothing else. The
/// tests build their own record: the point is what the engine does with a declaration,
/// never what today's tree happens to contain.
const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug"       each="1"
            field "kind"       each="1"
            field "realizes"   each="1"
            field "layer"      each="0..n"
            field "depends-on" each="0..n"
            field "state"      each="0..1"
        }
        entity "iteration" {
            field "on-slice" each="1"
            field "state" each="1"
            field "claim" each="0..n"
        }
    }
    admits "an iteration on a slice" {
        condition "slice-is-shaped" check="slice-is-shaped" because="a refused slice is a defect"
        condition "dependencies-delivered" check="dependencies-delivered" because="standing on nothing"
        condition "no-iteration-in-flight" check="no-iteration-in-flight" because="one open per slice"
        condition "nothing-left-to-admit" check="nothing-left-to-admit" because="done is not blocked"
        condition "work-is-disjoint" uncomputed="no seam manifest exists" because="overlap is a merge"
        condition "the-best-available" judgement=#true because="the tool does not rank"
    }
}
"##;

fn build(record: &str) -> (Corpus, praxis_core::Assessment) {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let conditions = Conditions::from_document(&schema_doc);
    assert!(!conditions.is_empty(), "the record declares conditions");
    let docs = vec![schema_doc.clone(), record_doc];
    let corpus = Corpus::from_documents(&docs, &schema);
    let assessment = assess(&corpus, &conditions, "2026-08-21T00:00:00Z");
    (corpus, assessment)
}

fn verdict<'a>(
    assessment: &'a praxis_core::Assessment,
    slice: &str,
    condition: &str,
) -> &'a Verdict {
    assessment
        .verdicts
        .get(slice)
        .unwrap_or_else(|| panic!("{slice} was assessed"))
        .iter()
        .find(|(c, _)| c == condition)
        .map(|(_, v)| v)
        .unwrap_or_else(|| panic!("{condition} was evaluated for {slice}"))
}

#[test]
fn c1_the_view_shows_exactly_what_the_assessment_admits() {
    let (corpus, assessment) = build(
        r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
    layer "docs"
}
thin-slice "TS.b" {
    slug "b"
    kind "command"
    realizes "CAP.x"
    layer "docs"
    depends-on "TS.a — first"
}
"##,
    );
    let model = project(&assessment, &corpus);
    let ready: Vec<&str> = model
        .sections
        .iter()
        .find(|s| s.name == "ready")
        .expect("a ready section")
        .rows
        .iter()
        .map(|r| r[0].as_str())
        .collect();

    // The projection re-derives nothing: its ready set is the assessment's ready set,
    // less the slices that are finished rather than admissible.
    let expected: Vec<&str> = assessment
        .ready()
        .into_iter()
        .filter(|id| !assessment.finished(id))
        .collect();
    assert_eq!(ready, expected);
    assert_eq!(ready, vec!["TS.a"], "b waits on a, which nothing has delivered");
}

#[test]
fn c2_no_exclusion_has_an_empty_reason() {
    let (corpus, assessment) = build(
        r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
    layer "docs"
}
thin-slice "TS.b" {
    slug "b"
    kind "command"
    realizes "CAP.x"
    layer "docs"
    depends-on "TS.a — first"
}
thin-slice "TS.c" {
    slug "c"
    kind "command"
    realizes "CAP.x"
    layer "docs"
}
iteration "ITER.1" {
    on-slice "TS.c"
    state "working"
}
"##,
    );
    let model = project(&assessment, &corpus);
    let blocked = model
        .sections
        .iter()
        .find(|s| s.name == "blocked")
        .expect("a blocked section");

    assert!(!blocked.rows.is_empty());
    for row in &blocked.rows {
        assert!(!row[1].trim().is_empty(), "the condition is named: {row:?}");
        assert!(!row[2].trim().is_empty(), "the reason is not empty: {row:?}");
    }
    // And the reason is SPECIFIC — the thing that blocks, not the category.
    assert!(blocked.rows.iter().any(|r| r[2].contains("TS.a")));
    assert!(blocked.rows.iter().any(|r| r[2].contains("ITER.1")));
}

#[test]
fn a_condition_the_engine_cannot_compute_is_never_a_pass() {
    let (_, assessment) = build(r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
}
"##);

    // Declared as uncomputed by the record.
    let disjoint = verdict(&assessment, "TS.a", "work-is-disjoint");
    assert!(matches!(disjoint, Verdict::Uncomputed(_)));
    assert!(!disjoint.blocks(), "uncomputed does not block");
    assert_ne!(disjoint, &Verdict::Admits, "and it is not an admission either");
    assert!(disjoint.detail().contains("seam manifest"));

    // Left to a human by the record.
    assert_eq!(verdict(&assessment, "TS.a", "the-best-available"), &Verdict::Judgement);
}

#[test]
fn a_check_the_record_names_and_the_engine_lacks_is_reported_not_assumed() {
    let schema_doc = parse(
        r##"
notional-architecture "NA.test" {
    schema { entity "thin-slice" { field "slug" each="1" } }
    admits "an iteration" {
        condition "the-moon-is-waxing" check="lunar-phase" because="declared, and unimplemented"
    }
}
"##,
    )
    .expect("parses");
    let record = parse("thin-slice \"TS.a\" {\n    slug \"a\"\n}\n").expect("parses");
    let schema = Schema::from_document(&schema_doc);
    let conditions = Conditions::from_document(&schema_doc);
    let docs = vec![schema_doc.clone(), record];
    let corpus = Corpus::from_documents(&docs, &schema);
    let assessment = assess(&corpus, &conditions, "now");

    let v = verdict(&assessment, "TS.a", "the-moon-is-waxing");
    assert!(matches!(v, Verdict::Uncomputed(_)), "an unimplemented check does not pass");
    assert!(v.detail().contains("lunar-phase"), "and the answer names it: {v:?}");

    // It reaches the reader. A gate half of which silently passes is the gate that was
    // never invoked, which is the whole of S3.
    let model = project(&assessment, &corpus);
    let undecided = model
        .sections
        .iter()
        .find(|s| s.name == "not decided for you")
        .expect("a section for what was not decided");
    assert!(undecided.rows.iter().any(|r| r[0] == "the-moon-is-waxing"));
}

#[test]
fn delivered_is_not_blocked() {
    let (corpus, assessment) = build(
        r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    claim "C1" state="met"
}
"##,
    );
    assert!(corpus.delivered("TS.a"));
    assert!(assessment.finished("TS.a"), "done, not stuck");

    let model = project(&assessment, &corpus);
    let done = model.sections.iter().find(|s| s.name == "delivered").expect("a section");
    assert_eq!(done.rows.len(), 1);
    assert!(done.rows[0][1].contains("ITER.1"));

    let blocked = model.sections.iter().find(|s| s.name == "blocked").expect("a section");
    assert!(
        !blocked.rows.iter().any(|r| r[0] == "TS.a"),
        "a delivered slice must not appear as blocked"
    );
}

#[test]
fn a_close_with_an_unsettled_claim_did_not_deliver_anything() {
    let (corpus, assessment) = build(
        r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    claim "C1" state="met"
    claim "C2" state="carried"
}
"##,
    );
    assert!(!corpus.delivered("TS.a"), "carried is not met");
    assert!(!assessment.finished("TS.a"));
    assert!(assessment.ready().contains(&"TS.a"), "so it is admissible again");
    let model = project(&assessment, &corpus);
    assert!(
        model.sections.iter().any(|s| s.name == "ready" && s.rows.iter().any(|r| r[0] == "TS.a")),
        "a slice a second iteration could take"
    );
}

#[test]
fn the_record_saying_delivered_does_not_make_it_so() {
    let (_, assessment) = build(
        r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
    state "delivered"
}
"##,
    );
    assert_eq!(assessment.drift.len(), 1, "the claim is derived, never read from the claimant");
    assert_eq!(assessment.drift[0].0, "TS.a");
    assert!(assessment.drift[0].1.contains("no closed iteration"));
}

#[test]
fn silence_is_not_drift_but_a_contradiction_is() {
    let (_, assessment) = build(
        r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    claim "C1" state="met"
}
thin-slice "TS.b" {
    slug "b"
    kind "command"
    realizes "CAP.x"
    state "active"
}
iteration "ITER.2" {
    on-slice "TS.b"
    state "closed"
    claim "C1" state="met"
}
"##,
    );
    // TS.a says nothing and is delivered: deferring to the derivation is correct, and
    // calling it drift would pressure the record into keeping a second copy of a fact it
    // can compute. TS.b says something else, and that is a contradiction.
    assert_eq!(assessment.drift.len(), 1, "silence defers; a statement can conflict");
    assert_eq!(assessment.drift[0].0, "TS.b");
    assert!(assessment.drift[0].1.contains("`state active`"));
}

#[test]
fn every_section_that_is_empty_says_why() {
    // read-model@v1, third constraint. A reader who cannot tell `nothing is blocked`
    // from `blocking was not computed` is reading the trust-transfer problem at table
    // granularity.
    let (corpus, assessment) =
        build(r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
}
"##);
    let model = project(&assessment, &corpus);
    assert!(model.sections.iter().any(|s| s.rows.is_empty()), "this record leaves some empty");
    assert!(model.flaws().is_empty(), "and none of them is silent: {:?}", model.flaws());
}

#[test]
fn a_cell_never_carries_markup() {
    // First constraint. The moment a capability may emit presentation, the read-side
    // split is fiction while every other test still passes.
    let (corpus, assessment) = build(
        r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
}
thin-slice "TS.b" {
    slug "b"
    kind "command"
    realizes "CAP.x"
    depends-on "TS.a — first"
}
"##,
    );
    let model = project(&assessment, &corpus);
    assert!(model.flaws().is_empty(), "{:?}", model.flaws());
    for section in &model.sections {
        for row in &section.rows {
            for cell in row {
                assert!(!cell.contains('|') && !cell.contains('<') && !cell.contains("**"));
            }
        }
    }
}

#[test]
fn the_reason_a_condition_exists_is_referenced_not_inlined() {
    // Second constraint. Prose a human wrote is data; prose a renderer composed is
    // presentation. A row names the condition; the sentence is looked up by that name.
    let (corpus, assessment) =
        build(r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
}
"##);
    let model = project(&assessment, &corpus);
    assert_eq!(
        model.definition("dependencies-delivered"),
        Some("standing on nothing"),
        "the record's own sentence, carried by reference"
    );
    for section in &model.sections {
        for row in &section.rows {
            for cell in row {
                assert_ne!(cell, "standing on nothing", "and never copied into a row");
            }
        }
    }
}

#[test]
fn a_record_declaring_no_conditions_admits_nothing_rather_than_everything() {
    // The same ruling as an empty schema (ITER.260821.01/Q2): an empty declaration
    // describes nothing, so it cannot be read as universal permission. The shell refuses
    // to answer at all; here we assert the emptiness is legible enough for it to.
    let doc = parse(r##"notional-architecture "NA.test" { schema { } }"##).expect("parses");
    assert!(Conditions::from_document(&doc).is_empty());
}

#[test]
fn a_dependency_naming_a_slice_in_prose_is_still_read() {
    // P2: every dependency in the tree names a slice inside a sentence, which the schema
    // forbids in favour of a frozen seam. Until seams are the unit, the id is read out.
    assert_eq!(
        praxis_core::admission::named_slices("TS.260820.03 and TS.260820.04 — both must exist"),
        vec!["TS.260820.03", "TS.260820.04"]
    );
    assert_eq!(
        praxis_core::admission::named_slices("the capability record format, frozen"),
        Vec::<String>::new(),
        "a seam-shaped dependency names no slice, and blocks on none"
    );
}
