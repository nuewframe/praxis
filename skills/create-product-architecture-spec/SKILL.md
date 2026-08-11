---
name: create-product-architecture-spec
description: >
  Author or refine technical architecture specifications in an initiative (INIT.<name>.md) or living capability record (CAP.<name>.md).
  Specifies domain boundaries, seam contracts (<name>@vN), Functional Core / Imperative Shell topology, Ports/Adapters, and failure modes.
user-invocable: true
disable-model-invocation: false
---

# Skill: Create Product Architecture Spec

Use this skill when authoring or refining technical architecture specifications.

**Audience:** Principal Engineer, Technical Lead, System Architect. **Purpose:** Define domain boundaries, seam contracts, Ports/Adapters topology, and data flows so implementation can proceed safely without architectural drift.

---

## What This Skill Produces

Refines the **Technical Architecture (Seams & Educated Theory)** section inside an initiative file (`docs/product/initiatives/INIT.<initiative-name>.md`) or promotes validated technical architecture into a living capability record (`docs/capabilities/CAP.<capability-name>.md#technical-architecture`).

**Initiative Architecture = Educated Theory; Capability Record = Living Truth.**  
Architecture notes inside an initiative file represent hypotheses. Upon sprint completion, `close-sprint` distills validated architecture into `docs/capabilities/CAP.<capability-name>.md`.

---

## Required Inputs

Read before drafting:
- [`docs/product.md`](../../docs/product.md) — Unified product context
- [`docs/architecture/README.md`](../../docs/architecture/README.md) — Global system topology & NFRs
- Target initiative: `docs/product/initiatives/INIT.<initiative-name>.md`
- Target capability records: `docs/capabilities/CAP.<capability-name>.md`
- Active ADRs in `docs/architecture/adr/`

---

## Step 1 — Declare Domain Ownership & Seam Contracts

Identify capabilities touched and seam boundaries required:

```markdown
### Seam Contract Declaration

- **Target Capabilities:** `CAP.<capability-A>`, `CAP.<capability-B>`
- **Seam Contract Tag:** `<contract-name>@vN`
- **Producer Unit:** `CAP.<capability-A>`
- **Consumer Unit:** `CAP.<capability-B>`
- **Contract Schema Path:** `.seam-contracts.json` / `contracts/<contract-name>.v1.json`
```

If seam boundaries are new or modified, invoke `define-seam-contract` to register them in `.seam-contracts.json`.

---

## Step 2 — Specify Functional Core vs. Imperative Shell Topology

Define the architectural breakdown isolating pure domain logic from side effects:

```markdown
### Architectural Breakdown

#### 1. Functional Core (Pure Domain Logic)
- **Pure Entities & Models:** `Initiative`, `ThinSlice`, `CapabilityRecord`
- **State Transition Rules:** Validates state changes without I/O or DB access
- **Invariants Enforced:** Zero side-effects; input values $\rightarrow$ output result

#### 2. Imperative Shell (Side Effects & Adapters)
- **Primary Adapters (Drivers):** HTTP Controller, CLI Command Handler, Event Subscriber
- **Secondary Adapters (Driven):** Postgres Repository, File System Storage, HTTP Client
- **Port Interfaces:**
  - `InitiativeRepositoryPort` (Save, FindByID, ListActive)
  - `NotificationPublisherPort` (PublishEvent)
```

---

## Step 3 — Define Sequence & Component Integration Flows

Map component interactions for critical flows:

```
[Driver: CLI / HTTP] 
        │ (Command DTO)
        ▼
[Port Handler: App Service] ──(Queries State)──► [Functional Core Model]
        │                                             │ (Validates Rules)
        │ (Uses Port Interface)                       ▼
        ▼                                      [Updated Domain Model]
[Driven Adapter: Repository / DB]
```

---

## Step 4 — Specify Failure Modes & Structural Invariants

Detail technical failure handling:
- **Database Boundary Failures:** Connection pool exhaustion handling, transaction rollback rules.
- **External Integration Failures:** Timeout limits, retry policy with jitter, fallback defaults.
- **Seam Contract Mis-matches:** Schema validation at adapter boundary; fast-fail on invalid payloads.

---

## Step 5 — Update the Target Document

1. **For Initiative Technical Educated Theory:** Update `## Progressive Refinement -> Technical Architecture` in `docs/product/initiatives/INIT.<initiative-name>.md`.
2. **For Global Topology:** Update [`docs/architecture/README.md`](../../docs/architecture/README.md).
3. **For Durable Decisions:** Create an ADR via `create-adr` (`docs/architecture/adr/ADR.<YYMMDD>.<seq>.md`).
4. **For Living Capability Architecture:** Promote validated architecture into `docs/capabilities/CAP.<capability-name>.md#technical-architecture` at sprint close.

---

## Quality Checklist

- [ ] Domain ownership and bounded context unambiguous
- [ ] Seam contracts explicitly declared with version tag `<name>@vN`
- [ ] Pure logic isolated from side-effects (Functional Core / Imperative Shell)
- [ ] Primary (driver) and secondary (driven) port interfaces defined
- [ ] Anti-dumping rules enforced (`utils/`, `helpers/` forbidden)
