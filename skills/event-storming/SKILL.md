---
name: event-storming
description: >
  Upstream domain discovery skill. Guides domain event storming from raw business requirements to bounded contexts,
  domain commands, aggregate boundaries, candidate living capability records (CAP.<name>.md), and initial growth initiatives (INIT.<name>.md).
user-invocable: true
disable-model-invocation: false
---

# Skill: Event Storming (`skills/event-storming/`)

Use this skill when exploring greenfield business requirements, discovering new product domains, or refactoring unmapped legacy systems into bounded contexts.

**Audience:** Product Manager, Product Designer, Principal Engineer, Domain Experts.  
**Purpose:** Bridge business requirements to capability boundaries before any initiative or wave is created. Prevents arbitrary feature grouping by deriving capabilities from business events and domain aggregates.

---

## What This Skill Produces

1. **Domain Event Map:** Visual/textual timeline of domain events, triggers, commands, and read models.
2. **Bounded Context & Candidate Capabilities:** Candidate living capability record skeletons (`docs/capabilities/CAP.<capability-name>.md`).
3. **Initial Growth Initiatives:** Initial initiative files (`docs/product/initiatives/INIT.<initiative-name>.md`) indexed on [`docs/product.md`](../../docs/product.md).

---

## Step 1 — Capture Domain Events (Orange Post-Its)

Discover all significant events that happen in the domain, written in **past tense**:
- *Examples:* `OrderPlaced`, `PaymentProcessed`, `InventoryReserved`, `InvoiceGenerated`, `ShipmentDispatched`.
- Sequence events chronologically from left to right along a business timeline.

---

## Step 2 — Identify Triggers & Commands (Blue Post-Its)

For each domain event, identify what triggered it:
- **User Command (Blue):** Action taken by a user (e.g., `PlaceOrder`, `CancelSubscription`).
- **External System Event (Pink):** Input from a third-party API or webhook (e.g., `StripePaymentReceived`).
- **Business Rule / Policy (Lilac):** Automated reaction (e.g., *Whenever `OrderPlaced` $\rightarrow$ trigger `ReserveInventory`*).

---

## Step 3 — Cluster Domain Aggregates & Bounded Contexts

Group related events, commands, and state rules into **Aggregates** and **Bounded Contexts**:
- **Aggregate Boundary:** Entities and logic that must remain transactionally consistent (e.g., `OrderAggregate`, `InventoryItem`).
- **Bounded Context:** Explicit boundary within which a domain model applies (e.g., *Ordering Domain*, *Fulfillment Domain*, *Billing Domain*).

---

## Step 4 — Map to Living Capabilities (`CAP.`)

Translate bounded contexts directly into Praxis living capability record skeletons:

| Bounded Context | Target Capability Record | Domain Owner | Primary Seam Contract |
| --------------- | ------------------------ | ------------ | --------------------- |
| Ordering Domain | `docs/capabilities/CAP.<order-management>.md` | Checkout Squad | `order-service@v1` |
| Billing Domain | `docs/capabilities/CAP.<billing-and-payments>.md` | Finance Squad | `payment-gateway@v1` |
| Fulfillment Domain | `docs/capabilities/CAP.<fulfillment>.md` | Logistics Squad | `fulfillment-api@v1` |

Use `create-capability-record` to scaffold the candidate `CAP.<name>.md` files.

---

## Step 5 — Scaffold Initial Initiatives (`INIT.`)

Group high-value user outcomes across capability boundaries into single-file initiatives:
- Use `create-initiative` to scaffold `docs/product/initiatives/INIT.<initiative-name>.md`.
- Populate $Iteration_1$ with thin-slices (`TS-001`, `TS-002`) derived from the event storming timeline.
- Register all initial initiatives on [`docs/product.md`](../../docs/product.md).

---

## Quality Checklist

- [ ] Domain events captured in past tense (`OrderPlaced`, not `PlaceOrder`)
- [ ] Triggers (user commands, external webhooks, policies) explicit
- [ ] Bounded context boundaries translate 1-to-1 with candidate `CAP.` records
- [ ] No arbitrary implementation folders created before capability boundaries are declared
