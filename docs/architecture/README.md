This tree is current-state truth, promoted by `close-sprint`. Planning-stage intent lives in `docs/product/initiatives/`.

## Identity

Praxis is a portable agent plugin that fuses a typed delivery graph with Principal Engineer discipline, distributed across six harnesses from one single-source tree. For the plugin's identity, the trust-transfer problem it exists to close, and the scope-rule doctrine that governs what belongs here, see [`../product.md`](../product.md) — this overview does not restate that content; it states the engineering-truth picture and points at the capability records in `praxis/capabilities/` for current-state detail behind each domain.

## The capability records

They are no longer here. Capabilities are records the checker reads — `praxis/capabilities/*.kdl`
— and the published set for a version is generated from them into
`docs/releases/<version>/capabilities/`, split into **what the product can do** and **how it is
built**.

`TS.260821.06` ported two of the three Markdown records that used to be described in this
section, and declined to port the third. `CAP.method-spine-and-execution` described the
`PLAN→TRIAGE→BUILD→LEARN→TEACH` spine, the tier branch and the sprint bridge; all three were
retired by `TS.260821.04`. It was not a capability that needed porting — it was the old method
wearing a capability's name, and what replaced it is `work-admission` and `claim-settlement`.

Ask the record rather than reading a summary of it:

```
praxis truth              what this repository already knows
praxis check-invariants   what the plugin guarantees, and what keeps each guarantee
praxis audit-surfaces     what it ships as instruction, and what asked for each piece
```

## Agents and instructions

The 3 personas in `agents/` (`principal-engineer` — three modes, architect/implementer/reviewer, tool surface governed by mode rather than by separate agent instances; `product-manager` and `product-designer`) and the 3 always-on guardrail sets in `instructions/` (`capability-driven-guardrails`, `lean-delivery-guardrails`, `code-contribution-intake`) do not get their own capability record: they are small and stable enough — a handful of files, low change-rate — that forcing them into a fourth top-level capability-record home would be ceremony disproportionate to their size, the same 4th-litmus-question discipline the plugin applies to everything else it might add. Read the files directly: `agents/principal-engineer.agent.md`, `agents/product-manager.agent.md`, `agents/product-designer.agent.md`, and the three `instructions/*.instructions.md` files.

## ADR index (cross-capability)

These ADRs genuinely cross more than one capability boundary; each is indexed here as well as in the capability record(s) it is homed under.

| ADR | Purpose |
| --- | --- |
| [ADR.260720.01: Design Approval git pre-push hook gate](adr/ADR.260720.01-design-approval-git-hook-gate.md) | Builds `check-design-approval-gate.sh`, the one gate in this repo that is hard-fail with no opt-out by design — the first gate Praxis demonstrably fails closed without an orchestration runtime, reaching a host project through the distribution capability's `verify.sh` and git hooks. |
| [ADR.260720.02: Single-source-of-truth generated tier-classification table](adr/ADR.260720.02-generated-tier-table.md) | Generates the tier-classification table into three skill/agent surfaces from one JSON source, using the generator pattern of its day. Both the table and the generator were retired by `TS.260821.04` and `TS.260821.07`; the ADR stands as the record of why it was built. |
| [ADR.260720.03: Artifact-fidelity review and the Trust Receipt](adr/ADR.260720.03-fidelity-review-and-trust-receipt.md) | Adds the artifact-fidelity review and Trust Receipt to `verify-and-assemble-pr`, closing the gap shape-checking probes cannot: whether an artifact's reasoning has substance, sourcing escape-hatch facts from the enforcement capability's `check-escape-hatch-usage.sh`. |
| [ADR.260724: Wave naming imposes no category taxonomy](adr/ADR.260724-wave-category-relaxation.md) | Homed in the `skills` capability (`create-wave` loses its category mandate), but its second half amends the plugin's evolution policy to classify rule *relaxation* as distinct from removal — a governance rule that binds every capability's future changes. |
| [ADR.260725: Declared exceptions move inline](adr/ADR.260725-inline-declared-exceptions.md) | Homed in `enforcement`, but changes how every capability's documents declare a sanctioned literal: structural citation detection plus inline reasoned markers replace path allowlists. **Status: Accepted** — implemented by `TS-008` of wave-self-conformance. |
| [ADR.260725.10: Retrofitting waves onto an existing product](adr/ADR.260725.10-brownfield-wave-retrofit.md) | Separates deriving an intent map from validated truth (legitimate) from fabricating history (forbidden), and establishes the two-tier wave form — README-only with cited evidence for delivered work, full four documents for open work. Closes the brownfield adoption gap `bootstrap-project` and `refactor-layered-to-capability` both leave open. **Status: Accepted** — already applied to Praxis's own tree; generalized into an adoption path by `TS-001` of wave-brownfield-adoption. |
| [ADR.260819.01: KDL State Architecture and Rust CLI Engine Consolidation](adr/ADR.260819.01-kdl-state-architecture-and-rust-cli-consolidation.md) | Adopts KDL for operational state extending `docs/product/` and a compiled Rust CLI (`praxis`) for `<10ms` verification and zero-regex whole-document Markdown projections (ADRs, capabilities, user guides, dashboards). **Status: Superseded by ADR.260819.02** — its runtime and encoding commitments carry forward; its state model and render semantics do not. |
| [ADR.260819.02: The Delivery Graph — iteration-native thin slices](adr/ADR.260819.02-delivery-graph-and-iteration-native-slices.md) | Models the method as a typed delivery graph with the thin slice as central entity and the iteration as a first-class node: single-writer facts, nothing derived at rest, prose in referenced Markdown fragments, work bound to a release at the iteration that lands it, and published Markdown as a projection valid for exactly one release. Collapses the sprint into the iteration, gates admission proportionally to initiative depth, gives the gap a lifecycle that closes only through shipped work, and carries the full entity/edge model, a concurrency-optimized layout, and a journey blueprint that returned twenty-eight findings, all closed. Converts thirteen fidelity checks from agent-attested to tool-enforced traversals, verticality among them. |

## Current posture

All harness manifests are held at version parity with `package.json` by `bump-version.sh`; `bump-version.sh --check` reports the current number, and this document deliberately does not restate it. `validate-plugin.sh` runs its self-test. The generators are gone: CI now publishes the release set and fails on a diff, so the record is the source and the document is the projection. `praxis check` fails closed with no configuration at all — see *Enforcement, honestly* in the README for the four kinds of gate and which of them actually compel.

**Known gap, stated plainly:** `.github/workflows/ci.yml` executes exactly one of the twelve `check-*.sh` scripts against this repo (`check-anti-dumping.sh`). The other eleven are passed to `bash -n` — a syntax check that proves they parse, not that they pass. `check-design-approval-gate.sh` has therefore never run in Praxis's own CI, and fails when run manually against the current tree. Three of the eleven are directly applicable to a repo with no runtime code (design-approval, sprint-id-collision, escape-hatch-usage); the remaining seven target host-repo request paths and seams Praxis does not have, and should assert a reasoned `n/a` rather than be silently absent. Closing this is `TS-005` of `INIT.self-conformance` on [`docs/product.md`](../product.md).
