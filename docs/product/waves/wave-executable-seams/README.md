# WAVE: Executable Seams

> **Delivered before wave adoption — a derived record, not a plan.** Reconstructed from validated truth (release history, ADRs, capability records) after the work shipped. It carries no hypothesis card and no acceptance criteria because none were written at the time; each slice cites the evidence for what it delivered. Current-state architecture lives in [docs/architecture/](../../../architecture/). Derivation rules: [ADR.260725.10](../../../architecture/adr/ADR.260725.10-brownfield-wave-retrofit.md).

**Status:** ✅ Delivered\
**Goal:** A team can build a unit of work against another unit's frozen promise instead of waiting for its internals to merge, and can trust that the promise is machine-checked rather than described.

---

## Tracking Rules

- Track each thin-slice by intended user outcome and current state only.
- If a thin-slice is reopened or corrected, keep the same slice ID and add one short tracking note next to that slice.
- Keep implementation history in sprint files and version control. This README stays focused on product intent.

---

## Value Theme

GenAI inverts the usual constraint: producing code stops being the bottleneck, and *agreeing on boundaries* becomes it. When a boundary exists only as prose, a dependent unit must either wait for the producer to merge or build against a snapshot that rots. Making the boundary executable — a frozen, versioned, machine-checkable contract — converts "wait for merge" into "build against a promise." Parallelism then becomes a *permission* that emerges from disjointness, never a schedule the method imposes.

---

## Scope

- A boundary description that is machine-checkable, versioned, and frozen: Shape plus shared Behavior suite.
- A conformance gate proving every declared seam has both, on both sides.
- Independent adversarial review of seam *behavior*, not just structure.
- Safety rails that make concurrency safe when a human or orchestrator chooses it.

**Out of scope:**

- Scheduling or orchestrating parallel work. Praxis supplies rails, never a scheduler.
- The four runtime anchors and their probes — that is [wave-production-readiness](../wave-production-readiness/README.md).

---

## Thin-Slices

### TS-001: A boundary becomes a frozen, versioned promise

> **Status:** ✅ Complete

**User Value:** As a team splitting work across a boundary, I need that boundary expressed as a versioned contract so that a dependent slice can start immediately instead of waiting for the producer's internals.

**Evidence:** `skills/define-seam-contract/SKILL.md` produces a machine-readable Shape (OpenAPI for http, JSON-Schema for event, native typed ports for port kinds — project-overridable), registers the shared Behavior suite, and assigns a frozen `<name>@vN` id in `.seam-contracts.json`. Seams are declared at the wave in `create-product-architecture-spec` before slices fork. Shipped in the [seam-contract release](../../../../CHANGELOG.md#020--2026-06-05).

---

### TS-002: A declared seam cannot exist without its proof

> **Status:** ✅ Complete

**User Value:** As a reviewer, I need a declared boundary to fail the build when its Shape or shared test suite is missing, so that "we have a contract" cannot be asserted without one.

**Evidence:** `scripts/check-seam-contract-parity.sh` generalizes the Port/Adapter parity gate to every declared seam, warn-first with mechanical promotion to `enforce`, skipping cleanly when no manifest exists. `verify-and-assemble-pr` Step 3 and the refactor matrix require every touched seam's shared suite to have run against **both** sides — consumer-driven, not producer-asserted.

---

### TS-003: Seam behavior is proven by a different head

> **Status:** ✅ Complete

**User Value:** As a team, I need someone other than the author to attack a boundary's behavior so that properties like idempotency and circuit-breaking are demonstrated rather than claimed in prose.

**Evidence:** `verify-and-assemble-pr` Step 5 — the reviewer demands the test proving the circuit opens mid-call, the handler is idempotent under retry, the correlation id crosses the seam, concurrent operations linearize. Run by a different session or agent by default, with a same-agent reviewer-mode switch on a fresh diff read as the recorded fallback, and the path used written to the ledger. Paired with property-over-example at high-risk seams in the AC↔test matrix and `test-by-ownership`.

**Tracking note:** No trace or existence gate enforces this slice, deliberately. A metric proxying for judgment manufactures false confidence, so the only honest enforcement is a different head rejecting the PR; the ledger records *which head* reviewed, and never certifies the verdict.

---

### TS-004: Concurrency is safe when someone chooses it

> **Status:** ✅ Complete

**User Value:** As a team or orchestrator dispatching two slices at once, I need the method to tell me when that is safe so that parallelism does not silently corrupt shared state.

**Evidence:** The four-condition disjointness rule in `using-praxis` and the capability-driven guardrails — two units may proceed concurrently only if disjoint across capability/files, persistent resources, and shared config keys, *and* each depends only on a frozen `<name>@vN` contract rather than the other's in-flight internals. Capability-disjointness alone is explicitly called insufficient. `scripts/check-sprint-id-collision.sh` makes sprint-id collision an exact, non-heuristic check. Snapshot-staleness re-anchoring in `intake-code-contribution` handles a bridge that sat queued while siblings merged.

---

### TS-005: Disjointness and contract freshness become mechanical

> **Status:** ✅ Complete

**User Value:** As a team actually running slices concurrently, I need disjointness and contract freshness checked by a script rather than by discipline, so that a violation fails the build instead of being noticed later.

**Acceptance Criteria:**

- [x] Given a machine-readable sprint footprint (touched capabilities and file globs, persistent resources, config keys, depended-on `<name>@vN`), when two active sprints overlap on any axis, then `check-sprint-disjointness.sh` reports the overlap
- [x] Given a sprint depending on a seam contract that moved since the bridge froze, when `check-contract-freshness.sh` runs, then it fails and names the contract
- [x] Given no concurrent dispatch has occurred, when the build runs, then neither check is required — the footprint artifact does not become mandatory before it is used

**Dependencies:** TS-001, TS-004.

**Tracking note:** The deferral held to its terms and was vindicated by them. The named trigger fired when two slices were dispatched concurrently in parallel worktrees, and the footprint's shape was learned from that dispatch rather than guessed — which mattered, because the four conditions alone were insufficient. Both sprints were disjoint on all four axes and still collided, so the footprint carries two fields the design did not anticipate:

- **`closeArtifacts`** — the four conditions model *implementation* footprint. Two sprints can be disjoint for their entire working life and still collide the moment they land, on the CHANGELOG, the dashboard, the wave README, and the capability record. Overlap here does not forbid concurrency; it requires close to be reconciled centrally rather than raced. Reported as advisory, never as a violation.
- **`baseRevision`** — both worktrees were cut two releases behind the frozen tip. Every contract hash still matched, because there were no contracts; the staleness lived in the base revision. Contract freshness alone cannot see this.

A third finding is recorded but not mechanized: an acceptance criterion can *span* footprints — writable by one sprint, verifiable only against a file that sprint may not touch. That one was bound with a required phrase rather than a probe.

The dispatch also broke two repo-wide scans, because its own artifacts belong to no footprint: worktrees created inside the repo, and sprint-shaped test fixtures. Both are now excluded.

---

## Success Criteria

TS-005's named trigger was reached and its two checks shipped. The discipline and its mechanical enforcement now both exist.

---

## Dependencies

- **Requires:** [wave-method-spine](../wave-method-spine/README.md).
- **Enables:** safe concurrent delivery, and the production-readiness anchors that attach their probes to seams.
