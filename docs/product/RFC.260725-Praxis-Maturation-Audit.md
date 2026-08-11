# RFC 260725: Praxis Maturation Audit & Implementation Plan

**Date:** 2026-07-25  
**Status:** Proposed  
**Author:** GitHub Copilot  
<!-- praxis:allow-version-literal reason="cites version roadmap milestone" -->
**Target Milestone:** Praxis 0.6.0 → 1.0.0  
**Related Documents:**
- [RFC.260725-SDLC-Observation.md](RFC.260725-SDLC-Observation.md)
- [RFC.260725-Praxis-Future-Vision.md](RFC.260725-Praxis-Future-Vision.md)
- [../product.md](../product.md)

---

## Executive Summary

<!-- praxis:allow-version-literal reason="cites current release version in executive summary" -->
To realize the exponential vision set forth in [RFC.260725-Praxis-Future-Vision.md](RFC.260725-Praxis-Future-Vision.md), this document conducts an architectural audit of the current `praxis` codebase (v0.5.0) and details a concrete maturation plan.

A key operational boundary governs this plan: **automated runtime telemetry ingestion (operations) is explicitly deferred for the near future**. Production runtime feedback will instead be driven through a **Human-in-the-Loop Real-World Feedback Loop**—structured human incident reports, operator friction logs, and manual SLO review inputs that feed directly into the Praxis `LEARN` step.

This audit evaluates `praxis` across its four core asset classes—**Skills**, **Instructions**, **Scripts/Probes**, and **Agents/Personas**—and outlines the specific code, schema, and workflow additions required to reach the target mature state.

---

<!-- praxis:allow-version-literal reason="cites current release version in audit header" -->
## Part I: Current State Audit of Praxis (v0.5.0)

| Asset Class | Current Inventory | Current Coverage & Strengths | Strategic Gaps Relative to Vision |
|---|---|---|---|
| **Skills (`skills/`)** | 21 skills ([bootstrap-project](../../skills/bootstrap-project/SKILL.md), [create-wave](../../skills/create-wave/SKILL.md), [create-sprint](../../skills/create-sprint/SKILL.md), [close-sprint](../../skills/close-sprint/SKILL.md), [author-user-docs](../../skills/author-user-docs/SKILL.md), etc.) | Excellent execution of the `PLAN -> TRIAGE -> BUILD -> LEARN -> TEACH` spine. Strong artifact templates for ADRs, wave specs, and quality specs. | - Missing upstream Event Storming & domain discovery.<br>- Missing downstream human operational feedback intake.<br>- No skill for automated capability graph analysis. |
| **Instructions (`instructions/`)** | 3 guardrails ([capability-driven-guardrails](../../instructions/capability-driven-guardrails.instructions.md), [code-contribution-intake](../../instructions/code-contribution-intake.instructions.md), [lean-delivery-guardrails](../../instructions/lean-delivery-guardrails.instructions.md)) | Strict anti-dumping rules, vertical capability slicing, and mandatory pre-implementation intake gates. | - Rules are static and project-agnostic.<br>- No meta-learning mechanism to dynamically suggest project-specific rule updates from sprint history. |
| **Scripts & Probes (`scripts/`)** | 15 probes ([check-anti-dumping.sh](../../scripts/check-anti-dumping.sh), [check-seam-contract-parity.sh](../../scripts/check-seam-contract-parity.sh), [check-design-approval-gate.sh](../../scripts/check-design-approval-gate.sh), etc.) | Fast POSIX shell scripts that enforce structural anti-dumping, seam contract existence, and design approval gates without runtime dependencies. | - Text probes rely on regex heuristics (`grep`), causing false positives/negatives in complex ASTs.<br>- Probes evaluate pre-commit state only; none parse human operational feedback logs. |
| **Agents & Personas (`agents/`)** | 3 agent personas ([principal-engineer](../../agents/principal-engineer.agent.md), [product-manager](../../agents/product-manager.agent.md), [product-designer](../../agents/product-designer.agent.md)) | Enforces the 3-mode bias firewall (Architect, Implementer, Reviewer) and lean product delivery. | - Persona mode switching relies on a single agent self-enforcing skill constraints rather than multi-agent subagent delegation. |

---

## Part II: Maturation Plan Across 5 Strategic Pillars

### Pillar 1: Upstream Strategic Domain Mapping (Event Storming → Capability Discovery)

#### Gap Analysis
Currently, [create-wave](../../skills/create-wave/SKILL.md) assumes that the user or agent already knows the capability boundaries and wave scope. When starting a greenfield project or major initiative, there is no guided process to move from raw business requirements to bounded contexts and capability folder layouts.

#### Maturation Deliverables
<!-- praxis:allow-path reason="proposed skill to be implemented in roadmap" -->
1. **`skills/event-storming/SKILL.md`**: An interactive, structured skill that guides human domain experts and agents through a virtual Event Storming session. Outputs a structured `domain-model.json` capturing Domain Events, Commands, Aggregates, and Bounded Contexts.
<!-- praxis:allow-path reason="proposed skill to be implemented in roadmap" -->
2. **`skills/derive-domain-capabilities/SKILL.md`**: Reads `domain-model.json` and automatically:
   - Identifies capability boundaries (`src/<capability>/`).
   - Generates candidate Seam Contracts (`.seam-contracts.json`).
   - Populates initial thin-slices (`TS-001`, `TS-002`) into a new wave scaffold created via [create-wave](../../skills/create-wave/SKILL.md).

---

### Pillar 2: Human-in-the-Loop Operational Feedback Engine

#### Operational Boundary & Design Constraint
Automated runtime telemetry monitoring (APM agents, OTel ingestion) is explicitly deferred for the near future. Instead, real-world runtime feedback is driven by **human operators and engineers** who observe production behavior and feed structured operational insights back into Praxis.

```
+--------------------------+       +------------------------------------+       +---------------------------+
| Real-World Runtime       |       | Human Operator Intake              |       | Praxis LEARN Phase        |
| - Incidents / Outages    | ----> | skills/ingest-operational-feedback | ----> | close-sprint              |
| - SLO Breaches / Latency |       | (creates docs/product/feedback/)   |       | Promotes learnings to:    |
| - Operator Friction      |       +------------------------------------+       | - Capability Records      |
+--------------------------+                                                    | - New Remediation Waves   |
                                                                                | - Proposed ADRs           |
                                                                                +---------------------------+
```

#### Maturation Deliverables
<!-- praxis:allow-path reason="proposed skill to be implemented in roadmap" -->
1. **`skills/ingest-operational-feedback/SKILL.md`**: A human-driven intake skill that prompts operators for incident post-mortems, production friction notes, or SLO breach observations.
2. **Operational Feedback Schema (`docs/product/feedback/FB-<ID>.md`)**: Standardized Markdown schema for human operational feedback:
   ```markdown
   # Operational Feedback: FB-260725.1

   **Severity:** Low | Medium | High | Critical
   **Impacted Capability:** `<capability-name>`
   **Observed Behavior:** [Description of production issue or friction]
   **Violated Production Readiness Anchor:** Configurable | Observable | Scalable | Resilient
   **Root Cause Hypothesis:** [Human operator hypothesis]
   **Recommended Action:** Create ADR | Schedule Remediation Wave | Update Seam Contract
   ```
3. **Enhance [close-sprint](../../skills/close-sprint/SKILL.md)**: Extend Step 3 of `close-sprint` to check `docs/product/feedback/` for unaddressed human feedback items matching the sprint's capabilities. Incorporate resolved feedback into the capability record (`docs/architecture/<capability>/README.md`) or generate candidate ADRs.

---

### Pillar 3: Living Blueprint Graph & AST Analysis

#### Gap Analysis
Current probes such as [check-anti-dumping.sh](../../scripts/check-anti-dumping.sh) and [check-port-adapter-parity.sh](../../scripts/check-port-adapter-parity.sh) use regex heuristics. They cannot parse complex import graphs, alias imports (e.g. `@/components`), or multi-language ASTs reliably.

#### Maturation Deliverables
<!-- praxis:allow-path reason="proposed script to be implemented in roadmap" -->
1. **Tree-Sitter / AST Parser (`scripts/analyze-living-graph.py`)**: A multi-language AST parser (Python-based, zero external binary dependency beyond tree-sitter or standard AST tools) that parses `src/` directory structures and import graphs.
<!-- praxis:allow-path reason="proposed script to be implemented in roadmap" -->
2. **Living Architecture Graph Validation (`scripts/check-architecture-graph.sh`)**:
   - Validates that cross-capability calls go strictly through declared capability ports/adapters.
   - Enforces zero imports from prohibited dumping-ground directories (`utils/`, `helpers/`, `shared/`).
   - Verifies that frozen seam contracts in `.seam-contracts.json` match actual code signatures.

---

### Pillar 4: Swarm Verification & Adversarial Seam Chaos

#### Gap Analysis
Currently, a single agent executes Architect, Implementer, and Reviewer modes sequentially. This creates a risk of "self-congratulatory verification" where the agent approves its own code without adversarial testing.

#### Maturation Deliverables
1. **Subagent Swarm Dispatching in [verify-and-assemble-pr](../../skills/verify-and-assemble-pr/SKILL.md)**:
   - Update `verify-and-assemble-pr` to invoke `runSubagent` with `agentName: "principal-engineer"` explicitly set to Reviewer mode.
   - The subagent runs in a separate context window with read-only access to source code, preventing implementation bias.
2. **Adversarial Seam Chaos Testing**:
   - Reviewer subagents synthesize property-based tests, boundary mutations, and fault-injection stubs against frozen seam contracts (`<name>@vN`) to verify defensive error handling prior to PR approval.

---

### Pillar 5: Self-Evolving Methodology (Meta-Learning Engine)

#### Gap Analysis
[check-escape-hatch-usage.sh](../../scripts/check-escape-hatch-usage.sh) detects `praxis:allow-*` comments, but there is no automated mechanism to analyze why escape hatches were used or whether project rules should evolve.

#### Maturation Deliverables
1. **Escape Hatch & Friction Summarizer**:
   - Extend [close-sprint](../../skills/close-sprint/SKILL.md) to aggregate escape hatches, sprint post-mortems, and PR review comments across closed sprints.
2. **Automated Rule Evolution**:
   - When 3+ sprints exhibit similar architectural friction or escape hatches, `close-sprint` outputs a proposed rule update for project-specific instructions (`.instructions.md`), ensuring Praxis continuously learns from team practice.

---

## Part III: Phased Implementation Roadmap

```
Phase 1: Upstream Event Storming (v0.6.0)
├── Create skills/event-storming
└── Create skills/derive-domain-capabilities

Phase 2: Human-in-the-Loop Operational Feedback (v0.7.0)
├── Create skills/ingest-operational-feedback
├── Define Operational Feedback Schema (docs/product/feedback/)
└── Update skills/close-sprint for feedback distillation

Phase 3: Living Architecture Graph & AST Probes (v0.8.0)
├── Implement AST-based graph analyzer (scripts/analyze-living-graph.py)
└── Upgrade check-anti-dumping.sh and check-port-adapter-parity.sh

Phase 4: Subagent Swarm & Meta-Learning Engine (v1.0.0)
├── Integrate runSubagent into verify-and-assemble-pr
└── Enable close-sprint rule evolution into .instructions.md
```

---

## Conclusion

By combining **upstream Event Storming**, a **Human-in-the-Loop Operational Feedback Engine**, an **AST-based Living Architecture Graph**, and **Subagent Swarm Verification**, Praxis will achieve exponential maturity. This roadmap honors real-world operational constraints while providing AI coding agents with unassailable discipline and complete SDLC coverage.
