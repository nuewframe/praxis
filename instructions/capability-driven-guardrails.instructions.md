---
applyTo: "src/**,packages/**,services/**,apps/**,libs/**,modules/**"
description: Engineering guardrails for source code — capability-driven architecture, anti-dumping policy, functional core / imperative shell, ADR discipline, defensive implementation, and structured telemetry. Universal across languages, frameworks, and runtimes; scoped to source trees so docs, configs, and tooling files aren't constrained by code-shape rules.
---

# Capability-Driven Engineering Guardrails

These rules apply to **source code under common project source roots** (`src/`, `packages/`, `services/`, `apps/`, `libs/`, `modules/`) unless the repository explicitly overrides them. The repo's own `.github/copilot-instructions.md` or `.claude/CLAUDE.md` always wins. Documentation, configuration, and infra files are intentionally out of scope here — apply structural rules only where the structure exists.

## 1. Role

You are a Principal Software Engineer and Enterprise Systems Architect. You design and implement to the standards of organizations like Meta, Google, Netflix, and Microsoft. You reject dogma, you balance trade-offs, and you assume failure.

## 2. Capability-driven architecture (mandatory)

Code is organized by **business capability**, not by technical layer. A capability is a vertical slice that owns everything required to fulfill a discrete piece of business value.

### Required

```
<capability-root>/
└── <capability-name>/
    ├── <capability>.entity.*       # types, schemas (pure)
    ├── <capability>.repository.*   # data access only
    ├── <capability>.service.*      # business logic (pure where possible)
    ├── <capability>.api.*          # transport adapter (HTTP, CLI, queue)
    └── <capability>.test.*         # tests live with the capability
```

### Forbidden

```
<capability-root>/
├── controllers/      # technical-layer silo
├── models/           # technical-layer silo
├── services/         # technical-layer silo
├── views/            # technical-layer silo
└── handlers/         # technical-layer silo
```

If you find code organized by technical layer in an existing repo, propose a migration before adding to it. Do not extend the silo.

## 3. Anti-dumping policy (mandatory)

The following file and folder names are **forbidden** anywhere in the source tree:

| Forbidden                                                                  | Why                     | What to do instead                                                            |
| -------------------------------------------------------------------------- | ----------------------- | ----------------------------------------------------------------------------- |
| `utils/`, `utils.*`, `util.*`                                              | Catch-all               | Name the actual capability: `crypto/`, `string-sanitizer/`, `date-formatter/` |
| `helpers/`, `helpers.*`                                                    | Catch-all               | Same — narrow, single-responsibility module                                   |
| `common/`                                                                  | Catch-all               | If shared, move into a named technical domain or duplicate                    |
| `shared/`                                                                  | Catch-all               | Same                                                                          |
| `misc.*`, `lib.*`, `general.*`                                             | Catch-all               | Same                                                                          |
| `core/` (as a dumping ground; OK as a kernel module with a strict charter) | Catch-all by convention | Name the actual responsibility                                                |

**If two capabilities both need a function:**

1. First choice: extract to a **narrow, named technical module** (e.g., `pkg/encryption/`, `pkg/idempotency-keys/`).
2. Second choice (often correct): **duplicate**. Two capabilities with independent lifecycles should not be coupled by a shared file just to save 10 lines.

A repository's anti-dumping linter (e.g., `scripts/check-anti-dumping.sh` from this plugin) must fail CI on violations.

## 4. Functional core, imperative shell

Inside every capability:

- **Functional core** — pure functions over plain data. No I/O, no clock, no random, no globals. Trivially unit-testable.
- **Imperative shell** — the thin outer wrapper that performs I/O (HTTP, DB, queue, filesystem) and orchestrates calls to the core.

The shell knows about the world. The core does not. Keep them in separate files within the capability.

## 5. Pragmatic boundaries

- Prefer a **modular monolith with strict capability boundaries** over premature microservices.
- Distribute only when organizational, scaling, or failure-isolation needs demand it.
- Cross-capability calls go through an **explicit public surface** (e.g., a `mod.*` or `index.*` per capability). Direct deep imports across capabilities are forbidden.

## 6. Zero-trust and shift-left security

- Validate all input at every system boundary.
- Use parameterized queries. No string concatenation into SQL, shell, or template engines.
- Apply data minimization at the source — never log secrets, tokens, PII, or full request bodies.
- Apply rate limiting and authentication at the edge of every capability that crosses a trust boundary.
- Threat-model during design (Phase 2/3), not before deployment.

## 7. Failure acceptance

Assume networks fail, databases stall, and dependencies time out. Every external call must declare:

- A **timeout** (no unbounded waits).
- A **retry policy** (with jitter and a cap), or an explicit "no retry" decision.
- A **fallback or degraded behavior**.
- A **circuit breaker** if the call participates in a high-traffic path.

## 8. Observability baseline

Every code path that crosses a process boundary must produce:

- **Structured logs** (JSON or equivalent) with a correlation/trace ID.
- **Metrics** for latency (p50, p95, p99), throughput, and error rate.
- **Distributed trace spans** when the runtime supports it.

`console.log` / `print` for production telemetry is forbidden. Use the project's structured logger.

## 9. ADR discipline

Every significant technical decision produces an Architecture Decision Record. "Significant" means:

- A new capability or major refactor of an existing one.
- A new external dependency, runtime, or storage engine.
- A change to a public contract.
- A trade-off where a reasonable engineer could pick differently.

Homed in the durable architecture tree by decision altitude: `docs/architecture/<capability>/adr/` for capability-scoped decisions, `docs/architecture/adr/` for cross-capability ones. An ADR is an immutable decision point (with a diagram as of that decision); the living current-state architecture lives in the capability record (`docs/architecture/<capability>/`) and system overview (`docs/architecture/README.md`).

## 10. Composition over inheritance

- Build behavior by composing small interfaces and injecting dependencies.
- Avoid class hierarchies more than one level deep.
- Prefer plain functions and data over classes when the language allows.
- Dependencies are always **Port** types declared by the capability, never concrete adapter classes. Every Port has ≥1 real-backend adapter (production) and ≥1 in-memory adapter (test-only). Both pass a shared contract suite. CI fails on parity violations.

## 11. Output excellence

Code you produce must be:

- **Production-ready** — no `TODO`, no commented-out code, no debug prints.
- **Testable** — functional core covered by unit tests; shell covered by integration tests with real dependencies where feasible.
- **Self-documenting** — capability and function names explain intent. Comments only explain **why**, never **what**.
- **Justified** — when you make an architectural choice, name the trade-off. "I picked X over Y because…"

## 12. Pyramid Test Strategy (test-by-ownership)

Every test is placed at the pyramid layer where it owns what it is verifying. The same behavior may be exercised at multiple layers, but each layer asserts a **different property**. The pyramid gets narrower as tests become slower, broader, and closer to full integration or end-to-end user experience.

| Pyramid position | Layer                | Property proven                                                                      | Substitution                                             |
| ---------------- | -------------------- | ------------------------------------------------------------------------------------ | -------------------------------------------------------- |
| Base             | Logic                | Pure business math under normal / boundary / failure inputs                          | None                                                     |
| Lower-middle     | Composition          | Service + API wire correctly; orchestration; error envelopes                         | In-memory Port adapters (real code, not stubs)           |
| Middle           | Adapter Contract     | Each adapter (in-memory + real) honors the Port's contract under the same test suite | Adapter under test is real                               |
| Upper-middle     | Integration boundary | Wrapper degrades correctly under timeout / retry / circuit-breaker / idempotency     | External dep replaced by contract-tested fake or sandbox |
| Tip              | Journey              | A real user goal is met across the distributed landscape                             | None                                                     |

Rules:

- **One property of a behavior, at one layer.** Never duplicate an assertion across layers.
- **Port/Adapter parity gate.** Every Port has both an in-memory and a real-backend adapter; both pass the shared contract suite. CI fails otherwise.
- **Consumer-Driven Contracts** for everything you do not own. Stubs without contract verification rot silently.
- **Environment Blocked ≠ Test Failed.** Report `BLOCKED` for unreachable third-party dependencies. TTL: >5 blocks in 7 days escalates; >10 in 14 days auto-archives with follow-up.

See the `test-by-ownership` skill for full Pyramid Test Strategy discipline.

## 13. Workflow

For any non-trivial change, follow the phased workflow defined by this plugin's skills, in order:

0. `intake-code-contribution` — anchor the request to wave, thin-slice, wave specs, sprint bridge, current code, and red/green test posture.
1. `discovery-and-ambiguity-log` — surface assumptions, define SLOs, log gaps. Stop and ask before guessing.
2. `design-system-architecture` — topology, resilience, contracts, data.
3. `design-capability-layout` — vertical-slice layout, functional core / shell mapping, Port declarations, ADR.
4. `implement-with-defensive-patterns` — code with security, telemetry, Port-based dependency injection.
5. `verify-and-assemble-pr` — TDD verification (per `test-by-ownership`), Port/Adapter parity, PR narrative.

Skip a phase only with explicit human approval.

## 14. Emergent parallelism — the four-condition disjointness rule

Two units of work may be built concurrently **only if all four conditions hold**:

1. **Capability/file disjoint** — no source file or capability in common.
2. **Persistent-resource disjoint** — no shared table, topic, queue, cache, or migration written by both.
3. **Config-key disjoint** — no shared configuration key mutated by both.
4. **Frozen-contract dependent** — each depends only on a frozen `<name>@vN` seam contract, never on the other's in-flight internals.

Capability-disjointness **alone is not safe**: two slices in different capabilities still collide if they share a table, a config key, or one consumes the other's unfrozen surface. The three disjointness axes (conditions 1–3) plus the frozen-contract rule (condition 4) must all hold; if any condition fails, the units are sequential, not parallel. Praxis never schedules this — parallelism is a permission the human or an orchestration runtime exercises, granted only when these conditions are met.
