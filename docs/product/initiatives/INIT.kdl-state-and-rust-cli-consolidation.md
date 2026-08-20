# INIT.kdl-state-and-rust-cli-consolidation: KDL State, Compiled Rust CLI Engine, and Deterministic Markdown Projections (Praxis `v0.8.0`)

**Status:** ⚪ Proposed  
**Capabilities Touched:** `CAP.plugin-conformance-and-validation-probes`, `CAP.method-spine-and-execution`  
**Target Horizon:** Praxis `v0.8.0` Release  

---

## Executive Summary & Hypothesis ($Iteration_1$)

- **Business Intent & ROI:** Replace fragile in-place regex mutations, multi-process bash probes (`check-*.sh`), and manual documentation synchronization with a strongly typed **KDL operational state layer** extending `docs/product/` and a high-performance **compiled Rust CLI engine (`praxis`)**. Treat Markdown documents not as mutable databases to be regex-patched, but as cleanly generated projections compiled directly from agile product state.
- **Hypothesis:** We believe establishing KDL as the single source of truth for agile product development state (initiatives, sprints, slice ledgers, gap registers, capability contracts, and decisions) backed by a compiled Rust CLI (`praxis`) will allow generators to synthesize fresh, authoritative Markdown files for the latest ADRs, living system capabilities (`CAP.`), user guides, and product dashboards (`docs/product.md`) with 100% mathematical fidelity and $<10\text{ms}$ execution speed for `v0.8.0`.
- **Validation Method:** `praxis check` and `praxis render` executed against a testbed workspace with 100+ KDL state files completing in $< 10\text{ms}$, verifying that all Markdown deliverables (ADRs, capabilities, user guides, dashboards) are freshly generated from KDL state without any regex mutations or formatting drift.

---

## User Value & Thin-Slices Roadmap ($Iteration_1 \rightarrow Iteration_2$)

### Thin-Slices Table

| Slice ID | Outcome & User Value | Status | SPRINT Link |
| -------- | -------------------- | ------ | ----------- |
| `TS-040` | Strongly Typed KDL State Models (`knuffel`/`kdl` for Sprints, Slices, Gaps, Initiatives, Capabilities, and ADRs) | ⚪ Proposed | — |
| `TS-041` | Unified Rust Verification Engine (`praxis check` with profile-aware checks for services vs. compilers/libraries) | ⚪ Proposed | — |
| `TS-042` | Whole-Document Markdown Projection Generators (`praxis render` generating `docs/product.md`, `CAP.<name>.md`, `ADR.<id>.md`, and User Guides from live state) | ⚪ Proposed | — |
| `TS-043` | Transactional Lifecycle & Ledger Automation (`praxis sprint create/close`, `praxis gap log` with closed finding vocabularies and zero-regex state updates) | ⚪ Proposed | — |
| `TS-044` | Project Overlay & Shell Harness Modernization (5-line lightweight `scripts/verify.sh` calling precompiled `praxis`) | ⚪ Proposed | — |
| `TS-045` | Praxis `v0.8.0` Release, Version Bump, & Migration CLI (`praxis migrate`) | ⚪ Proposed | — |

---

## Progressive Refinement ($Iteration_2 \rightarrow Iteration_N$)

### 1. User Experience & Acceptance Criteria (Given/When/Then)

#### AC-1: Happy Path — Invariant Validation on Valid KDL State
- **Given:** A repository containing valid `.praxis/` KDL documents (sprints, initiatives, gaps, decisions, and capability contracts).
- **When:** `praxis check` (or `bash scripts/verify.sh`) is executed.
- **Then:** The Rust engine parses all KDL files, validates domain invariants, executes configured quality gates, and exits `0` in $< 10\text{ms}`.

#### AC-2: Whole-Document Projection Generation (Zero Regex Splicing)
- **Given:** Current agile product state in `.praxis/` (including newly accepted ADRs, evolved capability contracts, and updated user journeys).
- **When:** `praxis render` (or `praxis render [capabilities|adr|guides|product]`) runs.
- **Then:** The CLI generators inspect the live agile state and synthesize fresh, perfectly structured Markdown files for `docs/capabilities/CAP.<name>.md`, `docs/architecture/adr/ADR.<id>.md`, `docs/guides/<guide>.md`, and `docs/product.md` without performing regex search-and-replace over old files.

#### AC-3: Gate Hard-Fail — Major Sprint Without Accepted ADR
- **Given:** A sprint declared with `tier="Major"` whose referenced ADR is either missing or status is `Draft`/`Proposed`.
- **When:** `praxis check` runs.
- **Then:** The CLI fails closed with exit code 1, emitting a compiler-grade diagnostic pointing to the exact line in the sprint's `.kdl` file.

#### AC-4: Premature Close Prevention — Unmet Criteria Without Recorded Gap
- **Given:** An active sprint where 2 acceptance criteria are unmet and no corresponding `gap-ref` (e.g. `G-042`) is assigned in residue.
- **When:** `praxis sprint close <ID>` is executed.
- **Then:** The command rejects the close operation with exit code 1, refusing to drop scope silently.

#### AC-5: Domain Profile Adaptation (Compilers/Libraries vs Services)
- **Given:** A project configured with `profile="compiler"` or `profile="library"` in `.praxis/config.kdl`.
- **When:** `praxis check` runs.
- **Then:** Service-specific probes (HTTP observability, remote config externalization, stateless request path) are omitted, while universal guardrails (anti-dumping, no-skipped-tests, no-sleep-waits, gap ratchets) are enforced.

---

### 2. UX State Transition & Ambiguity Matrix

| Input State | Condition / Trigger | Terminal UX Output | System Exit Code |
| ----------- | ------------------- | ------------------ | ---------------- |
| **Clean State** | All KDL state valid, all gates green | `praxis check: all 18 checks passed in 4.2ms` | `0` (Success) |
| **Render All Documents** | `praxis render` on updated agile state | `praxis render: generated 3 CAPs, 4 ADRs, 2 guides, and product.md (0 regex edits)` | `0` (Success) |
| **Missing ADR on Major Tier** | `tier="Major"` without `status="Accepted"` ADR | `praxis check: SPRINT.260819.01: Major tier requires an Accepted ADR (found: Draft)` | `1` (Failure) |
| **Unmet ACs on Close** | `praxis sprint close` with unfulfilled criteria | `praxis close: cannot close SPRINT.260819.01 — 2 ACs unmet without gap-ref` | `1` (Failure) |
| **Unregistered Gap Mention** | `G-NNN` referenced in code/docs but absent in `gaps.kdl` | `praxis check: unindexed gap reference G-055 in src/parser.rs:42` | `1` (Failure) |
| **Malformed KDL Syntax** | Syntax error in `.kdl` file | Prints file, line, column, and expected token diagnostic | `1` (Failure) |

---

### 3. Technical Architecture (Seams & Educated Theory — $Iteration_3$)

- **State Directory Layout (`.praxis/` extending `docs/product/`):**
  ```
  .praxis/
  ├── config.kdl              <-- Project profile (service, compiler, cli), quality gates
  ├── gaps.kdl                <-- Authoritative Gap Register (G-NNN)
  ├── ledger.kdl              <-- Historical Slice Ledger & Burndown
  ├── adr/
  │   └── ADR.<id>.kdl        <-- Structured Architectural Decisions & Invariants
  ├── initiatives/
  │   └── INIT.<name>.kdl     <-- Growth initiatives & thin-slice maps
  ├── sprints/
  │   └── SPRINT.<id>.kdl     <-- Ephemeral implementation bridges
  ├── capabilities/
  │   └── CAP.<name>.kdl      <-- Living capability declarations & seam contracts
  └── guides/
      └── GUIDE.<name>.kdl    <-- User guide outlines & verified capability mappings
  ```

- **Compiled Markdown Projection Pipeline:**
  ```
  ┌────────────────────────────────────────────────────────┐
  │         .praxis/ Live Agile State (KDL Data)           │
  │  (Initiatives, Sprints, Capabilities, ADRs, Gaps)      │
  └───────────────────────────┬────────────────────────────┘
                              │
                              ▼ (praxis render)
  ┌────────────────────────────────────────────────────────┐
  │        Clean Whole-Document Generation (Zero Regex)    │
  ├────────────────────────────┬───────────────────────────┤
  │ docs/product.md            │ Unified Product Dashboard │
  │ docs/capabilities/CAP.*.md │ Latest System Capabilities│
  │ docs/architecture/adr/*.md │ Published ADR Records     │
  │ docs/guides/*.md           │ Current-State User Guides │
  │ SPRINT.*.md                │ Sprint Review & Receipts  │
  └────────────────────────────┴───────────────────────────┘
  ```

- **Rust Engine Topology:**
  - **`praxis-core` (Functional Core):** Pure Rust domain types (`Sprint`, `Initiative`, `Capability`, `Gap`, `Adr`, `Guide`), KDL codecs (`knuffel`), DAG dependency analysis, invariant validators, and `minijinja` template projection engines.
  - **`praxis-cli` (Imperative Shell):** Subcommand dispatch (`check`, `render`, `sprint`, `gap`, `export`), subprocess runners, terminal diagnostics formatting (`miette`/`annotate-snippets`), and filesystem adapters.
- **Seam Contracts:** `cli-engine@v1`, `state-schema@v1` (shape defined in `CAP.plugin-conformance-and-validation-probes`).
- **Architectural Decision Record:** `ADR.260819.01: KDL State Architecture and Rust CLI Engine Consolidation` (**Status: Proposed**).

---

### 4. Quality & NFR Invariants

- **Performance SLA:** `praxis check` and `praxis render` complete in $< 10\text{ms}$ across repositories with 100+ state documents.
- **Zero Host Runtime Dependency:** Single precompiled standalone binary with zero external runtime dependencies on `jq`, `python3`, `bash`, or `ripgrep`. Available for macOS (arm64/x86_64), Linux (x86_64/aarch64), and Windows.
- **Test Layer Allocation:**
  - *Logic Layer:* Unit tests on domain constraints, gap rollups, and invariant rule evaluations.
  - *Composition Layer:* Golden-file test suites validating KDL decoding and complete Markdown whole-document template rendering.
  - *Integration Layer:* End-to-end CLI journey tests verifying `praxis check`, `praxis sprint close`, and `praxis render`.
