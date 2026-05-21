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

---

## What This Skill Produces

A planning-stage wave-level technical architecture spec. Not implementation code. Not a sprint execution plan. The wave README remains the only place that tracks thin-slice status or correction notes.

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

## Step 9 — Record ADR Triggers and Open Constraints

A wave architecture doc references decisions; durable technical policy belongs in ADRs. Create an ADR via `create-adr` when the wave introduces:

- A new runtime or deployable service
- A new trust boundary or auth model
- A new persistence or messaging strategy
- A provider selection strategy
- A long-lived platform rule that will outlast the wave

---

## Step 10 — Use This Structure

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

## Domain Structure

[Affected domains, new modules, responsibility boundaries.]

## Component Details

### 1. [Capability or subsystem]

**What it is**: [Purpose] **Contracts**: [Routes, interfaces] **Flow**: [Stepwise explanation or diagram] **Failure modes**: [Failure mode and behavior]

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

- [ADR link or note that ADR is required]
```

---

## Quality Checklist

- [ ] Spec explains how the wave works technically, not how to code it line by line
- [ ] Domain ownership is explicit and non-overlapping
- [ ] User-visible flows from `product-design.md` are supported by concrete contracts
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
