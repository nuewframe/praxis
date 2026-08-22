# Praxis — Product & Method Context

This is the single canonical entry point for understanding **what Praxis is, its product roadmap, its core method, and how it governs delivery.** Read it before adding, removing, or modifying any plugin file or using Praxis on a project.

---

## 1. Product Identity & Purpose

**Name:** `praxis`

**User:** An engineer, product designer, or product manager working through an LLM coding agent (Claude Code, Codex, Cursor, Gemini CLI, OpenCode, GitHub Copilot).

**Purpose:** Make an LLM coding agent execute disciplined, lean wave-based product delivery and Principal-Software-Engineer practice with **fidelity** — producing trustworthy artifacts a human team can build on without re-deriving the reasoning — instead of improvising.

**The Trust-Transfer Problem:** GenAI coding agents default to improvising structure, skipping discovery, drifting scope, and producing plausible-looking artifacts with no substance behind them. When an agent produces an architecture or design document, traditional trust is unearned — the artifact looks identical whether the agent reasoned hard or pattern-matched a template. Praxis exists to close that gap by making execution fidelity visible and script-verifiable.

**Product Dashboard Rule:** Product intent lives in [`docs/product.md`](product.md) and transient initiatives (`docs/product/initiatives/`). Engineering truth lives in living capability records (`docs/capabilities/`) and system topology (`docs/architecture/README.md`). Initiatives are *educated theories*; the architecture tree is validated truth, promoted there by `close-sprint`.

---

## 2. Active & Historical Initiatives (Roadmap Index)

Initiatives (waves) represent growth vectors delivered as transient, single-file specs (`docs/product/initiatives/INIT.<initiative-name>.md`).

| Initiative | Intent | Status | Slices |
| ---- | ------ | ------ | ------ |
| `INIT.method-spine` <!-- praxis:allow-path reason="illustrative index entry" --> | One ordered method carries work from intent to closed, instead of improvising. | ✅ Delivered† | 4 — 4 ✅ |
| `INIT.multi-harness-reach` <!-- praxis:allow-path reason="illustrative index entry" --> | Identical agent behavior across six harnesses from one source tree. | ✅ Delivered† | 3 — 3 ✅ |
| `INIT.executable-seams` <!-- praxis:allow-path reason="illustrative index entry" --> | Build against a frozen promise (`<name>@vN`) instead of waiting for a merge. | ✅ Delivered† | 5 — 5 ✅ |
| `INIT.production-readiness` <!-- praxis:allow-path reason="illustrative index entry" --> | Runtime posture decided at initiative, conformed per slice across 4 anchors. | ✅ Delivered† | 4 — 4 ✅ |
| `INIT.trust-transfer` <!-- praxis:allow-path reason="illustrative index entry" --> | Make agent discipline visible and script-checkable rather than hidden. | ✅ Delivered† | 5 — 5 ✅ |
| `INIT.self-conformance` <!-- praxis:allow-path reason="illustrative index entry" --> | Praxis demonstrably follows and runs every gate against itself. | ✅ Delivered | 11 — 11 ✅ |
| `INIT.brownfield-adoption` <!-- praxis:allow-path reason="illustrative index entry" --> | Adapt Praxis against existing products with release history. | ✅ Delivered | 2 — 2 ✅ |
| [`INIT.iterative-wave-and-dashboard-consolidation`](product/initiatives/INIT.iterative-wave-and-dashboard-consolidation.md) | Single-file INIT. initiatives, CAP. living records, and `docs/product.md` hub (`v0.6.0`). | ✅ Delivered | 8 — 8 ✅ |
| [`INIT.ast-seam-and-probe-validation`](product/initiatives/INIT.ast-seam-and-probe-validation.md) | AST-backed static analysis for port/adapter, seam contract, and probe validation (`v0.7.0`). | ✅ Delivered | 5 — 5 ✅ |
| [`INIT.ast-onboarding-and-guardrails`](product/initiatives/INIT.ast-onboarding-and-guardrails.md) | Dedicated AST onboarding skill, mandatory agent instructions, and anti-meta-commentary guardrails (`v0.7.1`). | ✅ Delivered | 5 — 5 ✅ |
| [`INIT.kdl-state-and-rust-cli-consolidation`](product/initiatives/INIT.kdl-state-and-rust-cli-consolidation.md) | Unified KDL state, compiled Rust CLI engine (`praxis`), and deterministic Markdown projections (`v0.8.0`). | ⚪ Proposed | 0 — 6 ⚪ |

*† Derived records reconstructed from release history and capability records after initial delivery.*

### Living Capability Records (`CAP.`)

Capability records are held in the record, not in prose: `praxis/capabilities/*.kdl`, checked
by `praxis check` and published per version to `docs/releases/<version>/capabilities/`.

| Capability | Facet | What it does |
| ---------- | ----- | ------------ |
| `multi-harness-distribution` | product | One tree to six harnesses, injected at session start |
| `conformance-probes` | product | Every guarantee paired with something that keeps it, at a severity an adopter can read |
| `work-admission` | engine | The one gate: admit or refuse, in writing |
| `claim-settlement` | engine | Evidence, findings, and closing without dropping scope in silence |
| `release-binding` | engine | Closed work attached to a version, and the version cut |
| `delivery-record` | engine | What is true, and what the record is allowed to hold |
| `document-projection` | engine | A view becomes a document, stamped and verifiable |

The three Markdown records that used to sit here were retired by `TS.260821.06`. Two were
ported — gaining four gate tests and a `keeps-consistent` each, neither of which the Markdown
originals declared. `CAP.method-spine-and-execution` was **not**: its subject was the
`PLAN→TRIAGE→BUILD` spine, tier routing and the sprint bridge, all retired by `TS.260821.04`.
It had been hollow since the graph replaced the spine, and nothing said so because a Markdown
capability record declares no gate test it could fail.


---

## 3. The Core Method & Opinions

Praxis is grounded in ten core principles:

1. **Code is organized by business capability, never by technical layer:** Vertical slices (`src/<capability>/`) own entities, repositories, services, adapters, and tests. Anti-dumping is absolute (`utils/`, `helpers/`, `shared/` forbidden).
2. **Functional core, imperative shell:** Pure business logic in one file, I/O wrapper in another.
3. **Initiatives (waves) are single-file, intent-named, and refine iteratively:** Initiatives live in `docs/product/initiatives/INIT.<name>.md`. They start lean on Iteration 1 (high-level intent) and refine in-place across iterations ($Iteration_1 \rightarrow Iteration_N$).
4. **Initiative = educated theory; capability record = truth:** Living current-state truth lives in `docs/capabilities/CAP.<capability>.md`, promoted there from initiative theories by `close-sprint`.
5. **Sprint is an ephemeral, immutable bridge:** A sprint (`docs/product/sprints/SPRINT.<YYMMDD>-<slug>.md`) locks product intent against engineering current state. Deleted upon `close-sprint`.
6. **Process is proportional to risk:** Trivial (fast path), Standard (full spine), Major (architect pipeline + ADR).
7. **The engineer who builds cannot approve:** Tri-mode Principal Engineer firewall (**architect**, **implementer**, **reviewer** — one at a time).
8. **Quality specified before tested; tests organized by ownership:** Logic $\rightarrow$ Composition $\rightarrow$ Adapter Contract $\rightarrow$ Integration Boundary $\rightarrow$ Journey.
9. **Durable decisions become ADRs; boundaries become frozen contracts:** `ADR.<YYMMDD>.<seq>.md` for decisions; `<name>@vN` registered in `.seam-contracts.json` for boundaries.
10. **Production readiness conformed at the slice:** 4 anchors (Observable, Configurable, Horizontally Scalable, Resilient).

---

## 4. The 5-Phase Spine

```
PLAN ──────────► TRIAGE ────────► BUILD ─────────────────────► LEARN ─────────► TEACH
create-wave →    start-thin-      create-sprint → intake →      close-sprint     author-
INIT.<name>.md   slice            implement → verify            (promotes to     user-docs
                                                                CAP.<name>.md)
```

---

## 5. File Prefix Grammar & Directory Topology

Praxis enforces explicit, search-optimized intent prefixes to eliminate nested generic `README.md` files:

| Prefix | Artifact Type | Location | Lifecycle |
| :--- | :--- | :--- | :--- |
| `CAP.` | Living Capability Record | `docs/capabilities/CAP.<capability>.md` | Durable / Living Source of Truth |
| `INIT.` | Growth Initiative (Wave) | `docs/product/initiatives/INIT.<name>.md` | Transient / Refines Iteratively |
| `ADR.` | Architectural Decision | `docs/architecture/adr/ADR.<YYMMDD>.<seq>.md` | Durable / Immutable |
| `SPRINT.` | Implementation Bridge | `docs/product/sprints/SPRINT.<YYMMDD>-<slug>.md` | Ephemeral / Deleted on Close |

### Directory Layout

```
docs/
├── product.md                         <-- Unified Product Dashboard & Method Context
├── architecture.md                    <-- Global System Topology & NFRs
├── capabilities/
│   └── CAP.<capability-name>.md       <-- Living Capability Records (1 file per domain)
└── product/
    ├── design.md                      <-- Global UX Design System & Personas
    └── initiatives/
        └── INIT.<initiative-name>.md  <-- Single-File Iterative Growth Initiatives
```

---

## 6. Inventory Index (Skills, Guardrails, & Probes)

### Personas (`agents/`)

- [`product-manager`](../agents/product-manager.agent.md)
- [`product-designer`](../agents/product-designer.agent.md)
- [`principal-engineer`](../agents/principal-engineer.agent.md)

### Guardrails (`instructions/`)

- [`capability-driven-guardrails.instructions.md`](../instructions/capability-driven-guardrails.instructions.md)

### Skills (`skills/`)

- [`bootstrap-project`](../skills/bootstrap-project/SKILL.md)
- [`define-seam-contract`](../skills/define-seam-contract/SKILL.md)
- [`design-capability-layout`](../skills/design-capability-layout/SKILL.md)
- [`design-system-architecture`](../skills/design-system-architecture/SKILL.md)
- [`event-storming`](../skills/event-storming/SKILL.md)
- [`implement-with-defensive-patterns`](../skills/implement-with-defensive-patterns/SKILL.md)
- [`ingest-operational-feedback`](../skills/ingest-operational-feedback/SKILL.md)
- [`prepare-project-for-ast`](../skills/prepare-project-for-ast/SKILL.md)
- [`provision-project-overlay`](../skills/provision-project-overlay/SKILL.md)
- [`refactor-layered-to-capability`](../skills/refactor-layered-to-capability/SKILL.md)
- [`test-by-ownership`](../skills/test-by-ownership/SKILL.md)
- [`using-praxis`](../skills/using-praxis/SKILL.md)

### Scripts & Probes (`scripts/`)

- [`ast_parse.sh`](../scripts/ast_parse.sh)
- [`bump-version.sh`](../scripts/bump-version.sh)
- [`check-anti-dumping.sh`](../scripts/check-anti-dumping.sh)
- [`check-config-externalized.sh`](../scripts/check-config-externalized.sh)
- [`check-escape-hatch-usage.sh`](../scripts/check-escape-hatch-usage.sh)
- [`check-no-skipped-tests.sh`](../scripts/check-no-skipped-tests.sh)
- [`check-no-sleep-waits.sh`](../scripts/check-no-sleep-waits.sh)
- [`check-observability-at-seams.sh`](../scripts/check-observability-at-seams.sh)
- [`check-port-adapter-parity.sh`](../scripts/check-port-adapter-parity.sh)
- [`check-resilient-boundary.sh`](../scripts/check-resilient-boundary.sh)
- [`check-seam-contract-parity.sh`](../scripts/check-seam-contract-parity.sh)
- [`check-stateless-request-path.sh`](../scripts/check-stateless-request-path.sh)
- [`citation_scan.py`](../scripts/citation_scan.py)
- [`test-citation-scan.sh`](../scripts/test-citation-scan.sh)
- [`test-probes.sh`](../scripts/test-probes.sh)
- [`validate-plugin.sh`](../scripts/validate-plugin.sh)

---

## 7. Three-Tier Enforcement Split

| Gate Kind | Enforced By | Fails Closed? | Examples |
| :--- | :--- | :--- | :--- |
| **Script-Enforced** | Deterministic shell scripts (`scripts/check-*.sh`) | **Yes** (in CI / git hooks) | `check-anti-dumping.sh`, `check-seam-contract-parity.sh`, `check-design-approval-gate.sh` |
| **Human-Signed** | Signature lines in Markdown files | **No** (compliant agent check) | Sprint Plan Approval line, Design Approval line |
| **Agent-Attested** | Prompt-instructed behavioral compliance | **No** (trusted uncompelled on bare LLM) | Tier classification, intake envelope, red-first posture |

---

## 8. Emergent Parallelism (4-Condition Disjointness Rule)

Parallel work between two units is permitted **only if all four hold**:
1. **Capability/file disjoint** — no source file or capability in common.
2. **Persistent-resource disjoint** — no shared table, queue, cache, or migration.
3. **Config-key disjoint** — no shared configuration key.
4. **Frozen-contract dependent** — each depends only on a frozen `<name>@vN` seam contract.

---

## 9. Evolution Policy & Governance

1. Single-source versioning: `package.json` is the sole authored version, synced via `scripts/bump-version.sh`.
2. Breaking or structural changes bump the minor position under pre-1.0 (`0.5.0 → 0.6.0`). `1.0.0` is reserved for explicit interface stabilization.
3. Real-repo evidence rule: methodology changes must cite evidence from real-repo adoption in `CHANGELOG.md`.

## Skill inventory — the delivery graph

Every skill below is anchored in the record (`praxis audit-surfaces`); each corresponds to a slice the record delivered.

- [`skills/ask-what-is-true/`](../skills/ask-what-is-true/SKILL.md) — Ask the record what this repository already knows before reconstructing it — and read what the answer says it does not cover.
- [`skills/event-storming/`](../skills/event-storming/SKILL.md) — Upstream domain discovery — map business events to bounded contexts and candidate capabilities.
- [`skills/name-a-capability/`](../skills/name-a-capability/SKILL.md) — Derive a capability from a cluster of events: consistency boundary, the four gate tests with their reasons, and the exclusion that makes the boundary real.
- [`skills/cut-a-slice/`](../skills/cut-a-slice/SKILL.md) — Cut an atomic vertical slice in a shape a checker can refuse — one command or one view, one actor, declared layers, claims naming their evidence.
- [`skills/declare-an-entity-kind/`](../skills/declare-an-entity-kind/SKILL.md) — Declare a kind the record must hold, and its shape, on the architecture's schema. An amendment to the record, never a change to the engine.
- [`skills/witness-a-rule/`](../skills/witness-a-rule/SKILL.md) — Declare a rule together with the record that demonstrates it refusing. A rule never shown to refuse is indistinguishable from one that cannot.
- [`skills/anchor-a-doctrine-surface/`](../skills/anchor-a-doctrine-surface/SKILL.md) — Declare what a shipped skill, guardrail, agent or probe exists to serve. Instruction the record cannot trace is doctrine on the plugin's authority alone.
- [`skills/declare-an-invariant/`](../skills/declare-an-invariant/SKILL.md) — Declare what this plugin guarantees about code it is loaded into, and anchor the probe that keeps it.
- [`skills/see-what-is-ready/`](../skills/see-what-is-ready/SKILL.md) — Which slices could be started right now, and what would refuse each of the rest — including the conditions the gate declares and nobody computed.
- [`skills/see-the-dashboard/`](../skills/see-the-dashboard/SKILL.md) — Where the product stands right now: several views composed on demand, each still singly owned, never committed.
- [`skills/pick-up-a-slice/`](../skills/pick-up-a-slice/SKILL.md) — Take one or more slices through the one gate, as one commitment. Refusals are recorded as facts; an override is recorded beside the refusal it overrides.
- [`skills/attach-evidence/`](../skills/attach-evidence/SKILL.md) — Back a declared layer with an artifact at the moment it is reached. Evidence outside the slice's declared set is refused.
- [`skills/review-in-flight/`](../skills/review-in-flight/SKILL.md) — Preview what an iteration promised against what it has shown, writing nothing. The layers it has NOT reached are listed, not omitted.
- [`skills/record-a-decision/`](../skills/record-a-decision/SKILL.md) — Record a choice with the alternatives it rejected and what would show it wrong, bound to the iteration that forced it, corrected only by appending.
- [`skills/document-usage/`](../skills/document-usage/SKILL.md) — Write how a capability is used while building it. A guide for behaviour a version never shipped is refused.
- [`skills/close-an-iteration/`](../skills/close-an-iteration/SKILL.md) — Close without dropping scope in silence. A shortfall is carried by a finding that names the claim; deleting a claim to make the close succeed is itself refused.
- [`skills/bind-work-to-a-version/`](../skills/bind-work-to-a-version/SKILL.md) — Attach closed iterations to a version so its content is derived rather than hand-kept. Only closed work binds, once.
- [`skills/declare-a-lifetime/`](../skills/declare-a-lifetime/SKILL.md) — Decide whether a read model survives being frozen, and record why. Refused in both directions without a reason.
- [`skills/publish-the-release-set/`](../skills/publish-the-release-set/SKILL.md) — Regenerate every published document whole for one version, before the cut. No splice path, no wall-clock stamp.
- [`skills/cut-the-version/`](../skills/cut-the-version/SKILL.md) — Cut a planned version and write its index node in one operation. The index points at a commit rather than copying it; the seal makes a later edit visible.
- [`skills/verify-the-published-tree/`](../skills/verify-the-published-tree/SKILL.md) — Prove no published document was hand-edited — compared against the commit its release names, never against the record it is meant to outlive.
- [`skills/promote-what-shipped/`](../skills/promote-what-shipped/SKILL.md) — Fold a cut release into what each capability says it is. Derived from bound work, recomputed by a rule, refused if hand-edited in either direction.
- [`skills/resolve-a-symptom/`](../skills/resolve-a-symptom/SKILL.md) — Close a symptom by naming a release that demonstrably attacked it. Computed from what shipped, and withdrawn rather than grandfathered when it cannot be.

## Skill inventory — engineering doctrine

Shipped, still valid, and **not yet modelled by the record**. `praxis audit-surfaces` reports each as unanchored; see `TS.260821.05`.

- [`skills/bootstrap-project/`](../skills/bootstrap-project/SKILL.md)
- [`skills/provision-project-overlay/`](../skills/provision-project-overlay/SKILL.md)
- [`skills/refactor-layered-to-capability/`](../skills/refactor-layered-to-capability/SKILL.md)
- [`skills/prepare-project-for-ast/`](../skills/prepare-project-for-ast/SKILL.md)
- [`skills/design-system-architecture/`](../skills/design-system-architecture/SKILL.md)
- [`skills/design-capability-layout/`](../skills/design-capability-layout/SKILL.md)
- [`skills/define-seam-contract/`](../skills/define-seam-contract/SKILL.md)
- [`skills/implement-with-defensive-patterns/`](../skills/implement-with-defensive-patterns/SKILL.md)
- [`skills/test-by-ownership/`](../skills/test-by-ownership/SKILL.md)
- [`skills/ingest-operational-feedback/`](../skills/ingest-operational-feedback/SKILL.md)
