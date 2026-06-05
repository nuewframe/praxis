# Changelog

All notable changes to the Praxis plugin are documented here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this plugin uses semantic versioning (see [project-context.md](project-context.md) for the policy).

## [Unreleased]

### Added

- **Adversarial seam review (Bundle C)** — turns seam *behavior* from self-attested to independently proven, the deliberate speed-for-quality trade (plan: `docs/plans/executable-seams-first.md`). GenAI supplies speed; this bundle spends some of it on assurance the model cannot fake.
  - **C1 — adversarial seam-behavior review** in `verify-and-assemble-pr` (new Step 5): beyond the structural refactor matrix, the reviewer attacks behavior at every touched `<name>@vN` seam — demands the test that proves the circuit opens mid-call, the handler is idempotent under retry, the correlation id crosses the seam, concurrent ops are linearizable. Run by a **different head** (separate session/agent default; same-agent reviewer-mode switch with a fresh diff read as fallback), with the path used recorded in the progress ledger. A seam whose property is asserted only in prose bounces back. Added to the PR narrative and anti-patterns; ledger template gained an "Adversarial Seam Review" block.
  - **C2 — property over example at high-risk seams** in `create-sprint`'s AC↔test matrix (new Evidence column) and `test-by-ownership` (new "Property over example at high-risk seams" section): a high-risk (Impact = High) AC that exercises a seam must map to a **property/contract test** (idempotency ∀ keys, retry-safe ∀ attempts, concurrent ops linearizable), not a single example. Sharpens the matrix; the adversarial review (C1) enforces it.
- **Remaining production-readiness anchor probes (Bundle B3)** — three executable probes that convert the Observable, Horizontally-scalable, and Resilient anchors from asserted checklist lines into build-time gates (plan: `docs/plans/executable-seams-first.md`). Each mirrors `check-config-externalized.sh`: warn-first with mechanical promotion to `enforce` (decision D3), reviewed opt-out, `scanPaths` falling back to `.anti-dumping.json` then defaults, bash 3.2 compatible; wired through `verify.sh` and covered by `validate-plugin.sh` (decision D5).
  - **`check-observability-at-seams.sh`** (Observable) — flags a file that crosses a process boundary (HTTP/RPC/queue/DB client) but carries no log/metric/trace/correlation-id. Warn-first via `.observability.json`; per-file opt-out via `praxis:allow-unobserved-boundary`.
  - **`check-stateless-request-path.sh`** (Horizontally scalable) — flags node-local mutable state on the request path (a module-level or static cache/session/registry singleton). Warn-first via `.statelessness.json`; per-line opt-out via `praxis:allow-local-state`.
  - **`check-resilient-boundary.sh`** (Resilient) — flags a boundary call that declares no timeout/retry/circuit-breaker/fallback. Warn-first via `.resilience.json`; per-file opt-out via `praxis:allow-unguarded-boundary`.
  - **`verify-and-assemble-pr`** now runs all four anchor probes in its verify entry point and lists them in the PR narrative checklist.
- **Production-Readiness posture spine (Bundle B, B1/B2)** — the four runtime anchors (observable, configurable, horizontally scalable, resilient) decided once at the wave level and conformed to per slice, instead of re-decided in isolation (plan: `docs/plans/executable-seams-first.md`).
  - **Wave posture in `create-product-architecture-spec`** — a new "Declare the Production-Readiness Posture" step + posture table where the wave states, per anchor, the correlation/observability contract, config & secrets strategy, statelessness boundary, and cross-slice failure model. Each anchor names its enforcing probe.
  - **Per-slice conformance in `create-sprint`** — a "Production-Readiness Conformance" block that names the seam(s) the slice touches and confirms it *preserves* the wave posture per anchor (conforms / reviewed deviation), replacing per-slice re-litigation and `N/A` theater.
  - **Intake gate** — `intake-code-contribution` now stops Standard/Major work whose conformance block is blank or leaves an anchor unaddressed; the slice must declare how it preserves the wave posture before coding.
- **Seam Contract construct (Bundle A keystone)** — the machine-checkable description of a boundary (plan: `docs/plans/executable-seams-first.md`). Introduces three coordinated pieces:
  - **`define-seam-contract` skill** (architect mode) — given a boundary, produces a machine-readable **Shape** (OpenAPI for http, JSON-Schema for event, native typed `*.ports.*` for port; project-overridable per decision D2), registers the shared **Behavior** contract test suite, and assigns a frozen **`<name>@vN`** id in `.seam-contracts.json`. The frozen version lets a dependent slice build against a promise instead of waiting for the producer's internals.
  - **`check-seam-contract-parity.sh` enforcement script** — generalizes the Port/Adapter parity gate from Ports to every declared seam: each seam in `.seam-contracts.json` must have a Shape and a Behavior suite on disk. Warn-first with mechanical promotion to `enforce` (decision D3); skips cleanly when no manifest exists. Wired through `verify.sh` and covered by `validate-plugin.sh` (decision D5).
  - **Seam declaration in `create-product-architecture-spec`** — the wave now names its seams (id, kind, producer, consumer, Shape, Behavior suite) before slices fork, and `verify-and-assemble-pr` Step 3 + refactor matrix enforce that every touched seam's shared suite ran against both sides (consumer-driven).
- **`check-config-externalized.sh` enforcement script** — the first executable seam-conformance probe (plan: `docs/plans/executable-seams-first.md`, decision D1). Converts the "Configurable" production-readiness anchor from an asserted checklist line into a build-time gate: warns (or, once promoted, fails) on hardcoded remote URLs, endpoints, or secret literals in source. Warn-first by default with mechanical promotion to `enforce` via `.config-externalization.json` (decision D3); per-line reviewed opt-out via a `praxis:allow-config-literal` comment. Wired through `verify.sh` and covered by `validate-plugin.sh` (decision D5).
- **`start-thin-slice` skill** — front-door triage and routing for slice work ("Work on TS-NNN"). Checks dependency/status preconditions, runs a provisional tier classification by reference to `intake-code-contribution` Step 0 (single source of truth, no forked table), runs a lightweight ambiguity log + pre-mortem for Standard work, and deterministically routes to `create-sprint` vs. the architect path.
- **Sprint Plan Approval gate** in `create-sprint` — a signed checkpoint for Standard- and Major-tier sprints (distinct from the Major-only Design Approval). `intake-code-contribution` refuses to pass to implementation until it is signed; Trivial writes `n/a`.
- **Resilience / failure-mode checklist** in the `create-sprint` implementation plan — idempotency, concurrency, offline/degraded dependency, version pinning, partial-failure recovery. Defines the "production-grade plan" bar.
- **Acceptance ↔ test traceability matrix** in `create-sprint` — every acceptance criterion maps to ≥1 test. Checked at intake (every AC mapped) and at verify (every mapped test actually ran).
- **Risk register (pre-mortem seed)** in `create-sprint`, seeded from `start-thin-slice` (Standard) or `discovery-and-ambiguity-log` (Major).
- **Progress ledger** (`sprint-NNN-*.ledger.md`) — a mutable, session-surviving execution-state file alongside the immutable sprint bridge. Created by `create-sprint`, restored by `intake-code-contribution` on resume, deleted by `close-sprint` after distillation.
- **Debugging-loop budget** in `verify-and-assemble-pr` — a self-applied stop rule (default 3 consecutive failed verifies on the same cause → halt + escalate) with an explicit FAIL-vs-BLOCKED determination before each retry. Shipped as discipline, not runtime enforcement, per Praxis's stated boundary.

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
