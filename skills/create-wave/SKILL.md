---
name: create-wave
description: >
  Create a new product wave following the four-document wave pattern: README (intent + thin-slices),
  product-design, product-architecture, and quality spec. Registers the wave in the project's
  product dashboard. Invoke when planning a new feature wave or platform initiative.
user-invocable: true
disable-model-invocation: false
---

# Skill: Create Wave

Use this skill when starting a new product wave.

A **wave** is the unit of intent in the Nuewframe Method — a coherent slice of product value delivered through a coordinated set of thin-slices. Waves outlive sprints and survive reorganization. They are how the team holds the line on what the product is becoming.

This skill creates the wave scaffold. After the scaffold exists:

- Use `create-product-design-spec` to author `product-design.md`
- Use `create-product-architecture-spec` to author `product-architecture.md` (the wave's **bet**, not the durable record)
- Use `create-quality-spec` to author `qa.md`
- Use `design-system-architecture` when the wave introduces a new subsystem, runtime boundary, or platform-wide constraint — it persists the durable architecture into `docs/architecture/`
- Use `create-adr` when the wave makes a durable technical decision
- Use `author-user-docs` (TEACH) once a capability's behavior ships, to render the capability record into user guides in `docs/guides/`

**Wave = bet; capability record = truth.** The wave's `product-architecture.md` is a planning-stage hypothesis. The living, validated architecture lives in the durable tree — `docs/architecture/README.md` (system overview) and `docs/architecture/<capability>/` (capability records) — and is promoted there by `close-sprint`.

## Project conventions

This skill assumes the host project defines:

- A **wave directory** (commonly `docs/product/waves/`)
- A **product dashboard** (commonly `docs/product/PRODUCT.md`)
- A **wave naming convention** (e.g. `wave-feature-*`, `wave-platform-*`, `wave-ext-*`)

Read the project's `project-context.md` and existing wave directory before invoking this skill. If those conventions don't exist yet, define them in the project's own context first.

---

## Step 1 — Name and Categorize the Wave

| Category          | Pattern                | Use When                        |
| ----------------- | ---------------------- | ------------------------------- |
| `wave-feature-*`  | User-facing capability | New user-visible feature set    |
| `wave-platform-*` | Infrastructure         | Foundation required by features |
| `wave-ext-*`      | Extension              | Optional add-on capability      |

Examples: `wave-feature-home-shell`, `wave-platform-identity-access`, `wave-ext-business-booking`.

---

## Step 2 — Create Wave Folder + Four Documents

```
<wave-root>/wave-<category>-<name>/
  README.md
  product-design.md
  product-architecture.md
  qa.md
```

### `README.md` — Intent + Thin-Slice Tracking

```markdown
# WAVE-[CATEGORY]: [Wave Name]

**Status:** ⚪ Not Started | 🔄 In Progress | ✅ Complete\
**Goal:** [One sentence describing the user outcome this wave delivers.]

---

## Tracking Rules

- Track each thin-slice by intended user outcome and current state only.
- If a thin-slice is reopened or corrected, keep the same slice ID and add one short tracking note next to that slice.
- Keep implementation history in sprint files and version control. This README stays focused on product intent.

---

## Value Theme

_[One sentence describing the product outcome or value theme.]_

---

## Scope

- [Core capability this wave owns]
- [Core capability this wave owns]

**Out of scope:**

- [Adjacent work tracked in another wave]

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

### TS-002: [Slice Name]

> **Status:** ⚪ Not Started

[repeat pattern]

---

## Success Criteria

Wave is complete when:

- [ ] All thin-slices are ✅ Complete
- [ ] Journey tests pass for all primary scenarios
- [ ] User guides updated (TEACH) for capabilities whose user-observable behavior changed — via `author-user-docs`
- [ ] Product dashboard updated to reflect completion

---

## Dependencies

- **Requires:** [What must be done before this wave]
- **Enables:** [What this wave unlocks]
```

---

### `product-design.md`, `product-architecture.md`, `qa.md`

Create empty stubs with file titles. Author each through its dedicated skill:

- `product-design.md` → `create-product-design-spec`
- `product-architecture.md` → `create-product-architecture-spec`
- `qa.md` → `create-quality-spec`

If the wave introduces system-wide concerns (new subsystem, integration, runtime boundary), also invoke `design-system-architecture` — which writes the durable topology and resilience posture into `docs/architecture/` — and pair with `create-adr`. The wave documents stay the bet; the durable record lives under `docs/architecture/`.

---

## Step 3 — Register in the Product Dashboard

Add or update the wave row in the project's product dashboard (e.g. `docs/product/PRODUCT.md`):

- Wave row in the roadmap table
- Wave summary section if the dashboard tracks that wave family
- Keep the dashboard quick-glance only: counts, current state, links
- Do not duplicate thin-slice correction notes from the wave README

---

## Quality Checklist

- [ ] Wave folder created with correct naming pattern
- [ ] All four documents created (README, product-design, product-architecture, qa)
- [ ] At least one thin-slice defined with acceptance criteria in README
- [ ] Thin-slices are written as atomic user outcomes, not implementation buckets
- [ ] Any correction or reopen stays on the same slice ID with a short tracking note
- [ ] Wave registered in the product dashboard
- [ ] Dependencies documented

---

## Anti-Patterns

- Treating waves as sprints with bigger scope (waves are intent; sprints are bridges to reality)
- Writing thin-slices as implementation tasks instead of user outcomes
- Allowing thin-slice tracking notes to become a changelog
- Splitting a single user outcome across multiple thin-slices to look like progress
