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

Ask exactly these five questions. Wait for answers before proceeding.

1. **Project name** (e.g., `acme-billing`)
2. **Primary language and runtime** (e.g., TypeScript + Deno, Python 3.13, Go 1.23, Rust)
3. **Primary framework** (e.g., Hono, FastAPI, gRPC server, CLI, none)
4. **Primary storage** (e.g., Postgres, SQLite, DynamoDB, none)
5. **Deployment target** (e.g., Cloud Run, Lambda, Kubernetes, single binary)

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
│   ├── architecture/                     # durable architecture tree (living = the truth)
│   │   ├── README.md                     # system overview: cross-capability topology + posture
│   │   ├── adr/
│   │   │   └── ADR.<ID>-technology-stack.md   # cross-capability decisions
│   │   └── <capability-1>/
│   │       ├── README.md                 # capability record (current-state truth)
│   │       └── adr/                       # capability-scoped decisions
│   ├── guides/                        # user-facing docs (TEACH) — rendered from capability records
│   │   ├── <capability-1>/                # capability guide: concepts + how-tos
│   │   └── tutorials/                     # cross-capability journey tutorials
│   ├── product/
│   │   ├── waves/
│   │   │   └── wave-000-bootstrap/
│   │   │       ├── README.md
│   │   │       ├── product-design.md
│   │   │       ├── product-architecture.md
│   │   │       └── qa.md
│   │   └── sprints/                       # flat, ephemeral — SPRINT.<ID>-<slug>.md, never nested in a wave
│   │       └── SPRINT.<ID>-placeholder.md
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
│   └── check-*.sh                        # every guardrail check, copied from this plugin (see Step 9)
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

````markdown
# <project-name> — Copilot Instructions

These instructions are mandatory. Repository rules override the `praxis` plugin defaults; plugin defaults apply where the repo is silent.

## Project identity

- **Name:** <project-name>
- **Stack:** <language and runtime> · <framework> · <storage> · <deployment target>
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

Two rules keep this pattern healthy as the codebase grows:

- **Splitting.** When a capability file no longer fits in one read (~400 lines is the smell threshold) or the capability serves more than one distinct sub-domain, split it into sub-capabilities (`billing/invoicing/`, `billing/settlement/`), each with its own `mod.<ext>`. Never relieve size pressure with a `utils` file — that recreates the dumping ground inside the sanctioned structure.
- **Graduation.** Code needed by two or more capabilities never gets a `shared/` folder. Either it earns a business-domain name and becomes its own capability (`money/`, `audit/`) with a `mod.<ext>` surface and an owner, or it stays duplicated until it does. Duplication is the visible, reversible cost; a `shared/` folder is the invisible, compounding one.

## Workflow

For non-trivial changes, run the principal-engineer phased skills in order:

0. `intake-code-contribution`
1. `discovery-and-ambiguity-log`
2. `design-system-architecture`
3. `design-capability-layout`
4. `create-adr`
5. `implement-with-defensive-patterns`
6. `verify-and-assemble-pr`

## Quality gates (mandatory before commit)

A single command runs the full chain:
```

bash scripts/verify.sh

```
The pipeline is:

- Format: `<format command>`
- Lint: `<lint command>`
- Type check: `<type-check command>`
- Praxis guardrails: every `scripts/check-*.sh` (copied from the plugin — `verify.sh` runs them all)
- Tests: `<test command>`

`scripts/verify.sh` is the single source of truth for the pipeline. Do not restate the check list elsewhere; link here or to the script.

## Non-negotiables

1. Capability-driven layout — no `controllers/`, `services/`, `models/` silos.
2. Anti-dumping — no `utils/`, `helpers/`, `common/`, `shared/`, `misc.*`.
3. Functional core, imperative shell — business logic does not depend on I/O.
4. ADR for every significant decision, homed in the durable architecture tree (`docs/architecture/<capability>/adr/`, or `docs/architecture/adr/` for cross-capability decisions).
5. Tests live with the capability, not in a separate tree.
6. Telemetry: structured logs, p95/p99 metrics, trace propagation. No `console.log` / `print`.
````

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

Durable architecture lives in [docs/architecture/](architecture/): the system overview ([README.md](architecture/README.md)), per-capability records (`<capability>/README.md`, the current-state truth), and ADRs (`architecture/adr/` for cross-capability, `<capability>/adr/` for capability-scoped). Start at [docs/architecture/adr/ADR.<ID>-technology-stack.md](architecture/adr/ADR.<ID>-technology-stack.md).

## Conventions

- Capability-driven layout. Anti-dumping enforced (`./scripts/check-anti-dumping.sh`).
- Functional core / imperative shell.
- TDD red-green-refactor.
- ADR for every significant decision.

## Quality gates

One command, mandatory before commit: `bash scripts/verify.sh`. The script is the single source of truth for the pipeline (format, lint, type check, praxis guardrail checks, tests) — see `.github/copilot-instructions.md` for the summary.
```

### Step 7 — Generate the durable architecture tree

Create `docs/architecture/README.md` (system overview stub: capability list + a placeholder cross-capability topology and product-wide posture) and the first ADR at `docs/architecture/adr/ADR.<ID>-technology-stack.md` (a cross-capability decision). The overview's first line states its authority: "This tree is current-state truth, promoted by `close-sprint`. Planning-stage intent lives in `docs/product/waves/`." The topology section is not optional decoration — it is the only home the cross-capability picture has; a stub is acceptable, an absent section is not. Use the ADR template from `create-adr` (which carries an as-of-decision diagram) and apply its ID convention. Fill in:

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
  "allowPatterns": [],
  "exemptions": []
}
```

(Adjust `scanPaths` to match the language convention — `services/`, `pkg/`, `apps/`, etc.)

**Per-ecosystem `allowPatterns` (fill in for your stack).** `allowPatterns` are regexes matched against a **basename**, so they exempt a framework-mandated *file* while still blocking the dumping-ground *directory* of the same name. Add the row(s) for your stack so the day-one build does not trip the linter:

| Stack | Add to `allowPatterns` | Exempts (idiomatic file) | Still blocked (silo dir) |
| ----- | ---------------------- | ------------------------ | ------------------------ |
| Rust (if `lib` is in `forbiddenNames`) | `"^lib\\.rs$"` | the mandatory crate root `lib.rs` | a `lib/` catch-all |
| Django / Python | `"^models\\.py$"`, `"^views\\.py$"` | per-app `models.py` / `views.py` | `models/` / `views/` |
| NestJS | (none) | `*.service.ts` already passes — its basename does not start with `service` | a bare `services/` |

For a framework whose idiom is a whole *directory* (Angular `services/`, ASP.NET MVC `Controllers/`/`Models/`/`Views/`), prefer the capability-driven layout. If you must keep the convention, add that specific path to `exemptions` — but that re-opens the dumping ground the linter exists to close, so scope it tightly and revisit it.

Leave `exemptions` empty for greenfield. The sanctioned escape hatch for cross-capability code is **graduation** into a named capability (see the Architecture rules in `.github/copilot-instructions.md`), not an exemption — an exemption re-opens the dumping ground the linter exists to close.

### Step 9 — Generate `scripts/`

Copy `scripts/verify.sh` from `<plugin-root>/skills/provision-project-overlay/templates/scripts/verify.sh.tmpl` and edit the `step_format`, `step_lint`, `step_typecheck`, and `step_tests` function bodies to call your project's actual commands.

Then copy or symlink **every** `check-*.sh` from `<plugin-root>/scripts/` into `scripts/`. Do not hand-pick a subset: `verify.sh` calls all of them, and a missing script breaks the quality gate on day one. (`validate-plugin.sh` is the one exception — it is a plugin self-test, copy it only if you want it.)

Before moving on, run the parity check — every check `verify.sh` calls must exist on disk:

```sh
for s in $(grep -o 'check-[a-z-]*\.sh' scripts/verify.sh | sort -u); do
  [ -f "scripts/$s" ] || echo "MISSING: scripts/$s"
done
```

No output is the pass condition. Make every script executable. Wire `bash scripts/verify.sh` into the project's task runner so it runs on every build and in CI.

### Step 9b — Scaffold the bootstrap wave

Create `docs/product/waves/wave-000-bootstrap/` with four placeholder docs:

Every wave document — stub or filled — opens with the planning-stage banner directly under its title, so the durable-vs-planning split is visible in the artifact itself, not just in the skill definitions:

```markdown
> **Planning-stage document — an educated theory, not yet the truth.** The best approach given what we know today; current-state architecture lives in [docs/architecture/](../../../architecture/), promoted there by `close-sprint`.
```

The durable tree carries no banner: it *is* the truth, and says so in its overview (Step 7).

Use the exact filenames the wave skills own — a stub under any other name is never picked up downstream:

- `README.md` — wave intent stub: name, purpose, primary user, success metric, thin-slices. The PM persona fills this in via `create-wave`.
- `product-design.md` — design spec stub. Filled in via `create-product-design-spec`.
- `product-architecture.md` — architecture spec stub. Filled in via `create-product-architecture-spec`.
- `qa.md` — quality spec stub. Filled in via `create-quality-spec`.

And one sprint placeholder:

- `docs/product/sprints/SPRINT.<ID>-placeholder.md` — a sprint **template** stub the team fills in for its first Standard/Major change or deletes. Trivial changes do not get a sprint (see `start-thin-slice` — Trivial routes straight to implementation with no bridge); this placeholder exists only to make the `create-sprint` mechanics visible from day one. Sprints live **flat** under `docs/product/sprints/`, never nested in a wave, and are deleted at close. Use the collision-safe `SPRINT.<ID>` id convention from `create-adr`.

These stubs make the workflow legible to a new contributor without forcing them to learn the persona-mode model first.

### Step 10 — Generate `BOOTSTRAP.md`

```markdown
# Bootstrap Notes

This project was scaffolded by the `praxis` plugin's `bootstrap-project` skill on <date>.

## What was generated

- `.github/copilot-instructions.md`
- `.claude/CLAUDE.md`
- `docs/project-context.md`
- `docs/architecture/README.md` and `docs/architecture/adr/ADR.<ID>-technology-stack.md`
- `docs/guides/` (user-facing docs home — TEACH; populated once capabilities ship)
- `src/<capability>/` skeletons for: <list>
- `scripts/verify.sh` plus every `scripts/check-*.sh` from the plugin
- `.anti-dumping.json`

## What to do next

1. Install dependencies for `<language and framework>`.
2. Wire the quality gates into your task runner (`<runner>`).
3. Run the first feature using the phased workflow: `intake-code-contribution` → `discovery-and-ambiguity-log` → `design-system-architecture` → `design-capability-layout` → `create-adr` → `implement-with-defensive-patterns` → `verify-and-assemble-pr`.
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
- Skipping the first ADR because "we'll write ADRs later." The first ADR captures the bootstrap decision itself and establishes the durable architecture tree (`docs/architecture/`).
- Generating sample code that violates the capability-driven structure as a "starter."
- Writing `.github/copilot-instructions.md` that re-states everything from the plugin instead of referencing it. Keep the project file lean.
