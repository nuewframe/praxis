# ADR.260819.01: KDL State Architecture and Rust CLI Engine Consolidation

- **Status:** Accepted — partially superseded by ADR.260819.02

> **Sealed legacy record, and a correction.** Supersession here is **partial**, not total.
> ADR.260819.02 **replaces** this decision's state model, render semantics, and whole-document
> synthesis, and **retains** its runtime, engine topology, KDL encoding, domain profiles, and
> hard-failing gates. The Rust engine and the `praxis-core` / `praxis-cli` split decided here are
> in force and govern every slice under the initiative.
>
> No longer a source: the decision lives in the delivery graph as
> [`praxis/adr/ADR.260819.01.kdl`](../../../praxis/adr/ADR.260819.01.kdl), whose `body` node
> carries this argument as KDL source. Pending archival.
- **Date:** 2026-08-19
- **Deciders:** Principal Engineer, Product Designer, Product Manager
- **Capabilities Touched:** `CAP.plugin-conformance-and-validation-probes`, `CAP.method-spine-and-execution`, `CAP.multi-harness-distribution`

---

## Context & Problem Statement

Praxis currently stores agile project state, sprint bridges, thin-slice ledgers, gap registers, capability contracts, and architectural decisions across semi-structured Markdown (`.md`) files with embedded YAML frontmatter, markdown tables, and checklist markers (`- [ ]`).

While Markdown is human-friendly for reading narratives, using Markdown as the **underlying state machine** creates significant failure modes in practice:

1. **Fragile In-Place Regex Mutations:** Tooling and AI agents mutate existing Markdown files by running regex search-and-replace on tables, checklists, and frontmatter. This frequently causes accidental syntax corruption, whitespace jitter, and dropped content.
2. **Multi-Document State Drift:** Status information duplicated across `docs/product.md`, `SPRINT.<ID>.md`, `SPRINT.<ID>.ledger.md`, and `INIT.<ID>.md` drifts silently without mechanical synchronization.
3. **Silent Scope-Dropping on Sprint Close:** In Markdown, an agent or developer can mark a sprint complete simply by removing unmet acceptance criteria checkboxes during sprint close without any tool objecting.
4. **Shell Script Portability and Performance Ceiling:** Praxis relies on ~15 separate bash scripts (`check-*.sh`) calling `jq`, `sed`, `grep`, and inline `python3`. This causes cross-platform disparities (macOS BSD vs. Linux GNU), spawns hundreds of slow subshells, and struggles to express complex domain invariants (such as capability DAG cycle detection or bidirectional gap rollups).
5. **Domain Mismatch in Quality Probes:** The standard `verify.sh` assumes a web/distributed service model (HTTP observability, externalized config URLs, stateless request paths), generating irrelevant warnings or no-ops on compilers (like SSF/Standoff), standalone CLI tools, and libraries.

---

## Research & Empirical Evaluation

### 1. The Standoff Case Study: JSON-Backed Mechanical State
An audit of the Standoff (SSF compiler) repository revealed the power of structured, machine-readable registries:
- **`.slice-ledger.json`:** Tracks per-slice outcomes, review head models, break ratios, criteria counts (`met`/`unmet`), and a closed vocabulary of failure shapes.
- **`.gap-register.json`:** Authoritative registry of known debt (`G-001`…`G-054`) enforced bidirectionally by `check-gap-register.sh` (fails closed if a gap is mentioned in prose without an ID, or if a resolved gap remains open).
- **Bidirectional Invariants:** Proved that load-bearing pointers (e.g. failing a close when criteria are unmet without a gap reference) eliminate the silent scope-dropping problem entirely.

### 2. Format Evaluation: JSON vs. YAML vs. CUE vs. KDL

| Format | Parsing Strictness & Determinism | Agent Generation Reliability | Multi-line Narrative Ergonomics | Native Comments & Slashdash |
| :--- | :--- | :--- | :--- | :--- |
| **JSON / JSONC** | **Highest** (No type coercion) | **Highest** (Zero indent errors) | Low (requires `\n` or array of strings) | Comments via JSONC |
| **YAML** | **Low** ("Norway Problem", `no`/`on` booleans) | **Medium** (Frequent whitespace/indent drift) | **High** (`\|` and `>` block scalars) | `#` comments |
| **CUE** | **Highest** (Unification lattice) | **Low** (High LLM hallucination rate on advanced types) | High | `//` comments |
| **KDL** | **High** (Node-and-attribute AST) | **High** (Clear node grammar, no indentation traps) | **Highest** (Raw strings `r#"..."#` without escapes) | `//`, `/* */`, and **slashdash `/-` node commenting** |

**Conclusion on Format:** **KDL (Keyboard Document Language)** offers the ideal balance: the strict AST parsing of JSON, the visual cleanliness of HCL, raw multi-line strings for Markdown prose, and first-class CSS-like node selectors (`sprint[tier="Major"] > acceptance-criteria`).

### 3. Execution Runtime: Shell Scripts vs. Compiled Rust CLI

Benchmarking demonstrated that moving from loose shell scripts to a single compiled **Rust CLI (`praxis`)** delivers:
- **Sub-10ms In-Memory Verification:** Loads 100+ state files, validates schemas, verifies DAG dependencies, and evaluates ratchets in RAM in $<5\text{ms}$.
- **Zero Host Runtime Dependencies:** Standalone binary without requiring `python3`, `jq`, `ripgrep`, or specific bash versions.
- **Type-Safe Domain Invariants:** Core business rules are enforced by compiler-grade Rust domain models (`knuffel`/`serde`).

---

## Architectural Decision

We adopt a **KDL-first state architecture governed by a compiled Rust CLI (`praxis`)**, establishing a clean separation between **operational data** and **rendered Markdown presentations**:

```
┌────────────────────────────────────────────────────────┐
│            1. Authoritative Operational State          │
│          (.praxis/ KDL Data — Extending docs/product/) │
│  • config.kdl     • gaps.kdl         • ledger.kdl      │
│  • adr/*.kdl      • initiatives/*.kdl • sprints/*.kdl  │
└───────────────────────────┬────────────────────────────┘
                            │
                            ▼ (praxis render)
┌────────────────────────────────────────────────────────┐
│     2. Fresh Whole-Document Projections (Zero Regex)   │
│  • docs/product.md              (Unified Dashboard)    │
│  • docs/capabilities/CAP.*.md   (Living Capabilities)  │
│  • docs/architecture/adr/*.md   (Published ADRs)       │
│  • docs/guides/*.md             (Current User Guides)  │
│  • SPRINT.*.md                  (Sprint Review & PR)   │
└────────────────────────────────────────────────────────┘
```

### Core Commitments

1. **State Lives in `.praxis/` as KDL Data:**
   All transactional agile state (sprints, thin slices, initiatives, capability declarations, ADRs, gaps, and guide outlines) is stored in `.praxis/*.kdl`. Markdown files are never treated as mutable state stores.
2. **Whole-Document Synthesis (Zero Regex Mutations):**
   `praxis render` compiles fresh, whole Markdown deliverables directly from KDL state using `minijinja` templates. Tooling never performs regex search-and-replace over existing Markdown documents.
3. **Rust Engine Topology (`praxis-core` / `praxis-cli`):**
   - **`praxis-core` (Pure Functional Core):** Domain types (`Sprint`, `Initiative`, `Capability`, `Gap`, `Adr`), KDL codecs, invariant validators, and projection templates.
   - **`praxis-cli` (Imperative Shell):** CLI subcommands (`check`, `render`, `sprint`, `gap`, `export`), process execution, and terminal diagnostics formatting (`miette`).
4. **Domain Profile Adaptability:**
   `.praxis/config.kdl` declares the repository profile (`profile="service"`, `profile="compiler"`, `profile="library"`). `praxis check` selectively activates relevant guardrails, eliminating service probe false positives on compilers and libraries.
5. **Hard-Failing Invariant Gates:**
   - A sprint marked `tier="Major"` without an `Accepted` ADR hard-fails verification.
   - `praxis sprint close` refuses to close if unmet acceptance criteria exist without an assigned `gap-ref`.
   - Any `G-NNN` referenced in code or prose that is absent from `gaps.kdl` hard-fails verification.

---

## Consequences

### Positive
- **100% Mathematical Dashboard Fidelity:** `docs/product.md` and roadmap tables are rendered directly from state with zero manual sync or formatting drift.
- **Blazing Fast Performance:** Full workspace verification and document rendering completes in $<10\text{ms}$.
- **Zero Host Dependency Friction:** A single prebuilt native binary runs identically on macOS, Linux, and Windows in CI and local developer workstations.
- **Human & AI Ergonomics:** KDL’s raw strings (`r#"..."#`), comments, and slashdash `/-` nodes provide superior authoring comfort compared to strict JSON while avoiding YAML indentation hazards.

### Negative & Mitigations
- **Binary Distribution Requirement:** Adopter repositories need access to the `praxis` binary.  
  *Mitigation:* Distribute prebuilt binaries via GitHub Releases with a 5-line bootstrapping wrapper in `scripts/verify.sh` or `cargo install praxis-cli`.
- **Migration of Existing Projects:** Existing repositories have legacy Markdown/JSON structures.  
  *Mitigation:* Provide `praxis migrate` to parse existing Markdown frontmatter and JSON configs into the canonical `.praxis/` KDL tree.

---

## Related Documents

- **Target Initiative:** [INIT.kdl-state-and-rust-cli-consolidation](../../product/initiatives/INIT.kdl-state-and-rust-cli-consolidation.md)
- **Product Dashboard & Context:** [docs/product.md](../../product.md)
- **Capability Records:**
  - [CAP.method-spine-and-execution](../../capabilities/CAP.method-spine-and-execution.md)
  - [CAP.plugin-conformance-and-validation-probes](../../capabilities/CAP.plugin-conformance-and-validation-probes.md)
  - [CAP.multi-harness-distribution](../../capabilities/CAP.multi-harness-distribution.md)
