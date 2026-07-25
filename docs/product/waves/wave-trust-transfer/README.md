# WAVE: Trust Transfer

> **Delivered before wave adoption — a derived record, not a plan.** Reconstructed from validated truth (release history, ADRs, capability records) after the work shipped. It carries no hypothesis card and no acceptance criteria because none were written at the time; each slice cites the evidence for what it delivered. Current-state architecture lives in [docs/architecture/](../../../architecture/). Derivation rules: [ADR.260725.10](../../../architecture/adr/ADR.260725.10-brownfield-wave-retrofit.md).

**Status:** ✅ Complete (delivered before wave adoption)\
**Goal:** A human reviewer can tell whether an agent actually did the disciplined work, and see precisely where it did not — instead of inheriting an artifact that looks identical either way.

---

## Tracking Rules

- Track each thin-slice by intended user outcome and current state only.
- If a thin-slice is reopened or corrected, keep the same slice ID and add one short tracking note next to that slice.
- Keep implementation history in sprint files and version control. This README stays focused on product intent.

---

## Value Theme

This is the problem the whole method exists to solve, named explicitly. An agent-generated architecture document looks the same whether the agent reasoned hard or pattern-matched a template. Structure is cheap for a model to produce; substance is not. Every gate that checks *shape* can be satisfied by a convincing-looking artifact, so shape-checking alone transfers no trust. This wave makes the difference between reasoning and imitation visible, and is honest that some of it is unenforceable.

---

## Scope

- The problem stated plainly in the method's own constitution, so the method can be judged against it.
- A test that stops rules from shipping merely because they are defensible.
- A review that grades whether an artifact's reasoning has substance, distinct from whether its structure is present.
- One aggregated per-PR artifact a human reads instead of re-deriving trust.
- Structural prevention of the drift that makes restated facts diverge.

**Out of scope:**

- Certifying that a decision is *correct*. These mechanisms grade whether reasoning happened, never whether it reached the right answer.
- Whether Praxis obeys any of this itself — that is [wave-self-conformance](../wave-self-conformance/README.md).

---

## Thin-Slices

### TS-001: The method states the problem it is solving

> **Status:** ✅ Complete

**User Value:** As an adopter, I need to know what the method is *for* so that I can judge whether it delivers, rather than inferring its purpose from a list of skills.

**Evidence:** The `## Problem` section in [docs/project-context.md](../../../project-context.md) § 2 — trust transfer, with execution fidelity named as the primary output — plus the method-at-a-glance spine and the stage-by-stage "where fidelity is made" table. Shipped in the [course-correction release](../../../../CHANGELOG.md#040--2026-07-20).

---

### TS-002: A rule must improve fidelity, not merely be defensible

> **Status:** ✅ Complete

**User Value:** As a maintainer, I need a test that rejects a rule which is universal and disciplined but does not measurably help, so that the method stops accreting reasonable-sounding weight.

**Evidence:** The fourth scope-litmus question in [docs/project-context.md](../../../project-context.md) § Governance — "does it measurably improve the agent's execution fidelity, or close a known agent failure mode?" — with universality made necessary but not sufficient. The evolution policy was strengthened in the same release to make real-repo validation non-negotiable before a minor bump, and to require a bump's entry to cite that evidence.

**Tracking note:** The question has since removed a shipped rule, which is the test working rather than decorating: the wave category taxonomy failed it and was deleted per [ADR.260724](../../../architecture/adr/ADR.260724-wave-category-relaxation.md).

---

### TS-003: An artifact's reasoning is graded, not just its shape

> **Status:** ✅ Complete

**User Value:** As a reviewer, I need to know whether an ADR's alternatives, a signed approval, an ambiguity log, or a risk register carries real substance or is boilerplate, because a shape-checking probe cannot tell the difference.

**Evidence:** `verify-and-assemble-pr` Step 6, the artifact-fidelity review — a separate head renders Substantive / Likely hollow / N/A against each artifact the slice produced, quoting the passage that grounds each verdict. Decided in [ADR.260720.03](../../../architecture/adr/ADR.260720.03-fidelity-review-and-trust-receipt.md).

**Tracking note:** A warn-signal for human judgment, not a hard gate, and deliberately so. It does not certify that a decision is correct — only that reasoning is present.

---

### TS-004: One block a human reads instead of re-deriving trust

> **Status:** ✅ Complete

**User Value:** As a reviewer, I need gate status, escape-hatch usage, and fidelity verdicts aggregated into one place so that assessing a PR's trustworthiness is a single read rather than an investigation.

**Evidence:** `verify-and-assemble-pr` Step 7's Trust Receipt, aggregating gate-kind status (script-enforced / human-signed / agent-attested), escape-hatch usage sourced from `check-escape-hatch-usage.sh`, and Step 6's verdicts. Praxis's first concrete artifact answering its own problem statement directly. The three-tier enforcement split is disclosed in `README.md`'s opening pitch and guarded by a required-phrase lint so the disclosure cannot silently regress.

---

### TS-005: A fact restated in three places cannot silently diverge

> **Status:** ✅ Complete

**User Value:** As a reader of any surface, I need duplicated facts to agree, because acting on a stale copy is indistinguishable from acting on a correct one.

**Evidence:** `scripts/data/tier-classification.json` as the single source for the tier facts, rendered by `scripts/gen-tier-table.sh` into three per-surface formats inside `BEGIN/END GENERATED` markers, with `--check` in CI. Decided in [ADR.260720.02](../../../architecture/adr/ADR.260720.02-generated-tier-table.md), which records that this exact failure shape had already caused a real defect — a Major-tier workflow that could never terminate because phase ordering was restated inconsistently.

---

## Success Criteria

Delivered. The problem is named, a litmus test guards additions, artifact substance is graded separately from structure, the Trust Receipt aggregates the verdict, and one class of drift is structurally prevented.

The bar this wave explicitly did **not** clear is stated in its own release notes: real-repo validation. The evolution policy demands it before a rule ships, and the release that introduced the demand did not meet it. That remains true.

---

## Dependencies

- **Requires:** [wave-method-spine](../wave-method-spine/README.md); [wave-production-readiness](../wave-production-readiness/README.md) for the escape-hatch reporting the Trust Receipt consumes.
- **Enables:** an honest basis for adoption, and the self-conformance work that tests whether Praxis holds itself to any of it.
