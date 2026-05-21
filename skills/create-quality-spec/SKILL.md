---
name: create-quality-spec
description: >
  Author or refine a wave's `qa.md` — the planning-stage quality specification. Defines risk
  tiers, test layer mapping, behavioral invariants, security coverage, explicit out-of-scope,
  and observable definition-of-done. This is a specification document, not test code. Pairs with
  `test-by-ownership` for the Pyramid Test Strategy.
user-invocable: true
disable-model-invocation: false
---

# Skill: Create Quality Spec

Use this skill when authoring or updating a wave's `qa.md`.

**Audience:** QA / principal engineer, product designer, product manager. **Purpose:** Define what tests must prove — the contract that the test suite executes against.

---

## What This Skill Produces

A wave-scoped quality specification — a planning artifact, **not implementation code**.

> ⚠️ **Never include code, imports, fixtures, file paths, or assertions in `qa.md`.** Code lives in test files. This document describes intent and risk.

A strong `qa.md` answers six questions:

1. **Where are the risks?** Assess failure modes by blast radius and likelihood — not all failures are equal; PII exposure is not the same as a UI glitch.
2. **What must each pyramid layer prove?** Map every behavior property to exactly one layer. Never duplicate across layers.
3. **What invariants must hold?** State behavioral contracts in precise plain language: given this input and this viewer, the response contains exactly these fields.
4. **What security scenarios must the suite prove are impossible?** Each threat must be traceable to a specific test.
5. **What is explicitly out of scope?** Owned coverage gaps with rationale are as important as covered cases.
6. **When is this wave fully tested?** A checklist of observable, verifiable outcomes — not "tests pass" but "email never appears in any response body."

---

## Required Sections

Every `qa.md` must include:

| Section                      | What it contains                                                                                 |
| ---------------------------- | ------------------------------------------------------------------------------------------------ |
| **Risk Analysis**            | Tiered table of failure modes: blast radius, likelihood, which test layer mitigates each         |
| **Pyramid Test Map**         | Every behavior property → layer → file. No property appears in two layers.                       |
| **Journey Specifications**   | Step-by-step behavioral intent per E2E scenario — start state, user actions, observable outcomes |
| **Composition Test Intent**  | Per-endpoint table of invariants: input conditions → response contract                           |
| **Logic Test Intent**        | Per-function equivalence classes: input category → expected output or error                      |
| **Security Coverage Matrix** | Threat → vector → test that proves it cannot happen                                              |
| **Coverage Gaps**            | Explicitly excluded scenarios with rationale                                                     |
| **Definition of Done**       | Checklist items written as observable facts, not "tests pass"                                    |

---

## Step 1 — Risk Analysis

Tier failure modes honestly. A test suite is prioritized by blast radius.

| Risk           | Blast Radius                   | Likelihood          | Layer that mitigates                                           |
| -------------- | ------------------------------ | ------------------- | -------------------------------------------------------------- |
| [Failure mode] | Critical / High / Medium / Low | High / Medium / Low | Logic / Composition / Adapter Contract / Integration / Journey |

Avoid marking everything "critical" — triage honestly.

---

## Step 2 — Pyramid Test Map (one property of a behavior, one layer)

| Behavior              | Layer       | Test file (intent only — no code) |
| --------------------- | ----------- | --------------------------------- |
| [Observable behavior] | Journey     | [test target]                     |
| [Endpoint contract]   | Composition | [test target]                     |
| [Pure function rule]  | Logic       | [test target]                     |

If a property appears in two rows, collapse to the lowest pyramid layer that proves it. Never duplicate.

See `test-by-ownership` for the Pyramid Test Strategy layer definitions.

---

## Step 3 — Journey Specifications

For each E2E scenario:

- **Start state:** [Pre-conditions in user terms]
- **User actions:** [Step-by-step user-visible actions]
- **Observable outcomes:** [What the user sees / can do next]
- **Failure / recovery branches:** [What happens when the action fails]

---

## Step 4 — Composition Test Intent

Per endpoint, an invariant table:

| Input condition                    | Expected response contract          |
| ---------------------------------- | ----------------------------------- |
| Valid request as authorized viewer | [Exact fields present, status code] |
| Missing required field             | [Status code, error shape]          |
| Authenticated but unauthorized     | [Status code, what is hidden]       |
| Resource not found                 | [Status code, response shape]       |
| Conflict / business-rule violation | [Status code, error shape]          |

Cover boundary, negative, and authorization cases. Happy-path-only is incomplete.

---

## Step 5 — Logic Test Intent

Per pure function, equivalence classes:

| Input category  | Expected output or error |
| --------------- | ------------------------ |
| [Class 1]       | [Output]                 |
| [Edge case]     | [Output or error code]   |
| [Invalid input] | [Error code]             |

---

## Step 6 — Security Coverage Matrix

Each threat traceable to a specific test:

| Threat                        | Vector                             | Test that proves it cannot happen |
| ----------------------------- | ---------------------------------- | --------------------------------- |
| [PII leak via response shape] | [Field exposed to wrong viewer]    | [Composition test intent]         |
| [Privilege escalation]        | [Endpoint accessible without role] | [Composition test intent]         |
| [Injection / unsafe input]    | [Field passed to query / shell]    | [Logic + composition test intent] |

---

## Step 7 — Coverage Gaps

Explicitly excluded scenarios with rationale. A spec without gaps is hiding gaps.

| Excluded scenario                      | Rationale           | Future trigger to revisit |
| -------------------------------------- | ------------------- | ------------------------- |
| [Performance under N concurrent users] | [Out of wave scope] | [When wave Y starts]      |

---

## Step 8 — Definition of Done

Observable facts only. Examples:

- [ ] Email field never appears in any response body for non-self viewers
- [ ] Anonymous request returns 401 from every protected endpoint
- [ ] Journey: user can complete primary flow on iOS Safari + Android Chrome
- [ ] All composition tests for new endpoints pass
- [ ] No new `sleep()` / `waitForTimeout()` calls
- [ ] No skipped tests

Avoid: "all tests pass" (vague), "performance is acceptable" (unmeasurable).

---

## Quality Checklist

- [ ] All six required sections present
- [ ] Risk tiers triaged honestly (not everything "critical")
- [ ] Every behavior mapped to exactly one test layer
- [ ] Each invariant table covers boundary, negative, and auth cases
- [ ] Security threats are traceable to specific tests
- [ ] Coverage gaps explicit with rationale
- [ ] Definition of done is written as observable facts

---

## Anti-Patterns

- ❌ Any code block — code lives in test files
- ❌ Import statements, fixture calls, assertion syntax, file paths
- ❌ Happy-path-only specs
- ❌ Vague intent — "test the visibility logic" is not a specification
- ❌ Restating the architecture — QA reasons about risk and behavioral confidence
- ❌ Marking every risk "critical" — triage honestly
- ❌ Omitting coverage gaps section ("we cover everything")
