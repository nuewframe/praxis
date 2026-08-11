# SPRINT.260811.02: AST Readiness Skill, Meta-Commentary Guardrails & Instructions (Praxis `v0.7.1`)

**Status:** 🟢 Active  
**Initiative:** [INIT.ast-onboarding-and-guardrails](../initiatives/INIT.ast-onboarding-and-guardrails.md)  
**Thin-Slices:** TS-035, TS-036, TS-037, TS-038, TS-039  
**Started:** 2026-08-11  
**Completed:** —

---

## Sprint Goal

Establish the `prepare-project-for-ast` skill to automate project toolchain diagnostics, AST seam discovery, and `.seam-contracts.json` generation. Update `capability-driven-guardrails.instructions.md`, `lean-delivery-guardrails.instructions.md`, `define-seam-contract`, and `provision-project-overlay` to mandate consistent polyglot AST usage and strictly prohibit meta-commentary (defensiveness, throat-clearing, and rationale deflection) across all AI agent workflows.

---

## Hypothesis Card (Lean Validation)

**Hypothesis:** We believe introducing `skills/prepare-project-for-ast` and enforcing anti-meta-commentary prose guardrails across Praxis instructions will ensure 100% of target repositories automatically initialize AST seam tracking while producing concise, non-defensive documentation.

**Validation method:** We will know this is true when:
1. `skills/prepare-project-for-ast/SKILL.md` passes toolchain checks, populates `.seam-contracts.json`, and verifies `scripts/ast_parse.sh`.
2. `instructions/capability-driven-guardrails.instructions.md` mandates AST-backed seam validation.
3. `instructions/lean-delivery-guardrails.instructions.md` and `validate-plugin.sh` enforce the anti-meta-commentary 3-part test and catch defensive phrases.
4. `skills/define-seam-contract/SKILL.md` and `skills/provision-project-overlay/SKILL.md` reference AST project preparation.
5. `scripts/validate-plugin.sh` and `scripts/bump-version.sh --audit` pass 100% clean at version `0.7.1`.

**Decision rule:**
- **Continue** if: `prepare-project-for-ast` initializes target repos in $< 1\text{sec}$ with zero configuration errors.
- **Pivot** if: Missing compiler toolchains cause silent build failures.

---

## Scope (Immutable Once Started)

### Thin-Slices Included

- [ ] **TS-035: Dedicated AST Onboarding Skill (`skills/prepare-project-for-ast/SKILL.md`)** — Create skill for automated AST toolchain verification, interface discovery, and manifest generation.
- [ ] **TS-036: AST Guardrails & Instruction Mandate (`instructions/capability-driven-guardrails.instructions.md`)** — Add explicit "AST-Backed Seam & Interface Conformance" rule to always-on instructions.
- [ ] **TS-037: Boundary & Provisioning Skill Integration** — Update `skills/define-seam-contract/SKILL.md` and `skills/provision-project-overlay/SKILL.md` to mandate AST project preparation.
- [ ] **TS-038: Anti-Meta-Commentary Instruction & Linter Probe (`instructions/lean-delivery-guardrails.instructions.md`)** — Add anti-meta-commentary guardrails (3-part test, bare assertion conversion, 3 tells) and update `validate-plugin.sh` terminology scanner.
- [ ] **TS-039: Release & Verification (`v0.7.1`)** — Bump single-source version to `0.7.1`, update `CHANGELOG.md`, and verify 100% clean validation pass.

---

## Mechanical Sprint Approval Gates

- [x] **Sprint Plan Approval:** Reviewed by Principal Engineer & Product Manager | Date: 2026-08-11 | Scope confirmed
- [x] **Design Approval:** Accepted [ADR.260811.01](../../architecture/adr/ADR.260811.01-ast-multi-language-parser-infrastructure.md) | Date: 2026-08-11

---

## Implementation Plan

### Phase 1: AST Readiness Skill (`TS-035`)
- Create `skills/prepare-project-for-ast/SKILL.md` with toolchain diagnostics, `check-seam-contract-parity.sh --generate` runner, and `ast_parse.sh` sanity check.

### Phase 2: Instruction Mandate (`TS-036`)
- Update `instructions/capability-driven-guardrails.instructions.md` with AST-backed seam parsing rule.

### Phase 3: Boundary Skill Integration (`TS-037`)
- Update `skills/define-seam-contract/SKILL.md` and `skills/provision-project-overlay/SKILL.md`.

### Phase 4: Anti-Meta-Commentary Guardrails (`TS-038`)
- Update `instructions/lean-delivery-guardrails.instructions.md` and `validate-plugin.sh` terminology checker to enforce the anti-meta-commentary 3-part test and 3 tells.

### Phase 5: Release & Verification (`TS-039`)
- Execute `bump-version.sh 0.7.1`, update `CHANGELOG.md`, and run audit probe.
