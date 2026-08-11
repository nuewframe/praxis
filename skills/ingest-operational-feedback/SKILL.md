---
name: ingest-operational-feedback
description: >
  Downstream human-in-the-loop operational feedback skill. Processes incident post-mortems, operator friction logs,
  SLO review notes, and production failures to update living capability records (CAP.<name>.md) and trigger defect/enhancement initiatives.
user-invocable: true
disable-model-invocation: false
---

# Skill: Ingest Operational Feedback (`skills/ingest-operational-feedback/`)

Use this skill when processing production incident post-mortems, operator friction logs, SLO review findings, or real-world operational feedback.

**Audience:** Principal Engineer, Product Manager, Site Reliability Lead.  
**Purpose:** Feed real-world runtime behavior and human operator feedback back into the Praxis `LEARN` loop. Guarantees production incidents directly update capability invariants and seed future growth initiatives.

---

## What This Skill Produces

1. **Capability Quality Invariant Updates:** Updates to `docs/capabilities/CAP.<capability-name>.md#quality-and-nfr-invariants` (adjusting timeouts, retries, fallbacks, circuit breakers, or SLO targets).
2. **Defect / Resilience Initiatives:** New single-file initiatives (`docs/product/initiatives/INIT.<name>.md`) for recurring operational friction or architectural remediation.
3. **Durable Architecture Decisions (Optional):** Triggers `create-adr` when incident resolution requires superseding a prior architectural decision.

---

## Step 1 — Parse Operational Feedback Input

Accept structured operational input from one of four sources:
- **Incident Post-Mortem:** Production outage, severity level, root cause analysis (RCA), contributing factors.
- **Operator Friction Log:** Developer or operator difficulty deploying, configuring, or debugging a capability.
- **SLO / SLA Review:** Latency p95/p99 breaches, error budget burn rates, resource consumption spikes.
- **Customer Friction Report:** Real-world usability or workflow failure reported by users.

---

## Step 2 — Identify Touch Capabilities (`CAP.`)

Locate the living capability records responsible for the impacted domain:
- Open `docs/capabilities/CAP.<capability-name>.md`.
- Review existing 4 Production Readiness anchors (Observable, Configurable, Scalable, Resilient).
- Identify which invariant failed or was absent during the incident.

---

## Step 3 — Distill Learnings into Living Capability Records

Update the target `docs/capabilities/CAP.<capability-name>.md` file:

```markdown
### Production Readiness Anchors (Updated from Incident INC-2026-08)

#### 4. Resilient (Updated)
- **Timeout Defaults:** Reduced HTTP client timeout from 10000ms to 2500ms to prevent connection pool exhaustion during downstream degradation.
- **Circuit Breaker:** Introduced circuit breaker pattern opening after 5 consecutive 5xx failures with 30s reset timer.
- **Fallback Behavior:** Returns cached payload when downstream billing API is unavailable.
```

Append the incident summary to the capability's **History & Lineage** section:
```markdown
- **INC-2026-08-10 (Post-Mortem):** Downstream payment gateway timeout caused connection pool exhaustion. Resolved by reducing timeouts and adding circuit breaker fallback.
```

---

## Step 4 — Scaffold Remediation Initiative (`INIT.`)

If remediation requires multi-sprint engineering work beyond an immediate hotfix:
1. Invoke `create-initiative` to scaffold `docs/product/initiatives/INIT.<remediation-slug>.md`.
2. Populate $Iteration_1$ with thin-slices (`TS-001`: Add circuit breaker fallback, `TS-002`: Externalize connection pool configuration).
3. Register the initiative on [`docs/product.md`](../../docs/product.md).

---

## Step 5 — Trigger ADR Supersession (If Required)

If the incident reveals a fundamental flaw in a prior architectural decision:
1. Invoke `create-adr` to author `ADR.<YYMMDD>.<seq>.md`.
2. Explicitly mark the prior ADR as `Superseded by ADR.<YYMMDD>.<seq>`.

---

## Quality Checklist

- [ ] Operational feedback grounded in real incident log, post-mortem, or SLO review
- [ ] Impacted living capability record (`CAP.<name>.md`) updated directly
- [ ] 4 Production Readiness anchors updated with explicit quantitative metrics
- [ ] Remediation initiatives (`INIT.`) created for multi-sprint fixes
