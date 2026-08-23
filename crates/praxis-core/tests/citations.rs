//! `TS.260821.17` — what the record may cite, and what a transcription cannot fake.
//!
//! Two halves of one instruction, and the second is what makes the first worth having.
//!
//! **The record says WHAT was shown.** The repository says where it lives and git says when
//! it moved. A path in the record is a fourth copy of the second, and it is the copy that
//! rots without anybody noticing — a renamed test leaves the claim it backed reading as
//! settled and pointing at nothing. A hundred and fifty-six of them stood here.
//!
//! **A transcribed capability is refused by the shape it does not have.** No rule was added
//! for it and none is wanted: a capability copied out of a Markdown document arrives with no
//! cluster, no gate tests and no exclusion, because a Markdown capability record declares no
//! gate test it can fail. `TS.260821.06` ran the experiment by accident — two records ported
//! and GAINED four gate tests each, and a third turned out to have been hollow for two
//! versions.

use praxis_core::{Known, Schema, check_corpus, check_node, index_all, parse};

/// Only what these tests exercise. Written out rather than loaded from the method, because a
/// test that reads the shipped method proves the method and this one is about the ENGINE.
const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "capability" {
            field "doing"            each="1"
            field "not"              each="1"
            field "state"            each="1"
            field "derived-from"     each="1"
            field "from-cluster"     each="1"
            field "owns-event"       each="1"
            field "gate-test"        each="1..n"
            field "keeps-consistent" each="1"
        }
        entity "iteration" {
            field "on-slice" each="1"
            field "state"    each="1"
            field "claim"    each="0..n"
            field "layer"    each="0..n"
        }
        entity "doctrine-surface" {
            field "path"  each="1"
            field "kind"  each="1"
        }
        rule "the-record-cites-what-it-holds" refuses="a field naming a file the checker cannot follow"
    }
}
"##;

fn violations(record: &str) -> Vec<praxis_core::Violation> {
    violations_under(SCHEMA, record)
}

fn violations_under(schema_src: &str, record: &str) -> Vec<praxis_core::Violation> {
    let schema_doc = parse(schema_src).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let mut known = Known::default();
    index_all(&schema_doc, &mut known);
    index_all(&record_doc, &mut known);
    let mut all: Vec<_> =
        record_doc.nodes().iter().flat_map(|n| check_node(n, &schema, &known)).collect();
    all.extend(check_corpus(&[schema_doc, record_doc], &schema));
    all
}

fn cited(record: &str) -> Vec<String> {
    violations(record)
        .into_iter()
        .filter(|v| v.refusal.rule() == "the-record-cites-what-it-holds")
        .map(|v| v.refusal.message())
        .collect()
}

/// C1 — a claim whose evidence is a path is refused, and the refusal names the path.
///
/// This is the shape the record carried everywhere: `go here and see`. The moment the
/// function is renamed the claim is orphaned, silently, while still reading as settled.
#[test]
fn a_claim_evidenced_by_a_path_is_refused_and_the_path_is_named() {
    let found = cited(
        r#"
iteration "ITER.1" {
    on-slice "TS.1"
    state "closed"
    claim "C1" evidence="tests/the_anchor.rs::a_mission_with_no_test_is_refused"
}
"#,
    );
    assert_eq!(found.len(), 1, "one citation, one refusal: {found:?}");
    assert!(
        found[0].contains("tests/the_anchor.rs::a_mission_with_no_test_is_refused"),
        "the refusal names what was cited, or the reader has to go looking: {found:?}"
    );
}

/// C4 — the same claim, stating what was shown, passes.
///
/// Not a weaker claim. A stronger one: `a mission with no test is refused` survives the
/// rename that would have orphaned the citation, and says what the test proved rather than
/// where somebody can go to watch it pass.
#[test]
fn evidence_that_states_what_was_shown_passes() {
    let found = cited(
        r#"
iteration "ITER.1" {
    on-slice "TS.1"
    state "closed"
    claim "C1" evidence="a mission with no test is refused, and a vision needs none"
}
"#,
    );
    assert!(found.is_empty(), "stating what was shown is the point of the rule: {found:?}");
}

/// C1 — the citation the record MAY make is its own id.
///
/// `ITER.260821.17/AG1` is followable: `dangling-relationship` refuses an id the record does
/// not hold, which is exactly the check a path cannot have. The rule has to leave those
/// alone, and `and/or` with them — a discriminator that fires on a slash refuses the
/// record's own citation form.
#[test]
fn an_id_is_a_citation_and_a_slash_is_not_a_path() {
    let found = cited(
        r#"
iteration "ITER.1" {
    on-slice "TS.1"
    state "closed"
    claim "C1" evidence="settled by ITER.260821.17/AG1 and TS.260821.02 — the rule refuses and/or reports"
    claim "C2" evidence="carried by AG3, and the 0.8.0 release index names the commit"
}
"#,
    );
    assert!(found.is_empty(), "the record's own ids are what it is allowed to cite: {found:?}");
}

/// C1 — the record's assertions about its own tree are not citations.
///
/// None of them can rot silently, which is the property that makes a citation dangerous.
/// `doctrine-surface.path` is checked by `a-declared-surface-ships`, which refuses when the
/// file is absent. If that distinction is wrong, this test is where to argue with it.
#[test]
fn what_the_record_writes_and_governs_is_not_a_citation() {
    let found = cited(
        r#"
doctrine-surface "surface.skill.cut-a-slice" {
    path "skills/cut-a-slice/SKILL.md"
    kind "skill"
}
"#,
    );
    assert!(found.is_empty(), "an anchored surface names a file BY DESIGN: {found:?}");
}

/// C1 — the rule goes silent when the record stops declaring it.
///
/// The engine enforces what the record declares and never what it knows. A repository whose
/// method predates this rule is not failing a check it never adopted.
#[test]
fn the_rule_is_silent_where_the_record_does_not_declare_it() {
    let undeclared = SCHEMA.replace(
        r#"rule "the-record-cites-what-it-holds" refuses="a field naming a file the checker cannot follow""#,
        "",
    );
    let found: Vec<_> = violations_under(
        &undeclared,
        r#"
iteration "ITER.1" {
    on-slice "TS.1"
    state "closed"
    claim "C1" evidence="tests/the_anchor.rs::a_mission_with_no_test_is_refused"
}
"#,
    )
    .into_iter()
    .filter(|v| v.refusal.rule() == "the-record-cites-what-it-holds")
    .collect();
    assert!(found.is_empty(), "a rule the record does not declare must not fire: {found:?}");
}

/// C2 — a transcribed capability is refused by the shape check, with no rule added.
///
/// This is what a Markdown capability record transcribes to: a name and a doing. Everything
/// that would make it checkable — the cluster it was derived from, the four gate tests, the
/// exclusion — is what the source document never had, so the transcription cannot carry it.
///
/// The refusal is the argument. A copied record looks identical to a derived one until
/// something asks it for the parts that only derivation produces.
#[test]
fn a_transcribed_capability_is_refused_for_what_it_cannot_have() {
    let missing: Vec<String> = violations(
        r#"
capability "CAP.transcribed" {
    doing "the sentence at the top of the Markdown document"
}
"#,
    )
    .into_iter()
    .filter(|v| v.refusal.rule() == "entity-without-a-required-field")
    .map(|v| v.refusal.field().to_owned())
    .collect();

    for wanted in ["from-cluster", "gate-test", "not"] {
        assert!(
            missing.iter().any(|f| f == wanted),
            "a transcription has no {wanted} — that is the half the source document never \
             wrote, and it is what the shape check is for: {missing:?}"
        );
    }
    assert!(
        !violations(
            r#"
capability "CAP.derived" {
    doing "hold what the method's own derivation says"
    not "judge whether a decision was good"
    state "active"
    derived-from "ES.1"
    from-cluster "the-record"
    owns-event "SliceCut"
    gate-test "a slice with no claim is refused"
    keeps-consistent "a claim and the commitment that carries it"
}
"#
        )
        .iter()
        .any(|v| v.refusal.rule() == "entity-without-a-required-field"),
        "the derived one passes on the same rule, or the test proves nothing about derivation"
    );
}
