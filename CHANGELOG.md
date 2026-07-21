# Changelog

All notable changes to the Praxis plugin are documented here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this plugin uses semantic versioning (see [project-context.md](project-context.md) for the policy).

## [Unreleased]

## [0.4.0] — 2026-07-20

### Note on evidence

This release ships the course-correction toward the plugin's own sharpened Problem statement (trust transfer / execution fidelity). The plugin's own evolution policy (see `project-context.md` § Evolution policy) says a minor bump's CHANGELOG entry "must cite the real-repo evidence... a bump that can only cite dogfooding does not qualify." That evidence does not yet exist — real-repo validation (running a full wave → sprint → close-sprint cycle at `profile: full` against an actual project) is the immediate next step after this release, not a precondition already met. Stated here plainly rather than silently omitted: everything below is verified by self-test and independent reconciliation against each item's stated Done criterion, not by real-world use yet.

### Added

- **`## Problem` section in `project-context.md`** — states the plugin's actual problem plainly: trust transfer. An agent-generated delivery artifact looks the same whether the agent reasoned hard or pattern-matched a template; Praxis's job is closing that gap, and its primary output is **execution fidelity** — the agent demonstrably did the disciplined work, and where it didn't is visible.
- **`## The method at a glance` section in `project-context.md`** — the ordered PLAN → TRIAGE → BUILD → LEARN → TEACH spine, Trivial/Standard/Major tier branching, and a stage-by-stage "where fidelity is made" table, so the high-level flow is legible from the constitution alone instead of only from the file tree.
- **4th scope-litmus question** — "Does it measurably improve the agent's execution fidelity, or close a known agent failure mode?" A rule can be universal, disciplined, and defensible and still not belong in the plugin if it fails this question; universality is necessary but not sufficient.
- **Strengthened evolution policy** — real-repo validation is now the non-negotiable step before a minor bump; dogfooding the plugin on itself no longer substitutes, and a bump's CHANGELOG entry must cite real-repo evidence (see the Note above — this release itself does not yet meet that bar, by design, and says so).
- **`README.md` § "Enforcement, honestly"** — discloses the script-enforced / human-signed / agent-attested gate split in the opening pitch, guarded by a new `.praxis-canon.json` `requiredPhrases` lint (the inverse of `forbiddenTerms`) so the disclosure can't silently regress. `scripts/validate-plugin.sh` gained a 12th check enforcing it.
- **`scripts/check-design-approval-gate.sh`** — the plugin's first gate that is genuinely hard-fail with no warn mode and no opt-out. For every Major-tier sprint, verifies the referenced ADR's status is `Accepted` and the sprint's Design Approval block is genuinely signed, not template placeholders. Wired into the host-repo `verify.sh` template and therefore the optional pre-push git hook — the first Praxis gate demonstrably failing closed without an orchestration runtime. Self-tested against 5 scenarios.
- **`scripts/data/tier-classification.json` + `scripts/gen-tier-table.sh`** — single source of truth for the Trivial/Standard/Major tier facts, rendered into marker-wrapped generated blocks across `intake-code-contribution`, `start-thin-slice`, and `principal-engineer.agent.md`, so the same fact restated three ways cannot silently diverge. `--check` runs in CI. Drift detection was independently proven twice before this release.
- **`verify-and-assemble-pr` Step 6 "Artifact-fidelity review"** — a separate-head rubric grading whether an ADR's alternatives, a Design Approval signature, an ambiguity log, a hypothesis card, or a risk-register entry carries real substance or is boilerplate. A warn-signal for human judgment, not a hard gate — it does not certify decision correctness, only reasoning substance.
- **`verify-and-assemble-pr` Step 7 "Trust Receipt"** (renumbered from Step 6) — aggregates gate-kind status (script-enforced / human-signed / agent-attested), escape-hatch usage, and Step 6's verdicts into one human-readable block per PR — the first concrete artifact answering the plugin's own trust-transfer problem statement directly.
- **`scripts/check-escape-hatch-usage.sh`** — diff-scoped scanner for the four `praxis:allow-*` escape-hatch markers, reporting exact file:line. Informational only, always exits 0 — the point is that using an opt-out is never silent to a reviewer, not blocking it.
- **`docs/architecture/`** — the plugin's own capability record, created for the first time, per its own `close-sprint` doctrine that "the capability record is the truth." A system overview, three capability records (`skills/`, `enforcement/`, `distribution/` — `agents/` and `instructions/` folded into the overview as too small/stable to warrant their own record), and three ADRs for the durable decisions in this release: the Design Approval gate, the generated tier table, and the fidelity-review/trust-receipt mechanism.

### Removed

- **The `lite`/`standard` adoption-profile dial** — deleted, not deprecated. It never shipped in a tagged release (confirmed via `git merge-base` against `v0.3.0`) and was never wired into `provision-project-overlay/manifest.yaml`'s conditional logic, so no adopter depends on it and the standard one-minor-version deprecation notice does not apply. `full` is the only supported profile. Two `.praxis-canon.json` `forbiddenTerms` entries guard against silent reintroduction. Multi-harness distribution and persona-alias support are unaffected — both remain deliberate, undiminished goals, not weight that was cut alongside the profile dial.

### Changed

- **`docs/plans/` pruned** — four fully-implemented plans removed after verifying no still-open commitment remained in any of them (`major-path-ordering-and-deadlock.md`, `praxis-maturity-roadmap.md`, `teach-phase-user-docs.md`, `thin-slice-loop-hardening.md`); their content is now either shipped code, `docs/architecture/`, or superseded by `executable-seams-first.md`, which is kept because its D1/D3 primitives remain deliberately deferred to a specific, not-yet-reached trigger (first real concurrent slice dispatch).

## [0.3.0] — 2026-07-18

### Added

- **Inventory-parity check in `validate-plugin.sh`** (check #7) — fails when any `skills/<name>/`, `scripts/*.sh`, or `instructions/*.instructions.md` on disk is not referenced in the canonical self-describing docs (README.md, project-context.md, and — for instructions — `using-praxis`). Makes documentation drift behind the file tree a build-time failure rather than a silent gap.

### Fixed

- **Documentation drift across canonical docs** — `project-context.md` status now tracks the shipped manifest version; README's always-on-guardrails table now lists all three instruction sets (adds `code-contribution-intake`); README skills tables + the compose diagram now include `start-thin-slice` and `provision-project-overlay`; `project-context.md` architecture tree now lists all 21 skills and all 12 scripts; `using-praxis` guardrail count corrected from "two" to "three".
- **Anti-dumping policy parity** — the linter's forbidden-name set (`.anti-dumping.json` + the `check-anti-dumping.sh` suggested config) now includes `lib` and `handlers`, matching the documented capability-driven guardrail (`lib.*` and the `handlers/` silo were previously documented as forbidden but not enforced).
- **Stale `docs/adr/**` glob** removed from the lean-delivery guardrail `applyTo` (and its `using-praxis` mirror); ADRs live under `docs/architecture/**`.
- **`LICENSE` file added** (MIT) — previously referenced by `package.json` and README but absent from the tree.

## [0.2.0] — 2026-06-05

### Added

- **Bundle D enforcement hardening (decision D6, post-ship sparring)** — splits Bundle D's three primitives by *enforceability* and acts on each honestly (plan: `docs/plans/executable-seams-first.md`, §7 + decision D6).
  - **`check-sprint-id-collision.sh`** (D2, now executable) — exact, not heuristic: fails when two active (non-ledger) sprint files share an id token (`NNN` or ADR-style `NNN.<seq>` from `sprint-<id>-<desc>.md`). Warn-first via `.sprint-coordination.json` (`mode`, `sprintDir`), promote to `enforce` once parallel sprint creation is routine; skips cleanly when no sprint dir/files exist. Wired into `create-sprint`, `verify.sh`, and covered by `validate-plugin.sh` (decision D5).
  - **D1/D3 designed but deferred** — disjointness and contract-freshness are mechanizable only once a machine-readable **sprint footprint** (touched capabilities/file-globs, persistent resources, config keys, depended-on `<name>@vN`) is declared; building that artifact + `check-sprint-disjointness.sh` + `check-contract-freshness.sh` before any real concurrent run is speculative generality. **Named build trigger:** the first time two slices are dispatched concurrently. Until then they remain prose discipline in the guardrails and intake gate.
  - **Bundle C deliberately left as prose** — no trace/existence gate is added for adversarial seam review: a metric proxying for judgment manufactures false confidence (Goodhart). C's only honest enforcement is a different head rejecting the PR; the ledger records *which head* (D4), never *certifies the verdict*.
- **Emergent-parallelism safety primitives (Bundle D)** — the smallest, last bundle: parallelism stays *emergent* (never scheduled by Praxis), but becomes **safe** when the human or an orchestration runtime exercises it (plan: `docs/plans/executable-seams-first.md`). No scheduler — only the safety rails.
  - **D1 — three-axis disjointness rule** in `using-praxis` and the capability-driven guardrails: two units may be built concurrently only if disjoint across **capability/files**, **persistent resources** (tables, topics, queues, caches, migrations), and **shared config keys**, *and* each depends only on a frozen `<name>@vN` contract — never on the other's in-flight internals. Capability-disjointness alone is explicitly called out as insufficient.
  - **D2 — collision-safe coordination artifacts** in `create-sprint`: sprint ids gain ADR-style collision handling (next free `NNN`, then shortest unique `.<seq>` tiebreaker; existing ids immutable), and live progress moves to per-slice surfaces (sprint header `Status` + ledger) reconciled into the shared wave README table at `close-sprint` — so N concurrent slices stop racing the coordination layer.
  - **D3 — snapshot staleness re-anchor gate** in `intake-code-contribution`: if a sprint sat queued while siblings merged, re-check whether any depended-on `<name>@vN` contract or current-state fact moved since the bridge froze; re-anchor before coding (or close and recreate if a contract changed incompatibly). The price of parallelism touching the immutable bridge — paid as a check, never by editing frozen scope in place.
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

- `scripts/verify.sh` — universal verification entry point generated into each project from the plugin's `verify.sh.tmpl` template by `bootstrap-project` (the plugin ships the template, not a root `scripts/verify.sh`).
- `scripts/check-anti-dumping.sh`, `check-no-skipped-tests.sh`, `check-no-sleep-waits.sh`, `check-port-adapter-parity.sh` — enforcement scripts wired through `verify.sh`.
- `scripts/validate-plugin.sh` — plugin self-test (SKILL.md frontmatter, JSON/YAML parse, cross-references, manifest version parity).
- `scripts/bump-version.sh` + `.version-bump.json` — single-source version-bump tool with drift detection.

### Project overlay

- `provision-project-overlay` — interview-driven scaffolding of a project-specific `.github/` overlay (skills, agents, prompts, persona instructions) on top of any repo that has installed Praxis. Writes `praxis.config.yaml`, emits managed files from plugin templates, optionally bootstraps `docs/project-context.md`, `docs/product/PRODUCT.md`, and a first technology-stack ADR file. Idempotent.
