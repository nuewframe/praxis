<!-- praxis:allow-path reason="ephemeral sprint file" -->
# SPRINT.260811: AST-Backed Seam & Probe Validation (Praxis `v0.7.0`)

**Status:** ⚪ Proposed\
**Wave:** wave-praxis-evolution\
**Thin-Slices:** TS-030, TS-031, TS-032, TS-033, TS-034\
**Started:** —\
**Completed:** —

---

## Sprint Goal

Upgrade Praxis validation probes (`check-port-adapter-parity.sh`, `check-seam-contract-parity.sh`, `check-observability-at-seams.sh`) from regex text heuristics to AST-backed static analysis for TypeScript, Go, and Python. Establish automated seam contract extraction from AST code structures into `.seam-contracts.json`, and bump the plugin version to **Praxis `v0.7.0`**.

---

<!-- praxis:allow-version-literal reason="cites target milestone in hypothesis card" -->
## Hypothesis Card (Lean Validation)

**Hypothesis:** We believe upgrading Praxis validation probes to AST-backed parsing (via lightweight AST syntax trees for TS, Go, and Python) will eliminate false positives/negatives in complex codebases, enable automated seam contract generation directly from code interfaces, and provide robust cross-language seam enforcement for `v0.7.0`.

**Validation method:** We will know this is true when:
1. `check-port-adapter-parity.sh` parses AST interface nodes to verify adapter implementations without relying on regex grep.
2. `check-seam-contract-parity.sh` extracts payload shapes directly from TypeScript interface and Go struct ASTs into `.seam-contracts.json`.
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
| Multi-language AST parsing requires external compiler binaries | M | H | Re-use standard Python `ast` module and lightweight TS/Go AST AST JSON parsers without heavy binary footprints. |
| AST node extraction fails on novel language syntax constructs | M | M | Fallback to structural heuristic scan when AST parsing encounters unparseable syntax nodes. |
| Version literal drift during release bump to `0.7.0` | L | H | Execute `scripts/bump-version.sh 0.7.0` and verify with `bump-version.sh --audit`. |

---

## Scope (Immutable Once Started)

### Thin-Slices Included

- [ ] **TS-030: Multi-Language AST Parsing Infrastructure (`scripts/ast_parse.py`)** <!-- praxis:allow-path reason="unreleased script planned for TS-030 in `v0.7.0`" --> — Establish lightweight AST parser wrappers for TypeScript, Go, and Python syntax trees.
- [ ] **TS-031: AST-Backed Port/Adapter Parity Probe (`check-port-adapter-parity.sh`)** — Upgrade `check-port-adapter-parity.sh` to parse interface declarations and verify adapter implementation methods via AST.
- [ ] **TS-032: AST-Backed Seam Contract Generator (`check-seam-contract-parity.sh`)** — Upgrade `check-seam-contract-parity.sh` to extract seam payload shapes directly from TS interface and Go struct ASTs into `.seam-contracts.json`.
- [ ] **TS-033: AST-Backed Seam Observability Probe (`check-observability-at-seams.sh`)** — Upgrade `check-observability-at-seams.sh` to verify correlation ID propagation within AST parent blocks of boundary calls.
- [ ] **TS-034: Praxis `v0.7.0` Release & Version Bump** — Bump plugin version from `0.6.0` to `0.7.0` using `scripts/bump-version.sh 0.7.0`, update `CHANGELOG.md` under `## [0.7.0]`, and run audit probe.

### Out of Scope

- Modifying runtime harness distribution manifests beyond version string bump.
- Adding binary tree-sitter native C bindings that require C compilation on host machines.

---

## Sprint Plan Approval

- [ ] **Sprint Plan Approval:** `[Pending Signature]` (Principal Engineer)

---

## Implementation Plan

### Phase 1: AST Parsing Infrastructure (`TS-030`)
- Create `scripts/ast_parse.py` <!-- praxis:allow-path reason="unreleased script planned for TS-030 in `v0.7.0`" --> to expose AST node parsing utilities for Python `ast`, TypeScript JSON AST, and Go AST output.

### Phase 2: Probe Upgrades (`TS-031`, `TS-032`, `TS-033`)
- Refactor `scripts/check-port-adapter-parity.sh` to consume `ast_parse.py`.
- Refactor `scripts/check-seam-contract-parity.sh` to extract seam contract payload shapes from AST nodes.
- Refactor `scripts/check-observability-at-seams.sh` to inspect AST parent call blocks.

### Phase 3: Version Bump & Release `v0.7.0` (`TS-034`)
- Run `scripts/bump-version.sh 0.7.0`.
- Document release notes in `CHANGELOG.md`.
- Verify `scripts/validate-plugin.sh` and `scripts/bump-version.sh --audit`.
