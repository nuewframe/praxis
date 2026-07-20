# Plan — Reconcile the Major-Path Ordering & Break the Design-Approval Deadlock

**Status:** Implemented
**Author:** Principal Engineer (architect mode)
**Scope:** Critical review item #3 — the Major-tier workflow contradicts itself on ordering across three sources, and the Major route can never satisfy its own Design Approval gate because no source creates the sprint that hosts that gate.
**Deliverable of this doc:** an implementation plan only. No skill/agent files change until this is approved.

---

## 1. Problem statement

The Major tier is defined in **three** places, and they disagree.

| Source | Stated Major order | `create-sprint` in the path? |
|---|---|---|
| `start-thin-slice` Step 5 | discovery → design-sys → design-cap → create-adr → **Design Approval** → intake → implement → verify | **No** |
| `intake-code-contribution` Step 7 | (intake runs first) → discovery → design-sys → design-cap → create-adr → **Design Approval** → implement → verify | **No** |
| `principal-engineer.agent.md` "Phase order for Major" | 0. intake → 1. discovery → 2. design-sys → 3. design-cap → 4. create-adr → 5. implement → 6. verify | **No** |

Two independent defects fall out of this table:

**Defect 3a — opposite ordering of intake vs. the architect phase.**
`start-thin-slice` runs the architect phase **first**, then intake. `intake` Step 7 and `principal-engineer.agent.md` put intake **first** (Phase 0), then the architect phase. A reader following one skill contradicts a reader following the other.

**Defect 3b — the Major route deadlocks on its own gate.**
None of the three sources invokes `create-sprint` on the Major path. But:

- The **Design Approval line lives in the sprint file** (`create-sprint` "Design Approval (Major-tier sprints only)").
- Architect mode's **exit signal requires** "ADR `status: Accepted` **plus a signed Design Approval line in the active sprint file**" (`principal-engineer.agent.md`, Mode A exit signal).
- `intake` Step 3 **hard-stops** without a sprint bridge.

Followed literally, a Major slice reaches "wait for Design Approval" with no sprint in existence, so the approval has no home to be signed in. The gate can never clear. Standard tier gets `create-sprint` explicitly; Major — the tier that most needs the bridge — never creates one.

---

## 2. Root cause

Single-source-of-truth erosion. The ordered cross-skill Major pipeline is **restated** in three documents instead of **defined once and referenced**. Each restatement drifted, and each independently dropped the `create-sprint` step because "the sprint" is treated as ambient rather than as an explicit pipeline stage.

---

## 3. Design decision

### 3.1 The one canonical Major pipeline

Fix the sequence once. The corrected, deadlock-free order is:

```
1. discovery-and-ambiguity-log        (architect)
2. design-system-architecture         (architect)
3. design-capability-layout           (architect)
4. create-adr                         (architect)  → ADR status: Accepted
5. create-sprint                      (PM)         → implementation plan informed by the Design Package;
                                                      the sprint carries BOTH the Sprint Plan Approval
                                                      and Design Approval lines
6. [human] sign Sprint Plan Approval + Design Approval
                                                   → mechanical Design Approval satisfied
                                                     (ADR Accepted + signed line in the sprint)
7. intake-code-contribution           (gate)       → final pre-implementation anchor against the
                                                      now-approved sprint bridge
8. implement-with-defensive-patterns  (implementer)
9. verify-and-assemble-pr             (reviewer)
```

Two principles make this the correct order (not merely *an* order):

- **Design precedes the sprint plan.** You cannot write a Major implementation plan before the architecture and ADR exist. So `create-sprint` lands *after* `create-adr` (step 5), and its plan is informed by the Design Package. This is exactly where the deadlock is broken: the sprint now exists to host the Design Approval line before step 6's gate.
- **Intake is the last gate before implementer mode, in every tier.** Intake's job is pre-implementation anchoring, and for Major it cannot complete until the design package + approved sprint exist. Placing it at step 7 (not Phase 0) resolves the ordering contradiction: intake is always the immediate predecessor of `implement-with-defensive-patterns`, never the front door.

This reframes "Phase 0: intake" in `principal-engineer.agent.md`: the *front door* is `start-thin-slice` (triage + provisional tier). The *full intake envelope* is the final gate. Tier is provisionally set at the door and authoritatively confirmed by intake Step 0 when intake runs at step 7.

### 3.2 Single source of truth

`start-thin-slice` Step 5 already owns the routing table (provisional tier → ordered path). Make it the **canonical definition** of the ordered Major pipeline. The other two sources **reference** it instead of restating:

- `intake-code-contribution` Step 7 stops restating the architect phase as a forward hand-off. For Major it states its own position — "you are the pre-implementation gate; the architect phase + approved sprint must already exist (Steps 2–3 verify this)" — and hands off only to `implement → verify`. It adds a **cold-entry redirect**: if intake is invoked directly on a Major change with no Design Package, bounce to the architect phase first.
- `principal-engineer.agent.md` "Phase order for Major" is corrected to insert `create-sprint` at step 5 and move intake to step 7 (immediately before implement), matching §3.1.

---

## 4. Edits (files and exact changes)

| # | File | Change |
|---|---|---|
| 1 | `skills/start-thin-slice/SKILL.md` Step 5 Major row | Insert `create-sprint` after `create-adr`; change the wait to "wait for **mechanical Design Approval** (ADR `status: Accepted` + signed **Design Approval** line) **and Sprint Plan Approval**"; keep `intake` immediately before `implement`. This row becomes the canonical ordered path. |
| 2 | `skills/start-thin-slice/SKILL.md` Step 4 note | The "Major → run full `discovery-and-ambiguity-log`" note already exists; add one line that the Major path creates its sprint *after* the ADR (plan informed by the Design Package), so no lightweight ambiguity/pre-mortem is produced here. |
| 3 | `skills/intake-code-contribution/SKILL.md` Step 7 Major row | Replace the restated architect chain with: confirm Design Package + approved sprint exist (per Steps 2–3), then `→ implement → verify`. Add the cold-entry redirect to the architect phase when no Design Package exists. Reference `start-thin-slice` Step 5 as the canonical ordered path. |
| 4 | `skills/intake-code-contribution/SKILL.md` Step 0 table, Major "Phased workflow path" cell | Update to name `create-sprint` between `create-adr` and Design Approval, and place full intake as the final gate. |
| 5 | `skills/intake-code-contribution/SKILL.md` Step 3 | Strengthen the Major sprint rule: the sprint's implementation plan must be informed by an Accepted ADR; if no Design Package exists on a Major change, run the architect phase + `create-sprint` before completing intake (do not fabricate a plan). |
| 6 | `agents/principal-engineer.agent.md` "Phase order for Major" | Insert `create-sprint` as a numbered step after `create-adr`; renumber so intake is the step immediately before `implement`. Reconcile the "Phase 0: intake" line with the front-door-is-`start-thin-slice` framing. |
| 7 | `skills/create-sprint/SKILL.md` (Design Approval section) | Add one clarifying sentence: on the Major path this sprint is created **after** the ADR is Accepted, and it is the home of the Design Approval line the architect-mode exit signal requires. (No structural change.) |

Non-goal for this item: the Standard-tier path and the Trivial-tier "who writes the code" question (review items 3-adjacent and #7) are **out of scope** here except where a Major edit touches a shared table row; Standard already correctly routes `create-sprint → intake → implement` via `start-thin-slice`.

---

## 5. Anti-patterns / checklist additions

- Add to `start-thin-slice` and `intake` anti-patterns: "Routing a Major slice to the Design Approval gate before `create-sprint` has created the sprint that hosts it."
- Add to `intake` Quality Checklist (Major): "Sprint created after the ADR and before the Design Approval gate."

---

## 6. Verification

1. Re-read all three sources; confirm the Major order is byte-for-byte consistent in sequence and that exactly one source (`start-thin-slice` Step 5) defines it while the others reference it.
2. `grep` each source's Major path for `create-sprint` appearing **before** the Design Approval gate. Every Major path must contain it in that position.
3. Confirm no source still lists a Major path in which the Design Approval gate precedes sprint creation.
4. Run `bash scripts/validate-plugin.sh` — cross-reference and inventory checks must stay green.
5. Trace one Major slice end-to-end on paper against the corrected pipeline and confirm every mechanical gate (ADR Accepted, Sprint Plan Approval, Design Approval, intake Step 3) has a satisfiable home in order.

---

## 7. Rollback

All edits are documentation-only across five files. Revert the commit to restore prior text; no code, no runtime surface, no generated artifacts are affected.
