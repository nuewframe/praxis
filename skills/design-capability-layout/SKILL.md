---
name: design-capability-layout
mode: architect
tools: [
  read_file,
  file_search,
  grep_search,
  semantic_search,
  create_file,
  replace_string_in_file,
]
description: Phase 4 of the principal-engineer workflow (architect mode). Use after system architecture is approved to design the per-capability folder structure, map functional core vs. imperative shell, declare Ports, and produce the ADR. Stops for human review before implementation. Architect mode forbids edits to source code.
---

# Design Capability Layout

## Use this when

- Phase 2/3 (`design-system-architecture`) is complete and approved.
- About to introduce or significantly modify a capability's internal structure.

## Do NOT use this when

- The change is a single function inside an existing well-structured capability — go straight to implementation.
- No system-level design exists yet. Return to Phase 2/3.

## Worked example

See [EXAMPLE.md](./EXAMPLE.md) for a full end-to-end walkthrough of all six phases applied to an `order-processing` capability, including the Ambiguity Log, topology, resilience table, OpenAPI contract, event schema, folder layout, ADR, code, tests, and PR narrative. Read it once before applying the steps below to your own work.

## Steps

### Step 1 — Print the target folder layout

Use the capability-driven structure. **No `utils/`, `helpers/`, `common/`, `shared/`, `misc.*`.**

```
<capability-root>/
└── <capability-name>/
    ├── <capability>.entity.<ext>          # types and schemas — pure (core)
    ├── <capability>.ports.<ext>           # Port interfaces declared by the capability (core)
    ├── <capability>.service.<ext>         # business logic — pure where possible (core)
    ├── <capability>.repository.<ext>      # real-backend adapter for the Repository Port (shell)
    ├── <capability>.adapter-memory.<ext>  # in-memory adapter(s) for tests only (shell, test-only)
    ├── <capability>.api.<ext>             # transport adapter (HTTP / CLI / queue) (shell)
    ├── <capability>.<port>.contract.test.<ext>  # shared contract suite — run against both adapters
    ├── <capability>.test.<ext>            # logic + composition tests live with the capability
    └── mod.<ext>                          # public surface — only this is imported by other capabilities
```

The `<capability>.adapter-memory.<ext>` file is real code, not stubs returning canned values. It implements the Port interface and is exercised by the Composition tests for fast TDD feedback. Production code must never import it — only the test files do.

If the capability is large enough to warrant sub-files (e.g., multiple entities), still keep the structure flat within the capability folder. Do not create a sub-`services/` folder inside a capability — that reintroduces technical-layer silos.

### Step 2 — Map functional core vs. imperative shell

For each file in the layout, label it:

| File                 | Layer             | Why                                                                                               |
| -------------------- | ----------------- | ------------------------------------------------------------------------------------------------- |
| `*.entity.*`         | Core              | Pure types and validation schemas.                                                                |
| `*.ports.*`          | Core              | Port interfaces declared by the capability. The core owns the contracts; adapters implement them. |
| `*.service.*`        | Core (mostly)     | Business rules. Depends on Port types only, never on concrete adapter classes.                    |
| `*.repository.*`     | Shell             | Real-backend adapter implementing the Repository Port. No business decisions.                     |
| `*.adapter-memory.*` | Shell (test-only) | In-memory adapter implementing the same Port. Real code, not stubs. Never imported by production. |
| `*.api.*`            | Shell             | Parses transport, validates input, calls service, formats response. No business decisions.        |

The rule: **the core never imports the shell.** The shell depends on the core (specifically on `*.ports.*`). If you find a business rule inside `*.api.*` or `*.repository.*`, move it.

**Port/Adapter parity:** every Port declared in `*.ports.*` must have ≥1 real-backend adapter and ≥1 in-memory adapter. Both pass the same contract suite. See the `test-by-ownership` skill's Pyramid Test Strategy.

### Step 3 — Define the public surface

Every capability exposes a single `mod.*` (or `index.*`) file. Other capabilities import **only** from this file. Direct deep imports across capabilities are forbidden.

What to export:

- The capability's primary types.
- The service functions that other capabilities are explicitly allowed to call.
- Nothing else. Repository, internal helpers, and transport adapters stay private.

### Step 4 — Cross-capability dependencies

For each cross-capability call, declare:

| Caller             | Callee      | Method                    | Rationale                                                   |
| ------------------ | ----------- | ------------------------- | ----------------------------------------------------------- |
| `order-processing` | `billing`   | async event `OrderPlaced` | Loose coupling; billing can be down without blocking orders |
| `order-processing` | `inventory` | sync REST `reserve()`     | Strong consistency required at order placement              |

If two capabilities are calling each other in both directions, that is a smell. Re-examine the boundary.

### Step 5 — Produce the ADR

Write an Architecture Decision Record using this template. Use the ADR ID convention from `create-adr` in the project's ADR directory.

```markdown
# ADR.<ID>: <decision title>

## Status

Proposed | Accepted | Superseded by ADR.<ID>

## Context

What forces are in play? What problem are we solving? Reference Phase 1 Ambiguity Log resolutions and Phase 2/3 design decisions.

## Decision

What we are doing. State it as an imperative.

## Alternatives considered

| Option     | Pros | Cons | Verdict            |
| ---------- | ---- | ---- | ------------------ |
| A — chosen | …    | …    | Selected           |
| B          | …    | …    | Rejected because … |
| C          | …    | …    | Rejected because … |

## Consequences

- Positive: …
- Negative: …
- Risks: …
- What this commits us to long-term: …

## Compliance

- Capability-driven: yes / no (justify)
- Anti-dumping: no forbidden folders introduced
- Functional core / shell: mapping documented
- ADR review: who reviewed, when
```

### Step 6 — Stop

Output:

1. Folder layout (ASCII tree).
2. Core/shell mapping table.
3. Public surface definition.
4. Cross-capability dependency table.
5. Complete ADR.

Then say: **"Please review the layout and ADR. Approve before I proceed to Phase 5 (implementation)."**

Do not write feature logic yet.

## Anti-patterns

- A `helpers.*` file appears anywhere in the capability. Reject and rename to a specific responsibility.
- A `services/` sub-folder inside a capability — that's a technical-layer silo at smaller scale.
- Public surface that re-exports everything. Be intentional about what's part of the contract.
- Business logic in the API or repository file. Move it to the service.
- ADR with no "Alternatives considered" table. Without alternatives, it's not a decision — it's a default.
- Cross-capability deep imports (`import { foo } from '../billing/internal/...`). Force everything through `mod.*`.

## Migrating into this layout

If you are working in a legacy codebase with `controllers/`, `services/`, `models/`, etc., do **not** add to those folders. Instead:

1. Run the `refactor-layered-to-capability` skill to plan the extraction.
2. Land the new capability in its target shape.
3. Migrate existing functionality incrementally and delete the legacy folder when empty.
