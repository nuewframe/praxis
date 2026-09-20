//! `TS.260821.11`: answer a question about the record without knowing what the record is about.
//!
//! Cut from `praxis/.usage/cli-invocations.md`. Eight questions in one session were asked with
//! `grep` because the CLI could not answer them, and every one traversed a graph the engine
//! already held in memory.
//!
//! The shape is forced by A4, not chosen. `praxis unbound-iterations` would encode a kind and
//! a field the record declares — the engine enforcing a vocabulary no record handed it — and
//! would answer nothing in a repository whose kinds are `cohort` and `experiment`. So the
//! question names a kind, the kind comes from the schema, and this module never learns what
//! any of it MEANS.

use kdl::{KdlDocument, KdlNode};

use crate::schema::{Schema, string_arg, string_args};

/// What was asked. Every part of it is checked against the schema before anything is read.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Question {
    pub kind: String,
    /// Field, value. All must hold — a row matches when every predicate does.
    pub where_: Vec<(String, String)>,
    /// Fields to project. Empty shows the identity alone: the usage log says no projection
    /// was ever consumed whole.
    pub show: Vec<String>,
    /// `kind.field` — keep only records nothing of that kind points at through that field.
    pub unreferenced_by: Option<(String, String)>,
}

/// What came back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Answer {
    /// Column headings and rows, in the order the record holds them.
    Rows { columns: Vec<String>, rows: Vec<Vec<String>> },
    /// The question named something the schema does not declare. Refused rather than
    /// answered empty: a filter on a field that does not exist matches nothing, and looks
    /// exactly like a filter that matched nothing.
    Refused(String),
}

impl Answer {
    pub fn count(&self) -> usize {
        match self {
            Self::Rows { rows, .. } => rows.len(),
            Self::Refused(_) => 0,
        }
    }
}

/// Ask the record a question.
///
/// Reads nothing and writes nothing. An `ask` that changed the record would be a command
/// wearing a question's clothes.
pub fn ask(docs: &[KdlDocument], schema: &Schema, question: &Question) -> Answer {
    let Some(spec) = schema.entity(&question.kind) else {
        let mut known: Vec<&str> = schema.kinds().collect();
        known.sort_unstable();
        return Answer::Refused(format!(
            "`{}` is not a kind this record declares. It holds: {}",
            question.kind,
            known.join(" · ")
        ));
    };

    // Every named field, before anything is read. A question that silently matches nothing
    // is worse than one that refuses: the empty answer is indistinguishable from the truth.
    let declared: Vec<&str> = spec.fields.iter().map(|f| f.name.as_str()).collect();
    for name in question.where_.iter().map(|(f, _)| f).chain(question.show.iter()) {
        if !declared.contains(&name.as_str()) {
            let mut fields = declared.clone();
            fields.sort_unstable();
            return Answer::Refused(format!(
                "`{}` declares no field `{name}`. It declares: {}",
                question.kind,
                if fields.is_empty() { "nothing — it is shapeless".to_owned() } else { fields.join(" · ") }
            ));
        }
    }
    if let Some((kind, field)) = &question.unreferenced_by
        && schema.entity(kind).is_none_or(|s| !s.fields.iter().any(|f| &f.name == field))
    {
        return Answer::Refused(format!(
            "`{kind}.{field}` is not an edge this record declares, so `unreferenced-by` has \
             nothing to follow. An edge the schema does not name is the engine guessing what \
             relates to what"
        ));
    }

    // Everything anything points at, through the named edge.
    let pointed_at: Vec<String> = question
        .unreferenced_by
        .as_ref()
        .map(|(kind, field)| {
            docs.iter()
                .flat_map(KdlDocument::nodes)
                .filter(|n| n.name().value() == kind)
                .flat_map(|n| children_named(n, field))
                .flat_map(string_args)
                .collect()
        })
        .unwrap_or_default();

    let mut rows = Vec::new();
    for node in docs.iter().flat_map(KdlDocument::nodes) {
        if node.name().value() != question.kind {
            continue;
        }
        let id = string_arg(node).unwrap_or_default();
        if !question.where_.iter().all(|(f, v)| values(node, f).iter().any(|found| found == v)) {
            continue;
        }
        if question.unreferenced_by.is_some() && pointed_at.iter().any(|p| p == &id) {
            continue;
        }
        let mut row = vec![id];
        for field in &question.show {
            row.push(values(node, field).join(" · "));
        }
        rows.push(row);
    }

    // A kind that is declared, appears nowhere at the root, and is nested somewhere. Zero
    // rows would be indistinguishable from "none match", which is the exact failure refusing
    // an undeclared field avoids — an empty answer that looks like the truth. Nested
    // traversal is excluded by this slice; answering 0 as though it were the count is not.
    let at_root = docs
        .iter()
        .flat_map(KdlDocument::nodes)
        .any(|n| n.name().value() == question.kind);
    if !at_root && nested_somewhere(docs, &question.kind) {
        return Answer::Refused(format!(
            "`{}` appears only nested inside another record, never at the root. This answers \
             over root records, so 0 here would mean `cannot see them` while reading as `none`",
            question.kind
        ));
    }

    let mut columns = vec![question.kind.clone()];
    columns.extend(question.show.iter().cloned());
    Answer::Rows { columns, rows }
}

/// Whether this kind appears as a child of something, anywhere.
fn nested_somewhere(docs: &[KdlDocument], kind: &str) -> bool {
    fn within(node: &KdlNode, kind: &str) -> bool {
        node.children().is_some_and(|body| {
            body.nodes().iter().any(|c| c.name().value() == kind || within(c, kind))
        })
    }
    docs.iter().flat_map(KdlDocument::nodes).any(|n| within(n, kind))
}

/// Every value of every occurrence of a field. `owns-event "A" "B"` is one node carrying two,
/// and reading the first of each is `ITER.260821.12/AB1`.
fn values(node: &KdlNode, field: &str) -> Vec<String> {
    children_named(node, field).into_iter().flat_map(string_args).collect()
}

fn children_named<'a>(node: &'a KdlNode, name: &str) -> Vec<&'a KdlNode> {
    node.children()
        .map(|body| body.nodes().iter().filter(|n| n.name().value() == name).collect())
        .unwrap_or_default()
}
