---
name: close-sprint
description: >
  Close a completed sprint: verify acceptance criteria, record outcome evidence, then distill
  learnings BIDIRECTIONALLY into both product artifacts (wave docs, dashboard) AND engineering
  artifacts (handbook, ADRs, capability layouts, refactor records). The sprint file is deleted —
  the bridge dissolves once both shores are updated.
user-invocable: true
disable-model-invocation: false
---

# Skill: Close Sprint

Use this skill when a sprint's thin-slices are complete or the work is done.

**Sprints are ephemeral.** The sprint file exists only while work is in progress. Closing means deleting the file — not archiving. The wave documents and engineering artifacts become the lasting record.

---

## Core Mental Model — Bidirectional Outflow

A sprint is a bridge between product intent and engineering reality. When it closes, the bridge dissolves and learnings flow to **both** shores. This is the moment the wave's **hypothesis** (the bet) is promoted into the durable **record** (the truth) now that it has been built and measured:

```
                    SPRINT (closing)
                          │
        ┌─────────────────┴─────────────────┐
        ↓                                   ↓
PRODUCT-side outflow              ENGINEERING-side outflow
─────────────────────             ──────────────────────────
wave README                       system overview (docs/architecture/README.md)
product-design.md                 capability record (docs/architecture/<capability>/)
product-architecture.md (the bet) ADRs (new or superseded)
qa.md                             capability layout docs
PRODUCT dashboard                 refactor records / anti-dumping baseline
user guides (docs/guides/, TEACH) ← rendered from the capability record
```

The wave `product-architecture.md` is the bet; the **capability record is the truth**. Promoting validated learning into the capability record is also what feeds the *next* phase downstream — user docs (how-tos, tutorials, product concepts) are generated from the capability record, not from sprint notes.

If a sprint only updates the product side, you've lost half the learning.

---

## Step 1 — Verify Completion

For each acceptance criterion, confirm it is met. Run the project's quality gates (typically: tests, lint, format).

If any criterion is unmet: **do not close the sprint.** Either finish the work, or explicitly descope — move the unfinished thin-slice back to `⚪ Not Started` in the wave README, then close what was actually delivered.

---

## Step 2 — Record Outcome Evidence (Lean Validation)

Before extracting learnings, capture outcome evidence for the sprint hypothesis:

- **Outcome evidence:** What observable result occurred? Cite test evidence, behavior evidence, or delivery evidence.
- **Decision:** Continue / Pivot / Stop
- **Decision rationale:** Why this decision was made based on the evidence

If outcome evidence is missing, the sprint cannot close.

---

## Step 3 — Extract Learnings (Both Sides)

Read the sprint working notes and any deviations from the original plan. Ask in **both** directions:

### Product-side learning prompts

- Did the user experience need adjustment from `product-design.md`? What is the correct picture now?
- Do any wave README thin-slices need to be reworded, split, or resequenced?
- Did the wave goal itself sharpen?
- Did the qa.md spec miss a risk that should be added for future waves?

### Engineering-side learning prompts

- Did the technical approach differ from the wave `product-architecture.md` bet? What is the correct picture now, and does the **capability record** (`docs/architecture/<capability>/`) need to be rewritten to that truth?
- Did the cross-capability topology or product-wide posture change? Update the **system overview** (`docs/architecture/README.md`).
- Did this sprint make a durable technical decision that needs an ADR? Or supersede an existing one?
- Did capability ownership shift? Does the capability layout doc need to update?
- Did the work surface anti-dumping or layout debt that should be tracked?
- Were there refactor moves worth recording for future similar work?
- Did user-observable behavior change? If so, which capability guide or journey tutorial must be re-derived (TEACH)?

If nothing changed on a side, skip its updates. But always check both sides.

---

## Step 4 — Update Product Artifacts

Update the wave documents and dashboard. Apply learnings as **corrections to intent — not as annotations or history**.

### Tone rules — non-negotiable

- Write in clean present tense as if this was always the design
- No `Sprint NNN discovered…`, `Note:`, `TODO:`, or `Updated after sprint`
- No sprint number or date references in wave content
- No passive-voice history (`was changed to`, `previously`)
- If the architecture evolved, rewrite the section to reflect the current correct design
- If the experience changed, rewrite the section to reflect the current correct experience
- Wave documents capture **intent**, not **history**

### Which file to update

| What changed                               | Update this file               |
| ------------------------------------------ | ------------------------------ |
| Technical approach within the wave         | wave `product-architecture.md` |
| User-facing behavior, flows, UI            | wave `product-design.md`       |
| Risk model, test layer mapping, invariants | wave `qa.md`                   |
| Thin-slice scope, sequencing, definition   | wave `README.md`               |
| Wave goal sharpened                        | wave `README.md`               |

If the sprint corrects or reopens a thin-slice: keep the same ID, update the text to the current intended outcome, add one short tracking note only if it explains the correction.

---

## Step 5 — Update Engineering Artifacts

Update the engineering side artifacts. Same tone rules — intent only, no history.

| What changed                                              | Update this artifact                                         |
| --------------------------------------------------------- | ------------------------------------------------------------ |
| Technical approach validated — the bet is now truth        | Capability record `docs/architecture/<capability>/` (rewrite current-state in place) |
| Cross-capability topology or product-wide posture          | System overview `docs/architecture/README.md`                |
| Durable technical decision (selection, pattern, boundary) | New ADR via `create-adr` (or supersede an existing one)      |
| Capability ownership or vertical-slice layout             | Capability layout doc + `design-capability-layout` if needed |
| Cross-cutting platform constraint                         | System overview or `design-system-architecture` output       |
| Anti-dumping debt discovered                              | Anti-dumping baseline / debt log                             |
| Refactor pattern worth reusing                            | Refactor record / handbook section                           |
| Test layer convention shifted                             | `test-by-ownership` reference in project context             |

Engineering artifacts are not just docs — they bind future work. The capability record is doubly load-bearing: it is where the next sprint inherits this sprint's learning **and** the source the user-facing docs are generated from. Rewrite it in clean present tense (same tone rules as the product side) — current-state truth, not sprint history.

---

## Step 6 — Refresh User Docs (TEACH)

If this sprint changed **user-observable behavior**, promote the validated learning one more step: into user-facing teaching. A pure refactor with no observable effect skips this step.

- Refresh the affected **capability guide** (`docs/guides/<capability>/`) and any **journey tutorial** (`docs/guides/tutorials/`) that traverses the changed capability, using `author-user-docs`.
- Derive from the now-updated capability record — never from sprint notes or the wave `product-architecture.md` bet.
- Same tone rules: clean present tense, no sprint history. Update each guide's `last-validated` date.

This is the phase the capability record exists to feed. A record updated in Step 5 but a guide left stale means the next user learns yesterday's behavior.

---

## Step 7 — Mark Thin-Slices Complete

In the wave README, update each delivered thin-slice status:

- `⚪ Not Started` → `✅ Complete`
- `🔄 In Progress` → `✅ Complete`

For any descoped thin-slice, leave as `⚪ Not Started`.

---

## Step 8 — Update the Product Dashboard

In the project's product dashboard (e.g. `PRODUCT.md`):

- Update the wave's progress to reflect completed thin-slices
- Update wave status if fully complete
- Remove the sprint from the active work section
- Keep dashboard quick-glance only: counts, state, links

---

## Step 9 — Delete the Sprint File

```
rm <sprint-dir>/sprint-NNN-<description>.md
rm -f <sprint-dir>/sprint-NNN-<description>.ledger.md
```

The sprint file is ephemeral collaboration space. Once both shores are updated, delete it. Delete the progress ledger too — it is execution state, not a durable record. Any learning worth keeping has already flowed into the product and engineering artifacts in Steps 4–6.

---

## Quality Checklist

- [ ] All acceptance criteria verified as met (unfinished work moved back to wave if descoped)
- [ ] Outcome evidence recorded and continue/pivot/stop decision documented
- [ ] **Product-side artifacts updated — intent only, no annotations, no sprint references**
- [ ] **Engineering-side artifacts updated — capability record + system overview (`docs/architecture/`), ADRs, capability layout, refactor record as applicable**
- [ ] **User docs refreshed (TEACH) when user-observable behavior changed — capability guide / journey tutorial re-derived via `author-user-docs`**
- [ ] Thin-slices marked ✅ Complete in wave README
- [ ] Product dashboard reflects current reality
- [ ] Sprint file deleted
- [ ] Progress ledger deleted (after learnings distilled)

---

## Anti-Patterns

- Closing with only product-side updates ("the wave docs are updated, ship it") — half the learning is lost
- Annotating wave docs with sprint history instead of rewriting intent
- Skipping ADR creation for a durable decision because "we'll do it later"
- Updating ADRs but leaving the capability record stale — the record is the source of truth *and* of the downstream user docs; a stale record misleads the next sprint and the docs author
- Updating the capability record but leaving the user guide stale after user-observable behavior changed — TEACH is the phase the record exists to feed
- Letting a descoped thin-slice silently disappear — it must move back to `⚪ Not Started` with intent intact
- Archiving the sprint file instead of deleting it — the bridge dissolves on close
- Leaving the progress ledger behind after close — it is execution state, not a record; distill then delete
