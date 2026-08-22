//! `TS.260821.03` — report any doctrine this plugin ships that no record entity justifies.
//!
//! The four claims. C1 and C2 are the two directions of the same edge and they are NOT
//! symmetric: a shipped file nobody declared is reported, because on the day the rule lands
//! there are forty-five of them; a declared file that does not ship refuses, because that
//! can never be legitimate work-in-progress.

use praxis_core::check::{Facts, check_corpus_given};
use praxis_core::surface::{Audit, Surface, audit};
use praxis_core::{Schema, Severity, parse};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug" each="1"
        }
        entity "doctrine-surface" states="current retired" {
            field "path"   each="1"
            field "kind"   each="1"
            field "serves" each="1..n" references="thin-slice"
            field "state"  each="0..1"
        }
        rule "a-declared-surface-ships"          refuses="a surface naming a path that does not ship"
        rule "every-shipped-surface-is-anchored" reports="a shipped file no surface declares"
    }
}
"##;

fn violations(record: &str, shipped: &[&str]) -> Vec<praxis_core::Violation> {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let facts = Facts { shipped: shipped.iter().map(|s| (*s).to_owned()).collect() };
    check_corpus_given(&[schema_doc, record_doc], &schema, &facts)
}

fn rules(violations: &[praxis_core::Violation]) -> Vec<&'static str> {
    violations.iter().map(|v| v.refusal.rule()).collect()
}

const ANCHORED: &str = r#"
thin-slice "TS.900" {
    slug "a-slice"
}
doctrine-surface "surface.one" {
    path "skills/anchored/SKILL.md"
    kind "skill"
    serves "TS.900"
}
"#;

/// C1 — a shipped file that no declared surface names is reported, by path.
#[test]
fn a_shipped_file_nobody_declared_is_reported_by_path() {
    let found = violations(ANCHORED, &["skills/anchored/SKILL.md", "skills/nobody-asked/SKILL.md"]);

    assert_eq!(rules(&found), ["every-shipped-surface-is-anchored"]);
    assert!(
        found[0].refusal.message().contains("skills/nobody-asked/SKILL.md"),
        "the report must name the PATH — a count of unanchored files is not actionable: {}",
        found[0].refusal.message()
    );
    // Reported, not refused. TS.260821.04 flips it once the count is zero; a rule that fails
    // closed on its first run names forty-five files and is one nobody can adopt.
    assert_eq!(found[0].severity(), Severity::Report);
}

/// C2 — a declared surface whose path does not ship refuses. The opposite direction from
/// C1, and the one that fails closed.
#[test]
fn a_declared_surface_that_does_not_ship_refuses() {
    let found = violations(ANCHORED, &[]);

    assert_eq!(rules(&found), ["a-declared-surface-ships"]);
    assert_eq!(
        found[0].severity(),
        Severity::Refuse,
        "a surface anchored to an absent file is a promise the record cannot keep, and unlike \
         C1 it can never be legitimate work-in-progress"
    );
}

/// C2, second half — a surface declared retired whose file is still there. The retirement
/// was recorded and never carried out, which is the failure the retirement existed to fix.
#[test]
fn a_retired_surface_still_in_the_tree_refuses() {
    let record = r#"
thin-slice "TS.900" {
    slug "a-slice"
}
doctrine-surface "surface.gone" {
    path "skills/retired/SKILL.md"
    kind "skill"
    serves "TS.900"
    state "retired"
}
"#;
    let found = violations(record, &["skills/retired/SKILL.md"]);

    assert_eq!(rules(&found), ["a-declared-surface-ships"]);
    assert_eq!(found[0].severity(), Severity::Refuse);

    // And a retirement that WAS carried out is silent in both directions: the file is gone,
    // so neither rule has anything to say.
    assert!(violations(record, &[]).is_empty());
}

/// C3 — the anchor is a relationship into the record, so a surface serving something the
/// record does not hold is already dangling. No second rule.
#[test]
fn a_surface_serving_nothing_the_record_holds_is_already_dangling() {
    let record = r#"
thin-slice "TS.900" {
    slug "a-slice"
}
doctrine-surface "surface.orphan" {
    path "skills/anchored/SKILL.md"
    kind "skill"
    serves "TS.999"
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
        .collect();

    assert_eq!(rules(&found), ["dangling-relationship"]);
}

/// C4 — the audit is total over its input and touches no filesystem. Handing it the whole
/// world as facts is the only way it learns what ships.
#[test]
fn the_audit_is_pure_and_total_over_its_input() {
    let surfaces = vec![
        Surface {
            id: "surface.one".to_owned(),
            path: "skills/one/SKILL.md".to_owned(),
            kind: "skill".to_owned(),
            serves: vec!["TS.900".to_owned()],
            state: "current".to_owned(),
            stands_at: None,
        },
        Surface {
            id: "surface.absent".to_owned(),
            path: "skills/absent/SKILL.md".to_owned(),
            kind: "skill".to_owned(),
            serves: vec!["TS.900".to_owned()],
            state: "current".to_owned(),
            stands_at: None,
        },
    ];
    let shipped =
        ["skills/one/SKILL.md".to_owned(), "skills/unasked/SKILL.md".to_owned()];

    let found = audit(&surfaces, &shipped);

    assert_eq!(
        found,
        Audit {
            unanchored: vec!["skills/unasked/SKILL.md".to_owned()],
            absent: vec![("surface.absent".to_owned(), "skills/absent/SKILL.md".to_owned())],
            still_shipped: vec![],
            anchored: vec!["surface.one".to_owned()],
        },
        "every shipped path lands in exactly one bucket and every declared surface is \
         accounted for — a partial audit reports a smaller number than the truth, which is \
         worse than no number at all"
    );
    assert!(!found.clean());

    // An empty world is not a clean one: the declared surfaces are all absent.
    assert!(!audit(&surfaces, &[]).clean());
    // And nothing declared, nothing shipped, is clean.
    assert!(audit(&[], &[]).clean());
}
