---
name: using-praxis
description: Bootstrap entry point for the Praxis plugin. Loaded at session start by every harness (Claude Code, Codex, Cursor, Gemini, OpenCode, Copilot). Teaches the agent which personas exist, which guardrails are always-on, and which skill to invoke for each product or engineering moment.
user-invocable: true
disable-model-invocation: false
---

# Praxis — Operational Index

You are operating with **Praxis** loaded: an opinionated method that fuses **lean wave-based product delivery** with **Principal Engineer discipline**. Doctrine is universal — language-, framework-, and runtime-agnostic; **static enforcement is best-effort per language** ([`docs/coverage-matrix.md`](../../docs/coverage-matrix.md)).

The host repository's own `.github/`, `.claude/`, or workspace instructions **always override** anything here. When in doubt, the repo wins.

In a **monorepo**, a package tier sits above the repository: `<pkg>/.praxis/context.md` and `<pkg>/.github/`. Walk from the file you are working on toward the repository root and take the nearest declaration of each fact — a package states only what differs and inherits the rest. When a product spans **several repositories**, `praxis.config.yaml`'s optional `paths.product_root` names where the whole product's intent lives; if it is set, this repository is a part and its dashboard says so.

This file is your **router** — what to do right now. It is re-injected at session start and after `/clear` or compaction so the guardrails below stay in force. Skim it; load specific skills on demand.

For the *why* — the trust-transfer problem, the ten opinions, the full enforcement rationale, and plugin governance — read [`docs/product.md`](../../docs/product.md). Don't load it mid-task unless the question is about the method itself.

---

## How to use this plugin

Identify the moment (plan, triage, design, implement, review, ship) → pick the persona → load that skill's `SKILL.md` → honor the always-on guardrails. When the host repo is silent, follow these defaults; when it speaks, it wins.

---

## Personas

Load the persona's full file before acting in that role.

| Persona | When to be this persona | File |
|---|---|---|
| **Product Manager** | Wave planning, sprint creation as immutable bridges, sprint closing with bidirectional learning capture, dashboard honesty | `agents/product-manager.agent.md` |
| **Product Designer** | User value, thin-slice acceptance criteria, authoring `product-design.md`; leads `qa.md` (paired with the engineer, designer-owned when user-facing risk dominates) | `agents/product-designer.agent.md` |
| **Principal Engineer** | Capability-driven architecture, refactoring, cross-cutting decisions; operates in three modes — architect, implementer, reviewer — never two at once | `agents/principal-engineer.agent.md` |

**The same engineer cannot self-approve.** If you are the implementer, you cannot also be the reviewer in the same session — switch personas explicitly or hand off.

---

## Always-on guardrails

These ship as `applyTo`-scoped `.instructions.md` files. **Copilot** auto-applies each one whenever you edit a file matching its glob. **Claude Code and other harnesses have no `applyTo` mechanism** and do not auto-load `instructions/` — for those, the summary below is the always-on surface, and you must treat each rule set as in force whenever you touch the matching paths. In a provisioned repo, `provision-project-overlay` copies these into `.github/instructions/` so Copilot picks them up natively.

Each guardrail's scope is generated from its own frontmatter — the table below is rendered by `scripts/gen-doctrine-index.sh`, and CI fails on a hand-edit inside it.

<!-- BEGIN GENERATED: guardrail-scope (source: instructions/*.instructions.md frontmatter; regenerate with scripts/gen-doctrine-index.sh --write) -->
| Guardrail | Applies to |
| --------- | ---------- |
| [Capability-Driven Engineering Guardrails](../../instructions/capability-driven-guardrails.instructions.md) | `src/**`, `packages/**`, `services/**`, `apps/**`, `libs/**`, `modules/**` |
| [Code Contribution Intake Guardrails](../../instructions/code-contribution-intake.instructions.md) | `**` |
| [Lean Delivery Guardrails](../../instructions/lean-delivery-guardrails.instructions.md) | `docs/product/**`, `docs/architecture/**`, `docs/capabilities/**`, `docs/guides/**`, `docs/sprints/**` |
<!-- END GENERATED -->

### Lean Delivery — `instructions/lean-delivery-guardrails.instructions.md`


- Waves are intent, not bigger sprints. Each wave has four documents: README, product-design, product-architecture, qa.
- Thin-slices are atomic user outcomes with stable IDs (`TS-NNN`). Corrections keep the same ID.
- Sprint is an immutable bridge — scope is locked once it starts. To change scope, close it and create a new one.
- Sprint close is bidirectional: learnings flow into product artifacts AND engineering artifacts; sprint files are then deleted.
- **Wave = educated theory; capability record = truth.** Wave docs point into `docs/architecture/`; they never duplicate current-state topology.
- Quality is specified in `qa.md` before it is tested. No code in `qa.md`.
- Code contribution intake (wave, slice, specs, sprint, code state, test posture) comes before any implementation.
- ADRs capture durable decisions with a mandatory alternatives table.

### Capability-Driven Engineering — `instructions/capability-driven-guardrails.instructions.md`


- Organize by **business capability**, not technical layer. No `controllers/`, `models/`, `services/`, `views/`, `handlers/` silos.
- **Anti-dumping:** `utils/`, `helpers/`, `common/`, `shared/`, `misc.*`, `lib.*` are forbidden. Name the actual capability or duplicate.
- Functional core, imperative shell — pure logic in one file, I/O wrapper in another.
- Cross-capability calls go through an explicit public surface. No deep imports.
- Every external call declares timeout, retry policy, fallback, and (for hot paths) circuit breaker.
- Every cross-process path produces structured logs with correlation ID, plus latency / throughput / error metrics.
- Two units may be built concurrently only under the four-condition rule below.

### Code Contribution Intake — `instructions/code-contribution-intake.instructions.md`

Applies before any user-story, feature, thin-slice, or behavior-changing contribution. Run `intake-code-contribution` first.

---

## The spine

```
PLAN ──────────► TRIAGE ────────► BUILD ─────────────────────► LEARN ─────────► TEACH
create-wave →    start-thin-      create-sprint → intake →      close-sprint     author-
product docs     slice            implement → verify                             user-docs
```

**Tier branching** — provisional at `start-thin-slice`, authoritative at `intake` Step 0:

- **Trivial** — `intake` (abbreviated) → `implement-with-defensive-patterns` → `verify-and-assemble-pr`. No sprint, no close.
- **Standard** — the full spine above.
- **Major** — architect pipeline *first*: `discovery-and-ambiguity-log` → `design-system-architecture` → `design-capability-layout` → `create-adr` (**`status: Accepted`**) → then `create-sprint` onward.

The **canonical ordered path**, including where each approval line is waited on, is `start-thin-slice` Step 5. Do not paraphrase it.

---

## Skill index

Load the `SKILL.md` of any skill you intend to follow.

| Stage | Skill | Use when |
|---|---|---|
| **Enter** | `bootstrap-project` | Greenfield repo needs `.github/` + `.claude/` + capability-driven `src/` |
| | `provision-project-overlay` | Existing repo just installed Praxis; needs a project overlay (interview-driven, idempotent) |
| | `refactor-layered-to-capability` | Legacy `controllers/` + `services/` → vertical slices, one shippable slice at a time |
| **PLAN** | `ask-what-is-true` | Ask the record what this repository already knows before reconstructing it — run first, and read what the answer says it does not cover |
| | `event-storming` | Upstream domain discovery — map business events to bounded contexts & candidate `CAP.` records |
| | `name-a-capability` | Derive a capability from a cluster of events in a storm — consistency boundaries, the four gate tests with their reasons, and the exclusion that makes the boundary real |
| | `cut-a-slice` | Cut an atomic vertical slice in a shape a checker can refuse — one command or one view, one actor, declared layers, claims naming their evidence |
| | `declare-an-entity-kind` | Declare a kind the record must hold, and its shape, on the architecture's schema — an amendment to the record, never a change to the engine |
| | `witness-a-rule` | Declare a rule together with the record that demonstrates it refusing — a rule never shown to refuse is indistinguishable from one that cannot |
| | `anchor-a-doctrine-surface` | Declare what a shipped skill, guardrail, agent or probe exists to serve — instruction the record cannot trace is doctrine an agent follows on the plugin's authority alone |
| | `see-what-is-ready` | Ask which slices could be started right now, and what would refuse each of the rest — including the conditions the gate declares and nobody computed |
| | `see-the-dashboard` | Where the product stands right now — several views composed on demand into one document, each still singly owned, and never committed |
| | `pick-up-a-slice` | Take a slice through the one gate — selection is the commitment, refusals are recorded as facts, and the override is recorded beside the refusal it overrides |
| | `attach-evidence` | Back a declared layer with an artifact at the moment it is reached — evidence outside the slice's declared set is refused, and a layer you did not reach stays visible as unevidenced |
| | `close-an-iteration` | Close without dropping scope in silence — a shortfall is carried by a finding that names the claim, and deleting a claim to make the close succeed is itself refused |
| | `review-in-flight` | Preview what an iteration promised against what it has shown, at any moment and writing nothing — the layers it has NOT reached are listed, not omitted |
| | `record-a-decision` | Record a choice with the alternatives it rejected and what would show it wrong — bound to the iteration that forced it, corrected only by appending |
| | `document-usage` | Write how a capability is used while building it, into the capability record — and a guide for behaviour a version never shipped is refused |
| | `bind-work-to-a-version` | Attach closed iterations to a version so its content is derived rather than hand-kept — only closed work binds, once, and the bump is proposed from configured rules |
| | `cut-the-version` | Cut a planned version and write its index node in one operation — the index points at a commit rather than copying it, and the seal makes a later edit visible |
| | `declare-a-lifetime` | Decide whether a read model survives being frozen and record why — the publication test, refused in both directions without a reason |
| | `publish-the-release-set` | Regenerate every published document whole for one version, before the cut — no splice path, no wall-clock stamp, and no renderer branch on what a view means |
| | `verify-the-published-tree` | Prove no published document was hand-edited — compared against the commit its release names, never against the record it is meant to outlive |
| | `promote-what-shipped` | Fold a cut release into what each capability says it is — derived from bound work, recomputed by a rule, and refused if hand-edited in either direction |
| | `resolve-a-symptom` | Close a symptom by naming a release that demonstrably attacked it — computed from what shipped, and withdrawn rather than grandfathered when it cannot be |
| | `create-initiative` | Scaffold or refine a single-file growth initiative (`INIT.<initiative-name>.md`) |
| | `create-capability-record` | Scaffold or update a living capability record (`CAP.<capability-name>.md`) |
| | `create-wave` | Alias for `create-initiative` — single-file growth initiative on `docs/product.md` |
| | `derive-waves-from-history` | Adopting on a product that already shipped; derives waves from its own record instead of inventing criteria |
| | `create-product-design-spec` | `INIT.` / `CAP.` — journeys, Given/When/Then criteria, UX state matrices, recovery paths |
| | `create-product-architecture-spec` | `INIT.` / `CAP.` — domain ownership, seam contracts (`<name>@vN`), Ports/Adapters breakdown |
| | `create-quality-spec` | `INIT.` / `CAP.` — test layer mapping, quantitative NFR targets, 4 Production Readiness anchors |
| **TRIAGE** | `start-thin-slice` | Front door ("Work on TS-NNN"); precondition hard-gate, provisional tier, route |
| **ARCHITECT**<br>*(Major only)* | `discovery-and-ambiguity-log` | Phase 1 — assumptions, SLO/SLA, Ambiguity Log |
| | `design-system-architecture` | Phases 2–3 — topology, resilience, contract-first APIs, persistence, migrations |
| | `design-capability-layout` | Phase 4 — slice layout, functional core / imperative shell, declared Ports |
| | `create-adr` | A decision binds future work; alternatives table required; must reach `status: Accepted` |
| | `define-seam-contract` | A boundary another slice builds against; Shape + Behavior suite + frozen `<name>@vN` |
| **BUILD** | `create-sprint` | Lock the bridge: slice intent + current-state snapshot + hypothesis card + test plan |
| | `intake-code-contribution` | Phase 0 gate — **mandatory before any code change** |
| | `test-by-ownership` | Right layer per behavior: Logic → Composition → Adapter Contract → Integration → Journey |
| | `implement-with-defensive-patterns` | Phase 5 — composition over inheritance, shift-left security, structured telemetry |
| | `verify-and-assemble-pr` | Phase 6 — captured `verify` output, refactor matrix, PR narrative + rollback |
| **LEARN** | `close-sprint` | Outcome evidence + continue/pivot/stop into product AND engineering artifacts; deletes the sprint |
| | `ingest-operational-feedback` | Process incident post-mortems, operator friction logs, and SLO reviews into `CAP.` invariants |
| **TEACH** | `author-user-docs` | Validated capability record → Diátaxis guides in `docs/guides/` (product-designer-owned) |

---

## Enforcement model — what is mechanical vs. trusted

Three kinds of gate. Knowing which is which is the difference between a guarantee and a good intention:

| Gate kind | Examples | Fails closed? |
|---|---|---|
| **Script-enforced** — the `check-*.sh` probes | anti-dumping, test hygiene, Port/Adapter + seam parity, the four readiness anchors | Yes, once wired into CI or a git hook. Several are warn-first until you set `mode: enforce` |
| **Human-signed** — a person fills an approval line | Sprint Plan Approval, Design Approval | Blocks the next skill — but only a compliant agent checks |
| **Agent-attested** — you, following the skill | tier classification, intake envelope, four-anchor conformance, red-first posture, adversarial seam review | **No.** Trusted, not compelled |

**"Mechanical" means script-checkable, not runtime-enforced.** A probe verifies an artifact exists and carries substance; it cannot stop you proceeding. The lower two tiers rest on your compliance — honor them anyway, and never describe a gate as stronger than it is.

Exception: `check-design-approval-gate.sh` **hard-fails** a Major-tier sprint whose ADR is not `Accepted` or whose Design Approval is unsigned.

Probes live in `scripts/` — wire them into the project's `verify` entry point. Full table in `README.md`; rationale in `docs/product.md`.

---

## Emergent parallelism — the four-condition disjointness rule

Praxis never schedules parallel work. Parallelism is an **emergent permission**, exercised by the human or an orchestration runtime. Two units may run concurrently **only if all four hold**:

1. **Capability/file disjoint** — no source file or capability in common.
2. **Persistent-resource disjoint** — no shared table, topic, queue, cache, or migration.
3. **Config-key disjoint** — no shared configuration key.
4. **Frozen-contract dependent** — each depends only on a frozen `<name>@vN` seam contract, never on the other's in-flight internals.

Capability-disjointness **alone is not sufficient**. If any condition fails, the units are sequential.

---

## Precedence

```
repo .github/copilot-instructions.md, .claude/CLAUDE.md   (highest — project owns final word)
repo .github/instructions/*.instructions.md (scoped)
repo .github/agents/, .github/skills/
─────────────────────────────────────────────────────────
plugin instructions, agents, skills                       (defaults — Praxis)
─────────────────────────────────────────────────────────
user ~/.claude/CLAUDE.md, VS Code user prompts            (personal preferences only)
```

Praxis sets defaults; it never claims the final word. Praxis is artifact discipline, not runtime orchestration — delegation, ticketing, and branch protection belong to a runtime such as Claude MPM (see `docs/product.md`).

---

## Quick verification

Ask the human partner:

> "Tell me about your praxis."

If you can name the three personas, the three always-on guardrail sets, and at least four skills with their triggers, the bootstrap is loaded correctly.
