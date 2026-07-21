---
name: create-product-design-spec
description: >
  Author or refine a planning-stage wave `product-design.md`. Specifies user journeys, UX states,
  acceptance-ready behavior, ambiguity handling, and recovery paths so architecture, QA, and
  sprint planning can proceed without guessing — without prescribing implementation.
user-invocable: true
disable-model-invocation: false
---

# Skill: Create Product Design Spec

Use this skill when authoring or updating a wave's `product-design.md`.

**Audience:** Product designer, principal engineer, product manager. **Purpose:** Define the ideal experience with enough precision that architecture, QA, and sprints can proceed without follow-up questions.

**Reference template:** `templates/thin-slice-template.md` — fill-in template for individual thin-slice acceptance entries. The wave README aggregates these; `product-design.md` deepens the journey, ambiguity, and recovery story around them.

---

## What This Skill Produces

A planning-stage design specification for a wave. It does **not** produce code, component props, API contracts, or database plans. The wave README remains the only place that tracks thin-slice status or correction notes.

A strong `product-design.md` answers:

- What problem does this wave solve, for which user?
- What should the user see, do, and understand at each step?
- Where does the experience begin, branch, fail, recover, and end?
- What must always be true from the user's point of view?
- What behavior must the engineer implement and the test suite validate?

---

## Required Inputs

Read before drafting:

- The project's `project-context.md` (or equivalent)
- The product dashboard
- The wave `README.md`
- The existing `product-design.md` if updating
- Adjacent wave docs if the journey crosses wave boundaries

If the wave README does not yet define thin-slices and acceptance criteria, stop and fix that first.

---

## Step 1 — Define the Experience Spine

Capture the minimum context the engineer and QA need:

- Primary user persona
- User problem in plain language
- One-sentence value outcome for this wave
- 2–4 design principles that constrain later decisions

Design principles are decision-making rules, not slogans. Examples:

- Confirm, don't surprise
- Resolve ambiguity explicitly
- Keep coordination inside the existing flow
- Always preserve a manual recovery path

---

## Step 2 — Map the User Journey End to End

Describe the ideal experience as a sequence of observable states.

For each major surface or flow, include:

- Entry point: where the user begins
- Trigger: what action they take
- Visible state: what appears on screen
- Decision point: what choice they make next
- Completion state: what successful progress looks like

Organize by meaningful experience surfaces, not by component tree.

---

## Step 3 — Specify Ambiguity, Empty, and Error States

Every flow must cover more than the happy path:

- Missing information: what the user sees when required input is absent
- Ambiguity: what happens when the system cannot confidently infer intent
- Permission limits: what unauthorized users can and cannot do
- Failure state: what the user sees when the action fails
- Recovery path: how the user retries, edits, or falls back
- Cancellation path: how the user backs out without damage

The goal is binary behavior that the test suite can verify without reading code.

---

## Step 4 — Make Cross-Cutting Interaction Rules Explicit

Document rules that hold across the whole experience. Examples:

- AI-generated output is always shown before any send action
- Draft content is editable before publishing
- Missing optional fields appear as prompts, not blockers
- The user never loses context when correcting one ambiguous field

These rules are often more important than any single screen description.

---

## Step 5 — Give Engineering Enough Shape, Without Designing the Code

Include:

- User-visible inputs and outputs
- State transitions that require backend support
- Which actions need confirmation, streaming, retry, or asynchronous feedback
- Privacy, trust, and tone expectations visible to the user

Do **not** include API endpoints, file paths, schema changes, service boundaries, or implementation tasks. If you find yourself naming handlers, repositories, or payload schemas, you have crossed into the architecture document.

---

## Step 6 — Use This Structure

```markdown
# [Wave Name]: Product Design

> **Planning-stage document — an educated theory, not yet the truth.** The best approach given what we know today; current-state architecture lives in [docs/architecture/](../../../architecture/), promoted there by `close-sprint`.

> **Wave**: wave-[category]-[name]\
> **Updated**: YYYY-MM-DD

---

## Design Philosophy

[Short paragraphs describing the user problem, intended outcome, and design principles.]

## UX Surface 1: [Primary Flow Name]

### Entry Point

### Primary Interaction

### Completion State

### Ambiguity and Recovery

## UX Surface 2: [Secondary Flow Name]

[Repeat as needed.]

## Cross-Cutting Interaction Rules

- [Rule 1]
- [Rule 2]

## Error States

### When [condition]

[Observable message, blocked action, recovery path.]

## Success Signals

- [Observable metric or behavioral signal]
```

ASCII wireframes are allowed when they clarify layout. Use sparingly.

---

## Quality Checklist

- [ ] Document is in present tense
- [ ] Each major journey has an entry point, flow, completion state, and recovery path
- [ ] Error and ambiguity states are explicit, not implied
- [ ] Cross-cutting interaction rules are stated clearly
- [ ] Engineering can infer required technical support without being told how to code it
- [ ] QA can derive observable pass/fail scenarios from the document
- [ ] No implementation details appear in the spec

---

## Anti-Patterns

- Turning `product-design.md` into a thin-slice tracker or sprint plan
- Duplicating wave status or correction history that belongs in the wave README
- Naming endpoints, repositories, or schema changes
- Vague criteria like "the user can fix errors easily"
- Leaving trust, permissions, or failure behavior unspecified
- Describing component internals instead of user-visible behavior
