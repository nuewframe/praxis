//! The gate's conditions, and the view computed from them.
//!
//! `TS.260820.04`: an agent about to choose work sees what is admissible now, and for
//! everything else the condition that blocks it. `TS.260820.05` — the gate — is built on
//! the same [`assess`] result, so the view cannot drift from the gate by construction
//! rather than by discipline.
//!
//! Which conditions exist is not decided here. They are read from the record's
//! `admits` declaration (`NA.260820.01`, capability-role work-admission), because
//! ADR.260819.01/A4 forbids the engine holding a rule the record does not state. This
//! module carries a *check* per declared name; a condition whose check it does not have
//! comes back [`Verdict::Uncomputed`] and is shown as such. Nothing declared is ever
//! silently passed.

use std::collections::BTreeMap;

use kdl::{KdlDocument, KdlNode};

use crate::check::{Known, check_document, index_all};
use crate::schema::{Schema, prop, string_arg, string_args};
use crate::view::{ReadModel, Section};

/// One condition the gate weighs, as the record declares it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition {
    pub name: String,
    /// The check this condition is decided by. `None` means the record declares the
    /// condition without naming a machine check for it.
    pub check: Option<String>,
    /// Why the record says this cannot be computed yet.
    pub uncomputed: Option<String>,
    /// Left to a human. Declared anyway, so the answer shows what it did not decide.
    pub judgement: bool,
    pub because: Option<String>,
    pub caveat: Option<String>,
}

/// The declared conditions, in the order the record declares them.
#[derive(Debug, Default, Clone)]
pub struct Conditions {
    pub of: String,
    pub all: Vec<Condition>,
}

impl Conditions {
    /// Read every `admits` block in a document. Depth-first, because `admits` sits inside
    /// a capability-role, which sits inside the architecture.
    pub fn from_document(doc: &KdlDocument) -> Self {
        let mut found = Self::default();
        collect_admits(doc, &mut found);
        found
    }

    pub fn is_empty(&self) -> bool {
        self.all.is_empty()
    }
}

fn collect_admits(doc: &KdlDocument, out: &mut Conditions) {
    for node in doc.nodes() {
        if node.name().value() == "admits" {
            out.of = string_arg(node).unwrap_or_default();
            for child in node.iter_children() {
                if child.name().value() != "condition" {
                    continue;
                }
                out.all.push(Condition {
                    name: string_arg(child).unwrap_or_default(),
                    check: prop(child, "check"),
                    uncomputed: prop(child, "uncomputed"),
                    judgement: child
                        .get("judgement")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    because: prop(child, "because"),
                    caveat: prop(child, "caveat"),
                });
            }
        }
        if let Some(children) = node.children() {
            collect_admits(children, out);
        }
    }
}

/// What one condition said about one slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// This condition does not stand in the way.
    Admits,
    /// This condition blocks, and names the specific thing.
    Blocks(String),
    /// Declared, and not decided. Never a pass.
    Uncomputed(String),
    /// A human decides this one.
    Judgement,
}

impl Verdict {
    pub fn blocks(&self) -> bool {
        matches!(self, Self::Blocks(_))
    }

    /// The specific detail, which is the half a reader actually needs. C2: no exclusion
    /// has an empty reason.
    pub fn detail(&self) -> &str {
        match self {
            Self::Blocks(why) | Self::Uncomputed(why) => why,
            Self::Admits => "",
            Self::Judgement => "left to the maintainer",
        }
    }
}

/// A slice, as much of it as admission needs.
#[derive(Debug, Clone)]
pub struct Slice {
    pub id: String,
    pub slug: String,
    pub kind: String,
    pub realizes: String,
    pub layers: Vec<String>,
    pub depends_on: Vec<String>,
    /// The claim ids the slice declares. Delivery is measured against these, never
    /// against one iteration's own accounting of what it took on.
    pub claims: Vec<String>,
    /// The symptoms this slice was cut to attack.
    pub attacks: Vec<String>,
    /// What the record SAYS its state is — which may disagree with what the iterations show.
    pub declared_state: Option<String>,

    // The half a reader wants. Every slice declares these by hand, in sentences, and until
    // TS.260821.12 nothing carried them out of the file — so the published set listed ids.
    /// One line saying what this slice makes true.
    pub title: String,
    /// The moment that calls for it.
    pub trigger: String,
    /// What is true afterwards.
    pub outcome: String,
    /// What you get if this ships and nothing after it does. Written for a reader deciding
    /// whether a version is worth having, which is what a release note is.
    pub useful_alone: Option<String>,
    /// The command this slice adds, if it adds one. How somebody starts.
    pub command: Option<String>,
}

/// One claim, as an iteration settled it. `from` is the slice that declared it, which
/// need not be the slice this iteration is on: a residue can be settled later, by the
/// iteration that finally had what settling it required.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settled {
    pub id: String,
    pub from: String,
    pub state: String,
}

/// A choice an iteration could not make implicitly, with what it rejected and what would
/// show it wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    pub title: String,
    pub iteration: String,
    pub chose: String,
    /// The alternatives genuinely available. A decision with none is a preference.
    pub over: Vec<String>,
    pub because: String,
    /// What would show it wrong. A decision with no falsifier is a preference too.
    pub falsified_by: Option<String>,
    pub state: String,
    pub seal: Option<String>,
    /// Corrections, appended. The original stays readable.
    pub amendments: Vec<String>,
}

/// One finding an iteration recorded, and the claim it accounts for if it accounts for
/// one. A shortfall carried by a finding is accounted; a shortfall carried by nothing is
/// scope dropped in silence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Carried {
    /// What the shortfall IS, in the words whoever hit it used. A finding published as an
    /// id tells a reader nothing they can act on.
    pub text: String,
    pub id: String,
    pub carries: Option<String>,
    /// A decision this finding tested, where it tested one.
    pub tests: Option<String>,
}

/// An attempt at a slice.
#[derive(Debug, Clone)]
pub struct Attempt {
    pub id: String,
    /// The slices this commitment covers. The ITERATION is the commitment; a slice is a
    /// unit of work. One ask can commit to several, and they are worked together.
    pub on_slices: Vec<String>,
    pub state: String,
    pub claims: Vec<Settled>,
    pub findings: Vec<Carried>,
    /// Layers this iteration says it reached: name, state, and the evidence named.
    pub layers: Vec<(String, String, String)>,
    /// The stages of the attempt, and who worked each.
    pub phases: Vec<Phase>,
    /// Which of the configured bump rules this iteration's work matches. Declared by the
    /// iteration, because only whoever did the work knows what kind of change it was.
    pub contributes: Vec<String>,
}

/// One observable thing that is wrong, resolving on its own schedule by naming a release.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symptom {
    pub id: String,
    pub state: String,
    /// The version claimed to have resolved it. Checkable: that release must exist AND
    /// must have bound a slice that attacks this symptom.
    pub resolved_by: Option<String>,
    pub seal: Option<String>,
    /// What is observably wrong, in the words whoever saw it used. A symptom published as
    /// an id tells a reader nothing.
    pub text: String,
}

impl Symptom {
    pub fn resolved(&self) -> bool {
        matches!(self.state.as_str(), "resolved" | "partially-resolved")
    }
}

/// A read model, and what the record declares about its lifetime. Membership of the
/// published set is derived from these and from nothing else (`TS.260820.12`).
#[derive(Debug, Clone)]
pub struct View {
    pub name: String,
    pub answers: String,
    /// `None` when nobody has said whether it survives being frozen. Not a default —
    /// perishability is a property of the question, not of the file type.
    pub publishable: Option<bool>,
    pub because: Option<String>,
    /// Where under the release directory it lands. Required of a publishable view unless
    /// a story names it as a part, in which case it lands where the story lands.
    pub publishes_to: Option<String>,
    /// Who this view is for, by persona. Declared on every read model since the storm was
    /// written and read by no projection until `TS.260821.14` — which is how a release
    /// directory came to sort its documents alphabetically for a reader who arrived with
    /// a reason.
    pub needed_by: Vec<String>,
    /// The story this view tells, in order, when it is one. Each part names the view whose
    /// content it carries; the story is the document and the parts are its chapters.
    pub parts: Vec<Part>,
}

/// One chapter of a story, and the view it carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    /// The label the record gives it — an ordinal, or anything else that orders.
    pub label: String,
    /// What this part is, in the story's words rather than the view's.
    pub is: String,
    /// The view whose content it carries.
    pub from: String,
}

/// A permanent doing the system must have, and the cluster it was derived from.
#[derive(Debug, Clone)]
pub struct Capability {
    pub id: String,
    pub state: String,
    pub from_cluster: String,
    /// `product` — what a reader installed. `engine` — how it is built. See TS.260821.06/C3.
    pub facet: String,
    pub owns: Vec<String>,
    /// Promoted truth: what shipped for this capability, and where. Derived, never
    /// hand-written — `promoted-truth-is-derived` recomputes it.
    pub shipped: Vec<crate::promote::Shipped>,
    /// Usage prose, written while the capability is built and stored IN the record. Not a
    /// file the record points at — a pointer is a second thing to keep in step.
    pub usage: Vec<String>,
    /// What this capability does, in a sentence.
    pub doing: String,
    /// What it refuses to do. Published beside `doing`, because a description that omits
    /// what a thing cannot do is the artifact this frame distrusts.
    pub not: String,
    /// The invariants it holds — what would be wrong if two of its facts disagreed.
    pub keeps_consistent: Vec<String>,
}

impl Capability {
    /// Whether this capability has a surface a person can use at `version` — which is
    /// exactly whether its truth was promoted for that release. A guide describing
    /// behaviour that never shipped is worse than no guide: it is confidently wrong.
    pub fn usable_at(&self, version: &str) -> bool {
        self.shipped.iter().any(|s| s.version == version)
    }
}

/// A version, and the work bound to it. Machine-owned: every field here is derived from
/// what was bound, which is why the record is rewritten whole rather than edited.
#[derive(Debug, Clone)]
pub struct Release {
    pub id: String,
    pub version: String,
    pub state: String,
    pub binds: Vec<String>,
    /// What the configured rules proposed, as distinct from what the maintainer confirmed
    /// at the cut.
    ///
    /// Both are kept: a confirmed bump differing from the proposed one is a maintainer
    /// overriding the rules, which is legitimate and worth being able to see. Read into the
    /// corpus because the cut composes the index node whole, and a field the corpus does not
    /// hold is one the cut drops (`AS1`).
    pub proposed_bump: String,
}

impl Release {
    /// Binding is reversible until the release is cut. Cutting is not.
    pub fn cut(&self) -> bool {
        self.state == "released"
    }
}

/// This repository's binding to the schema — the parts the engine reads.
#[derive(Debug, Clone, Default)]
pub struct Config {
    pub scheme: String,
    /// change-kind → which position to bump. Read from the record, never assumed: Praxis
    /// is pre-1.0 and bumps MINOR for a breaking change, so a hardcoded semver rule would
    /// misreport its own author's releases.
    pub bump_rules: Vec<(String, String)>,
    /// Invariants this repository's profile omits, by id. Omitting is a BINDING, not a
    /// failure — Praxis has no HTTP surface — and the record holds both so that "we do not
    /// need this" and "we did not do this" cannot read alike (TS.260821.05).
    pub omitted: Vec<String>,
}

/// Why the product exists. Not checked for truth — it is what truth is checked against.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Anchor {
    /// The world if this succeeds.
    pub vision: String,
    /// What this product does to move toward it.
    pub mission: String,
    /// What would count as having delivered the mission.
    pub delivered_when: String,
    /// The ordered approach — how we go about it, never how it is built.
    pub strategy: Vec<String>,
}

/// A problem, as the record states it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Frame {
    pub id: String,
    pub title: String,
    pub root_cause: String,
    pub principle: String,
}

/// A position a human or an agent occupies. Never a person.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Role {
    pub id: String,
    pub owns_phase: Vec<String>,
    /// The phases this role may not attest when it did the work. The field the kind exists
    /// for — everything else about a role is describable in prose, and this is not.
    pub never_for_own: Vec<String>,
}

impl Role {
    /// Whether an identity occupies this role.
    ///
    /// `agent:principal-engineer` occupies `principal-engineer`. A human identity carries no
    /// role in the record, and gets none here: the rule is about an agent attesting its own
    /// work, and a human closing an iteration an agent worked is exactly the handoff the
    /// rule exists to require.
    pub fn occupied_by(&self, identity: &str) -> bool {
        identity.strip_prefix("agent:").is_some_and(|name| name == self.id)
    }
}

/// One stage of an attempt, and who worked it.
///
/// `worked_by` is what makes `an-attestation-is-not-self-issued` computable. Before
/// TS.260821.08 a phase recorded what it PRODUCED and never who produced it, so "the same
/// engineer cannot self-approve" had nothing to compare.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Phase {
    pub kind: String,
    pub state: String,
    pub produced: String,
    /// Absent on every phase written before this was declared. Absent is not a violation —
    /// it is a phase whose worker nobody recorded, and refusing on it would make every
    /// historical iteration unclosable.
    pub worked_by: Option<String>,
}

impl Attempt {
    /// Whether this commitment covers a given slice.
    pub fn covers(&self, slice: &str) -> bool {
        self.on_slices.iter().any(|s| s == slice)
    }

    /// What to show a reader who wants one name for the commitment.
    pub fn slices(&self) -> String {
        self.on_slices.join(" · ")
    }

    pub fn in_flight(&self) -> bool {
        matches!(self.state.as_str(), "open" | "working")
    }
}

/// Everything the conditions are evaluated against.
#[derive(Debug, Default, Clone)]
pub struct Corpus {
    pub slices: Vec<Slice>,
    pub attempts: Vec<Attempt>,
    pub releases: Vec<Release>,
    pub symptoms: Vec<Symptom>,
    pub capabilities: Vec<Capability>,
    pub views: Vec<View>,
    pub decisions: Vec<Decision>,
    pub config: Config,
    /// The doctrine this plugin ships, as the record declares it (TS.260821.03).
    pub surfaces: Vec<crate::surface::Surface>,
    /// What the plugin guarantees about code it is loaded into (TS.260821.05).
    pub invariants: Vec<crate::invariant::Invariant>,
    /// Who may attest what (TS.260821.08).
    pub roles: Vec<Role>,
    /// The problems this repository is working on.
    pub frames: Vec<Frame>,
    /// Why the product exists, above the problem it attacks (TS.260821.16).
    pub anchor: Anchor,
    /// Kind → what it means, from the schema's own `is=`. Carried on the corpus so a
    /// projection can publish a glossary without the schema being threaded through every
    /// composer, and so a kind an adopting project declares appears in ITS glossary.
    pub vocabulary: Vec<(String, String)>,
    /// Ids the shape check refuses. A slice the checker refuses is not work waiting.
    pub refused: Vec<String>,
}

impl Corpus {
    pub fn from_documents(docs: &[KdlDocument], schema: &Schema) -> Self {
        let mut known = Known::default();
        for doc in docs {
            index_all(doc, &mut known);
        }

        let mut corpus = Self::default();
        for doc in docs {
            for violation in check_document(doc, schema, &known) {
                if violation.severity() == crate::Severity::Refuse
                    && let Some(id) = violation.entity_id
                    && !corpus.refused.contains(&id)
                {
                    corpus.refused.push(id);
                }
            }
            for node in doc.nodes() {
                match node.name().value() {
                    "thin-slice" => corpus.slices.push(slice_from(node)),
                    "iteration" => {
                        let id = string_arg(node).unwrap_or_default();
                        for child in node.iter_children().filter(|c| c.name().value() == "decision")
                        {
                            corpus.decisions.push(Decision {
                                title: string_arg(child).unwrap_or_default(),
                                iteration: id.clone(),
                                chose: prop(child, "chose").unwrap_or_default(),
                                over: child
                                    .iter_children()
                                    .filter(|g| g.name().value() == "over")
                                    .filter_map(string_arg)
                                    .chain(prop(child, "over"))
                                    .collect(),
                                because: prop(child, "because").unwrap_or_default(),
                                falsified_by: prop(child, "falsified-by"),
                                state: prop(child, "state").unwrap_or_else(|| "accepted".to_owned()),
                                seal: prop(child, "seal"),
                                amendments: child
                                    .iter_children()
                                    .filter(|g| g.name().value() == "amendment")
                                    .filter_map(string_arg)
                                    .collect(),
                            });
                        }
                        corpus.attempts.push(attempt_from(node));
                    }
                    "release" => corpus.releases.push(release_from(node)),
                    "capability" => corpus.capabilities.push(Capability {
                        id: string_arg(node).unwrap_or_default(),
                        state: child_arg(node, "state").unwrap_or_default(),
                        from_cluster: child_arg(node, "from-cluster").unwrap_or_default(),
                        // Undeclared reads as engine: better to under-claim what the
                        // product does than to hand a reader the internals as features.
                        facet: child_arg(node, "facet")
                            .unwrap_or_else(|| "engine".to_owned()),
                        // Every value, not the first of each node. `owns-event "A" "B" "C"`
                        // is one node carrying three events, and reading only the first
                        // reported every capability as owning exactly one (ITER.260821.12/AB1).
                        owns: node
                            .iter_children()
                            .filter(|c| c.name().value() == "owns-event")
                            .flat_map(string_args)
                            .collect(),
                        shipped: node
                            .iter_children()
                            .filter(|c| c.name().value() == "shipped")
                            .map(|c| crate::promote::Shipped {
                                version: string_arg(c).unwrap_or_default(),
                                iteration: prop(c, "by").unwrap_or_default(),
                                slice: prop(c, "slice").unwrap_or_default(),
                            })
                            .collect(),
                        usage: node
                            .iter_children()
                            .filter(|c| c.name().value() == "usage")
                            .filter_map(string_arg)
                            .collect(),
                        doing: child_arg(node, "doing").unwrap_or_default(),
                        not: child_arg(node, "not").unwrap_or_default(),
                        keeps_consistent: node
                            .iter_children()
                            .filter(|c| c.name().value() == "keeps-consistent")
                            .flat_map(string_args)
                            .collect(),
                    }),
                    "event-storm" => {
                        for child in node.iter_children() {
                            if child.name().value() == "read-model"
                                && let Some(name) = string_arg(child)
                            {
                                corpus.views.push(View {
                                    name,
                                    answers: prop(child, "answers").unwrap_or_default(),
                                    publishable: child
                                        .get("publishable")
                                        .and_then(|v| v.as_bool()),
                                    because: prop(child, "because"),
                                    publishes_to: prop(child, "publishes-to"),
                                    needed_by: crate::schema::all_props(child, "needed-by"),
                                    parts: child
                                        .iter_children()
                                        .filter(|p| p.name().value() == "part")
                                        .map(|p| Part {
                                            label: string_arg(p).unwrap_or_default(),
                                            is: prop(p, "is").unwrap_or_default(),
                                            from: prop(p, "from").unwrap_or_default(),
                                        })
                                        .collect(),
                                });
                            }
                        }
                    }
                    "frame" => {
                        corpus.frames.push(Frame {
                            id: string_arg(node).unwrap_or_default(),
                            title: child_arg(node, "title").unwrap_or_default(),
                            root_cause: child_arg(node, "root-cause").unwrap_or_default(),
                            principle: child_arg(node, "principle").unwrap_or_default(),
                        });
                        for child in node.iter_children().filter(|c| c.name().value() == "symptom")
                        {
                            if let Some(id) = string_arg(child) {
                                corpus.symptoms.push(Symptom {
                                    id,
                                    state: prop(child, "state").unwrap_or_default(),
                                    resolved_by: prop(child, "resolved-by"),
                                    text: prop(child, "text").unwrap_or_default(),
                                    seal: prop(child, "seal"),
                                });
                            }
                        }
                    }
                    "doctrine-surface" => corpus.surfaces.push(crate::surface::Surface {
                        id: string_arg(node).unwrap_or_default(),
                        path: child_arg(node, "path").unwrap_or_default(),
                        kind: child_arg(node, "kind").unwrap_or_default(),
                        // Every value of every node: `serves "A" "B"` is one node carrying
                        // two, and reading the first of each is AB1 (ITER.260821.12).
                        serves: node
                            .iter_children()
                            .filter(|c| c.name().value() == "serves")
                            .flat_map(string_args)
                            .collect(),
                        state: child_arg(node, "state")
                            .unwrap_or_else(|| "current".to_owned()),
                        stands_at: child_arg(node, "stands-at"),
                    }),
                    "invariant" => corpus.invariants.push(crate::invariant::Invariant {
                        id: string_arg(node).unwrap_or_default(),
                        protects: child_arg(node, "protects").unwrap_or_default(),
                        severity: child_arg(node, "severity").unwrap_or_default(),
                        languages: node
                            .iter_children()
                            .filter(|c| c.name().value() == "language")
                            .flat_map(string_args)
                            .collect(),
                        structural: child_arg(node, "structural"),
                    }),
                    "role" => corpus.roles.push(Role {
                        id: string_arg(node).unwrap_or_default(),
                        owns_phase: node
                            .iter_children()
                            .filter(|c| c.name().value() == "owns-phase")
                            .flat_map(string_args)
                            .collect(),
                        never_for_own: node
                            .iter_children()
                            .filter(|c| c.name().value() == "never-for-own")
                            .flat_map(string_args)
                            .collect(),
                    }),
                    "vision" => {
                        corpus.anchor.vision = child_arg(node, "is").unwrap_or_default();
                    }
                    "mission" => {
                        corpus.anchor.mission = child_arg(node, "is").unwrap_or_default();
                        corpus.anchor.delivered_when =
                            child_arg(node, "delivered-when").unwrap_or_default();
                    }
                    "strategy" => {
                        corpus.anchor.strategy = node
                            .iter_children()
                            .filter(|c| c.name().value() == "step")
                            .filter_map(|c| prop(c, "is"))
                            .collect();
                    }
                    "config" => corpus.config = config_from(node),
                    _ => {}
                }
            }
        }
        corpus.slices.sort_by(|a, b| a.id.cmp(&b.id));
        corpus.vocabulary = vocabulary_in_dependency_order(schema);
        corpus
    }

}

/// The declared kinds, each with what it IS, ordered so nothing is defined before the words
/// it is defined in terms of.
///
/// `TS.260821.14`/C4. Two edges, and they run opposite ways for a reason:
///
/// - a kind that **holds** another comes first — a container introduces what it contains,
///   which is how a reader meets `symptom` inside `frame` rather than out of nowhere;
/// - a kind another **references** comes first — you cannot read `realizes "CAP.x"` until
///   you know what a capability is.
///
/// Together they put `frame` before `symptom` before `thin-slice` before `iteration`, which
/// is the order somebody learns them in. Cycles are real in this graph and are not an
/// error: a kind this cannot place keeps its declared position, so the worst case is the
/// order the schema was written in.
fn vocabulary_in_dependency_order(schema: &Schema) -> Vec<(String, String)> {
    let kinds: Vec<String> = schema.kinds().map(str::to_owned).collect();
    let mut needs: Vec<(String, Vec<String>)> = Vec::new();
    for kind in &kinds {
        let Some(spec) = schema.entity(kind) else { continue };
        let mut before: Vec<String> = Vec::new();
        for field in &spec.fields {
            // `references` points across the graph: the target has to be known first.
            if let Some(target) = &field.references {
                for one in target.split_whitespace() {
                    if kinds.iter().any(|k| k == one) && !before.contains(&one.to_owned()) {
                        before.push(one.to_owned());
                    }
                }
            }
        }
        needs.push((kind.clone(), before));
    }
    // `holds` runs the other way: the HOLDER is introduced first, so what it contains
    // depends on it rather than the reverse.
    for kind in &kinds {
        let Some(spec) = schema.entity(kind) else { continue };
        for field in &spec.fields {
            let Some(held) = &field.holds else { continue };
            if let Some(slot) = needs.iter_mut().find(|(k, _)| k == held)
                && !slot.1.contains(kind)
            {
                slot.1.push(kind.clone());
            }
        }
    }

    let mut placed: Vec<String> = Vec::new();
    // One pass per kind is enough to settle any acyclic chain, and a cycle stops moving
    // rather than looping: the pass that places nothing new is the last one.
    for _ in 0..kinds.len() {
        let before = placed.len();
        for (kind, needed) in &needs {
            if placed.contains(kind) {
                continue;
            }
            if needed.iter().all(|n| placed.contains(n) || !kinds.iter().any(|k| k == n)) {
                placed.push(kind.clone());
            }
        }
        if placed.len() == before {
            break;
        }
    }
    // Whatever a cycle left behind, in the order the schema declares it.
    for kind in &kinds {
        if !placed.contains(kind) {
            placed.push(kind.clone());
        }
    }

    placed
        .into_iter()
        .filter_map(|kind| schema.entity(&kind).and_then(|s| s.is.clone()).map(|is| (kind, is)))
        .collect()
}

impl Corpus {
    pub fn slice(&self, id: &str) -> Option<&Slice> {
        self.slices.iter().find(|s| s.id == id)
    }

    /// Whether a slice is delivered — every claim IT declares met by some iteration.
    ///
    /// Measured against the slice's own claims rather than against one iteration's
    /// accounting, for two reasons. A closed iteration that met everything it took on has
    /// not delivered a slice it only partly attempted. And a claim whose settlement
    /// depends on work downstream of its own slice can be settled later, by the iteration
    /// that finally had what settling it required (ITER.260821.03/S1).
    pub fn delivered(&self, id: &str) -> bool {
        let Some(slice) = self.slice(id) else {
            return false;
        };
        !slice.claims.is_empty()
            && slice.claims.iter().all(|claim| {
                self.attempts.iter().any(|a| {
                    a.state == "closed"
                        && a.claims
                            .iter()
                            .any(|c| c.id == *claim && c.from == id && c.state == "met")
                })
            })
    }

    /// Which iteration settled the last of a slice's claims. What a reader wants next
    /// after being told something is done is who did it.
    pub fn delivered_by(&self, id: &str) -> Option<&str> {
        if !self.delivered(id) {
            return None;
        }
        self.attempts
            .iter()
            .filter(|a| a.claims.iter().any(|c| c.from == id && c.state == "met"))
            .map(|a| a.id.as_str())
            .next_back()
    }
}

fn slice_from(node: &KdlNode) -> Slice {
    Slice {
        id: string_arg(node).unwrap_or_default(),
        slug: child_arg(node, "slug").unwrap_or_default(),
        kind: child_arg(node, "kind").unwrap_or_default(),
        realizes: child_arg(node, "realizes").unwrap_or_default(),
        layers: child_args(node, "layer"),
        depends_on: child_args(node, "depends-on"),
        claims: child_args(node, "claim"),
        attacks: node
            .iter_children()
            .filter(|c| c.name().value() == "attacks")
            .flat_map(string_args)
            .collect(),
        declared_state: child_arg(node, "state"),
        title: child_arg(node, "title").unwrap_or_default(),
        trigger: child_arg(node, "trigger").unwrap_or_default(),
        outcome: child_arg(node, "outcome").unwrap_or_default(),
        useful_alone: child_arg(node, "useful-alone"),
        command: child_arg(node, "command"),
    }
}

fn attempt_from(node: &KdlNode) -> Attempt {
    Attempt {
        id: string_arg(node).unwrap_or_default(),
        on_slices: child_args(node, "on-slice"),
        state: child_arg(node, "state").unwrap_or_default(),
        claims: node
            .iter_children()
            .filter(|c| c.name().value() == "claim")
            .map(|c| Settled {
                id: string_arg(c).unwrap_or_default(),
                from: prop(c, "from-slice").unwrap_or_default(),
                state: prop(c, "state").unwrap_or_default(),
            })
            .collect(),
        contributes: child_args(node, "contributes"),
        layers: node
            .iter_children()
            .filter(|c| c.name().value() == "layer")
            .map(|c| {
                (
                    string_arg(c).unwrap_or_default(),
                    prop(c, "state").unwrap_or_default(),
                    prop(c, "evidence").unwrap_or_default(),
                )
            })
            .collect(),
        phases: node
            .iter_children()
            .filter(|c| c.name().value() == "phase")
            .map(|c| Phase {
                kind: string_arg(c).unwrap_or_default(),
                state: prop(c, "state").unwrap_or_default(),
                produced: prop(c, "produced").or_else(|| prop(c, "because")).unwrap_or_default(),
                worked_by: prop(c, "worked-by"),
            })
            .collect(),
        findings: node
            .iter_children()
            .filter(|c| c.name().value() == "finding")
            .map(|c| Carried {
                id: string_arg(c).unwrap_or_default(),
                text: prop(c, "text").unwrap_or_default(),
                carries: prop(c, "carries"),
                tests: prop(c, "tests"),
            })
            .collect(),
    }
}

fn release_from(node: &KdlNode) -> Release {
    Release {
        id: string_arg(node).unwrap_or_default(),
        version: child_arg(node, "version").unwrap_or_default(),
        state: child_arg(node, "state").unwrap_or_default(),
        binds: child_args(node, "binds"),
        proposed_bump: child_arg(node, "proposed-bump").unwrap_or_default(),
    }
}

fn config_from(node: &KdlNode) -> Config {
    let mut config = Config::default();
    for versioning in node.iter_children().filter(|c| c.name().value() == "versioning") {
        config.scheme = child_arg(versioning, "scheme").unwrap_or_default();
        for proposal in versioning.iter_children().filter(|c| c.name().value() == "bump-proposal") {
            for rule in proposal.iter_children() {
                if let Some(position) = string_arg(rule) {
                    config.bump_rules.push((rule.name().value().to_owned(), position));
                }
            }
        }
    }
    for profile in node.iter_children().filter(|c| c.name().value() == "profile") {
        for omit in profile.iter_children().filter(|c| c.name().value() == "omit-probe") {
            config.omitted.extend(string_args(omit));
        }
    }
    config
}

impl Corpus {
    /// The release a given iteration is bound to, if any. An iteration binds exactly once.
    pub fn bound_to(&self, iteration: &str) -> Option<&Release> {
        self.releases.iter().find(|r| r.binds.iter().any(|b| b == iteration))
    }

    /// Symptoms naming this release as what resolved them. Derived from the symptoms,
    /// never stored beside them: a second copy whose fidelity is unverifiable without
    /// checking the first is what this frame is about.
    pub fn resolved_by(&self, version: &str) -> Vec<&str> {
        self.symptoms
            .iter()
            .filter(|s| s.resolved_by.as_deref() == Some(version))
            .map(|s| s.id.as_str())
            .collect()
    }

    pub fn release(&self, version: &str) -> Option<&Release> {
        self.releases.iter().find(|r| r.version == version)
    }
}

fn child_arg(node: &KdlNode, name: &str) -> Option<String> {
    node.iter_children()
        .find(|c| c.name().value() == name)
        .and_then(string_arg)
}

fn child_args(node: &KdlNode, name: &str) -> Vec<String> {
    node.iter_children()
        .filter(|c| c.name().value() == name)
        .filter_map(string_arg)
        .collect()
}

/// Every declared condition's verdict on every slice, plus what the record says about
/// itself that the derivation contradicts.
#[derive(Debug, Clone)]
pub struct Assessment {
    pub conditions: Conditions,
    pub verdicts: BTreeMap<String, Vec<(String, Verdict)>>,
    /// Where a slice's own `state` disagrees with what its attempts show. The fitness
    /// function `no-drift-between-view-and-record`, computed rather than hoped for.
    pub drift: Vec<(String, String)>,
    pub as_of: String,
}

impl Assessment {
    /// The slices nothing blocks. This is the set the gate admits — the view is a
    /// projection of it and computes no condition of its own (TS.260820.04/C1).
    pub fn ready(&self) -> Vec<&str> {
        self.verdicts
            .iter()
            .filter(|(_, vs)| !vs.iter().any(|(_, v)| v.blocks()))
            .map(|(id, _)| id.as_str())
            .collect()
    }

    /// Whether a slice is blocked only by having nothing left to admit — done, rather
    /// than stuck. A reader who cannot tell those apart is being misled.
    pub fn finished(&self, id: &str) -> bool {
        self.verdicts.get(id).is_some_and(|vs| {
            vs.iter()
                .filter(|(_, v)| v.blocks())
                .all(|(c, _)| c == "nothing-left-to-admit")
                && vs.iter().any(|(c, v)| c == "nothing-left-to-admit" && v.blocks())
        })
    }
}

/// Evaluate every declared condition against every slice.
pub fn assess(corpus: &Corpus, conditions: &Conditions, as_of: &str) -> Assessment {
    let mut verdicts = BTreeMap::new();
    for slice in &corpus.slices {
        let per: Vec<(String, Verdict)> = conditions
            .all
            .iter()
            .map(|c| (c.name.clone(), decide(c, slice, corpus)))
            .collect();
        verdicts.insert(slice.id.clone(), per);
    }

    let mut drift = Vec::new();
    for slice in &corpus.slices {
        let derived = corpus.delivered(&slice.id);
        match (slice.declared_state.as_deref(), derived) {
            (Some("delivered"), false) => drift.push((
                slice.id.clone(),
                "declares `state delivered`, and no closed iteration on it has all claims met"
                    .to_owned(),
            )),
            // Silence is NOT drift. Delivery is derived from the iterations, so a slice
            // that says nothing is deferring to the derivation, which is correct. Only a
            // slice that says something ELSE is contradicting it.
            (Some(state), true) if state != "delivered" => drift.push((
                slice.id.clone(),
                format!("a closed iteration met every claim on it, and the slice says `state {state}`"),
            )),
            _ => {}
        }
    }

    Assessment { conditions: conditions.clone(), verdicts, drift, as_of: as_of.to_owned() }
}

/// The check registry. A declared condition naming a check that is not here is
/// `Uncomputed` — never a pass.
fn decide(condition: &Condition, slice: &Slice, corpus: &Corpus) -> Verdict {
    if condition.judgement {
        return Verdict::Judgement;
    }
    if let Some(why) = &condition.uncomputed {
        return Verdict::Uncomputed(why.clone());
    }
    let Some(check) = condition.check.as_deref() else {
        return Verdict::Uncomputed(
            "the record declares this condition and names no check for it".to_owned(),
        );
    };
    match check {
        "slice-is-shaped" => {
            if corpus.refused.contains(&slice.id) {
                Verdict::Blocks("the shape check refuses this slice".to_owned())
            } else {
                Verdict::Admits
            }
        }
        "dependencies-delivered" => {
            // A dependency naming nothing the record holds blocks too, and says so
            // differently: an unmet dependency is waiting, a dangling one is a defect.
            let mut missing = Vec::new();
            let mut unmet = Vec::new();
            for id in slice.depends_on.iter().flat_map(|d| named_slices(d)) {
                if corpus.slice(&id).is_none() {
                    missing.push(id);
                } else if !corpus.delivered(&id) {
                    unmet.push(id);
                }
            }
            if !missing.is_empty() {
                return Verdict::Blocks(format!(
                    "{} names no slice the record holds",
                    missing.join(", ")
                ));
            }
            match unmet.split_first() {
                None => Verdict::Admits,
                Some((first, rest)) if rest.is_empty() => {
                    Verdict::Blocks(format!("{first} is not delivered"))
                }
                Some(_) => Verdict::Blocks(format!("{} are not delivered", unmet.join(", "))),
            }
        }
        "no-iteration-in-flight" => match corpus
            .attempts
            .iter()
            .find(|a| a.on_slices.contains(&slice.id) && a.in_flight())
        {
            Some(a) => Verdict::Blocks(format!("{} is {} on this slice", a.id, a.state)),
            None => Verdict::Admits,
        },
        "nothing-left-to-admit" => match corpus.delivered_by(&slice.id) {
            Some(by) => Verdict::Blocks(format!("delivered by {by}")),
            None => Verdict::Admits,
        },
        other => Verdict::Uncomputed(format!(
            "the record declares check {other:?} and the engine carries no such check"
        )),
    }
}

/// Slice ids named inside a dependency's prose. P2: `depends-on` should name a frozen
/// seam, and every one in the tree today names a slice in a sentence. Reading the id out
/// is what can be done until seams are the unit — and the answer says that it did.
pub fn named_slices(prose: &str) -> Vec<String> {
    let chars: Vec<char> = prose.chars().collect();
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i..].starts_with(&['T', 'S', '.']) {
            let mut j = i + 3;
            while j < chars.len() && (chars[j].is_alphanumeric() || chars[j] == '.' || chars[j] == '-')
            {
                j += 1;
            }
            let id: String = chars[i..j].iter().collect();
            let id = id.trim_end_matches(['.', '-']).to_owned();
            if id.len() > 3 && !out.contains(&id) {
                out.push(id);
            }
            i = j;
        } else {
            i += 1;
        }
    }
    out
}

/// Project the assessment as `read-model@v1`. This composes; it decides nothing — every
/// verdict here was computed by [`assess`], which the gate calls too.
pub fn project(assessment: &Assessment, corpus: &Corpus) -> ReadModel {
    let mut model = ReadModel::new(
        "what-is-ready-to-pick-up",
        "which slices could be started right now, and what would refuse each of them?",
        &assessment.as_of,
    );

    // Prose a human wrote, referenced by condition name rather than inlined into a row.
    for condition in &assessment.conditions.all {
        if let Some(because) = &condition.because {
            model = model.define(&condition.name, because);
        }
        if let Some(caveat) = &condition.caveat {
            model = model.define(format!("{} · caveat", condition.name), caveat);
        }
    }

    let mut ready = Section::new("ready", &["slice", "slug", "kind", "capability", "layers"])
        .empty_because("every slice is blocked, delivered, or refused by the shape check");
    let mut blocked = Section::new("blocked", &["slice", "condition", "what blocks it"])
        .empty_because("no declared condition blocks any slice");
    let mut done = Section::new("delivered", &["slice", "by"])
        .empty_because("no slice has a closed iteration with every claim met");
    let mut undecided = Section::new("not decided for you", &["condition", "why", "slices"])
        .empty_because("every declared condition was computed for every slice");
    let mut drift = Section::new("record disagrees with itself", &["slice", "disagreement"])
        .empty_because("no slice's own state contradicts what its iterations show");

    for slice in &corpus.slices {
        let Some(verdicts) = assessment.verdicts.get(&slice.id) else {
            continue;
        };
        if assessment.finished(&slice.id) {
            let by = verdicts
                .iter()
                .find(|(c, _)| c == "nothing-left-to-admit")
                .map_or(String::new(), |(_, v)| v.detail().to_owned());
            done.push(vec![slice.id.clone(), by]);
            continue;
        }
        let blocking: Vec<&(String, Verdict)> =
            verdicts.iter().filter(|(_, v)| v.blocks()).collect();
        if blocking.is_empty() {
            ready.push(vec![
                slice.id.clone(),
                slice.slug.clone(),
                slice.kind.clone(),
                slice.realizes.clone(),
                slice.layers.len().to_string(),
            ]);
        } else {
            for (condition, verdict) in blocking {
                blocked.push(vec![
                    slice.id.clone(),
                    condition.clone(),
                    verdict.detail().to_owned(),
                ]);
            }
        }
    }

    // Conditions nobody decided. Named once, with every slice they left undecided —
    // because a reader must not mistake "nothing is blocked" for "blocking was not
    // computed", and this section is the entire difference between them.
    for condition in &assessment.conditions.all {
        let affected: Vec<&str> = assessment
            .verdicts
            .iter()
            .filter(|(_, vs)| {
                vs.iter().any(|(c, v)| {
                    c == &condition.name && matches!(v, Verdict::Uncomputed(_) | Verdict::Judgement)
                })
            })
            .map(|(id, _)| id.as_str())
            .collect();
        if affected.is_empty() {
            continue;
        }
        let why = if condition.judgement {
            "left to the maintainer — the tool shows, it does not rank".to_owned()
        } else {
            condition
                .uncomputed
                .clone()
                .unwrap_or_else(|| "no check answers this".to_owned())
        };
        undecided.push(vec![condition.name.clone(), why, affected.len().to_string()]);
    }

    for (id, what) in &assessment.drift {
        drift.push(vec![id.clone(), what.clone()]);
    }

    model
        .section(ready)
        .section(blocked)
        .section(done)
        .section(undecided)
        .section(drift)
}
