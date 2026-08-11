---
name: create-capability-record
description: >
  Scaffold or update a living capability record (CAP.<capability-name>.md) in docs/capabilities/.
  Serves as the single living source of truth per domain for UX journeys, architecture, seam contracts (<name>@vN), and quality invariants.
user-invocable: true
disable-model-invocation: false
---

# Skill: Create Capability Record (`CAP.`)

Use this skill to create or update a living capability record (`docs/capabilities/CAP.<capability-name>.md`).

**Audience:** Principal Engineer, Product Designer, Product Manager.  
**Purpose:** Establish and maintain the single living current-state source of truth per domain. Unlike initiative files (`INIT.`) which capture intent and educated theory, a capability record (`CAP.`) represents **validated current truth**.

---

## What This Skill Produces

1. A living capability record file: `docs/capabilities/CAP.<capability-name>.md`
2. An index entry in the capabilities inventory section of [`docs/product.md`](../../docs/product.md)

---

## Capability Record Structure (`CAP.<capability-name>.md`)

```markdown
# CAP.<capability-name>: [Capability Title]

**Domain Owner:** [Squad / Owner Lead]  
**Status:** Active | Emerging | Deprecated  
**Seam Contracts:** `<contract-name>@vN` ([contracts/contract-<name>.json](file:///path/to/contract))  

---

## 1. Domain Description & Bounded Context

- **Core Purpose:** [What business/technical domain this capability owns]
- **Bounded Context:** [Explicit boundaries; what this capability owns vs delegates]
- **Producer / Consumer Relationships:**
  - *Consumes:* `CAP.<other-cap>` via `<contract>@vN`
  - *Provides:* `<contract>@vN` to `CAP.<consumer-cap>`

---

## 2. User Experience & Living Journeys

- **User Personas Served:** [Target personas]
- **Primary User Journeys:** Entry point → Trigger → State → Completion
- **State Transition Matrix:**
  | Current State | Trigger / Event | Next State | Error / Recovery |
  | ------------- | --------------- | ---------- | ---------------- |
  | Idle | Submit Form | Validating | Highlight invalid fields |
- **Accessibility & UX Standards:** [WCAG level, keyboard navigation, focus management]

---

## 3. Technical Architecture & System Topology

- **Architecture Layout:** [Module / directory location in codebase]
- **Functional Core vs. Imperative Shell:**
  - *Functional Core (Pure Logic):* Domain models, calculations, state transitions
  - *Imperative Shell (Side Effects):* Persistence adapters, external API wrappers, event publishers
- **Ports & Adapters Interfaces:**
  - Primary Port (Driver): Command/Query handler interface
  - Secondary Port (Driven): Repository / Service Client interfaces

---

## 4. Seam Contracts & Integration Interfaces

- **Contract Version:** `<contract-name>@vN`
- **Schema Reference:** `.seam-contracts.json` / OpenAPI / Protobuf
- **Compatibility Contract:** [Backward compatibility rules, version migration path]

---

## 5. Quality & NFR Invariants

- **Test Layer Mapping:**
  - *Unit / Logic Tests:* Functional core coverage
  - *Adapter Contract Tests:* Port interface compliance
  - *Integration Tests:* Persistence and external boundary checks
  - *Journey Tests:* End-to-end UX flow validation
- **4 Production Readiness Anchors:**
  - *Observable:* Structured log fields, metrics exported, trace context propagated
  - *Configurable:* Externalized settings, dynamic flags
  - *Scalable:* Resource bounds, concurrency limits
  - *Resilient:* Timeouts, retries with jitter, fallbacks, circuit breakers

---

## 6. Capability History & Lineage

- **Initiatives Delivered:**
  - [INIT.<initiative-1>](../product/initiatives/INIT.<initiative-1>.md) — [Summary of additions]
- **Durable Decisions:**
  - [ADR.<YYMMDD>.<seq>](../architecture/adr/ADR.<YYMMDD>.<seq>.md) — [Decision summary]
```

---

## Step 1 — Scaffold the Capability Record

Create `docs/capabilities/CAP.<capability-name>.md` using the template structure above.

---

## Step 2 — Index on Product Dashboard

Register the capability record in the capability index section of [`docs/product.md`](../../docs/product.md):

```markdown
- [CAP.<capability-name>](capabilities/CAP.<capability-name>.md) — [Title & domain summary]
```

---

## Step 3 — Maintain via Sprint Distillation (`close-sprint`)

When sprints complete, `close-sprint` distills validated learnings from initiatives into this file:
- Update technical architecture topology with delivered code structure.
- Update UX state transitions with delivered flows.
- Update quality invariants with verified NFR baselines.
- Append delivered initiative link under Capability History.

---

## Quality Checklist

- [ ] File follows `CAP.<capability-name>.md` naming prefix
- [ ] Registered on [`docs/product.md`](../../docs/product.md)
- [ ] Contains domain ownership and bounded context definition
- [ ] Functional Core / Imperative Shell separation explicit
- [ ] Seam contracts tagged with version `<name>@vN`
