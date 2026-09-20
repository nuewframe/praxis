//! `TS.260820.13` — see where the product stands right now, without committing a document
//! that ages.
//!
//! `E19`'s test: a DOCUMENT composes several read models; a READ MODEL still has exactly
//! one owner. And `E17`'s: this is the view the publication test rejects by name.

use praxis_core::{Composed, Conditions, Corpus, Schema, dashboard, parse};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug"     each="1"
            field "realizes" each="1"
            field "claim"    each="0..n"
        }
        entity "iteration" {
            field "on-slice" each="1"
            field "state"    each="1"
            field "claim"    each="0..n"
        }
        entity "release" {
            field "version" each="1"
            field "state"   each="1"
            field "binds"   each="0..n"
        }
        entity "capability" id-prefix="CAP." {
            field "state"           each="1"
            field "owns-read-model" each="0..n"
        }
    }
    admits "an iteration on a slice" {
        condition "slice-is-shaped" check="slice-is-shaped" because="a refused slice is a defect"
        condition "the-best-available" judgement=#true because="the tool does not rank"
    }
}
"##;

const RECORD: &str = r##"
thin-slice "TS.a" {
    slug "a-thing"
    realizes "CAP.alpha"
    claim "C1"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    claim "C1" from-slice="TS.a" state="met"
}
// Ownership is DECLARED, and the composer asks the record for it. Until `TS.260823.03`
// the three owners were literals in `dashboard()` — this repository's capability names,
// compiled into the tool every repository runs — so this fixture never needed to say who
// owned anything and the parts came out attributed anyway.
capability "alpha" {
    state "sought"
    owns-read-model "what-is-currently-true"
}
capability "beta" {
    state "sought"
    owns-read-model "what-is-ready-to-pick-up"
}
capability "gamma" {
    state "sought"
    owns-read-model "the-published-set-for-a-release"
}
release "REL.0.1.0" {
    version "0.1.0"
    state "planned"
    binds "ITER.1"
}
"##;

fn compose(record: &str, as_of: &str) -> Composed {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let conditions = Conditions::from_document(&schema_doc);
    let corpus = Corpus::from_documents(&[schema_doc, record_doc], &schema);
    dashboard(&corpus, &conditions, as_of)
}

#[test]
fn c1_three_results_from_three_capabilities_compose_with_no_renderer_change() {
    let document = compose(RECORD, "now");
    assert_eq!(document.parts.len(), 3);
    // Composition is ordering, not interpretation: every part is the same type, and the
    // document knows only who owns each one.
    for part in &document.parts {
        assert!(part.owner.starts_with("CAP."));
        assert!(part.model.flaws().is_empty(), "{:?}", part.model.flaws());
    }
}

#[test]
fn c1_no_owner_appears_in_more_than_one_result() {
    // E19: a composite document would otherwise need a cross-capability owner, and the
    // one-owner rule would break.
    let document = compose(RECORD, "now");
    assert!(document.owners_are_distinct());
    // THIS fixture's capabilities, because ownership is read from the record it composed.
    //
    // Until `TS.260823.03` this line asserted `CAP.delivery-record`, `CAP.work-admission`
    // and `CAP.release-binding` — Praxis's own capability names — against a corpus that
    // declares alpha, beta and gamma and has never held any of the three. The test passed
    // because `dashboard()` carried them as literals, which is the defect stated as an
    // assertion: every adopter's dashboard was attributed to this repository's record.
    let owners: Vec<&str> = document.parts.iter().map(|p| p.owner.as_str()).collect();
    assert_eq!(owners, vec!["CAP.alpha", "CAP.beta", "CAP.gamma"]);
}

#[test]
fn c2_the_dashboard_is_marked_as_of_a_moment_never_as_of_a_release() {
    let document = compose(RECORD, "2026-08-21T09:00:00Z");
    assert_eq!(document.as_of, "2026-08-21T09:00:00Z");
    for part in &document.parts {
        assert_eq!(part.model.as_of, "2026-08-21T09:00:00Z", "{} carries the moment", part.owner);
        assert!(!part.model.publishable, "{} is not publishable", part.owner);
    }
}

#[test]
fn c2_the_release_part_carries_the_moment_even_though_published_it_would_carry_the_version() {
    // The same content, the same seam, a different lifetime. Composed on demand it is
    // current; published into a release directory it depicts a version and carries no
    // clock. That the SAME result can do both is what makes read-model@v1 a seam.
    let document = compose(RECORD, "now");
    let release_part = document
        .parts
        .iter()
        .find(|p| p.model.view == "the-published-set-for-a-release")
        .expect("the release part");
    assert_eq!(release_part.model.as_of, "now");
    assert_eq!(release_part.model.view, "the-published-set-for-a-release");
}

#[test]
fn c3_asking_twice_across_a_state_change_gives_two_different_answers() {
    let before = compose(RECORD, "now");
    let moved = RECORD.replace(
        r#"claim "C1" from-slice="TS.a" state="met""#,
        r#"claim "C1" from-slice="TS.a" state="carried""#,
    );
    let after = compose(&moved, "now");
    assert_ne!(before, after, "no regeneration step — the only input is the record");
}

#[test]
fn c4_composing_produces_no_file_at_all() {
    // Structural, like the preview: `dashboard` returns values. There is no path in the
    // type, so nothing under docs/releases/ can be reached from here.
    let document = compose(RECORD, "now");
    for part in &document.parts {
        for section in &part.model.sections {
            for row in &section.rows {
                for cell in row {
                    assert!(!cell.contains("docs/releases/"), "{cell}");
                }
            }
        }
    }
}

#[test]
fn the_readiness_part_is_omitted_when_the_record_declares_no_gate() {
    // An empty gate is not an open one. Rather than compute readiness from an assumed
    // set of conditions, the part is left out — and its absence is visible because the
    // owner is missing from the document.
    let schema_doc = parse(SCHEMA.replace("admits \"an iteration on a slice\"", "not-admits").as_str())
        .expect("parses");
    let record_doc = parse(RECORD).expect("parses");
    let schema = Schema::from_document(&schema_doc);
    let corpus = Corpus::from_documents(&[schema_doc, record_doc], &schema);
    let document = dashboard(&corpus, &Conditions::default(), "now");
    assert!(!document.parts.iter().any(|p| p.owner == "CAP.work-admission"));
    assert_eq!(document.parts.len(), 2);
}

#[test]
fn a_record_with_no_release_composes_the_parts_it_has() {
    let record = RECORD.replace(
        r##"release "REL.0.1.0" {
    version "0.1.0"
    state "planned"
    binds "ITER.1"
}"##,
        "",
    );
    let document = compose(&record, "now");
    assert_eq!(document.parts.len(), 2);
    assert!(document.owners_are_distinct());
}

#[test]
fn a_document_whose_owners_repeat_is_detectable() {
    // The check is on the DOCUMENT rather than on any model, because that is where the
    // rule can be broken.
    let mut document = compose(RECORD, "now");
    let first = document.parts[0].clone();
    document.parts.push(first);
    assert!(!document.owners_are_distinct());
}

/// `TS.260823.03`/C1 and C2. A composed part asks the record who owns its view, and a view
/// nobody owns yields no part rather than a part with an invented owner.
///
/// The validation review composed this dashboard in a repository whose only capability was
/// `payment`, and was told its parts belonged to `CAP.delivery-record` and
/// `CAP.work-admission` — capabilities that repository had never held. Three literals in
/// `dashboard()` were doing that, in the tool every adopter runs.
#[test]
fn c1_a_view_nobody_owns_is_named_rather_than_attributed() {
    // The same record, with the ownership declarations removed.
    let unowned = RECORD
        .replace(r#"    owns-read-model "what-is-currently-true""#, "")
        .replace(r#"    owns-read-model "what-is-ready-to-pick-up""#, "")
        .replace(r#"    owns-read-model "the-published-set-for-a-release""#, "");
    let document = compose(&unowned, "now");

    assert!(
        document.parts.is_empty(),
        "no capability declares owning anything, so nothing may be attributed: {:?}",
        document.parts.iter().map(|p| &p.owner).collect::<Vec<_>>()
    );
    assert_eq!(
        document.orphaned(),
        ["what-is-currently-true", "what-is-ready-to-pick-up", "the-published-set-for-a-release"],
        "and the views it could not attribute are NAMED — an absent section and an unowned \
         one look identical afterwards"
    );
}

/// C3. The check that cannot itself drift.
///
/// Any capability name in the engine is a name every repository inherits whether it declared
/// it or not, so the assertion is over the source rather than over behaviour.
#[test]
fn c3_no_capability_name_is_a_literal_in_the_engine() {
    let sources = [
        include_str!("../src/dashboard.rs"),
        include_str!("../src/promote.rs"),
        include_str!("../src/guide.rs"),
        include_str!("../src/truth.rs"),
        include_str!("../src/admission.rs"),
    ];
    for text in sources {
        for (number, line) in text.lines().enumerate() {
            // Prose may name them — the comments explaining this fix say `CAP.delivery-record`
            // out loud, and should. What may not appear is a string literal the engine acts on.
            let code = line.split("//").next().unwrap_or_default();
            assert!(
                !code.contains(r#""CAP."#),
                "line {} is a capability name compiled into the engine: {line}",
                number + 1
            );
        }
    }
}
