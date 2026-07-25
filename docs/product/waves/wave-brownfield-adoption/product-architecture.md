# Product Architecture — Brownfield Adoption

> **Planning-stage document — an educated theory, not yet the truth.** The best approach given what we know today; current-state architecture lives in [docs/architecture/](../../../architecture/), promoted there by `close-sprint`.

_Stub. Author via `create-product-architecture-spec`._

This wave declares **no seams**: it adds no runtime boundary, no external call, and no request path. Its output is skill prose and documented method, homed in the `skills` capability — [docs/architecture/skills/README.md](../../../architecture/skills/README.md).

The open architectural question is `TS-002`'s: whether multi-package and multi-repository context resolution is a precedence rule the agent applies at read time, or a generation step that materializes per-package context from one source. That choice determines whether the dual-home generator discussed in `TS-007` of [wave-self-conformance](../wave-self-conformance/README.md) is a dependency or an alternative.
