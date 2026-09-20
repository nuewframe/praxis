//! `TS.260823.06` — refuse shipped doctrine that teaches a vocabulary the method retired.
//!
//! The fault this closes is not a documentation lapse. `TS.260821.04` retired the sprint-era
//! spine and `using-praxis` said so, in a tree where thirteen top-level files and fifty-one
//! templates went on telling an agent to create a sprint. An agent loads both and does not
//! pick the newer one — it synthesises, which is S1, produced by the doctrine surface that
//! exists to prevent S1.
//!
//! The four claims split along one line: C1 and C2 are the mechanism, C3 is the exemption
//! that keeps the mechanism usable, and C4 is the purge actually being finished rather than
//! claimed.

use praxis_core::check::{Facts, check_corpus_given};
use praxis_core::schema::RetiredWord;
use praxis_core::surface::teaching;
use praxis_core::{Schema, Severity, parse};

/// A schema that retires a vocabulary, in the shape the method uses.
const RETIRES: &str = r##"
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
        retired "the witness-era spine" were="the old pipeline" {
            word "sprint" instead="an iteration, opened by the gate"
            word "wave"   instead="a frame"
        }
        a-retirement-may-be-explained marked-by="retire" marked-by="no longer"
        rule "surface-teaches-a-retired-kind"    refuses="doctrine teaching a retired kind"
        rule "a-retirement-names-its-vocabulary" reports="a retirement naming no words"
        rule "every-shipped-surface-is-anchored" reports="a shipped file no surface declares"
    }
}
"##;

/// The same schema with the vocabulary taken off the retirement.
const RETIRES_NOTHING: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug" each="1"
        }
        retired "the witness-era spine" were="the old pipeline"
        rule "a-retirement-names-its-vocabulary" reports="a retirement naming no words"
    }
}
"##;

const ANCHORED: &str = r#"
thin-slice "TS.900" {
    slug "a-slice"
}
doctrine-surface "surface.one" {
    path "skills/witness/SKILL.md"
    kind "skill"
    serves "TS.900"
}
"#;

fn over(schema_src: &str, shipped: &[(&str, &str)]) -> Vec<praxis_core::Violation> {
    let schema_doc = parse(schema_src).expect("schema parses");
    let record_doc = parse(ANCHORED).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let facts = Facts {
        shipped: shipped.iter().map(|(p, _)| (*p).to_owned()).collect(),
        owed: Vec::new(),
        text: shipped.iter().map(|(p, t)| ((*p).to_owned(), (*t).to_owned())).collect(),
    };
    check_corpus_given(&[schema_doc, record_doc], &schema, &facts)
}

fn firing<'a>(
    found: &'a [praxis_core::Violation],
    rule: &str,
) -> Vec<&'a praxis_core::Violation> {
    found.iter().filter(|v| v.refusal.rule() == rule).collect()
}

/// C1 — a retirement naming no vocabulary is reported, since it can enforce nothing.
///
/// Reported and not refused, deliberately. It is a gap in what the record says about its own
/// history, and a rule that failed the record closed over its history would make the history
/// the thing to delete.
#[test]
fn a_retirement_with_no_words_is_reported() {
    let found = over(RETIRES_NOTHING, &[]);
    let reported = firing(&found, "a-retirement-names-its-vocabulary");

    assert_eq!(reported.len(), 1, "the retirement naming no words is reported once");
    assert_eq!(reported[0].severity(), Severity::Report);
    assert!(
        reported[0].refusal.message().contains("the witness-era spine"),
        "the report names WHICH retirement is empty, not merely that one is: {}",
        reported[0].refusal.message()
    );
}

/// And the converse: a retirement that does name its vocabulary is not reported.
#[test]
fn a_retirement_with_words_is_not_reported() {
    let found = over(RETIRES, &[]);
    assert!(
        firing(&found, "a-retirement-names-its-vocabulary").is_empty(),
        "a retirement carrying words can enforce something and is left alone"
    );
}

/// C2 — a shipped surface instructing an agent in a retired kind is refused, naming the
/// file and the kind.
///
/// Both halves are asserted. A refusal that said only "some surface teaches a retired kind"
/// would leave the reader to run the audit by hand, which is the reading exercise this rule
/// exists to replace.
#[test]
fn a_surface_teaching_a_retired_kind_is_refused_by_name() {
    let found = over(
        RETIRES,
        &[("skills/witness/SKILL.md", "# Witness\n\nCreate a sprint for this work.\n")],
    );
    let refused = firing(&found, "surface-teaches-a-retired-kind");

    assert_eq!(refused.len(), 1, "one offending line, one refusal");
    assert_eq!(refused[0].severity(), Severity::Refuse);

    let message = refused[0].refusal.message();
    assert!(message.contains("skills/witness/SKILL.md"), "names the file: {message}");
    assert!(message.contains("sprint"), "names the retired kind: {message}");
    assert!(message.contains(":3"), "names the line, so the reader can open it: {message}");
    assert!(
        message.contains("an iteration, opened by the gate"),
        "says what to write instead — a refusal that only forbids leaves the author guessing: \
         {message}"
    );
}

/// One report per LINE, not per occurrence. A line saying "create a sprint, then close the
/// sprint" is one thing to fix.
#[test]
fn one_line_teaching_twice_is_one_refusal() {
    let found = over(
        RETIRES,
        &[("skills/witness/SKILL.md", "Create a sprint, then close the sprint.\n")],
    );
    assert_eq!(firing(&found, "surface-teaches-a-retired-kind").len(), 1);
}

/// C3 — a surface may name a retired kind while explaining that it was retired.
///
/// This is what keeps the rule usable. A rule refusing every mention would make the
/// retirement unexplainable, and an agent reading a method with a silent hole in its history
/// will fill the hole.
#[test]
fn explaining_a_retirement_checks_clean() {
    let found = over(
        RETIRES,
        &[(
            "skills/witness/SKILL.md",
            "The sprint was retired by TS.900; the commitment is now the iteration.\n\
             Waves are no longer part of this method.\n",
        )],
    );
    assert!(
        firing(&found, "surface-teaches-a-retired-kind").is_empty(),
        "naming a retired kind beside a retirement marker is explanation, not instruction"
    );
}

/// The exemption is per line and does not spread. A retirement notice does not license the
/// paragraph under it to go on teaching.
#[test]
fn the_exemption_does_not_leak_to_the_next_line() {
    let found = over(
        RETIRES,
        &[(
            "skills/witness/SKILL.md",
            "The sprint was retired by TS.900.\nCreate a sprint for this work.\n",
        )],
    );
    let refused = firing(&found, "surface-teaches-a-retired-kind");
    assert_eq!(refused.len(), 1, "the explaining line passes and the instructing line does not");
    assert!(refused[0].refusal.message().contains(":2"));
}

/// A word ending in `-` is a PREFIX, not a word — `TS-` is the head of an id form. Dropping
/// the hyphen made it match `tsx`, and every probe script that greps `--include='*.tsx'` was
/// refused for teaching an id form it never mentions.
#[test]
fn a_prefix_word_does_not_match_an_unrelated_token() {
    let id_form = RetiredWord { word: "TS-".to_owned(), instead: None };
    assert!(id_form.matches("ts-041"), "the old id form is the thing being retired");
    assert!(!id_form.matches("tsx"), "a TypeScript extension is not an id");
    assert!(!id_form.matches("ts"), "the bare token is not the id form either");
}

/// Plurals count, and compounds are read both whole and in parts: `wave-based` has to yield
/// `wave`, because that is the exact phrase five harness manifests carry.
#[test]
fn plurals_and_compounds_are_read() {
    let words = [
        RetiredWord { word: "sprint".to_owned(), instead: None },
        RetiredWord { word: "wave".to_owned(), instead: None },
    ];
    let refs: Vec<&RetiredWord> = words.iter().collect();

    let found = teaching("m.json", "lean wave-based product delivery\ntwo sprints\n", &refs, &[]);
    assert_eq!(found.len(), 2);
    assert_eq!(found[0].word, "wave");
    assert_eq!(found[1].word, "sprint");

    let clean = teaching("m.json", "wavelength and sprinters\n", &refs, &[]);
    assert!(clean.is_empty(), "substring matching would refuse both of these");
}

/// C4 — no shipped surface teaches a retired kind.
///
/// The check running over this plugin's own shipped set and refusing nothing, which is the
/// purge being finished rather than claimed. It is the one claim in this slice that cannot
/// be met by a fixture.
#[test]
fn this_plugin_ships_no_surface_teaching_a_retired_kind() {
    let root = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));

    let mut record = Vec::new();
    collect(&root.join("praxis"), "kdl", &mut record);
    let docs: Vec<_> = record
        .iter()
        .map(|(p, t)| parse(t).unwrap_or_else(|e| panic!("{}: {e}", p.display())))
        .collect();

    // The same composition the CLI does: the method the engine carries, extended by what
    // this repository declares. A test that built its own schema would prove something
    // about the test.
    let (mut schema, _) = Schema::method();
    for doc in &docs {
        if doc.nodes().iter().any(|n| n.name().value() == "method") {
            continue;
        }
        let local = Schema::from_document(doc);
        if !local.is_empty() {
            schema.extend(local);
        }
    }

    // And the same denominator: the shapes the config declares, never a set chosen here.
    let shapes = praxis_core::surface::declared_shapes(&docs);
    assert!(!shapes.is_empty(), "the config declares what ships as instruction");
    let mut shipped = Vec::new();
    for (dir, pattern) in &shapes {
        matching(&root.join(dir), root, pattern, &mut shipped);
    }
    shipped.sort();
    shipped.dedup();

    let text: Vec<(String, String)> = shipped
        .iter()
        .filter_map(|p| Some((p.clone(), std::fs::read_to_string(root.join(p)).ok()?)))
        .collect();
    let facts = Facts { shipped, owed: Vec::new(), text };

    let teaching: Vec<String> = check_corpus_given(&docs, &schema, &facts)
        .into_iter()
        .filter(|v| v.refusal.rule() == "surface-teaches-a-retired-kind")
        .map(|v| v.refusal.message())
        .collect();

    assert!(
        teaching.is_empty(),
        "{} shipped surface(s) still teach a retired kind:\n{}",
        teaching.len(),
        teaching.join("\n")
    );
}

fn collect(dir: &std::path::Path, ext: &str, out: &mut Vec<(std::path::PathBuf, String)>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, ext, out);
        } else if path.extension().is_some_and(|e| e == ext)
            && let Ok(text) = std::fs::read_to_string(&path)
        {
            out.push((path, text));
        }
    }
}

fn matching(dir: &std::path::Path, root: &std::path::Path, pattern: &str, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            matching(&path, root, pattern, out);
        } else if praxis_core::surface::matches_shape(
            &entry.file_name().to_string_lossy(),
            pattern,
        ) && let Ok(rel) = path.strip_prefix(root)
        {
            out.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
}
