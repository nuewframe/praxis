# WAVE: Multi-Harness Reach

> **Delivered before wave adoption — a derived record, not a plan.** Reconstructed from validated truth (release history, ADRs, capability records) after the work shipped. It carries no hypothesis card and no acceptance criteria because none were written at the time; each slice cites the evidence for what it delivered. Current-state architecture lives in [docs/architecture/](../../../architecture/). Derivation rules: [ADR.260725.10](../../../architecture/adr/ADR.260725.10-brownfield-wave-retrofit.md).

**Status:** ✅ Complete (delivered before wave adoption)\
**Goal:** A team gets identical agent behavior from the same method regardless of which coding harness they use, without the method being maintained separately per harness.

---

## Tracking Rules

- Track each thin-slice by intended user outcome and current state only.
- If a thin-slice is reopened or corrected, keep the same slice ID and add one short tracking note next to that slice.
- Keep implementation history in sprint files and version control. This README stays focused on product intent.

---

## Value Theme

A method that only works in one agent runtime binds a team to that runtime. Worse, a method forked per runtime drifts, and the drift is invisible until two developers on different tools get different answers. This wave makes harness support an adapter concern: one canonical content tree, many thin manifests pointing at it.

---

## Scope

- One substantive source tree (`skills/`, `agents/`, `instructions/`) with no per-harness content forks.
- Per-harness manifest and hook adapters that point at that tree.
- A guardrail-injection path for harnesses that have no native always-on instruction mechanism.
- An overlay generator so a host project can adopt the method with its own stack and paths.

**Out of scope:**

- The content being reached — that is [wave-method-spine](../wave-method-spine/README.md).
- Placement of context in monorepos and multi-repo products, which single-tree reach does not solve — that is [wave-brownfield-adoption](../wave-brownfield-adoption/README.md).

---

## Thin-Slices

### TS-001: One method, six harnesses, no content forks

> **Status:** ✅ Complete

**User Value:** As a team using mixed tooling, I need every developer's agent to load the same method so that behavior does not depend on which editor someone opened.

**Evidence:** Native installation into Claude Code, Codex (CLI and App), Cursor, Gemini CLI, OpenCode, and GitHub Copilot (CLI and VS Code) from one source tree, via `.claude-plugin/plugin.json`, `.codex-plugin/plugin.json`, `.cursor-plugin/plugin.json`, `gemini-extension.json`, `.opencode/plugins/praxis.js`, and `package.json`. Shipped in the [initial public release](../../../../CHANGELOG.md#010--2026-05-20). Capability record: [docs/architecture/distribution/README.md](../../../architecture/distribution/README.md), which states the invariant plainly — there are no per-harness content forks, only manifest and hook adapters.

---

### TS-002: Always-on rules survive where the harness has no mechanism for them

> **Status:** ✅ Complete

**User Value:** As a team on a harness with no `applyTo` support, I need the always-on constraints to still be always on, rather than silently degrading to advice the agent saw once.

**Evidence:** `hooks/session-start` reads the bootstrap skill in full and injects it as session context, firing on startup, `/clear`, and context compaction — the three moments the guardrail summary would otherwise fall out of context. Cross-platform wiring via `hooks/hooks.json`, `hooks/hooks-cursor.json`, and the `hooks/run-hook.cmd` polyglot wrapper. Root pointer files (`AGENTS.md`, `GEMINI.md`, `.claude/CLAUDE.md`) cover harnesses that auto-discover instead.

**Tracking note:** A harness-load defect surfaced after release — a duplicate `hooks` declaration in the Claude Code manifest broke plugin load, since Claude Code auto-loads `hooks/hooks.json` from the plugin root and re-declaring it conflicts. Corrected in a [patch release](../../../../CHANGELOG.md#013--2026-05-29).

---

### TS-003: A host project adopts the method with its own stack

> **Status:** ✅ Complete

**User Value:** As a team with an existing repo, I need the method's paths, personas, and quality gates to match my project rather than assuming Praxis's own layout.

**Evidence:** `skills/provision-project-overlay/SKILL.md` interviews for stack, paths, persona aliases, and gates, writes `praxis.config.yaml`, and emits a managed overlay from templates — idempotently, showing diffs before overwriting human-edited files. `skills/bootstrap-project/SKILL.md` covers the greenfield case. ADR ID conventions were unified across both so a generated first ADR follows the same rules as every later one ([patch release](../../../../CHANGELOG.md#012--2026-05-28)).

---

## Success Criteria

Delivered. Six harnesses install from one tree, the always-on surface survives on harnesses that lack a native mechanism, and a host project can adopt the method against its own paths.

The reach is real but its *verification* is thin: harness manifests are held at version parity mechanically, while the claim that every harness loads equivalent content rests on the single-tree invariant rather than on a per-harness test.

---

## Dependencies

- **Requires:** [wave-method-spine](../wave-method-spine/README.md) — there must be a method before there is anything to distribute.
- **Enables:** every adopter-facing capability. Enforcement that only ran in one harness would not be a gate.
