//! `TS.260821.05` — hold what the plugin guarantees, and what keeps each guarantee.
//!
//! The claim under all three: `enable-all-fail-closed #true` was a sentence about ten shell
//! scripts that nothing had ever compared to anything.

use praxis_core::check::{Facts, check_corpus_given};
use praxis_core::invariant::{Enforcement, check_invariants};
use praxis_core::{Corpus, Schema, Severity, parse};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug" each="1"
        }
        entity "invariant" {
            field "protects" each="1"
            field "severity" each="1"
            field "language" each="0..n"
        }
        entity "doctrine-surface" states="current retired" {
            field "path"   each="1"
            field "kind"   each="1"
            field "serves" each="1..n" references="thin-slice invariant"
            field "state"  each="0..1"
        }
        entity "config" {
            field "profile" each="1" holds="profile"
        }
        entity "profile" {
            field "omit-probe" each="0..n" references="invariant"
        }
        rule "an-enabled-invariant-is-enforced" reports="a guarantee nothing keeps"
    }
}
"##;

fn corpus(record: &str) -> Corpus {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    Corpus::from_documents(&[schema_doc, record_doc], &schema)
}

const TWO: &str = r#"
invariant "kept-one" {
    protects "something a probe checks"
    severity "refuse"
}
invariant "unkept-one" {
    protects "something nothing checks"
    severity "refuse"
}
doctrine-surface "surface.probe.one" {
    path "scripts/check-one.sh"
    kind "probe"
    serves "kept-one"
}
config {
    profile "test" {
    }
}
"#;

/// C1 — an enabled invariant that no surface enforces is reported, naming the invariant.
#[test]
fn a_guarantee_nothing_keeps_is_reported_by_name() {
    let found = check_invariants(&corpus(TWO));

    assert_eq!(found.len(), 2, "the check is total over the DECLARED set, not over the kept one");
    assert!(matches!(&found[0], Enforcement::Kept { by, .. } if by == &["scripts/check-one.sh"]));
    assert_eq!(found[1], Enforcement::Unkept { invariant: "unkept-one".to_owned() });

    // And through the checker, at report severity: an invariant declared before its probe is
    // written is a legitimate order of work.
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(TWO).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let violations =
        check_corpus_given(&[schema_doc, record_doc], &schema, &Facts::default());
    let reported: Vec<_> =
        violations.iter().filter(|v| v.refusal.rule() == "an-enabled-invariant-is-enforced").collect();
    assert_eq!(reported.len(), 1);
    assert!(reported[0].refusal.message().contains("unkept-one"));
    assert_eq!(reported[0].severity(), Severity::Report);
}

/// Omitting is a BINDING, not a gap. A profile that says it has no HTTP surface has answered
/// the question, and reporting it anyway teaches adopters to ignore the report.
#[test]
fn an_omitted_invariant_is_not_owed() {
    let record = r#"
invariant "not-needed-here" {
    protects "something this repository has no surface for"
    severity "refuse"
}
config {
    profile "test" {
        omit-probe "not-needed-here"
    }
}
"#;
    let found = check_invariants(&corpus(record));

    assert_eq!(found.len(), 1);
    assert!(!found[0].unkept(), "omitted is not unkept, and the two must not read alike");
    assert!(matches!(found[0], Enforcement::Omitted { .. }));
}

/// C2 — a probe anchors by naming the invariant it enforces. Before this, `serves` could
/// only name delivery entities, so a probe had nothing it could legitimately point at.
#[test]
fn a_probe_anchors_by_naming_what_it_enforces() {
    let found = corpus(TWO);
    let probe = found.surfaces.iter().find(|s| s.kind == "probe").expect("the probe is read");

    assert_eq!(probe.serves, ["kept-one"]);
    // A retired probe stops keeping anything. Leaving it as an enforcer would mean the
    // guarantee survives the thing that kept it.
    let retired = corpus(
        r#"
invariant "kept-one" {
    protects "something"
    severity "refuse"
}
doctrine-surface "surface.probe.one" {
    path "scripts/check-one.sh"
    kind "probe"
    serves "kept-one"
    state "retired"
}
config {
    profile "test" {
    }
}
"#,
    );
    assert!(check_invariants(&retired)[0].unkept());
}

/// C3 — config may bind an invariant and may never declare one.
#[test]
fn config_omitting_an_invariant_the_record_does_not_hold_is_refused() {
    let record = r#"
invariant "real-one" {
    protects "something"
    severity "refuse"
}
config {
    profile "test" {
        omit-probe "invented-here"
    }
}
"#;
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let mut known = praxis_core::Known::default();
    praxis_core::index_all(&schema_doc, &mut known);
    praxis_core::index_all(&record_doc, &mut known);

    let found: Vec<_> = record_doc
        .nodes()
        .iter()
        .flat_map(|n| praxis_core::check_node(n, &schema, &known))
        .filter(|v| v.refusal.rule() == "dangling-relationship")
        .collect();

    assert_eq!(
        found.len(),
        1,
        "config said `may enable or omit, never invent` in a comment for the whole life of the \
         binding, and nothing could tell omitting from inventing"
    );
}
