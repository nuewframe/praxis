This tree is current-state truth, promoted by `close-sprint`. Planning-stage intent lives in `docs/product/waves/`.

## Identity

Praxis is a portable agent plugin that fuses lean wave-based product delivery with Principal Engineer discipline, distributed across six harnesses from one single-source tree. For the plugin's identity, the trust-transfer problem it exists to close, and the scope-rule doctrine that governs what belongs here, see [`../product.md`](../product.md) — this overview does not restate that content; it states the engineering-truth picture and points at the three capability records below for the current-state detail behind each capability.

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
| [ADR.260724: Wave naming imposes no category taxonomy](adr/ADR.260724-wave-category-relaxation.md) | Homed in the `skills` capability (`create-wave` loses its category mandate), but its second half amends the plugin's evolution policy to classify rule *relaxation* as distinct from removal — a governance rule that binds every capability's future changes. |
| [ADR.260725: Declared exceptions move inline](adr/ADR.260725-inline-declared-exceptions.md) | Homed in `enforcement`, but changes how every capability's documents declare a sanctioned literal: structural citation detection plus inline reasoned markers replace path allowlists. **Status: Accepted** — implemented by `TS-008` of wave-self-conformance. |
| [ADR.260725.10: Retrofitting waves onto an existing product](adr/ADR.260725.10-brownfield-wave-retrofit.md) | Separates deriving an intent map from validated truth (legitimate) from fabricating history (forbidden), and establishes the two-tier wave form — README-only with cited evidence for delivered work, full four documents for open work. Closes the brownfield adoption gap `bootstrap-project` and `refactor-layered-to-capability` both leave open. **Status: Accepted** — already applied to Praxis's own tree; generalized into an adoption path by `TS-001` of wave-brownfield-adoption. |

## Current posture

All harness manifests are held at version parity with `package.json` by `bump-version.sh`; `bump-version.sh --check` reports the current number, and this document deliberately does not restate it. `validate-plugin.sh` runs 15 checks. Two generators (`gen-coverage-matrix.sh`, `gen-tier-table.sh`) are CI-enforced via `--check`. One enforcement gate, `check-design-approval-gate.sh`, hard-fails without any orchestration runtime — the rest of the plugin's mechanical gates are script-checkable but rest on a project actually wiring `verify.sh` into CI or a git hook to fail closed.

**Known gap, stated plainly:** `.github/workflows/ci.yml` executes exactly one of the twelve `check-*.sh` scripts against this repo (`check-anti-dumping.sh`). The other eleven are passed to `bash -n` — a syntax check that proves they parse, not that they pass. `check-design-approval-gate.sh` has therefore never run in Praxis's own CI, and fails when run manually against the current tree. Three of the eleven are directly applicable to a repo with no runtime code (design-approval, sprint-id-collision, escape-hatch-usage); the remaining seven target host-repo request paths and seams Praxis does not have, and should assert a reasoned `n/a` rather than be silently absent. Closing this is `TS-005` of [wave-self-conformance](../product/waves/wave-self-conformance/README.md).
