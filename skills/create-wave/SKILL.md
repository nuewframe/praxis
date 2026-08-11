---
name: create-wave
description: >
  Create a new product initiative (wave) as a single, intent-named file (INIT.<initiative-name>.md)
  that refines iteratively from high-level intent into detailed specs. Registers the initiative
  in the unified product dashboard (docs/product.md). Invoke when planning a new growth initiative.
user-invocable: true
disable-model-invocation: false
---

# Skill: Create Initiative (Wave)

Use this skill when starting a new product initiative (wave).

An **initiative** (wave) is the unit of intent in the Nuewframe Method — a coherent slice of product value delivered through a coordinated set of thin-slices. Initiatives outlive sprints and survive reorganization. They are how the team holds the line on what the product is becoming.

This skill creates a single, intent-named initiative file: `docs/product/initiatives/INIT.<initiative-name>.md`.

Initiatives start **lean** on Iteration 1 (high-level intent + thin-slices) and **refine progressively** across iterations as data, prototypes, or telemetry arrive:

- Use `create-product-design-spec` to refine the UX section inside `INIT.<name>.md` (or `docs/product/design.md` <!-- praxis:allow-path reason="illustrative global design path" --> for global system UX)
- Use `create-product-architecture-spec` to refine technical seams inside `INIT.<name>.md` (the educated theory, promoted to `docs/capabilities/CAP.<name>.md` at sprint close)
- Use `create-quality-spec` to refine NFRs and QA invariants inside `INIT.<name>.md`
- Use `create-adr` when the initiative makes a durable technical decision (`docs/architecture/adr/ADR.<YYMMDD>.<seq>.md`)
- Use `author-user-docs` (TEACH) once capability behavior ships to render user guides

**Initiative = educated theory; capability record = truth.** An initiative's design and architecture notes are a planning-stage hypothesis. The living, validated architecture lives in `docs/capabilities/CAP.<capability-name>.md` and is promoted there by `close-sprint`. Once validated, the initiative dissolves/archives—preventing document bloat.

---

## Project conventions

This skill assumes the host project defines:

- An **initiatives directory** (`docs/product/initiatives/`)
- A **product dashboard** ([`docs/product.md`](../../docs/product.md))
- An **initiative naming grammar**: `INIT.<initiative-name>.md`

Read [`docs/product.md`](../../docs/product.md) before invoking this skill.

---

## Step 1 — Name the Initiative

Name the file `INIT.<initiative-name>.md`, after the product outcome it delivers. A reader should be able to tell from the name alone what the product does differently once the initiative lands.

Examples: `INIT.home-shell.md`, `INIT.passkey-authentication.md`, `INIT.annual-subscriptions.md`.

---

## Step 2 — Scaffold the Initiative File (`INIT.<initiative-name>.md`)

Create `docs/product/initiatives/INIT.<initiative-name>.md` from this template:

```markdown
# INITIATIVE: [Initiative Name]

> **Planning-stage document — an educated theory, not yet the truth.** The best approach given what we know today; living capability truth lives in [docs/capabilities/](../../capabilities/), promoted there by `close-sprint`.

**Status:** ⚪ Not Started | 🔄 In Progress | ✅ Complete\
**Goal:** [One sentence describing the user outcome this initiative delivers.]

---

## Tracking Rules

- Track each thin-slice by intended user outcome and current state only.
- If a thin-slice is reopened or corrected, keep the same slice ID (`TS-NNN`) and add one short tracking note.
- Keep implementation history in sprint files and version control. This initiative file stays focused on intent.

---

## Value Theme & Strategic Context

_[One sentence describing the product outcome, business metric target, or value theme.]_

---

## Scope

- [Core capability this initiative owns]
- [Core capability this initiative owns]

**Out of scope:**

- [Adjacent work tracked in another initiative]

---

## Thin-Slices

### TS-001: [Slice Name]

> **Status:** ⚪ Not Started

**User Value:** As a [user type], I need [capability] so that [outcome].

**Tracking note:** [Only include when the slice is corrected, reopened, or intentionally narrowed. Otherwise omit.]

**Acceptance Criteria:**

- [ ] Given [state], when [action], then [observable result]
- [ ] Given [state], when [action], then [observable result]
- [ ] Error: given [invalid state], when [action], then [error shown]

**Dependencies:** None | TS-XXX

---

## Progressive Refinement (Iterative Specs)

_Fill these sections in iteratively as data, prototypes, or telemetry arrive across iterations. Small initiatives may leave these lean; complex initiatives expand them in-place._

### User Experience (UX Deltas)

- [Primary flow transitions, UI states, error handling]

### Technical Architecture (Seams & Educated Theory)

- [Seam contracts touched, Ports/Adapters, storage changes]

### Quality & NFR Invariants

- [Target latency p99, throughput, error budget, test layer mapping]

---

## Success Criteria

Initiative is complete when:

- [ ] All thin-slices are ✅ Complete
- [ ] Journey tests pass for all primary scenarios
- [ ] User guides updated (TEACH) for capabilities whose user-observable behavior changed — via `author-user-docs`
- [ ] Learnings promoted to `docs/capabilities/CAP.<name>.md` and dashboard index updated in `docs/product.md`

---

## Dependencies

- **Requires:** [What must be done before this initiative]
- **Enables:** [What this initiative unlocks]
```

---

## Step 3 — Register in the Product Dashboard

Add the initiative entry to the active roadmap index in [`docs/product.md`](../../docs/product.md):

```markdown
- [INIT.<initiative-name>.md](product/initiatives/INIT.<initiative-name>.md) — ⚪ Not Started ([Target Outcome])
```

Keep the dashboard index quick-glance only: state, title, links. Do not duplicate thin-slice details in `docs/product.md`.

---

## Quality Checklist

- [ ] Initiative file created as `docs/product/initiatives/INIT.<initiative-name>.md`
- [ ] Title opens with the planning-stage educated-theory banner
- [ ] At least one thin-slice defined with acceptance criteria
- [ ] Thin-slices are written as atomic user outcomes (`TS-NNN`), not implementation buckets
- [ ] Initiative registered in `docs/product.md` roadmap index
- [ ] Dependencies documented

---

## Anti-Patterns

- Creating subfolders or nested generic `README.md` files for an initiative (initiatives are single files)
- Forcing full 4-document scaffolding up front before data is collected (refine iteratively instead)
- Writing thin-slices as implementation tasks instead of user outcomes
- Letting initiative tracking notes become a changelog
