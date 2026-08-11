---
applyTo: "docs/product/**,docs/architecture/**,docs/capabilities/**,docs/guides/**,docs/sprints/**"
description: >
  Always-on lean delivery guardrails: initiative methodology (INIT.<name>.md), living capability records (CAP.<name>.md),
  sprint as immutable bridge, hypothesis cards, intent-not-history doc style, bidirectional sprint close.
---

# Lean Delivery Guardrails

Always-on rules for any product-planning artifact. The host project owns the final word — its own instructions override these.

**Path coverage.** Applies to `docs/product/**` (initiatives, dashboard, sprints), `docs/capabilities/**` (living capability records), `docs/architecture/**` (ADRs and topology), and `docs/guides/**` (TEACH-phase user docs).

---

## 1. Initiatives (Waves) Are Intent, Single-File, and Refine Iteratively

An **initiative** (wave) is a coherent slice of product value tracked through thin-slices (`TS-NNN`). Initiatives outlive sprints and survive reorganization.

- Use `create-wave` or `create-initiative` to scaffold any new initiative as a single, intent-named file: `docs/product/initiatives/INIT.<initiative-name>.md`.
- Initiatives start **lean** on Iteration 1 (high-level intent + thin-slices) and **refine progressively** across iterations ($Iteration_1 \rightarrow Iteration_N$) as data arrives.
- **Anti-Over-Refinement Rule:** Never attempt to populate $Iteration_N$ architecture topology, seam contracts, or NFR anchors up front for early exploratory spikes or $Iteration_1$ drafts. Keep $Iteration_1$ lean; deepen specs only when thin-slices are triaged into sprints.
- The initiative file is the only place that tracks thin-slice **status** and **correction notes**.
- Intent-named file prefixes (`CAP.`, `INIT.`, `ADR.`, `SPRINT.`) are required — no generic `README.md` files in subdirectories.

---

## 2. Thin-Slices Are Atomic User Outcomes

A thin-slice describes one user outcome end-to-end. Not a backlog item. Not an implementation task.

- Each thin-slice has a stable ID (`TS-NNN`).
- Corrections and reopens **keep the same ID**. Never invent a replacement slice for a correction.
- Status flows: `⚪ Not Started → 🔄 In Progress → ✅ Complete`. Add `🚫 Blocked` or `⚠️ At Risk` only when meaningful.
- A short tracking note next to a thin-slice is allowed when it explains a correction.

---

## 3. Sprint Is an Immutable Bridge

A sprint locks **product intent** against **engineering current state** at a fixed moment in time.

- Use `create-sprint` to author any sprint (`docs/product/sprints/SPRINT.<YYMMDD>-<slug>.md`). Capture the engineering current-state snapshot — codebase, toolchain, integrations, active ADRs, known debt.
- Scope is **immutable** once a sprint starts. To change scope, close the sprint and create a new one.
- Sprint files are **ephemeral** — mutable execution state lives in `SPRINT.<ID>-*.ledger.md` that is deleted at close.
- Every sprint includes a hypothesis card: hypothesis, validation method, continue/pivot/stop rule.
- Standard- and Major-tier sprints carry a signed **Sprint Plan Approval** line.

---

## 4. Sprint Close Is Bidirectional Outflow

When a sprint closes, learnings flow to **both** product artifacts AND engineering artifacts.

- Use `close-sprint`. Verify acceptance criteria, record outcome evidence, distill learnings into initiative files (`INIT.<name>.md`), the product dashboard ([`docs/product.md`](../docs/product.md)), AND living capability records (`docs/capabilities/CAP.<capability-name>.md`), then **delete** the sprint file.
- Initiative docs after close: clean present tense, no sprint references, no passive history. Rewrite sections to reflect current correct intent.

---

## 5. Quality Is Specified Before It Is Tested

Quality criteria and NFR invariants are refined inside `INIT.<name>.md` and promoted to `docs/capabilities/CAP.<name>.md#quality-and-nfr-invariants`.

- No code, imports, fixtures, file paths, or assertions in quality specs.
- Every behavior maps to exactly one test layer (see `test-by-ownership`).
- 4 Production-Readiness anchors (Observable, Configurable, Scalable, Resilient) are explicit.

---

## 6. Code Contribution Intake Comes Before Implementation

Before any contribution reaches implementation, use `intake-code-contribution`.

- For slice work, start at `start-thin-slice`: check dependency/status preconditions in `INIT.<initiative>.md` and route by tier.
- Confirm `INIT.<initiative-name>.md` exists and is specific enough.
- Confirm or create the sprint bridge before writing code.

---

## 7. ADRs Capture Durable Technical Decisions

Use `create-adr` whenever a sprint or initiative makes a decision that binds future work.

- Collision-safe date-based IDs (`ADR.<YYMMDD>[.HH…][.seq]`).
- Mandatory alternatives table with at least two options.
- Consequences cover both positive **and** negative outcomes.

---

## 8. Anti-Meta-Commentary & Bare Assertion Discipline

Prohibit **meta-commentary** — prose whose subject is *the document* rather than the schema or architecture, specifically **disclaimer-and-deflection** (justifying what's absent or pre-empting an argument).

- **The Three-Part Test (Cut a line if it)**:
  1. **Talks about the file** — *"The SDL delta only"*, *"This section covers..."*
  2. **Justifies an absence** — *"..are held in the architecture record"*
  3. **Instructs the reader's behavior** — *"before debating a decision here"*
- **The Conversion Rule:** Rationale reads *because X, therefore Y*. A decision reads *Y*. Delete the *because-clause*; keep *Y* as a bare assertion. If a fact is load-bearing, state it flat — do not argue it.
- **The Three Tells (Catch & Remove)**:
  - `so` / `because` / `since` — the joint where rationale attaches. Cut left of it, keep right.
  - ``"by design, not by oversight"`` — pure defensiveness. It answers a challenge nobody made. State the reason it's omitted as a flat fact and stop.
  - `"today"` / `"already"` / `"in production"` — intensifiers that argue a fact is true rather than stating it.

---

## Anti-Patterns (Always Refuse)

- ❌ Editing a sprint's **locked scope** after it has started.
- ❌ Creating nested generic `README.md` files in subdirectories (use `CAP.`, `INIT.`, `ADR.`, `SPRINT.`).
- ❌ Forcing 4-file folder scaffolding up front instead of single-file progressive refinement.
- ❌ Annotating initiative docs with sprint history (rewrite intent instead).
- ❌ Inventing replacement thin-slice IDs for corrections.
- ❌ Closing a sprint without updating living capability records (`CAP.<name>.md`).
- ❌ Writing meta-commentary, throat-clearing, or defensive rationale (``"by design, not by oversight"``) in documentation.
