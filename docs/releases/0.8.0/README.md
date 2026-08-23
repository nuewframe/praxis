# Where to start

what is this, what can it do, and where do I begin?

> Depicts **0.8.0**, and nothing else. Regenerated whole from
> the record; never edited in place.

## what is in here

|  | it answers | for |
| --- | --- | --- |
| why this exists | what problem does this exist for, and what does every word mean? | the-reader-of-a-release · the-next-agent |
| what it can do | what must the system be able to do, and which events does each keep consistent? | the-team-building-a-digital-product |
| how to start | how do I use what this version can do? | the-reader-of-a-release |
| what changed | what shipped at version N, which symptoms it resolved, and what it left owed? | the-reader-of-a-release |

## addressed to nobody

_Nothing here — every part of this story names who it is for._

## What this product means

what problem does this exist for, and what does every word mean?

### why this exists

|  |  |
| --- | --- |
| the world if this succeeds | a team can trust what an agent produced without re-deriving it |
| what this does about it | make execution fidelity a property computed from the record, rather than a claim made by whoever produced the artifact |
| what would count as delivered | somebody who did not do the work can ask the record what happened and act on the answer, without asking the person who did it and without reading a document that argues it went well |

### the problem this exists for

|  |  |
| --- | --- |
| what is wrong | Trust in an agent's artifact is unearned, because fidelity is invisible |
| why it is wrong | execution fidelity is invisible in the artifact, so trust in it must be granted rather than computed |
| what follows from that | fidelity is a property computed from the record, never a claim made by whoever produced it |

### how it shows up

|  | still present? |
| --- | --- |
| agents improvise structure instead of following a method — narrowed by guardrails and skills, not closed | yes |
| an artifact looks identical whether the agent reasoned hard or pattern-matched a template | yes |
| how much process a unit of work received is declared by the agent that received it | yes |
| a slice claimed vertical cannot be shown to have cut anything | yes |
| scope drifts silently — work closes with unmet claims and nothing objects | yes |
| a published document carries no evidence of when, or whether, it was ever true | yes |
| the next agent re-derives what is already known, because current truth is prose | yes |

### what the words mean

| term | meaning |
| --- | --- |
| adr | a durable technical decision, its argument, its alternatives, and its corrections |
| invariant | one property the plugin GUARANTEES about code it is loaded into |
| method | the vocabulary a repository is governed BY — kinds, rules, and the gate |
| persona | somebody the product is for — what they came for, and how they judge whether they got it |
| vision | the world if this succeeds |
| mission | what this product does to move toward the vision |
| frame | a problem — symptoms, one root cause, one principle |
| symptom | one observable thing that is wrong FOR SOMEBODY, resolving on its own schedule by naming a release |
| event-storm | one sweep of a domain's timeline — the evidence a capability is derived FROM |
| read-model | a view an actor must see before acting, owned by exactly one capability |
| cluster | a group of events that must stay consistent together — the unit a capability is derived FROM |
| capability | a permanent doing the system must have |
| thin-slice | an atomic vertical slice — one command or one read model, cut so it can exercise a capability |
| doctrine-surface | one file this plugin SHIPS as instruction — a skill, a guardrail, an agent, or a probe |
| iteration | one COMMITMENT — the slices an ask committed to, worked together |
| refusal | a pick-up the gate refused, and the condition that failed |
| release | an index into history — what it binds, what it resolves |
| decision | a choice an iteration could not make implicitly, with what it rejected and what would show it wrong |

### In detail

#### depicts

0.8.0

## Capabilities and what they own

what must the system be able to do, and which events does each keep consistent?

### what the product can do

| capability | derived from | events owned |
| --- | --- | --- |
| conformance-probes | evidence-and-settlement | 2 |
| multi-harness-distribution | projection | 3 |

### how it is built

| capability | derived from | events owned |
| --- | --- | --- |
| claim-settlement | evidence-and-settlement | 7 |
| delivery-record | problem-record | 19 |
| document-projection | projection | 3 |
| release-binding | release-binding | 3 |
| work-admission | iteration-control | 7 |

### what each owns

| capability | event |
| --- | --- |
| claim-settlement | EvidenceAttached |
| claim-settlement | ClaimSettled |
| claim-settlement | FindingRaised |
| claim-settlement | FindingTriaged |
| claim-settlement | DecisionRecorded |
| claim-settlement | DecisionAccepted |
| claim-settlement | AcceptanceRefused |
| conformance-probes | InvariantEnforced |
| conformance-probes | InvariantUnenforced |
| delivery-record | IntentRaised |
| delivery-record | SymptomRecorded |
| delivery-record | RootCauseConverged |
| delivery-record | ConvergenceRefused |
| delivery-record | SymptomResolved |
| delivery-record | FrameReopened |
| delivery-record | StormHeld |
| delivery-record | PathWalked |
| delivery-record | OptionRejected |
| delivery-record | CapabilityNamed |
| delivery-record | SliceCut |
| delivery-record | SliceCutRefused |
| delivery-record | ClaimStated |
| delivery-record | LifetimeDeclared |
| delivery-record | UsageDocumented |
| delivery-record | EntityDeclared |
| delivery-record | EntityDeclarationRefused |
| delivery-record | ClaimWithdrawn |
| delivery-record | RecordAmended |
| document-projection | DocumentsProjected |
| document-projection | DocumentStamped |
| document-projection | DriftDetected |
| multi-harness-distribution | RouterInjected |
| multi-harness-distribution | ManifestPublished |
| multi-harness-distribution | OverlayProvisioned |
| release-binding | WorkBound |
| release-binding | ReleaseCut |
| release-binding | TruthPromoted |
| work-admission | SliceVetted |
| work-admission | PickupRefused |
| work-admission | DependencyFrozen |
| work-admission | IterationClosed |
| work-admission | CloseRefused |
| work-admission | IterationStarted |
| work-admission | StartRefused |

### In detail

#### depicts

0.8.0

## How to use a capability

how do I use what 0.8.0 can do?

### how to use what shipped

| capability |
| --- |
| claim-settlement |
| delivery-record |
| document-projection |
| release-binding |
| work-admission |

### shipped without a guide

_Nothing here — every capability this version bound work for carries usage prose._

### written, not yet shipped

| capability | why it is not here |
| --- | --- |
| conformance-probes | 0.8.0 bound no slice realizing it |

### In detail

#### claim-settlement

`praxis close <ITER> --outcome continue|pivot|stop` — every claim settled, or every shortfall carried by a finding that NAMES it, or the close is refused.

`praxis review <ITER> [--markdown]` — preview what an iteration promised against what it has shown, at any moment, writing nothing. Layers the slice declared and nothing reached are listed, not omitted.

#### delivery-record

`praxis check [root]` — refuse any record that does not match the shape the record itself declares.

`praxis truth` — what this repository already knows, so an arriving agent does not re-derive it. Read the last section: it names what the answer does not cover.

Adopting on a repository that already shipped: the record is built NEW, and what is already there is discovery data. Capability docs, ADRs and release history are stormed, walked and argued with — a claim transcribed is a claim imported without its evidence, and a record that looks checked from its first commit is worth less than an empty one.

`what did this come from` is answered by the record's own derivation, never by a pointer at the document it was read from. A capability names the cluster it came from and the storm that cluster is in; a slice names its storm; a decision names what it rejected and what would show it wrong. Every one of those is an id the checker follows and refuses when it dangles. There is deliberately no `informed-by`: a path is the half guaranteed not to survive, and a dead pointer outlives the file as a confident lie.

#### document-projection

`praxis publish <VERSION> [--dry-run]` — regenerate the published set whole, before the cut. Membership comes from the lifetime declarations and the paths come with it.

`praxis verify-published` — prove no published document was hand-edited, comparing each release's directory against the commit its index names.

#### release-binding

`praxis bind <ITER> <VERSION> [--undo]` — attach closed work to a version. Only closed iterations bind, each binds once, and a cut release does not change.

`praxis cut-release <VERSION> --confirm` — cut the version and write its index node in one record. Refused until the published set exists and is committed.

`praxis promote <VERSION>` — fold what a cut release shipped into what each capability says it is. Derived from bound work and recomputed by a rule.

#### work-admission

`praxis ready` — which slices could be started right now, and what would refuse each of the rest. Read the `not decided for you` section first: it names the conditions the gate declares and did not compute.

`praxis pick-up <SLICE>` — take a slice through the one gate. Either an iteration opens, or a refusal record is written naming the condition that failed. Selection is the commitment.

## The published set for a release

what can you do at 0.8.0 that you could not before, and what is still missing?

### what you can do now

| what changed |
| --- |
| Cut a slice whose verticality can be refused rather than asserted |
| Refuse a record whose kind nothing declares, and say what to declare |
| Name a capability only where a storm's events cluster |
| See which slices could be started right now, and what would refuse each of the rest |
| Take a slice, and have the refusal recorded when it is refused |
| Back a layer's claim with an artifact instead of the agent's word for it |
| Refuse to close an iteration that is about to drop scope in silence |
| Attach finished iterations to a version, and refuse anything that did not close |
| Cut the version, and record the index that points into git |
| Regenerate every published document whole, each stating the release it depicts |
| Prove a published document was never hand-edited after the fact |
| Fold what shipped into what each capability says it is, in one operation |
| Ask the record what this repository already knows |
| Review what an iteration has actually shown, writing nothing to the published record |
| Decide what belongs in the published set, and refuse a choice made without a reason |
| Bind a decision to the work that forced it, and correct it only by appending |
| Write usage while building, and refuse to publish a guide for truth that never shipped |
| Close a symptom by naming a release that demonstrably attacked it |
| See where the product stands right now, without committing a document that ages |
| Report any declared rule that nothing in the record demonstrates refusing |
| Report any doctrine this plugin ships that no record entity justifies |
| Take the unanchored count to zero, and close the rule behind it |
| Hold the engineering invariant as a record kind, so a probe can name what it enforces |
| Publish a capability set that describes what the reader installed |
| Publish the shipped doctrine for a version, derived from the record rather than scraped |
| Refuse a close whose reviewer is the agent that did the work |
| Refuse a close whose attester the tool supplied rather than a person |
| Ship the method's vocabulary with the engine, so adopting is not forking |

### how to start

| command | when you reach for it |
| --- | --- |
| praxis cut-slice | a capability exists and a piece of it is worth doing before the rest |
| praxis declare-entity | the model gained a kind of thing the record does not yet know about |
| praxis name-capability | discovery has swept the domain and the events are grouped |
| praxis pick-up | an agent has chosen a slice and is about to start |
| praxis attach-evidence | a layer declared by the slice has been reached |
| praxis close-iteration | the agent believes the iteration is finished |
| praxis bind | an iteration has closed and belongs in the next version |
| praxis cut-release | everything intended for the version is bound and the version is going out |
| praxis publish | everything intended for the version is bound, and it is about to be cut |
| praxis verify-published | git reports a push |
| praxis promote-truth | a release has been cut and what each capability says about itself is now behind it |
| praxis declare-lifetime | a view exists and nobody has said whether it survives being frozen |
| praxis record-decision | an iteration hits a choice it cannot make implicitly |
| praxis document-usage | a capability gained a usable surface during an iteration |
| praxis resolve-symptom | a release shipped work that was supposed to remove a symptom |
| praxis prove-rules | the record declares a rule, and nobody knows whether it can fire |
| praxis audit-surfaces | the plugin ships a skill, a guardrail, an agent, or a probe, and nobody can say what in the record asks for it |
| praxis check-invariants | a probe fails, and what it was protecting is written only in its own header comment |
| praxis close | an iteration closes, and the only thing standing between it and self-attestation is a sentence in a Markdown file |
| praxis close | an iteration closes and the only identity on the close is the one the tool wrote |
| praxis check | a new project runs praxis check and is told the record declares no schema |

### what is known to be missing

| finding | against claim |
| --- | --- |
| S1 | C1 |
| X1 | C3 |
| Y2 | none |
| AG3 | C3 |
| AM1 | C1 |
| AM4 | C3 |

### problems this version attacked

_Nothing here — no symptom names this version as what resolved it._

### appendix — what bound to this version

| iteration | slice |
| --- | --- |
| ITER.260820.01 | TS.260820.01 |
| ITER.260821.01 | TS.260821.01 |
| ITER.260821.02 | TS.260820.02 |
| ITER.260821.03 | TS.260820.04 |
| ITER.260821.04 | TS.260820.05 |
| ITER.260821.05 | TS.260820.06 |
| ITER.260821.06 | TS.260820.07 |
| ITER.260821.07 | TS.260820.16 |
| ITER.260821.08 | TS.260820.09 |
| ITER.260821.09 | TS.260820.10 |
| ITER.260821.10 | TS.260820.11 |
| ITER.260821.11 | TS.260820.17 |
| ITER.260821.12 | TS.260820.03 |
| ITER.260821.13 | TS.260820.08 |
| ITER.260821.14 | TS.260820.12 |
| ITER.260821.15 | TS.260820.14 |
| ITER.260821.16 | TS.260820.15 |
| ITER.260821.17 | TS.260820.18 |
| ITER.260821.18 | TS.260820.13 |
| ITER.260821.19 | TS.260821.02 · TS.260820.18 |
| ITER.260822.01 | TS.260821.03 |
| ITER.260822.02 | TS.260821.04 |
| ITER.260822.03 | TS.260821.05 · TS.260821.06 |
| ITER.260822.04 | TS.260821.04 · TS.260821.07 |
| ITER.260822.05 | TS.260821.08 |
| ITER.260822.06 | TS.260821.09 |
| ITER.260822.07 | TS.260821.10 |

### In detail

#### Cut a slice whose verticality can be refused rather than asserted

verticality stops being a word the author applies to their own work. That is S4 answered, with nothing else in this frame delivered

#### Refuse a record whose kind nothing declares, and say what to declare

the record stops being able to grow a kind of thing in silence. Nothing else in this frame needs to ship for that to be worth having

#### Name a capability only where a storm's events cluster

the step from problem to capability becomes a derivation with its work shown. Nothing downstream needs to exist for that to be worth having

#### See which slices could be started right now, and what would refuse each of the rest

an agent stops choosing work by reading prose and guessing. Useful with no gate implemented at all, because the answer is already true

#### Take a slice, and have the refusal recorded when it is refused

the amount of process a unit of work received stops being the receiving agent's own account of it

#### Back a layer's claim with an artifact instead of the agent's word for it

how much of a slice was actually reached becomes readable off the record, with nothing else about closing or releasing in place

#### Refuse to close an iteration that is about to drop scope in silence

scope stops drifting silently. This is S5 closed on its own, and it is the symptom with the most day-to-day cost

#### Attach finished iterations to a version, and refuse anything that did not close

what is queued for the next version stops being a hand-kept list, and that is useful with nothing ever cut

#### Cut the version, and record the index that points into git

there is a point on the version line that everything else can be pinned to. Nothing in publishing, verification or symptom resolution is possible before it

#### Regenerate every published document whole, each stating the release it depicts

the regex ceiling is gone for published output, and every document becomes true for exactly one version and false for every other

#### Prove a published document was never hand-edited after the fact

a reader asking 'was this ever true?' gets a date rather than a shrug. S6 fully closed

#### Fold what shipped into what each capability says it is, in one operation

current truth stops being maintained by hand. This is the difference between the record being a by-product of the work and a chore performed beside it, and the frame says that difference is what makes the chore get skipped when the work is late

#### Ask the record what this repository already knows

S7 stops being structural. An agent that can ask does not re-derive, even with no projection, no gate and no release machinery present

#### Review what an iteration has actually shown, writing nothing to the published record

a reviewer stops judging an artifact by how finished it looks. That is S2, and it needs no release machinery

#### Decide what belongs in the published set, and refuse a choice made without a reason

the difference between a document worth freezing and a dashboard stops being decided per-release by whoever is publishing that day

#### Bind a decision to the work that forced it, and correct it only by appending

a reader stops asking why something is the way it is and getting an answer reconstructed from memory. Worth having with nothing published at all

#### Write usage while building, and refuse to publish a guide for truth that never shipped

usage stops being written from memory at release time, which is when it is least accurate and most rushed

#### Close a symptom by naming a release that demonstrably attacked it

`resolved-by` becomes checkable. Every frame in the repository can be audited against what actually shipped, with no projection and no publishing

#### See where the product stands right now, without committing a document that ages

the Markdown dashboard can be retired without losing anything, because the answer it was approximating is now available whenever anyone asks for it

#### Report any declared rule that nothing in the record demonstrates refusing

`which of our gates has never fired` becomes answerable. Worth having with nothing else changed, because the answer today is that nobody knows

#### Report any doctrine this plugin ships that no record entity justifies

`which of the things we ship does the record still ask for` becomes answerable, and the answer on the first run is that twenty-five of forty-seven skills have no anchor. Worth having with nothing deleted, because today that number cannot be produced at all

#### Take the unanchored count to zero, and close the rule behind it

the front door stops contradicting the house. An arriving agent reads a spine it can execute, which is the first thing every session does and currently the first thing that misleads it

#### Hold the engineering invariant as a record kind, so a probe can name what it enforces

`what does this plugin guarantee, and what enforces it` becomes answerable. Today `enable-all-fail-closed #true` is a claim about fourteen shell scripts that nothing has ever checked

#### Publish a capability set that describes what the reader installed

`what can this version of Praxis do` becomes answerable from the record. Today the answer is in three Markdown files nothing checks, and the published set answers a different question confidently

#### Publish the shipped doctrine for a version, derived from the record rather than scraped

`what did 0.8.0 ship, and what of it fails closed` becomes answerable from the record, for any version, at any time. Today it is answerable only for the working tree and only by running four scripts

#### Refuse a close whose reviewer is the agent that did the work

`the same engineer cannot self-approve` moves from agent-attested doctrine, the bottom row of the enforcement table, to record-enforced, the top. It is the single rule this plugin states most confidently and enforces least

#### Refuse a close whose attester the tool supplied rather than a person

the one gate this plugin describes as its strongest becomes reachable. It is currently a rule that passes its tests and cannot fire in production, which is worse than not having it: it reads as a guarantee

#### Ship the method's vocabulary with the engine, so adopting is not forking

a project can adopt the method without owning a copy of it. That is the whole of AK2, and until it is true every claim this frame makes about portability is a claim about one repository

#### S1

C1 cannot be settled by the slice that owns it. It asks for a property test against the GATE, and the gate is TS.260820.05, which depends on TS.260820.04. The settlement is circular in the cut, and neither the cut nor three iterations of vetting noticed

#### X1

C3's second half — that the recorded commit CONTAINS the release's published directory — cannot be settled here. Publishing is TS.260820.10, which depends on this slice

#### Y2

the archival documents first carried a `Generated at <clock>` line, so two publishes of an unchanged record produced different bytes. Verification of a published tree is a COMPARISON, so that document could never have been verified

#### AG3

nothing refuses a SILENT revert. A symptom edited from resolved back to present with no `previously-claimed` passes, because the check has no memory of what the record said before

#### AM1

the count did not reach zero and cannot yet. Twenty of the forty-five unanchored surfaces retired; the other twenty-five are Principal-Engineer doctrine the record has no vocabulary for — fourteen probes, one guardrail, three personas, seven engineering skills. There is no node for `a probe enforces an invariant`, so there is nothing for a probe to serve

#### AM4

the rule stays at REPORT. Flipping every-shipped-surface-is-anchored to refuse requires the count to be zero, and AM1 is why it is not

