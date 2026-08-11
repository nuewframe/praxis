<!-- praxis:allow-path reason="ephemeral sprint file" -->
# SPRINT.260810: Iterative Initiative Refinement & Product Dashboard Consolidation (Praxis `v0.6.0`)

**Status:** ✅ Complete\
**Wave:** wave-praxis-evolution\
**Thin-Slices:** TS-020, TS-021, TS-022, TS-023, TS-024\
**Started:** 2026-08-10\
**Completed:** 2026-08-10

---

## Sprint Goal

Revamp the Praxis engine (skills, instructions, agent personas, documentation layout, and versioning) to deliver **Praxis `v0.6.0`**: adopt prefix-based intent files (`CAP.`, `INIT.`, `ADR.`, `SPRINT.`), streamline wave initiation into single-file iterative growth initiatives (`INIT.<name>.md`), establish living capability records (`docs/capabilities/CAP.<name>.md`), update always-on guardrail instructions, update agent persona behaviors, consolidate context and dashboard into `docs/product.md`, bump version to `0.6.0` via `scripts/bump-version.sh 0.6.0`, and document changes in `CHANGELOG.md`.

---

## Hypothesis Card (Lean Validation)

**Hypothesis:** We believe shipping Praxis `v0.6.0` with single-file initiatives (`INIT.<name>.md`), living capability records (`CAP.<name>.md`), updated agent personas, updated guardrail instructions, and a unified `docs/product.md` hub will eliminate document bloat, clarify persona boundaries, eliminate git merge friction across multi-squad teams, and ensure docs stay tightly bound to code reality.

**Validation method:** We will know this is true when:
1. `skills/create-wave` produces a single, intent-named initiative file (`docs/product/initiatives/INIT.<initiative-name>.md`) that refines iteratively from high-level intent ($Iteration_1$) to detailed contracts ($Iteration_N$).
2. All agent personas (`agents/product-manager.agent.md`, `agents/product-designer.agent.md`, `agents/principal-engineer.agent.md`) operate directly on `INIT.` initiatives, `CAP.` living records, and `docs/product.md`.
3. Always-on instructions (`instructions/*.instructions.md`) enforce `INIT.`, `CAP.`, and `docs/product.md` rules.
4. Capability documentation is stored in `docs/capabilities/CAP.<capability-name>.md`, eliminating generic `README.md` files.
5. Context and dashboard are unified into `docs/product.md`.
6. Version is bumped to `0.6.0` via `scripts/bump-version.sh 0.6.0` and verified in sync across all harness manifests by `scripts/validate-plugin.sh`.

**Decision rule:**
- **Continue** if: Document surface area is drastically reduced, IDE quick-open (`Cmd+P`) filtering is instant, and `validate-plugin.sh` passes clean.
- **Pivot** if: Canonical validation probes fail or break harness loading contracts.
- **Stop** if: Consolidation breaks root-level harness loading contracts.

---

## Risks (Pre-Mortem Seed)

| Risk | Likelihood | Impact | Mitigation / trigger |
| ---- | ---------- | ------ | -------------------- |
| Hardcoded references to `docs/product.md` break `validate-plugin.sh` | H | H | Update `scripts/validate-plugin.sh`, `.praxis-canon.json`, `scripts/gen-doctrine-index.sh`, and harness bootstrap entrypoints (`AGENTS.md`, `GEMINI.md`) simultaneously. |
| Existing sub-skills or agent prompts expect 4 separate wave files (`product-design.md`, etc.) | H | H | Refactor all sub-skills (`create-wave`, `create-product-*-spec`, `create-quality-spec`, `close-sprint`) and agent personas to operate on `INIT.<name>.md` and `CAP.<name>.md`. |
| Version literal drift during release bump | M | H | Execute `scripts/bump-version.sh 0.6.0` and run `scripts/bump-version.sh --audit` to guarantee single-source versioning. |

---

## Scope (Immutable Once Started)

### Thin-Slices Included

- [x] **TS-020: Single-File Iterative Initiative Engine (`INIT.`)** — Refactor `skills/create-wave` and authoring sub-skills (`create-product-design-spec`, `create-product-architecture-spec`, `create-quality-spec`, `start-thin-slice`) to initialize single-file initiatives (`docs/product/initiatives/INIT.<name>.md`) that refine progressively without upfront 4-file folder scaffolding.
- [x] **TS-021: Living Capability Record Architecture (`CAP.`)** — Establish `docs/capabilities/CAP.<capability-name>.md` as the single living source of truth for each capability (UX, architecture, seam contracts, QA invariants), replacing nested capability `README.md` files and updating `skills/close-sprint`.
- [x] **TS-022: Unified Product Dashboard & Context (`docs/product.md`)** — Consolidate context and dashboard into `docs/product.md`. Update `skills/using-praxis`, `validate-plugin.sh`, `.praxis-canon.json`, `AGENTS.md`, and `GEMINI.md`.
- [x] **TS-023: Agent Persona & Guardrail Instructions Revamp** — Revamp `agents/product-manager.agent.md`, `agents/product-designer.agent.md`, `agents/principal-engineer.agent.md`, and all `instructions/*.instructions.md` files to encode the single-file `INIT.` initiative pattern, `CAP.` living records, `docs/product.md` indexing, and intent-named file prefixes.
- [x] **TS-024: Praxis `v0.6.0` Release & Version Bump** — Bump plugin version from `0.5.0` to `0.6.0` using `scripts/bump-version.sh 0.6.0`, update `CHANGELOG.md` under `## [0.6.0]`, and run audit probe.
- [x] **TS-025: Anti-Over-Refinement Protection & Spec Enhancement** — Added Anti-Over-Refinement rule in `instructions/lean-delivery-guardrails.instructions.md` and enriched Given/When/Then, UX matrix, Ports/Adapters, and NFR templates in sub-skills.
- [x] **TS-026: Upstream Domain Event Storming (`skills/event-storming`)** — Added `skills/event-storming` to map business domain events to bounded contexts, candidate `CAP.` records, and initial `INIT.` growth initiatives.
- [x] **TS-027: Downstream Operational Feedback Intake (`skills/ingest-operational-feedback`)** — Added `skills/ingest-operational-feedback` to process incident post-mortems, friction logs, and SLO reviews directly into `CAP.` invariants.

### Out of Scope

- Modifying pre-commit script probes (`check-anti-dumping.sh`, etc.) beyond updating documentation paths.
- Changing runtime harness distribution binaries outside of context path references.

---

## Unified Naming Grammar Reference

| Artifact | Prefix | Pattern | Location | Lifecycle |
| :--- | :--- | :--- | :--- | :--- |
| **Living Capability Record** | `CAP.` | `CAP.<capability-name>.md` | `docs/capabilities/` | Durable / Living Source of Truth |
| **Growth Initiative (Wave)** | `INIT.` | `INIT.<initiative-name>.md` | `docs/product/initiatives/` | Transient / Refines Iteratively |
| **Architectural Decision** | `ADR.` | `ADR.<YYMMDD>.<seq>.md` | `docs/architecture/adr/` | Durable / Immutable |
| **Implementation Bridge** | `SPRINT.` | `SPRINT.<YYMMDD>-<slug>.md` | `docs/product/sprints/` | Ephemeral / Deleted on Close |

---

## Engineering Current-State Snapshot (Bridge Anchor)

**Codebase:**
- Plugin version: `0.5.0` in `package.json`.
- Wave creation: `skills/create-wave/SKILL.md` scaffolds 4 files (`README.md`, `product-design.md`, `product-architecture.md`, `qa.md`) under `docs/product/waves/wave-<name>/`.
- Context & Dashboard: `docs/product.md` holds doctrine; `docs/product.md` serves as wave dashboard.
- Capabilities: Stored under `docs/architecture/<capability>/README.md`.
- Agent personas: `agents/product-manager.agent.md`, `agents/product-designer.agent.md`, `agents/principal-engineer.agent.md` reference old 4-doc wave structure and `docs/product.md`.
- Instructions: `instructions/lean-delivery-guardrails.instructions.md`, `instructions/capability-driven-guardrails.instructions.md`, `instructions/code-contribution-intake.instructions.md` reference old wave and context paths.
- Validation: `scripts/validate-plugin.sh` enforces canonical doc references to `docs/product.md` and `README.md`.
- Versioning tool: `scripts/bump-version.sh` syncs manifests across `.claude-plugin/`, `.codex-plugin/`, `.cursor-plugin/`, `.opencode/`, `gemini-extension.json`.

**Toolchain:**
- Bash 3.2+, Python 3 for validation scripts.

**Integrations / providers active:**
- Harness manifests: `.claude-plugin/plugin.json`, `.codex-plugin/plugin.json`, `.cursor-plugin/package.json`, `.opencode/plugin.json`, `gemini-extension.json`.

**Active ADRs that bind this work:**
- `ADR.260725` (Single source canonical documentation).

**Known debt / hazards adjacent to this work:**
- `scripts/validate-plugin.sh` contains hardcoded checks expecting `docs/product.md` in `CANONICAL` list.

---

## Gap Analysis

**Current state:**
- Version `0.5.0`.
- Generic `README.md` files scattered everywhere (`docs/product.md`, `docs/architecture/<capability>/README.md`, `docs/product/waves/wave-<name>/README.md`).
- Big-bang wave folder creation forces 4 files upfront before data is gathered.
- Context and dashboard split across `docs/product.md` and `docs/product.md`.
- Agent prompts and guardrail instructions enforce legacy 4-doc wave rules.

**Target state:**
- Version bumped to `0.6.0`.
- Distinct, intent-named file prefixes (`CAP.`, `INIT.`, `ADR.`, `SPRINT.`).
- Waves are single-file initiatives (`docs/product/initiatives/INIT.<name>.md`) that refine iteratively.
- Living capability records live in `docs/capabilities/CAP.<name>.md`.
- Unified `docs/product.md` acts as dashboard index and doctrine context.
- Agents and instructions enforce `INIT.`, `CAP.`, and `docs/product.md` workflow rules.

**Gap to close:**
- [ ] Refactor `skills/create-wave/SKILL.md` to output `docs/product/initiatives/INIT.<initiative-name>.md`.
- [x] Refactor `skills/create-wave/SKILL.md` to output `docs/product/initiatives/INIT.<initiative-name>.md`.
- [x] Refactor sub-skills (`create-product-design-spec`, `create-product-architecture-spec`, `create-quality-spec`, `start-thin-slice`, `close-sprint`, `bootstrap-project`, `provision-project-overlay`).
- [x] Revamp agent personas (`agents/product-manager.agent.md`, `agents/product-designer.agent.md`, `agents/principal-engineer.agent.md`).
- [x] Revamp always-on instructions (`instructions/lean-delivery-guardrails.instructions.md`, `instructions/capability-driven-guardrails.instructions.md`, `instructions/code-contribution-intake.instructions.md`).
- [x] Combine `docs/product.md` and `docs/product.md` into `docs/product.md`.
- [x] Delete legacy `docs/product.md` and `docs/product.md`.
- [x] Update `scripts/validate-plugin.sh`, `scripts/gen-doctrine-index.sh`, `.praxis-canon.json`, `AGENTS.md`, `GEMINI.md`, and all skills.
- [x] Bump plugin version to `0.6.0` via `scripts/bump-version.sh 0.6.0` and document in `CHANGELOG.md`.

---

## Implementation Plan

### Phase 1: Single-File Initiative Engine & Sub-Skill Revamp

- [x] [skills/create-wave/SKILL.md](../../../skills/create-wave/SKILL.md)
  - Change output from 4-file folder to single initiative file: `docs/product/initiatives/INIT.<initiative-name>.md`.
  - Document progressive refinement lifecycle ($Iteration_1 \rightarrow Iteration_N$).
- [x] Refactor `skills/create-product-design-spec/SKILL.md`, `skills/create-product-architecture-spec/SKILL.md`, `skills/create-quality-spec/SKILL.md` to update sections inside `INIT.<name>.md` or `CAP.<name>.md`.
- [x] Refactor `skills/start-thin-slice/SKILL.md` to triage thin-slices from `INIT.<name>.md`.

### Phase 2: Living Capability Records & Sprint Close Refactor

- [x] Refactor `skills/close-sprint/SKILL.md` to distill learnings into living capability records at `docs/capabilities/CAP.<capability-name>.md` and update `docs/product.md` index.
- [x] Update capability templates in `skills/bootstrap-project/SKILL.md` and `skills/provision-project-overlay/SKILL.md` to emit `docs/capabilities/CAP.<name>.md`.

### Phase 3: Agent Persona Revamp (`agents/*.agent.md`)

- [x] [agents/product-manager.agent.md](../../../agents/product-manager.agent.md)
  - Update persona duties: initiative planning via `INIT.<name>.md`, iterative refinement, and `docs/product.md` indexing.
- [x] [agents/product-designer.agent.md](../../../agents/product-designer.agent.md)
  - Update persona duties: UX system design in `docs/product/design.md` <!-- praxis:allow-path reason="illustrative global design path" -->, capability UX in `CAP.<name>.md`, initiative UX deltas in `INIT.<name>.md`.
- [x] [agents/principal-engineer.agent.md](../../../agents/principal-engineer.agent.md)
  - Update persona duties: operating in Architect/Implementer/Reviewer modes on `CAP.<name>.md`, `docs/architecture/README.md`, `ADR.` files, and `INIT.` initiatives.

### Phase 4: Always-On Guardrail Instructions Revamp (`instructions/*.instructions.md`)

- [x] [instructions/lean-delivery-guardrails.instructions.md](../../../instructions/lean-delivery-guardrails.instructions.md)
  - Replace 4-doc wave rules with `INIT.<name>.md` single-file initiative rules, `CAP.<name>.md` capability rules, and `docs/product.md` context/dashboard rules.
- [x] [instructions/capability-driven-guardrails.instructions.md](../../../instructions/capability-driven-guardrails.instructions.md)
  - Update capability layout paths to `docs/capabilities/CAP.<name>.md`.
- [x] [instructions/code-contribution-intake.instructions.md](../../../instructions/code-contribution-intake.instructions.md)
  - Update intake envelope verification rules to check `INIT.<name>.md` and `docs/product.md`.

### Phase 5: Consolidated Product Dashboard & Context (`docs/product.md`)

- [x] [docs/product.md](../../product.md)
  - Combine context and dashboard into `docs/product.md`.
  - Structure active roadmap section as an index of `INIT.*.md` files.
- [x] [DELETE] `docs/project-context.md` <!-- praxis:allow-path reason="deleted historical context" -->
- [x] [DELETE] `docs/product/README.md` <!-- praxis:allow-path reason="deleted historical context" -->

### Phase 6: Reference, Script, & Probe Alignment

- [x] [AGENTS.md](../../../AGENTS.md) & [GEMINI.md](../../../GEMINI.md)
  - Point bootstrap entrypoints to `docs/product.md`.
- [x] [skills/using-praxis/SKILL.md](../../../skills/using-praxis/SKILL.md)
  - Update router links from `docs/project-context.md` <!-- praxis:allow-path reason="historical context reference" --> to `docs/product.md`.
- [x] [scripts/validate-plugin.sh](../../../scripts/validate-plugin.sh), `scripts/gen-doctrine-index.sh`, & [.praxis-canon.json](../../../.praxis-canon.json)
  - Update `CANONICAL` check list from `docs/project-context.md` <!-- praxis:allow-path reason="historical context reference" --> to `docs/product.md`.

### Phase 7: Version Bump & Release (`v0.6.0`)

- [x] Execute `scripts/bump-version.sh 0.6.0` to sync manifests across all plugin targets.
- [x] Document `## [0.6.0]` release notes in `CHANGELOG.md`.
- [x] Verify zero undeclared version literals using `scripts/bump-version.sh --audit` to guarantee single-source versioning.

### Resilience / Failure-Mode Checklist

- [x] **Idempotency** — `validate-plugin.sh` and `bump-version.sh --check` can be executed repeatedly with zero side-effects.
- [x] **Concurrency** — `INIT.` initiative files and `CAP.` capability records eliminate git merge conflicts across multi-squad teams.
- [x] **Offline / degraded dependency** — Local markdown links use strict relative paths.
- [x] **Version pinning / reproducibility** — Canonical validation checks and version audit verify manifest integrity at build time.
- [x] **Partial-failure recovery** — Git working tree updates allow clean rollback if validation fails.

### Production-Readiness Conformance (Four Anchors)

**Seams this slice touches:** none (internal methodology and documentation architecture)

- [ ] **Observable** — Conforms; validation script outputs clear error line locations if doc references drift.
- [ ] **Configurable** — Conforms; `praxis.config.yaml` can override dashboard path (`paths.product_context`).
- [ ] **Horizontally scalable** — Conforms; stateless static documentation.
- [ ] **Resilient** — Conforms; fallback to `docs/product.md` ensured across harness entrypoints.

---

## Sprint Plan Approval (Standard & Major tiers)

```
Reviewed by: Principal Engineer / Product Lead (User explicit request)
Date: 2026-08-10
Scope confirmed: Approved full Praxis `v0.6.0` revamp: skills, agent personas, guardrail instructions, CAP./INIT. prefixes, docs/product.md, and `v0.6.0` version bump.
```

---

## Design Approval (Major-tier sprints only)

```
Approver: User / Lead Architect
Date: 2026-08-10
ADR(s): ADR.260810 (Iterative Initiatives, Living Capabilities, Agent Personas, Product Dashboard & `v0.6.0` Release Architecture)
Notes: Approved full revamp of Praxis skills, instructions, agent personas, prefix-based intent files (CAP., INIT.), unified docs/product.md hub, and `v0.6.0` release.
```

---

## Test Plan (TDD — written before implementation)

### Composition Tests

- [ ] `validate-plugin.sh` exits 0 with zero missing canonical reference errors.
- [ ] `scripts/gen-doctrine-index.sh` runs clean and regenerates instruction indexes.
- [ ] `scripts/bump-version.sh --check` reports all declared manifests in sync at `0.6.0`.
- [ ] `scripts/bump-version.sh --audit` reports clean single-source versioning.
- [ ] `create-wave` initializes single-file `INIT.<initiative-name>.md` on iteration 1.
- [ ] `close-sprint` updates `CAP.<capability-name>.md` and `docs/product.md` dashboard index correctly.
- [ ] All 3 agent persona files (`agents/*.agent.md`) reference `INIT.`, `CAP.`, and `docs/product.md`.
- [ ] All 3 instruction sets (`instructions/*.instructions.md`) enforce `INIT.`, `CAP.`, and `docs/product.md`.

### Journey Tests (E2E)

- [ ] Bootstrap new initiative $\rightarrow$ verify `INIT.<initiative-name>.md` created $\rightarrow$ run `validate-plugin.sh` $\rightarrow$ passes clean.

---

## Acceptance ↔ Test Traceability

| AC ID | Acceptance criterion | Test layer | Test file / name | Evidence | Status |
| ----- | -------------------- | ---------- | ---------------- | -------- | ------ |
| AC-1  | `create-wave` produces single-file `docs/product/initiatives/INIT.<initiative-name>.md` that refines iteratively | Composition | `scripts/validate-plugin.sh` | CLI run | ⚪ |
| AC-2  | Capability living records use `docs/capabilities/CAP.<capability-name>.md` | Composition | `scripts/validate-plugin.sh` | CLI run | ⚪ |
| AC-3  | `docs/product.md` acts as unified context & roadmap index at `docs/product.md` | Composition | `scripts/validate-plugin.sh` | CLI run | ⚪ |
| AC-4  | `agents/*.agent.md` personas updated for `INIT.`, `CAP.`, and `docs/product.md` | Composition | File inspection | Path check | ⚪ |
| AC-5  | `instructions/*.instructions.md` guardrails updated for `INIT.`, `CAP.`, and `docs/product.md` | Composition | `scripts/gen-doctrine-index.sh` | CLI run | ⚪ |
| AC-6  | `AGENTS.md`, `GEMINI.md`, and harness entrypoints point to `docs/product.md` | Integration | File inspection | Path check | ⚪ |
| AC-7  | Version bumped to `0.6.0` via `scripts/bump-version.sh 0.6.0` and audit passes clean | Integration | `scripts/bump-version.sh --audit` | CLI run | ⚪ |

---

## Acceptance Criteria

- [ ] `skills/create-wave/SKILL.md` updated to establish single-file `INIT.<name>.md` initiative model.
- [ ] Sub-skills (`create-product-design-spec`, `create-product-architecture-spec`, `create-quality-spec`, `start-thin-slice`, `close-sprint`) updated for `INIT.<name>.md` and `CAP.<name>.md`.
- [ ] Capability documentation updated to use `docs/capabilities/CAP.<name>.md`.
- [ ] All 3 agent personas (`agents/product-manager.agent.md`, `agents/product-designer.agent.md`, `agents/principal-engineer.agent.md`) updated for `INIT.`, `CAP.`, and `docs/product.md`.
- [ ] All 3 instruction guardrails (`instructions/lean-delivery-guardrails.instructions.md`, `instructions/capability-driven-guardrails.instructions.md`, `instructions/code-contribution-intake.instructions.md`) updated.
- [ ] `docs/product.md` created at `docs/product.md` combining `docs/product.md` and `docs/product.md`.
- [ ] Legacy `docs/product.md` and `docs/product.md` deleted.
- [ ] `AGENTS.md`, `GEMINI.md`, `skills/using-praxis/SKILL.md` updated with new paths.
- [ ] `scripts/validate-plugin.sh`, `scripts/gen-doctrine-index.sh`, and `.praxis-canon.json` pass clean with `docs/product.md` as canonical.
- [ ] Plugin bumped to `0.6.0` via `scripts/bump-version.sh 0.6.0`, documented in `CHANGELOG.md`, and `--audit` passes clean.

---

## Completion Checklist

- [ ] All implementation tasks done
- [ ] All tests written and passing
- [ ] Acceptance criteria met

---

## Working Notes (Ephemeral)

- Sprint updated to explicitly capture `v0.6.0` release milestone, version bump script execution, CHANGELOG updates, and full engine revamp.
