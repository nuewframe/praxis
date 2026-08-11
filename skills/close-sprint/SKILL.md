---
name: close-sprint
description: >
  Close a completed sprint: verify acceptance criteria, record outcome evidence, then distill
  learnings BIDIRECTIONALLY into both product artifacts (initiative files, docs/product.md) AND engineering
  artifacts (living capability records docs/capabilities/CAP.<name>.md, ADRs). The sprint file is deleted.
user-invocable: true
disable-model-invocation: false
---

# Skill: Close Sprint

Use this skill when a sprint's thin-slices are complete or the work is done.

**Sprints are ephemeral.** The sprint file exists only while work is in progress. Closing means deleting the file — not archiving. The living capability records (`docs/capabilities/CAP.<capability-name>.md`) and product dashboard ([`docs/product.md`](../../docs/product.md)) become the lasting record.

---

## Core Mental Model — Bidirectional Outflow

A sprint is a bridge between product intent and engineering reality. When it closes, the bridge dissolves and learnings flow to **both** shores:

```
                    SPRINT (closing)
                          │
        ┌─────────────────┴─────────────────┐
        ↓                                   ↓
PRODUCT-side outflow              ENGINEERING-side outflow
─────────────────────             ──────────────────────────
INIT.<initiative-name>.md         living capability record (docs/capabilities/CAP.<capability-name>.md)
docs/product/design.md            system overview (docs/architecture.md)
docs/product.md                   ADRs (new or superseded)
qa spec / NFR invariants          capability layout docs
user guides (docs/guides/, TEACH) ← rendered from the capability record
```

Promoting validated learning into `docs/capabilities/CAP.<capability-name>.md` is what transforms an educated theory into durable truth.

---

## Step 1 — Verify Completion

Confirm all acceptance criteria are met and project quality gates (`verify`) pass clean. If any criterion is unmet, do not close the sprint — descope or complete the work first.

---

## Step 2 — Record Outcome Evidence

Capture outcome evidence for the hypothesis:
- Outcome evidence (test results, behavior evidence)
- Decision (Continue / Pivot / Stop)
- Rationale

---

## Step 3 — Extract Learnings (Both Sides)

Ask in both directions:
- **Product side:** Did user experience in `INIT.<initiative>.md` or `docs/product/design.md` <!-- praxis:allow-path reason="illustrative global design path" --> need adjustment?
- **Engineering side:** Update the **living capability record** (`docs/capabilities/CAP.<capability-name>.md`) to reflect current-state truth. Update `docs/architecture/README.md` if system topology changed. Create/supersede ADRs as needed.

---

## Step 4 — Update Product Artifacts

Update the initiative file (`docs/product/initiatives/INIT.<initiative>.md`) and the roadmap index in [`docs/product.md`](../../docs/product.md). Apply learnings as present-tense intent (no sprint numbers or date references in initiative prose).

---

## Step 5 — Update Living Capability Records

Update `docs/capabilities/CAP.<capability-name>.md`. Rewrite current-state truth in place. The capability record is the source of truth that feeds downstream user guides (TEACH).

---

## Step 6 — Refresh User Docs (TEACH)

If user-observable behavior changed, invoke `author-user-docs` to refresh guides in `docs/guides/<capability>/`, derived directly from `docs/capabilities/CAP.<capability-name>.md`.

---

## Step 7 — Mark Thin-Slices Complete & Delete Ephemeral Sprint Files

1. Mark thin-slices `✅ Complete` in `INIT.<initiative>.md`.
2. Update progress index in `docs/product.md`.
3. Delete the sprint file and progress ledger:
   ```bash
   rm docs/product/sprints/SPRINT.<ID>-*.md
   rm -f docs/product/sprints/SPRINT.<ID>-*.ledger.md
   ```

---

## Quality Checklist

- [ ] All acceptance criteria verified
- [ ] Outcome evidence recorded
- [ ] Product artifacts updated (intent only, present tense)
- [ ] **Living capability record (`docs/capabilities/CAP.<capability-name>.md`) updated with current-state truth**
- [ ] Product dashboard [`docs/product.md`](../../docs/product.md) index updated
- [ ] User docs refreshed (TEACH) via `author-user-docs` if observable behavior changed
- [ ] Ephemeral sprint and ledger files deleted
