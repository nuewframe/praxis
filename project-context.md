# Praxis — Project Context

This is the single entry point for understanding what this plugin is, what belongs in it, and how it evolves. Read this before adding, removing, or modifying any plugin file.

## Identity

**Name:** `praxis` **Purpose:** Encode universal, lean wave-based product delivery **and** Principal-Software-Engineer discipline as installable, versioned guidance for Claude Code, Codex, Cursor, Gemini CLI, OpenCode, and GitHub Copilot, applicable to any language, framework, or runtime. **Status:** v0.1.0 — initial public release; multi-harness installable plugin.

## Scope rule (the litmus test)

A piece of guidance belongs in this plugin **only if** the answer to all three is yes:

1. Would it apply unchanged to a Rust CLI, a Python data pipeline, _and_ a TypeScript web app — across both engineering and product planning contexts?
2. Is it about engineering or delivery discipline, not personal style?
3. Would you defend it in a code review or planning review against any team?

If any answer is no, it belongs in a project's `.github/` (project-specific) or in `~/.claude/` / VS Code user prompts (personal preference) — **not here**.

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
│   ├── create-sprint/
│   ├── close-sprint/
│   ├── create-adr/
│   ├── define-seam-contract/
│   ├── discovery-and-ambiguity-log/
│   ├── design-system-architecture/
│   ├── design-capability-layout/
│   ├── implement-with-defensive-patterns/
│   ├── verify-and-assemble-pr/
│   ├── bootstrap-project/
│   └── refactor-layered-to-capability/
├── scripts/                                     # generic enforcement tooling capability
│   ├── check-anti-dumping.sh
│   ├── check-config-externalized.sh            # production-readiness probe (Configurable anchor)
│   ├── check-observability-at-seams.sh         # production-readiness probe (Observable anchor)
│   ├── check-stateless-request-path.sh         # production-readiness probe (Horizontally-scalable anchor)
│   ├── check-resilient-boundary.sh             # production-readiness probe (Resilient anchor)
│   ├── check-seam-contract-parity.sh           # seam-contract parity gate (Shape + Behavior suite)
│   ├── check-sprint-id-collision.sh            # coordination-artifact gate (parallel sprint-id collision)
│   ├── bump-version.sh                          # version-parity tool across manifests
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

1. Pass the scope litmus test above. If it fails, do not add.
2. Validate the rule by using it in at least one real repo first. Do not invent rules theoretically.
3. Once proven, write the artifact, run the same skill/instruction against itself if possible (dogfood), and bump the minor version.

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
