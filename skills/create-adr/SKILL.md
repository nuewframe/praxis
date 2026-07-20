---
name: create-adr
mode: architect
tools: [read_file, file_search, grep_search, semantic_search, create_file, replace_string_in_file]
description: >
  Create an Architecture Decision Record (ADR) for a durable technical decision. Architect-mode
   skill: writes are limited to the project's ADR directory. Includes collision-safe ADR IDs,
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

ADRs live in the durable architecture tree, homed at the **altitude of the decision**:

- **Capability-scoped decision** → `docs/architecture/<capability>/adr/ADR.<ID>-descriptive-name.md`
- **Cross-capability / system-wide decision** → `docs/architecture/adr/ADR.<ID>-descriptive-name.md`

Each ADR is registered in an **ADR index** — a table in the capability record (`docs/architecture/<capability>/README.md`), or in the system overview (`docs/architecture/README.md`) for cross-capability decisions.

### ADR (immutable) vs. capability record (living)

An ADR is an **immutable decision point**. It records *why* a durable choice was made, with a diagram of the shape **as of that decision**. It never mutates to track drift — a superseding ADR replaces it.

The **living, current-state** architecture lives elsewhere and is edited in place:

- **Capability record** (`docs/architecture/<capability>/`) — the truth for one capability.
- **System overview** (`docs/architecture/README.md`) — the cross-capability topology and product-wide posture.

The wave `product-architecture.md` is the **hypothesis** (the educated theory); the capability record is the **truth**. An ADR is the frozen decision between them. If these homes don't exist yet, define them in the project's own context first.

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

## Step 1 — Determine the Next ADR ID

The `ID` uses `<YYMMDD>[.<HH>[MM[SS]]][.<seq>]`. IDs must be unique.

- `YYMMDD` — required. Two-digit year, month, day (UTC creation date).
- `.HH` — optional. Two-digit hour (24h, UTC). Added on day collision.
- `MM` — optional. Two-digit minute. Added on hour collision (always paired with `HH` → written as `.HHMM`).
- `SS` — optional. Two-digit second. Added on minute collision (always paired with `HHMM` → written as `.HHMMSS`).
- `.<seq>` — optional, **final tiebreaker**. Zero-padded decimal sequence starting at `01`. Width grows as needed (`.01`..`.99`, then `.100`, `.101`, …). Added only when the precision ladder above is exhausted *or* when sub-second resolution is unavailable.

### Collision Escalation Rules

When registering a new ADR, check the project's ADR directory for existing IDs and pick the shortest unique suffix for the new ADR only. Existing ADR IDs are immutable after publication. Extend left-to-right along the precision ladder, then fall through to the sequence tiebreaker:

1. **No same-day ADR exists.** Use `YYMMDD` only.
   - Example: `ADR.260527-data-architecture`
2. **Day collision; different hour.** Add `.HH`.
   - Example: existing `ADR.260527`; new ADR at 03:00 UTC → `ADR.260527.03-data-architecture`.
3. **Hour collision; different minute.** Extend to `.HHMM`.
   - Example: existing `ADR.260527.03`; new ADR at 03:17 → `ADR.260527.0317-data-architecture`.
4. **Minute collision; different second.** Extend to `.HHMMSS`.
   - Example: existing `ADR.260527.0317`; new ADR at 03:17:45 → `ADR.260527.031745-data-architecture`.
5. **Second collision (or no sub-second clock available).** Append the sequence tiebreaker `.<seq>`, starting at `.01` and incrementing to the next free slot. The sequence grows beyond two digits when needed (`.99` → `.100`).
   - Example: 100 people create an ADR in the same second → `ADR.260527.031745.01`, `ADR.260527.031745.02`, …, `ADR.260527.031745.99`, `ADR.260527.031745.100`.
6. **Do not rename older ADRs to match a new collision level.** Collision handling applies to the new ADR only.

---

## Step 2 — Create the File

`ADR.<ID>-descriptive-name.md` — kebab-case, concise.

```markdown
# ADR.<ID>: [Decision Title]

**Status:** Proposed | Accepted | Deprecated | Superseded by ADR.<ID> **Date:** YYYY-MM-DD **Deciders:** [Names or roles]

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

## Architecture Snapshot (as of this decision)

<!-- The shape this decision commits to, frozen at decision time. This is a
     point-in-time snapshot, NOT the living architecture. Current-state topology
     lives in the capability record (docs/architecture/<capability>/). -->

```mermaid
flowchart LR
  %% Boxes = capabilities/components this decision commits to.
  %% Edges = call types (sync REST/gRPC, async event). Mark trust boundaries.
```

Resilience posture committed by this decision (only if the decision sets one):

| Boundary call | Timeout | Retry           | Fallback / degraded behavior |
| ------------- | ------- | --------------- | ---------------------------- |
| [call]        | [e.g. 2s] | [e.g. 3x, jitter] | [what the user sees on failure] |

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

- **Capability record (living architecture this decision shapes):** `docs/architecture/<capability>/README.md` — REQUIRED
- **Supersedes / Superseded by:** ADR.<ID> (if any)
- [Link to related ADRs]
- [Link to the wave or sprint that triggered this decision]
```

---

## Step 3 — Register in the ADR Index

Add the ADR to the index at its altitude — the capability record for a capability-scoped decision, or the system overview for a cross-capability one:

```markdown
| [ADR.<ID>: Decision Title](adr/ADR.<ID>-descriptive-name.md) | [Brief purpose, e.g., "Real-time sync strategy (Accepted)"] |
```

---

## Step 4 — Cross-Reference if Strategic

Every ADR links to the **capability record it shapes**. If the ADR represents a significant architectural milestone, also link it from:

- The capability record (`docs/architecture/<capability>/README.md`) — always; update the record's current-state to reflect the decision
- The system overview (`docs/architecture/README.md`) when the decision is cross-capability
- The product dashboard under the relevant wave or platform section
- The wave `product-architecture.md` that triggered the decision
- Any superseded ADR (mark the older one as `Superseded by ADR.<ID>`)

---

## Quality Checklist

- [ ] ADR ID is unique (no duplicates)
- [ ] Status set (`Proposed` until reviewed, `Accepted` after team alignment)
- [ ] Status field is **machine-readable on a single line** (`**Status:** Accepted`) so the validator and implementer-mode gate can parse it
- [ ] Context explains the _why_, not the solution
- [ ] Decision statement is unambiguous
- [ ] Architecture snapshot diagram included (the shape as of this decision)
- [ ] Alternatives comparison table present with at least 2 alternatives
- [ ] Consequences cover both positive **and** negative outcomes
- [ ] Links to the capability record it shapes; that record is updated to current-state
- [ ] Registered in the ADR index (capability record, or system overview if cross-capability)
- [ ] File named correctly: `ADR.<ID>-descriptive-name.md`
- [ ] When this ADR is paired with a Major-tier sprint, the sprint file's Design Approval line references this ADR by number

---

## Anti-Patterns

- Writing the ADR after the decision is already shipped (use `Accepted` retroactively only when necessary)
- Listing only one alternative — the comparison table must show real options
- Omitting negative consequences ("there are no downsides")
- Marking everything `Accepted` without a deciders field
- Embedding implementation code instead of pattern guidance
- Editing a published ADR to track architecture drift instead of superseding it and updating the capability record in place
- Duplicating the living current-state topology into the ADR — the snapshot is frozen as-of-decision; the capability record owns current-state
- Implementer mode flipping `status` from `Proposed` to `Accepted` to unblock itself — only architect mode + a human approver may set `Accepted`
