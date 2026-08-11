# INIT.ast-onboarding-and-guardrails: AST Onboarding Skill & Mandatory Guardrails (Praxis `v0.7.1`)

**Status:** 🟢 Active  
**Capabilities Touched:** `CAP.plugin-conformance-and-validation-probes`, `CAP.multi-harness-distribution`  
**Target Horizon:** Praxis `v0.7.1` Patch/Minor  

---

## Executive Summary & Hypothesis ($Iteration_1$)

- **Business Intent & ROI:** Provide a dedicated project onboarding skill (`skills/prepare-project-for-ast`) and mandatory engineering instructions (`instructions/capability-driven-guardrails.instructions.md`) so Praxis AI agents and developers consistently enforce polyglot AST parsing (`ast-parser@v1`) across TypeScript, Python, Go, Rust, C#, and Java/Kotlin codebases.
- **Hypothesis:** We believe adding `skills/prepare-project-for-ast` and updating `capability-driven-guardrails` with explicit AST parsing mandates will ensure 100% of newly provisioned or existing target repositories automatically discover interface files, verify compiler toolchains, populate `.seam-contracts.json`, and eliminate manual AST setup friction.
- **Validation Method:** `prepare-project-for-ast` executed against multi-language test fixtures in `scripts/__fixtures__/` returning clean toolchain status, auto-generating `.seam-contracts.json`, and passing `scripts/validate-plugin.sh` 15/15 checks clean.

---

## User Value & Thin-Slices Roadmap ($Iteration_1 \rightarrow Iteration_2$)

### Thin-Slices Table

| Slice ID | Outcome & User Value | Status | SPRINT Link |
| -------- | -------------------- | ------ | ----------- |
| `TS-035` | Dedicated AST Onboarding Skill (`skills/prepare-project-for-ast/SKILL.md`) | ⚪ | [SPRINT.260811.02](../sprints/SPRINT.260811.02-ast-readiness-skill-and-instructions.md) |
| `TS-036` | AST Guardrails & Instruction Mandate (`instructions/capability-driven-guardrails.instructions.md`) | ⚪ | [SPRINT.260811.02](../sprints/SPRINT.260811.02-ast-readiness-skill-and-instructions.md) |
| `TS-037` | Boundary Skill Integration (`skills/define-seam-contract/SKILL.md` & `skills/provision-project-overlay/SKILL.md`) | ⚪ | [SPRINT.260811.02](../sprints/SPRINT.260811.02-ast-readiness-skill-and-instructions.md) |
| `TS-038` | Praxis `v0.7.1` Release & Verification (`scripts/bump-version.sh 0.7.1`) | ⚪ | [SPRINT.260811.02](../sprints/SPRINT.260811.02-ast-readiness-skill-and-instructions.md) |

---

## Progressive Refinement ($Iteration_2 \rightarrow Iteration_N$)

### 1. User Experience & Acceptance Criteria (Given/When/Then)

#### AC-1: Project Preparation for AST Parsing
- **Given:** A repository containing TypeScript, Python, Go, Rust, C#, or JVM interface code.
- **When:** The user or agent invokes `prepare-project-for-ast`.
- **Then:** The skill runs toolchain diagnostics, executes `check-seam-contract-parity.sh --generate` to populate `.seam-contracts.json`, verifies `scripts/ast_parse.sh`, and outputs a clean readiness summary.

#### AC-2: Mandatory Agent AST Guardrail Compliance
- **Given:** An AI agent generating or validating interface seams or adapter implementations.
- **When:** The agent evaluates source code files.
- **Then:** The agent adheres to `capability-driven-guardrails.instructions.md` by using `scripts/ast_parse.sh` (`ast-parser@v1`) instead of regex heuristics.
