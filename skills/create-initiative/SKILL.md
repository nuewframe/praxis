---
name: create-initiative
description: >
  Scaffold or refine a single-file growth initiative (INIT.<initiative-name>.md) in docs/product/initiatives/
  and register it on the product dashboard (docs/product.md). Supports progressive refinement lifecycle from Iteration 1 to N.
user-invocable: true
disable-model-invocation: false
---

# Skill: Create Initiative (`INIT.`)

Use this skill to initialize or refine a single-file growth initiative (`docs/product/initiatives/INIT.<initiative-name>.md`).

**Audience:** Product Manager, Product Designer, Principal Engineer. **Purpose:** Capture high-level intent ($Iteration_1$) and refine it progressively ($Iteration_N$) into thin-slices, UX specs, architecture theory, and NFR targets in one living document.

---

## What This Skill Produces

1. A single initiative file: `docs/product/initiatives/INIT.<initiative-name>.md`
2. An index entry in the active roadmap section of [`docs/product.md`](../../docs/product.md)

---

## Initiative File Structure (`INIT.<initiative-name>.md`)

```markdown
# INIT.<initiative-name>: [Initiative Title]

**Status:** ⚪ Proposed | 🔄 In Progress | ✅ Delivered  
**Capabilities Touched:** `CAP.<cap-1>`, `CAP.<cap-2>`  
**Target Horizon:** [e.g., Q3 Milestone / Release 1.2]  

---

## Executive Summary & Hypothesis ($Iteration_1$)

- **Business Intent & ROI:** [High-level business target and North Star metric]
- **Hypothesis:** We believe [doing X] for [user Y] will achieve [outcome Z].
- **Validation Method:** [Empirical test or operational feedback loop]

---

## User Value & Thin-Slices Roadmap ($Iteration_1 \rightarrow Iteration_2$)

### Thin-Slices Table

| Slice ID | Outcome & User Value | Status | SPRINT Link |
| -------- | -------------------- | ------ | ----------- |
| `TS-001` | [Description of first thin slice] | ⚪ | — |
| `TS-002` | [Description of second thin slice] | ⚪ | — |

---

## Progressive Refinement ($Iteration_2 \rightarrow Iteration_N$)

### User Experience (UX Deltas)

- **Primary Persona:** [Target User Persona]
- **User Journey:** Entry point → Trigger → Observable State → Completion
- **Acceptance Criteria (Given/When/Then):**
  - **Given** [context], **When** [user action], **Then** [system response].
- **Ambiguity & Error States:** [Handling empty data, permissions, network failures]

### Technical Architecture (Seams & Educated Theory)

- **Capabilities Touched:** `CAP.<capability-name>`
- **Seam Contracts:** `<contract-name>@vN` ([contracts/contract-<name>.json](file:///path/to/contract))
- **Topology & Ports/Adapters:**
  - Functional Core: pure business rules
  - Imperative Shell: I/O adapters, storage, external API clients

### Quality & NFR Invariants

- **Test Layer Allocation:** Logic / Composition / Seam Contract / E2E
- **NFR Anchors:**
  - *Observable:* Correlation IDs, metric output
  - *Configurable:* Zero hardcoded limits
  - *Scalable:* Throughput/latency targets (p95/p99)
  - *Resilient:* Timeouts, retries, fallback behavior
```

---

## Step 1 — Scaffold the Initiative File

Create `docs/product/initiatives/INIT.<initiative-name>.md` using the standard structure above. Populate $Iteration_1$ sections (Summary, Hypothesis, initial Thin-Slices).

---

## Step 2 — Register on Product Dashboard

Update the active roadmap section in [`docs/product.md`](../../docs/product.md) to index the initiative under active initiatives:

```markdown
- [INIT.<initiative-name>](product/initiatives/INIT.<initiative-name>.md) — [Title & one-sentence summary]
```

---

## Step 3 — Progressive Refinement ($Iteration_2 \rightarrow Iteration_N$)

As UX design, technical architecture, and quality specs are developed:
1. Invoke `create-product-design-spec` to refine UX Deltas.
2. Invoke `create-product-architecture-spec` to refine Seams & Educated Theory.
3. Invoke `create-quality-spec` to refine NFR Invariants.

---

## Quality Checklist

- [ ] File follows `INIT.<initiative-name>.md` naming prefix
- [ ] Registered on [`docs/product.md`](../../docs/product.md)
- [ ] Contains stable thin-slice IDs (`TS-NNN`)
- [ ] Lean on $Iteration_1$, refined progressively up to $Iteration_N$
