# Praxis

A portable agent plugin that fuses **lean wave-based product delivery** with **Principal Engineer discipline** into a single composable method. Language-, framework-, and runtime-agnostic in doctrine (static enforcement is best-effort per language — see [docs/releases/<version>/doctrine/](docs/releases/<version>/doctrine/)); installable into Claude Code, Codex (CLI and App), Cursor, Gemini CLI, OpenCode, and GitHub Copilot (CLI and VS Code).

Praxis is universal: it does not assume any stack. Project-specific rules belong in the project's own `.github/` and `.claude/` files and override anything here.

## Enforcement, honestly

Praxis ships four kinds of gate, and a green check does not mean all four are equally binding.

**Record-enforced** gates — `praxis check`, `praxis pick-up`, `praxis close` — fail closed
with no configuration and no `mode`. They refuse: a malformed record, an edge naming
something the record does not hold, an unsigned admission, a claim dropped from a slice to
make a close succeed, a seal that no longer matches, a doctrine surface the plugin does not
ship. **Record-reported** gates name a problem every run without blocking — an unwitnessed
rule, an unanchored surface, a layer reached and unevidenced — and the count is the answer
to a question that was previously unanswerable. **Script-enforced** gates (the `check-*.sh`
probes wired into `verify.sh`, CI, or a git hook) fail closed once wired; several are
warn-first until you set `mode: enforce`. **Agent-attested** gates — four-anchor
conformance, red-first posture, the adversarial seam review — are honored in good faith and
are **not** mechanically compelled on a bare harness.

The point of the delivery graph is to move rules up that list. See
[`skills/using-praxis/SKILL.md`](skills/using-praxis/SKILL.md) § *Enforcement model*. The
production-readiness probes under `scripts/` are explicitly labeled heuristics, not proofs,
in their own header comments — read one before trusting a green run as more than that.

## Quickstart

Give your agent Praxis: [Claude Code](#claude-code) · [Codex CLI](#codex-cli) · [Codex App](#codex-app) · [Cursor](#cursor) · [Gemini CLI](#gemini-cli) · [OpenCode](#opencode) · [GitHub Copilot CLI](#github-copilot-cli) · [GitHub Copilot in VS Code](#github-copilot-in-vs-code).

## What this plugin gives you

### Personas

| Agent                                | Role                                                                                                                                                       |
| ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agents/product-manager.agent.md`    | Distinguished Product Manager — framing the problem, cutting slices from a storm, signing the admission, dashboard honesty. |
| `agents/product-designer.agent.md`   | Distinguished Product Designer — user value, the `design-ux` phase, a slice's scenario and claims, what a read model must answer before an actor can act. |
| `agents/principal-engineer.agent.md` | Distinguished Engineer — capability-driven architecture, refactoring, cross-cutting decisions.                                                             |

### Always-on guardrails

| Instruction                                                 | Scope                                                                                                                     |
| ----------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `instructions/capability-driven-guardrails.instructions.md` | Capability-driven layout, anti-dumping policy, functional core / imperative shell, ADR discipline, telemetry baseline.    |

### Skills — The delivery graph (the record, and the engine that checks it)

The `praxis` binary reads these records and refuses what does not hold together. Every skill
below corresponds to a slice the record delivered — run `praxis audit-surfaces` to see the
anchor.

| Skill | Purpose |
| ----- | ------- |
| `skills/ask-what-is-true/` | Ask the record what this repository already knows before reconstructing it — and read what the answer says it does not cover. |
| `skills/event-storming/` | Upstream domain discovery — map business events to bounded contexts and candidate capabilities. |
| `skills/name-a-capability/` | Derive a capability from a cluster of events: consistency boundary, the four gate tests with their reasons, and the exclusion that makes the boundary real. |
| `skills/cut-a-slice/` | Cut an atomic vertical slice in a shape a checker can refuse — one command or one view, one actor, declared layers, claims naming their evidence. |
| `skills/declare-an-entity-kind/` | Declare a kind the record must hold, and its shape, on the architecture's schema. An amendment to the record, never a change to the engine. |
| `skills/witness-a-rule/` | Declare a rule together with the record that demonstrates it refusing. A rule never shown to refuse is indistinguishable from one that cannot. |
| `skills/anchor-a-doctrine-surface/` | Declare what a shipped skill, guardrail, agent or probe exists to serve. Instruction the record cannot trace is doctrine on the plugin's authority alone. |
| `skills/declare-an-invariant/` | Declare what this plugin guarantees about code it is loaded into, and anchor the probe that keeps it. `enabled` and `enforced` are different facts. |
| `skills/see-what-is-ready/` | Which slices could be started right now, and what would refuse each of the rest — including the conditions the gate declares and nobody computed. |
| `skills/see-the-dashboard/` | Where the product stands right now: several views composed on demand, each still singly owned, never committed. |
| `skills/pick-up-a-slice/` | Take one or more slices through the one gate, as one commitment. Refusals are recorded as facts; an override is recorded beside the refusal it overrides. |
| `skills/attach-evidence/` | Back a declared layer with an artifact at the moment it is reached. Evidence outside the slice's declared set is refused. |
| `skills/review-in-flight/` | Preview what an iteration promised against what it has shown, writing nothing. The layers it has NOT reached are listed, not omitted. |
| `skills/record-a-decision/` | Record a choice with the alternatives it rejected and what would show it wrong, bound to the iteration that forced it, corrected only by appending. |
| `skills/document-usage/` | Write how a capability is used while building it. A guide for behaviour a version never shipped is refused. |
| `skills/close-an-iteration/` | Close without dropping scope in silence. A shortfall is carried by a finding that names the claim; deleting a claim to make the close succeed is itself refused. |
| `skills/bind-work-to-a-version/` | Attach closed iterations to a version so its content is derived rather than hand-kept. Only closed work binds, once. |
| `skills/declare-a-lifetime/` | Decide whether a read model survives being frozen, and record why. Refused in both directions without a reason. |
| `skills/publish-the-release-set/` | Regenerate every published document whole for one version, before the cut. No splice path, no wall-clock stamp. |
| `skills/cut-the-version/` | Cut a planned version and write its index node in one operation. The index points at a commit rather than copying it; the seal makes a later edit visible. |
| `skills/verify-the-published-tree/` | Prove no published document was hand-edited — compared against the commit its release names, never against the record it is meant to outlive. |
| `skills/promote-what-shipped/` | Fold a cut release into what each capability says it is. Derived from bound work, recomputed by a rule, refused if hand-edited in either direction. |
| `skills/resolve-a-symptom/` | Close a symptom by naming a release that demonstrably attacked it. Computed from what shipped, and withdrawn rather than grandfathered when it cannot be. |

### Skills — Engineering doctrine (not yet modelled by the record)

| Skill                                      | Purpose                                                                                                                      |
| ------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| `skills/event-storming/`                   | Upstream domain discovery — map business events to bounded contexts, candidate `CAP.` records, and `INIT.` initiatives.      |
| `skills/test-by-ownership/`                | Universal Pyramid Test Strategy: Logic base through Journey tip, with "one property of a behavior, one layer" rule.          |
| `skills/ingest-operational-feedback/`     | Downstream feedback intake — process incident post-mortems, operator friction logs, and SLO reviews into `CAP.` invariants. |
| `skills/define-seam-contract/`             | Define a Seam Contract for a boundary: machine-readable Shape + shared Behavior suite + frozen `<name>@vN` id in `.seam-contracts.json`. |

### Skills — Principal Engineer discipline (phased delivery + bootstrap)

| Skill                                       | Purpose                                                                                                              |
| ------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `skills/design-system-architecture/`        | Phases 2 + 3 — topology, resilience patterns, contract-first APIs, polyglot persistence, expand/contract migrations. |
| `skills/design-capability-layout/`          | Phase 4 — vertical-slice folder layout, functional core / imperative shell mapping.                                  |
| `skills/implement-with-defensive-patterns/` | Phase 5 — composition over inheritance, shift-left security, structured telemetry.                                   |
| `skills/adopt-the-method/`                  | The one front door. `praxis adopt` writes a binding, a verify entry point naming only probes that ship, and one engineering-doctrine pointer — greenfield or not. |
| `skills/prepare-project-for-ast/`           | Prepare a project or repository for polyglot AST seam parsing and probe validation (`ast-parser@v1`). |
| `skills/refactor-layered-to-capability/`    | Migrate a legacy `controllers/` + `services/` + `utils/` codebase into vertical slices.                              |

### Tooling

These scripts check **shape and presence** — a file exists, a pattern matches, a count is right — not the substance of the reasoning behind it. Several are explicitly labeled heuristics in their own header comments; read a `check-*.sh` script's top comment before trusting a green run as more than that.

| Script                                 | Purpose                                                                                                                                   |
| -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `scripts/ast_parse.sh`                 | Polyglot AST parsing dispatcher bridge: dispatches source files to language-native AST runners emitting standard `ast-parser@v1` JSON. |
| `scripts/check-anti-dumping.sh`        | Linter that fails on catch-all names (`utils/`, `helpers/`, `common/`, `shared/`, `misc.*`, `lib.*`) inside capability roots. Configurable via `.anti-dumping.json`. |
| `scripts/check-no-skipped-tests.sh`    | Fails on committed `.skip(` / `xit(` / `@Disabled` / `@pytest.mark.skip` markers — quarantined tests should never reach `main`.           |
| `scripts/check-no-sleep-waits.sh`      | Fails on `Thread.sleep`, `time.sleep`, `waitForTimeout` — hard-wait sleeps hide race conditions and slow suites.                          |
| `scripts/check-port-adapter-parity.sh` | For every `*.ports.*` ensures at least one adapter exists; warns if no in-memory test double is present.                                  |
| `scripts/check-seam-contract-parity.sh` | For every seam declared in `.seam-contracts.json`, ensures a machine-readable Shape and a shared Behavior suite exist. Warn-first, mode-promoted once clean. |
| `scripts/check-config-externalized.sh` | Production-readiness probe (Configurable anchor): fails on hardcoded remote URLs, endpoints, or secret literals. Warn-first via `.config-externalization.json`, reviewed per-line opt-out. |
| `scripts/check-observability-at-seams.sh` | Production-readiness probe (Observable anchor): flags a file that makes a boundary call but carries no log/metric/trace/correlation-id. Warn-first via `.observability.json`, reviewed per-file opt-out. |
| `scripts/check-stateless-request-path.sh` | Production-readiness probe (Horizontally-scalable anchor): flags node-local mutable state (module-level/static cache/session/registry) on the request path. Warn-first via `.statelessness.json`, reviewed per-line opt-out. |
| `scripts/check-resilient-boundary.sh` | Production-readiness probe (Resilient anchor): flags a file that makes a boundary call but declares no timeout/retry/circuit-breaker/fallback. Warn-first via `.resilience.json`, reviewed per-file opt-out. |
| `scripts/check-escape-hatch-usage.sh` | Scans a diff for Praxis's `praxis:allow-*` escape-hatch markers and reports each by file:line. Informational only — never fails the build; the point is that using an opt-out is never silent to a reviewer. |
| `scripts/validate-plugin.sh`           | Plugin self-test: SKILL.md frontmatter validity (incl. single-line `tools:`), JSON/YAML parse, cross-reference integrity, manifest version parity, enforcement-script syntax, inventory parity, agent-frontmatter validity, and fenced-code balance. |
| `scripts/test-probes.sh`               | Self-test for the guardrail probes' language coverage: runs `check-no-skipped-tests.sh` and `check-no-sleep-waits.sh` against multi-language fixtures and asserts the expected verdicts. |
| `scripts/test-citation-scan.sh`        | Self-test for `scripts/citation_scan.py`, the shared citation-vs-assertion implementation both literal scanners consume: asserts that a literal inside a fence, blockquote, or code span is a citation, that a long fence is not closed early by a shorter inner one, and that an inline marker without a reason fails. |

## How the two halves compose

```
THE DELIVERY GRAPH (the record)                 ENGINEERING DISCIPLINE (principal engineer)
─────────────────────────────────               ──────────────────────────────────────────
event-storming                                  praxis adopt (the front door)
name-a-capability                               refactor-layered-to-capability (legacy)
cut-a-slice                       ←──────────── design-system-architecture (cross-cutting)
  ↓                               ←──────────── test-by-ownership
praxis ready                                    design-capability-layout
  ↓                                               ↓
praxis pick-up  (the gate)                        ↓
  ↓  admits → ITER.                               ↓
  ↓  refuses → REF., nothing opens                ↓
[work happens]                    ←──────────── implement-with-defensive-patterns
  ↓  attach evidence per layer                    ↓
praxis close  (refuses a silent drop) ───────►  updates capability records, decisions
  ↓
praxis bind → publish → cut-release → promote → verify-published
```

The left column is **computed**. Every arrow is a command that refuses when its conditions
are not met, and every refusal is written down. The right column is doctrine an agent
follows — real, and not mechanically compelled. Which is which is the first thing to know
about any gate; see *Enforcement, honestly* above.


A wave's `product-architecture.md` (wave-scoped) is the planning input that triggers `design-system-architecture` (cross-cutting) when a wave introduces a new subsystem. A sprint close updates **both** product artifacts AND engineering artifacts — the bridge dissolves once both shores are updated.

## Composes with Claude MPM

If [Claude MPM](https://github.com/anthropics/claude-mpm) is installed in the same project:

- **Praxis** owns planning artifacts (waves, sprints, ADRs, design + architecture + quality specs).
- **MPM** owns runtime mechanics (delegation patterns, verification gates, ticketing, branch protection, circuit breakers).
- Sprint files and `qa.md` are the artifacts MPM's PM agent hands to specialist agents.

## Precedence

```
package <pkg>/.praxis/context.md, <pkg>/.github/          (highest — monorepo only)
repo .github/copilot-instructions.md, .claude/CLAUDE.md   (project owns final word)
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
2. In the repository — empty or not — run `praxis adopt --repository <owner>/<name>`. It writes a binding, a verify entry point, a pre-commit hook and one engineering-doctrine pointer, and **nothing about your product**. Read `adopt-the-method` for why that last part is the important one.
3. Ask the record what it already knows before reconstructing anything — `praxis truth`, and read what the answer says it does **not** cover.
4. Frame the problem and cut the first slices: `event-storming` → `name-a-capability` → `cut-a-slice`.
5. `praxis ready` says which slices could be started now, and what would refuse each of the rest.
6. `praxis pick-up TS.a TS.b` takes them through the one gate, as one commitment. It admits and opens an iteration, or refuses and records why.
7. Evidence each declared layer as you reach it; `praxis review` previews what has been shown without writing anything.
8. `praxis close` refuses a close that drops a claim in silence. A shortfall is carried by a finding that names it.
9. Wire `scripts/check-anti-dumping.sh` into the project's task runner and CI.

> **Adoption still owes one thing.** The schema arrives with the engine (`TS.260821.10`) and
> the front door writes a record that checks (`TS.260823.08`), so every step above runs. What
> is still unproven is whether the rest of the loop survives contact with a repository nobody
> here has seen — `adopt-the-method` says so, and this does not claim otherwise.

## Documentation

- [docs/product.md](docs/product.md) — the method, its doctrine, scope rules, and evolution policy.
- [CHANGELOG.md](CHANGELOG.md) — version history.

## License

MIT.
