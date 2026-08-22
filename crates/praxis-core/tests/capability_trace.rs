//! Evidence for `TS.260820.02`, claims C1–C3.
//!
//! `capability-must-trace` has been declared in `ES.260819.01` since the storm was held
//! and enforced by nothing since. These tests are what change that.

use std::fs;

use kdl::KdlDocument;
use praxis_core::{Known, Refusal, Schema, Severity, check_corpus, check_document, index_all, parse};

const ARCHITECTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../praxis/frames/FRAME.260819.01/discovery/",
    "NA.260820.01.capabilities-seams-and-who-may-change-what.kdl"
);

/// The method, extended by this repository's own architecture — which is what every command
/// does since `TS.260821.10`.
///
/// This used to read the architecture as THE schema, because the method's vocabulary lived
/// inside it. `capability`, `event-storm` and the rest are the method's now; what stays in
/// the architecture is what this repository adds. Reading only one of the two gives a schema
/// missing whichever half you did not read.
fn schema() -> Schema {
    let text = fs::read_to_string(ARCHITECTURE).expect("readable");
    let (mut schema, _) = Schema::method();
    schema.extend(Schema::from_document(&parse(&text).expect("parses")));
    schema
}

/// A storm holding one cluster and two events, so a capability has something real to
/// trace to and the coverage rule has something to be unsatisfied about.
fn storm() -> &'static str {
    r#"
event-storm "ES.TEST" {
    title "a test storm"
    frame "FRAME.TEST"
    command "do-a-thing"
    event "ThingDone"
    event "ThingRefused"
    cluster "the-cluster" {
        owns "ThingDone"
    }
}
"#
}

fn capability(id: &str, derived: &str, cluster: Option<&str>, events: &str) -> String {
    let from_cluster = cluster.map_or(String::new(), |c| format!("    from-cluster \"{c}\"\n"));
    format!(
        r#"
capability "{id}" {{
    doing "a doing"
    not "an exclusion"
    state "sought"
    derived-from "{derived}"
{from_cluster}    owns-event {events}
    owns-read-model "a-view"
    gate-test "names-a-doing" verdict="pass"
    keeps-consistent "something"
    attacks "S1"
    trail {{ entry at="t" by="human:x" action="created" }}
}}
"#
    )
}

fn corpus(sources: &[String]) -> (Vec<Refusal>, Vec<Refusal>) {
    let schema = schema();
    let docs: Vec<KdlDocument> = sources.iter().map(|s| parse(s).expect("fixture parses")).collect();
    let mut known = Known::default();
    for doc in &docs {
        index_all(doc, &mut known);
    }
    let mut all: Vec<_> = docs
        .iter()
        .flat_map(|d| check_document(d, &schema, &known))
        .collect();
    all.extend(check_corpus(&docs, &schema));
    let (refused, reported): (Vec<_>, Vec<_>) =
        all.into_iter().partition(|v| v.severity() == Severity::Refuse);
    (
        refused.into_iter().map(|v| v.refusal).collect(),
        reported.into_iter().map(|v| v.refusal).collect(),
    )
}

/// C1 — a capability naming no storm cluster is refused, and the diagnostic names it.
#[test]
fn c1_a_capability_with_no_cluster_is_refused() {
    let (refused, _) = corpus(&[
        storm().to_owned(),
        capability("untraced", "ES.TEST", None, r#""ThingDone""#),
    ]);
    let found = refused
        .iter()
        .find(|r| matches!(r, Refusal::MissingField { field, .. } if field == "from-cluster"))
        .expect("a capability with no cluster must be refused");
    assert!(
        found.message().contains("from-cluster") && found.message().contains("derivation"),
        "the refusal must say the cluster IS the derivation: {}",
        found.message()
    );
}

/// C1, second half — naming a cluster that does not exist is refused too. Presence is
/// not a trace; the cluster has to be real.
#[test]
fn c1_a_cluster_that_does_not_exist_is_refused() {
    let (refused, _) = corpus(&[
        storm().to_owned(),
        capability("invented", "ES.TEST", Some("no-such-cluster"), r#""ThingDone""#),
    ]);
    assert!(
        refused.iter().any(|r| matches!(
            r,
            Refusal::DanglingReference { field, kind, .. } if field == "from-cluster" && kind == "cluster"
        )),
        "a cluster that does not exist must be refused; got {refused:?}"
    );
}

/// C2 — an event owned by two capabilities is refused, naming both.
#[test]
fn c2_an_event_owned_twice_is_refused_naming_both() {
    let (refused, _) = corpus(&[
        storm().to_owned(),
        capability("first", "ES.TEST", Some("the-cluster"), r#""ThingDone""#),
        capability("second", "ES.TEST", Some("the-cluster"), r#""ThingDone""#),
    ]);
    let found = refused
        .iter()
        .find(|r| matches!(r, Refusal::ContestedValue { .. }))
        .expect("a contested event must be refused");
    let message = found.message();
    assert!(
        message.contains("ThingDone") && message.contains("first"),
        "the refusal must name the event and the other claimant: {message}"
    );
}

/// C3 — an event owned by no capability is REPORTED, never refused. A gap in the model
/// is not a malformed record.
#[test]
fn c3_an_orphan_event_is_reported_not_refused() {
    let (refused, reported) = corpus(&[
        storm().to_owned(),
        capability("only", "ES.TEST", Some("the-cluster"), r#""ThingDone""#),
    ]);
    assert!(
        !refused.iter().any(|r| matches!(r, Refusal::UnclaimedValue { .. })),
        "an orphan event must not fail closed"
    );
    let found = reported
        .iter()
        .find(|r| matches!(r, Refusal::UnclaimedValue { value, .. } if value == "ThingRefused"))
        .expect("the unowned event must be reported");
    assert!(
        found.message().contains("belongs to nothing"),
        "the report must say what is wrong: {}",
        found.message()
    );
}

/// A fully traced capability that owns every event is admitted with nothing reported.
#[test]
fn a_traced_capability_owning_everything_is_clean() {
    let (refused, reported) = corpus(&[
        storm().to_owned(),
        capability("whole", "ES.TEST", Some("the-cluster"), r#""ThingDone" "ThingRefused""#),
    ]);
    assert_eq!(refused, vec![], "a traced capability must not be refused");
    assert!(
        !reported.iter().any(|r| matches!(r, Refusal::UnclaimedValue { .. })),
        "every event is owned, so nothing should be reported unclaimed"
    );
}
