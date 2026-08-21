//! Evidence for `TS.260820.01`, claims C1–C3.
//!
//! The schema is loaded from the repository's own `NA.260820.01`, not from a fixture.
//! A test that invents its own schema proves the validator agrees with the test author;
//! this one proves it agrees with the record.

use std::fs;

use praxis_core::{Known, Refusal, Schema, Severity, check_document, index_all, parse};

const ARCHITECTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../praxis/frames/FRAME.260819.01/discovery/",
    "NA.260820.01.capabilities-seams-and-who-may-change-what.kdl"
);

fn schema() -> Schema {
    let text = fs::read_to_string(ARCHITECTURE).expect("the architecture is readable");
    let doc = parse(&text).expect("the architecture parses");
    let schema = Schema::from_document(&doc);
    assert!(!schema.is_empty(), "the record must declare a schema");
    schema
}

fn known() -> Known {
    let mut known = Known::default();
    known.insert("capability", "delivery-record");
    known.insert("event-storm", "ES.260819.01");
    known
}

/// A slice with every field the schema requires of a command slice.
fn whole() -> String {
    r#"
thin-slice "TS.999999.01" {
    slug "a-whole-slice"
    title "Every field the schema asks for"
    kind "command"
    realizes "CAP.delivery-record"
    from-storm "ES.260819.01"
    command "do-the-thing"
    actor "maintainer"
    produces-event "ThingDone"
    trigger "someone wants the thing"
    outcome "the thing is done"
    layer "doctrine" reaches="the skill teaches it"
    claim "C1" text="it happens" settled-by="a test"
}
"#
    .to_owned()
}

fn refusals(source: &str) -> Vec<Refusal> {
    let doc = parse(source).expect("the fixture parses — this is a SHAPE check, not a syntax one");
    check_document(&doc, &schema(), &known())
        .into_iter()
        .map(|v| v.refusal)
        .collect()
}

#[test]
fn a_whole_slice_is_admitted() {
    assert_eq!(refusals(&whole()), vec![], "a conforming slice must not be refused");
}

/// C1 — a slice missing any required field is refused. One case per field, driven by
/// the schema itself, so a field added to the record is covered without editing this.
#[test]
fn c1_every_required_field_is_refused_when_absent() {
    let schema = schema();
    let spec = schema.entity("thin-slice").expect("thin-slice is declared");
    let required: Vec<String> = spec
        .fields
        .iter()
        .filter(|f| f.applies_to(Some("command")) && !f.optional())
        .map(|f| f.name.clone())
        .collect();
    assert!(
        required.len() >= 4,
        "the schema should require at least command/actor/event/layer, found {required:?}"
    );

    for field in required {
        if field == "kind" {
            continue; // removing `kind` changes which fields apply; covered by its own case
        }
        let mutilated: String = whole()
            .lines()
            .filter(|line| line.trim_start().split([' ', '=']).next() != Some(field.as_str()))
            .collect::<Vec<_>>()
            .join("\n");
        let found = refusals(&mutilated);
        assert!(
            found.iter().any(|r| r.field() == field),
            "removing `{field}` was not refused; got {found:?}"
        );
    }
}

/// C2 — the refusal names the field, not merely that the document is invalid.
#[test]
fn c2_the_refusal_names_the_missing_field() {
    let without_actor: String = whole()
        .lines()
        .filter(|l| !l.trim_start().starts_with("actor "))
        .collect::<Vec<_>>()
        .join("\n");
    let found = refusals(&without_actor);
    let refusal = found
        .iter()
        .find(|r| matches!(r, Refusal::MissingField { .. }))
        .expect("a missing field is refused");
    assert_eq!(refusal.field(), "actor");
    assert!(
        refusal.message().contains("actor"),
        "the diagnostic must name the field: {}",
        refusal.message()
    );
    assert!(
        !refusal.message().to_lowercase().contains("invalid document"),
        "the diagnostic must not merely say the document is invalid"
    );
}

/// C3 — a slice naming a capability the record does not hold is refused.
#[test]
fn c3_a_slice_naming_an_absent_capability_is_refused() {
    let source = whole().replace("CAP.delivery-record", "CAP.does-not-exist");
    let found = refusals(&source);
    assert!(
        found.iter().any(|r| matches!(
            r,
            Refusal::DanglingReference { field, kind, .. } if field == "realizes" && kind == "capability"
        )),
        "a dangling capability reference must be refused; got {found:?}"
    );
}

/// The vocabulary is closed, and it is closed by the record rather than by this file.
#[test]
fn a_kind_outside_the_declared_vocabulary_is_refused() {
    let source = whole().replace(r#"kind "command""#, r#"kind "horizontal""#);
    let found = refusals(&source);
    assert!(
        found
            .iter()
            .any(|r| matches!(r, Refusal::NotInVocabulary { field, .. } if field == "kind")),
        "an undeclared kind must be refused; got {found:?}"
    );
}

/// The engine encodes nothing. Emptying the schema must disarm the checker completely —
/// if anything is still refused, a rule is hardcoded, which ADR.260819.01/A4 forbids.
///
/// This is why `undeclared-entity-kind` cannot fire on an empty schema: with nothing
/// declared, every kind is undeclared, and the engine's own emptiness would masquerade
/// as a verdict about the record (ITER.260821.01/Q2).
#[test]
fn a4_the_engine_holds_no_rule_of_its_own() {
    let doc = parse(&whole().replace("actor \"maintainer\"", "")).expect("parses");
    let violations = check_document(&doc, &Schema::default(), &known());
    assert_eq!(
        violations.len(),
        0,
        "with no schema the engine must refuse nothing; it refused {violations:?}"
    );
}

/// The eighteen committed slices are the corpus. If the engine refuses one, either the
/// slice is wrong or the schema is — and both are findings, not test failures to mute.
#[test]
fn the_committed_record_conforms() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../praxis");
    let mut sources = Vec::new();
    collect(std::path::Path::new(root), &mut sources);
    assert!(sources.len() > 20, "expected the whole record, found {}", sources.len());

    let schema = schema();
    let mut known = Known::default();
    let parsed: Vec<_> = sources
        .iter()
        .map(|(p, t)| (p.clone(), parse(t).unwrap_or_else(|e| panic!("{}: {e}", p.display()))))
        .collect();
    for (_, doc) in &parsed {
        index_all(doc, &mut known);
    }

    let mut refused = Vec::new();
    for (path, doc) in &parsed {
        for v in check_document(doc, &schema, &known) {
            if v.severity() == Severity::Refuse {
                refused.push(format!("{}: {}", path.display(), v.refusal.message()));
            }
        }
    }
    assert!(refused.is_empty(), "the committed record was refused:\n{}", refused.join("\n"));
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
