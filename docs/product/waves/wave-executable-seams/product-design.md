# Product Design — Executable Seams

> **Planning-stage document — an educated theory, not yet the truth.** The best approach given what we know today; current-state architecture lives in [docs/architecture/](../../../architecture/), promoted there by `close-sprint`.

This wave is mostly delivered; only `TS-005` remains open. Its architecture — the sprint footprint, the two probes, the four disjointness conditions, and the reasoning for deferring against a named trigger — is in [product-architecture.md](product-architecture.md).

The user is a **team or orchestration runtime dispatching two units of work concurrently**. The design question for `TS-005` is what that user is shown when disjointness is violated: which of the four conditions overlapped, between which two sprints, and which of three responses is correct — wait, re-scope, or re-anchor against a moved contract. A probe that reports only "not disjoint" pushes that diagnosis back onto the user and will be ignored.

The secondary design question is who authors the footprint. If it is hand-written per sprint it will drift from the sprint's actual reach; if it is derived from the diff it cannot be known before the work starts, which is when disjointness must be evaluated. That tension is unresolved and should be settled before the probes are built.

Author fully via `create-product-design-spec` when `TS-005`'s named trigger is reached — the first concurrent slice dispatch.
