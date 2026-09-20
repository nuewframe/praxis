//! `TS.260823.04` — a field is read in whichever valid form it was written.
//!
//! Three traps, one fault: the schema declared a field and the ENGINE decided what shape it
//! took. An author reading `praxis schema` could not tell which, and the refusal did not say.
//!
//! ```text
//! read-model "x" { publishable #false }   refused: "does not say whether it survives
//!                                          being frozen" — while it said so above
//! owns-event "A"                          refused: "appears 2 times; the schema
//! owns-event "B"                           requires exactly one"
//! followed="a" followed="b"               three of four approaches read as unfollowed
//! ```
//!
//! All three are valid KDL. The validation review hit every one inside twenty minutes while
//! building a first record by FOLLOWING the skills — and the fastest way past them was to
//! copy an existing record and mutate it, which is transcription, the one move
//! `adopt-the-method` spends its longest section arguing against.
//!
//! `D1` chose form-agnostic reading over declaring a form per field: nothing in this record
//! needs to insist on a form, and a method already carrying 27 kinds and 158 field names does
//! not need vocabulary for a distinction nobody wants enforced.

use praxis_core::{Known, Schema, check_document, parse};

const SCHEMA: &str = r##"
notional-architecture "NA.test" {
    schema {
        entity "capability" id-prefix="CAP." {
            field "doing"      each="1"
            field "owns-event" each="1..n"
        }
        entity "reading" {
            field "answers"     each="1"
            field "publishable" each="1"
        }
    }
}
"##;

fn refusals(record: &str) -> Vec<String> {
    let schema_doc = parse(SCHEMA).expect("the schema parses");
    let doc = parse(record).expect("the record parses");
    let schema = Schema::from_document(&schema_doc);
    check_document(&doc, &schema, &Known::default())
        .into_iter()
        .map(|v| v.refusal.message())
        .collect()
}

/// C1, both directions. The author who writes one node per value is doing the obvious
/// thing, and was refused for it.
#[test]
fn one_node_per_value_and_many_values_per_node_agree() {
    let per_node = refusals(
        r#"
capability "payment" {
    doing "take money once"
    owns-event "PaymentAuthorised"
    owns-event "PaymentDeclined"
}
"#,
    );
    let per_arg = refusals(
        r#"
capability "payment" {
    doing "take money once"
    owns-event "PaymentAuthorised" "PaymentDeclined"
}
"#,
    );
    assert!(per_node.is_empty(), "one node per value must check: {per_node:?}");
    assert!(per_arg.is_empty(), "many values per node must check: {per_arg:?}");
}

/// C3. Cardinality counts VALUES. Two values is two, written either way — and one value is
/// still one, so a genuine shortfall is still caught.
#[test]
fn cardinality_counts_values_and_still_refuses_a_real_shortfall() {
    let none = refusals(r#"capability "payment" { doing "take money once" }"#);
    assert!(
        none.iter().any(|m| m.contains("owns-event")),
        "a field with no values at all is still missing — counting values must not turn a \
         cardinality check into a formality: {none:?}"
    );
}

/// C1, the boolean case. `publishable #false` and `publishable=#false` are the same fact,
/// and the first was refused for *not saying* what it said on the line above.
#[test]
fn a_boolean_reads_as_a_child_node_and_as_a_property() {
    let as_child = refusals(
        r#"
reading "what-this-basket-owes" {
    answers "how much is owed"
    publishable #false
}
"#,
    );
    let as_property = refusals(
        r#"reading "what-this-basket-owes" answers="how much is owed" publishable=#false"#,
    );
    assert!(as_child.is_empty(), "the child-node form must check: {as_child:?}");
    assert!(as_property.is_empty(), "the property form must check: {as_property:?}");
}

/// C5. The prefix is the RECORD's, not the engine's. A repository free to use any other was
/// silently required to use this one, and told nothing when references stopped resolving.
#[test]
fn an_identity_prefix_comes_from_the_schema() {
    let doc = parse(SCHEMA).expect("parses");
    let schema = Schema::from_document(&doc);
    assert_eq!(schema.strip_prefix("capability", "CAP.payment"), "payment");
    assert_eq!(
        schema.strip_prefix("capability", "payment"),
        "payment",
        "an id already bare is unchanged"
    );
    assert_eq!(
        schema.strip_prefix("reading", "CAP.payment"),
        "CAP.payment",
        "a kind that declares no prefix strips nothing — guessing is what this replaced"
    );
}

/// A repository using its own prefix resolves, which is the case that could never work
/// while `CAP.` was compiled into four call sites.
#[test]
fn a_repository_may_choose_its_own_prefix() {
    let doc = parse(
        r##"
notional-architecture "NA.other" {
    schema {
        entity "capability" id-prefix="ABILITY::" {
            field "doing" each="1"
        }
    }
}
"##,
    )
    .expect("parses");
    let schema = Schema::from_document(&doc);
    assert_eq!(schema.strip_prefix("capability", "ABILITY::payment"), "payment");
}

/// C4. The gate and the schema stop contradicting each other.
///
/// `requires "every entity carries a trail"` sat beside a schema where eight of twenty-five
/// kinds declare one — so writing a trail on a capability was refused as an undeclared field
/// WHILE the gate called it required. Nothing could tell, because nothing in the engine reads
/// the `requires` block at all: every line in it is prose, and the ones that hold are held by
/// a `rule` further down.
///
/// The requirement was restated rather than dropped. Its reason — attribution distinguishes
/// human authorship from agent authorship — is sound and worth keeping; what was wrong was
/// the scope it claimed.
#[test]
fn the_gate_requires_no_field_a_kind_does_not_declare() {
    let (method, _) = Schema::method();
    let text = praxis_core::schema::METHOD;

    assert!(
        !text.contains(r#"requires "every entity carries a trail""#),
        "the gate must not require of every kind a field only eight declare"
    );

    // And what it says instead is true of what it names: a kind declaring a trail requires
    // one, so `entity-without-a-required-field` is what holds the restated sentence.
    let declaring: Vec<&str> = ["adr", "frame", "capability", "persona"]
        .into_iter()
        .filter(|kind| {
            method
                .entity(kind)
                .is_some_and(|e| e.fields.iter().any(|f| f.name == "trail"))
        })
        .collect();
    assert!(
        !declaring.is_empty(),
        "some kind declares a trail, or the restated requirement names nothing"
    );
}

/// C2, as amended by `ITER.260823.07`/`D1`.
///
/// Cut as *"a form mismatch names the expected form and the written one"*, which presupposed
/// the design D1 rejected: with both forms read there IS no mismatch to name. `ITER.260823.07`
/// carried the claim by a finding rather than reinterpreting it, and this is the half the fix
/// actually guarantees.
///
/// `missing` is the right word when the field is absent and the wrong word when it is present
/// in the other form. That was the whole injury: the refusal said *missing* while the value sat
/// on the line above, which sends an author to the wrong place and teaches them the checker is
/// unreliable.
#[test]
fn missing_is_said_when_and_only_when_the_field_is_absent() {
    // `answers` is declared and written as a property; `publishable` is declared and written
    // as a child node. Neither matches "the" form, because there is no such thing.
    let mixed = refusals(
        r#"
reading "what-this-basket-owes" answers="how much is owed" {
    publishable #false
}
"#,
    );
    assert!(
        mixed.iter().all(|m| !m.contains("missing")),
        "a field present in either form is never missing: {mixed:?}"
    );

    // And the word still works when it is true.
    let absent = refusals(r#"reading "what-this-basket-owes" answers="how much is owed""#);
    assert!(
        absent.iter().any(|m| m.contains("missing") && m.contains("publishable")),
        "a field absent in BOTH forms is missing, and says so: {absent:?}"
    );
}
