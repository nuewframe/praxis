//! The schema, read from the record rather than encoded here.
//!
//! ADR.260819.01/A4: the engine may not encode what the record declares. Every field
//! name, cardinality and reference below is loaded at runtime from
//! `NA.260820.01/schema`, so extending the vocabulary is an amendment to the
//! architecture rather than an edit to this file.

use std::collections::BTreeMap;

use kdl::{KdlDocument, KdlNode};

/// How many of a field an entity must carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cardinality {
    /// Exactly one.
    One,
    /// At least one.
    OneOrMore,
    /// None or one.
    Optional,
    /// Any number, including none.
    Any,
}

impl Cardinality {
    fn parse(raw: &str) -> Option<Self> {
        match raw {
            "1" => Some(Self::One),
            "1..n" => Some(Self::OneOrMore),
            "0..1" => Some(Self::Optional),
            "0..n" => Some(Self::Any),
            _ => None,
        }
    }

    fn satisfied_by(self, count: usize) -> bool {
        match self {
            Self::One => count == 1,
            Self::OneOrMore => count >= 1,
            Self::Optional => count <= 1,
            Self::Any => true,
        }
    }

    pub fn describe(self) -> &'static str {
        match self {
            Self::One => "exactly one",
            Self::OneOrMore => "at least one",
            Self::Optional => "at most one",
            Self::Any => "any number",
        }
    }
}

/// One field an entity must declare.
#[derive(Debug, Clone)]
pub struct FieldSpec {
    pub name: String,
    pub each: Cardinality,
    /// Only required when the entity's `kind` matches.
    pub when_kind: Option<String>,
    /// The entity kind this field's value must name.
    pub references: Option<String>,
    /// No two records of this kind may claim the same value for this field.
    pub unique_in: Option<String>,
    /// Every value of this field should be claimed by `<kind>.<field>` somewhere in the
    /// record. Unclaimed values are reported, never refused — absence of a claim is a
    /// gap in coverage, not a malformed fact.
    pub claimed_by: Option<String>,
    /// The entity kind this field CONTAINS, when the child is an entity in its own
    /// right rather than a value. `references` points across the graph by id; `holds`
    /// nests. Without the distinction a contained entity is indistinguishable from a
    /// plain field, which is how `phase` went missing.
    pub holds: Option<String>,
    /// A closed vocabulary for this field's value.
    pub one_of: Vec<String>,
    /// Why the schema requires it, verbatim, for the diagnostic.
    pub because: Option<String>,
}

impl FieldSpec {
    /// Whether this field applies to an entity declaring `kind`.
    pub fn applies_to(&self, kind: Option<&str>) -> bool {
        match (&self.when_kind, kind) {
            (None, _) => true,
            (Some(want), Some(have)) => want == have,
            (Some(_), None) => false,
        }
    }

    pub fn satisfied_by(&self, count: usize) -> bool {
        self.each.satisfied_by(count)
    }

    /// Whether the schema permits this field to be absent.
    pub fn optional(&self) -> bool {
        matches!(self.each, Cardinality::Optional | Cardinality::Any)
    }
}

/// What one entity kind must look like.
#[derive(Debug, Clone, Default)]
pub struct EntitySpec {
    pub fields: Vec<FieldSpec>,
}

/// The declared shape of every entity kind the record knows.
#[derive(Debug, Clone, Default)]
pub struct Schema {
    entities: BTreeMap<String, EntitySpec>,
}

impl Schema {
    /// Read the schema out of a notional-architecture document.
    ///
    /// Returns an empty schema rather than an error when the document carries no
    /// `schema` block: a document that declares nothing is not malformed, it is
    /// simply not the architecture.
    pub fn from_document(doc: &KdlDocument) -> Self {
        let mut entities = BTreeMap::new();
        for node in doc.nodes() {
            let Some(children) = node.children() else {
                continue;
            };
            let Some(schema) = children.get("schema") else {
                continue;
            };
            let Some(body) = schema.children() else {
                continue;
            };
            for entity in body.nodes().iter().filter(|n| n.name().value() == "entity") {
                let Some(name) = string_arg(entity) else {
                    continue;
                };
                entities.insert(name, EntitySpec { fields: fields_of(entity) });
            }
        }
        Self { entities }
    }

    pub fn entity(&self, kind: &str) -> Option<&EntitySpec> {
        self.entities.get(kind)
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    pub fn kinds(&self) -> impl Iterator<Item = &str> {
        self.entities.keys().map(String::as_str)
    }
}

fn fields_of(entity: &KdlNode) -> Vec<FieldSpec> {
    let Some(body) = entity.children() else {
        return Vec::new();
    };
    body.nodes()
        .iter()
        .filter(|n| n.name().value() == "field")
        .filter_map(|field| {
            let name = string_arg(field)?;
            let each = prop(field, "each").as_deref()
                .and_then(Cardinality::parse)
                .unwrap_or(Cardinality::One);
            Some(FieldSpec {
                name,
                each,
                when_kind: prop(field, "when-kind"),
                references: prop(field, "references"),
                holds: prop(field, "holds"),
                unique_in: prop(field, "unique-in"),
                claimed_by: prop(field, "claimed-by"),
                one_of: all_props(field, "one-of"),
                because: prop(field, "because"),
            })
        })
        .collect()
}

/// Every positional argument of a node, as strings. `owns-event "A" "B"` carries two.
pub fn string_args(node: &KdlNode) -> Vec<String> {
    node.entries()
        .iter()
        .filter(|e| e.name().is_none())
        .filter_map(|e| e.value().as_string())
        .map(str::to_owned)
        .collect()
}

/// A node's first positional argument, when it is a string.
pub fn string_arg(node: &KdlNode) -> Option<String> {
    node.entries()
        .iter()
        .find(|e| e.name().is_none())
        .and_then(|e| e.value().as_string())
        .map(str::to_owned)
}

/// A named property, when it is a string.
pub fn prop(node: &KdlNode, key: &str) -> Option<String> {
    node.entries()
        .iter()
        .find(|e| e.name().is_some_and(|n| n.value() == key))
        .and_then(|e| e.value().as_string())
        .map(str::to_owned)
}

/// Every value of a repeated property. KDL allows `one-of="a" "b"`, which arrives as
/// one named entry followed by positional ones, so the positional tail is collected too.
fn all_props(node: &KdlNode, key: &str) -> Vec<String> {
    let entries = node.entries();
    let Some(start) = entries
        .iter()
        .position(|e| e.name().is_some_and(|n| n.value() == key))
    else {
        return Vec::new();
    };
    let mut out = Vec::new();
    if let Some(first) = entries[start].value().as_string() {
        out.push(first.to_owned());
    }
    for entry in &entries[start + 1..] {
        if entry.name().is_some() {
            break;
        }
        if let Some(value) = entry.value().as_string() {
            out.push(value.to_owned());
        }
    }
    out
}
