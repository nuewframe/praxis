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
4. **Honor the guardrails.** Two always-on rule sets shape every decision (`instructions/*.instructions.md`).
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

These are loaded automatically by harnesses that support `applyTo`-scoped instructions (Claude Code, Copilot). For harnesses that do not, treat the rules below as in force whenever you touch the matching paths.

### Lean Delivery Guardrails — `instructions/lean-delivery-guardrails.instructions.md`

Applies to `docs/product/**`, `docs/waves/**`, `docs/sprints/**`, `docs/adr/**`.

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
| `create-adr` | A decision binds future work; ADR with alternatives table required |

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
| `scripts/check-config-externalized.sh` | Hardcoded remote URLs, endpoints, or secret literals (Configurable anchor); warn-first, reviewed per-line opt-out |
| `scripts/validate-plugin.sh` | Plugin self-test (run from this repo) |

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

If you can name the three personas, the two always-on guardrail sets, and at least four skills with their triggers, the bootstrap is loaded correctly.
