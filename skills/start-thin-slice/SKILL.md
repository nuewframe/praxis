---
name: start-thin-slice
description: >
  Entry point and triage gate for taking on a thin-slice by ID ("Work on TS-NNN"). Checks
  dependency and status preconditions, runs a provisional tier classification by reference to
  intake Step 0, runs a lightweight ambiguity log + pre-mortem for Standard work, and
  deterministically routes to create-sprint vs. the architect path. Produces a triage record
  that create-sprint consumes.
user-invocable: true
disable-model-invocation: false
---

# Skill: Start Thin-Slice

Use this skill at the **front door** of any slice work — when the human says "Work on TS-NNN", "Start AG-006", "Take the next slice", or similar. It is the router that decides which skill runs first.

This skill triages and routes—it does **not** implement or write production code.

---

## Step 1 — Locate the Slice

Resolve the slice ID (`TS-NNN`) in the initiative file (`docs/product/initiatives/INIT.<initiative-name>.md`) or roadmap index in [`docs/product.md`](../../docs/product.md).

- If the slice ID does not exist, **stop**. Ask whether to create it via `create-wave` / `INIT.<name>.md`, or whether the ID was mistyped.
- Read its title, status, dependencies, and acceptance criteria.

---

## Step 2 — Precondition Checks (Hard Gate)

Both checks must pass before routing:

1. **Status check:**
   - `⚪ Not Started` → proceed.
   - `🔄 In Progress` → confirm whether to resume (a sprint or ledger may already exist).
   - `✅ Complete` → this is a **correction or reopen**. Confirm with the human, then **keep the same slice ID** (`TS-NNN`).
   - `🚫 Blocked` / `⚠️ At Risk` → surface the reason; do not route until acknowledged.

2. **Dependency check:**
   - Every dependency the slice declares must be `✅ Complete`.
   - If any dependency is unmet, **stop** and report the blocking slice(s).

---

## Step 3 — Provisional Tier Classification

Classify the slice into **Trivial**, **Standard**, or **Major** by reference to `intake-code-contribution` Step 0. Output the provisional tier with the deciding reason.

---

## Step 4 — Ambiguity Log & Pre-Mortem (Standard Tier)

For Standard-tier slices, run a time-boxed clarity pass (Ambiguity Log + 3-row Pre-Mortem Risk Register) to seed `create-sprint`.

---

## Step 5 — Deterministic Routing

Route by provisional tier per the canonical spine table:
- **Trivial:** → `intake-code-contribution` → `implement-with-defensive-patterns` → `verify-and-assemble-pr`. No sprint.
- **Standard:** → `create-sprint` → wait for Sprint Plan Approval → `intake-code-contribution` → `implement-with-defensive-patterns` → `verify-and-assemble-pr`.
- **Major:** → `discovery-and-ambiguity-log` → `design-system-architecture` → `design-capability-layout` → `create-adr` (`status: Accepted`) → `create-sprint` → wait for Design & Sprint Plan Approvals → `intake-code-contribution` → `implement-with-defensive-patterns` → `verify-and-assemble-pr`.

---

## Step 6 — Emit Triage Record

Emit a compact triage record stating the slice ID, precondition status, provisional tier, and deterministic route.
