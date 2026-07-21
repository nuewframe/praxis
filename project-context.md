# Praxis — Project Context

This is the single entry point for understanding what this plugin is, what belongs in it, and how it evolves. Read this before adding, removing, or modifying any plugin file.

## Identity

**Name:** `praxis` **Purpose:** Make an LLM coding agent execute disciplined, lean wave-based delivery and Principal-Software-Engineer practice with **fidelity** — producing trustworthy artifacts a human team can build on — instead of improvising. **Reach:** installable, versioned guidance for Claude Code, Codex, Cursor, Gemini CLI, OpenCode, and GitHub Copilot, written to apply to any language, framework, or runtime. Wide availability across harnesses is a deliberate distribution goal, not a hedge to be earned into with usage evidence — that evidentiary bar applies to methodology-fidelity claims (see Problem, above), not to how many harnesses can install the same doctrine. Breadth is how widely the value ships; it is not the value itself. **Status:** v0.4.0 — multi-harness installable plugin.

## Problem

GenAI coding agents, left unconstrained, default to improvising structure, skipping discovery, drifting scope, and producing plausible-looking artifacts with no substance behind them. Human engineering teams already have the discipline that prevents this — waves, ADRs, thin slices, test-by-ownership, a review firewall between the engineer who builds and the one who approves. That discipline is not in question, and it is not what this plugin invents.

The problem is **trust transfer**. When a human engineer produces an architecture document, reviewers trust it reflects real reasoning. When an agent produces the identical document, that trust is unearned — the artifact looks the same whether the agent reasoned hard or pattern-matched a template. Agents now generate delivery artifacts at scale, so the gap between *looks like discipline happened* and *discipline happened* is newly acute and newly expensive.

Praxis exists to close that gap: to make agent-generated delivery artifacts trustworthy enough that a human team can build on them without re-deriving the reasoning. Its primary output is therefore **execution fidelity** — the agent demonstrably did the disciplined work, and the places it did not are visible rather than hidden. Legibility (making the discipline explicit enough for an agent to follow it intentionally) is Praxis's half of that; runtime enforcement is a separate layer (see *Composes with orchestration runtimes*).

## The method at a glance

Praxis is not a toolbox of independent skills; it is one ordered pipeline that carries a unit of work from intent to shipped, verified, and closed — emitting, at each stage, the script-checkable artifact that makes the discipline visible. That emission is the fidelity story of the Problem above: a reviewer can *see* the work happened rather than trust that it did.

This section is a map, not a specification. The **authoritative ordered path** is `start-thin-slice` Step 5; the **authoritative tier table** is `intake-code-contribution` Step 0. This view points at them and must not be treated as a second source — when they change, they win.

**The spine (Standard tier):**

```
PLAN ──────────► TRIAGE ────────► BUILD ─────────────────────► LEARN ─────────► TEACH
create-wave →    start-thin-      create-sprint → intake →      close-sprint     author-
product docs:    slice            implement → verify            (bidirectional   user-docs
the outcomes and (tier + hard                                   distillation,
the educated     precondition                                   then the sprint
theory of design gate)                                           is deleted)
```

PLAN produces the **product documents** — the intended product outcomes and the best **educated theory** of the proposed product design: its user experience (`product-design.md`) and product architecture (`product-architecture.md`). *Educated theory*, not settled truth — at PLAN these are the strongest reasoned proposal, validated only downstream. TRIAGE reads them to check a slice's preconditions and classify its tier, so PLAN is measured by whether the documents carry the outcomes and design a later decision needs, not by a document count; `intake` later verifies they are present and specific enough to build on.

**Tier branching** — decided provisionally at `start-thin-slice`, authoritatively at `intake` Step 0:

- **Trivial** — no sprint, no close: `intake` (abbreviated) → `implement-with-defensive-patterns` → `verify-and-assemble-pr`.
- **Standard** — the full spine above.
- **Major** — inserts the architect pipeline *before* the sprint: `discovery-and-ambiguity-log` → `design-system-architecture` → `design-capability-layout` → `create-adr` (**`status: Accepted`**) → then `create-sprint` onward. `define-seam-contract` is used wherever the design declares a boundary another slice will build against.

Greenfield work starts at `bootstrap-project`; a legacy codebase enters through `refactor-layered-to-capability`, one shippable slice at a time.

**Where fidelity is made.** Each stage emits a visible artifact or gate, so the discipline is checkable rather than assumed:

| Stage | Emits (the visible proof) | Kind |
|---|---|---|
| Plan | the product documents — intended product outcomes and the best educated theory of the proposed product design (UX in `product-design.md`, architecture in `product-architecture.md`), holding the thin-slice definitions and acceptance criteria TRIAGE classifies from; presence and specificity verified at intake | script-checkable |
| Triage | precondition hard-gate (status + dependencies `✅`) and the tier decision | script-checkable |
| Architecture (Major) | ADR with a real alternatives table and `status: Accepted`; frozen seam contracts in `.seam-contracts.json` | script-checkable |
| Sprint | signed Sprint Plan Approval (Standard+Major) and Design Approval (Major); Acceptance↔Test matrix; four production-readiness anchors | human-signed + script-checkable |
| Verify | captured `verify` output — a bare checkbox is rejected — plus the adversarial seam-behavior review | script-checkable + agent-attested |
| Close | outcome evidence and a continue / pivot / stop call, distilled back into product *and* engineering artifacts | agent-attested |

The **Kind** column maps to `using-praxis` § *Enforcement model* and to *Script-checkable vs. runtime-enforced* below: script-enforced gates fail closed once wired into CI or a git hook; human-signed gates block the next skill; agent-attested gates are honored in good faith on a bare harness.

## Scope rule (the litmus test)

A piece of guidance belongs in this plugin **only if** the answer to all four is yes:

1. Would it apply unchanged to a Rust CLI, a Python data pipeline, _and_ a TypeScript web app — across both engineering and product planning contexts?
2. Is it about engineering or delivery discipline, not personal style?
3. Would you defend it in a code review or planning review against any team?
4. Does it measurably improve the agent's execution fidelity, or close a known agent failure mode?

The first three questions decide _where_ a rule belongs. If any of them is no, it belongs in a project's `.github/` (project-specific) or in `~/.claude/` / VS Code user prompts (personal preference) — **not here**. The fourth decides whether the rule is worth adding _at all_: if it is no, the rule does not belong in the plugin regardless of how universal it is. Universality is necessary but not sufficient — a rule can be universal, disciplined, and defensible while still adding more ceremony than value, and such a rule stays out.

### Examples

| Item                                                                                 | In plugin?    | Reason                                                              |
| ------------------------------------------------------------------------------------ | ------------- | ------------------------------------------------------------------- |
| Capability-driven layout rule                                                        | Yes           | Universal, language-agnostic.                                       |
| Anti-dumping policy (no `utils/`, `helpers/`)                                        | Yes           | Universal.                                                          |
| Functional core / imperative shell                                                   | Yes           | Universal.                                                          |
| Phased delivery (Discovery → … → Verify)                                             | Yes           | Universal.                                                          |
| ADR creation discipline                                                              | Yes           | Universal.                                                          |
| Wave four-document pattern (README, design, architecture, qa)                        | Yes           | Universal delivery rhythm.                                          |
| Sprint-as-immutable-bridge model                                                     | Yes           | Universal lean discipline.                                          |
| Code contribution intake gate (wave, slice, specs, sprint, code state, test posture) | Yes           | Universal GenAI contribution hygiene before implementation.         |
| Pyramid Test Strategy (Logic base through Journey tip, organized by ownership)       | Yes           | Universal — naming may differ but boundaries are real in any stack. |
| Three-mode principal-engineer persona (architect / implementer / reviewer)           | Yes           | Universal bias firewall — same engineer cannot self-approve.        |
| Three-tier change triage (Trivial / Standard / Major)                                | Yes           | Universal scoping discipline — keeps process proportional.          |
| Mechanical Design Approval (ADR `status: Accepted` + signed sprint line)             | Yes           | Universal gate — prevents prose-only approvals.                     |
| `entity → repository → service → api` template                                       | No — project  | Stack-specific to Deno/Hono.                                        |
| SurrealDB query patterns                                                             | No — project  | Tech-specific.                                                      |
| `deno task` wiring of the linter                                                     | No — project  | Tooling-specific.                                                   |
| Project-specific design-token module names (theme objects, token CSS vars)           | No — project  | Stack-specific; lives in `design-tokens.instructions.md` overlay.   |
| "Use single quotes"                                                                  | No — personal | Personal style preference.                                          |
| Anti-dumping linter binary itself (generic, configurable)                            | Yes           | Universal mechanism, project supplies the config.                   |

## Architecture (capability-driven, applied to itself)

The plugin practices what it preaches. Each capability is a top-level folder. Harness-specific manifests are isolated in their own dotted folders so the substantive content (skills, agents, instructions) stays single-source.

```
plugin/
├── .claude-plugin/                              # Claude Code manifest capability
│   ├── plugin.json
│   └── marketplace.json
├── .codex-plugin/                               # Codex CLI / App manifest capability
│   └── plugin.json
├── .cursor-plugin/                              # Cursor manifest capability
│   └── plugin.json
├── .opencode/                                   # OpenCode plugin capability
│   ├── INSTALL.md
│   └── plugins/praxis.js
├── gemini-extension.json                        # Gemini CLI manifest
├── package.json                                 # OpenCode / npm metadata
├── hooks/                                       # session-start hooks capability
│   ├── hooks.json                               # Claude Code
│   ├── hooks-cursor.json                        # Cursor
│   ├── run-hook.cmd                             # cross-platform polyglot wrapper
│   └── session-start                            # bootstrap injector
├── instructions/                                # always-on guardrails capability
│   ├── capability-driven-guardrails.instructions.md
│   ├── lean-delivery-guardrails.instructions.md
│   └── code-contribution-intake.instructions.md
├── agents/                                      # persona capability
│   ├── principal-engineer.agent.md
│   ├── product-manager.agent.md
│   └── product-designer.agent.md
├── skills/                                      # delivery + engineering workflow capabilities
│   ├── using-praxis/                            # bootstrap entry point (loaded by every harness)
│   ├── create-wave/
│   ├── create-product-design-spec/
│   ├── create-product-architecture-spec/
│   ├── create-quality-spec/
│   ├── test-by-ownership/
│   ├── intake-code-contribution/
│   ├── start-thin-slice/
│   ├── create-sprint/
│   ├── close-sprint/
│   ├── author-user-docs/
│   ├── create-adr/
│   ├── define-seam-contract/
│   ├── discovery-and-ambiguity-log/
│   ├── design-system-architecture/
│   ├── design-capability-layout/
│   ├── implement-with-defensive-patterns/
│   ├── verify-and-assemble-pr/
│   ├── bootstrap-project/
│   ├── provision-project-overlay/
│   └── refactor-layered-to-capability/
├── scripts/                                     # generic enforcement tooling capability
│   ├── check-anti-dumping.sh
│   ├── check-no-skipped-tests.sh               # test-hygiene gate (no committed skipped tests)
│   ├── check-no-sleep-waits.sh                 # test-hygiene gate (no hard-wait sleeps)
│   ├── check-port-adapter-parity.sh            # port/adapter parity gate
│   ├── check-config-externalized.sh            # production-readiness probe (Configurable anchor)
│   ├── check-observability-at-seams.sh         # production-readiness probe (Observable anchor)
│   ├── check-stateless-request-path.sh         # production-readiness probe (Horizontally-scalable anchor)
│   ├── check-resilient-boundary.sh             # production-readiness probe (Resilient anchor)
│   ├── check-seam-contract-parity.sh           # seam-contract parity gate (Shape + Behavior suite)
│   ├── check-sprint-id-collision.sh            # coordination-artifact gate (parallel sprint-id collision)
│   ├── check-design-approval-gate.sh        # Major-tier Design Approval fail-closed gate (hard-fail, not warn-first)
│   ├── check-escape-hatch-usage.sh          # diff-scoped escape-hatch marker report (informational, never fails)
│   ├── bump-version.sh                          # version-parity tool across manifests
│   ├── test-probes.sh                           # self-test: probe language coverage (fixtures)
│   ├── gen-coverage-matrix.sh                   # generate/check docs/coverage-matrix.md from probe includes
│   ├── gen-tier-table.sh                        # generate/check the tier table across 3 surfaces from scripts/data/tier-classification.json
│   └── validate-plugin.sh                       # plugin self-test
├── AGENTS.md                                    # bootstrap pointer for AGENTS.md-aware harnesses
├── CLAUDE.md                                    # bootstrap pointer for Claude Code
├── GEMINI.md                                    # bootstrap pointer loaded by Gemini extension
├── README.md
├── project-context.md                           # this file
├── .version-bump.json                           # declared manifests for the bump tool
└── CHANGELOG.md
```

**No `utils/`. No `common/`. No `shared/`.** If a future addition tempts a dumping ground, name the actual capability or duplicate.

## Single source, many harnesses

Every harness manifest (`.claude-plugin/`, `.codex-plugin/`, `.cursor-plugin/`, `.opencode/`, `gemini-extension.json`) points at the **same** `skills/`, `agents/`, and `instructions/` trees. There are no per-harness content forks. The bootstrap skill `skills/using-praxis/SKILL.md` is the single entry point loaded by every harness through its native mechanism (SessionStart hook, system-prompt transform, or `@`-include in a context file). Versions across all manifests are kept in sync by `scripts/bump-version.sh` against `.version-bump.json`.

## Two halves, one method

The plugin bundles two complementary capability stacks that share the same precedence and scope rules:

- **Lean delivery** — wave planning, design / architecture / quality specs, sprint as immutable bridge, ADRs, Pyramid Test Strategy, bidirectional sprint close. Owned by the product-manager and product-designer personas.
- **Principal Engineer discipline** — phased workflow (Discovery → System Architecture → Capability Layout → Implementation → Verify), bootstrap, refactor-to-capability, anti-dumping enforcement. Owned by Principal Engineer persona.

The halves compose: `create-product-architecture-spec` (wave-scoped) feeds `design-system-architecture` (cross-cutting) when a wave introduces a new subsystem; `create-quality-spec` and `test-by-ownership` shape what `verify-and-assemble-pr` actually verifies; `close-sprint` writes back into both product artifacts (intent) and engineering artifacts (ADRs, capability layout, handbook).

## Composes with orchestration runtimes (Claude MPM, others)

This plugin is **artifact discipline**, not runtime orchestration. It deliberately does not implement:

- Agent delegation / hand-off mechanics
- Verification gates as runtime checks
- Ticketing integration
- Branch protection / PR creation flows
- Circuit breakers, error budgets at runtime

Those concerns belong in an orchestration runtime such as [Claude MPM](https://github.com/anthropics/claude-mpm). When MPM is installed alongside this plugin:

- Praxis produces the artifacts (`product-design.md`, `product-architecture.md`, `qa.md`, sprint files, ADRs, wave READMEs, handbook updates).
- MPM's PM agent uses those artifacts as the source of truth for delegation, and its specialists (`typescript`, `qa`, `design`, `product`) align to the personas defined here (`principal-engineer`, `product-designer`, `product-manager`). Host repos may overlay project-specific persona names (e.g. `manny`, `shelby`, `rusty`) that extend these role-based agents.

The two are orthogonal. Either can be used without the other; together they cover both "what to build" and "how the agents collaborate to build it".

The Major-tier Design Approval gate illustrates how the composition would extend beyond a bare harness. `scripts/check-design-approval-gate.sh` is a plain script with no runtime dependency — on a bare harness it runs as a pre-push git hook and fails the push. The same mechanism generalizes: an orchestration runtime such as MPM could invoke this script as a pre-delegation gate, run before the PM agent dispatches implementer-mode work on a Major-tier slice, refusing to hand off the task at all if the check fails — a stronger enforcement point than blocking only at push time, since it stops the agent from ever starting the work. This is a documented composition pattern, not a built or tested MPM integration; Praxis ships the mechanism (the script), not the runtime wiring, consistent with this plugin's stated non-goal of not shipping executable agents or MCP servers.

### Script-checkable vs. runtime-enforced

Two things are easily confused. A **script-checkable artifact** — an ADR with `status: Accepted`, a signed sprint line, a non-empty alternatives table — is something a probe can verify exists and carries substance. That is Praxis's job. A **runtime-enforced gate** — the agent is actually prevented from proceeding until the check passes — requires an orchestration runtime and is not Praxis's job on a bare harness. When this document or a skill calls a gate "mechanical," it means script-checkable, not runtime-enforced. On a bare harness, honoring the gate is a self-enforced behavioral contract; with MPM or equivalent it can be made to fail closed. Claiming more than this is the one failure the trust-transfer problem cannot tolerate — false trust is worse than none.

## Layering and precedence

The plugin is one tier in a four-tier system:

```
repo .github/copilot-instructions.md, .claude/CLAUDE.md      (highest)
repo .github/instructions/, .github/agents/, .github/skills/
─────────────────────────────────────────────────────────────
plugin instructions/, agents/, skills/                       (defaults)
─────────────────────────────────────────────────────────────
user ~/.claude/CLAUDE.md, VS Code user prompts               (personal)
```

Repo guidance always overrides plugin guidance. The plugin should never assume its rules are the final word.

## Evolution policy

### Adding a skill, instruction, or agent

1. Pass the scope litmus test above — all four questions. If it fails, do not add.
2. Validate the rule by using it in at least one real repo first. This is the non-negotiable step, not a formality. Dogfooding the plugin on itself does not substitute: it proves internal consistency, not that the rule improves fidelity on real work. Do not invent rules theoretically.
3. Only once real-repo validation exists: write the artifact, dogfood it against the plugin if possible, and bump the minor version. The `CHANGELOG.md` entry for that bump must cite the real-repo evidence from step 2; a bump that can only cite dogfooding does not qualify.

### Removing or breaking a rule

1. Document the deprecation in `CHANGELOG.md` one minor version before removal.
2. Major-version bump on actual removal.

### Versioning

- Patch: typo, clarification, formatting.
- Minor: new skill / instruction / agent / additive rule.
- Major: removed skill, breaking rule change, structural reorganization, or plugin rename.

## Quality gates for plugin contributions

- Every skill has `name` and `description` frontmatter and an "Use this when" section at the top.
- Every instruction has `applyTo` if scoped, or is explicitly always-on.
- Every agent has a single, unambiguous responsibility.
- No skill prescribes a specific language or framework. If an example is needed, show two contrasting languages.
- No file references stack-specific paths (e.g., `services/api/src/domains/`). Use placeholders like `<capability-root>/` or `<docs-root>/product/waves/`.
- Anti-dumping rule applies recursively — the plugin itself must not introduce `utils/`, `helpers/`, `common/`, `shared/`.

## Extraction status

The plugin is a standalone repository. Earlier versions incubated inside a host repo; that history has been migrated and no extraction work remains.

## Non-goals

- This plugin does **not** prescribe a stack, framework, or language.
- This plugin does **not** replace project-specific instructions, skills, or agents.
- This plugin does **not** include personal style preferences (formatting, quote style, naming flavor).
- This plugin does **not** provide runtime orchestration (delegation, ticketing, gates) — that belongs in MPM or equivalent.
- This plugin does **not** ship MCP servers or executable agents — only guidance and one generic linter.
