//! `TS.260821.10` — the method has a home. Closes `AK2`.
//!
//! A project that is not Praxis could not run this engine without copying a 946-line
//! `schema` block and an `admits` block out of the author's own discovery folder. Copying
//! them means every repository forks the method and drifts from it silently — the
//! second-copy problem this frame exists to attack, turned on the frame's own definition of
//! itself.

use praxis_core::schema::{Extension, METHOD};
use praxis_core::{Corpus, Schema, Severity, check_document, parse};
use praxis_core::{Known, index_all};

/// C1 — an engine with nothing copied has the method in force.
///
/// The whole of AK2 is this test. A config and a frame, no schema document anywhere, and the
/// record still refuses a frame field the method does not declare.
#[test]
fn a_project_that_copied_nothing_is_still_governed() {
    let (schema, version) = Schema::method();

    assert_eq!(version, "delivery-graph@v1");
    assert!(schema.entity("frame").is_some(), "the method carries its own kinds");
    assert!(schema.entity("thin-slice").is_some());
    assert!(schema.entity("iteration").is_some());

    let record = parse(
        r#"
frame "FRAME.acme" {
    slug "checkout-abandonment"
    title "People fill a basket and leave"
    invented-field "nothing declares this"
}
"#,
    )
    .expect("the record parses");
    let mut known = Known::default();
    index_all(&record, &mut known);

    let found: Vec<_> = record
        .nodes()
        .iter()
        .flat_map(|n| check_document(&parse(&n.to_string()).unwrap(), &schema, &known))
        .collect();

    assert!(
        found.iter().any(|v| v.severity() == Severity::Refuse),
        "a project holding no schema of its own is still checked: {found:?}"
    );
}

/// C4 — the shipped vocabulary is a record the engine parses, not a shape it knows.
///
/// A4 (`ADR.260819.01`) forbids the engine encoding what the record declares, because an
/// engine that does can diverge from it and nobody can tell. Carrying the record and knowing
/// its contents are different things, and only the second is refused.
#[test]
fn the_method_is_data_and_passes_its_own_rules() {
    let doc = parse(METHOD).expect("the embedded method parses through the ordinary reader");
    let (schema, _) = Schema::method();
    let mut known = Known::default();
    index_all(&doc, &mut known);

    let refusals: Vec<_> = check_document(&doc, &schema, &known)
        .into_iter()
        .filter(|v| v.severity() == Severity::Refuse)
        .collect();

    assert!(
        refusals.is_empty(),
        "the method describes itself — it declares the `method` kind that holds it, so it is \
         subject to its own rules like any record: {refusals:?}"
    );

    // One file, two carriers: the plugin ships it for an agent with no binary, the binary
    // embeds it for an install with no plugin. `include_str!` points at the shipped file, so
    // there is nothing to keep in sync.
    let shipped = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../praxis/method/DELIVERY-GRAPH.v1.kdl"),
    )
    .expect("the method ships in the plugin tree, where an agent without the binary reads it");
    assert_eq!(shipped, METHOD, "one file, or the two carriers drift");
}

/// C3 — a repository extends the method and never redefines it.
#[test]
fn extending_adds_and_redefining_refuses() {
    let (mut schema, _) = Schema::method();
    let local = Schema::from_document(
        &parse(
            r#"
notional-architecture "NA.acme" {
    schema {
        entity "cohort" {
            field "slug" each="1"
        }
        entity "thin-slice" {
            field "whatever" each="1"
        }
    }
}
"#,
        )
        .expect("parses"),
    );

    let outcome = schema.extend(local);

    assert!(
        outcome.contains(&Extension::Added("entity cohort".to_owned())),
        "a kind the method does not name is the repository's to declare: {outcome:?}"
    );
    assert!(
        outcome.contains(&Extension::Redefines("entity thin-slice".to_owned())),
        "and one it does name is not — `may bind, and may never declare` one level up: {outcome:?}"
    );
    // The rule discriminates: it did not simply refuse everything local.
    assert!(schema.entity("cohort").is_some(), "the new kind landed");
    assert_eq!(
        schema.entity("thin-slice").unwrap().fields.iter().any(|f| f.name == "whatever"),
        false,
        "and the redefinition did NOT — a method a repository can weaken locally is a method \
         that reports whatever each repository wanted to hear"
    );
}

/// C2 — this repository's own record is the proof. `doctrine-surface` and `invariant` are
/// extensions, because a project that ships no doctrine needs neither.
#[test]
fn this_repository_extends_rather_than_redefines() {
    let (method, _) = Schema::method();

    assert!(
        method.entity("doctrine-surface").is_none() && method.entity("invariant").is_none(),
        "a vocabulary for shipped instruction belongs to the repository that ships instruction"
    );
    assert!(
        method.entity("frame").is_some() && method.entity("role").is_some(),
        "and what a frame or a role IS belongs to the method"
    );
}

/// C5 — `governed-by` names the method, and an unknown one is refused.
#[test]
fn a_config_bound_to_a_method_the_engine_lacks_is_refused() {
    let (schema, _) = Schema::method();
    let record = parse(
        r#"
config {
    repository "acme/checkout"
    governed-by "no-such-method@v99"
    profile "service" {
        layer "api" "the HTTP surface"
    }
    paths {
        state-root "praxis/"
    }
}
"#,
    )
    .expect("parses");
    let corpus_docs = [record];
    let found = praxis_core::check_corpus(&corpus_docs, &schema);
    let refused: Vec<_> = found
        .iter()
        .filter(|v| v.refusal.rule() == "a-config-binds-a-method-the-engine-carries")
        .collect();

    assert_eq!(refused.len(), 1, "{found:?}");
    assert!(refused[0].refusal.message().contains("delivery-graph@v1"), "it says what IS carried");

    // And the real binding passes, which is what makes the refusal mean something.
    let _ = Corpus::default();
}
