---
name: create-product-design-spec
description: >
  Author or refine user design specifications in an initiative (INIT.<name>.md) or living capability record (CAP.<name>.md).
  Specifies user journeys, Given/When/Then acceptance criteria, UX state matrices, ambiguity handling, and error recovery paths.
user-invocable: true
disable-model-invocation: false
---

# Skill: Create Product Design Spec

Use this skill when authoring or refining user experience specifications.

**Audience:** Product Designer, Principal Engineer, Product Manager. **Purpose:** Define the user experience with enough precision that architecture, QA, and sprints can proceed without follow-up questions.

---

## What This Skill Produces

Refines the **User Experience (UX Deltas)** section of an initiative file (`docs/product/initiatives/INIT.<initiative-name>.md`) or promotes validated UX into a living capability record (`docs/capabilities/CAP.<capability-name>.md#user-experience`).

A complete UX spec answers:
- What problem does this initiative solve, for which user persona?
- What should the user see, do, and understand at each step?
- What are the binary, testable acceptance criteria in Given/When/Then format?
- Where does the experience begin, branch, fail, recover, and end?

---

## Required Inputs

Read before drafting:
- [`docs/product.md`](../../docs/product.md) — Unified product context & active roadmap
- `docs/product/design.md` <!-- praxis:allow-path reason="illustrative global design path" --> — Global UX design system & personas
- Target initiative: `docs/product/initiatives/INIT.<initiative-name>.md`
- Related capability records: `docs/capabilities/CAP.<capability-name>.md`

---

## Step 1 — Define Persona & Experience Spine

Capture the core user problem and design principles:
- **Primary Persona:** [Target user role / persona]
- **User Goal:** Plain-language statement of what the user wants to accomplish.
- **Design Principles:** 2–4 constraints (e.g., *Fail gracefully with clear recovery; Prefer explicit confirmation over silent defaults*).

---

## Step 2 — Specify Binary Acceptance Criteria (Given/When/Then)

Every thin-slice (`TS-NNN`) must carry binary, testable acceptance criteria:

```markdown
### Slice `TS-001`: [Slice Title]

#### Acceptance Criteria

- **AC-1 (Happy Path):**
  - **Given** a logged-in user on the dashboard with active project permissions,
  - **When** they click "New Initiative" and enter a valid title and summary,
  - **Then** the initiative is created in `⚪ Proposed` status and the editor opens.

- **AC-2 (Validation Failure):**
  - **Given** the initiative creation form is open,
  - **When** the user submits an empty title,
  - **Then** an inline error "Title is required" is displayed and focus moves to the title field.

- **AC-3 (Network Recovery):**
  - **Given** a user submitting an initiative when network connection drops,
  - **When** submission fails,
  - **Then** a banner displays "Offline — draft saved locally" with a "Retry" button.
```

---

## Step 3 — Build State Transition & Ambiguity Matrix

Detail all user-facing state transitions and edge cases:

| Current View / State | Trigger Event | Next Visible State | Error / Ambiguity Path |
| -------------------- | ------------- | ------------------ | ---------------------- |
| Empty Dashboard | Page Load | Empty State Banner ("Create your first initiative") | Failed fetch $\rightarrow$ Retry button |
| Form Editing | User Typing | Dirty State Indicator | Unsaved changes confirmation on navigate |
| Submitting | Click Submit | Disabled submit + Loading Spinner | Timeout (5s) $\rightarrow$ Keep form inputs intact |

---

## Step 4 — Specify UX Recovery & Cancellation Paths

Every flow must cover non-happy paths:
1. **Empty State:** Guidance displayed when no data exists.
2. **Ambiguity / Permission Limits:** Clear feedback when a user lacks access or inputs are incomplete.
3. **Failure State & Recovery:** Actionable error messages with inline recovery controls (retry, edit, fallback).
4. **Cancellation Path:** Safe exit path without unintended state mutation.

---

## Step 5 — Update the Target Document

1. **For Initiative UX Deltas:** Update `## Progressive Refinement -> User Experience (UX Deltas)` in `docs/product/initiatives/INIT.<initiative-name>.md`.
2. **For Global UX Patterns:** Update `docs/product/design.md` <!-- praxis:allow-path reason="illustrative global design path" -->.
3. **For Living Capability UX:** Promote validated UX into `docs/capabilities/CAP.<capability-name>.md#user-experience` at sprint close.

---

## Quality Checklist

- [ ] Document is in present tense
- [ ] Acceptance criteria written in binary Given/When/Then format
- [ ] Every journey has entry point, state transitions, completion, and recovery paths
- [ ] Empty, error, and cancellation states explicit
- [ ] Implementation details (class names, SQL queries, endpoints) kept out of UX spec
