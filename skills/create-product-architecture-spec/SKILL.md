---
name: create-product-architecture-spec
description: >
  Author or refine a planning-stage wave `product-architecture.md`. Translates wave intent into
  a buildable wave-level technical design — domain ownership, contracts, data flow, security,
  third-party integrations, configuration, and failure behavior — that guides sprint planning
  without becoming an execution backlog.
user-invocable: true
disable-model-invocation: false
---

# Skill: Create Product Architecture Spec

Use this skill when authoring or updating a wave's `product-architecture.md`.

**Audience:** Principal engineer, product designer, product manager. **Purpose:** Translate wave intent into a buildable technical specification that guides sprint planning and implementation without becoming an execution backlog.

---

## Skill Boundaries

This skill is **wave-scoped**. It documents how one wave fits into the existing system.

For broader concerns, pair this skill with:

| Concern                                                                     | Pair with                     |
| --------------------------------------------------------------------------- | ----------------------------- |
| New subsystem, runtime boundary, or platform-wide constraint                | `design-system-architecture`  |
| Per-capability folder layout and functional core / imperative shell mapping | `design-capability-layout`    |
| Durable technical decision the wave makes                                   | `create-adr`                  |
| Phase-1 unknowns that must be resolved before architecture can land         | `discovery-and-ambiguity-log` |
| The living, current-state architecture of a capability                      | capability record — `docs/architecture/<capability>/` |
| The cross-capability topology and product-wide posture                      | system overview — `docs/architecture/README.md` |

---

## Wave = Bet, Not Record

This document is the wave's **hypothesis** — the technical shape you *bet* will deliver the wave's intent, drawn at planning time, before you have built or measured anything. It is provisional and intent-coupled by design.

It is **not** the durable architecture record. The living, validated current-state lives in the durable architecture tree:

- **Capability record** (`docs/architecture/<capability>/`) — the truth for one capability.
- **System overview** (`docs/architecture/README.md`) — cross-capability topology + product-wide posture.
- **ADRs** (`docs/architecture/<capability>/adr/`) — immutable decisions.

So this spec is **forward-looking and pointer-heavy**: it names the bet, the seams it introduces, and the decisions that will need ADRs — and it **points into** the durable records rather than duplicating their current-state topology. When a sprint validates the bet, `close-sprint` promotes the learning into the capability record and system overview. **Wave = bet; capability record = truth.**

The moment this document starts describing current-state topology as settled fact, it has drifted into the capability record's job — link out instead.

---

## What This Skill Produces

A planning-stage wave-level technical architecture **hypothesis** — the bet. Not implementation code. Not a sprint execution plan. Not the durable architecture record (that is the capability record + system overview, promoted on `close-sprint`). The wave README remains the only place that tracks thin-slice status or correction notes.

A strong `product-architecture.md` answers:

- How does this wave fit into the existing architecture?
- Which domains own which responsibilities?
- What interfaces, routes, data contracts, and persistence changes are required?
- What third-party providers does the wave depend on, and how are they abstracted?
- How do data and control move through the system?
- What are the security, privacy, resilience, and rollout constraints?
- What configuration, environment, and deployment changes are needed for a healthy deployable?
- Which decisions require an ADR?

---

## Step 1 — State the Architectural Intent

Open with a concise summary that explains:

- What technical capability this wave adds
- How it fits the existing architecture
- What it explicitly does **not** introduce

Anchor the design to project principles: capability ownership, thin handlers, existing persistence, human approval boundaries.

---

## Step 2 — Define the Governing Principles

List the architectural rules that constrain the design. Examples:

- Keep domain ownership inside existing capability boundaries
- Reuse the existing primary datastore as source of truth
- Treat external providers as stateless adapters
- Make every AI-assisted path fall back to a manual path
- Do not allow external tools to publish live state without in-app approval

---

## Step 3 — Assign Domain Ownership

For each affected domain, specify:

- What it owns
- What it exposes
- What it must not own
- How it collaborates with other domains

If a new cross-cutting subsystem appears, define whether it is a domain capability, a shared adapter, or a separate service. If two domains both "sort of own" a concern, the document is not ready.

---

## Step 4 — Specify the Contracts

Document the technical contracts implementation must satisfy:

- HTTP routes / RPC endpoints and their purpose
- Request and response DTO expectations
- Domain service responsibilities
- Repository and persistence implications
- External provider interactions
- Event stream or push semantics if the user experience depends on them

Type and interface sketches are allowed when they reduce ambiguity. Keep illustrative, not exhaustive.

### Declare the wave's seams

A **seam** is any boundary this wave introduces or changes where one unit depends on another's promise — a public HTTP/RPC API, a published event, a capability ↔ adapter Port, or a CLI contract. Seams are where the runtime anchors (observable, configurable, scalable, resilient) actually live, so the cross-boundary view must get an owner **here, before slices fork** — otherwise each slice re-decides the boundary in isolation (the boundary-blind parallelism failure).

For each seam the wave introduces or changes, name:

- **Seam name + frozen contract id** — `<name>@vN` (e.g. `publish-api@v1`). The version is frozen *before* the producer's internals are done, so a dependent slice can build against the promise instead of waiting for the producer slice.
- **Kind** — `http` | `event` | `port` | `cli`.
- **Producer and consumer(s)** — who makes the promise, who depends on it.
- **Shape** — where the machine-readable interface lives (OpenAPI / JSON-Schema / typed Port).
- **Behavior suite** — where the shared contract test suite lives.

Define each seam's contract with the `define-seam-contract` skill, which records it in `.seam-contracts.json` and is enforced by `check-seam-contract-parity.sh`. List the wave's seams in this document so sprint planning can point each slice at a frozen `<name>@vN` rather than at another in-flight slice.

---

## Step 5 — Show the Data and Control Flow

Make motion visible. Use diagrams or stepwise flows for:

- User action to system response
- Sync vs async boundaries
- External provider calls
- Approval gates
- Read/write boundaries against persistence

ASCII diagrams are preferred when a flow spans multiple components.

---

## Step 6 — Define Third-Party Dependencies and Integration Strategy

For every external provider, SDK, or service:

- Purpose: what capability it enables
- Abstraction boundary: adapter interface that isolates vendor-specific code from domain logic
- Data contract: what is sent and received
- Timeout, retry, and fallback behavior
- Cost, quota, or rate-limit constraints
- Vendor lock-in avoidance: what changes if the provider is swapped

If the provider selection is durable, pair with `create-adr`. If the wave introduces no external dependencies, state that explicitly.

---

## Step 7 — Cover Failure, Security, and Operations

Required coverage:

- Authentication and authorization
- Rate limiting and abuse controls
- Privacy and context minimization
- Timeouts, retries, and fallback paths
- Stale data and concurrency risks
- Logging, monitoring, and operational signals
- Deployment or environment changes if applicable

If the design assumes manual fallback, describe the fallback precisely.

---

## Step 8 — Define Configuration and Deployment Strategy

Required coverage:

- Environment variables, feature flags, and secrets introduced by this wave
- Configuration per environment
- New infrastructure or service dependencies
- Health check and readiness criteria
- Rollout strategy and rollback path
- Observability: structured logs, metrics, alerts

If the wave changes nothing about configuration or deployment, state that explicitly.

---

## Step 9 — Declare the Production-Readiness Posture (Four Anchors)

The four runtime anchors — **observable, configurable, horizontally scalable, resilient** — are *cross-boundary* properties that live at seams, not inside any one slice. Decide them **once, here, with the whole wave in view**, so each slice *conforms to* a central posture instead of re-deciding it in isolation (the boundary-blind failure mode). Each anchor maps to an executable probe that holds slices to this posture.

For each anchor, state the wave's posture:

- **Observable** — the correlation/trace contract: which id propagates across every boundary call, what each new boundary call must log (structured), and the metric(s) a new boundary emits. Probe: `check-observability-at-seams.sh`.
- **Configurable** — the config & secrets strategy: what is environment-injected vs. compiled in, where secrets come from, and what must never appear as a source literal. Probe: `check-config-externalized.sh`.
- **Horizontally scalable** — the statelessness boundary: where request-scoped state may live and where it may **not** (no node-local mutable state on the request path), and where shared state is externalized (datastore, cache, queue). Probe: `check-stateless-request-path.sh`.
- **Resilient** — the cross-slice failure model: timeout / retry / fallback defaults for every external or boundary call, and the degraded behavior the user sees when a dependency is down. Probe: `check-resilient-boundary.sh`.

This posture is the spine every sprint's **Production-Readiness conformance** block (in `create-sprint`) points back to — slices *preserve* it, they do not re-litigate it. If the wave genuinely changes one anchor's posture from the platform default, say so explicitly here and link an ADR when the change is durable.

---

## Step 10 — Record ADR Triggers and Open Constraints

A wave architecture doc references decisions; durable technical policy belongs in ADRs. Create an ADR via `create-adr` when the wave introduces:

- A new runtime or deployable service
- A new trust boundary or auth model
- A new persistence or messaging strategy
- A provider selection strategy
- A long-lived platform rule that will outlast the wave

---

## Step 11 — Use This Structure

```markdown
# [Wave Name]: Product Architecture

> **Wave**: wave-[category]-[name]\
> **Version**: 1.0.0\
> **Updated**: YYYY-MM-DD

## Overview

[1-3 paragraphs describing the technical approach and key constraints.]

## Architectural Principles

### [Principle Name]

[Why it exists and how it shapes the design.]

## System Architecture

\`\`\`text [Context or flow diagram] \`\`\`

> Point into the durable record for current-state: capability record(s) at `docs/architecture/<capability>/` and the system overview at `docs/architecture/README.md`. Sketch here only the *change* this wave bets on.

## Domain Structure

[Affected domains, new modules, responsibility boundaries.]

## Component Details

### 1. [Capability or subsystem]

**What it is**: [Purpose] **Contracts**: [Routes, interfaces] **Flow**: [Stepwise explanation or diagram] **Failure modes**: [Failure mode and behavior]

## Seam Contracts

| Seam (`<name>@vN`) | Kind | Producer | Consumer(s) | Shape | Behavior suite |
| ------------------ | ---- | -------- | ----------- | ----- | -------------- |
| `publish-api@v1`   | http | [domain] | [domain]    | [path to OpenAPI] | [path to `*.contract.test.*`] |

[Or note that this wave introduces no new seams.]

## Production-Readiness Posture

| Anchor | Wave posture | Probe |
| ------ | ------------ | ----- |
| Observable | [correlation id + what each boundary logs/meters] | `check-observability-at-seams.sh` |
| Configurable | [env-injected vs. compiled; secret source] | `check-config-externalized.sh` |
| Horizontally scalable | [where request-scoped state may/may not live] | `check-stateless-request-path.sh` |
| Resilient | [timeout/retry/fallback defaults; degraded UX] | `check-resilient-boundary.sh` |

## Security Considerations

- [Auth requirement]
- [Rate limit]
- [Privacy rule]

## Third-Party Dependencies and Integration Strategy

### [Provider Name]

**Purpose**: [Capability enabled] **Abstraction**: [Adapter interface] **Data contract**: [What is sent and received] **Failure behavior**: [Timeout, retry, fallback, degraded mode] **Cost and quota constraints**: [Or "none"]

## Configuration and Deployment Strategy

- [Environment variables and secrets]
- [Feature flags if applicable]
- [Per-environment differences]
- [Health check and readiness criteria]
- [Rollout and rollback path]
- [Observability: logs, metrics, alerts]

## ADR Reference

- [Link to ADRs in `docs/architecture/<capability>/adr/`, or note which durable decisions this wave will require an ADR for]

## Durable Architecture Pointers

- Capability record(s): `docs/architecture/<capability>/`
- System overview: `docs/architecture/README.md`
```

---

## Quality Checklist

- [ ] Spec explains how the wave works technically, not how to code it line by line
- [ ] Domain ownership is explicit and non-overlapping
- [ ] User-visible flows from `product-design.md` are supported by concrete contracts
- [ ] Every seam the wave introduces or changes is named with a frozen `<name>@vN` id, kind, producer, consumer(s), Shape, and Behavior suite
- [ ] The Production-Readiness posture is stated for all four anchors (observable, configurable, horizontally scalable, resilient) so slices conform rather than re-decide
- [ ] Every third-party dependency has an abstraction boundary, timeout, fallback, and data-minimization rule
- [ ] Configuration, environment variables, and secrets are documented per environment
- [ ] Deployable health criteria are concrete enough to verify in CI or monitoring
- [ ] Failure, fallback, and stale-data behavior are described
- [ ] Security and privacy constraints are explicit
- [ ] Any durable architectural decision is linked to an ADR or flagged for one
- [ ] The document guides sprint planning without becoming a sprint file

---

## Anti-Patterns

- Turning the document into a task checklist or implementation backlog
- Duplicating thin-slice tracking or correction history (belongs in the wave README)
- Repeating the UX narrative instead of defining the technical shape
- Hand-waving domain ownership ("shared between everything")
- Naming specific code changes without explaining the architectural reason
- Omitting failure behavior because it "will be handled in code"
- Using third-party providers directly without an abstraction layer
- Omitting configuration and deployment strategy because it is "obvious"
- Assuming environment variables and secrets exist without documenting them
