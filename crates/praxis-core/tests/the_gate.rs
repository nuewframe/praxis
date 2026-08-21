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
        by: "agent:praxis".to_owned(),
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
    pick_up(slice, &corpus, &assessment, &ask(), &[])
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
    let second = pick_up("TS.ready", &corpus, &assessment, &ask(), &[a.id.clone()]);
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
            matches!(pick_up(id, &corpus, &assessment, &ask(), &[]), Pickup::Opened(_)),
            "{id} is listed ready and the gate refused it"
        );
    }
    // And every slice it excludes is refused — whether excluded as blocked or as done.
    for id in blocked.iter().chain(delivered.iter()) {
        assert!(
            matches!(pick_up(id, &corpus, &assessment, &ask(), &[]), Pickup::Refused(_)),
            "{id} is excluded by the view and the gate admitted it"
        );
    }
}
