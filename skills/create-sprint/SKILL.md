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

---

## Design Approval (Major-tier sprints only)

This is the **mechanical gate** between architect mode and implementer mode for Major-tier changes. Implementer mode does not start until both signals are present:

- **ADR status:** `Accepted` for ADR-NNN at `<adr path>` — verified by reading the file's status field.
- **Design Approval line:** filled in below by the human approver.
```

Design Approval Approver: <name or role> Date: YYYY-MM-DD ADR(s): ADR-NNN (status: Accepted) Notes: <one-line summary of what was approved>

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

## Quality Checklist

- [ ] Sprint number is sequential
- [ ] Thin-slices referenced from a wave README
- [ ] Corrections or reopens keep the original thin-slice ID
- [ ] **Engineering current-state snapshot captured (codebase, toolchain, integrations, ADRs, debt)**
- [ ] Gap analysis closes the gap between target state and the snapshot — not just copied from design docs
- [ ] Hypothesis card included (hypothesis + validation method + continue/pivot/stop rule)
- [ ] Implementation plan has specific file names and function names
- [ ] Test plan written in TDD format (tests before implementation), categorized by layer
- [ ] Acceptance criteria match the wave README exactly
- [ ] Scope is realistic (1–2 thin-slices typical)
- [ ] Out-of-scope section explicitly documents excluded work

---

## Anti-Patterns

- Treating the sprint as an editable plan — it is an immutable contract
- Skipping the engineering current-state snapshot ("everyone knows the codebase")
- Letting acceptance criteria drift from the wave README
- Inventing replacement thin-slice IDs for corrections
- Writing implementation tasks before tests
- Using sprint working notes as a permanent decision log (they are deleted at close)
