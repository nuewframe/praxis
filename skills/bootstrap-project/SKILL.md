---
name: bootstrap-project
description: Scaffold a new project from empty (greenfield) with capability-driven architecture, anti-dumping enforcement, ADR discipline, and the principal-engineer phased workflow wired in. Generates .github/, .claude/, docs/, and a capability-driven src/ skeleton.
---

# Bootstrap Project

## Use this when

- Starting a brand-new repository.
- Adopting the `praxis` plugin in an existing repository that has no `.github/copilot-instructions.md` or `.claude/CLAUDE.md`.

## Do NOT use this when

- The repository already has Copilot or Claude configuration. Use `refactor-layered-to-capability` if structural migration is needed; otherwise extend existing files manually.

## Steps

### Step 1 — Interview the human

Ask exactly these four questions. Wait for answers before proceeding.

1. **Project name** (e.g., `acme-billing`)
2. **Primary language and runtime** (e.g., TypeScript + Deno, Python 3.13, Go 1.23, Rust)
3. **Primary framework** (e.g., Hono, FastAPI, gRPC server, CLI, none)
4. **Deployment target** (e.g., Cloud Run, Lambda, Kubernetes, single binary)

If any answer is "I don't know yet," push back: those decisions shape the structure. Don't proceed with placeholders.

### Step 2 — Confirm the capability boundaries

Ask: **"What are the 2–4 initial capabilities this project will own?"** Capability names are business-domain nouns (`billing`, `identity`, `shipping`), never technical layers (`api`, `controllers`, `models`).

If the human can't name them yet, run `discovery-and-ambiguity-log` first.

### Step 3 — Generate the structure

Create this layout (paths adapted to the language):

```
<repo-root>/
├── .github/
│   ├── copilot-instructions.md          # references plugin guardrails + project-specific rules
│   ├── instructions/                    # project-specific scoped instructions go here
│   ├── agents/                          # project-specific personas go here
│   └── skills/                          # project-specific workflows go here
├── .claude/
│   └── CLAUDE.md                        # mirrors copilot-instructions; entry point for Claude Code
├── docs/
│   ├── project-context.md               # single entry point — how to navigate this repo
│   ├── adr/
│   │   └── ADR-001-initial-architecture.md
│   ├── product/
│   │   └── waves/
│   │       └── wave-000-bootstrap/
│   │           ├── brief.md
│   │           ├── design.md
│   │           ├── architecture.md
│   │           ├── quality.md
│   │           └── sprints/
│   │               └── TS-001-placeholder.md
│   └── README.md
├── src/                                 # or services/ or pkg/ depending on language convention
│   ├── <capability-1>/
│   │   ├── <capability-1>.entity.<ext>
│   │   ├── <capability-1>.repository.<ext>
│   │   ├── <capability-1>.service.<ext>
│   │   ├── <capability-1>.api.<ext>
│   │   ├── <capability-1>.test.<ext>
│   │   └── mod.<ext>
│   └── <capability-2>/
│       └── …
├── scripts/
│   ├── verify.sh                         # universal verification entry point
│   ├── check-anti-dumping.sh             # symlink or copy from this plugin
│   ├── check-no-skipped-tests.sh         # symlink or copy from this plugin
│   ├── check-no-sleep-waits.sh           # symlink or copy from this plugin
│   └── check-port-adapter-parity.sh      # symlink or copy from this plugin
├── .anti-dumping.json                   # config for the linter (paths to scan)
├── README.md
├── CONTRIBUTING.md
└── BOOTSTRAP.md                         # what was generated and what to do next
```

**Forbidden** — do not create:

- `src/utils/`, `src/helpers/`, `src/common/`, `src/shared/`, `src/lib/` (as a dumping ground), `src/misc/`
- `src/controllers/`, `src/services/`, `src/models/`, `src/views/` at the top level

### Step 4 — Generate `.github/copilot-instructions.md`

Use this template, filling in `<placeholders>`:

```markdown
# <project-name> — Copilot Instructions

These instructions are mandatory. Repository rules override the `praxis` plugin defaults; plugin defaults apply where the repo is silent.

## Project identity

- **Name:** <project-name>
- **Stack:** <language and runtime> · <framework> · <deployment target>
- **Entry point:** [docs/project-context.md](../docs/project-context.md) — read first.

## Architecture

Capability-driven. See [docs/project-context.md](../docs/project-context.md) for the capability list and boundaries.

Per-capability file pattern:

| File                            | Responsibility                                               |
| ------------------------------- | ------------------------------------------------------------ |
| `<capability>.entity.<ext>`     | Types and schemas. Pure.                                     |
| `<capability>.repository.<ext>` | Data access only. No business logic.                         |
| `<capability>.service.<ext>`    | Business logic. Pure where possible.                         |
| `<capability>.api.<ext>`        | Transport adapter. Parse → validate → call service → format. |
| `mod.<ext>`                     | Public surface. Other capabilities import only from here.    |

## Workflow

For non-trivial changes, run the principal-engineer phased skills in order:

0. `intake-code-contribution`
1. `discovery-and-ambiguity-log`
2. `design-system-architecture`
3. `design-capability-layout`
4. `implement-with-defensive-patterns`
5. `verify-and-assemble-pr`

## Quality gates (mandatory before commit)

A single command runs the full chain:
```

bash scripts/verify.sh

```
The pipeline is:

- Format: `<format command>`
- Lint: `<lint command>`
- Type check: `<type-check command>`
- Anti-dumping: `bash scripts/check-anti-dumping.sh`
- No skipped tests: `bash scripts/check-no-skipped-tests.sh`
- No sleep waits: `bash scripts/check-no-sleep-waits.sh`
- Port/adapter parity: `bash scripts/check-port-adapter-parity.sh`
- Tests: `<test command>`

## Non-negotiables

1. Capability-driven layout — no `controllers/`, `services/`, `models/` silos.
2. Anti-dumping — no `utils/`, `helpers/`, `common/`, `shared/`, `misc.*`.
3. Functional core, imperative shell — business logic does not depend on I/O.
4. ADR for every significant decision in `docs/adr/`.
5. Tests live with the capability, not in a separate tree.
6. Telemetry: structured logs, p95/p99 metrics, trace propagation. No `console.log` / `print`.
```

### Step 5 — Generate `.claude/CLAUDE.md`

```markdown
# <project-name> — Claude Code Guide

This file mirrors `.github/copilot-instructions.md` for Claude Code. Both apply.

See [docs/project-context.md](../docs/project-context.md) before starting any work.

The `praxis` plugin provides the universal engineering discipline. This project's specific rules (capability list, stack-specific patterns) live in `.github/copilot-instructions.md` and `docs/project-context.md`.
```

### Step 6 — Generate `docs/project-context.md`

```markdown
# <project-name> — Project Context

Single entry point. Read this first.

## Identity

- **Purpose:** <one sentence>
- **Stack:** <language> · <framework> · <storage> · <deployment target>
- **Status:** bootstrap

## Capabilities

| Capability       | Responsibility | Owner |
| ---------------- | -------------- | ----- |
| `<capability-1>` | …              | TBD   |
| `<capability-2>` | …              | TBD   |

## Architecture

See [docs/adr/ADR-001-initial-architecture.md](adr/ADR-001-initial-architecture.md).

## Conventions

- Capability-driven layout. Anti-dumping enforced (`./scripts/check-anti-dumping.sh`).
- Functional core / imperative shell.
- TDD red-green-refactor.
- ADR for every significant decision.

## Quality gates

- `<format command>`
- `<lint command>`
- `<type-check command>`
- `./scripts/check-anti-dumping.sh`
- `<test command>`
```

### Step 7 — Generate `docs/adr/ADR-001-initial-architecture.md`

Use the ADR template from `design-capability-layout`. Fill in:

- The capability list from Step 2.
- The stack choices from Step 1.
- "Capability-driven layout, anti-dumping enforced, praxis plugin adopted" as the foundational decisions.
- Alternatives considered (technical-layer organization, single-flat-package) with rejection reasons.

### Step 8 — Generate `.anti-dumping.json`

```json
{
  "scanPaths": ["src/**"],
  "forbiddenNames": [
    "utils",
    "util",
    "helpers",
    "helper",
    "common",
    "shared",
    "misc",
    "general",
    "controllers",
    "services",
    "models",
    "views"
  ],
  "exemptions": []
}
```

(Adjust `scanPaths` to match the language convention — `services/`, `pkg/`, `apps/`, etc.)

### Step 9 — Generate `scripts/`

Copy or symlink from `<plugin-root>/scripts/`:

- `check-anti-dumping.sh`
- `check-no-skipped-tests.sh`
- `check-no-sleep-waits.sh`
- `check-port-adapter-parity.sh`
- `validate-plugin.sh` (optional — for plugin self-test)

Also copy `scripts/verify.sh` from `<plugin-root>/skills/provision-project-overlay/templates/scripts/verify.sh.tmpl` and edit the `step_format`, `step_lint`, `step_typecheck`, and `step_tests` function bodies to call your project's actual commands. Make every script executable. Wire `bash scripts/verify.sh` into the project's task runner so it runs on every build and in CI.

### Step 9b — Scaffold the bootstrap wave

Create `docs/product/waves/wave-000-bootstrap/` with four placeholder docs:

- `brief.md` — product brief stub: name, purpose, primary user, success metric. The PM persona fills this in via `create-wave`.
- `design.md` — design spec stub. Filled in via `create-product-design-spec`.
- `architecture.md` — architecture spec stub. Filled in via `create-product-architecture-spec`.
- `quality.md` — quality spec stub. Filled in via `create-quality-spec`.

And one sprint placeholder:

- `sprints/TS-001-placeholder.md` — a Trivial-tier sprint stub the team can either fill in for the first real change or delete. Includes the Design Approval section (n/a since Trivial) so the create-sprint mechanics are visible from day one.

These stubs make the workflow legible to a new contributor without forcing them to learn the persona-mode model first.

### Step 10 — Generate `BOOTSTRAP.md`

```markdown
# Bootstrap Notes

This project was scaffolded by the `praxis` plugin's `bootstrap-project` skill on <date>.

## What was generated

- `.github/copilot-instructions.md`
- `.claude/CLAUDE.md`
- `docs/project-context.md`
- `docs/adr/ADR-001-initial-architecture.md`
- `src/<capability>/` skeletons for: <list>
- `scripts/check-anti-dumping.sh`
- `.anti-dumping.json`

## What to do next

1. Install dependencies for `<language and framework>`.
2. Wire the quality gates into your task runner (`<runner>`).
3. Run the first feature using the phased workflow: `intake-code-contribution` → `discovery-and-ambiguity-log` → `design-system-architecture` → `design-capability-layout` → `implement-with-defensive-patterns` → `verify-and-assemble-pr`.
4. Once the first feature ships, decide whether to add project-specific personas (in `.github/agents/`) or skills (in `.github/skills/`).

## Plugin precedence

The `praxis` plugin provides defaults. This repository's `.github/` overrides the plugin where they differ. Personal preferences live in user-level Claude / VS Code config, not here.

Delete this file once you've internalized it.
```

### Step 11 — Print a summary and stop

Output:

1. The generated tree.
2. The list of files created.
3. The list of next-step commands the human should run (`<install>`, `<format>`, `<lint>`, `<test>`).
4. A reminder: **"The first feature should follow the phased workflow. Start with `intake-code-contribution`."**

Do not write feature code as part of bootstrap.

## Anti-patterns

- Generating `src/utils/` or `src/controllers/` because the language convention "expects it." It doesn't.
- Skipping ADR-001 because "we'll write ADRs later." ADR-001 captures the bootstrap decision itself.
- Generating sample code that violates the capability-driven structure as a "starter."
- Writing `.github/copilot-instructions.md` that re-states everything from the plugin instead of referencing it. Keep the project file lean.
