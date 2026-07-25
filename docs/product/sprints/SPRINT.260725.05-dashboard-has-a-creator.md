# SPRINT.260725.05: The dashboard convention has a creator

**Status:** 🔄 In Progress\
**Wave:** wave-self-conformance\
**Thin-Slices:** TS-010\
**Started:** 2026-07-25\
**Completed:** —

---

## Sprint Goal

A team bootstrapping a repo with Praxis gets the product dashboard created for them, so the convention Praxis tells them to keep is one its own scaffolding produces.

---

## Sprint Footprint (machine-readable)

> Declared for the first concurrent dispatch. This block is the artifact `TS-005` of
> [wave-executable-seams](../waves/wave-executable-seams/README.md) needs, and its shape is
> being learned here rather than guessed.

```json
{
  "sprint": "260725.05",
  "capabilities": ["skills"],
  "fileGlobs": ["skills/bootstrap-project/SKILL.md"],
  "persistentResources": [],
  "configKeysWritten": [],
  "dependsOnContracts": [],
  "closeArtifacts": ["CHANGELOG.md", "docs/product/README.md", "docs/product/waves/wave-self-conformance/README.md"]
}
```

---

## Hypothesis Card

**Hypothesis:** `bootstrap-project` creating `docs/product/README.md` closes the last gap between the dashboard convention Praxis names and the tree Praxis scaffolds.

**Validation method:** The scaffold step exists, names the same path every other surface names, and carries the authority statement.

**Decision rule:** Continue if the created file matches the convention; stop if creating it requires deciding wave-dashboard content that belongs to `create-wave`.

---

## Risks (Pre-Mortem Seed)

| Risk | Likelihood | Impact | Mitigation / trigger |
| ---- | ---------- | ------ | -------------------- |
| The scaffolded dashboard duplicates what `create-wave` writes, so two skills own one file | M | M | `bootstrap-project` creates the shell and the table header only; row content stays `create-wave`'s. |
| Concurrent sibling sprint writes the same close artifacts | **H** | M | Known and deliberate — this dispatch exists to surface it. Close is reconciled centrally, not raced. |

---

## Scope (Immutable Once Started)

### Thin-Slices Included

- [ ] TS-010: The dashboard convention has a creator, not just a name.

### Out of Scope

- Wave row content — `create-wave` owns it.
- Any change to `scripts/` — that is the sibling sprint's footprint.

---

## Engineering Current-State Snapshot (Bridge Anchor)

**Codebase:** `skills/bootstrap-project/SKILL.md` Step 7 creates `docs/architecture/README.md` and the first ADR; a later step creates `docs/product/waves/wave-000-bootstrap/` and `docs/product/sprints/`. No step creates `docs/product/README.md`. `docs/product/README.md` in this repo opens by claiming it is "the anchor that Praxis instructs host repos to keep" — an instruction that does not exist.

**Toolchain:** documentation-only change; no runtime.

**Active ADRs that bind this work:** none directly.

**Known debt / hazards:** the sibling sprint `260725.06` is active concurrently and shares this sprint's close artifacts.

---

## Gap Analysis

**Current state:** Every surface names the dashboard; none creates it.

**Target end state — outcome and behavior:** A team that runs `bootstrap-project` finds a product dashboard already in their tree, matching the path and authority statement every other Praxis surface names.

**Gap to close:**

- [ ] No scaffold step creates `docs/product/README.md`

---

## Implementation Plan

### Phase 1

- [ ] Add the dashboard to `bootstrap-project`'s scaffolding step, next to where `docs/product/waves/` and `docs/product/sprints/` are created.
- [ ] Shell only: title, the product-intent authority line, and an empty wave-dashboard table with headers. Row content remains `create-wave`'s job.

### Resilience / Failure-Mode Checklist

- [x] **Idempotency** — scaffolding is create-if-absent, consistent with the surrounding steps.
- [x] **Concurrency** — N/A within the sprint; the cross-sprint case is handled at close.
- [x] **Offline / degraded dependency** — N/A.
- [x] **Version pinning** — N/A.
- [x] **Partial-failure recovery** — a missing dashboard is the current state; partial application is not worse.

### Production-Readiness Conformance (Four Anchors)

**Seams this slice touches:** none.

- [x] **Observable** / **Configurable** / **Horizontally scalable** / **Resilient** — conform vacuously; documentation-only, no boundary.

---

## Sprint Plan Approval (Standard & Major tiers)

```
Reviewed by: Wael Rabadi — pre-authorized for this batch via /loop on 2026-07-25
Date: 2026-07-25
Scope confirmed: Approved as part of the first concurrent dispatch firing wave-executable-seams TS-005's named trigger.
```

---

## Design Approval (Major-tier sprints only)

`n/a (tier: Standard)`

---

## Test Plan (TDD)

### Adapter Contract Tests

- [ ] `should name docs/product/README.md in the scaffolding step`
- [ ] `should carry the same authority statement other surfaces claim`
- [ ] `validate-plugin stays green`

---

## Acceptance ↔ Test Traceability

| AC ID | Acceptance criterion | Test layer | Test file / name | Evidence | Status |
| ----- | -------------------- | ---------- | ---------------- | -------- | ------ |
| AC-1 | Given `bootstrap-project` runs, when scaffolding completes, then `docs/product/README.md` exists with the wave-dashboard structure | Adapter Contract | `should name docs/product/README.md in the scaffolding step` | example | ⚪ |
| AC-2 | Given the dashboard is created, when it is read, then its authority statement matches what the dashboard claims Praxis instructs host repos to keep | Adapter Contract | `should carry the same authority statement other surfaces claim` | example | ⚪ |

---

## Acceptance Criteria

- [ ] Given `bootstrap-project` runs, when scaffolding completes, then `docs/product/README.md` exists with the wave-dashboard structure
- [ ] Given the dashboard is created, when it is read, then its authority statement matches what `docs/product/README.md` claims Praxis instructs host repos to keep
- [ ] `validate-plugin.sh` passes

---

## Completion Checklist

- [ ] All implementation tasks done
- [ ] Acceptance criteria met

---

## Working Notes (Ephemeral)

Dispatched concurrently with `SPRINT.260725.06` to fire the named build trigger for `TS-005` of wave-executable-seams.
