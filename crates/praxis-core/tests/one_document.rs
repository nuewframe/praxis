//! `TS.260821.14` — a release is one document, and the engineering stays in the record.
//!
//! Seven sibling directories, alphabetical, no index. A reader who wanted outcomes opened
//! `architecture/` because it sorted first, and a reader who wanted the plumbing got there
//! by luck.
//!
//! The root cause was an ordering error in how the documents were built: what the record
//! could COMPUTE was published, and who it was for was asked afterwards. `needed-by` had
//! been on every read model since the storm was written and no projection had ever read it.

use praxis_core::view::ReadModel;
use praxis_core::{
    Corpus, Known, Publication, Schema, check_corpus, check_node, index_all, parse, publish,
};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "frame" is="a problem — symptoms, one root cause, one principle" {
            field "title"      each="1"
            field "root-cause" each="1"
            field "principle"  each="1"
            field "symptom"    each="0..n" holds="symptom"
        }
        entity "symptom" is="one observable thing that is wrong for somebody" {
            field "state" each="1"
        }
        entity "event-storm" is="one sweep of a domain's timeline" {
            field "read-model" each="0..n"
        }
        entity "thin-slice" is="an atomic vertical slice, cut so it can exercise a capability" {
            field "slug"         each="1"
            field "title"        each="1"
            field "trigger"      each="1"
            field "outcome"      each="1"
            field "useful-alone" each="0..1"
            field "command"      each="0..1"
            field "realizes"     each="1" references="capability"
        }
        entity "capability" is="a permanent doing the system must have" {
            field "state"            each="1"
            field "facet"            each="0..1"
            field "doing"            each="1"
            field "not"              each="1"
            field "keeps-consistent" each="0..n"
        }
        entity "iteration" is="one commitment — the slices an ask committed to" {
            field "on-slice" each="1..n" references="thin-slice"
            field "state"    each="1"
            field "finding"  each="0..n"
        }
        entity "release" is="an index into history — what it binds, what it resolves" {
            field "version" each="1"
            field "state"   each="1"
            field "binds"   each="0..n" references="iteration"
        }
        entity "walkthrough" is="one path through the method, walked on paper before it is built" {
            field "slug" each="1"
        }
        rule "publish-only-what-survives-freezing" refuses="a read model that does not declare whether it survives being frozen"
        rule "a-published-view-names-its-reader" reports="a publishable view naming no persona it is for"
    }
}
"##;

/// One storm, one story. The engineering views are declared and NOT published: they carry
/// `publishable=#false` with the reason, which is what `declare-a-lifetime` already owns.
const RECORD: &str = r##"
frame "FRAME.test" {
    title "Trust in an artifact is unearned"
    root-cause "fidelity is invisible in the artifact"
    principle "fidelity is computed from the record, never claimed by whoever produced it"
    symptom "S1" state="present"
}
event-storm "ES.test" {
    read-model "where-to-start" needed-by="the-reader-of-a-release" \
        answers="what is this, what can it do, and where do I begin?" \
        publishable=#true because="it IS the release" \
        publishes-to="docs/releases/<version>/README.md" {
        part "1" is="why this exists" from="what-this-product-means"
        part "2" is="what it can do"  from="capabilities-and-what-they-own"
        part "3" is="what changed"    from="the-published-set-for-a-release"
    }
    read-model "what-this-product-means" needed-by="the-reader-of-a-release" \
        answers="what problem does this exist for, and what does every word mean?" \
        publishable=#true because="the words are what a changelog about slices needs"
    read-model "capabilities-and-what-they-own" needed-by="the-team-building-a-digital-product" \
        answers="what must the system be able to do?" \
        publishable=#true because="what it could do at a version becomes history"
    read-model "the-published-set-for-a-release" needed-by="the-reader-of-a-release" \
        answers="what can you do now?" \
        publishable=#true because="a release note is useful only frozen"
    read-model "how-it-fits-together" \
        answers="how does it fit?" \
        publishable=#false because="engineering. `praxis view` answers it for the tree in front of you"
    read-model "the-decisions-that-shaped-this" \
        answers="what was decided?" \
        publishable=#false because="the seal already freezes it, and better"
    read-model "what-this-plugin-ships" \
        answers="what doctrine did this version ship?" \
        publishable=#false because="an adopter has the repository"
}
capability "the-gate" {
    state "active"
    facet "product"
    doing "admit work, or refuse it in writing"
    not "it never decides whether the work is worth doing"
    keeps-consistent "at most one open iteration per slice"
}
thin-slice "TS.a" {
    slug "pick-up-a-slice"
    title "Take a slice, and have the refusal recorded when it is refused"
    trigger "somebody wants to start work"
    outcome "the gate admits, or refuses and says which condition failed"
    useful-alone "a refusal stops being invisible. Worth having with nothing else changed"
    command "pick-up"
    realizes "CAP.the-gate"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    finding "F1" text="disjointness is declared and never computed" carries="C3"
}
release "REL.0.1.0" {
    version "0.1.0"
    state "planned"
    binds "ITER.1"
}
"##;

fn corpus_of(record: &str) -> Corpus {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    Corpus::from_documents(&[schema_doc, record_doc], &schema)
}

fn documents(record: &str) -> Vec<praxis_core::Document> {
    match publish("0.1.0", &corpus_of(record)) {
        Publication::Ready { documents, .. } => documents,
        Publication::Refused(why) => panic!("publish refused: {why:?}"),
    }
}

fn violations(record: &str) -> Vec<praxis_core::Violation> {
    let schema_doc = parse(SCHEMA).expect("schema parses");
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

fn section<'a>(model: &'a ReadModel, name: &str) -> &'a praxis_core::Section {
    model.sections.iter().find(|s| s.name == name).unwrap_or_else(|| {
        panic!("no section {name:?} — it holds {:?}", model.sections.iter().map(|s| &s.name).collect::<Vec<_>>())
    })
}

/// C1 — a release is ONE document with a declared order, not a set of sibling directories.
///
/// The order is the record's, not the filesystem's. Alphabetically `capabilities` precedes
/// `where`, and the reader who arrives wanting to know what this IS would meet the
/// capability list first — which is how the seven-directory version failed.
#[test]
fn a_release_is_one_document_in_the_order_the_record_declares() {
    let published = documents(RECORD);
    assert_eq!(published.len(), 1, "one document: {:?}", published.iter().map(|d| &d.file).collect::<Vec<_>>());
    assert!(published[0].file.ends_with("README.md"), "{}", published[0].file);

    let read_in_order: Vec<&str> = published[0].models.iter().map(|m| m.view.as_str()).collect();
    assert_eq!(
        read_in_order,
        vec![
            "where-to-start",
            "what-this-product-means",
            "capabilities-and-what-they-own",
            "the-published-set-for-a-release",
        ],
        "the story's own order, and not the alphabet"
    );

    // The declared order is the record's to change. Reversing the parts reverses the
    // document, which is what makes this a declaration rather than a coincidence.
    let reordered = RECORD
        .replace(r#"part "1" is="why this exists" from="what-this-product-means""#, "@@1@@")
        .replace(r#"part "3" is="what changed"    from="the-published-set-for-a-release""#, r#"part "3" is="why this exists" from="what-this-product-means""#)
        .replace("@@1@@", r#"part "1" is="what changed" from="the-published-set-for-a-release""#);
    let swapped = documents(&reordered);
    assert_eq!(
        swapped[0].models[1].view, "the-published-set-for-a-release",
        "the order comes from the record, so changing the record changes the document"
    );
}

/// C2 — each index entry carries the question that document answers, from the view's own
/// `answers`, verbatim.
///
/// Copied and not paraphrased. A publisher that rewords its source is a second author, and
/// a reader has no way to tell which of the two they are reading.
#[test]
fn the_index_quotes_each_view_s_own_question() {
    let published = documents(RECORD);
    let index = section(published[0].model(), "what is in here");

    for (part, view) in [
        ("why this exists", "what-this-product-means"),
        ("what it can do", "capabilities-and-what-they-own"),
        ("what changed", "the-published-set-for-a-release"),
    ] {
        let declared = corpus_of(RECORD)
            .views
            .iter()
            .find(|v| v.name == view)
            .expect("declared")
            .answers
            .clone();
        let row = index
            .rows
            .iter()
            .find(|r| r[0] == part)
            .unwrap_or_else(|| panic!("no index entry for {part}"));
        assert_eq!(row[1], declared, "the index quotes {view}, verbatim");
    }
}

/// C3 — a publishable view naming no audience is reported, and NAMED.
///
/// Both halves matter. The checker reports it because a document nobody is for is one
/// nobody will read; the story lists it under `addressed to nobody` because hiding the view
/// hides the fault with it.
#[test]
fn a_view_for_nobody_is_reported_and_named_in_the_index() {
    let anonymous = RECORD.replace(
        r#"read-model "capabilities-and-what-they-own" needed-by="the-team-building-a-digital-product" \"#,
        r#"read-model "capabilities-and-what-they-own" \"#,
    );

    let reported: Vec<_> = violations(&anonymous)
        .into_iter()
        .filter(|v| v.refusal.rule() == "a-published-view-names-its-reader")
        .collect();
    assert_eq!(reported.len(), 1, "{reported:?}");
    assert_eq!(reported[0].severity(), praxis_core::Severity::Report);
    assert!(reported[0].refusal.message().contains("capabilities-and-what-they-own"));

    let published = documents(&anonymous);
    let unaddressed = section(published[0].model(), "addressed to nobody");
    assert_eq!(
        unaddressed.rows.iter().map(|r| r[0].as_str()).collect::<Vec<_>>(),
        vec!["what it can do"],
        "named in the story, not dropped from it"
    );
    // And the index still carries it, with the gap said out loud.
    let index = section(published[0].model(), "what is in here");
    let row = index.rows.iter().find(|r| r[0] == "what it can do").expect("still indexed");
    assert_eq!(row[2], "nobody named");

    // With a reader, both go quiet.
    let named = documents(RECORD);
    assert!(section(named[0].model(), "addressed to nobody").is_empty());
    assert!(
        !violations(RECORD)
            .iter()
            .any(|v| v.refusal.rule() == "a-published-view-names-its-reader")
    );
}

/// C4 — the glossary carries the words the story uses, in dependency order.
///
/// Nothing is defined before the words it is defined in terms of: a container introduces
/// what it contains, and a kind you must already know comes before the kind that references
/// it. That puts `frame` before `thin-slice` before `iteration`, which is the order somebody
/// learns them in rather than the order the schema happens to be written in.
#[test]
fn the_glossary_carries_the_story_s_words_in_dependency_order() {
    // The order, over the whole declared vocabulary. `frame` holds `symptom`, `thin-slice`
    // references it, and `iteration` references the slice — so a reader meets each word
    // before the word defined in terms of it.
    let corpus = corpus_of(RECORD);
    let vocabulary: Vec<&str> = corpus.vocabulary.iter().map(|(kind, _)| kind.as_str()).collect();
    let rank = |word: &str| {
        vocabulary
            .iter()
            .position(|w| *w == word)
            .unwrap_or_else(|| panic!("{word} is missing: {vocabulary:?}"))
    };
    assert!(rank("frame") < rank("thin-slice"), "{vocabulary:?}");
    assert!(rank("thin-slice") < rank("iteration"), "{vocabulary:?}");

    // And the words, over the story. A word the story never says is absent: the full
    // vocabulary is a command, not a release artifact, and a reader looking up
    // `walkthrough` after reading a release that never mentions one has been handed the
    // schema instead of a document.
    let published = documents(RECORD);
    let concepts = published[0]
        .models
        .iter()
        .find(|m| m.view == "what-this-product-means")
        .expect("part one");
    let words: Vec<&str> =
        section(concepts, "what the words mean").rows.iter().map(|r| r[0].as_str()).collect();
    assert!(words.contains(&"thin-slice"), "the story is about slices: {words:?}");
    assert!(!words.contains(&"walkthrough"), "{words:?}");

    // The glossary is a subset of the order, not a re-sort of it.
    let kept: Vec<usize> = words.iter().map(|w| rank(w)).collect();
    assert!(kept.windows(2).all(|p| p[0] < p[1]), "{words:?}");
}

/// C5 — the engineering views are in the record and answerable there, not published.
///
/// Flipping them is a lifetime decision and nothing else. A decision record is not less
/// true for living in the record: an accepted decision is sealed and append-only, which is
/// a stronger guarantee than a copy in a release directory.
#[test]
fn the_engineering_views_are_answerable_and_unpublished() {
    let published = documents(RECORD);
    let in_the_release: Vec<&str> = published
        .iter()
        .flat_map(|d| d.models.iter().map(|m| m.view.as_str()))
        .collect();

    let corpus = corpus_of(RECORD);
    for engineering in
        ["the-decisions-that-shaped-this", "how-it-fits-together", "what-this-plugin-ships"]
    {
        assert!(
            !in_the_release.contains(&engineering),
            "{engineering} is engineering and a reader of a release did not come for it"
        );
        // Still declared, still reasoned about, and still composable from the record —
        // which is what `praxis view` reaches. Unpublished is not unanswerable.
        let view = corpus
            .views
            .iter()
            .find(|v| v.name == engineering)
            .unwrap_or_else(|| panic!("{engineering} left the record entirely"));
        assert_eq!(view.publishable, Some(false));
        assert!(view.because.is_some(), "a lifetime with no reason is an oversight");
        assert!(
            praxis_core::compose_view(engineering, "0.1.0", &corpus).is_some(),
            "{engineering} must still compose, or `it lives in the record` is a claim with \
             nothing behind it"
        );
    }
}

/// C1's other half — a part naming a view the engine cannot compose refuses the publish.
///
/// A story with a missing chapter is worse than no story: it reads as complete.
#[test]
fn a_story_with_a_chapter_the_engine_cannot_compose_is_refused() {
    let broken = RECORD.replace(
        r#"part "2" is="what it can do"  from="capabilities-and-what-they-own""#,
        r#"part "2" is="what it can do"  from="a-view-nobody-wrote-a-composer-for""#,
    );
    match publish("0.1.0", &corpus_of(&broken)) {
        Publication::Refused(why) => assert!(
            why.iter().any(|w| w.contains("a-view-nobody-wrote-a-composer-for")),
            "{why:?}"
        ),
        Publication::Ready { documents, .. } => {
            panic!("published a story with a hole in it: {documents:?}")
        }
    }
}

/// A view a story carries needs no `publishes-to` of its own, and one nothing carries still
/// does.
///
/// The lifetime rule asks where a publishable view LANDS. `where-to-start` answers that for
/// its parts, and for nothing else.
#[test]
fn a_carried_view_lands_where_the_story_lands() {
    assert!(
        !violations(RECORD)
            .iter()
            .any(|v| v.refusal.rule() == "publish-only-what-survives-freezing"),
        "{:?}",
        violations(RECORD)
    );

    let orphaned = RECORD.replace(r#"part "2" is="what it can do"  from="capabilities-and-what-they-own""#, "");
    let found: Vec<_> = violations(&orphaned)
        .into_iter()
        .filter(|v| v.refusal.rule() == "publish-only-what-survives-freezing")
        .collect();
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].refusal.message().contains("publishes-to"));
}
