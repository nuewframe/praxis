# SPRINT.260811: AST-Backed Seam & Probe Validation (Praxis `v0.7.0`)

**Status:** 🟢 Active  
**Initiative:** [INIT.ast-seam-and-probe-validation](../initiatives/INIT.ast-seam-and-probe-validation.md)  
**Thin-Slices:** TS-030, TS-031, TS-032, TS-033, TS-034  
**Started:** 2026-08-11  
**Completed:** —

---

## Sprint Goal

Upgrade Praxis validation probes (`check-port-adapter-parity.sh`, `check-seam-contract-parity.sh`, `check-observability-at-seams.sh`) from regex text heuristics to AST-backed static analysis across the 6 primary enterprise stacks (TypeScript, Python, Go, Rust, C#, Java/Kotlin). Establish automated seam contract extraction from AST code structures into `.seam-contracts.json`, and bump the plugin version to **Praxis `v0.7.0`**.

---

## Hypothesis Card (Lean Validation)

**Hypothesis:** We believe upgrading Praxis validation probes to polyglot AST-backed parsing (via language-native AST runners for TS, Py, Go, Rust, C#, and JVM) will eliminate false positives/negatives in complex codebases, enable automated seam contract generation directly from code interfaces, and provide robust cross-language seam enforcement for `v0.7.0`.

**Validation method:** We will know this is true when:
1. `check-port-adapter-parity.sh` parses AST interface nodes to verify adapter implementations without relying on regex grep.
2. `check-seam-contract-parity.sh` extracts payload shapes directly from TS, Py, Go, Rust, C#, and JVM ASTs into `.seam-contracts.json`.
3. `check-observability-at-seams.sh` verifies correlation ID propagation and log/metric calls within AST parent blocks of boundary calls.
4. All existing fixtures in `scripts/__fixtures__/` pass AST parsing probes cleanly.
5. Plugin version is bumped to `0.7.0` via `scripts/bump-version.sh 0.7.0` and verified in sync by `scripts/validate-plugin.sh` and `bump-version.sh --audit`.

**Decision rule:**
- **Continue** if: AST parsing completes in $< 500\text{ms}$ across large codebases and zero false positives are reported on real multi-language repos.
- **Pivot** if: AST parsing requires heavy binary dependencies that fail on POSIX environments.
- **Stop** if: AST parsing degrades test suite performance or breaks zero-dependency installation.

---

## Risks (Pre-Mortem Seed)

| Risk | Likelihood | Impact | Mitigation / trigger |
| ---- | ---------- | ------ | -------------------- |
| Multi-language AST parsing requires external compiler binaries | M | H | Re-use language-native tools (`node`, `python3`, `go`, `rustc`, `dotnet`, `java`) already present on developer machines. |
| AST node extraction fails on novel language syntax constructs | M | M | Fallback to structural heuristic scan when AST parsing encounters unparseable syntax nodes. |
| Version literal drift during release bump to `0.7.0` | L | H | Execute `scripts/bump-version.sh 0.7.0` and verify with `bump-version.sh --audit`. |

---

## Scope (Immutable Once Started)

### Thin-Slices Included

- [ ] **TS-030: Polyglot AST Parsing Infrastructure (`scripts/ast_parse.sh`)** — Establish language-native AST parser runners for TypeScript, Python, Go, Rust, C#, and Java/Kotlin.
- [ ] **TS-031: AST-Backed Port/Adapter Parity Probe (`check-port-adapter-parity.sh`)** — Upgrade `check-port-adapter-parity.sh` to parse interface declarations and verify adapter implementation methods via AST.
- [ ] **TS-032: AST-Backed Seam Contract Generator (`check-seam-contract-parity.sh`)** — Upgrade `check-seam-contract-parity.sh` to extract seam payload shapes directly from AST nodes into `.seam-contracts.json`.
- [ ] **TS-033: AST-Backed Seam Observability Probe (`check-observability-at-seams.sh`)** — Upgrade `check-observability-at-seams.sh` to verify correlation ID propagation within AST parent blocks of boundary calls.
- [ ] **TS-034: Praxis `v0.7.0` Release & Version Bump** — Bump plugin version from `0.6.0` to `0.7.0` using `scripts/bump-version.sh 0.7.0`, update `CHANGELOG.md` under `## [0.7.0]`, and run audit probe.

### Out of Scope

- Modifying runtime harness distribution manifests beyond version string bump.
- Adding binary tree-sitter native C bindings that require C compilation on host machines.

---

## Mechanical Sprint Approval Gates

- [x] **Sprint Plan Approval:** Reviewed by Principal Engineer & Product Manager | Date: 2026-08-11 | Scope confirmed
- [x] **Design Approval:** Accepted [ADR.260811.01](../../architecture/adr/ADR.260811.01-ast-multi-language-parser-infrastructure.md) | Date: 2026-08-11

---

## Implementation Plan

### Phase 1: Polyglot AST Parsing Infrastructure (`TS-030`)
- Create `scripts/ast_parse.sh` dispatcher bridge and language-native runners (`ast_parse_ts.js`, `ast_parse_py.py`, `ast_parse_go.go`, `ast_parse_rs.rs`, `ast_parse_cs.cs`, `ast_parse_kt.kt`).

### Phase 2: Probe Upgrades (`TS-031`, `TS-032`, `TS-033`)
- Refactor `scripts/check-port-adapter-parity.sh` to consume `ast_parse.sh`.
- Refactor `scripts/check-seam-contract-parity.sh` to extract seam contract payload shapes from AST nodes.
- Refactor `scripts/check-observability-at-seams.sh` to inspect AST parent call blocks.

### Phase 3: Version Bump & Release `v0.7.0` (`TS-034`)
- Run `scripts/bump-version.sh 0.7.0`.
- Document release notes in `CHANGELOG.md`.
- Verify `scripts/validate-plugin.sh` and `scripts/bump-version.sh --audit`.
