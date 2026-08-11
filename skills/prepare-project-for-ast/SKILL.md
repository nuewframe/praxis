---
name: prepare-project-for-ast
mode: architect
tools: [read_file, file_search, grep_search, run_command, create_file]
description: >
  Prepare a project or repository for polyglot AST seam parsing and probe validation.
  Runs toolchain diagnostics across 6 primary enterprise stacks (TypeScript, Python,
  Go, Rust, C#, Java/Kotlin), auto-discovers interface files (*.ports.*, @seam), and
  generates or updates .seam-contracts.json via check-seam-contract-parity.sh --generate.
user-invocable: true
disable-model-invocation: false
---

# Skill: Prepare Project for AST Seam Parsing (`prepare-project-for-ast`)

Use this skill when onboarding an existing or greenfield repository to Praxis AST-backed seam validation (`ast-parser@v1`), or when verifying toolchain prerequisites before executing seam contract probes.

---

## Core Mental Model — Polyglot AST Readiness

Praxis validation probes (`check-port-adapter-parity.sh`, `check-seam-contract-parity.sh`, `check-observability-at-seams.sh`) consume `scripts/ast_parse.sh` to extract exact interface shapes directly from source code ASTs without regex heuristics or external C binary dependencies.

```
TARGET REPOSITORY                  AST DIAGNOSTICS & ONBOARDING                  PRAXIS MANIFEST
─────────────────                  ────────────────────────────                  ───────────────
Source files (*.ts, *.py,          1. Check native compiler APIs                 Generates/updates
*.go, *.rs, *.cs, *.kt)       ──>  2. Discover *.ports.* & @seam files      ──>  .seam-contracts.json
                                   3. Run check-seam-contract-parity --generate  with frozen <name>@vN
                                   4. Verify sub-50ms ast_parse.sh execution
```

---

## Step 1 — Verify Compiler Toolchain Prerequisites

Inspect developer environment and repository toolchains for language-native AST runners:

| Language Stack | Primary Runner | Fallback Bridge | Diagnostic Command |
| -------------- | -------------- | --------------- | ------------------ |
| **TypeScript / JS** | `node` (`scripts/ast_parse_ts.cjs`) | `python3` bridge | `node --version` |
| **Python** | `python3` (`scripts/ast_parse_py.py`) | built-in `ast` stdlib | `python3 --version` |
| **Go** | `go` (`scripts/ast_parse_go.go` / `bin/ast_parse_go`) | `python3` bridge | `go version` |
| **Rust** | `python3` structural AST parser | `syn` parser | `python3 --version` |
| **C#** | `python3` structural AST parser | Roslyn API | `python3 --version` |
| **Java / Kotlin** | `python3` structural AST parser | JVM AST runner | `python3 --version` |

Run diagnostics to ensure at least one native runner or `python3` is present:

```bash
command -v node >/dev/null 2>&1 && echo "Node.js AST runner: available" || echo "Node.js AST runner: missing (using fallback)"
command -v python3 >/dev/null 2>&1 && echo "Python AST runner: available" || echo "Python AST runner: missing"
command -v go >/dev/null 2>&1 && echo "Go AST runner: available" || echo "Go AST runner: missing (using fallback)"
```

---

## Step 2 — Discover Interface Seam Files

Scan target source trees (`src/`, `packages/`, `services/`, `apps/`, `libs/`, `modules/`) for interface declarations:

1. `*.ports.ts`, `*.ports.py`, `*.ports.go`, `*.ports.rs`, `*.ports.cs`, `*.ports.java`, `*.ports.kt`
2. Files containing `@seam(<name>@vN)` annotations or interface signatures.

---

## Step 3 — Auto-Generate `.seam-contracts.json` Manifest

Execute the seam contract generator in `--generate` mode to auto-extract AST interface shapes into `.seam-contracts.json`:

```bash
bash scripts/check-seam-contract-parity.sh --generate .
```

Verify that `.seam-contracts.json` is created at the repository root with frozen `<name>@vN` entries:

```json
{
  "mode": "warn",
  "contractsDir": "docs/product/contracts",
  "seams": [
    {
      "id": "checkoutport@v1",
      "kind": "port",
      "shape": "src/checkout/checkout.ports.ts",
      "behavior": "src/checkout/checkout.contract.test.ts"
    }
  ]
}
```

---

## Step 4 — Run AST Parsing Sanity Verification

Test `scripts/ast_parse.sh` against discovered interface files to verify AST JSON extraction speed ($< 50\text{ms}$):

```bash
bash scripts/ast_parse.sh <path-to-first-discovered-port-file>
```

Assert stdout format follows the `ast-parser@v1` schema:

```json
{
  "file": "src/checkout/checkout.ports.ts",
  "language": "typescript",
  "interfaces": [
    {
      "name": "CheckoutPort",
      "line": 1,
      "methods": [
        {
          "name": "processOrder",
          "line": 2,
          "params": [{ "name": "order", "type": "OrderPayload" }],
          "returnType": "Promise<OrderResult>"
        }
      ]
    }
  ]
}
```

---

## Step 5 — Verify Probe Readiness

Execute standard Praxis probes to confirm zero false-positive warnings:

```bash
bash scripts/check-port-adapter-parity.sh .
bash scripts/check-seam-contract-parity.sh .
```

Report clean AST readiness summary to developer or pipeline.
