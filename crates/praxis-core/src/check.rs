//! The shape check. `TS.260820.01`: a slice is refused at write time when it is missing
//! a field the schema requires, and the refusal names the field.
//!
//! This is a validator over a parsed document, never a codec-level refusal
//! (ITER.260820.01, "where the shape check lives"): a document-model codec can only
//! fail on syntax, and *which field is missing* is semantic.

use kdl::{KdlDocument, KdlNode};
use miette::SourceSpan;

use crate::schema::{Schema, prop, string_arg};

/// Why a node was refused. One variant per rule the schema declares.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// A required field is absent.
    MissingField { field: String, because: Option<String> },
    /// A field is present the wrong number of times.
    WrongCardinality { field: String, found: usize, wanted: &'static str },
    /// A field's value is outside its closed vocabulary.
    NotInVocabulary { field: String, found: String, allowed: Vec<String> },
    /// A field names an entity the record does not hold.
    DanglingReference { field: String, names: String, kind: String },
}

impl Refusal {
    /// The field this refusal is about. C2: the diagnostic names the field, never
    /// merely that the document is invalid.
    pub fn field(&self) -> &str {
        match self {
            Self::MissingField { field, .. }
            | Self::WrongCardinality { field, .. }
            | Self::NotInVocabulary { field, .. }
            | Self::DanglingReference { field, .. } => field,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::MissingField { field, because } => match because {
                Some(why) => format!("missing `{field}` — {why}"),
                None => format!("missing `{field}`"),
            },
            Self::WrongCardinality { field, found, wanted } => {
                format!("`{field}` appears {found} times; the schema requires {wanted}")
            }
            Self::NotInVocabulary { field, found, allowed } => {
                format!("`{field}` is {found:?}; the schema allows {}", allowed.join(" · "))
            }
            Self::DanglingReference { field, names, kind } => {
                format!("`{field}` names {names:?}, which is no {kind} the record holds")
            }
        }
    }
}

/// One refusal, located.
#[derive(Debug, Clone)]
pub struct Violation {
    pub entity_kind: String,
    pub entity_id: Option<String>,
    pub refusal: Refusal,
    pub span: SourceSpan,
}

/// What the record already holds, so a reference can be checked against it.
#[derive(Debug, Default, Clone)]
pub struct Known {
    ids: Vec<(String, String)>,
}

impl Known {
    /// Record that an entity of `kind` exists under `id`.
    pub fn insert(&mut self, kind: impl Into<String>, id: impl Into<String>) {
        self.ids.push((kind.into(), id.into()));
    }

    fn holds(&self, kind: &str, id: &str) -> bool {
        self.ids.iter().any(|(k, i)| k == kind && i == id)
    }

    /// Whether anything at all of this kind is known. A reference cannot be judged
    /// against an empty index — absence of the index is not absence of the target.
    fn knows_kind(&self, kind: &str) -> bool {
        self.ids.iter().any(|(k, _)| k == kind)
    }
}

/// Check one entity node against the schema.
pub fn check_node(node: &KdlNode, schema: &Schema, known: &Known) -> Vec<Violation> {
    let kind = node.name().value().to_owned();
    let Some(spec) = schema.entity(&kind) else {
        return Vec::new();
    };
    let id = string_arg(node);
    let declared_kind = child_arg(node, "kind");

    let mut out = Vec::new();
    for field in &spec.fields {
        if !field.applies_to(declared_kind.as_deref()) {
            continue;
        }
        let found = count_of(node, &field.name);
        if found == 0 {
            out.push(Violation {
                entity_kind: kind.clone(),
                entity_id: id.clone(),
                refusal: Refusal::MissingField {
                    field: field.name.clone(),
                    because: field.because.clone(),
                },
                span: node.span(),
            });
            continue;
        }
        if !field.satisfied_by(found) {
            out.push(Violation {
                entity_kind: kind.clone(),
                entity_id: id.clone(),
                refusal: Refusal::WrongCardinality {
                    field: field.name.clone(),
                    found,
                    wanted: field.each.describe(),
                },
                span: field_span(node, &field.name).unwrap_or(node.span()),
            });
        }
        let Some(value) = child_arg(node, &field.name) else {
            continue;
        };
        if !field.one_of.is_empty() && !field.one_of.contains(&value) {
            out.push(Violation {
                entity_kind: kind.clone(),
                entity_id: id.clone(),
                refusal: Refusal::NotInVocabulary {
                    field: field.name.clone(),
                    found: value.clone(),
                    allowed: field.one_of.clone(),
                },
                span: field_span(node, &field.name).unwrap_or(node.span()),
            });
            continue;
        }
        if let Some(target) = &field.references {
            let names = value.trim_start_matches("CAP.").to_owned();
            if known.knows_kind(target) && !known.holds(target, &names) && !known.holds(target, &value)
            {
                out.push(Violation {
                    entity_kind: kind.clone(),
                    entity_id: id.clone(),
                    refusal: Refusal::DanglingReference {
                        field: field.name.clone(),
                        names: value,
                        kind: target.clone(),
                    },
                    span: field_span(node, &field.name).unwrap_or(node.span()),
                });
            }
        }
    }
    out
}

/// Check every root node in a document.
pub fn check_document(doc: &KdlDocument, schema: &Schema, known: &Known) -> Vec<Violation> {
    doc.nodes()
        .iter()
        .flat_map(|node| check_node(node, schema, known))
        .collect()
}

fn body(node: &KdlNode) -> Option<&KdlDocument> {
    node.children()
}

fn count_of(node: &KdlNode, field: &str) -> usize {
    body(node).map_or(0, |b| {
        b.nodes().iter().filter(|n| n.name().value() == field).count()
    })
}

fn child_arg(node: &KdlNode, field: &str) -> Option<String> {
    body(node)?
        .nodes()
        .iter()
        .find(|n| n.name().value() == field)
        .and_then(string_arg)
}

fn field_span(node: &KdlNode, field: &str) -> Option<SourceSpan> {
    body(node)?
        .nodes()
        .iter()
        .find(|n| n.name().value() == field)
        .map(|n| n.span())
}

/// Index every entity a document declares, so references can be checked later.
pub fn index(doc: &KdlDocument, schema: &Schema, known: &mut Known) {
    for node in doc.nodes() {
        let kind = node.name().value();
        if schema.entity(kind).is_none() && !schema.kinds().any(|k| k == kind) {
            continue;
        }
        if let Some(id) = string_arg(node) {
            known.insert(kind, id);
        }
    }
}

/// Index a node whose kind the schema may not declare — used for capabilities, which
/// carry their identity as the node's own argument.
pub fn index_all(doc: &KdlDocument, known: &mut Known) {
    for node in doc.nodes() {
        if let Some(id) = string_arg(node) {
            known.insert(node.name().value(), id);
        }
    }
}

/// Ignore the field-level detail and answer the one question a gate asks.
pub fn refused(violations: &[Violation]) -> bool {
    !violations.is_empty()
}

/// Re-exported so the shell can render a violation without reaching into `prop`.
pub fn note(node: &KdlNode, key: &str) -> Option<String> {
    prop(node, key)
}
