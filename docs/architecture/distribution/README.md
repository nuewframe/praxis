# Distribution — Capability Record

`hooks/`, the per-harness manifests (`.claude-plugin/`, `.codex-plugin/`, `.cursor-plugin/`, `.opencode/`, `gemini-extension.json`), and `skills/provision-project-overlay/` together are how Praxis reaches a host project across six harnesses — Claude Code, Codex CLI/App, Cursor, Gemini CLI, OpenCode, and GitHub Copilot CLI/VS Code — from one single-source `skills/` / `agents/` / `instructions/` tree. There are no per-harness content forks, only per-harness manifest and hook adapters pointing at the same substantive files.

Multi-harness reach is a deliberate distribution goal stated in `project-context.md`'s Identity section, not a hedge requiring adoption evidence — that evidentiary bar applies to methodology-fidelity claims, not to how many harnesses can install the same doctrine. Reach is proven on Claude Code, the harness this repo is built and tested against, and remains aspirational on the others until validated.

## Session-start injection

`hooks/session-start` reads `skills/using-praxis/SKILL.md` in full and injects it as session context on every harness that lacks a native `applyTo` mechanism. On a harness like Claude Code, this injection is the *only* always-on guardrail surface: the matcher fires on startup, `/clear`, and context compaction by design, because those are exactly the moments the always-on guardrail summary would otherwise silently fall out of the model's context.

## The overlay path: `provision-project-overlay`

`bootstrap-project` scaffolds a brand-new repository. `provision-project-overlay/SKILL.md` is the interview-driven overlay generator for an *existing* host repo: it writes `praxis.config.yaml`, renders roughly 40 manifest-driven overlay files with `{{placeholder}}` substitution, and — in a step the SKILL.md itself calls out as easy to silently break (`exit 127` if skipped) — copies every `check-*.sh` script **verbatim, via a glob** (`cp <plugin-root>/scripts/check-*.sh scripts/`) into the host repo's `scripts/`, since those scripts are not templates and are not listed in `manifest.yaml`.

This glob-copy mechanism is precisely why the enforcement capability's newest scripts (`check-escape-hatch-usage.sh`, `check-design-approval-gate.sh`) reach a host project automatically, with zero manifest change required — a new `check-*.sh` file in the plugin's `scripts/` directory is picked up the next time a host project runs `provision-project-overlay` or `--upgrade`, without anyone touching `manifest.yaml`.

Persona-alias support (`personas.use_aliases`) and the prompt-overlay command templates (11 files under `.github/prompts/*.prompt.md`) both ship as-is. The latter is explicitly self-described in `provision-project-overlay/SKILL.md`'s own Concepts section as "unvalidated speculative generality... with no evidence any adopter uses them" — a genuinely candid admission preserved verbatim here rather than summarized away.

## `verify.sh` — the generated single entry point

The generated `scripts/verify.sh` (rendered from `templates/scripts/verify.sh.tmpl`) is the single entry point chaining all 15 pipeline steps in a fixed, documented order: format → lint → typecheck → anti-dumping → no-skipped → no-sleeps → port-parity → seam-parity → config-extern → observability → statelessness → resilience → sprint-ids → design-approval → tests.

Two git hooks consume it in a provisioned repo:

- `.githooks/pre-push` runs the whole chain before every push.
- `.githooks/pre-commit` runs the fast subset — no tests — before every commit.

This is where [ADR.260720.01](../adr/ADR.260720.01-design-approval-git-hook-gate.md)'s Design Approval gate actually lands once a host project generates its hooks: the gate itself (`scripts/check-design-approval-gate.sh`) is enforcement-capability tooling, but it protects nothing until a host repo runs `provision-project-overlay`, gets `verify.sh` and the `.githooks/` wiring, and installs the pre-push hook (`git config core.hooksPath .githooks`). No ADR is scoped to this capability directly — its role here is carrying an enforcement-capability decision across the reach this capability provides.
