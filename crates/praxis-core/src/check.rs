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
    /// A cut release whose content no longer matches the seal written when it was cut.
    BrokenSeal { release: String, expected: String, found: String },
    /// A cut release with no seal at all, so nothing about it can be checked.
    UnsealedRelease { release: String },
    /// An iteration carrying two claims with one id, from one slice.
    DuplicateClaim { claim: String, slice: String },
    /// A record whose kind declares immutability and that carries no seal.
    Unsealed { kind: String, id: String, state: String },
    /// A commitment that does not carry a claim one of its slices declares.
    ClaimNotCarried { claim: String, slice: String },
    /// A declared rule that nothing demonstrates refusing.
    UnwitnessedRule { rule: String, why: &'static str },
    /// A doctrine-surface names a path the plugin does not ship.
    SurfaceDoesNotShip { path: String },
    /// A surface declared retired whose file is still in the tree.
    RetiredSurfaceStillShips { path: String },
    /// A shipped instruction file no doctrine-surface declares.
    UnanchoredSurface { path: String },
    /// An invariant the config enables that no surface enforces.
    UnkeptInvariant { invariant: String, protects: String },
    /// The config binds to a method this engine does not carry.
    UnknownMethod { named: String, carried: String },
    /// A kind whose `states=` and whose `state` field's `one-of=` disagree.
    SplitStateVocabulary { kind: String, only_in_states: Vec<String>, only_in_field: Vec<String> },
    /// The record holds no persona at all, so nothing it contains says who it is for.
    NobodyItIsFor,
    /// A frame naming no strategy it is worked under.
    FrameWithNoStrategy { frame: String },
    /// A slice claiming value without naming whose value it is.
    ValueWithNoJudge { slice: String },
    /// A field disagreeing with the latest `matured` entry recorded for it.
    ContradictedMaturation { field: String, matured_to: String, holds: String },
    /// An implement phase marked complete against a design-system that produced an approach
    /// it does not name.
    ImplementWithoutApproach { iteration: String, approaches: usize },
    /// A teach phase that names no reader.
    TaughtNobody { iteration: String },
    /// An implement naming an approach the iteration never produced.
    FollowedNothing { iteration: String, named: String },
    /// An approach neither followed nor abandoned.
    ApproachNotTaken { iteration: String, approach: String },
    /// A symptom resolved by a release the record does not hold.
    ResolvedByNothing { symptom: String, version: String },
    /// A symptom resolved by a release that bound no slice attacking it.
    ResolvedWithoutAttack { symptom: String, version: String },
    /// A decision with no alternatives, or none that would show it wrong.
    PreferenceNotDecision { decision: String, missing: &'static str },
    /// A decision that belongs to no iteration.
    UnboundDecision { decision: String },
    /// An accepted decision whose body no longer matches the seal written when it was accepted.
    RewrittenDecision { decision: String },
    /// A read model nobody has said whether to publish.
    UndeclaredLifetime { view: String },
    /// A publishable view with no reason, or nowhere to land.
    UnjustifiedLifetime { view: String, missing: &'static str },
    /// A capability whose promoted truth is not what the record derives from cut releases.
    HandPromoted { capability: String, found: usize, derived: usize },
    /// A layer the slice declared that the iteration has not evidenced. Reported: it must
    /// be visible as unevidenced rather than absent, which is C2.
    UnevidencedLayer { layer: String, slice: String },
    /// A field naming a file outside the record. The record cannot follow it, so nothing
    /// notices when it is renamed and the claim it backs is orphaned while still reading
    /// as settled.
    UnfollowableCitation { field: String, cited: String },
}

impl Refusal {
    /// The rule this refusal enforces, by the name the RECORD declares for it.
    ///
    /// This is what makes "which rule fired" answerable, and therefore what makes a
    /// witness checkable. Writing it also found that the engine was enforcing five rules
    /// the record never named — A4 inverted (ITER.260821.19/AJ1).
    pub fn rule(&self) -> &'static str {
        match self {
            Self::MissingField { .. } => "entity-without-a-required-field",
            Self::WrongCardinality { .. } => "field-outside-its-cardinality",
            Self::NotInVocabulary { .. } => "undeclared-state",
            Self::DanglingReference { .. } => "dangling-relationship",
            Self::UndeclaredKind { .. } => "undeclared-entity-kind",
            Self::ShapelessKind { .. } => "kind-declared-without-a-shape",
            Self::ContestedValue { .. } => "a-value-claimed-twice",
            Self::UnclaimedValue { .. } => "a-value-nothing-claims",
            Self::UndeclaredLayer { .. } | Self::UnevidencedLayer { .. } => "evidence-names-its-layer",
            Self::SilentDrop { .. } => "no-silent-drop",
            Self::DroppedClaim { .. } => "claim-dropped-from-the-slice",
            Self::DuplicateClaim { .. } => "a-claim-id-is-unique-in-its-iteration",
            Self::ClaimNotCarried { .. } => "a-commitment-carries-every-claim",
            Self::BrokenSeal { .. } | Self::UnsealedRelease { .. } => "cut-release-is-immutable",
            Self::Unsealed { .. } => "an-immutable-record-carries-a-seal",
            Self::HandPromoted { .. } => "promoted-truth-is-derived",
            Self::UndeclaredLifetime { .. } | Self::UnjustifiedLifetime { .. } => {
                "publish-only-what-survives-freezing"
            }
            Self::PreferenceNotDecision { missing, .. } => {
                if *missing == "over" {
                    "a-decision-names-what-it-rejected"
                } else {
                    "a-decision-names-its-falsifier"
                }
            }
            Self::UnboundDecision { .. } => "a-decision-is-bound-to-an-iteration",
            Self::RewrittenDecision { .. } => "an-accepted-decision-is-append-only",
            Self::ResolvedByNothing { .. } | Self::ResolvedWithoutAttack { .. } => {
                "resolution-names-a-release"
            }
            Self::UnwitnessedRule { .. } => "a-rule-has-a-witness",
            Self::SurfaceDoesNotShip { .. } | Self::RetiredSurfaceStillShips { .. } => {
                "a-declared-surface-ships"
            }
            Self::UnanchoredSurface { .. } => "every-shipped-surface-is-anchored",
            Self::UnkeptInvariant { .. } => "an-enabled-invariant-is-enforced",
            Self::UnknownMethod { .. } => "a-config-binds-a-method-the-engine-carries",
            Self::SplitStateVocabulary { .. } => "a-state-vocabulary-is-declared-once",
            Self::NobodyItIsFor => "a-record-names-somebody-it-is-for",
            Self::FrameWithNoStrategy { .. } => "a-frame-is-worked-under-a-strategy",
            Self::ValueWithNoJudge { .. } => "a-claim-of-value-names-whose",
            Self::ContradictedMaturation { .. } => "a-value-agrees-with-its-maturation",
            Self::ImplementWithoutApproach { .. } => "implement-follows-an-approach",
            Self::TaughtNobody { .. } => "teach-reaches-an-end-user",
            Self::FollowedNothing { .. } | Self::ApproachNotTaken { .. } => {
                "the-plan-and-the-code-agree"
            }
            Self::UnfollowableCitation { .. } => "the-record-cites-what-it-holds",
        }
    }

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
            Self::BrokenSeal { .. } | Self::UnsealedRelease { .. } => "seal",
            Self::HandPromoted { .. } => "shipped",
            Self::UndeclaredLifetime { .. } => "publishable",
            Self::PreferenceNotDecision { missing, .. } => missing,
            Self::UnboundDecision { .. } | Self::RewrittenDecision { .. } => "decision",
            Self::ResolvedByNothing { .. } | Self::ResolvedWithoutAttack { .. } => "resolved-by",
            Self::DuplicateClaim { claim, .. } | Self::ClaimNotCarried { claim, .. } => claim,
            Self::Unsealed { .. } => "seal",
            Self::UnwitnessedRule { .. } => "witness",
            Self::SurfaceDoesNotShip { .. }
            | Self::RetiredSurfaceStillShips { .. }
            | Self::UnanchoredSurface { .. } => "path",
            Self::UnkeptInvariant { .. } => "protects",
            Self::UnknownMethod { .. } => "governed-by",
            Self::SplitStateVocabulary { .. } => "states",
            Self::NobodyItIsFor => "persona",
            Self::FrameWithNoStrategy { .. } => "under",
            Self::ValueWithNoJudge { .. } => "useful-to",
            Self::ContradictedMaturation { field, .. } => field,
            Self::ImplementWithoutApproach { .. } => "followed",
            Self::TaughtNobody { .. } => "produced",
            Self::FollowedNothing { .. } => "followed",
            Self::ApproachNotTaken { .. } => "approach",
            Self::UnjustifiedLifetime { missing, .. } => missing,
            Self::UnfollowableCitation { field, .. } => field,
        }
    }

    /// Whether this stops the work.
    pub fn severity(&self) -> Severity {
        match self {
            Self::ShapelessKind { .. }
            | Self::UnclaimedValue { .. }
            | Self::UnwitnessedRule { .. }
            // An invariant declared before its probe is written is a legitimate order of
            // work. What is not legitimate is nobody knowing which.
            | Self::UnkeptInvariant { .. }
            // One primary persona is enough. This reports the ABSENCE of any, never the
            // absence of many — enumerating personas upfront is a week spent on people
            // nobody has met.
            | Self::NobodyItIsFor
            // One frame exists and it predates the kind. Every enforcement in this frame
            // that failed closed on arrival had to be walked back.
            | Self::FrameWithNoStrategy { .. }
            // Reported: half this record predates the persona kind, and a rule failing closed
            // on the day it lands is one nobody adopts. It flips when the count is zero, the
            // way every enforcement in this frame has had to earn its severity.
            | Self::ValueWithNoJudge { .. }
            // Reported: every teach phase in this record names a skill today, and a rule
            // failing closed on arrival names fourteen. Flips when the count is zero.
            | Self::TaughtNobody { .. }
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
                    "`{kind}` inside `{parent}` is neither a field {parent} declares nor a kind \
                     the schema knows. If {parent} is the method's, `praxis schema --print` \
                     shows what it takes; if it is yours, declare the field on your own schema \
                     block — you may EXTEND the method and may not redefine it"
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
            Self::ClaimNotCarried { claim, slice } => format!(
                "commits to {slice} and does not carry its {claim}. The close accounts for the \
                 claims an ITERATION holds, so a claim dropped from the iteration is a claim \
                 nothing objects to — the same move as deleting it from the slice, one level over"
            ),
            Self::DuplicateClaim { claim, slice } => format!(
                "carries {claim} from {slice} twice. The gate pins a claim at open and the working \
                 agent settles it — writing the settled one BESIDE the pinned one leaves both, and \
                 the close reads whichever comes first"
            ),
            Self::Unsealed { kind, id, state } => format!(
                "{id} is a {kind} in state {state:?} and carries no seal. Append-only that nothing \
                 seals is append-only nobody has: the rules that check a seal check it ONLY WHEN \
                 PRESENT, so an unsealed record is protected by nothing"
            ),
            Self::SurfaceDoesNotShip { path } => format!(
                "declares `{path}`, which the plugin does not ship — a surface anchored to an \
                 absent file is a promise the record cannot keep"
            ),
            Self::RetiredSurfaceStillShips { path } => format!(
                "is retired and `{path}` is still in the tree — the retirement was recorded and \
                 never carried out"
            ),
            Self::UnanchoredSurface { path } => format!(
                "`{path}` ships and no doctrine-surface declares it — instruction an agent \
                 follows on the plugin's authority alone. Anchor it (`anchor-a-doctrine-surface`) \
                 or retire it; the count reached zero at 0.8.0 and this rule fails closed to keep \
                 it there"
            ),
            Self::UnkeptInvariant { invariant, protects } => format!(
                "`{invariant}` is enabled and nothing enforces it — the plugin guarantees that \
                 {protects}, and no shipped surface keeps it"
            ),
            Self::ImplementWithoutApproach { iteration, approaches } => format!(
                "{iteration} implemented without naming which of its {approaches} approach(es) it \
                 followed. Deriving the better approach is what design-system is FOR, and an \
                 implement that does not point at one built something the plan did not describe"
            ),
            Self::FollowedNothing { iteration, named } => format!(
                "{iteration} followed {named:?}, which its design-system never produced. An \
                 approach the record says was taken and does not hold is worse than no plan — no \
                 plan is silence, and this is an account of a choice nobody made"
            ),
            Self::ApproachNotTaken { iteration, approach } => format!(
                "{iteration} produced the approach {approach:?} and neither followed nor abandoned \
                 it. Abandoning a plan is normal and expected — say so with `abandoned`, and the \
                 alternative stays in the record where it is worth something"
            ),
            Self::TaughtNobody { iteration } => format!(
                "{iteration}'s teach phase names no reader. Say who it taught with `taught`: a \
                 skill teaches an AGENT and is the doctrine layer, while a version's docs teach \
                 the reader of a release. Counting one artifact as both is what hid the \
                 difference for fourteen iterations"
            ),
            Self::ContradictedMaturation { field, matured_to, holds } => format!(
                "`{field}` matured to {matured_to:?} and the record holds {holds:?}. One of the \
                 two is wrong and a reader cannot tell which — which is worse than not recording \
                 the change, because it reads as an account of what happened"
            ),
            Self::UnfollowableCitation { field, cited } => format!(
                "`{field}` cites {cited:?}, which is a file and not something the record holds. \
                 Nothing notices when it is renamed, so the claim it backs is orphaned while \
                 still reading as settled. State WHAT was shown; the repository already knows \
                 where it lives. To point at something, point at an id — the record holds those, \
                 and a dangling one is refused"
            ),
            Self::ValueWithNoJudge { slice } => format!(
                "{slice} says what you get and not who gets it. Value is not a property of a \
                 change — it is a judgement somebody makes, and a record stating the change \
                 without the judge has recorded half of it. Name one with `useful-to`"
            ),
            Self::FrameWithNoStrategy { frame } => format!(
                "{frame} names no strategy it is worked under. Everything this record holds \
                 descends from a frame, and a frame with nothing above it is a problem nobody can \
                 justify working on — the check is that nothing is orphaned, never whether the \
                 strategy is right"
            ),
            Self::NobodyItIsFor => "the record names nobody it is for. Every fact it holds \
                 exists to serve somebody, and `useful-alone` — what you get if this ships — has \
                 an unstated subject until one persona is declared"
                .to_owned(),
            Self::SplitStateVocabulary { kind, only_in_states, only_in_field } => format!(
                "`{kind}` declares its states twice and the two disagree — {} only in `states=`, \
                 {} only in the `state` field's `one-of=`. A kind refuses what it also permits, \
                 and adding a state to one of them changes nothing visible",
                if only_in_states.is_empty() { "nothing".to_owned() } else { only_in_states.join(" · ") },
                if only_in_field.is_empty() { "nothing".to_owned() } else { only_in_field.join(" · ") }
            ),
            Self::UnknownMethod { named, carried } => format!(
                "`governed-by` names {named}, and this engine carries {carried}. A binding to \
                 a method the engine does not have is a repository being checked against \
                 rules nobody can see — install the engine that carries {named}, or bind to \
                 what this one has"
            ),
            Self::UnwitnessedRule { rule, why } => format!(
                "{rule} is declared and {why}. A rule that has never been shown to refuse is \
                 indistinguishable from one that CANNOT — which is S3 turned on the enforcement \
                 layer itself"
            ),
            Self::ResolvedByNothing { symptom, version } => format!(
                "{symptom} names {version:?} as what resolved it, and the record holds no cut \
                 release at that version. A frame shrinks by a release, never by an opinion"
            ),
            Self::ResolvedWithoutAttack { symptom, version } => format!(
                "{symptom} names {version} as what resolved it, and {version} bound no slice whose \
                 `attacks` names {symptom}. Resolution is COMPUTED from what shipped rather than \
                 asserted by whoever is closing it"
            ),
            Self::PreferenceNotDecision { decision, missing } => format!(
                "{decision:?} declares no `{missing}`. A decision with no alternatives it rejected, \
                 or nothing that would show it wrong, is a preference — and a preference recorded \
                 as a decision is the hardest kind to argue with later"
            ),
            Self::UnboundDecision { decision } => format!(
                "{decision:?} belongs to no iteration. A decision bound to a folder beside the work \
                 loses the one thing that makes it readable later: what forced it"
            ),
            Self::RewrittenDecision { decision } => format!(
                "{decision:?} is accepted and its body no longer matches what was accepted. Correct \
                 it by APPENDING an amendment — a decision that can be edited afterwards is a \
                 decision nobody can cite"
            ),
            Self::UndeclaredLifetime { view } => format!(
                "{view} does not say whether it survives being frozen. There is no repository-wide \
                 default, because perishability is a property of the question and not of the file \
                 type — declare `publishable`, either way"
            ),
            Self::UnjustifiedLifetime { view, missing } => format!(
                "{view} is publishable and declares no `{missing}`. The whole decision is a \
                 judgement about perishability, and an unjustified judgement is indistinguishable \
                 from an oversight"
            ),
            Self::HandPromoted { capability, found, derived } => format!(
                "{capability} declares {found} shipped entr{} and the record derives {derived} from \
                 what cut releases bound. Promoted truth is DERIVED — if it did not come from bound \
                 work it is not promotion, it is an assertion about the past",
                if *found == 1 { "y" } else { "ies" }
            ),
            Self::BrokenSeal { release, expected, found } => format!(
                "{release} is cut and its content no longer matches the seal written when it was \
                 cut — expected {expected}, computed {found}. A cut release is a point on the \
                 version line, and a point that moved is not one"
            ),
            Self::UnsealedRelease { release } => format!(
                "{release} is cut and carries no seal, so nothing about it can be checked against \
                 what was cut"
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
    let declared_state = child_arg(node, "state");

    let mut out = Vec::new();
    for field in &spec.fields {
        if !field.applies_to(declared_kind.as_deref())
            || !field.applies_in(declared_state.as_deref())
        {
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
            // A field may reference one of SEVERAL kinds: `doctrine-surface.serves` names a
            // slice, a capability, or a read model, and splitting that into three fields
            // would make the schema describe the checker's convenience rather than the
            // thing. Dangling only when some named kind is known and none of them holds it.
            let targets: Vec<&str> = target.split_whitespace().collect();
            let any_known = targets.iter().any(|t| known.knows_kind(t));
            let held = targets
                .iter()
                .any(|t| known.holds(t, &names) || known.holds(t, &value));
            if any_known && !held {
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

/// How many times a field appears — as a child node, or as a property.
///
/// A kind whose values are carried as properties was unshapeable until `TS.260821.19`, and
/// `phase` was declared "deliberately shapeless" for exactly that reason. It was not a
/// choice about phases; it was the schema having no way to describe them.
fn count_of(node: &KdlNode, field: &str) -> usize {
    let children = body(node).map_or(0, |b| {
        b.nodes().iter().filter(|n| n.name().value() == field).count()
    });
    if children > 0 {
        return children;
    }
    usize::from(node.get(field).is_some())
}

/// A field's single value, from a child node or from a property.
fn child_arg(node: &KdlNode, field: &str) -> Option<String> {
    body(node)
        .and_then(|b| b.nodes().iter().find(|n| n.name().value() == field))
        .and_then(string_arg)
        .or_else(|| prop(node, field))
}

fn field_span(node: &KdlNode, field: &str) -> Option<SourceSpan> {
    body(node)?
        .nodes()
        .iter()
        .find(|n| n.name().value() == field)
        .map(|n| n.span())
}

/// What the world outside the record looked like when the check ran.
///
/// Every rule before `TS.260821.03` was decidable from the record alone. The surface rules
/// are the first that are not: whether a file ships is not something the record can hold
/// without duplicating the tree. So the world is handed IN — the core still reads nothing,
/// and a rule about the world is witnessed by a rule that declares the world it assumes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Facts {
    /// Every instruction file the plugin ships, as the shell found them.
    pub shipped: Vec<String>,
}

/// Rules that can only be decided by looking at the whole record at once: whether two
/// entities contest one value, and whether a value nothing claims exists.
///
/// Facts default to empty. A caller with a real tree wants `check_corpus_given`.
pub fn check_corpus(docs: &[KdlDocument], schema: &Schema) -> Vec<Violation> {
    check_corpus_given(docs, schema, &Facts::default())
}

/// As `check_corpus`, with what the shell observed outside the record.
pub fn check_corpus_given(
    docs: &[KdlDocument],
    schema: &Schema,
    facts: &Facts,
) -> Vec<Violation> {
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

    // `an-immutable-record-carries-a-seal` and `a-claim-id-is-unique-in-its-iteration`.
    // The first is what makes the other seal rules mean anything: they check a seal ONLY
    // WHEN PRESENT, so an unsealed record is protected by nothing.
    if schema.declares_rule("an-immutable-record-carries-a-seal")
        || schema.declares_rule("a-claim-id-is-unique-in-its-iteration")
        || schema.declares_rule("a-commitment-carries-every-claim")
    {
        out.extend(check_seals_and_claims(
            docs,
            schema.declares_rule("an-immutable-record-carries-a-seal"),
            schema.declares_rule("a-claim-id-is-unique-in-its-iteration"),
            schema.declares_rule("a-commitment-carries-every-claim"),
        ));
    }

    // `resolution-names-a-release`, if the record declares it. The sharpest check here:
    // it makes progress against a problem computable from the record rather than declared
    // by whoever wants to close it.
    if schema.declares_rule("resolution-names-a-release") {
        let corpus = crate::admission::Corpus::from_documents(docs, schema);
        for symptom in corpus.symptoms.iter().filter(|s| s.resolved()) {
            let Some(version) = &symptom.resolved_by else { continue };
            let Some(release) = corpus.release(version).filter(|r| r.cut()) else {
                out.push(Violation {
                    entity_kind: "symptom".to_owned(),
                    entity_id: Some(symptom.id.clone()),
                    refusal: Refusal::ResolvedByNothing {
                        symptom: symptom.id.clone(),
                        version: version.clone(),
                    },
                    span: SourceSpan::from(0),
                });
                continue;
            };
            let attacked = release.binds.iter().any(|iteration| {
                corpus
                    .attempts
                    .iter()
                    .find(|a| &a.id == iteration)
                    .is_some_and(|a| {
                        a.on_slices
                            .iter()
                            .filter_map(|id| corpus.slice(id))
                            .any(|slice| slice.attacks.contains(&symptom.id))
                    })
            });
            if !attacked {
                out.push(Violation {
                    entity_kind: "symptom".to_owned(),
                    entity_id: Some(symptom.id.clone()),
                    refusal: Refusal::ResolvedWithoutAttack {
                        symptom: symptom.id.clone(),
                        version: version.clone(),
                    },
                    span: SourceSpan::from(0),
                });
            }
        }
    }

    // The decision rules, if the record declares them. A decision is bound to the
    // iteration that FORCED it, names what it rejected and what would show it wrong, and
    // once accepted is corrected only by appending.
    // Every decision rule, including the one this guard originally omitted — which made
    // a-decision-is-bound-to-an-iteration unreachable whenever it was the only decision
    // rule declared. Found by its own witness (ITER.260821.19/AJ3).
    if schema.declares_rule("a-decision-names-what-it-rejected")
        || schema.declares_rule("a-decision-names-its-falsifier")
        || schema.declares_rule("a-decision-is-bound-to-an-iteration")
        || schema.declares_rule("an-accepted-decision-is-append-only")
    {
        out.extend(check_decisions(docs, schema));
    }

    // `publish-only-what-survives-freezing`, if the record declares it. Membership of the
    // published set comes from these declarations and from nothing else.
    if schema.declares_rule("publish-only-what-survives-freezing") {
        for doc in docs {
            for storm in doc.nodes().iter().filter(|n| n.name().value() == "event-storm") {
                for view in children_named(storm, "read-model") {
                    let Some(name) = string_arg(view) else { continue };
                    match view.get("publishable").and_then(|v| v.as_bool()) {
                        None => out.push(Violation {
                            entity_kind: "read-model".to_owned(),
                            entity_id: Some(name.clone()),
                            refusal: Refusal::UndeclaredLifetime { view: name },
                            span: view.span(),
                        }),
                        Some(true) => {
                            for (field, missing) in
                                [("because", "because"), ("publishes-to", "publishes-to")]
                            {
                                if prop(view, field).is_none() {
                                    out.push(Violation {
                                        entity_kind: "read-model".to_owned(),
                                        entity_id: Some(name.clone()),
                                        refusal: Refusal::UnjustifiedLifetime {
                                            view: name.clone(),
                                            missing,
                                        },
                                        span: view.span(),
                                    });
                                }
                            }
                        }
                        Some(false) => {
                            if prop(view, "because").is_none() {
                                out.push(Violation {
                                    entity_kind: "read-model".to_owned(),
                                    entity_id: Some(name.clone()),
                                    refusal: Refusal::UnjustifiedLifetime {
                                        view: name.clone(),
                                        missing: "because",
                                    },
                                    span: view.span(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // `promoted-truth-is-derived`, if the record declares it. This is the rule that makes
    // W7's class answerable: a derived field written once and never rechecked is a cache;
    // one recomputed by a rule is a projection with a proof attached.
    if schema.declares_rule("promoted-truth-is-derived") {
        let corpus = crate::admission::Corpus::from_documents(docs, schema);
        for (capability, found, derived) in crate::promote::undeclared_promotions(&corpus) {
            let span = docs
                .iter()
                .flat_map(|d| d.nodes())
                .find(|n| n.name().value() == "capability" && string_arg(n).as_deref() == Some(&capability))
                .map_or_else(|| SourceSpan::from(0), KdlNode::span);
            out.push(Violation {
                entity_kind: "capability".to_owned(),
                entity_id: Some(capability.clone()),
                refusal: Refusal::HandPromoted {
                    capability,
                    found: found.len(),
                    derived: derived.len(),
                },
                span,
            });
        }
    }

    // `cut-release-is-immutable`, if the record declares it.
    if schema.declares_rule("cut-release-is-immutable") {
        out.extend(check_seals(docs));
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

    out.extend(check_surfaces(docs, schema, facts));
    out.extend(check_invariants(docs, schema));
    out.extend(check_binding(docs));
    // `TS.260821.19`. What each phase produced, and whether the next one read it.
    if schema.entity("approach").is_some() {
        for doc in docs {
            for node in doc.nodes().iter().filter(|n| n.name().value() == "iteration") {
                let id = string_arg(node).unwrap_or_default();
                let phases: Vec<&KdlNode> = children_named(node, "phase");
                let complete = |kind: &str| -> Option<&KdlNode> {
                    phases.iter().copied().find(|p| {
                        string_arg(p).as_deref() == Some(kind)
                            && child_arg(p, "state").as_deref() == Some("complete")
                    })
                };

                // An implement that did not read its own plan.
                let approaches = complete("design-system")
                    .map(|p| children_named(p, "approach").len())
                    .unwrap_or_default();
                if approaches > 0
                    && let Some(implement) = complete("implement")
                    && children_named(implement, "followed").is_empty()
                    && prop(implement, "followed").is_none()
                {
                    out.push(Violation {
                        entity_kind: "phase".to_owned(),
                        entity_id: Some(id.clone()),
                        refusal: Refusal::ImplementWithoutApproach {
                            iteration: id.clone(),
                            approaches,
                        },
                        span: implement.span(),
                    });
                }

                // The plan and the code agree. Not `was it written first` — that is
                // unverifiable and was the wrong question (ITER.260822.14/AZ2, superseded).
                // What is checkable is that every approach the record holds was taken, and
                // that what implement claims to have followed exists.
                if let Some(design) = complete("design-system") {
                    let produced: Vec<String> = children_named(design, "approach")
                        .iter()
                        .filter_map(|a| string_arg(a))
                        .collect();
                    let followed: Vec<String> = complete("implement")
                        .map(|i| {
                            children_named(i, "followed")
                                .iter()
                                .filter_map(|f| string_arg(f))
                                .chain(prop(i, "followed"))
                                .collect()
                        })
                        .unwrap_or_default();

                    for named in &followed {
                        if !produced.contains(named) {
                            out.push(Violation {
                                entity_kind: "phase".to_owned(),
                                entity_id: Some(id.clone()),
                                refusal: Refusal::FollowedNothing {
                                    iteration: id.clone(),
                                    named: named.clone(),
                                },
                                span: design.span(),
                            });
                        }
                    }
                    for approach in children_named(design, "approach") {
                        let Some(name) = string_arg(approach) else { continue };
                        // Abandoning is a recorded act. The approach stays in the record —
                        // deleting it loses the alternative that was considered, which is the
                        // reason the plan was worth writing.
                        let abandoned = !children_named(approach, "abandoned").is_empty()
                            || prop(approach, "abandoned").is_some();
                        if !abandoned && !followed.contains(&name) {
                            out.push(Violation {
                                entity_kind: "phase".to_owned(),
                                entity_id: Some(id.clone()),
                                refusal: Refusal::ApproachNotTaken {
                                    iteration: id.clone(),
                                    approach: name,
                                },
                                span: approach.span(),
                            });
                        }
                    }
                }

                // A teach phase that does not say who it taught. Naming the reader rather
                // than sniffing the path: `produced="skills/…"` was a heuristic on a string,
                // and it would have called a correct teach phase wrong the moment somebody
                // wrote a guide under a different directory.
                if let Some(teach) = complete("teach")
                    && children_named(teach, "taught").is_empty()
                    && prop(teach, "taught").is_none()
                {
                    out.push(Violation {
                        entity_kind: "phase".to_owned(),
                        entity_id: Some(id.clone()),
                        refusal: Refusal::TaughtNobody { iteration: id },
                        span: teach.span(),
                    });
                }
            }
        }
    }

    // A value that disagrees with its own maturation. Checked from the record alone: the
    // prior value is carried, so no comparison against git is needed — and reaching outside
    // the record for it would make this rule depend on the least durable thing there is.
    if schema.entity("matured").is_some() {
        for doc in docs {
            for node in doc.nodes() {
                // The LATEST maturation per field, and only that one. A field that matured
                // twice has a stale first `to` by construction — that is what iterating IS,
                // and refusing it would make a chain of improvements look like a defect.
                let mut latest: Vec<(String, &KdlNode)> = Vec::new();
                for entry in children_named(node, "matured") {
                    let Some(field) = string_arg(entry) else { continue };
                    match latest.iter_mut().find(|(f, _)| f == &field) {
                        Some(slot) => slot.1 = entry,
                        None => latest.push((field, entry)),
                    }
                }
                for (field, entry) in latest {
                    let Some(to) = child_arg(entry, "to") else { continue };
                    let Some(holds) = child_arg(node, &field) else { continue };
                    if holds != to {
                        out.push(Violation {
                            entity_kind: node.name().value().to_owned(),
                            entity_id: string_arg(node),
                            refusal: Refusal::ContradictedMaturation {
                                field,
                                matured_to: to,
                                holds,
                            },
                            span: entry.span(),
                        });
                    }
                }
            }
        }
    }

    // A slice claiming value with an unstated subject. Only where the kind is declared: a
    // repository whose method predates `persona` is not missing something it never had.
    if schema.entity("persona").is_some() {
        for doc in docs {
            for node in doc.nodes().iter().filter(|n| n.name().value() == "thin-slice") {
                if children_named(node, "useful-alone").is_empty()
                    || !children_named(node, "useful-to").is_empty()
                {
                    continue;
                }
                out.push(Violation {
                    entity_kind: "thin-slice".to_owned(),
                    entity_id: string_arg(node),
                    refusal: Refusal::ValueWithNoJudge {
                        slice: string_arg(node).unwrap_or_default(),
                    },
                    span: field_span(node, "useful-alone").unwrap_or_else(|| node.span()),
                });
            }
        }
    }

    // A frame with nothing above it. Only where the kind is declared: a repository whose
    // method predates `strategy` is not missing something it never had.
    if schema.entity("strategy").is_some() {
        for doc in docs {
            for node in doc.nodes().iter().filter(|n| n.name().value() == "frame") {
                if child_arg(node, "under").is_none() {
                    out.push(Violation {
                        entity_kind: "frame".to_owned(),
                        entity_id: string_arg(node),
                        refusal: Refusal::FrameWithNoStrategy {
                            frame: string_arg(node).unwrap_or_default(),
                        },
                        span: node.span(),
                    });
                }
            }
        }
    }

    // A field naming a file the checker cannot follow. Refuses on arrival rather than
    // reporting first: the count was taken to zero inside the iteration that wrote the rule,
    // the way `every-shipped-surface-is-anchored` had to earn its severity.
    if schema.declares_rule("the-record-cites-what-it-holds") {
        out.extend(check_citations(docs));
    }

    // A record with no persona at all. Checked only where the kind is declared: a repository
    // whose method predates it is not missing something it never had.
    if schema.entity("persona").is_some()
        && !docs.iter().any(|d| d.nodes().iter().any(|n| n.name().value() == "persona"))
    {
        out.push(Violation {
            entity_kind: "persona".to_owned(),
            entity_id: None,
            refusal: Refusal::NobodyItIsFor,
            span: SourceSpan::from(0..0),
        });
    }
    // The composed schema, and any schema a document declares. A rule about the SCHEMA
    // cannot be witnessed by a record unless the witness's own declarations are checked —
    // and a repository's extension block deserves the same rule as the method's.
    // Deduplicated by kind: the composed schema and the document that declared it are the
    // same schema when a repository has one document, and a kind reported twice reads as two
    // faults.
    let mut split = check_state_vocabulary(schema);
    for doc in docs {
        let declared = Schema::from_document(doc);
        if !declared.is_empty() {
            split.extend(check_state_vocabulary(&declared));
        }
    }
    let mut seen: Vec<String> = Vec::new();
    split.retain(|v| match &v.entity_id {
        Some(kind) if seen.contains(kind) => false,
        Some(kind) => {
            seen.push(kind.clone());
            true
        }
        None => true,
    });
    out.extend(split);
    out
}

/// `TS.260821.17`/C1 and C4. A field naming a file the checker cannot follow.
///
/// The record says WHAT was shown, the repository says where it lives, and git says when it
/// moved. A path in the record is a fourth copy of the second one, and it is the copy that
/// rots without anybody noticing — a renamed test leaves the claim it backed reading as
/// settled and pointing at nothing.
///
/// What the record MAY cite is its own ids: `dangling-relationship` refuses one it does not
/// hold, which is exactly the check a path cannot have.
fn check_citations(docs: &[KdlDocument]) -> Vec<Violation> {
    let mut out = Vec::new();
    for doc in docs {
        for node in doc.nodes() {
            let kind = node.name().value().to_owned();
            let id = string_arg(node);
            walk_citations(node, &kind, &id, &mut out);
        }
    }
    out
}

/// The record's own assertions about its tree, which this rule does not touch. None of them
/// can rot silently, which is the property that makes a citation dangerous:
///
/// - `path` is checked by `a-declared-surface-ships`, which refuses when the file is absent
/// - `owns` and the `paths` block are READ by the tool, which fails when they are wrong
/// - `publishes-to` is where the record writes rather than what it reads
/// - `given-shipped` and `witness` declare a synthetic world for a witness, and name files
///   that deliberately do not exist
const CITES_ITS_OWN_TREE: &[&str] = &[
    "path",
    "owns",
    "publishes-to",
    "given-shipped",
    "witness",
    "product-root",
    "state-root",
    "archival-projection",
    "release-index",
    "working-projection",
    "cache",
];

fn walk_citations(
    node: &KdlNode,
    owner_kind: &str,
    owner_id: &Option<String>,
    out: &mut Vec<Violation>,
) {
    let name = node.name().value();
    if CITES_ITS_OWN_TREE.contains(&name) {
        return;
    }
    let mut flag = |field: &str, value: &str| {
        for cited in cited_files(value) {
            out.push(Violation {
                entity_kind: owner_kind.to_owned(),
                entity_id: owner_id.clone(),
                refusal: Refusal::UnfollowableCitation { field: field.to_owned(), cited },
                span: node.span(),
            });
        }
    };
    // A child field written as a bare node — `excludes "…"`, `step "1" does="…"`. The
    // first argument of an ENTITY is its id and never a citation; ids hold no dots that
    // read as an extension, so scanning them costs nothing and misses nothing.
    for value in string_args(node) {
        flag(name, &value);
    }
    for entry in node.entries() {
        let Some(key) = entry.name() else { continue };
        if CITES_ITS_OWN_TREE.contains(&key.value()) {
            continue;
        }
        let Some(value) = entry.value().as_string() else { continue };
        flag(key.value(), value);
    }
    if let Some(body) = body(node) {
        for child in body.nodes() {
            walk_citations(child, owner_kind, owner_id, out);
        }
    }
}

/// Every suffix this scanner will call a file. A CLOSED list, and deliberately so: the
/// discriminator has to be one a writer can predict. A rule that guesses at what looks like
/// a path refuses `ITER.260821.17/AG1` — the record's own citation form — the first time
/// somebody writes a slug with a dot in it.
const FILE_SUFFIXES: &[&str] = &[
    "rs", "md", "kdl", "sh", "toml", "json", "yaml", "yml", "py", "ts", "tsx", "js", "jsx",
    "lock", "cs", "java", "go", "rb", "sql", "xml", "html", "css", "txt", "cfg", "ini", "bash",
    "zsh", "sql", "proto", "gradle", "csproj",
];

/// The files a value names. Two shapes, and nothing else:
///
/// - a token ending in a known suffix — `docs/product.md`, `config.kdl`, `Cargo.toml`,
///   and `tests/proving.rs::a_rule_with_no_witness` with the function trimmed off
/// - a directory — a token ending in `/` with a separator inside it, so `and/or` and
///   `ITER.260821.17/AG1` are left alone
///
/// A trailing-slash directory with no inner separator — `docs/` — is missed. Stated rather
/// than fixed: the only places it occurs are the two fields that declare where the tool
/// writes, and both are exempt.
fn cited_files(value: &str) -> Vec<String> {
    let mut out = Vec::new();
    for raw in value.split(|c: char| {
        c.is_whitespace()
            || matches!(c, '`' | '(' | ')' | '"' | '\'' | ',' | ';' | '[' | ']' | '{' | '}' | '·')
    }) {
        let token = raw.trim_matches(|c: char| matches!(c, '.' | ':' | '—' | '*' | '?' | '!' | '='));
        if token.is_empty() {
            continue;
        }
        // A test citation carries the function after `::`. The file is what rots; the
        // function is what rots faster.
        let head = token.split("::").next().unwrap_or(token);
        let named = match head.rsplit_once('.') {
            Some((stem, suffix)) if !stem.is_empty() && FILE_SUFFIXES.contains(&suffix) => true,
            _ => {
                let inner = token.trim_end_matches('/');
                token.ends_with('/') && inner.contains('/')
            }
        };
        if named && !out.contains(&token.to_owned()) {
            out.push(token.to_owned());
        }
    }
    out
}

/// `TS.260821.13`/C5. A kind declares its states once, or it refuses what it also permits.
///
/// `withdrawn` had to be added to `release` in two places — `states=` on the kind and
/// `one-of=` on its `state` field — and the first edit alone changed nothing, which reads as
/// the schema not having been picked up (`WALK.260822.02/AS7`).
fn check_state_vocabulary(schema: &Schema) -> Vec<Violation> {
    let mut out = Vec::new();
    for kind in schema.kinds() {
        let Some(spec) = schema.entity(kind) else { continue };
        let Some(field) = spec.fields.iter().find(|f| f.name == "state") else { continue };
        if spec.states.is_empty() || field.one_of.is_empty() {
            continue;
        }
        let only_in_states: Vec<String> =
            spec.states.iter().filter(|s| !field.one_of.contains(s)).cloned().collect();
        let only_in_field: Vec<String> =
            field.one_of.iter().filter(|s| !spec.states.contains(s)).cloned().collect();
        if only_in_states.is_empty() && only_in_field.is_empty() {
            continue;
        }
        out.push(Violation {
            entity_kind: kind.to_owned(),
            entity_id: Some(kind.to_owned()),
            refusal: Refusal::SplitStateVocabulary {
                kind: kind.to_owned(),
                only_in_states,
                only_in_field,
            },
            span: SourceSpan::from(0..0),
        });
    }
    out
}

/// `TS.260821.10`/C5. The config binds to a method, and the engine carries one.
///
/// `governed-by` has pointed at nothing since ADR.260819.02 was withdrawn, because the thing
/// it should have named had no home. Now it does, and a binding to a method the engine does
/// not carry is a repository being checked against rules nobody can see — which is worse
/// than no binding at all, because it reads as one.
fn check_binding(docs: &[KdlDocument]) -> Vec<Violation> {
    let (_, carried) = Schema::method();
    let mut out = Vec::new();
    for doc in docs {
        for node in doc.nodes().iter().filter(|n| n.name().value() == "config") {
            let Some(named) = child_arg(node, "governed-by") else { continue };
            if named != carried {
                out.push(Violation {
                    entity_kind: "config".to_owned(),
                    entity_id: string_arg(node),
                    refusal: Refusal::UnknownMethod { named, carried: carried.clone() },
                    span: field_span(node, "governed-by").unwrap_or_else(|| node.span()),
                });
            }
        }
    }
    out
}

/// `TS.260821.05`: every invariant the config enables names something that keeps it.
///
/// Read entirely from the record — unlike the surface rules, no part of this needs the
/// world. What the plugin guarantees and what enforces it are both things the record says.
fn check_invariants(docs: &[KdlDocument], schema: &Schema) -> Vec<Violation> {
    if schema.entity("invariant").is_none() {
        return Vec::new();
    }

    // Every non-retired surface's `serves`, and every id the profile omits.
    let mut enforced: Vec<String> = Vec::new();
    let mut omitted: Vec<String> = Vec::new();
    for doc in docs {
        for node in doc.nodes() {
            match node.name().value() {
                "doctrine-surface" if child_arg(node, "state").as_deref() != Some("retired") => {
                    for child in children_named(node, "serves") {
                        enforced.extend(string_args(child));
                    }
                }
                "config" => {
                    for profile in children_named(node, "profile") {
                        for omit in children_named(profile, "omit-probe") {
                            omitted.extend(string_args(omit));
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let mut out = Vec::new();
    for doc in docs {
        for node in doc.nodes().iter().filter(|n| n.name().value() == "invariant") {
            let Some(id) = string_arg(node) else { continue };
            // Omitted is a BINDING, not a gap. A profile that says it has no HTTP surface is
            // answering the question, and reporting it would teach adopters to ignore the
            // report — which is how a rule stops being read.
            if omitted.iter().any(|o| o == &id) || enforced.iter().any(|e| e == &id) {
                continue;
            }
            out.push(Violation {
                entity_kind: "invariant".to_owned(),
                entity_id: Some(id.clone()),
                refusal: Refusal::UnkeptInvariant {
                    invariant: id,
                    protects: child_arg(node, "protects").unwrap_or_default(),
                },
                span: field_span(node, "protects").unwrap_or_else(|| node.span()),
            });
        }
    }
    out
}

/// `TS.260821.03`: the record's doctrine against the tree's.
///
/// Both directions, because they mean opposite things. A declared surface that does not ship
/// REFUSES — it can never be legitimate work-in-progress, because the record made a promise
/// about a file. A shipped file nothing declares is REPORTED — on the day this lands there
/// are twenty-five of them, and a rule that fails closed on its first run is one nobody can
/// adopt.
fn check_surfaces(docs: &[KdlDocument], schema: &Schema, facts: &Facts) -> Vec<Violation> {
    // The engine does not carry its own list of what a doctrine-surface is. If the schema
    // does not declare the kind, this record is not using the rule, and silence is the
    // answer — not a refusal about a vocabulary the record never adopted.
    if schema.entity("doctrine-surface").is_none() {
        return Vec::new();
    }

    let mut out = Vec::new();
    let mut declared: Vec<(String, &KdlNode)> = Vec::new();

    for doc in docs {
        for node in doc.nodes().iter().filter(|n| n.name().value() == "doctrine-surface") {
            let Some(path) = child_arg(node, "path") else { continue };
            let retired = child_arg(node, "state").as_deref() == Some("retired");
            let ships = facts.shipped.iter().any(|p| p == &path);
            declared.push((path.clone(), node));

            let refusal = match (retired, ships) {
                (false, false) => Some(Refusal::SurfaceDoesNotShip { path }),
                (true, true) => Some(Refusal::RetiredSurfaceStillShips { path }),
                _ => None,
            };
            if let Some(refusal) = refusal {
                out.push(Violation {
                    entity_kind: "doctrine-surface".to_owned(),
                    entity_id: string_arg(node),
                    refusal,
                    span: field_span(node, "path").unwrap_or_else(|| node.span()),
                });
            }
        }
    }

    // The reverse sweep. Anchored to the whole declared set, retired included: a file the
    // record retired and left in the tree is already refused above, and reporting it a
    // second time as unanchored would name one fault twice.
    for path in &facts.shipped {
        if !declared.iter().any(|(p, _)| p == path) {
            out.push(Violation {
                entity_kind: "doctrine-surface".to_owned(),
                entity_id: None,
                refusal: Refusal::UnanchoredSurface { path: path.clone() },
                span: SourceSpan::from(0..0),
            });
        }
    }

    out
}

/// `TS.260821.02`/C4 and `ITER.260821.19`/AI1.
///
/// A record whose kind declares immutability carries a seal, and an iteration carries each
/// of a slice's claims once. Both were declared before they were enforced, and the second
/// reproduced itself inside the iteration that declared it.
fn check_seals_and_claims(
    docs: &[KdlDocument],
    seals: bool,
    unique: bool,
    carried: bool,
) -> Vec<Violation> {
    let mut out = Vec::new();

    // What each slice declares, so a commitment can be held to all of it.
    let mut declared: Vec<(String, Vec<String>)> = Vec::new();
    if carried {
        for doc in docs {
            for node in doc.nodes().iter().filter(|n| n.name().value() == "thin-slice") {
                if let Some(id) = string_arg(node) {
                    let claims =
                        children_named(node, "claim").iter().filter_map(|c| string_arg(c)).collect();
                    declared.push((id, claims));
                }
            }
        }
    }

    for doc in docs {
        if seals {
            // An accepted decision. Immutability begins at acceptance, not before.
            for iteration in doc.nodes().iter().filter(|n| n.name().value() == "iteration") {
                for decision in children_named(iteration, "decision") {
                    let state = prop(decision, "state").unwrap_or_else(|| "accepted".to_owned());
                    if state == "accepted" && prop(decision, "seal").is_none() {
                        out.push(Violation {
                            entity_kind: "iteration".to_owned(),
                            entity_id: string_arg(iteration),
                            refusal: Refusal::Unsealed {
                                kind: "decision".to_owned(),
                                id: string_arg(decision).unwrap_or_default(),
                                state,
                            },
                            span: decision.span(),
                        });
                    }
                }
            }
            // A resolved symptom. TS.260820.18/C3: without a seal, a silent revert to
            // `present` is accepted, because the check has no memory of what was there.
            for frame in doc.nodes().iter().filter(|n| n.name().value() == "frame") {
                for symptom in children_named(frame, "symptom") {
                    let state = prop(symptom, "state").unwrap_or_default();
                    if matches!(state.as_str(), "resolved" | "partially-resolved")
                        && prop(symptom, "seal").is_none()
                    {
                        out.push(Violation {
                            entity_kind: "frame".to_owned(),
                            entity_id: string_arg(frame),
                            refusal: Refusal::Unsealed {
                                kind: "symptom".to_owned(),
                                id: string_arg(symptom).unwrap_or_default(),
                                state,
                            },
                            span: symptom.span(),
                        });
                    }
                }
            }
        }

        if carried {
            for iteration in doc.nodes().iter().filter(|n| n.name().value() == "iteration") {
                let on: Vec<String> = children_named(iteration, "on-slice")
                    .iter()
                    .filter_map(|n| string_arg(n))
                    .collect();
                let held: Vec<(String, String)> = children_named(iteration, "claim")
                    .iter()
                    .filter_map(|c| {
                        Some((prop(c, "from-slice").unwrap_or_default(), string_arg(c)?))
                    })
                    .collect();
                for slice in &on {
                    let Some((_, claims)) = declared.iter().find(|(id, _)| id == slice) else {
                        continue;
                    };
                    for claim in claims {
                        if !held.iter().any(|(from, id)| from == slice && id == claim) {
                            out.push(Violation {
                                entity_kind: "iteration".to_owned(),
                                entity_id: string_arg(iteration),
                                refusal: Refusal::ClaimNotCarried {
                                    claim: claim.clone(),
                                    slice: slice.clone(),
                                },
                                span: iteration.span(),
                            });
                        }
                    }
                }
            }
        }

        if unique {
            for iteration in doc.nodes().iter().filter(|n| n.name().value() == "iteration") {
                let mut seen: Vec<(String, String)> = Vec::new();
                for claim in children_named(iteration, "claim") {
                    let Some(id) = string_arg(claim) else { continue };
                    let from = prop(claim, "from-slice").unwrap_or_default();
                    if seen.contains(&(id.clone(), from.clone())) {
                        out.push(Violation {
                            entity_kind: "iteration".to_owned(),
                            entity_id: string_arg(iteration),
                            refusal: Refusal::DuplicateClaim { claim: id, slice: from },
                            span: claim.span(),
                        });
                        continue;
                    }
                    seen.push((id, from));
                }
            }
        }
    }
    out
}

/// `TS.260820.14`. A decision is bound, argued and falsifiable, or it is a preference.
fn check_decisions(docs: &[KdlDocument], schema: &Schema) -> Vec<Violation> {
    let mut out = Vec::new();
    for doc in docs {
        // A decision at the root belongs to no iteration at all.
        if schema.declares_rule("a-decision-is-bound-to-an-iteration") {
            for node in doc.nodes().iter().filter(|n| n.name().value() == "decision") {
                out.push(Violation {
                    entity_kind: "decision".to_owned(),
                    entity_id: string_arg(node),
                    refusal: Refusal::UnboundDecision {
                        decision: string_arg(node).unwrap_or_default(),
                    },
                    span: node.span(),
                });
            }
        }

        for iteration in doc.nodes().iter().filter(|n| n.name().value() == "iteration") {
            for decision in children_named(iteration, "decision") {
                let Some(title) = string_arg(decision) else { continue };
                let id = string_arg(iteration);

                if schema.declares_rule("a-decision-names-what-it-rejected")
                    && prop(decision, "over").is_none()
                    && children_named(decision, "over").is_empty()
                {
                    out.push(Violation {
                        entity_kind: "iteration".to_owned(),
                        entity_id: id.clone(),
                        refusal: Refusal::PreferenceNotDecision {
                            decision: title.clone(),
                            missing: "over",
                        },
                        span: decision.span(),
                    });
                }
                if schema.declares_rule("a-decision-names-its-falsifier")
                    && prop(decision, "falsified-by").is_none()
                {
                    out.push(Violation {
                        entity_kind: "iteration".to_owned(),
                        entity_id: id.clone(),
                        refusal: Refusal::PreferenceNotDecision {
                            decision: title.clone(),
                            missing: "falsified-by",
                        },
                        span: decision.span(),
                    });
                }

                // Append-only. The seal covers the body and deliberately not the
                // amendments, so appending one is not an edit.
                if schema.declares_rule("an-accepted-decision-is-append-only")
                    && prop(decision, "state").as_deref().unwrap_or("accepted") == "accepted"
                    && let Some(written) = prop(decision, "seal")
                {
                    let body = decision_body(decision, &title);
                    if crate::cut::seal(&body, "", &[], &[]) != written {
                        out.push(Violation {
                            entity_kind: "iteration".to_owned(),
                            entity_id: id.clone(),
                            refusal: Refusal::RewrittenDecision { decision: title.clone() },
                            span: decision.span(),
                        });
                    }
                }
            }
        }
    }
    out
}

/// The sealed material of a decision: what it chose, what it rejected, why, and what would
/// show it wrong. Amendments are excluded — appending one must not read as an edit.
pub fn decision_body(node: &KdlNode, title: &str) -> String {
    let mut over: Vec<String> = children_named(node, "over").iter().filter_map(|n| string_arg(n)).collect();
    if let Some(single) = prop(node, "over") {
        over.push(single);
    }
    over.sort();
    const UNIT: char = '\u{1f}';
    format!(
        "{title}{UNIT}{}{UNIT}{}{UNIT}{}{UNIT}{}",
        prop(node, "chose").unwrap_or_default(),
        over.join(","),
        prop(node, "because").unwrap_or_default(),
        prop(node, "falsified-by").unwrap_or_default()
    )
}

/// `TS.260820.09`/C4. A cut release's index node is never edited, and the seal is how
/// that stops being a promise. It detects an edit; it does not prevent one.
fn check_seals(docs: &[KdlDocument]) -> Vec<Violation> {
    let mut out = Vec::new();
    for doc in docs {
        for node in doc.nodes().iter().filter(|n| n.name().value() == "release") {
            let state = children_named(node, "state").first().and_then(|n| string_arg(n));
            if state.as_deref() != Some("released") {
                continue;
            }
            let id = string_arg(node);
            let version =
                children_named(node, "version").first().and_then(|n| string_arg(n)).unwrap_or_default();
            let binds: Vec<String> =
                children_named(node, "binds").iter().filter_map(|n| string_arg(n)).collect();
            let resolves: Vec<String> =
                children_named(node, "resolves").iter().filter_map(|n| string_arg(n)).collect();
            let commit = children_named(node, "index")
                .first()
                .and_then(|index| children_named(index, "commit").first().and_then(|n| string_arg(n)))
                .unwrap_or_default();

            let Some(written) = children_named(node, "seal").first().and_then(|n| string_arg(n))
            else {
                out.push(Violation {
                    entity_kind: "release".to_owned(),
                    entity_id: id.clone(),
                    refusal: Refusal::UnsealedRelease {
                        release: id.clone().unwrap_or_else(|| version.clone()),
                    },
                    span: node.span(),
                });
                continue;
            };

            let computed = crate::cut::seal(&version, &commit, &binds, &resolves);
            if computed != written {
                out.push(Violation {
                    entity_kind: "release".to_owned(),
                    entity_id: id.clone(),
                    refusal: Refusal::BrokenSeal {
                        release: id.clone().unwrap_or_else(|| version.clone()),
                        expected: written,
                        found: computed,
                    },
                    span: node.span(),
                });
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
            let on_slices: Vec<String> =
                children_named(node, "on-slice").iter().filter_map(|n| string_arg(n)).collect();
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
                    && on_slices.contains(&from)
                    && let Some((_, claims)) = declared.iter().find(|(s, _)| s == &from)
                    && !claims.is_empty()
                    && !claims.contains(&claim_id)
                {
                    out.push(Violation {
                        entity_kind: "iteration".to_owned(),
                        entity_id: id.clone(),
                        refusal: Refusal::DroppedClaim {
                            claim: claim_id.clone(),
                            slice: from.clone(),
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
            let on_slices: Vec<String> =
                children_named(node, "on-slice").iter().filter_map(|n| string_arg(n)).collect();
            if on_slices.is_empty() {
                continue;
            }
            // The union across the committed slices. A commitment may reach any layer any
            // of its slices declared, and owes every layer all of them declared.
            let mut allowed: Vec<String> = Vec::new();
            let mut known = false;
            for id in &on_slices {
                if let Some((_, layers)) = declared.iter().find(|(s, _)| s == id) {
                    known = true;
                    for layer in layers {
                        if !allowed.contains(layer) {
                            allowed.push(layer.clone());
                        }
                    }
                }
            }
            if !known {
                continue;
            }
            let on_slice = on_slices.join(" · ");
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
