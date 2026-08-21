//! Evidence for `TS.260821.01`, claims C1–C4.
//!
//! The rule under test — `undeclared-entity-kind` — was declared on the architecture from
//! the day the schema moved there, and enforced by nothing. These tests are what change
//! that, so each one is written against the record's own schema rather than a fixture.

use std::fs;

use kdl::KdlDocument;
use praxis_core::{Known, Refusal, Schema, Severity, check_document, parse, refused};

const ARCHITECTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../praxis/frames/FRAME.260819.01/discovery/",
    "NA.260820.01.capabilities-seams-and-who-may-change-what.kdl"
);

fn architecture() -> KdlDocument {
    let text = fs::read_to_string(ARCHITECTURE).expect("the architecture is readable");
    parse(&text).expect("the architecture parses")
}

/// The schema as it would have stood before `kind` was ever declared.
fn schema_without(kind: &str) -> Schema {
    let mut doc = architecture();
    let entities = doc
        .nodes_mut()
        .iter_mut()
        .find(|n| n.name().value() == "notional-architecture")
        .and_then(|n| n.children_mut().as_mut())
        .and_then(|c| c.nodes_mut().iter_mut().find(|n| n.name().value() == "schema"))
        .and_then(|s| s.children_mut().as_mut())
        .expect("the architecture carries a schema block");
    entities
        .nodes_mut()
        .retain(|n| !(n.name().value() == "entity" && n.entries().first().and_then(|e| e.value().as_string()) == Some(kind)));
    let schema = Schema::from_document(&doc);
    assert!(
        schema.entity(kind).is_none(),
        "`{kind}` should have been removed from the schema"
    );
    schema
}

fn schema() -> Schema {
    Schema::from_document(&architecture())
}

fn check(source: &str, schema: &Schema) -> Vec<Refusal> {
    let doc = parse(source).expect("the fixture parses — this is a vocabulary check, not a syntax one");
    check_document(&doc, schema, &Known::default())
        .into_iter()
        .map(|v| v.refusal)
        .collect()
}

/// C1 — a node whose kind the schema does not declare is refused, naming the kind.
#[test]
fn c1_an_undeclared_root_kind_is_refused_by_name() {
    let found = check(r#"invented-thing "X.1" { slug "whatever" }"#, &schema());
    let refusal = found
        .iter()
        .find(|r| matches!(r, Refusal::UndeclaredKind { .. }))
        .expect("an undeclared kind must be refused");
    assert_eq!(refusal.field(), "invented-thing");
    assert!(
        refusal.message().contains("invented-thing"),
        "the diagnostic must name the kind: {}",
        refusal.message()
    );
    assert_eq!(refusal.severity(), Severity::Refuse, "it must fail closed");
}

/// C2 — declaring the kind admits it, and the engine is not touched to do so.
#[test]
fn c2_declaring_the_kind_admits_it_with_no_engine_change() {
    let record = r#"widget "W.1" { name "a widget" }"#;
    assert!(
        refused(&{
            let doc = parse(record).unwrap();
            check_document(&doc, &schema(), &Known::default())
        }),
        "undeclared, it is refused"
    );

    // The only thing that changes is the record. No engine code is involved.
    let declared: KdlDocument = r#"
notional-architecture "NA.TEST" {
    schema {
        entity "widget" is="a thing under test" {
            field "name" each="1"
        }
    }
}
"#
    .parse()
    .expect("the fixture schema parses");
    let doc = parse(record).unwrap();
    let violations = check_document(&doc, &Schema::from_document(&declared), &Known::default());
    assert!(
        !refused(&violations),
        "declared, the same record must be admitted; got {violations:?}"
    );
}

/// C3 — a kind declared with no fields is admitted and reported, never silently accepted.
#[test]
fn c3_a_shapeless_kind_is_reported_not_refused_and_not_silent() {
    let declared: KdlDocument = r#"
notional-architecture "NA.TEST" {
    schema {
        entity "widget" is="declared, but with no shape at all"
    }
}
"#
    .parse()
    .expect("parses");
    let schema = Schema::from_document(&declared);
    let doc = parse(r#"widget "W.1" { anything "goes" }"#).unwrap();
    let violations = check_document(&doc, &schema, &Known::default());

    assert!(!refused(&violations), "a shapeless kind must not fail closed");
    let report = violations
        .iter()
        .find(|v| matches!(v.refusal, Refusal::ShapelessKind { .. }))
        .expect("it must still be REPORTED — silence and acceptance must not look the same");
    assert_eq!(report.severity(), Severity::Report);
    assert!(report.refusal.message().contains("widget"));
}

/// C4 — the three kinds that actually went undeclared would each have been caught.
///
/// `iteration` is a root kind; `phase` and `read-model` are contained, and are only
/// distinguishable from a field because their containers now declare shapes. That is the
/// whole reason this iteration declared `iteration` and `event-storm` in full.
#[test]
fn c4_the_three_kinds_that_went_missing_would_have_been_caught() {
    const ITERATION: &str = r#"
iteration "ITER.1" {
    slug "x"
    on-slice "TS.1"
    state "open"
    opened-at "t"
    opened-by "a"
    approval "admission" { status "signed" }
    vet "create" { condition "c" }
    trail { entry at="t" by="a" action="created" }
}
"#;
    const WITH_PHASE: &str = r#"
iteration "ITER.1" {
    slug "x"
    on-slice "TS.1"
    state "open"
    opened-at "t"
    opened-by "a"
    approval "admission" { status "signed" }
    vet "create" { condition "c" }
    trail { entry at="t" by="a" action="created" }
    phase "implement" { kind "implement"; state "active" }
}
"#;
    const WITH_READ_MODEL: &str = r#"
event-storm "ES.1" {
    title "t"
    frame "FRAME.1"
    command "c"
    event "E"
    read-model "a-view"
}
"#;
    let cases: [(&str, &str); 3] = [
        ("iteration", ITERATION),
        ("phase", WITH_PHASE),
        ("read-model", WITH_READ_MODEL),
    ];

    for (kind, record) in cases {
        let found = check(record, &schema_without(kind));
        assert!(
            found.iter().any(|r| matches!(
                r,
                Refusal::UndeclaredKind { kind: k, .. } if k == kind
            )),
            "removing `{kind}` from the schema should have made its use refusable; got {found:?}"
        );
    }
}

/// The committed record passes, and its shapeless kinds are reported rather than ignored.
#[test]
fn the_record_conforms_and_names_what_still_has_no_shape() {
    let root = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../praxis"));
    let mut files = Vec::new();
    collect(root, &mut files);

    let schema = schema();
    let mut known = Known::default();
    let parsed: Vec<_> = files
        .iter()
        .map(|(p, t)| (p.clone(), parse(t).unwrap_or_else(|e| panic!("{}: {e}", p.display()))))
        .collect();
    for (_, doc) in &parsed {
        praxis_core::index_all(doc, &mut known);
    }

    let mut hard = Vec::new();
    let mut reported = 0;
    for (path, doc) in &parsed {
        for v in check_document(doc, &schema, &known) {
            match v.severity() {
                Severity::Refuse => hard.push(format!("{}: {}", path.display(), v.refusal.message())),
                Severity::Report => reported += 1,
            }
        }
    }
    assert!(hard.is_empty(), "the committed record was refused:\n{}", hard.join("\n"));
    assert!(
        reported > 0,
        "kinds still carry no shape; the check must say so rather than pass in silence"
    );
}

fn collect(dir: &std::path::Path, out: &mut Vec<(std::path::PathBuf, String)>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out);
        } else if path.extension().is_some_and(|e| e == "kdl") {
            let text = fs::read_to_string(&path).expect("readable");
            out.push((path, text));
        }
    }
}
