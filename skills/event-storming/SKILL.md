---
name: event-storming
description: >
  Upstream domain discovery skill. Guides domain event storming from raw business requirements to bounded contexts,
  domain commands, aggregate boundaries, candidate capabilities, and the first thin-slices cut from the storm.
user-invocable: true
disable-model-invocation: false
---

# Skill: Event Storming (`skills/event-storming/`)

Use this skill when exploring greenfield business requirements, discovering new product domains, or refactoring unmapped legacy systems into bounded contexts.

**Audience:** Product Manager, Product Designer, Principal Engineer, Domain Experts.  
**Purpose:** Bridge business requirements to capability boundaries before any slice is cut. Prevents arbitrary feature grouping by deriving capabilities from business events and domain aggregates.

---

## What This Skill Produces

1. **Domain Event Map:** Visual/textual timeline of domain events, triggers, commands, and read models.
2. **Bounded Context & Candidate Capabilities:** Candidate capability records, one per cluster, cut with `name-a-capability`.
3. **The First Slices:** thin-slices cut from the storm with `cut-a-slice`, each realizing exactly one of those capabilities.

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

Use `name-a-capability` to cut the candidate capability records. A capability that cannot pass its four gate tests is not one.

---

## Step 5 — Cut the First Slices (`TS.`)

Turn high-value user outcomes into slices, each realizing exactly one capability:
- Use `cut-a-slice`. A slice names its command or its view, the capability it realizes, and the storm it came from.
- Give each one a `scenario`, an `excludes`, and claims that name the evidence settling them — the shape check refuses a slice without them.
- `praxis ready` then says which of them could be started now, and what would refuse the rest.

---

## Quality Checklist

- [ ] Domain events captured in past tense (`OrderPlaced`, not `PlaceOrder`)
- [ ] Triggers (user commands, external webhooks, policies) explicit
- [ ] Bounded context boundaries translate 1-to-1 with candidate `CAP.` records
- [ ] No arbitrary implementation folders created before capability boundaries are declared
