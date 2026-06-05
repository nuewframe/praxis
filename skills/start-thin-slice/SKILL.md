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

Use this skill at the **front door** of any slice work — when the human says "Work on TS-NNN", "Start AG-006", "Take the next slice", or similar. It is the router that decides which skill runs first. Without it, "Work on a slice" silently relies on the operator to validate preconditions, classify the change, and choose a path. This skill makes that routing explicit and repeatable.

This skill does **not** implement, design, or write production code. It triages and routes.

---

## When to use

- The human references a specific thin-slice to begin work on.
- You are about to pick up the next slice in a wave.

## When NOT to use

- The change has **no** thin-slice and is genuinely Trivial (typo, log line, dependency bump). Go straight to `intake-code-contribution` (which owns the Trivial path).
- The human explicitly asks for exploration, review, or planning only.

---

## Step 1 — Locate the Slice

Resolve the slice ID in the wave README.

- If the slice ID does not exist, **stop**. Ask whether to create it via `create-wave` / `create-product-design-spec`, or whether the ID was mistyped.
- Read its title, status, dependencies, and acceptance criteria.

---

## Step 2 — Precondition Checks (Hard Gate)

Both checks must pass before routing. If either fails, stop and report.

**Status check.**

- `⚪ Not Started` → proceed.
- `🔄 In Progress` → confirm whether to resume (a sprint or ledger may already exist — see Step 5 routing).
- `✅ Complete` → this is a **correction or reopen**. Confirm with the human, then **keep the same slice ID** (never invent a replacement). Treat as at least Standard tier.
- `🚫 Blocked` / `⚠️ At Risk` → surface the reason; do not route until the block is acknowledged.

**Dependency check.**

- Every dependency the slice declares must be `✅ Complete`.
- If any dependency is unmet, **stop** and report the specific blocking slice(s). Do not route to a sprint for a slice whose dependencies are not satisfied.

---

## Step 3 — Provisional Tier Classification

Classify the slice into Trivial / Standard / Major **by reference to the canonical tier table in `intake-code-contribution` Step 0**. Do not restate or fork that table here — read it and apply it. This is a *provisional* call whose only job is to choose the routing path in Step 5. `intake-code-contribution` Step 0 remains the **final, authoritative** classification and may confirm or escalate this call.

Output the provisional tier with the one deciding reason (e.g., "Standard — new behavior inside an existing capability, no new dependency" or "Major — shells out to a new external tool and does network provisioning, ADR-worthy").

Apply the same escalation defaults intake uses: default to the higher tier when uncertain; any new external dependency or public-contract change escalates; a correction/reopen is at minimum Standard.

---

## Step 4 — Lightweight Ambiguity Log + Pre-Mortem (Standard tier)

For **Standard**-tier slices, run a time-boxed (~5 minute) clarity pass and produce two short artifacts. These seed `create-sprint` — they are not a full discovery phase.

**Ambiguity log (top unknowns):**

```
- Unknown: <what is unclear> → Assumption: <the assumption we will proceed on>
- Unknown: … → Assumption: …
- Unknown: … → Assumption: …
```

If any unknown is load-bearing enough that a wrong assumption would change scope, **stop and clarify with the human** before routing.

**Pre-mortem risk register (top 3):** assume the slice failed six months out — what went wrong?

```
| Risk | Likelihood | Impact | Mitigation / trigger |
| ---- | ---------- | ------ | -------------------- |
```

Hand both artifacts to `create-sprint`, which carries them into its Risk register.

- **Trivial** tier: skip this step.
- **Major** tier: skip this lightweight pass and run the full `discovery-and-ambiguity-log` instead.

---

## Step 5 — Deterministic Routing

Route by the provisional tier. State the route explicitly before handing off.

| Tier | Route |
| ---- | ----- |
| **Trivial** | → `intake-code-contribution` (abbreviated envelope) → `verify-and-assemble-pr`. No sprint. |
| **Standard** | → `create-sprint` (seed the Risk register from Step 4) → **wait for the Sprint Plan Approval line to be signed** → `intake-code-contribution` → `implement-with-defensive-patterns` → `verify-and-assemble-pr`. |
| **Major** | → `discovery-and-ambiguity-log` → `design-system-architecture` → `design-capability-layout` → `create-adr` → wait for **mechanical Design Approval** → `intake-code-contribution` → `implement-with-defensive-patterns` → `verify-and-assemble-pr`. |

If a sprint and/or progress ledger already exist for this slice (resume case), route to `intake-code-contribution`, which restores state from the ledger before any code is written.

---

## Step 6 — Emit the Triage Record

Produce this compact record so the human can see the routing decision and `create-sprint` can consume the risk seeds.

```markdown
## Thin-Slice Triage

Slice: [TS-NNN + title]
Status: [current status]
Preconditions: [dependencies satisfied? status OK?] — PASS | STOP (reason)
Provisional tier: [Trivial | Standard | Major] — [deciding reason]
Ambiguity log: [3 lines, or n/a for Trivial / see discovery for Major]
Top risks: [3-row register, or n/a]
Route: [the Step 5 path]
```

---

## Quality Checklist

- [ ] Slice located in a wave README (stopped if missing)
- [ ] Status checked; correction/reopen keeps the same slice ID
- [ ] All dependencies satisfied (stopped and named the blocker if not)
- [ ] Provisional tier classified by reference to intake Step 0 (table not copied)
- [ ] (Standard) Ambiguity log + 3-row pre-mortem produced and handed to create-sprint
- [ ] Route stated explicitly and matches the tier
- [ ] Triage record emitted

---

## Anti-Patterns

- Routing a slice whose dependencies are not yet `✅ Complete`
- Copying or paraphrasing the tier table instead of referencing intake Step 0 (two sources drift)
- Treating a `✅ Complete` reopen as new work with a fresh slice ID
- Skipping the ambiguity log for Standard work and discovering the unknown mid-implementation
- Jumping straight to `create-sprint` or `implement-with-defensive-patterns` without a precondition gate
- Doing the design/architecture work here — this skill triages and routes only
