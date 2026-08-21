//! The shape check. `TS.260820.01`: a slice is refused at write time when it is missing
//! a field the schema requires, and the refusal names the field.
//!
//! This is a validator over a parsed document, never a codec-level refusal
//! (ITER.260820.01, "where the shape check lives"): a document-model codec can only
//! fail on syntax, and *which field is missing* is semantic.

use kdl::{KdlDocument, KdlNode};
use miette::SourceSpan;

use crate::schema::{Schema, prop, string_arg, string_args};

/// Whether a finding stops the work or merely tells the reader.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Fails closed.
    Refuse,
    /// Named, but admitted. Silence and acceptance must not look the same.
    Report,
}

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
    /// A node's kind is absent from the schema, so nothing can say what it must carry.
    UndeclaredKind { kind: String, within: Option<String> },
    /// A kind is declared but carries no field declarations, so it refuses nothing.
    ShapelessKind { kind: String },
    /// Two records of one kind claim the same value for a field that must be unique.
    ContestedValue { field: String, value: String, kind: String, other: String },
    /// A value nothing claims. Reported: a coverage gap, not a malformed fact.
    UnclaimedValue { field: String, value: String, wanted_by: String },
    /// An iteration evidencing a layer its slice never declared. The slice sets the
    /// granularity; evidence outside it is evidence for something nobody asked about.
    UndeclaredLayer { layer: String, slice: String, declared: Vec<String> },
    /// A closed iteration carrying a claim that is neither met nor carried by a finding.
    SilentDrop { claim: String, state: String },
    /// A claim frozen into an iteration that its slice no longer declares.
    DroppedClaim { claim: String, slice: String },
    /// A layer the slice declared that the iteration has not evidenced. Reported: it must
    /// be visible as unevidenced rather than absent, which is C2.
    UnevidencedLayer { layer: String, slice: String },
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
            Self::UndeclaredKind { kind, .. } | Self::ShapelessKind { kind } => kind,
            Self::ContestedValue { field, .. } | Self::UnclaimedValue { field, .. } => field,
            Self::UndeclaredLayer { .. } | Self::UnevidencedLayer { .. } => "layer",
            Self::SilentDrop { claim, .. } | Self::DroppedClaim { claim, .. } => claim,
        }
    }

    /// Whether this stops the work.
    pub fn severity(&self) -> Severity {
        match self {
            Self::ShapelessKind { .. }
            | Self::UnclaimedValue { .. }
            | Self::UnevidencedLayer { .. } => Severity::Report,
            _ => Severity::Refuse,
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
            Self::UndeclaredKind { kind, within } => match within {
                Some(parent) => format!(
                    "`{kind}` inside `{parent}` is neither a field {parent} declares nor a kind the \
                     schema knows — declare it on the architecture's schema block"
                ),
                None => format!(
                    "`{kind}` is not a kind the schema declares — declare it on the architecture's \
                     schema block, which is an amendment to the record and not a change to the engine"
                ),
            },
            Self::ShapelessKind { kind } => format!(
                "`{kind}` is declared with no fields, so nothing about its records can be refused"
            ),
            Self::ContestedValue { field, value, kind, other } => format!(
                "`{field}` claims {value:?}, which {other:?} also claims — one {kind} owns it, or the \
                 boundary between them is drawn wrong"
            ),
            Self::UnclaimedValue { field, value, wanted_by } => format!(
                "`{field}` {value:?} is claimed by no {wanted_by} — it belongs to nothing, which is \
                 a gap in the model rather than a malformed record"
            ),
            Self::UndeclaredLayer { layer, slice, declared } => format!(
                "evidences layer {layer:?}, which {slice} never declared — it declares {}. The \
                 slice sets the granularity, and evidence outside it is evidence for something \
                 nobody asked about",
                if declared.is_empty() { "none".to_owned() } else { declared.join(" · ") }
            ),
            Self::SilentDrop { claim, state } => format!(
                "closed with {claim} {} and no finding carrying it — record a finding that names \
                 the claim, or settle it. A shortfall nobody wrote down and a claim that was met \
                 look identical afterwards",
                if state.is_empty() { "unsettled".to_owned() } else { format!("{state:?}") }
            ),
            Self::DroppedClaim { claim, slice } => format!(
                "carries {claim}, frozen at open, and {slice} no longer declares it — deleting a \
                 claim is not a way to settle it"
            ),
            Self::UnevidencedLayer { layer, slice } => format!(
                "layer {layer:?} is declared by {slice} and this iteration evidences nothing for \
                 it — carried as unevidenced rather than dropped, because a layer that vanishes \
                 from the accounting was never reached and never refused"
            ),
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

impl Violation {
    pub fn severity(&self) -> Severity {
        self.refusal.severity()
    }
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
        if found == 0 && field.optional() {
            continue;
        }
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

    // Contained nodes. Only where the parent declares fields: an entity that declares
    // none makes no claim about its children, and refusing them would be the engine
    // inventing a shape the record never stated.
    if !spec.fields.is_empty()
        && let Some(body) = node.children()
    {
        for child in body.nodes() {
            let child_kind = child.name().value();
            // A field that HOLDS an entity is checked as one. The kind it names must be
            // declared, or the container is nesting something nothing describes.
            if let Some(field) = spec.fields.iter().find(|f| f.name == child_kind)
                && let Some(held) = &field.holds
            {
                if schema.entity(held).is_none() {
                    out.push(Violation {
                        entity_kind: held.clone(),
                        entity_id: string_arg(child),
                        refusal: Refusal::UndeclaredKind {
                            kind: held.clone(),
                            within: Some(kind.clone()),
                        },
                        span: child.span(),
                    });
                } else {
                    out.extend(check_node(child, schema, known));
                }
                continue;
            }
            if spec.fields.iter().any(|f| f.name == child_kind) {
                continue;
            }
            if schema.entity(child_kind).is_some() {
                out.extend(check_node(child, schema, known));
                continue;
            }
            out.push(Violation {
                entity_kind: child_kind.to_owned(),
                entity_id: string_arg(child),
                refusal: Refusal::UndeclaredKind {
                    kind: child_kind.to_owned(),
                    within: Some(kind.clone()),
                },
                span: child.span(),
            });
        }
    }
    out
}

/// Check every root node in a document.
pub fn check_document(doc: &KdlDocument, schema: &Schema, known: &Known) -> Vec<Violation> {
    // A schema that declares nothing describes nothing. Refusing every kind here would
    // make the engine's own emptiness look like a verdict about the record, and it would
    // put a rule in the engine that no record states (ADR.260819.01/A4).
    if schema.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for node in doc.nodes() {
        let kind = node.name().value();
        match schema.entity(kind) {
            None => out.push(Violation {
                entity_kind: kind.to_owned(),
                entity_id: string_arg(node),
                refusal: Refusal::UndeclaredKind { kind: kind.to_owned(), within: None },
                span: node.span(),
            }),
            Some(spec) if spec.fields.is_empty() => out.push(Violation {
                entity_kind: kind.to_owned(),
                entity_id: string_arg(node),
                refusal: Refusal::ShapelessKind { kind: kind.to_owned() },
                span: node.span(),
            }),
            Some(_) => out.extend(check_node(node, schema, known)),
        }
    }
    out
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

/// Rules that can only be decided by looking at the whole record at once: whether two
/// entities contest one value, and whether a value nothing claims exists.
pub fn check_corpus(docs: &[KdlDocument], schema: &Schema) -> Vec<Violation> {
    let mut out = Vec::new();
    let mut claimed: Vec<(String, String, String)> = Vec::new(); // kind.field, value, owner

    // First pass: everything anyone claims, and any contest over it.
    for doc in docs {
        for node in doc.nodes() {
            let kind = node.name().value();
            let Some(spec) = schema.entity(kind) else { continue };
            let owner = string_arg(node).unwrap_or_else(|| kind.to_owned());
            for field in &spec.fields {
                let Some(unique_in) = &field.unique_in else { continue };
                for child in children_named(node, &field.name) {
                    for value in string_args(child) {
                        let key = format!("{unique_in}.{}", field.name);
                        if let Some((_, _, other)) =
                            claimed.iter().find(|(k, v, o)| k == &key && v == &value && o != &owner)
                        {
                            out.push(Violation {
                                entity_kind: kind.to_owned(),
                                entity_id: Some(owner.clone()),
                                refusal: Refusal::ContestedValue {
                                    field: field.name.clone(),
                                    value: value.clone(),
                                    kind: unique_in.clone(),
                                    other: other.clone(),
                                },
                                span: child.span(),
                            });
                        }
                        claimed.push((key, value, owner.clone()));
                    }
                }
            }
        }
    }

    // `evidence-names-its-layer`, if the record declares it. An iteration's layers are
    // bounded by the layers its SLICE declared: the slice set the granularity when it was
    // cut, and that is what makes "how much of this was reached" answerable at all.
    if schema.declares_rule("evidence-names-its-layer") {
        out.extend(check_layers(docs));
    }

    // `no-silent-drop` and `claim-dropped-from-the-slice`, if the record declares them.
    // The close command refuses at the moment; these refuse forever, over a record nobody
    // is currently asking about.
    if schema.declares_rule("no-silent-drop") || schema.declares_rule("claim-dropped-from-the-slice")
    {
        out.extend(check_claims(
            docs,
            schema.declares_rule("no-silent-drop"),
            schema.declares_rule("claim-dropped-from-the-slice"),
        ));
    }

    // Second pass: values that wanted a claimant and found none.
    for doc in docs {
        for node in doc.nodes() {
            let kind = node.name().value();
            let Some(spec) = schema.entity(kind) else { continue };
            for field in &spec.fields {
                let Some(target) = &field.claimed_by else { continue };
                for child in children_named(node, &field.name) {
                    let Some(value) = string_arg(child) else { continue };
                    let wanted = target.replace('.', ".");
                    if !claimed.iter().any(|(k, v, _)| k == &wanted && v == &value) {
                        out.push(Violation {
                            entity_kind: kind.to_owned(),
                            entity_id: string_arg(node),
                            refusal: Refusal::UnclaimedValue {
                                field: field.name.clone(),
                                value,
                                wanted_by: target.split('.').next().unwrap_or(target).to_owned(),
                            },
                            span: child.span(),
                        });
                    }
                }
            }
        }
    }
    out
}

/// `TS.260820.07`. A closed iteration accounts for every claim it holds, and no claim it
/// froze at open has since vanished from its slice.
fn check_claims(docs: &[KdlDocument], silent_drop: bool, dropped: bool) -> Vec<Violation> {
    let mut declared: Vec<(String, Vec<String>)> = Vec::new();
    for doc in docs {
        for node in doc.nodes().iter().filter(|n| n.name().value() == "thin-slice") {
            if let Some(id) = string_arg(node) {
                let claims =
                    children_named(node, "claim").iter().filter_map(|c| string_arg(c)).collect();
                declared.push((id, claims));
            }
        }
    }

    let mut out = Vec::new();
    for doc in docs {
        for node in doc.nodes().iter().filter(|n| n.name().value() == "iteration") {
            let id = string_arg(node);
            let on_slice = children_named(node, "on-slice")
                .first()
                .and_then(|n| string_arg(n))
                .unwrap_or_default();
            let closed = children_named(node, "state")
                .first()
                .and_then(|n| string_arg(n))
                .is_some_and(|s| s == "closed");
            let findings: Vec<Option<String>> =
                children_named(node, "finding").iter().map(|f| prop(f, "carries")).collect();

            for claim in children_named(node, "claim") {
                let Some(claim_id) = string_arg(claim) else { continue };
                let from = prop(claim, "from-slice").unwrap_or_default();
                let state = prop(claim, "state").unwrap_or_default();

                if dropped
                    && from == on_slice
                    && let Some((_, claims)) = declared.iter().find(|(s, _)| s == &on_slice)
                    && !claims.is_empty()
                    && !claims.contains(&claim_id)
                {
                    out.push(Violation {
                        entity_kind: "iteration".to_owned(),
                        entity_id: id.clone(),
                        refusal: Refusal::DroppedClaim {
                            claim: claim_id.clone(),
                            slice: on_slice.clone(),
                        },
                        span: claim.span(),
                    });
                    continue;
                }

                if silent_drop
                    && closed
                    && state != "met"
                    && !findings.iter().any(|f| f.as_deref() == Some(claim_id.as_str()))
                {
                    out.push(Violation {
                        entity_kind: "iteration".to_owned(),
                        entity_id: id.clone(),
                        refusal: Refusal::SilentDrop { claim: claim_id, state },
                        span: claim.span(),
                    });
                }
            }
        }
    }
    out
}

/// `TS.260820.06`. Evidence naming a layer the slice never declared is refused; a
/// declared layer the iteration evidences nothing for is reported, never omitted.
fn check_layers(docs: &[KdlDocument]) -> Vec<Violation> {
    let mut declared: Vec<(String, Vec<String>)> = Vec::new();
    for doc in docs {
        for node in doc.nodes().iter().filter(|n| n.name().value() == "thin-slice") {
            if let Some(id) = string_arg(node) {
                let layers = children_named(node, "layer")
                    .iter()
                    .filter_map(|c| string_arg(c))
                    .collect();
                declared.push((id, layers));
            }
        }
    }

    let mut out = Vec::new();
    for doc in docs {
        for node in doc.nodes().iter().filter(|n| n.name().value() == "iteration") {
            let Some(on_slice) = children_named(node, "on-slice").first().and_then(|n| string_arg(n))
            else {
                continue;
            };
            let Some((_, allowed)) = declared.iter().find(|(id, _)| id == &on_slice) else {
                continue;
            };
            let id = string_arg(node);

            let mut evidenced: Vec<String> = Vec::new();
            for layer in children_named(node, "layer") {
                let Some(name) = string_arg(layer) else { continue };
                if allowed.contains(&name) {
                    evidenced.push(name);
                    continue;
                }
                out.push(Violation {
                    entity_kind: "iteration".to_owned(),
                    entity_id: id.clone(),
                    refusal: Refusal::UndeclaredLayer {
                        layer: name,
                        slice: on_slice.clone(),
                        declared: allowed.clone(),
                    },
                    span: layer.span(),
                });
            }

            // An iteration that has not sealed its layer set is still deciding what it
            // will reach, so an absent layer is not yet a gap.
            let sealed = children_named(node, "layer-set")
                .first()
                .and_then(|n| n.get("sealed").and_then(|v| v.as_bool()))
                .unwrap_or(false);
            if !sealed {
                continue;
            }
            for layer in allowed.iter().filter(|l| !evidenced.contains(l)) {
                out.push(Violation {
                    entity_kind: "iteration".to_owned(),
                    entity_id: id.clone(),
                    refusal: Refusal::UnevidencedLayer {
                        layer: layer.clone(),
                        slice: on_slice.clone(),
                    },
                    span: node.span(),
                });
            }
        }
    }
    out
}

fn children_named<'a>(node: &'a KdlNode, name: &str) -> Vec<&'a KdlNode> {
    node.children()
        .map(|b| b.nodes().iter().filter(|n| n.name().value() == name).collect())
        .unwrap_or_default()
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
    fn walk(node: &KdlNode, known: &mut Known) {
        if let Some(id) = string_arg(node) {
            known.insert(node.name().value(), id);
        }
        if let Some(body) = node.children() {
            for child in body.nodes() {
                walk(child, known);
            }
        }
    }
    for node in doc.nodes() {
        walk(node, known);
    }
}

/// Ignore the field-level detail and answer the one question a gate asks: does any of
/// this fail closed? A report is named but does not refuse.
pub fn refused(violations: &[Violation]) -> bool {
    violations.iter().any(|v| v.severity() == Severity::Refuse)
}

/// Re-exported so the shell can render a violation without reaching into `prop`.
pub fn note(node: &KdlNode, key: &str) -> Option<String> {
    prop(node, key)
}
