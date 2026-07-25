# Quality Spec — Self-Conformance

> **Planning-stage document — an educated theory, not yet the truth.** The best approach given what we know today; current-state architecture lives in [docs/architecture/](../../../architecture/), promoted there by `close-sprint`.

_Stub. Author via `create-quality-spec`._

This wave's "code" is prose, skill definitions, and self-test scripts, so the Pyramid maps to script-level verification: Logic at individual `check-*.sh` / `validate-plugin.sh` checks, Composition at the suite's exit code, and Journey at `create-wave` running end-to-end plus the session-start hook emitting valid JSON. Every new check must be negative-tested — a deliberate defect introduced, confirmed to fail the build with a `file:line`, then reverted.
