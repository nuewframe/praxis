---
name: create-quality-spec
description: >
  Author or refine quality, NFR, and production readiness specifications inside an initiative (INIT.<name>.md)
  or living capability record (CAP.<name>.md). Maps test layers by ownership and states quantitative NFR targets.
user-invocable: true
disable-model-invocation: false
---

# Skill: Create Quality Spec

Use this skill when authoring or refining quality specifications and NFR targets.

**Audience:** Product Designer, Principal Engineer, QA Lead. **Purpose:** Define quality targets, test-layer mapping, NFR thresholds, and observable DoD without code in the spec.

---

## What This Skill Produces

Refines the **Quality & NFR Invariants** section of an initiative file (`docs/product/initiatives/INIT.<initiative-name>.md`) or promotes validated quality invariants into a living capability record (`docs/capabilities/CAP.<capability-name>.md#quality-and-nfr-invariants`).

Quality specs state quantitative criteria in prose and tables. Implementation code lives strictly in test files.

---

## Step 1 — Map Test Pyramid by Ownership

Assign testing responsibilities strictly by layer:

| Test Layer | Target Scope | Ownership & Responsibility | Verification Mechanism |
| ---------- | ------------ | -------------------------- | ---------------------- |
| **1. Unit / Logic Tests** | Functional Core domain logic | Pure logic state transitions, invariants, calculations | Fast unit test suite (milliseconds) |
| **2. Adapter Contract Tests** | Port interfaces & Seam Contracts | Validates adapter implementations against frozen contracts (`<name>@vN`) | Mock-backed contract test suite |
| **3. Integration Tests** | Imperative Shell & Database | Real DB transactions, file system I/O, external API stubs | Containerized / local service tests |
| **4. Journey Tests (E2E)** | Full User Flows | Critical end-to-end user flows & UI interaction paths | Automated browser / E2E test suite |

---

## Step 2 — Specify Quantitative NFRs & 4 Production Anchors

Specify explicit NFR metrics across the 4 Production Readiness domains:

```markdown
### Production Readiness Anchors

#### 1. Observable
- **Structured Logging:** All log output formatted as JSON with mandatory fields (`timestamp`, `trace_id`, `span_id`, `capability`, `event`).
- **Metrics Exported:** Counter for requests, histogram for duration (`http_request_duration_seconds`).
- **Trace Propagation:** W3C Trace Context headers propagated across seam calls.

#### 2. Configurable
- **Externalized Settings:** 100% of environment settings loaded via environment variables or config files. Zero hardcoded secrets or endpoints.
- **Feature Flags:** Experimental capabilities wrapped in dynamic flags with safe default off.

#### 3. Scalable
- **Latency Targets:** p95 $< 150\text{ms}$, p99 $< 500\text{ms}$ under standard load.
- **Throughput Bounds:** Supports up to 500 req/sec per node with max 512MB RAM heap allocation.

#### 4. Resilient
- **Timeout Defaults:** External HTTP calls time out at 3000ms.
- **Retry Policy:** Exponential backoff with jitter (max 3 retries) on transient $5xx$ errors.
- **Fallback Behavior:** Graceful degraded response when downstream service is unavailable.
```

---

## Step 3 — Define Definition of Done (DoD) & Verification Criteria

Specify explicit DoD gates:
- All Given/When/Then acceptance criteria verified green.
- Seam contracts validated against `.seam-contracts.json`.
- Zero unhandled exceptions or swallowed errors.
- 4 Production Readiness anchors satisfied.

---

## Step 4 — Update the Target Document

1. **For Initiative NFR Targets:** Update `## Progressive Refinement -> Quality & NFR Invariants` in `docs/product/initiatives/INIT.<initiative-name>.md`.
2. **For Living Capability Quality Invariants:** Promote to `docs/capabilities/CAP.<capability-name>.md#quality-and-nfr-invariants` at sprint close.

---

## Quality Checklist

- [ ] Quality criteria specified without code snippets in the doc
- [ ] Test layers assigned strictly by ownership
- [ ] 4 Production Readiness anchors (Observable, Configurable, Scalable, Resilient) explicit
- [ ] Quantitative NFR metrics stated (latency, throughput, timeouts)
- [ ] Environment failure distinguished from test failure
