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
| `INIT.iterative-wave-and-dashboard-consolidation` <!-- praxis:allow-path reason="illustrative index entry" --> | Single-file INIT. initiatives, CAP. living records, and docs/product.md hub. | 🔄 In Progress | 5 — 0 ✅ |

*† Derived records reconstructed from release history and capability records after initial delivery.*

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

- [`lean-delivery-guardrails.instructions.md`](../instructions/lean-delivery-guardrails.instructions.md)
- [`capability-driven-guardrails.instructions.md`](../instructions/capability-driven-guardrails.instructions.md)
- [`code-contribution-intake.instructions.md`](../instructions/code-contribution-intake.instructions.md)

### Skills (`skills/`)

- [`author-user-docs`](../skills/author-user-docs/SKILL.md)
- [`bootstrap-project`](../skills/bootstrap-project/SKILL.md)
- [`close-sprint`](../skills/close-sprint/SKILL.md)
- [`create-adr`](../skills/create-adr/SKILL.md)
- [`create-capability-record`](../skills/create-capability-record/SKILL.md)
- [`create-initiative`](../skills/create-initiative/SKILL.md)
- [`create-product-architecture-spec`](../skills/create-product-architecture-spec/SKILL.md)
- [`create-product-design-spec`](../skills/create-product-design-spec/SKILL.md)
- [`create-quality-spec`](../skills/create-quality-spec/SKILL.md)
- [`create-sprint`](../skills/create-sprint/SKILL.md)
- [`create-wave`](../skills/create-wave/SKILL.md)
- [`define-seam-contract`](../skills/define-seam-contract/SKILL.md)
- [`derive-waves-from-history`](../skills/derive-waves-from-history/SKILL.md)
- [`design-capability-layout`](../skills/design-capability-layout/SKILL.md)
- [`design-system-architecture`](../skills/design-system-architecture/SKILL.md)
- [`discovery-and-ambiguity-log`](../skills/discovery-and-ambiguity-log/SKILL.md)
- [`event-storming`](../skills/event-storming/SKILL.md)
- [`implement-with-defensive-patterns`](../skills/implement-with-defensive-patterns/SKILL.md)
- [`ingest-operational-feedback`](../skills/ingest-operational-feedback/SKILL.md)
- [`intake-code-contribution`](../skills/intake-code-contribution/SKILL.md)
- [`provision-project-overlay`](../skills/provision-project-overlay/SKILL.md)
- [`refactor-layered-to-capability`](../skills/refactor-layered-to-capability/SKILL.md)
- [`start-thin-slice`](../skills/start-thin-slice/SKILL.md)
- [`test-by-ownership`](../skills/test-by-ownership/SKILL.md)
- [`using-praxis`](../skills/using-praxis/SKILL.md)
- [`verify-and-assemble-pr`](../skills/verify-and-assemble-pr/SKILL.md)

### Scripts & Probes (`scripts/`)

- [`bump-version.sh`](../scripts/bump-version.sh)
- [`check-anti-dumping.sh`](../scripts/check-anti-dumping.sh)
- [`check-config-externalized.sh`](../scripts/check-config-externalized.sh)
- [`check-contract-freshness.sh`](../scripts/check-contract-freshness.sh)
- [`check-design-approval-gate.sh`](../scripts/check-design-approval-gate.sh)
- [`check-escape-hatch-usage.sh`](../scripts/check-escape-hatch-usage.sh)
- [`check-no-skipped-tests.sh`](../scripts/check-no-skipped-tests.sh)
- [`check-no-sleep-waits.sh`](../scripts/check-no-sleep-waits.sh)
- [`check-observability-at-seams.sh`](../scripts/check-observability-at-seams.sh)
- [`check-port-adapter-parity.sh`](../scripts/check-port-adapter-parity.sh)
- [`check-resilient-boundary.sh`](../scripts/check-resilient-boundary.sh)
- [`check-seam-contract-parity.sh`](../scripts/check-seam-contract-parity.sh)
- [`check-sprint-disjointness.sh`](../scripts/check-sprint-disjointness.sh)
- [`check-sprint-id-collision.sh`](../scripts/check-sprint-id-collision.sh)
- [`check-stateless-request-path.sh`](../scripts/check-stateless-request-path.sh)
- [`citation_scan.py`](../scripts/citation_scan.py)
- [`gen-coverage-matrix.sh`](../scripts/gen-coverage-matrix.sh)
- [`gen-doctrine-index.sh`](../scripts/gen-doctrine-index.sh)
- [`gen-tier-table.sh`](../scripts/gen-tier-table.sh)
- [`test-citation-scan.sh`](../scripts/test-citation-scan.sh)
- [`test-probes.sh`](../scripts/test-probes.sh)
- [`test-sprint-coordination.sh`](../scripts/test-sprint-coordination.sh)
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
