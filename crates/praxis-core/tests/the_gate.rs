//! `TS.260820.05` — take a slice, and have the refusal recorded when it is refused.
//!
//! And the residue of `TS.260820.04`/C1, which could not be settled by the iteration that
//! raised it: the view and the gate evaluate identical conditions. It is settled here
//! because settling it required the gate to exist, and the gate depended on the view
//! (ITER.260821.03/S1).

use praxis_core::{Ask, Conditions, Corpus, Pickup, Schema, assess, parse, pick_up, project};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "thin-slice" {
            field "slug"       each="1"
            field "kind"       each="1"
            field "realizes"   each="1"
            field "depends-on" each="0..n"
            field "claim"      each="0..n"
        }
        entity "iteration" {
            field "on-slice" each="1"
            field "state"    each="1"
            field "claim"    each="0..n"
        }
    }
    admits "an iteration on a slice" {
        condition "slice-is-shaped" check="slice-is-shaped" because="a refused slice is a defect"
        condition "dependencies-delivered" check="dependencies-delivered" because="standing on nothing"
        condition "no-iteration-in-flight" check="no-iteration-in-flight" because="one open per slice"
        condition "nothing-left-to-admit" check="nothing-left-to-admit" because="done is not blocked"
        condition "work-is-disjoint" uncomputed="no seam manifest exists" because="overlap is a merge"
        condition "the-best-available" judgement=#true because="the tool does not rank"
    }
}
"##;

fn ask() -> Ask {
    Ask {
        signer: "human:someone".to_owned(),
        at: "2026-08-21T09:00:00Z".to_owned(),
        by: "agent:praxis".to_owned(), attested_by: None,
    }
}

fn gate(record: &str, slice: &str) -> Pickup {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let conditions = Conditions::from_document(&schema_doc);
    let docs = vec![schema_doc, record_doc];
    let corpus = Corpus::from_documents(&docs, &schema);
    let assessment = assess(&corpus, &conditions, &ask().at);
    pick_up(&[slice.to_owned()], &corpus, &assessment, &ask(), &[])
}

fn refusal(pickup: &Pickup) -> &praxis_core::Record {
    match pickup {
        Pickup::Refused(record) => record,
        other => panic!("expected a refusal, got {other:?}"),
    }
}

const READY: &str = r##"
thin-slice "TS.ready" {
    slug "ready"
    kind "command"
    realizes "CAP.x"
    claim "C1"
}
"##;

#[test]
fn c1_a_refused_pickup_names_the_condition_dependencies() {
    let pickup = gate(
        r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
    claim "C1"
}
thin-slice "TS.b" {
    slug "b"
    kind "command"
    realizes "CAP.x"
    claim "C1"
    depends-on "TS.a — first"
}
"##,
        "TS.b",
    );
    let record = refusal(&pickup);
    assert_eq!(record.failed.len(), 1);
    assert_eq!(record.failed[0].0, "dependencies-delivered");
    assert!(record.failed[0].1.contains("TS.a"), "the SPECIFIC dependency");
    assert!(record.kdl.contains(r#"condition "dependencies-delivered" failed=#true"#));
    assert!(record.kdl.contains("TS.a is not delivered"));
}

#[test]
fn c1_a_refused_pickup_names_the_condition_shape() {
    // The slice is missing `realizes`, which the schema requires.
    let pickup = gate(
        r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    claim "C1"
}
"##,
        "TS.a",
    );
    let record = refusal(&pickup);
    assert!(record.failed.iter().any(|(c, _)| c == "slice-is-shaped"));
    assert!(record.kdl.contains("the shape check refuses this slice"));
}

#[test]
fn c1_a_refused_pickup_names_the_condition_already_delivered() {
    let pickup = gate(
        r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
    claim "C1"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "closed"
    claim "C1" from-slice="TS.a" state="met"
}
"##,
        "TS.a",
    );
    let record = refusal(&pickup);
    assert!(record.failed.iter().any(|(c, d)| c == "nothing-left-to-admit" && d.contains("ITER.1")));
}

#[test]
fn c3_a_second_iteration_on_a_live_slice_is_refused() {
    let pickup = gate(
        r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
    claim "C1"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "working"
}
"##,
        "TS.a",
    );
    let record = refusal(&pickup);
    assert!(
        record.failed.iter().any(|(c, d)| c == "no-iteration-in-flight" && d.contains("ITER.1")),
        "and it names WHICH iteration holds it: {:?}",
        record.failed
    );
}

#[test]
fn c2_a_refused_pickup_opens_nothing() {
    let pickup = gate(
        r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
    claim "C1"
}
iteration "ITER.1" {
    on-slice "TS.a"
    state "working"
}
"##,
        "TS.a",
    );
    let record = refusal(&pickup);
    assert!(!record.kdl.contains("iteration \""), "no iteration is composed");
    assert!(record.kdl.starts_with("//") && record.kdl.contains("refusal \""));
    assert!(record.id.starts_with("REF."), "and it is not named like one either");
    assert!(record.file.contains("iterations/"), "beside the iterations, which is not among them");
}

#[test]
fn an_admission_opens_an_iteration_that_the_record_will_accept() {
    let pickup = gate(READY, "TS.ready");
    let Pickup::Opened(record) = &pickup else {
        panic!("expected an admission, got {pickup:?}");
    };
    assert!(record.failed.is_empty());
    assert!(record.id.starts_with("ITER."));

    // It must parse, and it must carry what the schema requires of an iteration.
    let doc = parse(&record.kdl).expect("the gate writes valid KDL");
    let node = doc.nodes().iter().find(|n| n.name().value() == "iteration").expect("an iteration");
    for required in ["slug", "on-slice", "state", "opened-at", "opened-by", "approval", "vet", "trail"] {
        assert!(
            node.iter_children().any(|c| c.name().value() == required),
            "the opened iteration carries `{required}`"
        );
    }
    assert!(record.kdl.contains(r#"signer "human:someone""#), "signed by a human, or not signed");
    assert!(record.kdl.contains(r#"state "open""#), "opened, not working — the start vet is a second ask");
}

#[test]
fn the_gate_records_what_it_did_not_decide() {
    // An admission that does not say what it left undecided claims more than it checked.
    let pickup = gate(READY, "TS.ready");
    let Pickup::Opened(record) = &pickup else { panic!("expected an admission") };
    assert!(record.kdl.contains(r#"verdict="not-computed""#));
    assert!(record.kdl.contains("no seam manifest exists"));
    assert!(record.kdl.contains(r#"verdict="left-to-the-maintainer""#));
}

#[test]
fn an_ask_naming_no_slice_decides_nothing() {
    match gate(READY, "TS.nowhere") {
        Pickup::NoSuchSlice(why) => assert!(why.contains("TS.nowhere")),
        other => panic!("neither admitted nor refused: {other:?}"),
    }
}

#[test]
fn two_pickups_on_one_day_do_not_contend() {
    let first = gate(READY, "TS.ready");
    let Pickup::Opened(a) = &first else { panic!("expected an admission") };

    let schema_doc = parse(SCHEMA).expect("parses");
    let record_doc = parse(READY).expect("parses");
    let schema = Schema::from_document(&schema_doc);
    let conditions = Conditions::from_document(&schema_doc);
    let docs = vec![schema_doc, record_doc];
    let corpus = Corpus::from_documents(&docs, &schema);
    let assessment = assess(&corpus, &conditions, &ask().at);
    let second = pick_up(&["TS.ready".to_owned()], &corpus, &assessment, &ask(), &[a.id.clone()]);
    let Pickup::Opened(b) = &second else { panic!("expected an admission") };

    assert_ne!(a.id, b.id, "two agents on one day get distinct files and never contend");
    assert_ne!(a.file, b.file);
}

/// `TS.260820.04`/C1, in full. The residue ITER.260821.03 could not settle.
#[test]
fn ts_260820_04_c1_the_view_and_the_gate_evaluate_identical_conditions() {
    let record = r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
    claim "C1"
}
thin-slice "TS.b" {
    slug "b"
    kind "command"
    realizes "CAP.x"
    claim "C1"
    depends-on "TS.a — first"
}
thin-slice "TS.c" {
    slug "c"
    kind "command"
    realizes "CAP.x"
    claim "C1"
}
iteration "ITER.1" {
    on-slice "TS.c"
    state "working"
}
thin-slice "TS.d" {
    slug "d"
    kind "command"
    realizes "CAP.x"
    claim "C1"
}
iteration "ITER.2" {
    on-slice "TS.d"
    state "closed"
    claim "C1" from-slice="TS.d" state="met"
}
"##;
    let schema_doc = parse(SCHEMA).expect("parses");
    let record_doc = parse(record).expect("parses");
    let schema = Schema::from_document(&schema_doc);
    let conditions = Conditions::from_document(&schema_doc);
    let docs = vec![schema_doc, record_doc];
    let corpus = Corpus::from_documents(&docs, &schema);
    let assessment = assess(&corpus, &conditions, &ask().at);
    let model = project(&assessment, &corpus);

    let listed = |section: &str| -> Vec<String> {
        model
            .sections
            .iter()
            .find(|s| s.name == section)
            .expect("a section")
            .rows
            .iter()
            .map(|r| r[0].clone())
            .collect()
    };
    let ready = listed("ready");
    let blocked = listed("blocked");
    let delivered = listed("delivered");
    assert!(!ready.is_empty() && !blocked.is_empty() && !delivered.is_empty(), "all three occur");

    // Every slice the VIEW calls ready is admitted by the GATE.
    for id in &ready {
        assert!(
            matches!(pick_up(&[id.to_owned()], &corpus, &assessment, &ask(), &[]), Pickup::Opened(_)),
            "{id} is listed ready and the gate refused it"
        );
    }
    // And every slice it excludes is refused — whether excluded as blocked or as done.
    for id in blocked.iter().chain(delivered.iter()) {
        assert!(
            matches!(pick_up(&[id.to_owned()], &corpus, &assessment, &ask(), &[]), Pickup::Refused(_)),
            "{id} is excluded by the view and the gate admitted it"
        );
    }
}

/// The ITERATION is the commitment; a slice is a unit of work. One ask can commit to
/// several, and they are worked together (ITER.260821.19).
const TWO_READY: &str = r##"
thin-slice "TS.one" {
    slug "one"
    kind "command"
    realizes "CAP.x"
    claim "C1"
}
thin-slice "TS.two" {
    slug "two"
    kind "command"
    realizes "CAP.x"
    claim "C1"
    claim "C2"
}
"##;

fn gate_many(record: &str, slices: &[&str]) -> Pickup {
    let schema_doc = parse(SCHEMA).expect("schema parses");
    let record_doc = parse(record).expect("record parses");
    let schema = Schema::from_document(&schema_doc);
    let conditions = Conditions::from_document(&schema_doc);
    let docs = vec![schema_doc, record_doc];
    let corpus = Corpus::from_documents(&docs, &schema);
    let assessment = assess(&corpus, &conditions, &ask().at);
    let ids: Vec<String> = slices.iter().map(|s| (*s).to_owned()).collect();
    pick_up(&ids, &corpus, &assessment, &ask(), &[])
}

#[test]
fn one_ask_over_several_slices_opens_one_commitment() {
    let Pickup::Opened(record) = gate_many(TWO_READY, &["TS.one", "TS.two"]) else {
        panic!("both are admissible")
    };
    let doc = parse(&record.kdl).expect("valid KDL");
    let node = doc.nodes().iter().find(|n| n.name().value() == "iteration").expect("an iteration");

    let on: Vec<&str> = node
        .iter_children()
        .filter(|c| c.name().value() == "on-slice")
        .filter_map(|c| c.entries().first().and_then(|e| e.value().as_string()))
        .collect();
    assert_eq!(on, vec!["TS.one", "TS.two"], "one record, both slices");
    assert_eq!(
        doc.nodes().iter().filter(|n| n.name().value() == "iteration").count(),
        1,
        "one commitment, not one iteration each"
    );
}

#[test]
fn the_commitment_pins_every_claim_of_every_slice_it_covers() {
    let Pickup::Opened(record) = gate_many(TWO_READY, &["TS.one", "TS.two"]) else {
        panic!("expected an admission")
    };
    // Three claims across two slices, each carrying the slice that declared it — which is
    // what lets the close check C3 against the right one.
    assert_eq!(record.kdl.matches("    claim ").count(), 3);
    assert!(record.kdl.contains(r#"claim "C1" from-slice="TS.one""#));
    assert!(record.kdl.contains(r#"claim "C1" from-slice="TS.two""#));
    assert!(record.kdl.contains(r#"claim "C2" from-slice="TS.two""#));
}

#[test]
fn each_slice_is_vetted_separately_and_the_verdicts_are_kept_apart() {
    let Pickup::Opened(record) = gate_many(TWO_READY, &["TS.one", "TS.two"]) else {
        panic!("expected an admission")
    };
    assert_eq!(record.kdl.matches(r#"vet "create""#).count(), 2, "one vet per slice");
    assert!(record.kdl.contains(r#"on-slice="TS.one""#));
    assert!(record.kdl.contains(r#"on-slice="TS.two""#));
}

#[test]
fn a_commitment_is_refused_whole_when_any_slice_is_refused() {
    // A commitment that admits its easy half is not one thing, and the ask was for one
    // thing.
    let record = format!(
        "{TWO_READY}\niteration \"ITER.1\" {{\n    on-slice \"TS.two\"\n    state \"working\"\n}}\n"
    );
    let Pickup::Refused(refusal) = gate_many(&record, &["TS.one", "TS.two"]) else {
        panic!("TS.two is held by an open iteration, so the commitment cannot be admitted")
    };
    assert!(
        refusal.failed.iter().any(|(c, _)| c.contains("no-iteration-in-flight") && c.contains("TS.two")),
        "and the refusal says WHICH slice failed: {:?}",
        refusal.failed
    );
    assert!(
        !refusal.failed.iter().any(|(c, _)| c.contains("TS.one")),
        "the admissible one is not accused"
    );
}

#[test]
fn a_single_slice_ask_is_unchanged_by_any_of_this() {
    // The suffix that names which slice failed is noise when there is only one, and
    // eighteen iterations of records were written without it.
    let record = r##"
thin-slice "TS.a" {
    slug "a"
    kind "command"
    realizes "CAP.x"
    claim "C1"
}
thin-slice "TS.b" {
    slug "b"
    kind "command"
    realizes "CAP.x"
    claim "C1"
    depends-on "TS.a — first"
}
"##;
    let Pickup::Refused(refusal) = gate_many(record, &["TS.b"]) else { panic!("expected refusal") };
    assert_eq!(refusal.failed[0].0, "dependencies-delivered", "no slice suffix");
}
