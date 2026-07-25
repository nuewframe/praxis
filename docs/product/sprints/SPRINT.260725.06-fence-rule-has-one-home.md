# SPRINT.260725.06: The fence rule has one implementation

**Status:** 🔄 In Progress\
**Wave:** wave-self-conformance\
**Thin-Slices:** TS-011\
**Started:** 2026-07-25\
**Completed:** —

---

## Sprint Goal

A maintainer writing about markdown syntax can quote a link construct inside a code span without tripping the link checker, because check #14 reads citations the same way every other check does.

---

## Sprint Footprint (machine-readable)

> Declared for the first concurrent dispatch. This block is the artifact `TS-005` of
> [wave-executable-seams](../waves/wave-executable-seams/README.md) needs, and its shape is
> being learned here rather than guessed.

```json
{
  "sprint": "260725.06",
  "capabilities": ["enforcement"],
  "fileGlobs": ["scripts/validate-plugin.sh"],
  "persistentResources": [],
  "configKeysWritten": [],
  "dependsOnContracts": [],
  "closeArtifacts": ["CHANGELOG.md", "docs/product/README.md", "docs/product/waves/wave-self-conformance/README.md", "docs/architecture/enforcement/README.md"]
}
```

---

## Hypothesis Card

**Hypothesis:** Routing check #14 through `citation_scan` removes the last hand-rolled fence loop and makes a quoted link construct a citation, matching how #4 and #10 already behave.

**Validation method:** A link construct inside a code span stops failing the build; a genuinely broken link still fails.

**Decision rule:** Continue if both hold; stop if code-span exemption would hide real broken links in prose.

---

## Risks (Pre-Mortem Seed)

| Risk | Likelihood | Impact | Mitigation / trigger |
| ---- | ---------- | ------ | -------------------- |
| Code-span exemption hides a genuinely broken link written in backticks | **M** | H | A markdown link is not normally written inside backticks — unlike a path. Verify a real broken link still fails before accepting. |
| Behavior change to link resolution regresses the checks that depend on it | M | H | Negative-test both directions; the clean tree must stay green. |
| Concurrent sibling sprint writes the same close artifacts | **H** | M | Known and deliberate — this dispatch exists to surface it. Close is reconciled centrally. |

---

## Scope (Immutable Once Started)

### Thin-Slices Included

- [ ] TS-011: The fence rule has one implementation, including its origin.

### Out of Scope

- Any change to `skills/` — that is the sibling sprint's footprint.
- Changing what counts as a resolvable link target.

---

## Engineering Current-State Snapshot (Bridge Anchor)

**Codebase:** `scripts/validate-plugin.sh` check #14 carries its own fence loop (the original one). `citation_scan.py` holds the shared implementation, consumed by checks #4 and #10. Check #14 has no code-span awareness, so a link construct quoted in backticks is matched as a live link.

**Toolchain:** bash 3.2 floor; python3.

**Active ADRs that bind this work:** [ADR.260725](../../architecture/adr/ADR.260725-inline-declared-exceptions.md) — mandates one shared fence implementation rather than duplicates.

**Known debt / hazards:** the sibling sprint `260725.05` is active concurrently and shares this sprint's close artifacts.

---

## Gap Analysis

**Current state:** The fence rule originated in check #14 and was moved into `citation_scan` for the other callers, leaving the origin as the one caller not using it.

**Target end state — outcome and behavior:** Documenting the link checker no longer breaks the link checker; all three literal-position checks read citations by one rule.

**Gap to close:**

- [ ] Check #14 carries a duplicate fence loop
- [ ] Check #14 has no code-span awareness

---

## Implementation Plan

### Phase 1

- [ ] Route check #14 through `citation_scan.analyze(path, marker='praxis:allow-path', pattern=<link regex>)` so a link construct inside a code span or fence is a citation.
- [ ] Delete #14's local fence loop.

### Resilience / Failure-Mode Checklist

- [x] **Idempotency** — read-only check.
- [x] **Concurrency** — N/A within the sprint.
- [x] **Offline / degraded dependency** — N/A.
- [x] **Version pinning** — no new dependency.
- [x] **Partial-failure recovery** — a failed change leaves the check red, which is the correct signal.

### Production-Readiness Conformance (Four Anchors)

**Seams this slice touches:** none.

- [x] **Observable** / **Configurable** / **Horizontally scalable** / **Resilient** — conform vacuously; build-time linter, no boundary.

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

- [ ] `should not report a link construct inside an inline code span`
- [ ] `should not report a link construct inside a fence`
- [ ] `should still fail on a genuinely broken link in prose`
- [ ] `validate-plugin stays green on the clean tree`

---

## Acceptance ↔ Test Traceability

| AC ID | Acceptance criterion | Test layer | Test file / name | Evidence | Status |
| ----- | -------------------- | ---------- | ---------------- | -------- | ------ |
| AC-1 | Given a markdown link construct inside an inline code span or fence, when check #14 runs, then it is treated as a citation and not reported | Adapter Contract | code-span + fence cases | property (two citation positions) | ⚪ |
| AC-1 | *(no false negative)* a real broken link still fails | Adapter Contract | `should still fail on a genuinely broken link in prose` | example | ⚪ |
| AC-2 | Given check #14, when it resolves links, then it consumes `citation_scan` rather than carrying its own fence loop | Adapter Contract | verified by reading the diff — local fence loop deleted | example | ⚪ |

---

## Acceptance Criteria

- [ ] Given a markdown link construct inside an inline code span or fence, when check #14 runs, then it is treated as a citation and not reported
- [ ] Given check #14, when it resolves links, then it consumes `citation_scan` rather than carrying its own fence loop
- [ ] `validate-plugin.sh` passes

---

## Completion Checklist

- [ ] All implementation tasks done
- [ ] Acceptance criteria met

---

## Working Notes (Ephemeral)

Dispatched concurrently with `SPRINT.260725.05` to fire the named build trigger for `TS-005` of wave-executable-seams.
