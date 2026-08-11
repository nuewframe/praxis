# INIT.ast-onboarding-and-guardrails: AST Onboarding Skill & Mandatory Guardrails (Praxis `v0.7.1`)

**Status:** 🟢 Active  
**Capabilities Touched:** `CAP.plugin-conformance-and-validation-probes`, `CAP.multi-harness-distribution`, `CAP.method-spine-and-execution`  
**Target Horizon:** Praxis `v0.7.1` Patch/Minor  

---

## Executive Summary & Hypothesis ($Iteration_1$)

- **Business Intent & ROI:** Provide a dedicated project onboarding skill (`skills/prepare-project-for-ast`), mandatory engineering instructions (`instructions/capability-driven-guardrails.instructions.md`), and anti-meta-commentary prose rules (`instructions/lean-delivery-guardrails.instructions.md`) so Praxis AI agents and developers consistently enforce polyglot AST parsing (`ast-parser@v1`) and produce non-defensive documentation.
- **Hypothesis:** We believe adding `skills/prepare-project-for-ast` and updating system instructions with explicit AST parsing mandates and anti-meta-commentary rules will ensure 100% of target repositories automatically initialize AST seam tracking while producing clean, bare-assertion specification prose.
- **Validation Method:** `prepare-project-for-ast` executed against multi-language test fixtures in `scripts/__fixtures__/` returning clean toolchain status, auto-generating `.seam-contracts.json`, and passing `scripts/validate-plugin.sh` 15/15 checks clean.

---

## User Value & Thin-Slices Roadmap ($Iteration_1 \rightarrow Iteration_2$)

### Thin-Slices Table

| Slice ID | Outcome & User Value | Status | SPRINT Link |
| -------- | -------------------- | ------ | ----------- |
| `TS-035` | Dedicated AST Onboarding Skill (`skills/prepare-project-for-ast/SKILL.md`) | ✅ Complete | [SPRINT.260811.02](../sprints/SPRINT.260811.02-ast-readiness-skill-and-instructions.md) |
| `TS-036` | AST Guardrails & Instruction Mandate (`instructions/capability-driven-guardrails.instructions.md`) | ✅ Complete | [SPRINT.260811.02](../sprints/SPRINT.260811.02-ast-readiness-skill-and-instructions.md) |
| `TS-037` | Boundary Skill Integration (`skills/define-seam-contract/SKILL.md` & `skills/provision-project-overlay/SKILL.md`) | ⚪ | [SPRINT.260811.02](../sprints/SPRINT.260811.02-ast-readiness-skill-and-instructions.md) |
| `TS-038` | Anti-Meta-Commentary Guardrails (`instructions/lean-delivery-guardrails.instructions.md`, `validate-plugin.sh`) | ⚪ | [SPRINT.260811.02](../sprints/SPRINT.260811.02-ast-readiness-skill-and-instructions.md) |
| `TS-039` | Praxis `v0.7.1` Release & Verification (`scripts/bump-version.sh 0.7.1`) | ⚪ | [SPRINT.260811.02](../sprints/SPRINT.260811.02-ast-readiness-skill-and-instructions.md) |

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

#### AC-3: Anti-Meta-Commentary Prose Enforcement
- **Given:** An AI agent drafting or editing specification, design, or architecture documentation.
- **When:** The agent writes text.
- **Then:** The agent applies the 3-part test to eliminate meta-commentary, throat-clearing, and defensive rationale deflection ("by design, not by oversight"), writing bare assertions instead of rationale arguments.
