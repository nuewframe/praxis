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
    let facts = Facts {
        shipped: shipped.iter().map(|s| (*s).to_owned()).collect(),
        // No debt in these fixtures: `owed` is the case where the record NAMES who owes a
        // file's disposal, and every surface here is either anchored or genuinely orphaned.
        owed: Vec::new(),
        text: Vec::new(),
    };
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

/// C1 — a shipped file that no declared surface names is named, by path.
///
/// It landed at REPORT severity, naming forty-five files. `ITER.260822.04` flipped it to
/// refuse once the count reached zero (48 of 48 anchored), which is `TS.260821.04`/C3. The
/// assertion below is on the SEVERITY as it stands, so a silent flip back would fail here.
#[test]
fn a_shipped_file_nobody_declared_is_named_by_path() {
    let found = violations(ANCHORED, &["skills/anchored/SKILL.md", "skills/nobody-asked/SKILL.md"]);

    assert_eq!(rules(&found), ["every-shipped-surface-is-anchored"]);
    assert!(
        found[0].refusal.message().contains("skills/nobody-asked/SKILL.md"),
        "the report must name the PATH — a count of unanchored files is not actionable: {}",
        found[0].refusal.message()
    );
    // Refuses, since ITER.260822.04. The count is zero and the rule is what keeps it there:
    // adding an unanchored surface now fails the check rather than being noted in a list
    // nobody reads to the bottom of.
    assert_eq!(found[0].severity(), Severity::Refuse);
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

/// `TS.260821.08`/C3 — the root is not a `serves` target.
///
/// `ITER.260822.04` anchored three personas to `FRAME.260819.01`, and the decision recorded
/// there predicted the falsifier would be a fourth persona. It was not. The frame is the
/// ROOT: every surface descends from it by construction, so `serves "FRAME..."` is true of
/// everything and discriminates nothing — and a field that cannot discriminate is a dumping
/// ground whether it holds one thing or forty.
///
/// The fix is not a rule that measures the pile. `no-dumping-grounds` forbids the NAMES
/// `utils`, `helpers`, `common`; it never checks whether a folder became a junk drawer.
/// Anti-dumping works by making the dumping ground unnameable, and this is the same move
/// applied to an edge.
#[test]
fn a_surface_cannot_anchor_to_the_root() {
    const WITH_FRAME: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "frame" {
            field "title" each="1"
        }
        entity "thin-slice" {
            field "slug" each="1"
        }
        entity "doctrine-surface" {
            field "path"   each="1"
            field "kind"   each="1"
            field "serves" each="1..n" references="thin-slice"
        }
    }
}
"##;
    let record = r#"
frame "FRAME.test" {
    title "a problem"
}
thin-slice "TS.900" {
    slug "a-slice"
}
doctrine-surface "surface.dumped" {
    path "agents/someone.agent.md"
    kind "agent"
    serves "FRAME.test"
}
"#;
    let schema_doc = parse(WITH_FRAME).expect("schema parses");
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
        "the frame is in the record and the surface still cannot anchor to it — because the \
         vocabulary does not admit it, which is a stronger guarantee than a rule that would \
         have to decide how much is too much"
    );
}

/// `TS.260823.05`/C2 and C3. A shipped file whose disposal is owed is counted and reported,
/// never refused — and never silently exempt.
///
/// The audit reported "55 of 55 anchored" over a set the engine chose: four shapes globbed
/// in `shipped_doctrine`, silent about fifty-three files under the overlay's `templates/`,
/// which is the tree copied INTO an adopter's repository. A denominator chosen by the thing
/// being measured is the trust-transfer problem with a percentage sign after it.
#[test]
fn a_file_whose_disposal_is_owed_is_reported_and_not_refused() {
    let record = r#"
doctrine-surface "surface.kept" {
    path "skills/kept/SKILL.md"
    kind "skill"
    serves "something"
}
"#;
    let schema_doc = parse(SCHEMA).expect("the schema parses");
    let doc = parse(record).expect("the record parses");
    let schema = Schema::from_document(&schema_doc);
    let facts = Facts {
        shipped: vec![
            "skills/kept/SKILL.md".to_owned(),
            "skills/going/templates/thing.tmpl".to_owned(),
            "skills/orphan/SKILL.md".to_owned(),
        ],
        owed: vec![("skills/going/templates/thing.tmpl".to_owned(), "TS.999999.01".to_owned())],
        text: Vec::new(),
    };
    let found = check_corpus_given(&[schema_doc.clone(), doc], &schema, &facts);

    let owed: Vec<_> = found
        .iter()
        .filter(|v| v.refusal.message().contains("thing.tmpl"))
        .collect();
    assert_eq!(owed.len(), 1, "counted once, not once per surfaces file");
    assert_eq!(
        owed[0].severity(),
        Severity::Report,
        "a debt the record NAMES an owner for is reported; refusing it would force anchoring \
         doctrine that slice is going to delete"
    );
    assert!(
        owed[0].refusal.message().contains("TS.999999.01"),
        "and it says who owes it — an exemption says a rule does not apply, this says somebody \
         owes the answer: {}",
        owed[0].refusal.message()
    );

    // A file nobody owes is still refused. The debt is not a way out of the rule.
    let orphan: Vec<_> = found
        .iter()
        .filter(|v| v.refusal.message().contains("orphan"))
        .collect();
    assert_eq!(orphan.len(), 1);
    assert_eq!(
        orphan[0].severity(),
        Severity::Refuse,
        "an unanchored file with no owner named is refused as it always was"
    );
}

/// `TS.260823.05`/C1. Which shapes count as instruction comes from the RECORD.
///
/// It used to be a glob in the shell naming four shapes. Widening it in place would have
/// fixed the number and left the next shape to be discovered the same way — A4 says the
/// engine may not hold what the record declares, and *what this repository ships as
/// instruction* is a fact about the repository.
#[test]
fn c1_the_shapes_are_declared_and_a_repository_may_add_one() {
    let config = parse(
        r#"
config {
    repository "acme/checkout"
    ships-doctrine {
        from "playbooks" named="*.play.md"
        from "skills"    named="*.tmpl" owed-to="TS.999999.01"
    }
}
"#,
    )
    .expect("parses");

    assert_eq!(
        praxis_core::surface::declared_shapes(&[config.clone()]),
        vec![
            ("playbooks".to_owned(), "*.play.md".to_owned()),
            ("skills".to_owned(), "*.tmpl".to_owned()),
        ],
        "a repository declares its own shapes, including ones this plugin has never had"
    );

    assert_eq!(
        praxis_core::surface::owed_shapes(&[config]),
        vec![("skills".to_owned(), "*.tmpl".to_owned(), "TS.999999.01".to_owned())],
        "and the shapes whose disposal is owed name who owes it"
    );
}

/// A repository declaring nothing yields nothing, so the caller refuses rather than falling
/// back to a set the engine chose. The fallback IS the fault.
#[test]
fn c1_a_repository_declaring_no_shapes_yields_none() {
    let bare = parse(r#"config { repository "acme/checkout" }"#).expect("parses");
    assert!(praxis_core::surface::declared_shapes(&[bare]).is_empty());
}

/// One `*`, anywhere, or an exact name — enough for every shape this plugin ships and
/// nothing more, because a shape language is a second thing to learn.
#[test]
fn c1_a_shape_matches_by_prefix_suffix_or_exactly() {
    use praxis_core::surface::matches_shape;
    assert!(matches_shape("SKILL.md", "*SKILL.md"));
    assert!(matches_shape("verify.sh.tmpl", "*.tmpl"));
    assert!(matches_shape("check-anti-dumping.sh", "check-*.sh"));
    assert!(matches_shape("CLAUDE.md", "CLAUDE.md"));

    assert!(!matches_shape("SKILL.md", "*.tmpl"));
    assert!(!matches_shape("gen-coverage.sh", "check-*.sh"));
    // A generator is not doctrine, and neither is a file that merely contains the shape.
    assert!(!matches_shape("notes-about-SKILL.md.bak", "*SKILL.md"));
}
