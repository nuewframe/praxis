---
name: create-adr
mode: architect
tools: [
  read_file,
  file_search,
  grep_search,
  semantic_search,
  create_file,
  replace_string_in_file,
]
description: >
  Create an Architecture Decision Record (ADR) for a durable technical decision. Architect-mode
  skill: writes are limited to the project's ADR directory. Includes sequential numbering,
  mandatory alternatives comparison table, consequences (positive + negative + risks),
  registration in the project's ADR index, and a `status` lifecycle (`Proposed` → `Accepted`
  → `Superseded`). Implementer mode requires `status: Accepted` before Major-tier work begins.
user-invocable: true
disable-model-invocation: false
---

# Skill: Create ADR

Use this skill when making a significant technical decision that needs durable documentation.

---

## Project Conventions

This skill assumes the host project defines:

- An **ADR directory** (commonly `docs/product/adr/` or `docs/adr/`)
- An **ADR index** (commonly a table in `project-context.md` or a dedicated `README.md` in the ADR directory)
- ADR file naming: `ADR-NNN-descriptive-name.md`

If those don't exist, define them in the project's own context first.

---

## When to Create an ADR

Create an ADR for a decision that:

- Selects a technology, framework, or library that will be hard to change
- Defines an architectural pattern or constraint that binds future work
- Establishes a security or infrastructure approach
- Changes an existing decision (supersedes a prior ADR)
- Resolves a significant design disagreement

**Do NOT create an ADR for:** implementation details, refactoring without strategic change, minor dependency upgrades.

---

## Step 1 — Determine the Next ADR Number

List the ADR directory and find the highest `ADR-NNN`. Increment by 1. No gaps, no duplicates.

---

## Step 2 — Create the File

`ADR-NNN-descriptive-name.md` — kebab-case, concise.

```markdown
# ADR-NNN: [Decision Title]

**Status:** Proposed | Accepted | Deprecated | Superseded by ADR-NNN **Date:** YYYY-MM-DD **Deciders:** [Names or roles]

> **Approval mechanics:** `status` is the mechanical gate between architect mode and implementer mode for Major-tier changes. Implementer mode REJECTS the work if `status` is not `Accepted`. Pair this status with a signed Design Approval line in the active sprint file (see `create-sprint`). Both signals are required.

---

## Context

[2-4 sentences: What is the problem or situation forcing this decision? What constraints, goals, or requirements must we satisfy? Be specific — technical, operational, and business context.]

---

## Decision

**We will [specific, unambiguous decision statement].**

[1-3 sentences elaborating on the key aspects.]

---

## Rationale

[Why is this the right choice given the context? Reference evaluation criteria. Connect back to the constraints from Context.]

| Criterion     | How This Decision Satisfies It |
| ------------- | ------------------------------ |
| [Criterion 1] | [Explanation]                  |
| [Criterion 2] | [Explanation]                  |

---

## Alternatives Considered

<!-- MANDATORY: Every ADR must include a comparison table with at least 2 alternatives -->

| Option                  | Pros        | Cons         | Why Not Chosen    |
| ----------------------- | ----------- | ------------ | ----------------- |
| **[Option A — Chosen]** | [Strengths] | [Weaknesses] | Selected          |
| **[Option B]**          | [Strengths] | [Weaknesses] | [Reason rejected] |
| **[Option C]**          | [Strengths] | [Weaknesses] | [Reason rejected] |

---

## Consequences

### Positive

- [What this enables]
- [What problems it solves]

### Negative

- [What this constrains]
- [What complexity it introduces]
- [What future options it closes off]

### Risks & Mitigations

| Risk     | Likelihood      | Mitigation          |
| -------- | --------------- | ------------------- |
| [Risk 1] | High/Medium/Low | [How we address it] |

---

## Implementation Notes

[Optional: specific guidance for implementation. Reference relevant files, commands, or existing patterns.]

---

## Related Documents

- [Link to related ADRs]
- [Link to relevant handbook section]
- [Link to wave or sprint that implements this]
```

---

## Step 3 — Register in the ADR Index

Add the ADR to the project's index (e.g. table in `project-context.md`):

```markdown
| [ADR-NNN: Decision Title](path/ADR-NNN-descriptive-name.md) | [Brief purpose, e.g., "Real-time sync strategy (Accepted)"] |
```

---

## Step 4 — Cross-Reference if Strategic

If the ADR represents a significant architectural milestone, link it from:

- The product dashboard under the relevant wave or platform section
- The wave `product-architecture.md` that triggered the decision
- Any superseded ADR (mark the older one as `Superseded by ADR-NNN`)

---

## Quality Checklist

- [ ] Sequential number (no gaps, no duplicates)
- [ ] Status set (`Proposed` until reviewed, `Accepted` after team alignment)
- [ ] Status field is **machine-readable on a single line** (`**Status:** Accepted`) so the validator and implementer-mode gate can parse it
- [ ] Context explains the _why_, not the solution
- [ ] Decision statement is unambiguous
- [ ] Alternatives comparison table present with at least 2 alternatives
- [ ] Consequences cover both positive **and** negative outcomes
- [ ] Registered in the ADR index
- [ ] File named correctly: `ADR-NNN-descriptive-name.md`
- [ ] When this ADR is paired with a Major-tier sprint, the sprint file's Design Approval line references this ADR by number

---

## Anti-Patterns

- Writing the ADR after the decision is already shipped (use `Accepted` retroactively only when necessary)
- Listing only one alternative — the comparison table must show real options
- Omitting negative consequences ("there are no downsides")
- Marking everything `Accepted` without a deciders field
- Embedding implementation code instead of pattern guidance
- Implementer mode flipping `status` from `Proposed` to `Accepted` to unblock itself — only architect mode + a human approver may set `Accepted`
