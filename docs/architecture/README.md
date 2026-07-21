This tree is current-state truth. Planning-stage intent lives in `docs/plans/`.

## Identity

Praxis is a portable agent plugin that fuses lean wave-based product delivery with Principal Engineer discipline, distributed across six harnesses from one single-source tree. For the plugin's identity, the trust-transfer problem it exists to close, and the scope-rule doctrine that governs what belongs here, see [`../../project-context.md`](../../project-context.md) — this overview does not restate that content; it states the engineering-truth picture and points at the three capability records below for the current-state detail behind each capability.

## The three capabilities

### Skills

The ordered delivery-plus-engineering pipeline — PLAN → TRIAGE → tier branch (Trivial / Standard / Major) → BUILD → LEARN → TEACH — that carries a unit of work from wave intent through a sprint bridge to a reviewed, closed PR. It hosts the `verify-and-assemble-pr` review chain (Pyramid Test Strategy through the artifact-fidelity review and Trust Receipt) and both sprint approval gates (Sprint Plan Approval, Design Approval).

→ [skills/README.md](skills/README.md)

### Enforcement

The generic, project-agnostic `scripts/` tooling: 11 `check-*.sh` probes bucketed into three enforcement postures (warn-first mode-promotable, hard-fail with no warn mode, informational never-fails), two generators that keep documentation honest against the file tree (`gen-coverage-matrix.sh`, `gen-tier-table.sh`), and `validate-plugin.sh`, the plugin's own 12-check self-test.

→ [`enforcement/README.md`](enforcement/README.md)

### Distribution

How Praxis actually reaches a host project across six harnesses — Claude Code, Codex CLI/App, Cursor, Gemini CLI, OpenCode, and GitHub Copilot CLI/VS Code — from one single-source tree: the session-start injection pattern, the `provision-project-overlay` interview and glob-copy mechanism, the generated 15-step `verify.sh`, and the git hooks that consume it.

→ [`distribution/README.md`](distribution/README.md)

## Agents and instructions

The 3 personas in `agents/` (`principal-engineer` — three modes, architect/implementer/reviewer, tool surface governed by mode rather than by separate agent instances; `product-manager` and `product-designer`) and the 3 always-on guardrail sets in `instructions/` (`capability-driven-guardrails`, `lean-delivery-guardrails`, `code-contribution-intake`) do not get their own capability record: they are small and stable enough — a handful of files, low change-rate — that forcing them into a fourth top-level capability-record home would be ceremony disproportionate to their size, the same 4th-litmus-question discipline the plugin applies to everything else it might add. Read the files directly: `agents/principal-engineer.agent.md`, `agents/product-manager.agent.md`, `agents/product-designer.agent.md`, and the three `instructions/*.instructions.md` files.

## ADR index (cross-capability)

These ADRs genuinely cross more than one capability boundary; each is indexed here as well as in the capability record(s) it is homed under.

| ADR | Purpose |
| --- | --- |
| [ADR.260720.01: Design Approval git pre-push hook gate](adr/ADR.260720.01-design-approval-git-hook-gate.md) | Builds `check-design-approval-gate.sh`, the one gate in this repo that is hard-fail with no opt-out by design — the first gate Praxis demonstrably fails closed without an orchestration runtime, reaching a host project through the distribution capability's `verify.sh` and git hooks. |
| [ADR.260720.02: Single-source-of-truth generated tier-classification table](adr/ADR.260720.02-generated-tier-table.md) | Generates the tier-classification table into three skill/agent surfaces from one JSON source, using the enforcement capability's `gen-coverage-matrix.sh` generator pattern as precedent. |
| [ADR.260720.03: Artifact-fidelity review and the Trust Receipt](adr/ADR.260720.03-fidelity-review-and-trust-receipt.md) | Adds the artifact-fidelity review and Trust Receipt to `verify-and-assemble-pr`, closing the gap shape-checking probes cannot: whether an artifact's reasoning has substance, sourcing escape-hatch facts from the enforcement capability's `check-escape-hatch-usage.sh`. |

## Current posture

Version `0.3.0` across all manifests (`.claude-plugin/plugin.json`, `.codex-plugin/plugin.json`, `.cursor-plugin/plugin.json`, `package.json`), confirmed in parity. `validate-plugin.sh` runs 12 checks. Two generators (`gen-coverage-matrix.sh`, `gen-tier-table.sh`) are CI-enforced via `--check`. One enforcement gate, `check-design-approval-gate.sh`, hard-fails without any orchestration runtime — the rest of the plugin's mechanical gates are script-checkable but rest on a project actually wiring `verify.sh` into CI or a git hook to fail closed. `CHANGELOG.md`'s `[Unreleased]` section does not yet reflect the three ADRs above.
