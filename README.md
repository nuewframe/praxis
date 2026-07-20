# Praxis

A portable agent plugin that fuses **lean wave-based product delivery** with **Principal Engineer discipline** into a single composable method. Language-, framework-, and runtime-agnostic in doctrine (static enforcement is best-effort per language — see [docs/coverage-matrix.md](docs/coverage-matrix.md)); installable into Claude Code, Codex (CLI and App), Cursor, Gemini CLI, OpenCode, and GitHub Copilot (CLI and VS Code).

Praxis is universal: it does not assume any stack. Project-specific rules belong in the project's own `.github/` and `.claude/` files and override anything here.

## Quickstart

Give your agent Praxis: [Claude Code](#claude-code) · [Codex CLI](#codex-cli) · [Codex App](#codex-app) · [Cursor](#cursor) · [Gemini CLI](#gemini-cli) · [OpenCode](#opencode) · [GitHub Copilot CLI](#github-copilot-cli) · [GitHub Copilot in VS Code](#github-copilot-in-vs-code).

## What this plugin gives you

### Personas

| Agent                                | Role                                                                                                                                                       |
| ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agents/product-manager.agent.md`    | Distinguished Product Manager — wave planning, sprint creation as immutable bridges, sprint closing with bidirectional learning capture, honest dashboard. |
| `agents/product-designer.agent.md`   | Distinguished Product Designer — user value, thin-slice acceptance criteria, `product-design.md` and `qa.md` authorship.                                   |
| `agents/principal-engineer.agent.md` | Distinguished Engineer — capability-driven architecture, refactoring, cross-cutting decisions.                                                             |

### Always-on guardrails

| Instruction                                                 | Scope                                                                                                                     |
| ----------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `instructions/lean-delivery-guardrails.instructions.md`     | Wave methodology, sprint-as-immutable-bridge, hypothesis cards, intent-not-history doc style, bidirectional sprint close. |
| `instructions/capability-driven-guardrails.instructions.md` | Capability-driven layout, anti-dumping policy, functional core / imperative shell, ADR discipline, telemetry baseline.    |
| `instructions/code-contribution-intake.instructions.md`     | Pre-implementation intake gate: anchor every code change to wave, thin-slice, specs, sprint, current code state, and red/green test posture before implementing. |

### Skills — Lean delivery (planning artifacts)

| Skill                                      | Purpose                                                                                                                      |
| ------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| `skills/create-wave/`                      | Scaffold a new wave with the four-document pattern (README, design, architecture, qa).                                       |
| `skills/create-product-design-spec/`       | Author `product-design.md` — user journeys, ambiguity handling, recovery paths.                                              |
| `skills/create-product-architecture-spec/` | Author wave-scoped `product-architecture.md` — the wave's **educated theory**: domain ownership, contracts, seams, integrations, pointing into the durable capability records.         |
| `skills/create-quality-spec/`              | Author `qa.md` — risk tiers, test layer mapping, security coverage matrix, observable definition-of-done.                    |
| `skills/test-by-ownership/`                | Universal Pyramid Test Strategy: Logic base through Journey tip, with "one property of a behavior, one layer" rule.          |
| `skills/intake-code-contribution/`         | Pre-implementation GenAI contribution gate: wave, thin-slice, specs, sprint, current code, and red/green test posture.       |
| `skills/start-thin-slice/`                 | Front door for slice work ("Work on TS-NNN"): checks dependency/status preconditions, runs a provisional tier + lightweight ambiguity/pre-mortem, then routes to `create-sprint` or the architect path. |
| `skills/create-sprint/`                    | Lock the immutable bridge: thin-slice intent + engineering current-state snapshot + hypothesis card + test plan.             |
| `skills/close-sprint/`                     | Bidirectional outflow: distill learnings into both product artifacts AND engineering artifacts, then delete the sprint file. |
| `skills/author-user-docs/`                 | TEACH phase — render a validated capability record into Diátaxis user guides (`docs/guides/`); product-designer-owned.       |
| `skills/create-adr/`                       | Immutable Architecture Decision Records (with an as-of-decision diagram + mandatory alternatives table), homed in the durable architecture tree `docs/architecture/<capability>/adr/`. |
| `skills/define-seam-contract/`             | Define a Seam Contract for a boundary: machine-readable Shape + shared Behavior suite + frozen `<name>@vN` id in `.seam-contracts.json`. |

### Skills — Principal Engineer discipline (phased delivery + bootstrap)

> The pre-implementation intake gate (`skills/intake-code-contribution/`) is listed in the lean-delivery half above. The engineering workflow consumes it as Phase 0; it isn't repeated here.

| Skill                                       | Purpose                                                                                                              |
| ------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `skills/discovery-and-ambiguity-log/`       | Phase 1 — surface assumptions, define SLO/SLA, produce an Ambiguity Log.                                             |
| `skills/design-system-architecture/`        | Phases 2 + 3 — topology, resilience patterns, contract-first APIs, polyglot persistence, expand/contract migrations. |
| `skills/design-capability-layout/`          | Phase 4 — vertical-slice folder layout, functional core / imperative shell mapping.                                  |
| `skills/implement-with-defensive-patterns/` | Phase 5 — composition over inheritance, shift-left security, structured telemetry.                                   |
| `skills/verify-and-assemble-pr/`            | Phase 6 — TDD verification, integration boundary tests, PR narrative.                                                |
| `skills/bootstrap-project/`                 | Greenfield scaffolder — generates `.github/` + `.claude/` + capability-driven `src/` skeleton.                       |
| `skills/provision-project-overlay/`         | Generate a project-specific `.github/` overlay (skills, agents, prompts, persona instructions) on an existing repo that just installed Praxis; interview-driven, idempotent. |
| `skills/refactor-layered-to-capability/`    | Migrate a legacy `controllers/` + `services/` + `utils/` codebase into vertical slices.                              |

### Tooling

| Script                                 | Purpose                                                                                                                                   |
| -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `scripts/check-anti-dumping.sh`        | Linter that fails on catch-all names (`utils/`, `helpers/`, `common/`, `shared/`, `misc.*`, `lib.*`) inside capability roots. Configurable via `.anti-dumping.json`. |
| `scripts/check-no-skipped-tests.sh`    | Fails on committed `.skip(` / `xit(` / `@Disabled` / `@pytest.mark.skip` markers — quarantined tests should never reach `main`.           |
| `scripts/check-no-sleep-waits.sh`      | Fails on `Thread.sleep`, `time.sleep`, `waitForTimeout` — hard-wait sleeps hide race conditions and slow suites.                          |
| `scripts/check-port-adapter-parity.sh` | For every `*.ports.*` ensures at least one adapter exists; warns if no in-memory test double is present.                                  |
| `scripts/check-seam-contract-parity.sh` | For every seam declared in `.seam-contracts.json`, ensures a machine-readable Shape and a shared Behavior suite exist. Warn-first, mode-promoted once clean. |
| `scripts/check-config-externalized.sh` | Production-readiness probe (Configurable anchor): fails on hardcoded remote URLs, endpoints, or secret literals. Warn-first via `.config-externalization.json`, reviewed per-line opt-out. |
| `scripts/check-observability-at-seams.sh` | Production-readiness probe (Observable anchor): flags a file that makes a boundary call but carries no log/metric/trace/correlation-id. Warn-first via `.observability.json`, reviewed per-file opt-out. |
| `scripts/check-stateless-request-path.sh` | Production-readiness probe (Horizontally-scalable anchor): flags node-local mutable state (module-level/static cache/session/registry) on the request path. Warn-first via `.statelessness.json`, reviewed per-line opt-out. |
| `scripts/check-resilient-boundary.sh` | Production-readiness probe (Resilient anchor): flags a file that makes a boundary call but declares no timeout/retry/circuit-breaker/fallback. Warn-first via `.resilience.json`, reviewed per-file opt-out. |
| `scripts/check-sprint-id-collision.sh` | Coordination-artifact gate (emergent parallelism): fails when two active sprint files share an id token, the collision a bare `NNN+1` increment causes under parallel sprint creation. Exact, not heuristic. Warn-first via `.sprint-coordination.json`. |
| `scripts/validate-plugin.sh`           | Plugin self-test: SKILL.md frontmatter validity (incl. single-line `tools:`), JSON/YAML parse, cross-reference integrity, manifest version parity, enforcement-script syntax, inventory parity, agent-frontmatter validity, and fenced-code balance. |
| `scripts/test-probes.sh`               | Self-test for the guardrail probes' language coverage: runs `check-no-skipped-tests.sh` and `check-no-sleep-waits.sh` against multi-language fixtures and asserts the expected verdicts. |
| `scripts/gen-coverage-matrix.sh`       | Generates / checks `docs/coverage-matrix.md` from each probe's `--include` list, so the language-coverage claim cannot drift from reality. |

## How the two halves compose

```
PLANNING ARTIFACTS (lean delivery)              ENGINEERING DISCIPLINE (principal engineer)
─────────────────────────────────               ──────────────────────────────────────────
create-wave                                     bootstrap-project (greenfield)
  ↓                                             refactor-layered-to-capability (legacy)
create-product-design-spec                        ↓
create-product-architecture-spec  ←──────────── design-system-architecture (cross-cutting)
create-quality-spec               ←──────────── test-by-ownership
  ↓                                             design-capability-layout
start-thin-slice (triage + route)                 ↓
  ↓                                             ↓
create-sprint (immutable bridge)                  ↓
  ↓                                             ↓
intake-code-contribution                         ↓
  ↓                                             implement-with-defensive-patterns
[work happens]                                    ↓
  ↓                                             verify-and-assemble-pr
close-sprint (bidirectional outflow) ────────►  updates capability layout, ADRs, handbook
                                  └──────────►  updates wave docs (intent only)
```

A wave's `product-architecture.md` (wave-scoped) is the planning input that triggers `design-system-architecture` (cross-cutting) when a wave introduces a new subsystem. A sprint close updates **both** product artifacts AND engineering artifacts — the bridge dissolves once both shores are updated.

## Composes with Claude MPM

If [Claude MPM](https://github.com/anthropics/claude-mpm) is installed in the same project:

- **Praxis** owns planning artifacts (waves, sprints, ADRs, design + architecture + quality specs).
- **MPM** owns runtime mechanics (delegation patterns, verification gates, ticketing, branch protection, circuit breakers).
- Sprint files and `qa.md` are the artifacts MPM's PM agent hands to specialist agents.

## Precedence

```
repo .github/copilot-instructions.md, .claude/CLAUDE.md   (highest — project owns final word)
repo .github/instructions/*.instructions.md (scoped)
repo .github/agents/, .github/skills/
─────────────────────────────────────────────────────────
plugin instructions, agents, skills                       (defaults — what this plugin provides)
─────────────────────────────────────────────────────────
user ~/.claude/CLAUDE.md, VS Code user prompts            (personal preferences only)
```

A repo can disable any plugin instruction or skill by adding a same-named file with stricter rules, or by referencing it explicitly in its own `copilot-instructions.md`.

## Installation

Installation differs by harness. If you use more than one, install Praxis separately for each one. Every harness loads the same canonical bootstrap (`skills/using-praxis/SKILL.md`) so the agent's behavior is consistent across runtimes.

### Claude Code

Register the marketplace, then install the plugin:

```bash
/plugin marketplace add nuewframe/praxis
/plugin install praxis@nuewframe-marketplace
```

The `SessionStart` hook (`hooks/hooks.json`) automatically injects the bootstrap on every new session.

### Codex CLI

If the marketplace is registered:

```bash
/plugins
```

Search for `praxis` and select **Install Plugin**. Praxis exposes its skills and agents via `.codex-plugin/plugin.json`.

For manual install from this repo, follow Codex's documented `git+https` install flow against `https://github.com/nuewframe/praxis`.

### Codex App

In the Codex app sidebar, open **Plugins**, find **Praxis** in the Coding category, click `+` and follow the prompts.

### Cursor

In Cursor Agent chat:

```text
/add-plugin praxis
```

Or search for `praxis` in the Cursor plugin marketplace. Praxis ships a Cursor `sessionStart` hook (`hooks/hooks-cursor.json`) that injects the bootstrap automatically.

### Gemini CLI

```bash
gemini extensions install https://github.com/nuewframe/praxis
```

Update later:

```bash
gemini extensions update praxis
```

Gemini reads `gemini-extension.json` and loads `GEMINI.md`, which references the bootstrap skill.

### OpenCode

Add Praxis to the `plugin` array in your `opencode.json` (global or project-level):

```json
{
  "plugin": ["praxis@git+https://github.com/nuewframe/praxis.git"]
}
```

Restart OpenCode. The plugin registers the skills directory and injects the bootstrap into the first user message of every session. Detailed docs: [`.opencode/INSTALL.md`](./.opencode/INSTALL.md).

### GitHub Copilot CLI

```bash
copilot plugin marketplace add nuewframe/praxis
copilot plugin install praxis@nuewframe-marketplace
```

### GitHub Copilot in VS Code

Filesystem install (per-machine):

```bash
PLUGIN_SRC="$(pwd)"
mkdir -p ~/.copilot/installed-plugins/nuewframe-marketplace
ln -sf "$PLUGIN_SRC" ~/.copilot/installed-plugins/nuewframe-marketplace/praxis
```

Workspace-scoped (no symlink):

```jsonc
// .vscode/settings.json
{
  "chat.pluginLocations": {
    "${workspaceFolder}": true
  }
}
```

Restart VS Code so Copilot rescans. Both Claude Code and VS Code Copilot load `instructions/*.instructions.md` natively, so the always-on guardrails activate in any workspace where Praxis is installed.

### Verifying the install

In any harness, ask:

> *Tell me about your praxis.*

The agent should name the three personas, the always-on guardrails, and at least four skills with their triggers. If it can't, the bootstrap isn't loaded — see the harness's troubleshooting docs.

## How to use it from a new project

1. Install the plugin once per machine.
2. From an empty repo, ask Claude or Copilot: **"Bootstrap this project using the praxis plugin."** The `bootstrap-project` skill takes over.
3. For the first wave, run:
   - `create-wave` (product-manager)
   - `create-product-design-spec` (product-designer)
   - `create-product-architecture-spec` (principal-engineer)
   - `create-quality-spec` (product-designer + principal-engineer)
4. For the first sprint: run `create-sprint` (product-manager — locks the bridge), then `intake-code-contribution` (principal-engineer — confirms wave, thin-slice, specs, sprint bridge, current code, and red/green test posture), then implement using `discovery-and-ambiguity-log` → `design-capability-layout` → `implement-with-defensive-patterns` → `verify-and-assemble-pr`, then `close-sprint` (product-manager — bidirectional outflow).
5. Wire `scripts/check-anti-dumping.sh` into the project's task runner and CI.

## Documentation

- [project-context.md](project-context.md) — plugin governance, scope rules, evolution policy.
- [CHANGELOG.md](CHANGELOG.md) — version history.

## License

MIT.
