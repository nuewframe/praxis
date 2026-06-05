---
name: intake-code-contribution
description: >
  Run the pre-implementation intake gate for a GenAI code contribution. Use before writing or
  modifying implementation code for a user story, feature, thin-slice, behavior change, or
  non-trivial refactor. Anchors the work to wave intent, thin-slice acceptance criteria, wave
  specs, sprint bridge, current code state, and red/green test posture.
user-invocable: true
disable-model-invocation: false
---

# Skill: Intake Code Contribution

Use this skill before writing or modifying implementation code for a user story, feature, thin-slice, behavior-changing contribution, or non-trivial refactor.

This skill is the front door for high-quality GenAI implementation. It prevents the agent from coding from a chat prompt alone by anchoring the work to product intent, sprint scope, current engineering reality, and the correct test posture.

## Do Not Use This When

- The change is documentation-only and does not affect product behavior, code, test posture, architecture, or delivery workflow.
- The human explicitly asks for exploration, review, or planning only.
- Emergency production mitigation has been explicitly approved by the human. Even then, record which intake steps were deferred and why.

---

## Step 0 - Classify The Change Tier

Before any other intake work, classify the change into one of three tiers. The tier determines which subsequent steps run. Tier choice is **declared in the contribution envelope** (Step 6) and is auditable.

| Tier         | Use when                                                                                                                                                        | Steps required                                                                        | Phased workflow path                                                      |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| **Trivial**  | ≤ ~20 LOC, single capability, no public contract change, no new dependency, no behavior change. Examples: typo, log-line fix, dependency bump, internal rename. | Step 4 (current code touchpoints) and Step 5 (test posture: green-baseline) only.     | Skip architect + full implementer rituals → run `verify-and-assemble-pr`. |
| **Standard** | Within an existing capability and an active thin-slice; no new capability, no new ADR-level decision; existing wave specs cover the intent.                     | Steps 1-6. Skip Step 2 if wave specs are already current.                             | Skip architect mode. `implement-with-defensive-patterns` → reviewer mode. |
| **Major**    | New capability; new external dependency; public contract change; cross-cutting trade-off; any change that should produce an ADR.                                | All steps 1-6, including a refreshed Ambiguity Log and architect mode design package. | Architect mode (Phases 1-4) → implementer mode → reviewer mode.           |

### Tier Decision Rules

- **Default to the higher tier when uncertain.** A misclassified Trivial that mutates behavior is the most expensive failure.
- **Public-contract change always escalates.** Any HTTP/gRPC/event/CLI/SDK surface modification is at minimum Standard, usually Major.
- **A new external dependency always escalates to Major.** New runtime, storage, or third-party API requires an ADR.
- **A correction or reopen of a thin-slice keeps the slice ID** and is at minimum Standard, regardless of LOC.
- **Refusal:** if the human disputes a tier escalation, capture the disagreement in the envelope and escalate to a human decision before proceeding.

If Trivial: produce the abbreviated envelope, run the project's `verify` entry point, capture output, and hand off. Do **not** invent waves or sprints to satisfy a higher tier when the change is genuinely trivial.

If Standard or Major: continue to Step 1.

---

## Step 1 - Locate Product Intent

Read the project's context, product dashboard, and relevant wave directory.

Confirm:

- Active wave name and path
- Thin-slice ID, title, status, dependencies, and acceptance criteria
- Whether the request is new work, a correction, a reopen, or a refactor supporting an active thin-slice

If no wave or thin-slice exists, stop. Create or refine the wave through `create-wave` and `create-product-design-spec` before implementation planning.

If the request corrects a completed thin-slice, keep the same thin-slice ID. Do not invent a replacement slice.

---

## Step 2 - Verify Wave Specs

Confirm the wave has the four-document pattern and that each file is specific enough to guide implementation:

| File                      | Required signal                                                                   |
| ------------------------- | --------------------------------------------------------------------------------- |
| `README.md`               | Thin-slice status, user value, acceptance criteria, dependencies                  |
| `product-design.md`       | User journeys, ambiguity states, error/recovery paths, success signals            |
| `product-architecture.md` | Ownership, contracts, data/control flow, security, integrations, failure behavior |
| `qa.md`                   | Risk tiers, behavior-to-layer map, security coverage, definition of done          |

If a document is missing or too vague, stop and invoke the matching authoring skill. Do not guess the missing intent in code.

---

## Step 3 - Confirm The Sprint Bridge

Implementation must be anchored to a sprint file.

Confirm:

- Sprint file path and number
- Included thin-slices
- Immutable scope and out-of-scope items
- Engineering current-state snapshot
- Gap analysis
- Implementation plan
- Test plan
- Acceptance criteria copied from the wave README

**Sprint Plan Approval gate (Standard & Major).** Confirm the sprint's `Sprint Plan Approval` line is signed (`Reviewed by` / `Date` / `Scope confirmed`). If it is blank, **stop** — implementation does not start until a human signs it. Trivial-tier sprints carry `n/a (tier: Trivial)` and skip this gate.

**Acceptance ↔ test traceability gate.** Confirm every acceptance criterion has at least one mapped test in the sprint's traceability matrix. If any AC is unmapped, **stop** and add the missing test rows before coding — an unmapped AC ships uncovered.

**Production-Readiness conformance gate.** Confirm the sprint's Production-Readiness conformance block names the seam(s) the slice touches and marks each of the four anchors (observable, configurable, horizontally scalable, resilient) as conforming to the wave posture or carrying a reviewed deviation. If the block is blank or an anchor is unaddressed, **stop** — the slice must declare how it preserves the wave's posture before coding (the probes enforce it at `verify`, but the conformance is decided here).

**Snapshot staleness re-anchor gate (parallel-safe).** If this sprint sat queued while sibling slices merged, the engineering current-state snapshot and the seam contracts this slice depends on may have moved since the bridge was frozen. Before coding, re-check: has any `<name>@vN` seam contract this slice depends on, or any current-state fact in the Step-3 snapshot, changed since the sprint froze? If yes, **stop and re-anchor** — re-read the changed contract/snapshot and confirm the slice's plan still holds — before writing code. This is the price of parallelism touching the immutable bridge: paid as a re-anchor *check*, never by editing the frozen scope in place. If a depended-on contract changed incompatibly, the slice's scope is invalid — close the sprint and create a new one rather than coding against a stale freeze.

If no sprint exists, use `create-sprint`. If the sprint exists but does not match the work, do not edit scope in place; close or descope it and create a new sprint.

---

## Step 4 - Correlate Against Current Code

Inspect the current repository before changing files.

**Resume from the progress ledger first.** If a `sprint-NNN-*.ledger.md` exists for this sprint, read it before re-deriving anything. Restore: which plan phase is in progress, the current red/green test posture, the verify-attempt counter, and what's left. This prevents context amnesia across sessions — do not reconstruct progress from scratch when the ledger already records it.

Record:

- **Codebase touchpoints:** capabilities, modules, files, public contracts, migrations, UI surfaces
- **Existing tests:** Logic, Composition, Adapter Contract, Integration boundary, Journey coverage relevant to the change
- **Toolchain:** runtime, package manager, lint/format/type/test commands
- **Integrations:** third-party services, feature flags, environment variables, queues, databases
- **Active ADRs:** decisions constraining the implementation
- **Known hazards:** missing tests, layout violations, anti-dumping debt, flaky tests, ambiguous ownership

The implementation plan must close the gap between the wave target state and this current state.

---

## Step 5 - Decide Test Posture

Classify every impacted behavior before implementation.

| Situation                        | Required action                                                                                                                                            |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| New behavior                     | Add the lowest-layer test that specifies it, run it, observe `RED` for the expected reason.                                                                |
| Behavior change                  | Update the existing lowest-layer test or add one, run it, observe `RED` for the expected reason.                                                           |
| Refactor with preserved behavior | Run the relevant existing tests and observe `GREEN` before editing. If no test exists, add a characterization test and observe `GREEN` before refactoring. |
| Behavior unclear                 | Stop and clarify the wave, sprint, or acceptance criteria.                                                                                                 |
| Baseline test already failing    | Record the failure and ask whether to fix it first or mark it as an external blocker.                                                                      |

Use `test-by-ownership` to choose the layer:

- Logic - pure rules, validation, transformation
- Composition - service/API orchestration with in-memory Port adapters
- Adapter Contract - shared Port contract suite against in-memory and real adapters
- Integration boundary - external dependency wrapper behavior
- Journey - full user goal through the real public surface

Do not duplicate the same property at multiple layers.

---

## Step 6 - Produce The Contribution Envelope

Before implementation, output this compact envelope. Trivial-tier changes use the abbreviated form; Standard and Major use the full form.

### Trivial envelope

```markdown
## Code Contribution Intake (Trivial)

Tier: Trivial Reason: [why this qualifies — LOC, single capability, no contract change, no behavior change] Files touched: [paths] Green-baseline: [verify command + expected green tests] Verification: [single `verify` entry point]
```

### Standard / Major envelope

```markdown
## Code Contribution Intake

Tier: Standard | Major Wave: [name + path] Thin-slice: [TS-NNN + title] Sprint: [sprint file] Wave specs: [README/product-design/product-architecture/qa status] Design approval (Major only): [ADR.<ID> status: Accepted | Pending | n/a] + [sprint Design Approval line ref]

Current code touchpoints:

- [capability/file/contract]

Behavior impact: [changes behavior | preserves behavior | mixed]

Test posture:

- Red-first: [test file/name + expected red reason]
- Green-baseline: [test command/file/name]
- Missing coverage to add: [layer + intent]

Implementation boundary:

- In scope: [specific work]
- Out of scope: [explicit exclusions]

Verification:

- Single entry point: [project `verify` command]
```

The envelope may live in the sprint working notes if useful, but it is not a permanent product artifact. Permanent learnings flow through `close-sprint`.

---

## Step 7 - Hand Off By Tier

After intake is complete, hand off according to the declared tier. The principal-engineer persona switches modes based on the active skill (see `agents/principal-engineer.agent.md`).

| Tier     | Hand-off path                                                                                                                                                                                                                                                                                                                                    |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Trivial  | → `verify-and-assemble-pr` (reviewer mode). Skip architect and implementer ceremonies.                                                                                                                                                                                                                                                           |
| Standard | → `implement-with-defensive-patterns` (implementer mode) → `verify-and-assemble-pr` (reviewer mode). Existing wave specs are the design package.                                                                                                                                                                                                 |
| Major    | → `discovery-and-ambiguity-log` → `design-system-architecture` → `design-capability-layout` → `create-adr` (architect mode) → wait for **mechanical Design Approval** (`status: Accepted` ADR + signed Design Approval line in sprint file) → `implement-with-defensive-patterns` (implementer mode) → `verify-and-assemble-pr` (reviewer mode). |

In every tier:

- Follow red/green/refactor according to the test posture.
- `close-sprint` runs only after acceptance criteria and outcome evidence are verified.
- If implementation discovers a design flaw on a Major change, **stop** and re-enter architect mode for clarification — do not silently edit ADRs from implementer mode.

---

## Quality Checklist

- [ ] Tier declared (Trivial / Standard / Major) with reason
- [ ] (Standard/Major) Wave and thin-slice identified
- [ ] (Standard/Major) Thin-slice acceptance criteria are testable
- [ ] (Standard/Major) All four wave specs exist and are specific enough
- [ ] (Standard/Major) Sprint bridge exists and matches the requested work
- [ ] (Standard/Major) Sprint Plan Approval line is signed (or `n/a (tier: Trivial)`)
- [ ] (Standard/Major) Every acceptance criterion maps to ≥1 test in the traceability matrix
- [ ] (Standard/Major) Production-Readiness conformance block present; seams named and each anchor conforming or a reviewed deviation recorded
- [ ] (Major) Mechanical Design Approval present: ADR `status: Accepted` and signed Design Approval line in active sprint file
- [ ] Engineering current-state snapshot read or refreshed
- [ ] Current code touchpoints inspected
- [ ] Test impact classified as red-first, green-baseline, characterization, or blocked
- [ ] Test layer chosen with `test-by-ownership`
- [ ] Contribution envelope produced before implementation

---

## Anti-Patterns

- Misclassifying a behavior-changing contribution as Trivial to escape intake
- Fabricating a wave / thin-slice / sprint to satisfy Standard/Major intake when the change is genuinely Trivial
- Coding directly from the user's prompt without finding the wave and thin-slice (Standard/Major)
- Treating a sprint as optional because the task seems obvious (Standard/Major)
- Starting Standard/Major implementation before the Sprint Plan Approval line is signed
- Leaving an acceptance criterion with no mapped test and proceeding anyway
- Letting missing `product-design.md`, `product-architecture.md`, or `qa.md` become implementation guesswork
- Starting Major-tier implementation before mechanical Design Approval is recorded
- Implementer silently editing an Accepted ADR mid-flight instead of bouncing back to architect mode
- Writing production code before red-first tests for changed behavior
- Refactoring without green-baseline evidence
- Adding a Journey assertion for a property already proven by a Logic or Composition test
- Editing live sprint scope instead of creating a new bridge
