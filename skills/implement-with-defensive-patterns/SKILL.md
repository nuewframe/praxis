---
name: implement-with-defensive-patterns
mode: implementer
tools: [read_file, file_search, grep_search, semantic_search, create_file, replace_string_in_file, run_in_terminal]
description: "Phase 5 of the principal-engineer workflow (implementer mode). Use after the capability layout and ADR are approved (mechanical Design Approval = ADR status Accepted + signed Design Approval line in active sprint file) to write production-ready code with composition over inheritance, shift-left security, and structured telemetry. Implementer mode cannot modify approved ADRs, design specs, or wave docs — if a design flaw is discovered, bounce back to architect mode."
---

# Implement with Defensive Patterns

## Use this when

- Phase 4 (`design-capability-layout`) is complete and approved.
- About to write or modify implementation code.

## Do NOT use this when

- The layout or ADR has not been reviewed. Return to Phase 4.
- Tests have not been written first for the change. Write the failing test first (TDD — see Phase 6).

## The implementation order (TDD)

```
1. Write a failing test for the next behavior          (RED)
2. Write the minimum code to make it pass              (GREEN)
3. Refactor while keeping tests green                  (REFACTOR)
4. Repeat
```

Never write production code without a failing test that demands it. If you wrote code first, delete it and start from the test.

## Steps

### Step 1 — Functional core first

Implement the pure business logic before any I/O. The core takes plain data in, returns plain data out, and has no side effects.

```
// Core — pure, trivially testable
function calculateTax(order: Order, region: Region): Money { ... }

// Shell — talks to the world
async function placeOrder(req: Request): Response {
  const order = parseOrder(req)
  const region = await regionRepo.findByZip(order.zip)
  const tax = calculateTax(order, region)        // <-- core
  await orderRepo.save({ ...order, tax })
  return ok({ orderId: order.id, tax })
}
```

### Step 2 — Ports and adapters

Every external resource access — database, queue, third-party API, clock, filesystem — goes through a **Port interface** declared by the capability. The capability owns the interface; the implementation is an **adapter**.

For every Port there must be **two** adapter implementations:

- **In-memory adapter** — real code (not a stub), used at the Composition test layer for fast TDD feedback. Lives next to the capability (e.g., `<capability>.adapter-memory.<ext>`) and is test-only — never imported by production code.
- **Real-backend adapter** — production implementation (e.g., `<capability>.repository.<ext>` for a relational- or document-DB-backed `Repository` Port).

Both adapters import and pass a **shared contract test suite** (`<capability>.<port>.contract.test.<ext>`). The Port/Adapter parity gate fails CI if a Port has only one of the two.

```
// Port — declared by the capability (core)
export interface OrderRepository {
  save(o: Order): Promise<Order>
  findByIdempotencyKey(k: string): Promise<Order | null>
}

// Shared contract suite — both adapters must pass it
export function orderRepositoryContract(make: () => OrderRepository) { ... }

// In-memory adapter (test-only)
export function createInMemoryOrderRepository(): OrderRepository { ... }

// Real adapter (production)
export function createPostgresOrderRepository(db: Pool): OrderRepository { ... }
```

See the `test-by-ownership` skill for the test layer mapping (Composition vs. Adapter Contract vs. Integration boundary).

### Step 3 — Composition over inheritance

- Compose behavior by passing dependencies in (constructor injection or function arguments). Dependencies are always Port types, never concrete adapter classes.
- Avoid class hierarchies more than one level deep.
- Prefer plain functions and data over classes when the language allows.

```
// Good — explicit Port dependencies, easy to test with the in-memory adapter
function createOrderService(deps: {
  orderRepo: OrderRepository      // Port, not Postgres class
  billingPort: BillingPort
  clock: () => Date
}) {
  return {
    placeOrder: (input) => { ... },
  }
}

// Bad — implicit dependencies, hard to test
class OrderService extends BaseService {
  placeOrder(input) {
    Database.query(...)        // global
    new Date()                 // hidden clock
    BillingService.charge(...) // static call
  }
}
```

### Step 4 — Shift-left security

For every input crossing a trust boundary:

| Concern                                       | Required action                                                   |
| --------------------------------------------- | ----------------------------------------------------------------- |
| Type validation                               | Validate with a schema library (Zod, Pydantic, JSON Schema, etc.) |
| String injection (SQL, shell, HTML, template) | Use parameterized queries / safe templating only                  |
| Authentication                                | Verify identity at the edge of the capability                     |
| Authorization                                 | Check permission per operation, not per session                   |
| Rate limiting                                 | Apply per principal and per route                                 |
| Data minimization                             | Strip secrets, tokens, and PII before logging                     |
| CORS / CSRF                                   | Default deny; allow-list explicitly                               |

For every output crossing a trust boundary:

- Never include sensitive fields in error messages.
- Use a uniform error envelope (status, error code, human message, correlation ID).
- Sanitize anything echoed back to the client.

### Step 5 — Structured telemetry

Every external boundary call produces:

```
log.info('placing order', { orderId, userId, correlationId })
metrics.histogram('order.place.latency_ms', durationMs, { region })
metrics.counter('order.place.count', 1, { result: 'success' })
trace.span('order.place', { orderId }, async () => { ... })
```

Required everywhere:

- **Correlation / trace ID** propagated from inbound request to all outbound calls.
- **Latency histogram** for p50/p95/p99.
- **Error counter** with reason label.
- **Structured log** (JSON), never `console.log` / `print` for production paths.

Forbidden:

- Logging full request/response bodies.
- Logging secrets, tokens, passwords, full PII records.
- String-concatenated log messages without structured fields.

### Step 6 — Defensive external calls

Every external call (DB, HTTP, queue, file) must declare:

```
const result = await withTimeout(
  withRetry(
    () => externalApi.call(input),
    { attempts: 3, baseMs: 100, maxMs: 5000, jitter: true }
  ),
  { ms: 2000 }
)
```

- **Timeout** — never unbounded.
- **Retry** — only for idempotent operations; with jitter; with a cap.
- **Circuit breaker** — for high-traffic critical paths.
- **Fallback** — degraded behavior when the call fails.

### Step 7 — Idempotency for mutating operations

If the operation mutates state and may be retried:

- Require an `Idempotency-Key` header (or equivalent).
- Store the key plus the response for a deduplication window.
- Return the stored response on duplicate keys.

### Step 8 — Comment discipline

Default to no comments. Code is the source of truth; comments drift, code doesn't.

Write a comment **only** when one of these is true:

- The _why_ isn't obvious from the code (a non-obvious trade-off, a workaround for an external constraint, a deliberate non-idiomatic choice).
- A reader would otherwise have to re-derive the same reasoning the author already did (a tricky invariant, a load-bearing assumption, coupling to something offscreen).
- A public API contract needs documenting (kdoc / jsdoc / docstring on exported surfaces).

Do **not** write:

- Comments that restate the next line of code.
- Banners, dividers, and decorative section headers inside code files.
- TODOs with no owner and no condition for removal.
- Author / date / changelog comments — that's what git is for.
- Commented-out code — delete it; git keeps it.

If a comment is needed to make a piece of code understandable, first ask whether renaming a variable, extracting a function, or restructuring the logic would remove the need. A comment is the second-best fix.

```
// Bad — restates the code
// Increment counter
counter += 1

// Bad — narrating intent that the function name already states
function calculateTax(...) {
  // Calculate the tax
  ...
}

// Good — explains a non-obvious external constraint
// The store rejects `optional<object>` for arbitrary JSON; the schema requires
// a flexible field — see vendor docs.
metadata FLEXIBLE TYPE object,

// Good — flags a load-bearing assumption that a reader would otherwise miss
// `host_raw` arrives as either an opaque record token or a plain string
// depending on driver version. Always normalize before comparison.
const hostId = String(hostRaw).replace(/^user:/, '')
```

### Step 9 — Design-token adherence (single source rule)

A UI surface is not done until every color, spacing, radius, typography, elevation, and motion value reads from a **single named token source** — the project's design system / theme module — rather than being inlined. Tokens are the single point of control: change the brand purple once and every surface updates. Inlined raw values defeat the system and silently rot the next time the palette shifts.

Universal rule:

- **Forbidden:** raw hex colors, off-scale numeric dimensions, framework-default theme reads (e.g. `MaterialTheme.colorScheme.*`, default Tailwind palette classes when a project palette exists, hard-coded `rgb()`/`rgba()`).
- **Required:** every visual value resolves to a project-defined token name (e.g. `Theme.colors.primary`, `var(--color-primary)`, `tokens.spacing.s5`).
- **Missing token?** Extend the token source. Never inline.
- **Allowed exceptions** — must carry a one-line _why_ comment per Step 8:
  1. Third-party brand colors dictated by external branding guidelines.
  2. Intentionally off-scale dimensions explicitly called out by design.

Project-specific guidance — exact token module paths, theme-object names, per-stack rules, and self-review greps — lives in a project overlay file (typically `.github/instructions/design-tokens.instructions.md`) generated by `provision-project-overlay`. The plugin intentionally stays stack-agnostic; the overlay is where stack details belong.

Self-review checklist for the diff:

- [ ] No raw hex (`0x[0-9A-Fa-f]{6,8}` / `#[0-9A-Fa-f]{3,8}`) outside the token source.
- [ ] No off-scale numeric dimensions in screen code.
- [ ] No framework-default theme reads where a project token exists.
- [ ] Any exception carries a single-line why-comment.

### Step 10 — Self-review before handing off

Before declaring the implementation done, re-read the diff and check:

- [ ] No business logic in the API or repository file.
- [ ] No `utils/`, `helpers/`, `common/`, `shared/`, `misc.*` introduced.
- [ ] No `console.log` / `print` for production paths.
- [ ] No unbounded timeouts.
- [ ] No SQL string concatenation.
- [ ] No globals; all dependencies injected as Port types.
- [ ] No classes deeper than one level of inheritance.
- [ ] Every Port has both an in-memory and a real-backend adapter, both passing the shared contract suite.
- [ ] Every function has a test that fails when the function is broken.
- [ ] Public surface (`mod.*`) updated only with intentional additions. In-memory adapters are **not** exported.
- [ ] Correlation ID propagated through every boundary call.
- [ ] Every comment in the diff either explains a non-obvious _why_ or documents a public API. No restating-the-code comments, no decorative banners, no commented-out code.
- [ ] No raw hex, no off-scale dimensions, no framework-default theme reads in screen code — every visual value reads from the project's design-token source (or carries a one-line why for the rare exception). See the project's `design-tokens.instructions.md` overlay for the exact token module paths.

If any check fails, fix before moving to Phase 6.

## Anti-patterns

- "I'll add the test after." No.
- "It's just a small helper." That's how `utils.ts` starts. Name the capability.
- "I'll handle the error later." No — declare timeout, retry, fallback now.
- "Let's use inheritance for the shared behavior." Almost never. Use composition.
- Catching exceptions with no logging and no rethrow — silent failure is the worst failure.
- `try { } catch (e) { return null }` — name the failure mode and propagate or degrade explicitly.
- Comments that narrate what the next line does. If the code needs that, rename or extract until it doesn't.
- "I'll wire the colors to tokens later." No — token adherence is part of "done", not a follow-up. The next person reading the diff has no idea the value is provisional.
