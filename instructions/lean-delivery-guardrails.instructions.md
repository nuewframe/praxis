---
applyTo: "docs/product/**,docs/architecture/**,docs/guides/**,docs/waves/**,docs/sprints/**"
description: >
  Always-on lean delivery guardrails: wave methodology, sprint as immutable bridge, hypothesis
  cards, intent-not-history doc style, bidirectional sprint close. Pairs with
  capability-driven-guardrails for the engineering side.
---

# Lean Delivery Guardrails

Always-on rules for any product-planning artifact. The host project owns the final word — its own instructions override these.

---

## 1. Waves Are Intent, Not Sprints With Bigger Scope

A **wave** is a coherent slice of product value tracked through thin-slices. Waves outlive sprints and survive reorganization.

- Use `create-wave` to scaffold any new wave.
- Each wave has four documents: `README.md` (intent + thin-slice tracking), `product-design.md`, `product-architecture.md`, `qa.md`.
- The wave `README.md` is the only place that tracks thin-slice **status** and **correction notes**. Other wave docs describe **intent**.

---

## 2. Thin-Slices Are Atomic User Outcomes

A thin-slice describes one user outcome end-to-end. Not a backlog item. Not an implementation task.

- Each thin-slice has a stable ID (`TS-NNN`).
- Corrections and reopens **keep the same ID**. Never invent a replacement slice for a correction.
- Status flows: `⚪ Not Started → 🔄 In Progress → ✅ Complete`. Add `🚫 Blocked` or `⚠️ At Risk` only when meaningful.
- A short tracking note next to a thin-slice is allowed when it explains a correction. Wave docs are not a changelog.

---

## 3. Sprint Is an Immutable Bridge

A sprint locks **product intent** against **engineering current state** at a fixed moment in time. Two free dimensions: WHEN work starts, and the CURRENT STATE of engineering when it starts.

- Use `create-sprint` to author any sprint. Capture the engineering current-state snapshot — codebase, toolchain, integrations, active ADRs, known debt.
- Scope is **immutable** once a sprint starts. To change scope, close the sprint and create a new one.
- Sprint files are **ephemeral** — they live in a sprint directory, not woven into permanent docs. Mutable execution state lives in a separate `SPRINT.<ID>-*.ledger.md` that survives sessions and is deleted at close.
- Every sprint includes a hypothesis card: hypothesis, validation method, continue/pivot/stop rule.
- Standard- and Major-tier sprints carry a signed **Sprint Plan Approval** line; implementation does not start until it is signed (Trivial writes `n/a`).

---

## 4. Sprint Close Is Bidirectional

When a sprint closes, learnings flow to **both** product artifacts AND engineering artifacts. Closing only one side loses half the learning.

- Use `close-sprint`. Verify acceptance criteria, record outcome evidence, distill learnings into wave docs (intent only) and engineering artifacts (handbook, ADRs, capability layouts, refactor records), update the dashboard, then **delete** the sprint file.
- Wave docs after close: clean present tense, no `SPRINT.<ID> discovered…`, no `Updated after sprint`, no sprint references, no passive history. Rewrite the section to reflect the current correct intent.

---

## 5. Quality Is Specified Before It Is Tested

Each wave has a `qa.md` authored via `create-quality-spec`. It is a **planning artifact**, not test code.

- No code, imports, fixtures, file paths, or assertions in `qa.md`.
- Every behavior maps to exactly one test layer (see `test-by-ownership`).
- Risk tiers are honest — not everything is "critical."
- Coverage gaps are explicit, with rationale.

---

## 6. Code Contribution Intake Comes Before Implementation

Before any user story, feature, thin-slice, behavior-changing contribution, or non-trivial refactor reaches implementation, use `intake-code-contribution`.

- For slice work, start at the front door with `start-thin-slice`: check dependency/status preconditions and route by tier before a sprint exists.
- Locate the wave and thin-slice first. A chat prompt is not enough product intent.
- Confirm `README.md`, `product-design.md`, `product-architecture.md`, and `qa.md` exist and are specific enough.
- Confirm or create the sprint bridge before writing code.
- Correlate the sprint plan against the current codebase, tests, toolchain, integrations, ADRs, and known hazards.
- Decide test posture before code: changed behavior gets red-first tests; preserved behavior gets a green baseline first.

---

## 7. ADRs Capture Durable Decisions

Use `create-adr` whenever a sprint or wave makes a decision that binds future work — technology selection, architectural pattern, security or infrastructure approach, supersession of a prior decision.

- Collision-safe date-based IDs (`ADR.<YYMMDD>[.HH…][.seq]`) per `create-adr` — **not** sequential numbering.
- Immutable once published; supersede with a new ADR rather than editing in place.
- Mandatory alternatives table with at least two options.
- Consequences cover both positive **and** negative outcomes.

---

## 8. Composes With MPM and Other Runtimes

If Claude MPM (or another agent orchestration runtime) is in use:

- The Nuewframe Method owns **planning artifacts** (waves, sprints, ADRs, design + architecture + quality specs).
- The orchestration runtime owns **runtime mechanics** (delegation, verification gates, ticketing, branch protection).
- Sprint files and `qa.md` are the artifacts the orchestration runtime hands to specialist agents.

---

## Anti-Patterns (Always Refuse)

- ❌ Editing a sprint after it has started (close it; open a new one)
- ❌ Annotating wave docs with sprint history (rewrite intent instead)
- ❌ Inventing replacement thin-slice IDs for corrections
- ❌ Marking a sprint complete without recording outcome evidence
- ❌ Closing a sprint without updating engineering artifacts
- ❌ Embedding code in a `qa.md`
- ❌ Treating waves as "big sprints" or sprints as "small waves"
