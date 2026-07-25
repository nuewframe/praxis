# Praxis — Project Context

This is the single entry point for understanding **what this method is, why it exists, what belongs in it, and how it evolves.** Read it before adding, removing, or modifying any plugin file.

Praxis is one opinionated software-development skill: a single ordered method for carrying a unit of work from intent to shipped, verified, and closed — fusing **lean wave-based product delivery** with **Principal Engineer discipline**.

Everything else in this repository is that method decomposed. The sub-skills under [`skills/`](../skills/) are its steps, the personas under [`agents/`](../agents/) are the roles it assigns, the always-on rule sets under [`instructions/`](../instructions/) are the constraints it never relaxes, and the probes under [`scripts/`](../scripts/) are the parts of it a machine can check without trusting anyone.

The doctrine is universal — language-, framework-, and runtime-agnostic. The **static enforcement is best-effort per language** ([`docs/coverage-matrix.md`](coverage-matrix.md)).

> **This file is the narrative and the governance.** It is not a source of truth for any table it summarizes. The authoritative ordered path per tier is [`start-thin-slice`](../skills/start-thin-slice/SKILL.md) Step 5; the authoritative tier criteria are [`intake-code-contribution`](../skills/intake-code-contribution/SKILL.md) Step 0 (both generated from [`scripts/data/tier-classification.json`](../scripts/data/tier-classification.json)). The session-time operational index is [`skills/using-praxis/SKILL.md`](../skills/using-praxis/SKILL.md). Where this file and those disagree, **they win**.

**Sections 1–9 are the method** (read these to *use* Praxis). **Sections 10–11 are the plugin** (read these to *change* Praxis).

---

## 1. Identity

**Name:** `praxis`

**Purpose:** Make an LLM coding agent execute disciplined, lean wave-based delivery and Principal-Software-Engineer practice with **fidelity** — producing trustworthy artifacts a human team can build on — instead of improvising.

**Reach:** installable, versioned guidance for Claude Code, Codex, Cursor, Gemini CLI, OpenCode, and GitHub Copilot, written to apply to any language, framework, or runtime. Wide availability across harnesses is a deliberate distribution goal, not a hedge to be earned into with usage evidence — that evidentiary bar applies to methodology-fidelity claims (§2), not to how many harnesses can install the same doctrine. Breadth is how widely the value ships; it is not the value itself.

**Status:** multi-harness installable plugin. The version is declared once, in `package.json`, and synced to every harness manifest by [`bump-version.sh`](../scripts/bump-version.sh) — no document states it in prose, and `--audit` fails the build if one starts to. See [`CHANGELOG.md`](../CHANGELOG.md) for what shipped when.

---

## 2. Why the method exists

GenAI coding agents, left unconstrained, default to improvising structure, skipping discovery, drifting scope, and producing plausible-looking artifacts with no substance behind them. Human engineering teams already have the discipline that prevents this — waves, ADRs, thin slices, test-by-ownership, a review firewall between the engineer who builds and the one who approves. That discipline is not in question, and it is not what Praxis invents.

The problem is **trust transfer**. When a human engineer produces an architecture document, reviewers trust it reflects real reasoning. When an agent produces the identical document, that trust is unearned — the artifact looks the same whether the agent reasoned hard or pattern-matched a template. Agents now generate delivery artifacts at scale, so the gap between *looks like discipline happened* and *discipline happened* is newly acute and newly expensive.

Praxis exists to close that gap: to make agent-generated delivery artifacts trustworthy enough that a human team can build on them without re-deriving the reasoning. Its primary output is therefore **execution fidelity** — the agent demonstrably did the disciplined work, and the places it did not are **visible rather than hidden**. Every stage emits a script-checkable artifact for exactly this reason: so a reviewer can *see* that the work happened rather than take it on faith.

Legibility — making the discipline explicit enough for an agent to follow it intentionally — is Praxis's half of that. Runtime enforcement is a separate layer (§9).

The corollary is the method's hardest rule: **never claim a gate is stronger than it is.** False trust is worse than no trust, and it is the one failure the trust-transfer problem cannot tolerate.

---

## 3. What the method is opinionated about

Ten positions. Each is enforced somewhere downstream — by a skill, a guardrail, or a probe.

### 3.1 Code is organized by business capability, never by technical layer

No `controllers/`, `models/`, `services/`, `views/`, `handlers/` silos. A capability is a vertical slice that owns everything needed to deliver a discrete piece of business value. Cross-capability calls go through an explicit public surface — no deep imports.

**Anti-dumping is absolute:** `utils/`, `helpers/`, `common/`, `shared/`, `misc.*`, `lib.*` are forbidden inside capability roots. Name the actual capability, or duplicate. *(Applied recursively — this plugin obeys it too.)*

→ [`capability-driven-guardrails`](../instructions/capability-driven-guardrails.instructions.md), [`design-capability-layout`](../skills/design-capability-layout/SKILL.md), [`check-anti-dumping.sh`](../scripts/check-anti-dumping.sh)

### 3.2 Functional core, imperative shell

Pure logic in one file, I/O wrapper in another. This is what makes the base of the test pyramid cheap and the boundaries explicit enough to instrument.

→ [`design-capability-layout`](../skills/design-capability-layout/SKILL.md), [`implement-with-defensive-patterns`](../skills/implement-with-defensive-patterns/SKILL.md)

### 3.3 Waves are intent — not sprints with bigger scope

A wave is a coherent slice of product value tracked through thin-slices. Waves outlive sprints and survive reorganization. Each wave carries four documents: `README.md` (intent + thin-slice tracking), `product-design.md`, `product-architecture.md`, `qa.md`.

Thin-slices are atomic user outcomes with **stable IDs** (`TS-NNN`). A correction to a completed slice keeps its original ID — it never gets a fresh one.

→ [`create-wave`](../skills/create-wave/SKILL.md), [`lean-delivery-guardrails`](../instructions/lean-delivery-guardrails.instructions.md)

### 3.4 Wave = educated theory; capability record = truth

A wave's `product-architecture.md` is the **best educated theory** of the proposed design — the strongest reasoned proposal at planning time, validated only downstream. It is never "the settled answer," and the vocabulary matters: this method says *educated theory*, not *bet*.

The living, validated architecture lives in the durable tree — `docs/architecture/README.md` (system overview) and `docs/architecture/<capability>/README.md` (capability records). Wave docs point into those records; they never duplicate current-state topology. `close-sprint` is what promotes theory into truth.

→ [`create-product-architecture-spec`](../skills/create-product-architecture-spec/SKILL.md), [`design-system-architecture`](../skills/design-system-architecture/SKILL.md)

### 3.5 A sprint is an immutable bridge, and it is ephemeral

A sprint locks a thin-slice against the codebase, toolchain, and integration reality *at the moment work begins*. Scope is immutable once started — to change scope you close the sprint and create a new one. When it closes, the sprint file is **deleted**: the bridge dissolves once both shores have been updated.

→ [`create-sprint`](../skills/create-sprint/SKILL.md), [`close-sprint`](../skills/close-sprint/SKILL.md)

### 3.6 Process is proportional to risk

Three tiers — **Trivial**, **Standard**, **Major** — decided provisionally at triage and authoritatively at intake. A typo does not earn an ADR; a new capability does not get to skip one. Ceremony that outweighs its own value is a defect, not rigor.

→ [`start-thin-slice`](../skills/start-thin-slice/SKILL.md) Step 5 (routing), [`intake-code-contribution`](../skills/intake-code-contribution/SKILL.md) Step 0 (criteria)

### 3.7 The engineer who builds cannot approve

One Principal Engineer persona, three explicit modes — **architect**, **implementer**, **reviewer** — never two at once. Architect mode cannot edit source. Implementer mode cannot modify approved ADRs, design specs, or wave docs; a design flaw bounces back to architect mode. Reviewer mode is read-only on source and files structured change requests. This is a bias firewall, and it is the discipline most easily lost when a single agent does everything.

→ [`principal-engineer.agent.md`](../agents/principal-engineer.agent.md)

### 3.8 Quality is specified before it is tested; tests are organized by ownership

`qa.md` states risk tiers, test-layer mapping, behavioral invariants, security coverage, explicit out-of-scope, and observable definition-of-done — as a specification, with **no code in it**.

Execution follows the Pyramid Test Strategy organized by ownership: **Logic → Composition → Adapter Contract → Integration Boundary → Journey**. The cardinal rule is *one property of a behavior, at one layer*. Asserting business math in a Composition test is a defect in the test, not a bonus.

Two reporting rules are non-negotiable: **"Environment Blocked ≠ Test Failed"**, and no skipped tests or hard-wait sleeps reach `main`.

→ [`create-quality-spec`](../skills/create-quality-spec/SKILL.md), [`test-by-ownership`](../skills/test-by-ownership/SKILL.md), [`check-no-skipped-tests.sh`](../scripts/check-no-skipped-tests.sh), [`check-no-sleep-waits.sh`](../scripts/check-no-sleep-waits.sh), [`check-port-adapter-parity.sh`](../scripts/check-port-adapter-parity.sh)

### 3.9 Decisions that bind the future become ADRs; boundaries become frozen contracts

An ADR carries a **real alternatives comparison table**, consequences including the negative ones, and a `status` lifecycle (`Proposed` → `Accepted` → `Superseded`). Major-tier implementation may not begin until `status: Accepted`.

A boundary another unit will build against becomes a **Seam Contract**: a machine-readable Shape, a shared Behavior suite, and a frozen `<name>@vN` id in `.seam-contracts.json`. The freeze is what converts *"wait for the producer to merge"* into *"build against a promise."*

→ [`create-adr`](../skills/create-adr/SKILL.md), [`define-seam-contract`](../skills/define-seam-contract/SKILL.md), [`check-seam-contract-parity.sh`](../scripts/check-seam-contract-parity.sh)

### 3.10 Production readiness is declared at the wave, conformed at the slice

Four anchors — **Configurable**, **Observable**, **Horizontally-scalable**, **Resilient**. The wave declares the posture; a slice does not re-litigate it, it names the seams it touches and confirms it *preserves* the posture, or records a **reviewed deviation**.

Every external call declares timeout, retry policy, fallback, and — on hot paths — a circuit breaker. Every cross-process path emits structured logs with a correlation ID plus latency / throughput / error metrics.

An honest one-line `n/a (<reason>)` beats a padded block. A **blank or reasonless N/A is exactly the theater this method rejects.**

→ [`check-config-externalized.sh`](../scripts/check-config-externalized.sh), [`check-observability-at-seams.sh`](../scripts/check-observability-at-seams.sh), [`check-stateless-request-path.sh`](../scripts/check-stateless-request-path.sh), [`check-resilient-boundary.sh`](../scripts/check-resilient-boundary.sh)

---

## 4. The spine

The method is one ordered pipeline, not a toolbox of independent skills. At Standard tier:

```
PLAN ──────────► TRIAGE ────────► BUILD ─────────────────────► LEARN ─────────► TEACH
create-wave →    start-thin-      create-sprint → intake →      close-sprint     author-
product docs:    slice            implement → verify            (bidirectional   user-docs
the outcomes and (tier + hard                                   distillation,
the educated     precondition                                   then the sprint
theory of design gate)                                          is deleted)
```

**PLAN** produces the product documents: the intended outcomes and the best educated theory of the design — user experience in `product-design.md`, architecture in `product-architecture.md`. PLAN is measured by whether those documents carry the outcomes and design that a later decision needs, not by a document count; `intake` later verifies they are present and specific enough to build on.

**TRIAGE** reads them to check a slice's preconditions and classify its tier. Both checks are a hard gate: status must be workable and every declared dependency must be `✅ Complete`, or the work stops here.

**BUILD** locks the bridge, runs the intake envelope, implements, and verifies. Verification requires **captured `verify` output — a bare checkbox is rejected** — plus an adversarial review of seam behavior.

**LEARN** records outcome evidence and makes a continue / pivot / stop call, then distills bidirectionally: into product artifacts (wave docs, dashboard) *and* engineering artifacts (capability records, ADRs, handbook). Then the sprint file is deleted.

**TEACH** renders validated behavior into Diátaxis-structured user guides. Only validated behavior — if the capability record is still an educated theory, there is nothing to teach yet.

### Tier branching

| Tier | Path |
|---|---|
| **Trivial** | `intake` (abbreviated) → `implement-with-defensive-patterns` → `verify-and-assemble-pr`. **No sprint, no close.** |
| **Standard** | The full spine above. |
| **Major** | Inserts the architect pipeline *before* the sprint: `discovery-and-ambiguity-log` → `design-system-architecture` → `design-capability-layout` → `create-adr` (**`status: Accepted`**) → then `create-sprint` onward. `define-seam-contract` is used wherever the design declares a boundary another slice will build against. |

The exact ordered routing, including where each approval line is waited on, is [`start-thin-slice`](../skills/start-thin-slice/SKILL.md) Step 5. Do not paraphrase it — two sources drift.

### Entry points

| Situation | Start here |
|---|---|
| Greenfield repo | [`bootstrap-project`](../skills/bootstrap-project/SKILL.md) |
| Existing repo that just installed Praxis | [`provision-project-overlay`](../skills/provision-project-overlay/SKILL.md) |
| Legacy layered codebase | [`refactor-layered-to-capability`](../skills/refactor-layered-to-capability/SKILL.md) — one shippable slice at a time, never a big-bang rewrite |
| A named slice ("Work on TS-NNN") | [`start-thin-slice`](../skills/start-thin-slice/SKILL.md) |

---

## 5. Where fidelity is made

Each stage emits something a reviewer can inspect. This table is the fidelity story of §2 made concrete.

| Stage | Emits (the visible proof) | Kind |
|---|---|---|
| **Plan** | the product documents — intended product outcomes and the best educated theory of the proposed product design (UX in `product-design.md`, architecture in `product-architecture.md`), holding the thin-slice definitions and acceptance criteria TRIAGE classifies from; presence and specificity verified at intake | script-checkable |
| **Triage** | precondition hard-gate (status + dependencies `✅`) and the tier decision | script-checkable |
| **Architecture** (Major) | ADR with a real alternatives table and `status: Accepted`; frozen seam contracts in `.seam-contracts.json` | script-checkable |
| **Sprint** | signed Sprint Plan Approval (Standard + Major) and Design Approval (Major); Acceptance↔Test matrix; four production-readiness anchors | human-signed + script-checkable |
| **Verify** | captured `verify` output — a bare checkbox is rejected — plus the adversarial seam-behavior review | script-checkable + agent-attested |
| **Close** | outcome evidence and a continue / pivot / stop call, distilled back into product *and* engineering artifacts | agent-attested |

---

## 6. Enforcement, honestly

The **Kind** column above maps onto three gate types, and knowing which is which is the difference between a guarantee and a good intention:

| Gate kind | Enforced by | Fails closed? |
|---|---|---|
| **Script-enforced** | `verify.sh` / CI running the `check-*.sh` probes | Yes — once wired into CI or a git hook. Several probes are warn-first until the project sets `mode: enforce`. |
| **Human-signed** | a person filling an approval line | The next skill refuses to proceed without the signature — but only a compliant agent checks. |
| **Agent-attested** | the agent following the skill | **No** on a bare harness. Trusted, not mechanically compelled. |

One gate fails closed today with no runtime required: **Major-tier Design Approval** ([`check-design-approval-gate.sh`](../scripts/check-design-approval-gate.sh)) hard-fails when a Major sprint's referenced ADR is not `Accepted` or its Design Approval block is still template placeholders.

The full operational breakdown — which named gate sits in which tier — is [`using-praxis`](../skills/using-praxis/SKILL.md) § *Enforcement model*.

### Script-checkable vs. runtime-enforced

Two things are easily confused.

A **script-checkable artifact** — an ADR with `status: Accepted`, a signed sprint line, a non-empty alternatives table — is something a probe can verify exists and carries substance. That is Praxis's job.

A **runtime-enforced gate** — the agent is actually *prevented* from proceeding until the check passes — requires an orchestration runtime and is not Praxis's job on a bare harness.

When this document or a skill calls a gate "mechanical," it means **script-checkable, not runtime-enforced**. On a bare harness, honoring the gate is a self-enforced behavioral contract; with an orchestration runtime it can be made to fail closed.

### Shape vs. substance

The probes check that a file exists, a pattern matches, a count is right — not the quality of the reasoning behind it. Several are labeled heuristics in their own header comments; read a `check-*.sh` header before trusting a green run as more than that. A green run is evidence, not proof.

[`check-escape-hatch-usage.sh`](../scripts/check-escape-hatch-usage.sh) exists so that using a `praxis:allow-*` opt-out is never silent to a reviewer — it reports every marker by `file:line` and never fails the build.

---

## 7. Parallelism is a permission, not a plan

Praxis never schedules parallel work. Concurrency is an **emergent permission** exercised by a human or an orchestration runtime. Two units may run concurrently **only if all four conditions hold**:

1. **Capability/file disjoint** — no source file or capability in common.
2. **Persistent-resource disjoint** — no shared table, topic, queue, cache, or migration.
3. **Config-key disjoint** — no shared configuration key.
4. **Frozen-contract dependent** — each depends only on a frozen `<name>@vN` seam contract, never on the other's in-flight internals.

Capability-disjointness alone is **not** sufficient. If any condition fails, the units are sequential. The collision-safe coordination artifacts in [`create-sprint`](../skills/create-sprint/SKILL.md) (guarded by [`check-sprint-id-collision.sh`](../scripts/check-sprint-id-collision.sh)) and the staleness re-anchor in [`intake-code-contribution`](../skills/intake-code-contribution/SKILL.md) are what make a permitted parallel run actually safe.

---

## 8. The sub-skills

Each sub-skill is a focused, repeatable workflow. Load its `SKILL.md` before following it.

### Roles

| Persona | Owns |
|---|---|
| [`product-manager`](../agents/product-manager.agent.md) | Wave planning, sprint creation as immutable bridges, bidirectional sprint close, an honest dashboard |
| [`product-designer`](../agents/product-designer.agent.md) | User value, thin-slice acceptance criteria, `product-design.md`; leads `qa.md` when user-facing risk dominates; owns TEACH |
| [`principal-engineer`](../agents/principal-engineer.agent.md) | Capability-driven architecture, refactoring, cross-cutting decisions — in architect / implementer / reviewer mode, one at a time |

### Always-on constraints

| Guardrail | Scope |
|---|---|
| [`lean-delivery-guardrails.instructions.md`](../instructions/lean-delivery-guardrails.instructions.md) | `docs/product/**`, `docs/architecture/**`, `docs/guides/**`, `docs/waves/**`, `docs/sprints/**` |
| [`capability-driven-guardrails.instructions.md`](../instructions/capability-driven-guardrails.instructions.md) | `src/**`, `packages/**`, `services/**`, `apps/**`, `libs/**`, `modules/**` |
| [`code-contribution-intake.instructions.md`](../instructions/code-contribution-intake.instructions.md) | Everything, before any implementation begins |

> Copilot auto-applies these by `applyTo` glob. Claude Code and other harnesses have no `applyTo` mechanism — for those, the compressed summary in [`using-praxis`](../skills/using-praxis/SKILL.md) is the always-on surface, and the rules are in force whenever you touch matching paths.

### PLAN — product intent and the educated theory

| Skill | Use when |
|---|---|
| [`bootstrap-project`](../skills/bootstrap-project/SKILL.md) | A greenfield repo needs `.github/` + `.claude/` + a capability-driven `src/` skeleton |
| [`provision-project-overlay`](../skills/provision-project-overlay/SKILL.md) | An existing repo just installed Praxis and needs a project-specific overlay (interview-driven, idempotent) |
| [`create-wave`](../skills/create-wave/SKILL.md) | Starting a wave; scaffolds the four-document pattern and registers it on the dashboard |
| [`create-product-design-spec`](../skills/create-product-design-spec/SKILL.md) | Authoring `product-design.md` — journeys, UX states, ambiguity handling, recovery paths |
| [`create-product-architecture-spec`](../skills/create-product-architecture-spec/SKILL.md) | Authoring wave-scoped `product-architecture.md` — domain ownership, contracts, seams, integrations, failure behavior |
| [`create-quality-spec`](../skills/create-quality-spec/SKILL.md) | Authoring `qa.md` — risk tiers, test-layer mapping, security coverage, observable DoD |

### TRIAGE — preconditions and routing

| Skill | Use when |
|---|---|
| [`start-thin-slice`](../skills/start-thin-slice/SKILL.md) | The front door for slice work; hard precondition gate, provisional tier, deterministic route |

### ARCHITECT (Major tier) — before any sprint

| Skill | Phase | Use when |
|---|---|---|
| [`discovery-and-ambiguity-log`](../skills/discovery-and-ambiguity-log/SKILL.md) | 1 | Surfacing assumptions, defining SLO/SLA, producing an Ambiguity Log |
| [`design-system-architecture`](../skills/design-system-architecture/SKILL.md) | 2–3 | Topology, resilience patterns, contract-first APIs, storage strategy, expand/contract migrations |
| [`design-capability-layout`](../skills/design-capability-layout/SKILL.md) | 4 | Mapping the capability into a vertical slice with functional core / imperative shell and declared Ports |
| [`create-adr`](../skills/create-adr/SKILL.md) | — | A decision binds future work; alternatives table mandatory; must reach `status: Accepted` |
| [`define-seam-contract`](../skills/define-seam-contract/SKILL.md) | — | A boundary another slice will build against; produces Shape + Behavior suite + frozen `<name>@vN` |

### BUILD — bridge, gate, implement, verify

| Skill | Phase | Use when |
|---|---|---|
| [`create-sprint`](../skills/create-sprint/SKILL.md) | — | Locking the immutable bridge: slice intent + current-state snapshot + hypothesis card + test plan |
| [`intake-code-contribution`](../skills/intake-code-contribution/SKILL.md) | 0 | Pre-implementation gate — mandatory before **any** code change |
| [`test-by-ownership`](../skills/test-by-ownership/SKILL.md) | — | Choosing the right layer for each behavior; Port/Adapter parity; consumer-driven contracts |
| [`implement-with-defensive-patterns`](../skills/implement-with-defensive-patterns/SKILL.md) | 5 | Writing the implementation — composition over inheritance, shift-left security, structured telemetry |
| [`verify-and-assemble-pr`](../skills/verify-and-assemble-pr/SKILL.md) | 6 | Verifying with captured output, running the refactor decision matrix, assembling the PR narrative + rollback plan |

### LEARN and TEACH

| Skill | Use when |
|---|---|
| [`close-sprint`](../skills/close-sprint/SKILL.md) | Recording outcome evidence and distilling learnings into **both** product and engineering artifacts; then deleting the sprint file |
| [`author-user-docs`](../skills/author-user-docs/SKILL.md) | Rendering a validated capability record into Diátaxis guides in `docs/guides/` |

### Continuous

| Skill | Use when |
|---|---|
| [`refactor-layered-to-capability`](../skills/refactor-layered-to-capability/SKILL.md) | Migrating a legacy `controllers/` + `services/` + `utils/` codebase into vertical slices, one shippable slice at a time |
| [`using-praxis`](../skills/using-praxis/SKILL.md) | Session bootstrap — the operational index re-surfaced at session start and after `/clear` or compaction |

### How the two halves compose

Praxis bundles two complementary capability stacks that share the same precedence and scope rules:

- **Lean delivery** — wave planning, design / architecture / quality specs, sprint as immutable bridge, ADRs, Pyramid Test Strategy, bidirectional sprint close. Owned by the product-manager and product-designer personas.
- **Principal Engineer discipline** — the phased workflow (Discovery → System Architecture → Capability Layout → Implementation → Verify), bootstrap, refactor-to-capability, anti-dumping enforcement. Owned by the principal-engineer persona.

They interlock: `create-product-architecture-spec` (wave-scoped) feeds `design-system-architecture` (cross-cutting) when a wave introduces a new subsystem; `create-quality-spec` and `test-by-ownership` shape what `verify-and-assemble-pr` actually verifies; `close-sprint` writes back into both product artifacts (intent) and engineering artifacts (ADRs, capability layout, handbook).

---

## 9. What the method is not

- **Not a stack.** No framework, language, or runtime is prescribed. If an example is needed, it shows two contrasting languages.
- **Not a replacement for project instructions.** Praxis sets defaults; the host repo's `.github/`, `.claude/`, or workspace instructions **always win**.
- **Not personal style.** Formatting, quote style, naming flavor — out of scope, by rule.
- **Not runtime orchestration.** Delegation mechanics, ticketing, branch protection, runtime verification gates, and circuit breakers as live checks belong elsewhere.
- **Not an executable.** No MCP servers, no executable agents. Guidance plus a set of generic, configurable probes.

### Layering and precedence

Praxis is one tier in a four-tier system:

```
repo .github/copilot-instructions.md, .claude/CLAUDE.md      (highest)
repo .github/instructions/, .github/agents/, .github/skills/
─────────────────────────────────────────────────────────────
plugin instructions/, agents/, skills/                       (defaults — Praxis)
─────────────────────────────────────────────────────────────
user ~/.claude/CLAUDE.md, VS Code user prompts               (personal)
```

Repo guidance always overrides plugin guidance. Praxis should never assume its rules are the final word.

### Composes with orchestration runtimes (Claude MPM, others)

Praxis is **artifact discipline**, not runtime orchestration. It deliberately does not implement agent delegation / hand-off mechanics, verification gates as runtime checks, ticketing integration, branch protection / PR creation flows, or circuit breakers and error budgets at runtime.

Those concerns belong in an orchestration runtime such as [Claude MPM](https://github.com/bobmatnyc/claude-mpm). When MPM is installed alongside Praxis:

- Praxis produces the artifacts (`product-design.md`, `product-architecture.md`, `qa.md`, sprint files, ADRs, wave READMEs, handbook updates).
- MPM's PM agent uses those artifacts as the source of truth for delegation, and its specialists (`typescript`, `qa`, `design`, `product`) align to the personas defined here. Host repos may overlay project-specific persona names that extend these role-based agents.

The two are orthogonal. Either can be used without the other; together they cover both "what to build" and "how the agents collaborate to build it."

The Major-tier Design Approval gate shows how the composition extends beyond a bare harness. `scripts/check-design-approval-gate.sh` is a plain script with no runtime dependency — on a bare harness it runs as a pre-push git hook and fails the push. The same mechanism generalizes: an orchestration runtime could invoke it as a **pre-delegation** gate before dispatching implementer-mode work on a Major-tier slice, refusing the hand-off entirely if the check fails — a stronger enforcement point than blocking at push time, because it stops the agent from ever starting. This is a documented composition pattern, **not a built or tested integration**; Praxis ships the mechanism, not the runtime wiring.

---

## 10. How the plugin is built

*(From here on, this document is about changing Praxis rather than using it.)*

### Architecture (capability-driven, applied to itself)

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
│   ├── check-no-skipped-tests.sh                # test-hygiene gate (no committed skipped tests)
│   ├── check-no-sleep-waits.sh                  # test-hygiene gate (no hard-wait sleeps)
│   ├── check-port-adapter-parity.sh             # port/adapter parity gate
│   ├── check-config-externalized.sh             # production-readiness probe (Configurable anchor)
│   ├── check-observability-at-seams.sh          # production-readiness probe (Observable anchor)
│   ├── check-stateless-request-path.sh          # production-readiness probe (Horizontally-scalable anchor)
│   ├── check-resilient-boundary.sh              # production-readiness probe (Resilient anchor)
│   ├── check-seam-contract-parity.sh            # seam-contract parity gate (Shape + Behavior suite)
│   ├── check-sprint-id-collision.sh             # coordination-artifact gate (parallel sprint-id collision)
│   ├── check-design-approval-gate.sh            # Major-tier Design Approval fail-closed gate (hard-fail, not warn-first)
│   ├── check-escape-hatch-usage.sh              # diff-scoped escape-hatch marker report (informational, never fails)
│   ├── bump-version.sh                          # version-parity tool across manifests
│   ├── test-probes.sh                           # self-test: probe language coverage (fixtures)
│   ├── test-citation-scan.sh                    # self-test: citation_scan.py (fence/quote/span + markers)
│   ├── gen-doctrine-index.sh                    # generator: guardrail applyTo scope table (--write/--check)
│   ├── citation_scan.py                         # shared citation-vs-assertion rules (ADR.260725)
│   ├── gen-coverage-matrix.sh                   # generate/check docs/coverage-matrix.md from probe includes
│   ├── gen-tier-table.sh                        # generate/check the tier table across 3 surfaces from scripts/data/tier-classification.json
│   └── validate-plugin.sh                       # plugin self-test
├── docs/                                        # all project context — mirrors what Praxis tells host repos
│   ├── project-context.md                       # this file — the method and its governance
│   ├── coverage-matrix.md                       # generated: probe language coverage
│   ├── DEPLOY.md                                # release/publish runbook
│   ├── product/                                 # product tree
│   │   ├── README.md                            # product overview + dashboard
│   │   └── waves/                               # wave intent (four-document pattern)
│   ├── architecture/                            # durable engineering truth
│   │   ├── README.md                            # system overview
│   │   ├── adr/                                 # cross-capability decisions
│   │   ├── skills/  distribution/  enforcement/ # capability records
├── .claude/CLAUDE.md                            # bootstrap pointer for Claude Code (repo-direct sessions)
├── AGENTS.md                                    # bootstrap pointer — root-mandated by Codex/Aider/Amp
├── GEMINI.md                                    # bootstrap pointer — root-mandated by gemini-extension.json
├── README.md                                    # human front door — pitch and install
├── .version-bump.json                           # declared manifests for the bump tool
└── CHANGELOG.md
```

**No `utils/`. No `common/`. No `shared/`.** If a future addition tempts a dumping ground, name the actual capability or duplicate.

### Why the root is nearly empty

All project context lives under `docs/`, exactly as Praxis instructs host repos ([`bootstrap-project`](../skills/bootstrap-project/SKILL.md) Step 6). Praxis follows its own convention rather than exempting itself.

Four files cannot move, and the reason is an external contract, not preference:

| File | Why it must stay at root |
|---|---|
| `README.md` | GitHub renders it as the repository front page |
| `AGENTS.md` | The AGENTS.md convention is repo-root-only; Codex, Aider, and Amp auto-load it from there |
| `GEMINI.md` | `gemini-extension.json` declares `contextFileName: GEMINI.md`, resolved from the extension root |
| `CHANGELOG.md` | Keep-a-Changelog convention; release tooling and GitHub both expect it at root |

`CLAUDE.md` **did** move — to `.claude/CLAUDE.md`, which Claude Code reads natively. Every one of these is a pointer of ~15 lines; none carries doctrine.

### Single source, many harnesses

Every harness manifest (`.claude-plugin/`, `.codex-plugin/`, `.cursor-plugin/`, `.opencode/`, `gemini-extension.json`) points at the **same** `skills/`, `agents/`, and `instructions/` trees. There are no per-harness content forks.

The bootstrap skill `skills/using-praxis/SKILL.md` is the single entry point loaded by every harness through its native mechanism (SessionStart hook, system-prompt transform, or `@`-include in a context file). It is the **only** file any harness loads automatically in full, and it is re-injected on every `/clear` and compaction — so every token in it is paid repeatedly. Keep it operational; keep doctrine here.

Versions across all manifests are kept in sync by `scripts/bump-version.sh` against `.version-bump.json`.

---

## 11. Governance

### Scope rule (the litmus test)

A piece of guidance belongs in this plugin **only if** the answer to all four is yes:

1. Would it apply unchanged to a Rust CLI, a Python data pipeline, _and_ a TypeScript web app — across both engineering and product planning contexts?
2. Is it about engineering or delivery discipline, not personal style?
3. Would you defend it in a code review or planning review against any team?
4. Does it measurably improve the agent's execution fidelity, or close a known agent failure mode?

The first three questions decide _where_ a rule belongs. If any of them is no, it belongs in a project's `.github/` (project-specific) or in `~/.claude/` / VS Code user prompts (personal preference) — **not here**. The fourth decides whether the rule is worth adding _at all_: if it is no, the rule does not belong in the plugin regardless of how universal it is. Universality is necessary but not sufficient — a rule can be universal, disciplined, and defensible while still adding more ceremony than value, and such a rule stays out.

#### Examples

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

### Evolution policy

#### Adding a skill, instruction, or agent

1. Pass the scope litmus test above — all four questions. If it fails, do not add.
2. Validate the rule by using it in at least one real repo first. This is the non-negotiable step, not a formality. Dogfooding the plugin on itself does not substitute: it proves internal consistency, not that the rule improves fidelity on real work. Do not invent rules theoretically.
3. Only once real-repo validation exists: write the artifact, dogfood it against the plugin if possible, and bump the minor version. The `CHANGELOG.md` entry for that bump must cite the real-repo evidence from step 2; a bump that can only cite dogfooding does not qualify.

#### Removing or breaking a rule

1. Document the deprecation in `CHANGELOG.md` one minor version before removal.
2. Major-version bump on actual removal.

#### Relaxing a rule

A **relaxation** widens the set of valid inputs, structures, or usages without invalidating anything that was valid before. It is governed by neither clause above — no one-minor-version deprecation notice, and no removal bump.

1. Apply the breakage test: **does any tree that validated before the change fail after it?** If yes, the change is a removal or a break and the clauses above apply in full. Classification turns on adopter breakage, not on whether the diff deletes lines.
2. Ship it in the release it lands in, at the bump its own change class earns.
3. The `CHANGELOG.md` entry must state explicitly that prior usage remains valid, so the absence of a migration step is a claim a reader can check rather than an omission they have to trust.

A change that relaxes one constraint while tightening an adjacent one takes the stricter classification. See `docs/architecture/adr/ADR.260724-wave-category-relaxation.md` for the decision that added this case and the worked example that motivated it.

#### Versioning

- **Patch:** typo, clarification, formatting.
- **Minor:** new skill / instruction / agent / additive rule, or a relaxed rule.
- **Major:** removed skill, breaking rule change, structural reorganization, or plugin rename.

**Pre-1.0 (`0.y.z`), the minor position carries the major signal.** This is standard semver — under `0.y.z` the public API is not yet declared stable, so a breaking or structural change bumps `y`, not `x`. A change that would be `1.x → 2.0.0` after 1.0 is `0.4.0 → 0.5.0` today.

The classification above is unchanged by this — a structural reorganization is still a *major* change, and it still earns the strongest bump available. Pre-1.0 that bump is the minor position. Reserve `1.0.0` for the deliberate declaration that the skill, instruction, and agent interfaces are stable and will be supported — not for the next breaking change that happens to come along.

### Quality gates for plugin contributions

- Every skill has `name` and `description` frontmatter and an "Use this when" section at the top.
- Every instruction has `applyTo` if scoped, or is explicitly always-on.
- Every agent has a single, unambiguous responsibility.
- No skill prescribes a specific language or framework. If an example is needed, show two contrasting languages.
- No file references stack-specific paths (e.g., `services/api/src/domains/`). Use placeholders like `<capability-root>/` or `<docs-root>/product/waves/`.
- Anti-dumping rule applies recursively — the plugin itself must not introduce `utils/`, `helpers/`, `common/`, `shared/`.
- `scripts/validate-plugin.sh` must pass. Its **inventory parity** check requires every skill, script, and instruction to be named in this file and in `README.md` — so adding a capability without documenting it fails the build by design.

---

## Verifying the method is loaded

Ask the agent:

> *Tell me about your praxis.*

It should name the three personas, the three always-on guardrail sets, and at least four skills with their triggers. If it cannot, the bootstrap is not loaded.
