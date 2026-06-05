---
name: verify-and-assemble-pr
mode: reviewer
tools: [read_file, file_search, grep_search, semantic_search, run_in_terminal]
description: Phase 6 of the principal-engineer workflow (reviewer mode). Use after implementation to verify behavior with the project's `verify` entry point, evaluate the refactor decision matrix, and assemble a structured PR narrative including rollback plan. Reviewer mode is read-only on source code — it files structured change requests; the implementer applies them. Approval requires the captured `verify` output, never a bare checkbox.
---

# Verify and Assemble PR

## Use this when

- Phase 5 (`implement-with-defensive-patterns`) is complete.
- Code is ready for human review.

## Do NOT use this when

- Tests are not yet written. Return to Phase 5 — TDD means tests came first.
- The change has not been self-reviewed against the Phase 5 checklist.

## The principle

```
Evidence before assertions. Run the verification, capture the output, then claim done.
```

Never report "tests pass" without having just run them. Never report "lint clean" without having just run the linter.

## Steps

### Step 1 — Pyramid Test Strategy (test-by-ownership)

Place each test at the right pyramid layer. The same behavior may be exercised at multiple layers, but each layer must assert a **different property**. The pyramid should be broad at Logic and Composition, narrower through boundary coverage, and narrowest at Journey.

| Pyramid position | Layer                    | Property proven                                                                                  | What's real                                                                                 | What's substituted                                                   | Guidance                                      |
| ---------------- | ------------------------ | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | --------------------------------------------- |
| Base             | **Logic**                | Pure business math under normal / boundary / failure inputs                                      | Pure functions only                                                                         | n/a                                                                  | Many tests; fastest and most deterministic    |
| Lower-middle     | **Composition**          | Service + API wire correctly; orchestration order; error envelopes propagate                     | Service + API + **in-memory adapter** for every Port                                        | The adapter under test (in-memory implementation of the Port)        | Broad service/API wiring coverage             |
| Middle           | **Adapter Contract**     | The in-memory adapter does not lie. SQL, indexes, transactions behave as claimed                 | Each adapter run against its real backend. **Same suite** runs against both implementations | Adapter under test is real                                           | Enough coverage to prove every touched Port   |
| Upper-middle     | **Integration boundary** | Wrapper around an external call degrades correctly: timeout, retry, circuit breaker, idempotency | Your boundary code                                                                          | The external dependency → **contract-tested fake** or vendor sandbox | Focused external dependency behavior coverage |
| Tip              | **Journey (E2E)**        | A real user goal is met across the distributed landscape                                         | Full system in Integration Env                                                              | Nothing                                                              | Few tests; full user journey scope            |

Rules:

- Tests live **alongside** the capability they verify (`<capability>.test.*`), not in a separate `tests/` tree.
- The functional core is tested without mocking — pure functions don't need mocks.
- Composition uses real **in-memory Port adapters**, not hand-rolled stubs returning canned values.
- Adapter Contract suites are **shared** between the in-memory and real-backend runs — one file, two adapters. Both must pass.
- Mock only what you do not own, and only with a contract-tested fake re-validated against the real external on a schedule.
- **Port/Adapter parity gate:** every Port must have ≥1 in-memory adapter and ≥1 real-backend adapter, both green against the shared contract suite. CI fails otherwise.
- See the `test-by-ownership` skill for the full Pyramid Test Strategy.

### Step 2 — Coverage that matters

For each new behavior, write tests for:

- **Happy path** — input is well-formed and dependencies behave.
- **Boundary** — empty, max-size, edge value (zero, negative, unicode, very long, very small).
- **Validation failure** — malformed input is rejected with a clear error.
- **Authorization failure** — wrong principal is rejected.
- **Dependency timeout** — external call exceeds its budget.
- **Dependency error** — external call returns a failure.
- **Retry behavior** — retried calls succeed on subsequent attempts.
- **Idempotency** — same key returns the stored response.
- **Concurrency** — two simultaneous identical operations behave correctly.

Coverage percentage is a lagging indicator. The above scenarios are the leading indicator.

### Step 3 — Run the verification

The project exposes a **single `verify` entry point** (e.g., `make verify`, `scripts/verify.sh`, `npm run verify`, `deno task verify`) generated by `bootstrap-project`. Reviewer mode runs that one command and pastes the captured output into the PR. Do not run individual tools ad hoc — the entry point sequences them deterministically.

The entry point must run, in order, and abort on first failure:

1. **Format check** — fail if formatting is off.
2. **Lint** — fail on warnings as well as errors.
3. **Type check** — full strict-mode compile.
4. **Anti-dumping linter** — `scripts/check-anti-dumping.sh` (this plugin).
5. **No-skipped-tests linter** — `scripts/check-no-skipped-tests.sh` (this plugin).
6. **No-sleep-waits linter** — `scripts/check-no-sleep-waits.sh` (this plugin).
7. **Port/Adapter parity gate** — `scripts/check-port-adapter-parity.sh` (this plugin).
8. **Seam-contract parity gate** — `scripts/check-seam-contract-parity.sh` (this plugin). Every seam declared in `.seam-contracts.json` has a machine-readable Shape and a shared Behavior suite on disk.
9. **Config-externalization probe** — `scripts/check-config-externalized.sh` (this plugin). Production-readiness: Configurable anchor.
10. **Observability-at-seams probe** — `scripts/check-observability-at-seams.sh` (this plugin). Production-readiness: Observable anchor — a boundary call with no log/metric/trace/correlation-id.
11. **Stateless-request-path probe** — `scripts/check-stateless-request-path.sh` (this plugin). Production-readiness: Horizontally-scalable anchor — node-local mutable state on the request path.
12. **Resilient-boundary probe** — `scripts/check-resilient-boundary.sh` (this plugin). Production-readiness: Resilient anchor — a boundary call with no timeout/retry/circuit-breaker/fallback.
13. **Logic tests** — fast, pure-function unit tests.
14. **Composition tests** — service + API + in-memory Port adapters wired together.
15. **Adapter Contract tests** — shared suite run against both the in-memory and real-backend adapters of each touched Port.
16. **Seam Behavior tests** — each touched seam's shared contract suite (`*.contract.test.*`) run against **both** sides (consumer-driven). Parity (Shape + suite exist) is checked structurally by step 8; this step proves the suite actually **ran and passed**.
17. **Integration boundary tests** — wrapper code (timeout/retry/circuit breaker) against contract-tested fakes or sandboxes.
18. **Journey tests** — for changes that affect a user-facing flow.

Capture exit code and the last lines of output. If anything fails, **bounce back** — implementer mode fixes; reviewer mode does not edit code. Re-run the entire `verify` entry point after the fix. Partial reruns are not evidence.

**Acceptance ↔ test traceability check.** Cross-reference the sprint's Acceptance ↔ Test Traceability matrix against the captured `verify` output. Every mapped test must have actually run. If a mapped test did not execute (filtered out, file renamed, silently skipped), treat it as a missing-coverage failure and bounce back — a green run that never exercised an AC's test is not evidence the AC holds.

**Reviewer evidence requirement:** the PR must include the captured `verify` output. A bare checkbox is not evidence; reviewer mode REJECTS the PR if the output is missing.

#### Environment Blocked ≠ Test Failed

Journey tests run against an Integration Env that may include systems you do not own. A test that cannot reach a third-party sandbox is **not** a regression in your code.

| Outcome                                                      | Report as | Action                                         |
| ------------------------------------------------------------ | --------- | ---------------------------------------------- |
| Assertion failed                                             | `FAIL`    | Investigate as a regression — do not merge     |
| External dependency unreachable before any business call     | `BLOCKED` | Notify env owner; do not block the PR          |
| External returned an error that the system handled correctly | `PASS`    | The Integration boundary layer earned its keep |

**TTL:** a test `BLOCKED` >5 times in 7 days is escalated; >10 times in 14 days is auto-archived with a tracking issue. `BLOCKED` is never a permanent state.

#### Debugging Loop Budget (self-applied discipline)

An unsupervised agent will happily "fix code" against a failure that is not a code regression — burning a session editing against, say, a platform that simply is not running. This is a **self-applied stop rule**, not runtime enforcement. (Per Praxis's stated boundary, hard runtime circuit-breakers — process kill, automatic escalation — belong to an orchestration runtime such as MPM. Praxis owns the discipline; the runtime owns the mechanism.)

Before **every** retry of the `verify` entry point after a failure:

1. **Classify the failure as `FAIL` vs `BLOCKED`** using the table above. A `BLOCKED` failure (environment unreachable, dependency not running) is **not** a reason to edit code — fix or escalate the environment instead.
2. **Increment the verify-attempt counter** in the progress ledger (`sprint-NNN-*.ledger.md`), scoped to the current root cause.

**Stop rule:** after **3 consecutive failed verify cycles on the same cause** (Praxis default; a project may override the number in its own context), **HALT**. Do not attempt a 4th code edit. Produce a halt summary and escalate to the human:

```markdown
## Debugging Halt — budget reached

Cause (unchanged across N attempts): …
Attempts: [what was tried each cycle]
Current FAIL/BLOCKED determination: [FAIL | BLOCKED + which dependency]
Hypothesis: …
Asking the human: [environment fix needed? scope/AC wrong? design flaw → bounce to architect?]
```

Reset the counter to 0 only when the root cause changes (a genuinely new failure), not when the same failure recurs with a tweaked patch.

### Step 4 — Refactor decision matrix (reviewer mode lens)

Reviewer mode reads the diff with two lenses: correctness (does it match the plan?) and **maintainability** (will this still be well-shaped six months from now?). The matrix below is the only allowed way to escalate a refactor opportunity into PR scope. Anything not marked **Block PR** is a follow-up by default — it does **not** expand this PR.

| Observed condition                                                            | Block PR | Follow-up issue | Inline change request |
| ----------------------------------------------------------------------------- | :------: | :-------------: | :-------------------: |
| Cross-capability deep import introduced                                       |    ✅    |                 |                       |
| Forbidden dumping ground introduced (`utils/`, `helpers/`, etc.)              |    ✅    |                 |                       |
| Port without parity (only one adapter, or contract suite missing)             |    ✅    |                 |                       |
| Declared seam without a Shape or Behavior suite (`check-seam-contract-parity`) |    ✅    |                 |                       |
| Dependent built against an unversioned/unfrozen seam (no `<name>@vN`)          |    ✅    |                 |                       |
| Touched seam whose shared Behavior suite did not run in `verify`              |    ✅    |                 |                       |
| Same property asserted at two pyramid layers                                  |    ✅    |                 |                       |
| Public-contract change without an `Accepted` ADR                              |    ✅    |                 |                       |
| Missing telemetry on a new boundary call                                      |    ✅    |                 |                       |
| Business logic placed in `*.api.*` or `*.repository.*`                        |    ✅    |                 |                       |
| Duplication across two adapters of the **same** Port                          |    ✅    |                 |                       |
| Unbounded timeout / retry / no fallback on external call                      |    ✅    |                 |                       |
| `console.log` / `print` on production path                                    |    ✅    |                 |                       |
| Skipped test (`.skip`, `xit`, `@Disabled`, etc.)                              |    ✅    |                 |                       |
| `sleep()` / `waitForTimeout()` introduced                                     |    ✅    |                 |                       |
| Service file growing past the project's size budget                           |          |       ✅        |                       |
| Duplication across **different** capabilities (extract or duplicate decision) |          |       ✅        |                       |
| Refactor opportunity that would unblock the next thin-slice cheaply           |          |       ✅        |                       |
| Naming inconsistent with capability vocabulary                                |          |                 |          ✅           |
| Comment that restates the next line                                           |          |                 |          ✅           |
| Missing `why`-comment on a non-obvious choice                                 |          |                 |          ✅           |

**Reviewer output contract:** for every row triggered, name the file, the smell, and the matrix decision. Do not invent rows; if a smell isn't in the matrix, propose adding it but treat it as a follow-up for this PR.

**No-write rule:** reviewer mode never edits source code. Inline change requests are written as structured comments the implementer applies in a follow-up commit. The bias firewall is the tool restriction, not the persona name.

**Drift policy:** if the diff drifts from product intent (changed user-visible behavior beyond the thin-slice scope, removed acceptance-criteria coverage), flag it but do not gate. PM gates intent drift at intake and at `close-sprint`.

### Step 5 — PR narrative

Produce a Pull Request description with these sections:

```markdown
## Summary

One paragraph. What changed and why.

## Capability and ADR

- Capability: `<name>`
- ADR: ADR.<ID> (link)

## Architectural changes

- Topology changes: …
- Contract changes: …
- Schema changes: …

## Behavior

- New: …
- Changed: …
- Removed: …

## Verification

- [x] `verify` entry point: command + exit code + last 20 lines of output pasted below
- [x] Format
- [x] Lint
- [x] Type check
- [x] Anti-dumping linter
- [x] No skipped tests linter (`scripts/check-no-skipped-tests.sh`)
- [x] No sleep-waits linter (`scripts/check-no-sleep-waits.sh`)
- [x] Port/Adapter parity gate (`scripts/check-port-adapter-parity.sh`) — every touched Port has both in-memory and real adapter passing the shared contract suite
- [x] Seam-contract parity gate (`scripts/check-seam-contract-parity.sh`) — every declared seam has a Shape and a Behavior suite
- [x] Config-externalization probe (`scripts/check-config-externalized.sh`) — Configurable anchor
- [x] Observability-at-seams probe (`scripts/check-observability-at-seams.sh`) — Observable anchor
- [x] Stateless-request-path probe (`scripts/check-stateless-request-path.sh`) — Horizontally-scalable anchor
- [x] Resilient-boundary probe (`scripts/check-resilient-boundary.sh`) — Resilient anchor
- [x] Logic tests (N tests, N pass)
- [x] Composition tests (N tests, N pass)
- [x] Adapter Contract tests (N tests, N pass — in-memory + real backend)
- [x] Seam Behavior tests (N tests, N pass — both sides, consumer-driven)
- [x] Integration boundary tests (N tests, N pass)
- [x] Journey tests (N pass, N blocked-with-reason)
```

<paste captured `verify` output here — reviewer mode REJECTS the PR if this is missing>

```
## Refactor decision matrix

- Block-PR rows triggered: [list, or "none"]
- Follow-up rows triggered: [link the issue, or "none"]
- Inline change requests: [list, or "none"]

## Telemetry

- Logs added: …
- Metrics added: …
- Trace spans added: …

## Risk and rollback

- Blast radius: …
- Reversible: yes / no
- Rollback procedure: …
- Monitoring signal that would indicate trouble: …

## Out of scope

- What this PR does _not_ do, and why.

## Follow-ups

- Tracked separately as: …
```

### Step 6 — Final self-correction

Before submitting, re-read the diff one more time. Check for:

- [ ] Comments that explain _what_ instead of _why_ — delete them.
- [ ] `TODO` / `FIXME` / commented-out code — remove or convert to a tracked follow-up.
- [ ] Unused imports, dead code paths.
- [ ] Inconsistent naming with the rest of the capability.
- [ ] Stylistic nits the reviewer will flag — fix yourself.

## Anti-patterns

- "Tests pass on my machine" without showing the output.
- Claiming complete with `--no-verify` or `--skip-tests`.
- Coverage report at the end of the PR. The leading indicator is the scenario list above, not a percentage.
- Hand-rolled jest stubs of your own repositories to make Composition tests faster. Use a real **in-memory Port adapter** that passes the same contract suite as the real one — fast **and** correct.
- Mocking a Port's real backend at the Adapter Contract layer. The whole point of that layer is to exercise the real backend.
- Adding only an in-memory adapter (or only a real one). The parity gate requires both.
- Hand-rolled stubs of third-party APIs without a contract test verifying them on a schedule. Stubs rot silently.
- `BLOCKED` used as a permanent state to hide a real failure. Enforce the TTL.
- Editing code against a `BLOCKED` (environment) failure instead of fixing the environment.
- Exceeding the debugging-loop budget — a 4th code edit against an unchanged failing cause instead of halting and escalating.
- "Rollback: revert the commit." Spell out the procedure including data implications.
- A PR description that doesn't name the trade-off the reviewer should evaluate.
