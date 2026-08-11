# INIT.ast-seam-and-probe-validation: AST-Backed Seam & Probe Validation (Praxis `v0.7.0`)

**Status:** ⚪ Proposed  
**Capabilities Touched:** `CAP.plugin-conformance-and-validation-probes`, `CAP.method-spine-and-execution`  
**Target Horizon:** Praxis `v0.7.0` Release  

---

## Executive Summary & Hypothesis ($Iteration_1$)

- **Business Intent & ROI:** Replace regex text heuristics in seam parity and port-adapter probes (`check-port-adapter-parity.sh`, `check-seam-contract-parity.sh`, `check-observability-at-seams.sh`) with AST-backed static analysis for TypeScript, Go, and Python. Establish automated seam contract extraction directly from AST interfaces into `.seam-contracts.json`.
- **Hypothesis:** We believe upgrading Praxis validation probes to AST-backed parsing (via lightweight AST syntax trees for TS, Go, and Python) will eliminate false positives/negatives in complex codebases, enable automated seam contract generation directly from code interfaces, and provide robust cross-language seam enforcement for `v0.7.0`.
- **Validation Method:** `check-port-adapter-parity.sh` and `check-seam-contract-parity.sh` executed against multi-language test fixtures in `scripts/__fixtures__/` returning clean AST parsing verdicts in $< 500\text{ms}$.

---

## User Value & Thin-Slices Roadmap ($Iteration_1 \rightarrow Iteration_2$)

### Thin-Slices Table

| Slice ID | Outcome & User Value | Status | SPRINT Link |
| -------- | -------------------- | ------ | ----------- |
| `TS-030` | Multi-Language AST Parsing Infrastructure (`scripts/ast_parse.py` <!-- praxis:allow-path reason="unreleased script planned for TS-030 in `v0.7.0`" -->) | ⚪ | [SPRINT.260811](../sprints/SPRINT.260811-ast-seam-and-probe-validation.md) |
| `TS-031` | AST-Backed Port/Adapter Parity Probe (`check-port-adapter-parity.sh`) | ⚪ | [SPRINT.260811](../sprints/SPRINT.260811-ast-seam-and-probe-validation.md) |
| `TS-032` | AST-Backed Seam Contract Generator (`check-seam-contract-parity.sh`) | ⚪ | [SPRINT.260811](../sprints/SPRINT.260811-ast-seam-and-probe-validation.md) |
| `TS-033` | AST-Backed Seam Observability Probe (`check-observability-at-seams.sh`) | ⚪ | [SPRINT.260811](../sprints/SPRINT.260811-ast-seam-and-probe-validation.md) |
| `TS-034` | Praxis `v0.7.0` Release & Version Bump (`scripts/bump-version.sh 0.7.0`) | ⚪ | [SPRINT.260811](../sprints/SPRINT.260811-ast-seam-and-probe-validation.md) |

---

## Progressive Refinement ($Iteration_2 \rightarrow Iteration_N$)

### User Experience (UX Deltas)
- Probe failure output states exact AST node (Interface name, Method signature, Contract version) rather than raw regex diff.

### Technical Architecture (Seams & Educated Theory)
- `scripts/ast_parse.py` <!-- praxis:allow-path reason="unreleased script planned for TS-030 in `v0.7.0`" --> provides lightweight AST parsing wrappers for Python `ast`, TypeScript JSON AST, and Go AST.

### Quality & NFR Invariants
- AST parsing completes in $< 500\text{ms}$ across large repositories without requiring native C compilation dependencies.
