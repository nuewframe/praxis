# Changelog

All notable changes to the Praxis plugin are documented here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this plugin uses semantic versioning (see [project-context.md](project-context.md) for the policy).

## [Unreleased]

## [0.1.3] — 2026-05-29

### Fixed

- Removed duplicate `hooks` declaration from `.claude-plugin/plugin.json`. Claude Code auto-loads `hooks/hooks.json` from the plugin root; declaring it again in `manifest.hooks` caused a "Duplicate hooks file detected" plugin load error.

## [0.1.2] — 2026-05-28

### Changed

- Unified ADR file naming to `ADR.<ID>-descriptive-name.md` across ADR workflows and examples.
- Removed the ADR-001 bootstrap exception; first ADR generation now follows the same `create-adr` ID convention as all subsequent ADRs.
- Updated provision overlay scaffolding to accept `bootstrap.first_adr_id` and emit `ADR.{{bootstrap.first_adr_id}}-technology-stack.md`.

## [0.1.0] — 2026-05-20

Initial public release.

### Personas

- **Product Manager** — wave planning, sprint creation as immutable bridges between product intent and engineering reality, sprint closing with bidirectional learning capture, honest product dashboard.
- **Product Designer** — authoritative voice of the user; defines value in measurable user outcomes; owns wave `product-design.md` and thin-slice acceptance criteria.
- **Principal Engineer** — one persona, three explicit modes (architect, implementer, reviewer); enforces capability-driven structure, anti-dumping policy, defensive design, the phased workflow, the refactor decision matrix, and the mechanical Design Approval gate.

### Always-on guardrails

- `lean-delivery-guardrails.instructions.md` — wave/sprint discipline, immutable sprint scope, bidirectional learning at close.
- `capability-driven-guardrails.instructions.md` — vertical slices, anti-dumping policy, port/adapter parity, no shared technical layers.
- `code-contribution-intake.instructions.md` — three-tier change triage (Trivial / Standard / Major) and the mechanical Design Approval gate for Major work.

### Skills

Lean delivery: `create-wave`, `create-product-design-spec`, `create-product-architecture-spec`, `create-quality-spec`, `create-sprint`, `close-sprint`.

Engineering: `discovery-and-ambiguity-log`, `design-system-architecture`, `design-capability-layout`, `create-adr`, `implement-with-defensive-patterns`, `verify-and-assemble-pr`, `intake-code-contribution`, `test-by-ownership`, `refactor-layered-to-capability`, `bootstrap-project`, `provision-project-overlay`.

Bootstrap: `using-praxis` — single skill index loaded at session start by every harness.

### Multi-harness installability

Praxis installs natively into Claude Code, Codex CLI, Codex App, Cursor, Gemini CLI, OpenCode, and GitHub Copilot (CLI + VS Code) from a single source tree. Every harness loads the same canonical bootstrap (`skills/using-praxis/SKILL.md`) so agent behavior is consistent across runtimes.

- Harness manifests: `.claude-plugin/plugin.json`, `.codex-plugin/plugin.json`, `.cursor-plugin/plugin.json`, `gemini-extension.json`, `.opencode/plugins/praxis.js`, top-level `package.json`.
- Session-start hooks: `hooks/hooks.json` (Claude Code), `hooks/hooks-cursor.json` (Cursor), `hooks/run-hook.cmd` (cross-platform polyglot wrapper), `hooks/session-start` (bootstrap injector).
- Root context files: `AGENTS.md`, `CLAUDE.md`, `GEMINI.md` — thin pointers to the bootstrap skill for harnesses that auto-discover them.

### Engineering posture

- **Three-mode principal-engineer persona** with explicit mode-switch protocol, output contracts per mode, refusal conditions, and a "Switching to <mode> mode for <skill>." transition signal. Reviewer mode is strict no-write on source code.
- **Three-tier change triage** in `intake-code-contribution`: Trivial, Standard, Major. Trivial bypasses architect mode; Standard runs a lightweight architect pass; Major runs the full phased workflow including ADR + Design Approval.
- **Mechanical Design Approval** for Major-tier changes — ADR `status: Accepted` plus a signed Design Approval block in the active sprint file. Implementer mode cannot flip ADR status to unblock itself.
- **Refactor decision matrix** in `verify-and-assemble-pr` covering 18 conditions with Block-PR / Follow-up / Inline-change-request routing.

### Tooling

- `scripts/verify.sh` — universal verification entry point copied into every project by `bootstrap-project`.
- `scripts/check-anti-dumping.sh`, `check-no-skipped-tests.sh`, `check-no-sleep-waits.sh`, `check-port-adapter-parity.sh` — enforcement scripts wired through `verify.sh`.
- `scripts/validate-plugin.sh` — plugin self-test (SKILL.md frontmatter, JSON/YAML parse, cross-references, manifest version parity).
- `scripts/bump-version.sh` + `.version-bump.json` — single-source version-bump tool with drift detection.

### Project overlay

- `provision-project-overlay` — interview-driven scaffolding of a project-specific `.github/` overlay (skills, agents, prompts, persona instructions) on top of any repo that has installed Praxis. Writes `praxis.config.yaml`, emits managed files from plugin templates, optionally bootstraps `docs/project-context.md`, `docs/product/PRODUCT.md`, and a first technology-stack ADR file. Idempotent.
