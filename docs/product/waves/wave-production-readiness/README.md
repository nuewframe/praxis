# WAVE: Production-Readiness Posture

> **Delivered before wave adoption — a derived record, not a plan.** Reconstructed from validated truth (release history, ADRs, capability records) after the work shipped. It carries no hypothesis card and no acceptance criteria because none were written at the time; each slice cites the evidence for what it delivered. Current-state architecture lives in [docs/architecture/](../../../architecture/). Derivation rules: [ADR.260725.10](../../../architecture/adr/ADR.260725.10-brownfield-wave-retrofit.md).

**Status:** ✅ Complete (delivered before wave adoption)\
**Goal:** A team decides its runtime posture once, at the wave, and every slice conforms to it mechanically — instead of re-litigating observability, config, statelessness, and resilience per change and marking them `N/A`.

---

## Tracking Rules

- Track each thin-slice by intended user outcome and current state only.
- If a thin-slice is reopened or corrected, keep the same slice ID and add one short tracking note next to that slice.
- Keep implementation history in sprint files and version control. This README stays focused on product intent.

---

## Value Theme

"Is it production ready?" asked per change gets answered per change, inconsistently, and usually with a checkbox. Four runtime properties — observable, configurable, horizontally scalable, resilient — are architectural decisions that belong to the wave, not the slice. Deciding them once and then checking conformance converts four checklist lines into four build-time gates, and removes the `N/A` theater that a per-slice question invites.

---

## Scope

- The four anchors declared once at the wave, each naming the probe that enforces it.
- Per-slice conformance that confirms the posture is *preserved*, with reasoned deviations rather than blanks.
- An executable probe per anchor, warn-first with mechanical promotion to fail-closed.
- An intake gate refusing work whose conformance block leaves an anchor unaddressed.

**Out of scope:**

- The seam construct the probes attach to — that is [wave-executable-seams](../wave-executable-seams/README.md).
- Whether the *plugin itself* runs these probes against its own tree — that is `TS-005` of [wave-self-conformance](../wave-self-conformance/README.md).

---

## Thin-Slices

### TS-001: Runtime posture is decided at the wave, not per change

> **Status:** ✅ Complete

**User Value:** As an architect, I need to state the correlation contract, config and secrets strategy, statelessness boundary, and cross-slice failure model once, so that every slice inherits them instead of inventing its own answer.

**Evidence:** The "Declare the Production-Readiness Posture" step and posture table in `skills/create-product-architecture-spec/SKILL.md`, where each anchor names its enforcing probe. Shipped in the [seam-contract release](../../../../CHANGELOG.md#020--2026-06-05) as Bundle B1/B2.

---

### TS-002: A slice states how it preserves the posture, or it does not start

> **Status:** ✅ Complete

**User Value:** As a reviewer, I need each slice to name the seams it touches and confirm it preserves the wave posture per anchor, so that "production readiness" is a claim about this change rather than a form.

**Evidence:** The "Production-Readiness Conformance" block in `skills/create-sprint/SKILL.md`, plus the gate in `skills/intake-code-contribution/SKILL.md` that stops Standard and Major work whose conformance block is blank or leaves an anchor unaddressed. Each `n/a` must carry its reason — the "reasoned n/a, never blank" rule.

---

### TS-003: Each anchor has a probe, and each probe can be promoted to fail-closed

> **Status:** ✅ Complete

**User Value:** As a team, I need each runtime property checked by a script so that conformance is measured rather than asserted, and I need to tighten each gate on my own schedule rather than adopting all four at full strictness at once.

**Evidence:** Four probes — `check-config-externalized.sh` (Configurable, the first one built), `check-observability-at-seams.sh` (Observable), `check-stateless-request-path.sh` (Horizontally scalable), and `check-resilient-boundary.sh` (Resilient). Each is warn-first with mechanical promotion to `enforce` via its own config file, carries a per-line or per-file reviewed opt-out marker, falls back through `scanPaths` to `.anti-dumping.json` then defaults, and is bash 3.2 compatible. All four run in `verify-and-assemble-pr`'s verify entry point and appear in the PR narrative checklist. Shipped across the [seam-contract release](../../../../CHANGELOG.md#020--2026-06-05) as Bundle B3.

**Tracking note:** These probes are heuristic scanners for risky shapes, not exact checks. The warn-first default and the reviewed opt-out markers exist because a heuristic that fails closed on day one trains a team to disable it.

---

### TS-004: Using an opt-out is never silent

> **Status:** ✅ Complete

**User Value:** As a reviewer, I need to see when a probe was opted out of, so that an escape hatch is a visible decision rather than a quiet one.

**Evidence:** `scripts/check-escape-hatch-usage.sh` scans a diff for the four `praxis:allow-*` markers and reports each by `file:line`. Informational by design, always exiting 0 — the point is visibility, not blocking, since escape hatches are sometimes correct. Feeds the Trust Receipt from [ADR.260720.03](../../../architecture/adr/ADR.260720.03-fidelity-review-and-trust-receipt.md).

---

## Success Criteria

Delivered. Four anchors, four probes, wave-level declaration, per-slice conformance, an intake gate, and visible opt-outs.

The honest limit: these gates protect a *host* repo only once it wires `verify.sh` into CI or installs the pre-push hook. Praxis ships them; a project has to run them. That asymmetry — and the fact that Praxis did not run them against itself — is the subject of `TS-005` of [wave-self-conformance](../wave-self-conformance/README.md).

---

## Dependencies

- **Requires:** [wave-method-spine](../wave-method-spine/README.md); [wave-executable-seams](../wave-executable-seams/README.md) for the seam the observability and resilience probes attach to.
- **Enables:** a wave-level posture that later work conforms to rather than re-deciding.
