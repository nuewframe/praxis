---
name: refactor-layered-to-capability
description: Migrate a legacy codebase organized by technical layer (controllers/, services/, models/, utils/) into capability-driven vertical slices, incrementally and safely with no big-bang rewrite.
---

# Refactor Layered to Capability

## Use this when

- Working in a codebase with technical-layer folders (`controllers/`, `services/`, `models/`, `views/`).
- Working in a codebase with dumping grounds (`utils/`, `helpers/`, `common/`, `shared/`, `misc.*`).
- About to add a new feature and tempted to "just put it in `services/` for now."

## Do NOT use this when

- You're under time pressure to ship a critical fix. Land the fix in the legacy structure, file the migration as a follow-up, then run this skill.
- The codebase is already capability-driven and you mistook a folder name. Verify first.

## Principle

```
Migrate by capability, not by file. Land one full vertical slice at a time.
```

A big-bang refactor of a layered codebase fails. Incremental capability extraction succeeds because each step is a complete, testable, shippable unit.

## Steps

### Step 1 — Inventory the legacy structure

For the area you're about to touch, list:

| Layer folder                     | Files involved | Belongs to capability                        |
| -------------------------------- | -------------- | -------------------------------------------- |
| `controllers/orderController.*`  | …              | `order-processing`                           |
| `services/orderService.*`        | …              | `order-processing`                           |
| `services/taxService.*`          | …              | `order-processing`? `billing`?               |
| `models/Order.*`, `models/Tax.*` | …              | `order-processing`, `billing`                |
| `utils/dateFormatter.*`          | …              | should become `pkg/date-formatter/`          |
| `utils/utils.*`                  | mixed          | split each function into its true capability |

Mark anything ambiguous and ask the human before guessing.

### Step 2 — Pick one capability to extract first

Choose by these criteria, in priority order:

1. **The capability you're about to modify.** Do the migration as part of the change you were going to make anyway.
2. **The smallest cohesive capability.** Easier to land cleanly.
3. **The capability with the fewest cross-cutting dependencies.** Less drag.

Do not start with the largest or most coupled capability. That's where this kind of work dies.

### Step 3 — Design the target slice

Use `design-capability-layout`. Produce:

- The target folder layout for this single capability.
- The public surface (`mod.*`).
- The cross-capability dependency table — explicitly note legacy callers that will need updating.

### Step 4 — Migration plan (expand / contract for code)

Apply the same pattern as zero-downtime data migrations:

1. **Expand** — create the new capability folder. Copy (don't move) the relevant files into it. Restructure into `entity / repository / service / api`. Wire `mod.*` to the new locations.
2. **Dual-publish** — the old layered files re-export from the new capability. Existing callers continue to work.
3. **Switch callers** — update each caller, one at a time, to import from the new `mod.*`. Land each as a small PR.
4. **Verify no callers remain** — search for imports from the legacy paths. Zero results.
5. **Contract** — delete the legacy files. Delete the legacy folders if empty.

Each step is independently shippable. You can pause between any two.

### Step 5 — Handle dumping grounds (`utils/`, `helpers/`, `common/`, `shared/`)

For each function in a dumping ground, decide:

| Decision                                                                                | When                                                                            |
| --------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| **Move into the only consuming capability**                                             | Used by exactly one capability                                                  |
| **Extract into a narrow named module** (e.g., `pkg/encryption/`, `pkg/date-formatter/`) | Used by 2+ capabilities, genuinely generic, stable interface                    |
| **Duplicate**                                                                           | Used by 2+ capabilities but the lifecycles diverge or the function is ≤10 lines |
| **Delete**                                                                              | Unused (you'll be surprised how often)                                          |

Forbidden choice: leave it in `utils/` "for now."

### Step 6 — Update tests as you go

- Tests for the new capability live with it (`<capability>.test.*`).
- Tests for moved functions get moved with the function.
- Run the full test suite after each step. Green before proceeding.

### Step 7 — Update the ADR registry

Write an ADR per migrated capability:

- "ADR.<ID>: Extract `<capability>` from layered structure"
- Reference the source files, the new location, the migration steps taken, and the callers updated.

### Step 8 — Wire the anti-dumping linter

Once the first capability is extracted, add `scripts/check-anti-dumping.sh` to the project. Configure `.anti-dumping.json` with **exemptions** for the legacy folders that still exist:

```json
{
  "scanPaths": ["src/**"],
  "forbiddenNames": [
    "utils",
    "helpers",
    "common",
    "shared",
    "misc",
    "controllers",
    "services",
    "models",
    "views"
  ],
  "exemptions": [
    "src/legacy/services",
    "src/legacy/controllers"
  ]
}
```

As each legacy folder is emptied and deleted, remove its exemption. The exemption list shrinks to zero over the course of the migration. The linter ratchets you forward.

### Step 9 — Track progress

Maintain a `MIGRATION.md` at the repo root:

```markdown
# Layered → Capability Migration

## Capabilities extracted

- [x] order-processing — ADR.<ID>, completed YYYY-MM-DD
- [x] billing — ADR.<ID>, completed YYYY-MM-DD
- [ ] shipping — in progress
- [ ] notifications — not started

## Legacy folders remaining

- `src/legacy/services/` — 4 files left
- `src/legacy/controllers/` — 2 files left
- `src/legacy/utils/` — 12 functions left, plan: split into `pkg/encryption/`, `pkg/date-formatter/`, duplicate the rest

## Anti-dumping linter exemptions

- `src/legacy/services` — remove when shipping migration done
- `src/legacy/controllers` — remove when shipping + notifications done
```

When the file is empty, delete it.

## Anti-patterns

- **Big-bang rewrite.** Fails. Always.
- **Migrate by file.** Moves chaos around without making it shippable. Migrate by capability.
- **Skip the dual-publish step.** Forces a flag-day update of every caller. Breaks things.
- **Leave `utils/`.** "Just for now" becomes "forever." Set a removal deadline and enforce it.
- **Refactor without a test safety net.** Add the tests before moving the code, or you have no way to verify behavior preserved.
- **Migrate during a feature freeze.** Migrate as part of the work you were going to do anyway.

## Halt conditions

Stop and ask the human if:

- A function in `utils/` is called from 10+ places and the right home isn't obvious.
- Two capabilities both seem to legitimately own the same domain concept (boundary is wrong — re-examine).
- The "expand" step would require breaking an external public contract.
- A migration step would require a database schema change in the same PR — split them.
