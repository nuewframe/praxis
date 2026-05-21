---
name: test-by-ownership
description: >
  Universal Pyramid Test Strategy organized by ownership: Logic at the base, then Composition,
  Adapter Contract, Integration Boundary, and Journey at the tip. Enforces the cardinal rule
  "one property of a behavior, at one layer," Port/Adapter parity, Consumer-Driven Contracts for
  external dependencies, and the "Environment Blocked ≠ Test Failed" reporting protocol. Pairs
  with `create-quality-spec` (planning side) and `verify-and-assemble-pr` (execution side).
user-invocable: true
disable-model-invocation: false
---

# Skill: Pyramid Test Strategy

Use this skill when deciding **which** layer to test a behavior at, or when explaining the Pyramid Test Strategy to a contributor.

> The goal of tests is to **ensure correctness** and **maintain quality**. Slow tests get skipped — and skipped tests do neither. The pyramid shape keeps the broad base fast and deterministic while reserving the tip for a small number of high-value full journeys.

---

## Principle: ownership is the boundary

Test layer is determined by **what you own**, not by what runs in the same process.

| You own                                         | Examples                                                       | Test against                                                                            | Substitution                         |
| ----------------------------------------------- | -------------------------------------------------------------- | --------------------------------------------------------------------------------------- | ------------------------------------ |
| Pure code                                       | Your business math                                             | Real code                                                                               | None                                 |
| Internal Ports                                  | Your `OrderRepository` interface                               | Real **in-memory adapter** at Composition; real **backend adapter** at Adapter Contract | In-memory implementation of the Port |
| Internal backends (your DB schema, your queues) | Postgres, Kafka, Redis used by **your** capability             | Real test container                                                                     | None                                 |
| External services                               | Stripe, partner APIs, other teams' services                    | Contract-tested fake or sandbox                                                         | Contract-tested fake                 |
| Cross-system flows                              | Your system + external systems together in the Integration Env | Real systems                                                                            | None                                 |

This is the line between Composition (in-memory adapter) and Adapter Contract (real backend) and Integration boundary (contract-tested fake). Get this right and the matrix below is non-overlapping.

---

## Pyramid Layers

The pyramid gets narrower as scope, runtime cost, and environment complexity increase. Keep many tests at the base, enough tests in the middle to prove boundaries, and only a few high-value Journey tests at the tip.

```text
          Journey
        Full user experience / E2E
        few, high-value

         Integration Boundary
       External dependency behavior

        Adapter Contract
    Real adapter parity and backend contract

           Composition
    Service/API wiring with in-memory Port adapters

           Logic
    Pure rules, validation, transformations
      many, fast, deterministic
```

| Layer                    | Pyramid position | Property proven                                                                                                             | What's real                                                                                                              | What's substituted                                                                        | Guidance                                           |
| ------------------------ | ---------------- | --------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------- | -------------------------------------------------- |
| **Logic**                | Base             | Business math, validation, transformation under normal / boundary / failure inputs                                          | Pure functions only                                                                                                      | n/a                                                                                       | Many tests; fastest and most deterministic         |
| **Composition**          | Lower-middle     | Components wire correctly; orchestration order is right; error envelopes propagate                                          | Service + API + **in-memory adapter** for each Port                                                                      | The adapter under test (in-memory implementation of the same Port interface)              | Broad service/API confidence without real backends |
| **Adapter Contract**     | Middle           | The in-memory adapter does not lie. SQL, indexes, transactions, ordering guarantees work as claimed                         | Each adapter against its real backend (test Postgres, test Kafka). **Same test suite** runs against both implementations | Adapter under test is real                                                                | Enough coverage to prove every touched Port        |
| **Integration boundary** | Upper-middle     | Your boundary code degrades correctly when an external dependency misbehaves (timeout, retry, circuit breaker, idempotency) | Your wrapper around the external call                                                                                    | The external dependency itself — replaced by a **contract-tested fake** or vendor sandbox | Focused tests for external dependency behavior     |
| **Journey**              | Tip              | A user goal is met across the distributed landscape                                                                         | Full system in Integration Env                                                                                           | Nothing                                                                                   | Few tests; full journey or user experience scope   |

---

## Cardinal rule — one property of a behavior, at one layer

> The same behavior is naturally exercised at multiple layers. The discipline is to assert a **different property** at each layer.

Example — `placeOrder`:

| Layer                | Property asserted                                                                                           |
| -------------------- | ----------------------------------------------------------------------------------------------------------- |
| Logic                | `calculateTax` math is right                                                                                |
| Composition          | Service calls repo, then inventory, then event bus, in that order; correlation ID propagates                |
| Adapter Contract     | The repo's SQL round-trips through Postgres with the right transactional semantics                          |
| Integration boundary | When inventory returns 5xx, the call times out at 500ms, retries twice, and opens the circuit at 50% errors |
| Journey              | A real `POST /orders` returns 201 and `billing` consumes `OrderPlaced` end-to-end                           |

If you find yourself asserting the **same property** at two layers, delete the higher-layer assertion. Inner layers are faster and more deterministic.

---

## Layer-by-layer discipline

### Logic

- No I/O of any kind — no DB, no HTTP, no filesystem, no time without injection.
- Cover equivalence classes: typical input, boundary, edge, invalid.
- Each test should fail for exactly one reason.
- **Forbidden:** mocks. If you need a mock to test a pure function, the function is not pure — extract the I/O.

### Composition

- Service, API, and **in-memory adapters** wired together with real DI.
- In-memory adapters implement the same Port interface as the real adapters. They are real code, not stubs that return canned values.
- Required coverage per endpoint:
  - Happy path with valid input
  - Missing required field (4xx)
  - Invalid field format (4xx)
  - Unauthenticated (401)
  - Authenticated but unauthorized (403)
  - Resource not found (404)
  - Conflict / duplicate (409 or domain-specific)
  - Business-rule violation relevant to the feature
- **Forbidden:** asserting business math here (that's Logic) or asserting SQL behavior (that's Adapter Contract).
- **Forbidden:** stub objects returning canned values. Use real in-memory implementations of Ports.

### Adapter Contract

- One **shared contract suite** per Port lives in `<capability>.<port>.contract.test.*`. Both adapters (in-memory + real) import and run the same suite.
- The real-backend run uses TestContainers or the equivalent ephemeral instance.
- Required coverage per adapter:
  - Round-trip identity (write then read returns equal)
  - Constraint enforcement (unique, FK, not-null, ordering)
  - Transactional semantics (commit on success, rollback on failure)
  - Concurrent-write outcomes (last-write-wins, optimistic-lock, whatever the Port promises)
- **CI parity gate:** every Port must have ≥1 in-memory adapter and ≥1 real-backend adapter, both running the contract suite. CI fails if a Port has only one adapter.

### Integration boundary

- Test your **wrapper** code around an external call — timeout, retry policy, circuit breaker, idempotency replay.
- Substitute the external dependency with a **contract-tested fake** (e.g., Pact, WireMock with recorded interactions, Stripe's `stripe-mock`) or the vendor's sandbox.
- A "contract-tested fake" is one whose responses have been **verified against the real external service on a schedule**. Plain hand-rolled stubs are forbidden — they rot silently.
- Required coverage per external dependency:
  - Happy path
  - Timeout exceeded
  - 5xx response triggers retry
  - Retries exhausted → caller sees the expected error envelope
  - Circuit opens after threshold; subsequent calls fail fast
  - Duplicate request with same idempotency key returns stored response

### Journey

- Real user interactions only — click, type, navigate (UI), or full request cycle (API).
- No direct DB seeding to set up state — go through the real public surface.
- No implementation assertions (component class names, internal state, SQL row counts).
- Always use event-driven waits (`await expect(locator).toBeVisible()`) — never `sleep()` / `waitForTimeout()`.
- One scenario per test — happy path, error recovery, permission boundary.
- **Environment Blocked ≠ Test Failed.** See the protocol below.

---

## Environment Blocked vs Test Failed

Journey tests run against an Integration Env that may include systems you don't own. A test that cannot reach a third-party sandbox is **not** a regression in your code.

| Outcome                                                                          | Report as | Action                                         |
| -------------------------------------------------------------------------------- | --------- | ---------------------------------------------- |
| Assertion failed                                                                 | `FAIL`    | Investigate as a regression                    |
| Could not reach external dependency (DNS, 503, timeout before any business call) | `BLOCKED` | Notify the env owner; do not block the PR      |
| External dependency returned an error that the system handled correctly          | `PASS`    | The Integration boundary layer earned its keep |

**TTL rule:** a test that has been `BLOCKED` for >5 runs over 7 days is escalated. >10 runs over 14 days, it is auto-archived with a follow-up issue. This prevents `BLOCKED` from becoming a way to hide flaky regressions.

---

## TDD red-green-refactor at each layer

```
🔴 RED      → Write a failing test at the right layer
🟢 GREEN    → Write minimum code to make it pass
🔵 REFACTOR → Improve quality while keeping green
(repeat)
```

**Plan-first reminder:** before implementation, draft the test layer mapping (Logic / Composition / Adapter Contract / Integration / Journey) so each property has one home. This is what `create-quality-spec` produces at the wave level.

---

## Test naming

Every test name:

- Starts with `should` (or the language equivalent).
- Describes the **observable outcome**, not the implementation.
- Is specific enough to understand without reading the test body.

```
✅ should reject invitation to event the user was not invited to
✅ should return empty list when user has no circles
✅ should preserve response history when user changes answer

❌ test invitation rejection
❌ circles array
❌ response history test
```

---

## Quality gates

- [ ] All Journey scenarios for primary flows `PASS` or `BLOCKED` with valid reason.
- [ ] All Composition tests pass for changed endpoints.
- [ ] All Adapter Contract tests pass for changed Ports — both in-memory and real backends.
- [ ] All Logic tests pass for changed pure functions.
- [ ] All Integration boundary tests pass for changed external calls.
- [ ] Port/Adapter parity gate green: every Port has ≥1 in-memory and ≥1 real-backend adapter wired into the contract suite.
- [ ] No skipped tests (`.skip`, `xit`, etc.).
- [ ] No `sleep()` / `waitForTimeout()` introduced.
- [ ] No property duplicated across layers.
- [ ] No hand-rolled mocks for external services without a contract-test backing.

---

## Anti-patterns

- ❌ Using "Integration test" for internal-class wiring. That is **Composition**. Reserve "Integration" for tests touching resources outside your repository's control.
- ❌ Stub objects returning canned values at the Composition layer (e.g., `{ findById: jest.fn().mockReturnValue(...) }`). Use a real **in-memory adapter** of the same Port instead.
- ❌ Mocking the real DB inside the Adapter Contract layer. The whole point of that layer is to exercise the real backend.
- ❌ Adding only an in-memory adapter without a real one (or vice versa). Both must exist; the parity gate enforces this.
- ❌ Hand-rolled stubs of third-party APIs without a contract test verifying them on a schedule. Stubs rot.
- ❌ `BLOCKED` used as a permanent state to hide a real failure. Enforce the TTL rule.
- ❌ Asserting business math in Composition (that's Logic), or asserting SQL semantics in Composition (that's Adapter Contract).
- ❌ Co-locating Journey and Composition assertions in the same test.
- ❌ Writing tests after implementation ("we'll add tests next sprint").
- ❌ `sleep()` / `waitForTimeout()` to "wait for state" instead of event-driven waits.

---

## Refactor after green

Once tests pass at the chosen layer, refactor production and test code together:

- Names should speak the ubiquitous language.
- Each test fails for one reason and runs in any order.
- Duplicated arrange blocks belong in fixtures.
- `describe` / `it` names read like specification chapters.
- Shared contract suites stay in **one** file imported by both adapter tests — there is one source of truth.

Rerun the touched tests after refactor — green must stay green.
