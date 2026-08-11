# WAVE: Method Spine

> **Delivered before wave adoption — a derived record, not a plan.** Reconstructed from validated truth (release history, ADRs, capability records) after the work shipped. It carries no hypothesis card and no acceptance criteria because none were written at the time; each slice cites the evidence for what it delivered. Current-state architecture lives in [docs/architecture/](../../../architecture/). Derivation rules: [ADR.260725.10](../../../architecture/adr/ADR.260725.10-brownfield-wave-retrofit.md).

**Status:** ✅ Complete (delivered before wave adoption)\
**Goal:** An agent can carry a unit of work from intent to shipped, verified, and closed by following one ordered method, instead of improvising a process per task.

---

## Tracking Rules

- Track each thin-slice by intended user outcome and current state only.
- If a thin-slice is reopened or corrected, keep the same slice ID and add one short tracking note next to that slice.
- Keep implementation history in sprint files and version control. This README stays focused on product intent.

---

## Value Theme

An LLM coding agent will produce *something* for any request. Without a method it improvises the process itself, so the shape of the output depends on the phrasing of the prompt. This wave makes the process itself the constant: one ordered path, three roles with single responsibilities, and three constraint sets that apply whether or not the agent thought to consult them.

---

## Scope

- Three personas, each with one responsibility and no authority to approve its own work.
- Three always-on guardrail sets that apply without being invoked.
- An ordered pipeline of focused skills covering both the delivery and engineering halves.
- One bootstrap index every harness loads at session start, so the map is present before the first request.

**Out of scope:**

- Reaching more than one harness — that is [wave-multi-harness-reach](../wave-multi-harness-reach/README.md).
- Making any of this mechanically checkable rather than described — that is [wave-self-conformance](../wave-self-conformance/README.md) and the enforcement work in [wave-production-readiness](../wave-production-readiness/README.md).

---

## Thin-Slices

### TS-001: Three roles, none of which can approve its own work

> **Status:** ✅ Complete

**User Value:** As a team, I need the agent to occupy one clearly-bounded role at a time so that the engineer who builds a change is structurally incapable of also being the one who approves it.

**Evidence:** `agents/product-manager.agent.md`, `agents/product-designer.agent.md`, and `agents/principal-engineer.agent.md` — the last carrying three explicit modes (architect, implementer, reviewer) whose tool surface is governed by the active skill rather than by spawning separate personas. Shipped in the [initial public release](../../../../CHANGELOG.md#010--2026-05-20); current-state description in [docs/architecture/README.md](../../../architecture/README.md) § Agents and instructions.

---

### TS-002: Constraints that apply without being invoked

> **Status:** ✅ Complete

**User Value:** As a team, I need the delivery and engineering rules to bind the agent by default so that compliance does not depend on someone remembering to ask for it.

**Evidence:** `instructions/lean-delivery-guardrails.instructions.md`, `instructions/capability-driven-guardrails.instructions.md`, and `instructions/code-contribution-intake.instructions.md`, scoped by `applyTo` where the harness supports it and injected via the session-start hook where it does not. Shipped in the [initial public release](../../../../CHANGELOG.md#010--2026-05-20).

---

### TS-003: One ordered pipeline, not a toolbox

> **Status:** ✅ Complete

**User Value:** As a team, I need each moment of delivery to have exactly one skill that owns it so that the agent's next step is determined by where the work is, not by which instruction it recalls.

**Evidence:** The PLAN → TRIAGE → BUILD → LEARN → TEACH spine documented in [docs/product.md](../../../product.md) § 4, realized as the `skills/` pipeline whose authoritative ordering lives in `skills/start-thin-slice/SKILL.md` Step 5. Tier branching (Trivial / Standard / Major) keeps process proportional to risk, generated from one source per [ADR.260720.02](../../../architecture/adr/ADR.260720.02-generated-tier-table.md). Capability record: [docs/architecture/skills/README.md](../../../architecture/skills/README.md).

---

### TS-004: The map is present before the first request

> **Status:** ✅ Complete

**User Value:** As a team, I need the agent to know which skills exist and which rules are always on at the moment a session opens, rather than discovering them partway through a task.

**Evidence:** `skills/using-praxis/SKILL.md` as the single bootstrap index, re-asserted on startup, `/clear`, and context compaction — because those are exactly the moments the guardrail summary would otherwise fall out of context. Shipped in the [initial public release](../../../../CHANGELOG.md#010--2026-05-20); injection mechanics in [docs/architecture/distribution/README.md](../../../architecture/distribution/README.md) § Session-start injection.

---

## Success Criteria

Delivered. The method spine is in place and every subsequent wave builds on it: the ordered path exists, the roles are separated, the guardrails are always-on, and the index loads before work begins.

What this wave deliberately did **not** establish is whether the agent actually follows any of it — that gap is the subject of [wave-trust-transfer](../wave-trust-transfer/README.md), and mechanically checking the plugin's own conformance is [wave-self-conformance](../wave-self-conformance/README.md).

---

## Dependencies

- **Requires:** nothing. This is the foundational wave.
- **Enables:** every other wave. Seam contracts, production-readiness posture, and trust-transfer artifacts are all extensions of this spine rather than parallel constructions.
