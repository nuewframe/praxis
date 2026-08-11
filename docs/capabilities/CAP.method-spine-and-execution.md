# CAP.method-spine-and-execution: Method Spine & Execution Infrastructure

**Domain Owner:** Praxis Core Maintainers  
**Status:** Active  
**Seam Contracts:** `method-spine@v1` (`.praxis-canon.json`)  

---

## 1. Domain Description & Bounded Context

- **Core Purpose:** Owns the core delivery spine (`PLAN -> TRIAGE -> BUILD -> LEARN -> TEACH`), thin-slice lifecycle, sprint-as-immutable-bridge contract, tier-table routing (Trivial, Standard, Major), and bidirectional sprint close distillation.
- **Bounded Context:** Governs agent persona interaction, sub-skill routing, and sprint artifact execution. Delegates multi-harness session injection to `CAP.multi-harness-distribution` and automated structural verification to `CAP.plugin-conformance-and-validation-probes`.

---

## 2. User Experience & Living Journeys

- **User Personas Served:** Product Manager, Product Designer, Principal Engineer, AI Coding Agents.
- **Primary Delivery Spine:**
  1. `PLAN`: `create-initiative` / `create-wave` $\rightarrow$ `INIT.<name>.md`
  2. `TRIAGE`: `start-thin-slice` $\rightarrow$ precondition check, tier classification, deterministic route
  3. `BUILD`: `create-sprint` $\rightarrow$ `intake-code-contribution` $\rightarrow$ `implement-with-defensive-patterns` $\rightarrow$ `verify-and-assemble-pr`
  4. `LEARN`: `close-sprint` $\rightarrow$ distills outcome evidence into `INIT.<name>.md`, `docs/product.md`, and `CAP.<name>.md`
  5. `TEACH`: `author-user-docs` $\rightarrow$ renders capability record into `docs/guides/`

- **Tier-Table Execution Routing:**
  - **Trivial:** Intake (abbreviated) $\rightarrow$ Implementer Mode $\rightarrow$ Reviewer Mode. No sprint.
  - **Standard:** `create-sprint` $\rightarrow$ Sprint Plan Approval $\rightarrow$ Intake $\rightarrow$ Implementer Mode $\rightarrow$ Reviewer Mode.
  - **Major:** Discovery $\rightarrow$ Design Architecture $\rightarrow$ Capability Layout $\rightarrow$ `create-adr` (`status: Accepted`) $\rightarrow$ `create-sprint` $\rightarrow$ Design & Sprint Plan Approvals $\rightarrow$ Intake $\rightarrow$ Implementer Mode $\rightarrow$ Reviewer Mode.

---

## 3. Technical Architecture & System Topology

- **Architecture Layout:** `skills/`, `agents/`, `instructions/`, `docs/product/initiatives/`, `docs/product/sprints/`
- **Functional Core vs. Imperative Shell:**
  - *Functional Core:* Tier classification rules (`scripts/data/tier-classification.json`), intake gate contracts, thin-slice state transitions (`⚪ -> 🔄 -> ✅`).
  - *Imperative Shell:* Sub-skill markdown instruction interpreters (`skills/*/SKILL.md`), CLI helper scripts (`scripts/gen-tier-table.sh`).

---

## 4. Quality & NFR Invariants

- **Test Layer Mapping:**
  - *Unit / Logic Tests:* Tier table generation tests (`scripts/gen-tier-table.sh --check`)
  - *Adapter Contract Tests:* Agent persona frontmatter validation (`validate-plugin.sh` Check #8)
  - *Integration Tests:* Self-conformance probe suite (`scripts/validate-plugin.sh`)
- **4 Production Readiness Anchors:**
  - *Observable:* Deterministic triage records and progress ledgers (`SPRINT.<ID>.ledger.md`) logging execution posture.
  - *Configurable:* `praxis.config.yaml` declaring custom paths (`paths.product_root`).
  - *Scalable:* Fast single-file loading ($< 10\text{ms}$) per skill invocation.
  - *Resilient:* Precondition gates halting slice triage if dependencies are blocked (`🚫 Blocked`).

---

## 5. Capability History & Lineage

- **Initiatives Delivered:**
  <!-- praxis:allow-version-literal reason="cites release initiative" -->
  - [INIT.praxis-v0.6.0-consolidation](../product/initiatives/INIT.praxis-v0.6.0-consolidation.md) — Single-file initiatives, living capability records, and unified product dashboard.
- **Durable Decisions:**
  - [ADR.260720.02](../architecture/adr/ADR.260720.02-generated-tier-table.md) — Single source for tier-classification table.
  - [ADR.260724](../architecture/adr/ADR.260724-wave-category-relaxation.md) — Relaxation of wave category taxonomy.
