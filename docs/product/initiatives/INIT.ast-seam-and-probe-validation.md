# INIT.ast-seam-and-probe-validation: AST-Backed Seam & Probe Validation (Praxis `v0.7.0`)

**Status:** ⚪ Proposed  
**Capabilities Touched:** `CAP.plugin-conformance-and-validation-probes`, `CAP.method-spine-and-execution`  
**Target Horizon:** Praxis `v0.7.0` Release  

---

## Executive Summary & Hypothesis ($Iteration_1$)

- **Business Intent & ROI:** Replace regex text heuristics in seam parity and port-adapter probes (`check-port-adapter-parity.sh`, `check-seam-contract-parity.sh`, `check-observability-at-seams.sh`) with AST-backed static analysis across all 6 main enterprise stacks (**TypeScript**, **Python**, **Go**, **Rust**, **C#**, **Java/Kotlin**). Establish automated seam contract extraction directly from AST interfaces into `.seam-contracts.json`.
- **Hypothesis:** We believe upgrading Praxis validation probes to AST-backed parsing (via language-native AST runners for TS, Py, Go, Rust, C#, and JVM) will eliminate false positives/negatives in complex codebases, enable automated seam contract generation directly from code interfaces, and provide robust cross-language seam enforcement for `v0.7.0`.
- **Validation Method:** `check-port-adapter-parity.sh` and `check-seam-contract-parity.sh` executed against multi-language test fixtures in `scripts/__fixtures__/` returning clean AST parsing verdicts in $< 500\text{ms}$.

---

## User Value & Thin-Slices Roadmap ($Iteration_1 \rightarrow Iteration_2$)

### Thin-Slices Table

| Slice ID | Outcome & User Value | Status | SPRINT Link |
| -------- | -------------------- | ------ | ----------- |
| `TS-030` | Polyglot AST Parsing Infrastructure (`scripts/ast_parse.sh` for TS, Py, Go, Rust, C#, JVM) | ⚪ | [SPRINT.260811](../sprints/SPRINT.260811-ast-seam-and-probe-validation.md) |
| `TS-031` | AST-Backed Port/Adapter Parity Probe (`check-port-adapter-parity.sh`) | ⚪ | [SPRINT.260811](../sprints/SPRINT.260811-ast-seam-and-probe-validation.md) |
| `TS-032` | AST-Backed Seam Contract Generator (`check-seam-contract-parity.sh`) | ⚪ | [SPRINT.260811](../sprints/SPRINT.260811-ast-seam-and-probe-validation.md) |
| `TS-033` | AST-Backed Seam Observability Probe (`check-observability-at-seams.sh`) | ⚪ | [SPRINT.260811](../sprints/SPRINT.260811-ast-seam-and-probe-validation.md) |
| `TS-034` | Praxis `v0.7.0` Release & Version Bump (`scripts/bump-version.sh 0.7.0`) | ⚪ | [SPRINT.260811](../sprints/SPRINT.260811-ast-seam-and-probe-validation.md) |

---

## Progressive Refinement ($Iteration_2 \rightarrow Iteration_N$)

### 1. User Experience & Acceptance Criteria (Given/When/Then)

#### AC-1: Happy Path — Valid AST Interface Parity Across Stacks
- **Given:** A repository containing TypeScript interfaces, Go structs, Python protocols, Rust traits, C# interfaces, or Java/Kotlin interfaces implementing declared seam contracts (`<name>@vN`).
- **When:** `check-seam-contract-parity.sh` or `check-port-adapter-parity.sh` is executed.
- **Then:** The AST parser extracts exact method signatures and field types, confirms matching shapes in `.seam-contracts.json`, and outputs `ok` in $< 500\text{ms}$ with exit code 0.

#### AC-2: Boundary Violation — AST Method Signature Mismatch
- **Given:** An adapter method signature diverges from its declared Port interface (e.g. missing `ctx context.Context` parameter or changed return type).
- **When:** `check-port-adapter-parity.sh` runs.
- **Then:** The probe outputs structured AST line-level error:
  ```text
  check-port-adapter-parity: src/checkout/adapter.ts:42
    Interface: CheckoutPort (contract checkout-service@v1)
    Expected:  processOrder(order: OrderPayload): Promise<OrderResult>
    Actual:    processOrder(order: OrderPayload, flag?: boolean): Promise<OrderResult>
    Error:     Adapter method signature diverges from declared Port AST node.
  ```
  and fails closed with exit code 1.

#### AC-3: Fault Tolerance — Unparseable Source Syntax Recovery
- **Given:** A source code file containing a syntax error or unparseable experimental syntax feature.
- **When:** `scripts/ast_parse.sh` attempts to parse the AST.
- **Then:** The parser catches the parse exception, emits a warning stating `[WARN] AST parse skipped on src/legacy/broken.ts (line 12: syntax error), falling back to structural scan`, and continues validating remaining files without crashing.

---

### 2. UX State Transition & Ambiguity Matrix

| Input State | Condition / Trigger | Terminal UX Output | System Exit Code |
| ----------- | ------------------- | ------------------ | ---------------- |
| **Empty Codebase** | 0 seam contracts or ports declared | `check-seam-contract-parity: no contracts declared (n/a)` | `0` (Success) |
| **Valid AST Alignment** | 100% method signature & contract shape parity | `check-port-adapter-parity: all 12 ports verified ok` | `0` (Success) |
| **Signature Drift** | Method parameters or field types diverge | Prints AST node location, expected vs actual signature | `1` (Failure) |
| **Unparseable Syntax** | Syntax error in source file | Emits non-fatal warning + falls back to structural scan | `0` (Warn) |
| **Missing Contract ID** | Boundary call lacks frozen `<name>@vN` id | Prints file:line and instructions to run `define-seam-contract` | `1` (Failure) |

---

### 3. Technical Architecture (Seams & Educated Theory — $Iteration_3$)

- **Polyglot AST Parsing Engine (`scripts/ast_parse.sh`):** Dispatcher shell script delegating to language-native AST runners emitting a unified `ast-parser@v1` JSON stream across 6 main stacks:
  - **TypeScript / JS:** `scripts/ast_parse_ts.js` (Node.js compiler API)
  - **Python:** `scripts/ast_parse_py.py` (Python 3 stdlib `ast`)
  - **Go:** `scripts/ast_parse_go.go` (Go `go/ast`)
  - **Rust:** `scripts/ast_parse_rs.rs` (`rustc` / `syn` parser)
  - **C#:** `scripts/ast_parse_cs.cs` (.NET Roslyn API)
  - **Java / Kotlin:** `scripts/ast_parse_java.java` & `scripts/ast_parse_kt.kt` (JVM compiler parser)
- **Seam Contract Generator (`check-seam-contract-parity.sh`):** Extracts AST interface signatures and serializes them into `.seam-contracts.json`.
- **Architectural Decision Record:** [ADR.260811.01: Polyglot Language-Native AST Parsing Infrastructure](../../architecture/adr/ADR.260811.01-ast-multi-language-parser-infrastructure.md) (**Status: Accepted**).

---

### 4. Quality & NFR Invariants

- **Performance SLA:** AST parsing completes in $< 500\text{ms}$ across repositories with 500+ source files.
- **Zero Binary Dependency:** Uses standard library / lightweight JSON AST formatters without native C tree-sitter compilation requirements.
