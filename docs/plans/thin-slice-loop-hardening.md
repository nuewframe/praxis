# Plan — Thin-Slice Loop Hardening

**Status:** Proposed (awaiting sign-off before implementation)
**Author:** Principal Engineer (architect mode)
**Scope:** Close the 7 gaps in the "Work on AG-XXX → sprint → implement → verify → close" loop.
**Deliverable of this doc:** an implementation plan only. No skill/template files change until this is approved.

---

## 1. Problem statement

The Praxis loop has a strong spine — immutable sprint bridge, tier classification, red-first posture, the mechanical Design Approval gate for Major work, and bidirectional learning at close. The weaknesses are all at the **edges of the loop**: the front door (routing), the human checkpoint for Standard work, the definition of "production-grade," coverage traceability, cross-session durability, and an unbounded debugging retry loop.

Seven gaps, validated against the real artifacts:

| # | Gap | Where it bites | Fix lives in |
|---|---|---|---|
| 1 | No entry/triage skill — "Work on AG-006" is unspecified routing | front door | **new** `start-thin-slice` |
| 2 | No plan-approval gate for Standard work (only Major has one) | human checkpoint | `create-sprint` + `intake-code-contribution` |
| 3 | "Production-grade plan" undefined — no failure-mode bar | plan quality | `create-sprint` |
| 4 | No AC ↔ test traceability matrix | coverage integrity | `create-sprint` + `intake` + `verify-and-assemble-pr` |
| 5 | No durable progress ledger — sessions die, state evaporates | cross-session durability | `create-sprint` + `intake` + `close-sprint` |
| 6 | No circuit-breaker on the debugging loop | agent thrash | `verify-and-assemble-pr` (as **discipline**, not runtime) |
| 7 | Clarification + pre-mortem informal for Standard tier | risk blindness | `start-thin-slice` → `create-sprint` |

---

## 2. Two resolved design tensions

These shape the whole plan; both were confirmed with the human.

### Resolution A — Gap #6 ships as discipline, not runtime enforcement

`using-praxis` explicitly disclaims runtime mechanics: *"It does not implement runtime mechanics — delegation, verification gates, ticketing, branch protection, **circuit breakers as runtime checks**. Those belong in an orchestration runtime such as Claude MPM."*

Therefore Gap #6 is delivered as a **self-applied stop-rule convention** — a documented halt-and-escalate protocol the agent follows, with the attempt count recorded in the progress ledger (Gap #5). Runtime enforcement (hard process kill, automatic escalation) remains delegated to an orchestration runtime. The plan will cite this boundary in the skill text so the line is explicit and future-proof.

### Resolution B — one canonical tier-classification source

Tier classification already exists and is battle-tested as **Step 0 of `intake-code-contribution`**. The new `start-thin-slice` skill must **not** fork that table. Instead:

- `start-thin-slice` performs a **provisional** tier classification *by reference* to intake Step 0, solely to choose the routing path (create-sprint vs. architect path).
- `intake-code-contribution` Step 0 remains the **canonical, final** classification and confirms or escalates the provisional call.
- The table is defined in exactly one place (intake Step 0). `start-thin-slice` links to it; it never copies it.

This keeps Trivial-without-a-slice work flowing straight through intake (which still owns Step 0) while giving slice work a deterministic front door.

---

## 3. The three-tier state model (prevents the ledger from eroding immutability)

Gap #5 introduces persisted execution state. To protect the immutable-bridge philosophy, the plan codifies three distinct artifacts with non-overlapping lifecycles:

| Artifact | Mutability | Lifetime | Holds |
|---|---|---|---|
| **Sprint bridge** (`sprint-NNN-*.md`) | Immutable once started | Deleted at close | Scope, AC, hypothesis, test plan, plan-approval line |
| **Progress ledger** (`sprint-NNN-*.ledger.md`) | Mutable each session | Survives session death; deleted at close after distillation | Plan-phase checkboxes, current red/green posture, verify-attempt counter, last verify result |
| **Working notes** (section in the bridge) | Mutable scratch | Deleted at close | Free-form discoveries, distilled into product/eng artifacts |

The ledger is **artifact discipline** (a file that survives sessions), not runtime mechanics — squarely inside Praxis scope. It is the Praxis-native replacement for ad-hoc repo-memory state files.

---

## 4. Work breakdown — sequenced by the 3-bundle recommendation

Order: **Bundle 2 template fields first** (they are referenced by Bundle 1 routing), then **Bundle 1 triage** (wires the references), then **Bundle 3 durability**. Presented below in bundle priority order; build order noted per task.

### Bundle 1 — `start-thin-slice` triage skill — closes #1, forces #2 and #7 early

**New file:** `skills/start-thin-slice/SKILL.md` (user-invocable, model-invocable).

Steps the skill defines:

1. **Locate the slice.** Resolve the slice ID in the wave README. Stop if it does not exist.
2. **Precondition checks (hard gate).**
   - Status is not already `✅ Done` (else confirm this is a correction/reopen — keep the same slice ID).
   - All declared dependencies are satisfied (each dependency slice is `Done`). If any are unmet → **stop and report the blocking slice**.
3. **Provisional tier classification** — by reference to `intake-code-contribution` Step 0 (Resolution B). Output a provisional tier with the deciding reason.
4. **Lightweight clarification + pre-mortem (Standard tier; closes #7).** A time-boxed pass that produces:
   - A 3-line ambiguity log (top unknowns + the assumption being made for each).
   - A 3-line pre-mortem risk register: top 3 risks with likelihood / impact / mitigation.
   This output is **handed to `create-sprint`** to seed its risk register (Bundle 2). Major tier escalates to the full `discovery-and-ambiguity-log` instead.
5. **Deterministic routing:**
   - **Trivial** → `intake-code-contribution` (abbreviated) → `verify-and-assemble-pr`.
   - **Standard** → `create-sprint` → **Sprint Plan Approval** (Bundle 2) → `intake` → `implement-with-defensive-patterns` → `verify`.
   - **Major** → `discovery-and-ambiguity-log` → architect path (unchanged).
6. **Emit a triage record** (provisional tier, precondition results, routing decision, risk seeds) for the human and for `create-sprint` to consume.

**Build order:** implement after Bundle 2 fields exist so step 5's references resolve.

**Acceptance for this skill:**
- Refuses to route when a dependency is unmet or the slice is missing.
- Never copies the tier table; links to intake Step 0.
- Standard path always produces the risk seed consumed by `create-sprint`.

### Bundle 2 — `create-sprint` template upgrade — closes #2, #3, #4, #7

All edits are to `skills/create-sprint/SKILL.md` (template + Quality Checklist + Anti-Patterns).

**2a. Sprint Plan Approval line (#2).** New section, **all tiers** (distinct from the Major-only Design Approval):

```
## Sprint Plan Approval (all tiers)

Reviewed by: <name or role>
Date: YYYY-MM-DD
Scope confirmed: <one line — plan reviewed, scope + risks understood>
```

This is the mechanical home for the "pause here for me to review" habit. `intake-code-contribution` Step 3 (Confirm Sprint Bridge) gains a check that **refuses to pass to implementation** unless this line is filled. Added to the intake Quality Checklist.

**2b. Resilience / failure-mode checklist (#3).** Added inside the Implementation Plan section. Each item must be filled or explicitly `N/A` with a reason:

```
### Resilience / Failure-Mode Checklist (fill or mark N/A with reason)

- [ ] Idempotency — repeated execution is safe because: …
- [ ] Concurrency — behavior under simultaneous execution: …
- [ ] Offline / degraded dependency — detection + behavior: …
- [ ] Version pinning / reproducibility — pinned versions, no `latest`: …
- [ ] Partial-failure recovery — resume/rollback of half-done work: …
```

These map directly to the capability guardrails (timeout / retry / fallback / circuit breaker) and to AG-006-class slices (warm-up, `~/.rover/bin`, network provisioning).

**2c. AC ↔ test traceability matrix (#4).** New required section linking the existing Acceptance Criteria and Test Plan:

```
## Acceptance ↔ Test Traceability

| AC ID | Acceptance criterion | Test layer | Test file / name | Status |
|-------|----------------------|------------|------------------|--------|
| AC-1  | …                    | Logic      | …                | ⚪/🔴/🟢 |
```

Rule: **every AC maps to ≥1 test.** Enforced at two points (Bundle-2 cross-skill edits):
- `intake` checks every AC has a mapped test before implementation starts.
- `verify-and-assemble-pr` checks every mapped test actually ran in the captured `verify` output.

**2d. Risk register (#7).** New short section near the Hypothesis Card, seeded by the `start-thin-slice` pre-mortem:

```
## Risks (pre-mortem seed)

| Risk | Likelihood | Impact | Mitigation / trigger |
|------|-----------|--------|----------------------|
| …    | L/M/H     | L/M/H  | …                    |
```

**Build order:** do this bundle first; its field names are referenced by Bundle 1 and Bundle 3.

### Bundle 3 — progress ledger + verify budget — closes #5, #6

**3a. Progress ledger (#5).** New durable file `sprint-NNN-<desc>.ledger.md`, defined by a template added to `create-sprint`:

```
# Sprint NNN — Progress Ledger (mutable; survives sessions; deleted at close)

## Plan phase
- [ ] Phase 1: <name>
- [ ] Phase 2: <name>

## Test posture (current)
| Behavior | Layer | State (🔴/🟢) | Last run |

## Verify attempts
- Consecutive failed verifies on current cause: N
- Last verify exit code + cause: …

## What's left
- …
```

Cross-skill wiring:
- `create-sprint` creates the ledger at sprint start.
- `intake-code-contribution` gains a **resume step**: if a ledger exists, restore state from it before re-deriving — prevents context amnesia.
- `close-sprint` deletes the ledger **after** distilling any durable learning, same as working notes.

**3b. Verify budget + BLOCKED-vs-FAIL decision (#6, as discipline).** Edits to `verify-and-assemble-pr` Step 3:

- Add a **"Debugging Loop Budget"** subsection: before each retry, classify the failure as `FAIL` (code regression) vs `BLOCKED` (environment) using the **existing** `Environment Blocked ≠ Test Failed` table.
- **Stop rule:** after **3 consecutive failed verify cycles on the same cause**, HALT. Do not keep editing code. Produce a halt summary (what was tried, current hypothesis, FAIL-vs-BLOCKED determination, suspected blocked dependency) and escalate to the human.
- The attempt counter lives in the progress ledger (ties #5 and #6 together).
- Explicit framing in the skill text: *this is self-applied discipline; hard runtime enforcement is delegated to an orchestration runtime (e.g., MPM) per Praxis's stated boundary.*

---

## 5. Cross-cutting edits

- **`skills/using-praxis/SKILL.md`** — add the `start-thin-slice` row to the planning-artifacts skill index; note the progress-ledger artifact and the Sprint Plan Approval gate in the lean-delivery guardrail summary.
- **`instructions/lean-delivery-guardrails.instructions.md`** — one-line additions for the plan-approval gate and the ledger artifact (keep minimal; the host repo still wins).
- **`scripts/validate-plugin.sh`** — confirm the new skill passes frontmatter/structure self-test; run after Bundle 1.
- **`README.md` / `CHANGELOG.md`** — note the new skill and the create-sprint template upgrade (changelog entry only; no behavioral claims).

No new enforcement script is required — Gap #6 is discipline, and Gap #4 is checked by editing existing intake/verify steps rather than a new linter.

---

## 6. Pre-mortem on this plan

Assume six months out the hardening "failed." The likely causes and pre-applied mitigations:

| Failure mode | Mitigation baked into the plan |
|---|---|
| Skill sprawl / over-engineering | One new skill only; pre-mortem folded into triage, not a separate skill; #4 and #6 reuse existing steps, no new scripts. |
| Tier logic forks and drifts | Resolution B — single canonical table in intake Step 0; triage links, never copies. |
| Ledger erodes the immutable-bridge model | Section 3 three-tier state model with strict lifecycles; ledger is execution state, deleted at close. |
| #6 misread as a runtime feature Praxis shouldn't own | Resolution A — explicit "discipline not runtime" framing + boundary citation in the skill text. |
| AC↔test matrix becomes box-ticking | Mechanically checked at both intake (every AC mapped) and verify (every mapped test ran). |
| Plan-approval gate ignored like the chat habit it replaces | intake Step 3 hard-refuses to pass without the signed line. |

---

## 7. Sequencing summary

1. **Bundle 2 fields** in `create-sprint` (plan-approval line, resilience checklist, AC↔test matrix, risk register) + the matching intake/verify checks.
2. **Bundle 1** `start-thin-slice` skill (references Bundle 2 fields) + `using-praxis` index entry.
3. **Bundle 3** progress-ledger template + intake resume step + close-sprint deletion + verify-budget discipline.
4. Cross-cutting docs + `validate-plugin.sh` pass + CHANGELOG entry.

This closes ~80% of the loop risk with the first two bundles; Bundle 3 removes the cross-session and thrash failure modes.

---

## 8. Decisions (signed off)

1. **Ledger file vs. section** → **separate `sprint-NNN-*.ledger.md` file.** Survives session death independently of the immutable bridge; deleted at close after distillation.
2. **Stop-rule budget** → **default 3** consecutive failed verifies on the same cause, **project-configurable** (a project may override the number in its own context; 3 is the Praxis default).
3. **Plan-approval scope** → **required for Standard and Major; Trivial is exempt** and writes `n/a (tier: Trivial)`. Keeps typo-class changes friction-free while still mechanically gating real work.
