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
    /// The state this field applies in. A field whose purpose is discharged by a transition
    /// is not missing afterwards — `release.proposed-bump` matters while a version is being
    /// PLANNED, and once cut the index's `confirmed-bump` is what the maintainer decided.
    ///
    /// Declared in the schema and unenforced until `ITER.260822.08/AS2`, which is the shape
    /// `a-rule-has-a-witness` exists to catch one level down: a property nothing reads looks
    /// exactly like one that is honoured.
    pub when_state: Option<String>,
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
        Self::matches(&self.when_kind, kind)
    }

    /// Whether this field applies to an entity in `state`.
    pub fn applies_in(&self, state: Option<&str>) -> bool {
        Self::matches(&self.when_state, state)
    }

    /// A gate is satisfied when it is not declared, or when it matches what the record says.
    /// An undeclared value against a declared gate does NOT match: a record that does not say
    /// which state it is in has not earned the exemption the gate grants.
    fn matches(want: &Option<String>, have: Option<&str>) -> bool {
        match (want, have) {
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
    /// The prefix a reference to this kind carries, if it carries one — `CAP.` for a
    /// capability. Declared on the kind rather than known to the engine (`TS.260823.04`).
    pub id_prefix: Option<String>,
    /// What this kind IS, in the words the schema uses. A sentence written for a person,
    /// and the glossary a release publishes.
    pub is: Option<String>,
    /// The states this kind declares on itself. Read since `TS.260821.13`: `undeclared-state`
    /// works off the `state` field's `one-of=`, so a kind could declare `states=` and have
    /// nothing consult it — adding a state there changed no behaviour, which reads as the
    /// schema not being picked up (`WALK.260822.02/AS7`).
    pub states: Vec<String>,
    pub fields: Vec<FieldSpec>,
}

/// The declared shape of every entity kind the record knows.
#[derive(Debug, Clone, Default)]
pub struct Schema {
    entities: BTreeMap<String, EntitySpec>,
    rules: Vec<Rule>,
}

/// A rule as the record declares it: its name, and the record that demonstrates it
/// refusing. A witness written in the RECORD is evidence; one written in the engine would
/// be a second implementation of the same check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub name: String,
    pub witness: Option<String>,
    /// The world the witness assumes, for a rule the record alone cannot decide.
    ///
    /// Every rule before `TS.260821.03` was decidable from the record, so a witness was a
    /// record and nothing else. A rule about what the plugin SHIPS needs a tree to be about,
    /// and the honest place to declare that tree is beside the rule — written by whoever
    /// declares the rule, which is what keeps a witness evidence rather than a second
    /// implementation of the same check.
    pub given_shipped: Vec<String>,
}

/// The method's own vocabulary, carried by the engine.
///
/// `TS.260821.10`. This is DATA, parsed through the same reader as any record — not a shape
/// the engine knows. A4 (`ADR.260819.01`) forbids the engine encoding what the record
/// declares, because an engine that does can diverge from it and nobody can tell. Carrying
/// the record and knowing its contents are different things, and only the second is refused.
///
/// **One file, two carriers.** The plugin ships to six harnesses as a Markdown and scripts
/// tree with no binary in it; the engine is a separate `cargo install`. So neither carrier
/// alone reaches everybody:
///
/// - an agent with the plugin and no binary reads `praxis/method/DELIVERY-GRAPH.v1.kdl`
/// - a binary installed with no plugin has it embedded, below, from that same file
///
/// `include_str!` points at the shipped file rather than at a copy beside this module, so
/// there is nothing to keep in sync — the alternative is two files and a test that hopes.
/// It is embedded rather than resolved at runtime because a path would move AK2 rather than
/// close it: the binary would need the plugin tree beside it, and the two versions would
/// skew independently.
pub const METHOD: &str = include_str!("../../../praxis/method/DELIVERY-GRAPH.v1.kdl");

/// What a repository's own schema did when composed onto the method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Extension {
    /// A kind or rule the method does not name. Added.
    Added(String),
    /// A kind or rule the method already declares. REFUSED — a repository extends the
    /// method and never redefines it, which is `may bind, and may never declare` one level
    /// up. A method a repository can weaken locally reports whatever it wanted to hear.
    Redefines(String),
}

impl Schema {
    /// The method the engine carries, and its identity.
    ///
    /// Panics only if the embedded record does not parse, which a test catches before any
    /// binary ships: an engine whose own method is malformed can check nothing at all, and
    /// limping on with an empty vocabulary would report every record as fine.
    pub fn method() -> (Self, String) {
        let doc: KdlDocument = METHOD.parse().expect("the embedded method parses");
        let version = doc
            .nodes()
            .iter()
            .find(|n| n.name().value() == "method")
            .and_then(|n| {
                let id = n.entries().first()?.value().as_string()?.to_owned();
                let v = n.children()?.get("version")?.entries().first()?.value().as_string()?;
                Some(format!("{id}@v{v}"))
            })
            .unwrap_or_default();
        (Self::from_document(&doc), version)
    }

    /// Compose a repository's own schema onto the method.
    ///
    /// Adds what the method does not name; refuses what it does. The returned list says
    /// which happened for each, because a composition that silently kept one of two
    /// definitions is the drift this whole slice exists to prevent — and before it, the
    /// engine took the LAST schema it found, so a project declaring its own would have
    /// replaced the method rather than extended it.
    pub fn extend(&mut self, local: Self) -> Vec<Extension> {
        let mut out = Vec::new();
        for (name, spec) in local.entities {
            if self.entities.contains_key(&name) {
                out.push(Extension::Redefines(format!("entity {name}")));
            } else {
                out.push(Extension::Added(format!("entity {name}")));
                self.entities.insert(name, spec);
            }
        }
        for rule in local.rules {
            if self.rules.iter().any(|r| r.name == rule.name) {
                out.push(Extension::Redefines(format!("rule {}", rule.name)));
            } else {
                out.push(Extension::Added(format!("rule {}", rule.name)));
                self.rules.push(rule);
            }
        }
        out
    }

    /// Read the schema out of a notional-architecture document.
    ///
    /// Returns an empty schema rather than an error when the document carries no
    /// `schema` block: a document that declares nothing is not malformed, it is
    /// simply not the architecture.
    pub fn from_document(doc: &KdlDocument) -> Self {
        let mut entities = BTreeMap::new();
        let mut rules = Vec::new();
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
                entities.insert(
                    name,
                    EntitySpec {
                        id_prefix: prop(entity, "id-prefix"),
                        is: prop(entity, "is"),
                        states: prop(entity, "states")
                            .map(|s| s.split_whitespace().map(str::to_owned).collect())
                            .unwrap_or_default(),
                        fields: fields_of(entity),
                    },
                );
            }
            for rule in body.nodes().iter().filter(|n| n.name().value() == "rule") {
                if let Some(name) = string_arg(rule) {
                    rules.push(Rule {
                        name,
                        witness: prop(rule, "witness"),
                        given_shipped: rule
                            .entries()
                            .iter()
                            .filter(|e| e.name().map(|n| n.value()) == Some("given-shipped"))
                            .filter_map(|e| e.value().as_string().map(str::to_owned))
                            .collect(),
                    });
                }
            }
        }
        Self { entities, rules }
    }

    /// Strip the prefix a kind declares for its references, if it declares one.
    ///
    /// `TS.260823.04`. The engine used to do `trim_start_matches("CAP.")` in four call
    /// sites, which is A4 exactly: a naming convention only the engine knew, in a record
    /// free to use any other and never told why nothing resolved. The fact now lives where
    /// every other fact about a kind lives, and `praxis schema` prints it.
    pub fn strip_prefix<'a>(&self, kind: &str, id: &'a str) -> &'a str {
        match self.entities.get(kind).and_then(|e| e.id_prefix.as_deref()) {
            Some(prefix) => id.strip_prefix(prefix).unwrap_or(id),
            None => id,
        }
    }

    /// The prefix a capability reference carries, as the record declares it.
    #[must_use]
    pub fn capability_prefix(&self) -> Option<String> {
        self.entities.get("capability").and_then(|e| e.id_prefix.clone())
    }

    /// Whether the record declares this rule. A rule the record does not name is a rule
    /// the engine may not apply — the same reading of A4 that put the admission
    /// conditions on the architecture rather than in the gate.
    pub fn declares_rule(&self, name: &str) -> bool {
        self.rules.iter().any(|r| r.name == name)
    }

    pub fn rules(&self) -> impl Iterator<Item = &Rule> {
        self.rules.iter()
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
                when_state: prop(field, "when-state"),
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

/// Every value of a repeated property, in two spellings the record uses interchangeably.
///
/// `one-of="a" "b"` arrives as one named entry followed by positional ones, so a positional
/// tail belongs to the key before it. `needed-by="a" needed-by="b"` repeats the key. Both
/// are written in this record and reading only the first spelling silently drops half of
/// what a node says — which is how `needed-by` named two personas and answered for one.
/// How many values a property carries, whatever their type.
///
/// `all_props` collects STRINGS, because every caller wanted strings. Counting with it made
/// `publishable=#false` uncountable and reported the field as missing while it sat on the
/// line — the same trap this slice removes, re-created inside the fix for it
/// (`TS.260823.04`).
#[must_use]
pub fn count_props(node: &KdlNode, key: &str) -> usize {
    let mut count = 0;
    let mut collecting = false;
    for entry in node.entries() {
        match entry.name() {
            Some(name) => collecting = name.value() == key,
            None if !collecting => continue,
            None => {}
        }
        if collecting {
            count += 1;
        }
    }
    count
}

pub fn all_props(node: &KdlNode, key: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut collecting = false;
    for entry in node.entries() {
        match entry.name() {
            Some(name) => collecting = name.value() == key,
            // A positional entry belongs to the last named one.
            None if !collecting => continue,
            None => {}
        }
        if collecting
            && let Some(value) = entry.value().as_string()
        {
            out.push(value.to_owned());
        }
    }
    out
}
