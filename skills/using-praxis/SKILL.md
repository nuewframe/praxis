---
name: using-praxis
description: Bootstrap entry point for the Praxis plugin. Loaded at session start by every harness (Claude Code, Codex, Cursor, Gemini, OpenCode, Copilot). Teaches the agent which personas exist, which guardrails are always-on, and which skill to invoke for each product or engineering moment.
---

# Praxis — Skill Index and Bootstrap

You are operating with the **Praxis** plugin loaded. Praxis is an opinionated plugin that fuses **lean wave-based product delivery** with **Principal Engineer discipline** into a single composable method. It is universal — language-, framework-, and runtime-agnostic.

The host repository's own `.github/`, `.claude/`, or workspace instructions **always override** anything in this plugin. When in doubt, the repo wins.

This file is your map. Read it once at session start. Load specific skills on demand.

---

## How to use this plugin

1. **Identify the moment.** Is the human asking you to plan, design, implement, review, refactor, or ship?
2. **Pick the persona.** Three roles, each with a single responsibility (`agents/*.agent.md`).
3. **Pick the skill.** Each skill is a focused, repeatable workflow (`skills/<name>/SKILL.md`).
4. **Honor the guardrails.** Three always-on rule sets shape every decision (`instructions/*.instructions.md`).
5. **When the host repo is silent, follow the plugin defaults. When it speaks, the host repo wins.**

---

## Personas

Load the persona's full file before acting in that role.

| Persona | When to be this persona | File |
|---|---|---|
| **Product Manager** | Wave planning, sprint creation as immutable bridges, sprint closing with bidirectional learning capture, dashboard honesty | `agents/product-manager.agent.md` |
| **Product Designer** | User value, thin-slice acceptance criteria, authoring `product-design.md` and `qa.md` | `agents/product-designer.agent.md` |
| **Principal Engineer** | Capability-driven architecture, refactoring, cross-cutting decisions; operates in three modes — architect, implementer, reviewer — never two at once | `agents/principal-engineer.agent.md` |

The same engineer cannot self-approve. If you are the implementer, you cannot also be the reviewer in the same session — switch personas explicitly or hand off.

---

## Always-on guardrails

These guardrails ship as `applyTo`-scoped `.instructions.md` files. **Copilot** auto-applies each one whenever you edit a file matching its `applyTo` glob. **Claude Code and other harnesses have no `applyTo` mechanism** and do not auto-load the plugin's `instructions/` directory — for those, the compressed summary below is the always-on surface (this file is loaded at session start), and you must treat each rule set as in force whenever you touch the matching paths. In a provisioned repo, `provision-project-overlay` copies these files into `.github/instructions/` so Copilot picks them up natively.

### Lean Delivery Guardrails — `instructions/lean-delivery-guardrails.instructions.md`

Applies to `docs/product/**`, `docs/architecture/**`, `docs/guides/**`, `docs/waves/**`, `docs/sprints/**`.

- Waves are intent, not bigger sprints. Each wave has four documents: README, product-design, product-architecture, qa.
- Thin-slices are atomic user outcomes with stable IDs (`TS-NNN`). Corrections keep the same ID.
- Sprint is an immutable bridge — scope is locked once it starts. To change scope, close it and create a new one.
- Sprint close is bidirectional: learnings flow into product artifacts AND engineering artifacts; sprint files are then deleted.
- Quality is specified in `qa.md` before it is tested. No code in `qa.md`.
- Code contribution intake (wave, slice, specs, sprint, code state, test posture) comes before any implementation.
- ADRs capture durable decisions with mandatory alternatives table.

### Capability-Driven Engineering Guardrails — `instructions/capability-driven-guardrails.instructions.md`

Applies to `src/**`, `packages/**`, `services/**`, `apps/**`, `libs/**`, `modules/**`.

- Organize by **business capability**, not technical layer. No `controllers/`, `models/`, `services/`, `views/`, `handlers/` silos.
- **Anti-dumping policy:** `utils/`, `helpers/`, `common/`, `shared/`, `misc.*`, `lib.*` are forbidden. Name the actual capability or duplicate.
- Functional core, imperative shell — pure logic in one file, I/O wrapper in another.
- Cross-capability calls go through an explicit public surface. No deep imports across capabilities.
- Every external call declares timeout, retry policy, fallback, and (for hot paths) circuit breaker.
- Every cross-process path produces structured logs with correlation ID, plus latency / throughput / error metrics.
- Two units may be built concurrently only if disjoint across capability/files, persistent resources, and config keys, and each depends only on a frozen `<name>@vN` contract.

### Code Contribution Intake — `instructions/code-contribution-intake.instructions.md`

Applies before any user-story, feature, thin-slice, or behavior-changing contribution. Run the intake skill first.

---

## Skill index

Skills are grouped by phase. Load the SKILL.md file of any skill you intend to follow.

### Lean delivery — planning artifacts

| Skill | Use when |
|---|---|
| `bootstrap-project` | A greenfield repo needs `.github/` + `.claude/` + capability-driven `src/` skeleton |
| `provision-project-overlay` | An existing repo has just installed Praxis and needs a project-specific overlay (interview-driven, idempotent) |
| `create-wave` | Starting a new wave; scaffolds the four-document pattern |
| `create-product-design-spec` | Authoring `product-design.md` — user journeys, ambiguity handling, recovery paths |
| `create-product-architecture-spec` | Authoring wave-scoped `product-architecture.md` — domain ownership, contracts, integrations |
| `create-quality-spec` | Authoring `qa.md` — risk tiers, test layer mapping, coverage matrix, observable DoD |
| `start-thin-slice` | Front door for slice work ("Work on TS-NNN"); checks dependency/status preconditions, provisional tier, then routes to create-sprint vs. the architect path |
| `create-sprint` | Locking the immutable bridge: thin-slice intent + engineering current-state snapshot + hypothesis card + test plan |
| `intake-code-contribution` | Pre-implementation gate; mandatory before any code change |
| `close-sprint` | Distilling learnings bidirectionally into product AND engineering artifacts; deletes the sprint file |
| `author-user-docs` | TEACH phase — rendering a validated capability record into Diátaxis user guides in `docs/guides/` (product-designer-owned) |
| `create-adr` | A decision binds future work; ADR with alternatives table required |
| `define-seam-contract` | A wave/slice crosses a boundary that must be honored executably; produces Shape + Behavior suite + frozen `<name>@vN` id |

### Engineering discipline — phased delivery

| Skill | Phase | Use when |
|---|---|---|
| `discovery-and-ambiguity-log` | 1 | Surfacing assumptions, defining SLO/SLA, producing an Ambiguity Log |
| `design-system-architecture` | 2–3 | Topology, resilience patterns, contract-first APIs, polyglot persistence, expand/contract migrations |
| `design-capability-layout` | 4 | Mapping the capability into a vertical-slice folder layout with functional core / imperative shell |
| `implement-with-defensive-patterns` | 5 | Writing the implementation; composition over inheritance, shift-left security, structured telemetry |
| `verify-and-assemble-pr` | 6 | TDD verification, integration boundary tests, PR narrative |
| `refactor-layered-to-capability` | — | Migrating a legacy `controllers/` + `services/` codebase into vertical slices |
| `test-by-ownership` | — | Picking the right test layer (Logic → Component → Integration → Journey) for each behavior |

---

## Tooling

Praxis ships generic, configurable enforcement scripts. Wire them into the project task runner and CI:

| Script | Fails on |
|---|---|
| `scripts/check-anti-dumping.sh` | `utils/`, `helpers/`, `common/`, `shared/`, `misc.*` inside capability roots |
| `scripts/check-no-skipped-tests.sh` | Committed `.skip(`, `xit(`, `@Disabled`, `@pytest.mark.skip` markers |
| `scripts/check-no-sleep-waits.sh` | `Thread.sleep`, `time.sleep`, `waitForTimeout` |
| `scripts/check-port-adapter-parity.sh` | `*.ports.*` with no adapter; warns if no in-memory test double |
| `scripts/check-seam-contract-parity.sh` | A seam in `.seam-contracts.json` missing its Shape or Behavior suite; warn-first, mode-promoted |
| `scripts/check-config-externalized.sh` | Hardcoded remote URLs, endpoints, or secret literals (Configurable anchor); warn-first, reviewed per-line opt-out |
| `scripts/check-observability-at-seams.sh` | A boundary call with no log/metric/trace/correlation-id (Observable anchor); warn-first, reviewed per-file opt-out |
| `scripts/check-stateless-request-path.sh` | Node-local mutable state on the request path (Horizontally-scalable anchor); warn-first, reviewed per-line opt-out |
| `scripts/check-resilient-boundary.sh` | A boundary call with no timeout/retry/circuit-breaker/fallback (Resilient anchor); warn-first, reviewed per-file opt-out |
| `scripts/check-sprint-id-collision.sh` | Two active sprint files sharing an id token (parallel-creation collision); exact, warn-first via `.sprint-coordination.json` |
| `scripts/validate-plugin.sh` | Plugin self-test (run from this repo) |

---

## Emergent parallelism — the three-axis disjointness rule

Praxis never schedules parallel work. Parallelism is an **emergent permission**, exercised by the human or an orchestration runtime — never forced by the method, never an artifact Praxis produces. A unit of work (a slice/sprint) may run concurrently with another **only if all four conditions hold**:

1. **Capability/file disjoint** — they touch no source file or capability in common.
2. **Persistent-resource disjoint** — they write no shared table, topic, queue, cache, or migration in common.
3. **Config-key disjoint** — they mutate no shared configuration key in common.
4. **Frozen-contract dependent** — each depends only on a frozen `<name>@vN` seam contract (`define-seam-contract`), never on the other's in-flight internals.

Capability-disjointness **alone is not sufficient**: two slices in different capabilities still collide if they share a table, a config key, or one consumes the other's unfrozen surface. All three axes *plus* the frozen-contract rule must hold. If any axis overlaps, the units are sequential, not parallel. The collision-safe coordination artifacts (`create-sprint`) and the staleness re-anchor at intake (`intake-code-contribution`) are what make a permitted parallel run *safe*.

---

## Composition with orchestration runtimes (MPM and others)

Praxis owns **artifact discipline**. It does not implement runtime mechanics — delegation, verification gates, ticketing, branch protection, circuit breakers as runtime checks. Those belong in an orchestration runtime such as [Claude MPM](https://github.com/anthropics/claude-mpm).

When MPM is installed:

- Praxis produces the artifacts (`product-design.md`, `product-architecture.md`, `qa.md`, sprint files, ADRs, wave READMEs, handbook updates).
- MPM's PM agent uses those artifacts as the source of truth for delegation; specialists align to the personas defined here (`principal-engineer`, `product-designer`, `product-manager`).

The two are orthogonal. Either can be used without the other.

---

## Precedence

```
repo .github/copilot-instructions.md, .claude/CLAUDE.md   (highest — project owns final word)
repo .github/instructions/*.instructions.md (scoped)
repo .github/agents/, .github/skills/
─────────────────────────────────────────────────────────
plugin instructions, agents, skills                       (defaults — Praxis)
─────────────────────────────────────────────────────────
user ~/.claude/CLAUDE.md, VS Code user prompts            (personal preferences only)
```

If a host repo's instruction conflicts with this plugin, follow the host. Praxis sets defaults; it never claims the final word.

---

## Quick verification

Ask the human partner:

> "Tell me about your praxis."

If you can name the three personas, the three always-on guardrail sets, and at least four skills with their triggers, the bootstrap is loaded correctly.
