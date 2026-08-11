# RFC 260725: SDLC Observation Correlation & Exponential Vision for Praxis

**Date:** 2026-07-25\
**Status:** Proposed\
**Author:** GitHub Copilot\
**Related Documents:**

- [RFC.260725-SDLC-Observation.md](RFC.260725-SDLC-Observation.md)
- [../product.md](../product.md)
- [../architecture/README.md](../architecture/README.md)

---

## Executive Summary

Software development across industry practice alternates between high-level intent definition and low-level code implementation. [RFC.260725-SDLC-Observation.md](RFC.260725-SDLC-Observation.md) captures this reality across four observed workflows: **Business Process Requirements**, **Agile Blueprint**, **System Development**, and **Software Runtime**.

This RFC correlates those observed workflows against the governing doctrine of Praxis in [../product.md](../product.md). It demonstrates that Praxis currently provides an industry-leading execution engine for the **Agile Blueprint** loop (`PLAN -> TRIAGE -> BUILD -> LEARN -> TEACH`) and **System Development** discipline. However, it also uncovers three strategic gaps:

1. **Upstream Gap:** Lack of formal domain-mapping skills (e.g., Event Storming to Capability boundaries) prior to wave creation.
2. **Incomplete Workflow Articulation:** Section 3 (**System Development**) of the observation RFC remained unarticulated (`...`).
3. **Downstream Gap:** Pre-commit/static bias that stops at build/verify, omitting structured operational feedback from real-world runtime behavior into the `LEARN` loop.

To achieve exponential growth in value and skill, Praxis must evolve from a **disciplined pre-commit AI coding method** into an **end-to-end, human-grounded, self-maturing SDLC engine**. This RFC details that future vision across five strategic pillars and provides a concrete roadmap for implementation.

---

## Part I: Systematic Correlation Matrix

The table below maps the four observed SDLC workflows from [RFC.260725-SDLC-Observation.md](RFC.260725-SDLC-Observation.md) directly to Praxis doctrine in [../product.md](../product.md).

| Observed SDLC Workflow                                                                                    | Praxis Core Analogue                                                                                                                         | Degree of Alignment            | Key Strengths in Praxis Today                                                                                                                                                                                                                                          | Uncovered Gaps & Opportunities                                                                                                                                                                        |
| --------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **1. Business Process Requirements** (Event Storming, Thin Slices, Notional Architecture, NFRs)           | `PLAN` phase (`create-wave`, `create-product-design-spec`, `create-product-architecture-spec`, `create-quality-spec`)                        | **Moderate to High**           | - Waves represent product intent.<br>- Notional Architecture is explicitly modeled as "Educated Theory" in wave `product-architecture.md`.<br>- NFRs are anchored in 4 Production Readiness domains (Configurable, Observable, Scalable, Resilient).                   | - No formal `skills/event-storming` or domain event mapping skill upstream.<br>- Capability boundary extraction from business domain maps relies on human pre-work.                                   |
| **2. Agile Blueprint** (Micro-artifacts maturing UX, Architecture, ADRs, Code, Living Docs & User Guides) | The Praxis Spine (`PLAN -> TRIAGE -> BUILD -> LEARN -> TEACH`)                                                                               | **Very High (Core Identity)**  | - `close-sprint` acts as the bidirectional distillation pump promoting educated theory into living capability records.<br>- `TEACH` phase ([author-user-docs](../../skills/author-user-docs/SKILL.md)) renders validated capability records into Diátaxis user guides. | - Maturity tracking across multiple waves is implicit rather than measured as a system-level metric.<br>- Freshness between code and user guides relies on soft gates rather than AST diffs.          |
| **3. System Development** (Unarticulated `...` in observation RFC)                                        | Principal Engineer Discipline & `BUILD` phase ([implement-with-defensive-patterns](../../skills/implement-with-defensive-patterns/SKILL.md)) | **High (Unformalized in RFC)** | - Capability-driven layout (anti-dumping, vertical slices).<br>- Functional Core / Imperative Shell.<br>- Seam Contracts (`define-seam-contract`).<br>- 3-mode persona bias firewall (Architect, Implementer, Reviewer).                                               | - The observation RFC left this section unarticulated; Praxis can provide the authoritative specification for it.<br>- Lacks automated AST-level seam generation and automated refactoring pipelines. |
| **4. Software Runtime** (Platform Development, Deployment, Monitoring, Maintenance)                       | 4 Production Readiness Anchors & Pre-Commit Probes (`check-*.sh`)                                                                            | **Low to Moderate**            | - Static verification of externalized config, seam observability, stateless request paths, and resilient boundary rules before push.                                                                                                                                   | - Praxis is currently pre-commit and pre-push only.<br>- Near-term operational feedback must be human-driven (incident post-mortems, real-world friction logs) feeding back into the `LEARN` loop.    |

---

## Part II: Completing Section 3 — System Development

[RFC.260725-SDLC-Observation.md](RFC.260725-SDLC-Observation.md) left Section 3 incomplete as `3. **System Development**: ...`. We complete this definition by articulating it through the lens of Praxis Principal Engineer discipline.

### 3. System Development Workflow Definition

System Development is the translation of micro-artifacts and thin-slices into resilient, maintainable, and verifiable source code. It governs the structural organization of software, interface boundaries, and defensive execution mechanics.

Key activities include:

- **Capability-Driven Structural Organization:** Code is structured strictly in vertical capability slices that own their logic, persistence, and external adapters. Technical layering silos (`controllers/`, `services/`, `utils/`) are prohibited. Cross-capability dependencies are explicitly gated through public interfaces.
- **Functional Core & Imperative Shell Isolation:** Pure domain logic is strictly isolated from side effects, I/O, and external infrastructure. Pure logic forms the base of the test pyramid, maximizing execution speed and determinism.
- **Seam Contract Freezing:** Interface boundaries between capabilities or external services are formalized as machine-readable Shapes, paired with Behavior test suites, and assigned immutable contract IDs (`<name>@vN`). Dependent slices build against frozen promises rather than waiting for producer merges.
- **Pyramid Test-by-Ownership Strategy:** Testing is structured strictly by ownership tier: **Logic → Composition → Adapter Contract → Integration Boundary → Journey**. Every test asserts exactly one property of a behavior at one layer.
- **Tri-Mode Persona Bias Firewall:** System development mandates strict separation between **Architect mode** (cannot edit source), **Implementer mode** (cannot modify approved design specs or ADRs), and **Reviewer mode** (read-only on source, issues structured change requests).

---

## Part III: The Capability Maturity Continuum Model

A fundamental insight emerges when synthesizing the Agile Blueprint with Praxis doctrine: **Wave and Capability Record are not fundamentally different concepts; they are the SAME capability viewed across a maturity timeline.**

In an iterative, agile practice, a Capability evolves through four discrete maturity states:

```mermaid
timeline
    title Capability Maturity Timeline
    t0 : PLAN (Wave) : Unhardened Hypothesis / Educated Theory : Bounded Context / Intent
    t1 : BUILD (Sprint) : Active Transformation : Thin-Slices / Seam Contracts / Code
    t2 : LEARN & TEACH (Close-Sprint) : Hardened Truth : Living Capability Record & Diátaxis Guides
    t3 : RUNTIME (Operations) : Human Operational Feedback Loop : Incident Logs / Real-World Friction / SLO Reviews
```

1. **$t_0$ — Unhardened Proposal (The Wave):**\
   The Capability is discovered and bounded (e.g. via Event Storming). It exists as an _educated theory_ / hypothesis in `docs/product/waves/wave-<name>/` with its `product-architecture.md` and thin-slices (`TS-NNN`).
2. **$t_1$ — Active Transformation (The Sprint):**\
   Thin-slices of the capability are locked in immutable sprint files. Implementers write pure core logic and imperative shells in `src/<capability>/`, bound by frozen seam contracts.
3. **$t_2$ — Hardened Truth (Capability Record & User Guides):**\
   Upon sprint completion, `close-sprint` promotes validated learning into the durable architecture record (`docs/architecture/<capability>/README.md`) and renders user-facing guides (`docs/guides/<capability>/`). The capability is now hardened truth.
4. **$t_3$ — Real-World Operational Feedback (Software Runtime):**\
   The hardened capability runs in production. Real-world feedback (human-reported incidents, operator friction logs, production SLO reviews) continuously validates or challenges the hardened truth, feeding new unhardened proposals ($t_0$) whenever real-world operational drift occurs.

---

## Part IV: The Exponential Future Vision for Praxis

To transform Praxis from a pre-commit agent discipline framework into an **exponentially valuable, end-to-end SDLC power engine**, Praxis must evolve along five strategic pillars.

```mermaid
flowchart TD
    subgraph P1[Pillar 1: Upstream Discovery]
        ES[Event Storming] --> DM[Domain Model]
        DM --> CB[Capability Bounding]
        CB --> W[Wave & Thin Slices]
    end

    subgraph P2[Pillar 2 & 3: Living Blueprint Engine]
        W --> SP[Praxis Spine: PLAN->TRIAGE->BUILD]
        SP --> LAG[Living Architecture Graph]
        LAG --> ADD[Automated Drift Detection]
    end

    subgraph P4[Pillar 4: Swarm Verification]
        ADD --> MAS[Multi-Agent Swarm]
        MAS --> AC[Adversarial Chaos Seam Tests]
    end

    subgraph P5[Pillar 5: Human Operational Feedback]
        AC --> DEP[Deploy & Software Runtime]
        DEP --> HOF[Human Operational Feedback & Incident Reports]
        HOF --> LEARN[LEARN Phase: Close-Sprint & ADRs]
        LEARN --> META[Meta-Learning: Self-Evolving Rules]
        META -.-> ES
    end
```

### Pillar 1: Upstream Strategic Domain Mapping (Event-Storming to Capability Synthesis)

- **Concept:** Extend Praxis upstream before wave creation. Introduce `skills/event-storming` and `skills/derive-domain-capabilities`.
- **Value:** Agents collaborate with domain experts to run virtual Event Storming workshops, generating Domain Events, Commands, Aggregates, and Bounded Contexts.
- **Skill Mechanism:** Automatically output initial capability folder layouts, candidate Seam Contracts, and prioritized Thin Slices (`TS-NNN`) directly into wave documents.

### Pillar 2: Human-in-the-Loop Operational Feedback Engine

- **Concept:** Structure real-world human operational feedback (incidents, operator friction, production SLO reviews) directly into the `LEARN` step of the spine.
- **Value:** Fully automated runtime telemetry ingestion remains out of near-term scope; human operators provide the critical judgment and context on production runtime reality.
- **Skill Mechanism:** Introduce a human-driven feedback intake skill (`skills/ingest-operational-feedback`). Human operators record incident summaries or real-world friction notes. Praxis processes this input, compares it against the 4 Production Readiness Anchors (Configurable, Observable, Scalable, Resilient), and automatically outputs ADR proposals or new remediation thin-slices.

### Pillar 3: Living Blueprint Graph & Automated Architecture Drift Engine

- **Concept:** Replace static text files and regex probes with an active, AST-parsed **Living Architecture Graph** (LAG).
- **Value:** Real-time visibility into capability coupling, undeclared imports, broken seam contract versions, and stale capability records.
- **Skill Mechanism:** Continuous background analysis parses source ASTs and `.seam-contracts.json`. When code drifts from declared architecture or capability records, Praxis generates an automated AST refactoring plan via [refactor-layered-to-capability](../../skills/refactor-layered-to-capability/SKILL.md) to realign implementation with truth.

### Pillar 4: Adversarial Multi-Agent Swarm & Seam Chaos Verification

- **Concept:** Elevate the 3-mode persona firewall (Architect, Implementer, Reviewer) into autonomous, contract-bound subagent swarms.
- **Value:** Eliminate self-congratulatory agent verification where the builder checks their own work.
- **Skill Mechanism:** Introduce **Adversarial Seam Chaos Testing** in Reviewer mode. Before PR assembly, the Reviewer agent dynamically synthesizes property-based edge-case inputs, latency injections, and network partition mocks targeting frozen seam contracts. Work is approved only when defensive patterns withstand adversarial probes.

### Pillar 5: Self-Evolving Methodology (Meta-Learning Engine)

- **Concept:** Enable Praxis to learn from its own project execution history.
- **Value:** Every project has unique failure modes, anti-patterns, and escape-hatch usages (`praxis:allow-*`).
- **Skill Mechanism:** Ingest sprint post-mortems, PR review rejections, and escape-hatch logs during `close-sprint`. Automatically propose updates to project-specific instructions (`.instructions.md`) and project context, ensuring the method becomes smarter with every closed sprint.

---

## Part V: Concrete Capability & Skill Expansion Roadmap

To realize this vision without violating the core principle that ceremony must remain proportional to risk, we propose a four-phase expansion roadmap:

```
Phase 1: Domain Mapping (Upstream) ──► Phase 2: Runtime Telemetry (Downstream)
                 │                                   │
                 ▼                                   ▼
Phase 3: Living Blueprint Graph   ──► Phase 4: Self-Evolving Engine
```

### Phase 1: Upstream Domain & Capability Discovery

1. Create `skills/event-storming`: Interactive domain event discovery and bounded context identification.
2. Create `skills/derive-domain-capabilities`: Converts event storming outputs into capability layouts and initial wave specifications.

### Phase 2: Downstream Human Operational Feedback Integration

1. Create `skills/ingest-operational-feedback`: Structured human incident/friction intake skill into the `LEARN` phase.
2. Create `check-slo-drift.sh` (operational probe): Evaluates human-submitted incident/friction logs against declared wave SLOs and 4-anchor posture.

### Phase 3: Living Architecture Graph & AST Analysis

1. Create AST-based dependency graph analyzer for seam validation and capability boundary checking.
2. Upgrade [scripts/check-anti-dumping.sh](../../scripts/check-anti-dumping.sh) from heuristic regex searching to semantic AST import analysis.

### Phase 4: Self-Evolving Meta-Learning Loop

1. Extend [skills/close-sprint/SKILL.md](../../skills/close-sprint/SKILL.md) to analyze escape hatches and PR rejection logs.
2. Automatically generate project-level `.instructions.md` rules when recurring anti-patterns are detected across 3+ sprints.

---

## Conclusion

By correlating [RFC.260725-SDLC-Observation.md](RFC.260725-SDLC-Observation.md) against [../product.md](../product.md), we have defined the missing **System Development** workflow and identified the key frontiers for Praxis. Expanding Praxis upstream into **Domain Event Storming**, downstream into **Human-in-the-Loop Operational Feedback**, and internally into a **Living Architecture Graph** will exponentially increase its value—enabling AI coding agents to deliver software with unassailable discipline, true trust transfer, and continuous real-world validation.
