---
name: create-sprint
description: >
  Create a sprint document — the immutable bridge between product intent and engineering
  current state. A sprint locks a thin-slice (or set of thin-slices) against the codebase,
  toolchain, and integration reality at the moment work begins. Scope is immutable once started;
  the sprint file is ephemeral and deleted when the sprint closes via `close-sprint`.
user-invocable: true
disable-model-invocation: false
---

# Skill: Create Sprint

Use this skill when starting a new sprint from a wave's thin-slices, or to take on a correction or improvement that needs the full sprint discipline.

---

## Core Mental Model — Sprint as Immutable Bridge

A sprint is **not** an execution plan that you re-plan as you go. It is a **bridge**:

```
PRODUCT INTENT                 SPRINT (immutable bridge)              ENGINEERING REALITY
─────────────                  ──────────────────────────             ──────────────────
wave goal                                                             current codebase
thin-slice                     locks at sprint start:                 current capability layout
acceptance criteria            • thin-slice intent                    current tooling
hypothesis card                • engineering current-state snapshot   current integrations
                               • when work will start                 active ADRs
                               • acceptance criteria
                               • test plan
                               • implementation plan
                                          ↓
                                    work happens
                                          ↓
                                    learnings flow BIDIRECTIONALLY
                                          ↓
                          ┌───────────────┴───────────────┐
                          ↓                               ↓
                  product artifacts                engineering artifacts
                  (wave docs, PRODUCT)             (handbook, ADRs, capability layouts)
                  via close-sprint                 via close-sprint
```

**Two free dimensions in a thin-slice:** **WHEN** the work will start, and **CURRENT STATE** of engineering when it starts. Intent does not vary between sprints; reality does.

**Immutable once started:** scope, acceptance criteria, hypothesis, test plan structure. If the bridge is wrong, close the sprint and create a new one — do not edit a live bridge.

---

## Project Conventions

This skill assumes the host project defines:

- A **sprint directory** (commonly `docs/product/sprints/`)
- A **sprint file naming** convention: `sprint-NNN-<short-description>.md`
- A wave directory with at least one wave README containing thin-slices

If those don't exist, define them in the project's own context first.

---

## Step 1 — Identify the Sprint Number

Find the highest existing `sprint-NNN-*.md` and increment by 1.

---

## Step 2 — Select Thin-Slices to Deliver

From the wave README:

- Pull thin-slices that are `⚪ Not Started` or being corrected/reopened
- Sequence by dependency order
- 1–2 thin-slices per sprint is typical — quality over quantity
- If correcting an existing thin-slice, **keep the same slice ID**. Do not invent a replacement slice for a correction.

---

## Step 3 — Capture Engineering Current-State Snapshot

This is what makes the sprint a bridge. Document **what exists today** — not what should exist.

Required snapshot coverage:

- **Codebase state:** which files, modules, capabilities exist that this work touches
- **Toolchain state:** versions of frameworks, runtimes, lint/test tooling that constrain choices
- **Integration state:** which third-party providers, services, or feature flags are active
- **Active ADRs:** which durable decisions constrain how this work must be done
- **Known debt / hazards:** capability layout violations, anti-dumping debt, missing tests adjacent to the work

The snapshot freezes the engineering side of the bridge. Without it, the sprint cannot be a contract.

---

## Step 4 — Run Gap Analysis

Compare:

- **`product-design.md`** — what the experience should be
- **`product-architecture.md`** — how it should work technically
- **Engineering current-state snapshot from Step 3** — what actually exists today

Document the gap explicitly. The sprint plan closes this specific gap.

---

## Step 5 — Create the Sprint File

```markdown
# Sprint NNN: [Sprint Title]

**Status:** 🔄 In Progress | ✅ Complete\
**Wave:** [wave-category-name]\
**Thin-Slices:** [TS-NNN, TS-NNN]\
**Started:** YYYY-MM-DD\
**Completed:** —

---

## Sprint Goal

_[One sentence: what user value will this sprint deliver?]_

---

## Hypothesis Card (Lean Validation)

**Hypothesis:** We believe [doing X] will result in [observable outcome Y] for [user/system].

**Validation method:** We will know this is true when [measurable signal / test evidence].

**Decision rule:**

- **Continue** if: [condition]
- **Pivot** if: [condition]
- **Stop** if: [condition]

---

## Risks (Pre-Mortem Seed)

Seeded from the `start-thin-slice` pre-mortem (Standard tier) or the `discovery-and-ambiguity-log` risk pass (Major tier). Assume this sprint failed six months out — what went wrong? List the top risks before locking scope.

| Risk | Likelihood | Impact | Mitigation / trigger |
| ---- | ---------- | ------ | -------------------- |
| [What could fail] | L/M/H | L/M/H | [How we prevent it, or the signal that we pivot] |

If this is a Trivial-tier sprint with no meaningful risk, write `n/a (tier: Trivial)`.

---

## Scope (Immutable Once Started)

### Thin-Slices Included

- [ ] TS-NNN: [Name] — [brief description]

If this sprint corrects or reopens a thin-slice, describe the changed outcome here, but keep the original slice ID.

### Out of Scope

- [What we explicitly decided NOT to include and why]

---

## Engineering Current-State Snapshot (Bridge Anchor)

**Codebase:** [Files/modules/capabilities this sprint touches and their current state.]

**Toolchain:** [Frameworks, runtime versions, test/lint tooling that constrain decisions in this sprint.]

**Integrations / providers active:** [Third-party services, feature flags, environment variables in play.]

**Active ADRs that bind this work:** [Links + one-line reasons.]

**Known debt / hazards adjacent to this work:** [Capability layout violations, anti-dumping debt, missing test coverage near the change site.]

---

## Gap Analysis

**Current state:** [What exists today — specific files, endpoints, behaviors.]

**Target state:** [What must exist after this sprint — reference design + architecture sections.]

**Gap to close:**

- [ ] [Missing capability]
- [ ] [Missing UI behavior]
- [ ] [Missing validation]
- [ ] [Missing test coverage]

---

## Implementation Plan

### Phase 1: [Name — e.g., Entity + Repository]

- [ ] [Specific change with file path]

### Phase 2: [Name — e.g., Service Logic]

- [ ] [Specific change with file path]
  - Business rule: [describe]
  - Error: `[ERROR_CODE]` when [condition]

### Phase 3: [Name — e.g., API/UI surface]

- [ ] [Specific change with file path]

### Resilience / Failure-Mode Checklist

A production-grade plan names how the work behaves when things go wrong, not only on the happy path. Fill each item, or mark `N/A` with a one-line reason. These map directly to the capability guardrails (timeout / retry / fallback / circuit breaker).

- [ ] **Idempotency** — repeated execution is safe because: [reason, or N/A]
- [ ] **Concurrency** — behavior under two simultaneous executions: [reason, or N/A]
- [ ] **Offline / degraded dependency** — detection + behavior when a dependency is unreachable: [reason, or N/A]
- [ ] **Version pinning / reproducibility** — exact versions pinned, never `latest`: [reason, or N/A]
- [ ] **Partial-failure recovery** — resume or roll back half-completed work: [reason, or N/A]

### Production-Readiness Conformance (Four Anchors)

The wave's `product-architecture.md` declared a **Production-Readiness posture** for the four anchors. A slice does **not** re-decide that posture — it names the seam(s) it touches and confirms it *preserves* the wave posture. This is conformance to a central decision, not a per-slice litigation (which is what produces `N/A` theater).

**Seams this slice touches:** [`<name>@vN`, … — or "none"]

For each anchor, confirm conformance or declare a reviewed deviation:

- [ ] **Observable** — emits the wave's correlation id and the required structured log/metric on every boundary this slice adds: [conforms / deviation + reason]
- [ ] **Configurable** — no new source-literal config or secret; values are environment-injected per the wave strategy: [conforms / deviation + reason]
- [ ] **Horizontally scalable** — adds no node-local mutable state on the request path; shared state is externalized per the wave's statelessness boundary: [conforms / deviation + reason]
- [ ] **Resilient** — every external/boundary call this slice adds carries the wave's timeout/retry/fallback defaults: [conforms / deviation + reason]

A deviation is a reviewed, recorded exception — not silence. The matching probes (`check-observability-at-seams.sh`, `check-config-externalized.sh`, `check-stateless-request-path.sh`, `check-resilient-boundary.sh`) enforce these at `verify`.

---

## Sprint Plan Approval (Standard & Major tiers)

This is the mechanical home for the "pause here for me to review" checkpoint. For Standard- and Major-tier sprints, implementation does not start until this line is filled in. `intake-code-contribution` refuses to pass to implementation without it.

```
Reviewed by: <name or role>
Date: YYYY-MM-DD
Scope confirmed: <one line — plan reviewed, scope + risks + failure modes understood>
```

If this sprint is Trivial-tier, write `n/a (tier: Trivial)`. This gate is distinct from and additional to the Major-only Design Approval below.

---

## Design Approval (Major-tier sprints only)

This is the **mechanical gate** between architect mode and implementer mode for Major-tier changes. Implementer mode does not start until both signals are present:

- **ADR status:** `Accepted` for ADR.<ID> at `<adr path>` — verified by reading the file's status field.
- **Design Approval line:** filled in below by the human approver.
```

Design Approval Approver: <name or role> Date: YYYY-MM-DD ADR(s): ADR.<ID> (status: Accepted) Notes: <one-line summary of what was approved>

```
If the sprint is Trivial- or Standard-tier, write `n/a (tier: <Trivial|Standard>)` here. Reviewer mode verifies this section before approving any Major-tier PR.

---

## Test Plan (TDD — written before implementation)

Per the Pyramid Test Strategy in `test-by-ownership`, specify Logic / Composition / Adapter Contract / Integration boundary / Journey tests by file and intent. No code in this section — code lives in test files.

### Composition Tests

- [ ] `should [happy path]`
- [ ] `should return 4xx for [auth/validation/business rule failure]`
- [ ] `should return 404 for [not found scenario]`

### Journey Tests (E2E)

- [ ] `[user completes primary workflow]`
- [ ] `[user sees error for invalid input]`

### Logic Tests

- [ ] [Pure function name → equivalence class → expected output]

---

## Acceptance ↔ Test Traceability

Every acceptance criterion maps to at least one test. This matrix is the link the test plan and acceptance criteria otherwise lack — without it, "all tests pass" can be true while an AC is silently uncovered. `intake-code-contribution` checks every AC has a mapped test before implementation; `verify-and-assemble-pr` checks every mapped test actually ran.

| AC ID | Acceptance criterion | Test layer | Test file / name | Evidence | Status |
| ----- | -------------------- | ---------- | ---------------- | -------- | ------ |
| AC-1 | [Given/when/then] | Logic/Composition/Journey | [file → test name] | example / property | ⚪ / 🔴 / 🟢 |

Rule: no AC may be left without a mapped test. If a criterion is genuinely unverifiable by an automated test, state the manual verification method explicitly in the row.

**Property over example at high-risk seams.** For an AC whose Impact is **High** in the risk register **and** which exercises a seam (`<name>@vN`), a single happy-path example is **insufficient** — map it to a **property/contract test** (idempotency ∀ keys, retry-safe ∀ attempts, concurrent ops linearizable) and mark the Evidence column `property`. Example tests remain appropriate for ordinary ACs. This sharpens — it does not replace — the matrix; `verify-and-assemble-pr`'s adversarial seam review (Step 5) rejects a high-risk seam AC backed only by an example.

---

## Acceptance Criteria

Derived verbatim from the wave README thin-slices. Sprint is complete when ALL are satisfied:

- [ ] Given [state], when [action], then [observable result]
- [ ] Error: given [invalid state], when [action], then [error shown]
- [ ] All composition tests pass
- [ ] All journey tests pass (if applicable)
- [ ] Lint passes with zero warnings
- [ ] Format check passes

---

## Completion Checklist

- [ ] All implementation tasks done
- [ ] All tests written and passing
- [ ] Acceptance criteria met

_When done, use `close-sprint` to record outcome evidence, distill learnings into both product and engineering artifacts, then delete this file._

---

## Working Notes (Ephemeral)

_Capture decisions, discoveries, and scope clarifications during the sprint. These are deleted when the sprint closes — distilled learnings flow into wave documents and engineering artifacts._
```

---

## Step 6 — Create the Progress Ledger

The sprint file is the **immutable bridge** — it does not change once work starts. Execution state, however, must survive session death so a multi-session feature does not reconstruct (and drift) its progress from scratch. That state lives in a separate, **mutable** ledger file alongside the sprint:

```
<sprint-dir>/sprint-NNN-<description>.ledger.md
```

Three artifacts, three lifecycles — do not conflate them:

| Artifact | Mutability | Holds |
|---|---|---|
| `sprint-NNN-*.md` | Immutable once started | Scope, AC, hypothesis, test plan, approvals |
| `sprint-NNN-*.ledger.md` | Mutable every session | Plan-phase progress, current test posture, verify-attempt counter |
| Working Notes (in the sprint file) | Mutable scratch | Free-form discoveries, distilled at close |

Create the ledger from this template:

```markdown
# Sprint NNN — Progress Ledger

_Mutable execution state. Survives session death. Deleted at `close-sprint` after learnings are distilled. This is NOT the immutable bridge — never record scope or acceptance-criteria changes here._

## Plan Phase Progress

- [ ] Phase 1: [name]
- [ ] Phase 2: [name]
- [ ] Phase 3: [name]

## Current Test Posture

| Behavior | Layer | State (🔴/🟢) | Last run |
| -------- | ----- | ------------- | -------- |
| [name]   | …     | ⚪            | —        |

## Verify Attempts

- Consecutive failed verifies on the current cause: 0
- Last verify exit code + cause: —
- Stop-rule budget: 3 consecutive failed verifies on the same cause → HALT and escalate (project may override)

## Adversarial Seam Review

- Reviewer head used: [separate session/agent | same-agent reviewer-mode switch with fresh diff read] — recorded per the adversarial-review decision; a separate head carries more assurance than a self-review.

## What's Left

- [next concrete step]
```

`intake-code-contribution` restores state from this ledger when resuming; `close-sprint` deletes it after distillation.

---

## Quality Checklist

- [ ] Sprint number is sequential
- [ ] Thin-slices referenced from a wave README
- [ ] Corrections or reopens keep the original thin-slice ID
- [ ] **Engineering current-state snapshot captured (codebase, toolchain, integrations, ADRs, debt)**
- [ ] Gap analysis closes the gap between target state and the snapshot — not just copied from design docs
- [ ] Hypothesis card included (hypothesis + validation method + continue/pivot/stop rule)
- [ ] Risk register seeded (top risks with likelihood/impact/mitigation, or `n/a (tier: Trivial)`)
- [ ] Implementation plan has specific file names and function names
- [ ] Resilience / failure-mode checklist filled or each item marked N/A with reason
- [ ] Production-Readiness conformance block present; seams named and each anchor confirmed conforming or a reviewed deviation recorded
- [ ] Test plan written in TDD format (tests before implementation), categorized by layer
- [ ] Acceptance ↔ test traceability matrix present; every AC maps to ≥1 test
- [ ] High-risk seam ACs (Impact = H, touching a `<name>@vN` seam) mapped to a property/contract test, not a single example
- [ ] Acceptance criteria match the wave README exactly
- [ ] Sprint Plan Approval line present (signed for Standard/Major, `n/a (tier: Trivial)` otherwise)
- [ ] Scope is realistic (1–2 thin-slices typical)
- [ ] Out-of-scope section explicitly documents excluded work
- [ ] Progress ledger file created alongside the sprint (`sprint-NNN-*.ledger.md`)

---

## Anti-Patterns

- Treating the sprint as an editable plan — it is an immutable contract
- Skipping the engineering current-state snapshot ("everyone knows the codebase")
- Letting acceptance criteria drift from the wave README
- Inventing replacement thin-slice IDs for corrections
- Writing implementation tasks before tests
- Leaving acceptance criteria with no mapped test in the traceability matrix
- Listing happy-path tasks only and skipping the resilience / failure-mode checklist
- Starting Standard/Major implementation before the Sprint Plan Approval line is signed
- Using sprint working notes as a permanent decision log (they are deleted at close)
