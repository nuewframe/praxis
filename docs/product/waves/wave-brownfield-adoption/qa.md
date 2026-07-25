# Quality Spec — Brownfield Adoption

> **Planning-stage document — an educated theory, not yet the truth.** The best approach given what we know today; current-state architecture lives in [docs/architecture/](../../../architecture/), promoted there by `close-sprint`.

_Stub. Author via `create-quality-spec`._

The highest-risk behavior in this wave is **fabrication**: a path that produces retrospective waves can be followed in a way that invents evidence, and the resulting artifact asserts validated learning it does not contain. That risk is not testable by a script — an invented evidence line and a real one have the same shape — so it belongs to the artifact-fidelity review (`verify-and-assemble-pr` Step 6) rather than to a probe.

What *is* mechanically checkable: that a delivered wave carries no acceptance criteria or hypothesis card, that an open wave carries all four documents, and that every evidence link resolves (check #14 already covers file-level targets; anchor fragments are a known gap noted in [ADR.260725.10](../../../architecture/adr/ADR.260725.10-brownfield-wave-retrofit.md)).

Explicitly out of scope for testing: whether the derived wave boundaries are the *right* ones. That is a judgment call, and a metric proxying for it would manufacture false confidence.
