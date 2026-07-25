# Quality Spec — Executable Seams

> **Planning-stage document — an educated theory, not yet the truth.** The best approach given what we know today; current-state architecture lives in [docs/architecture/](../../../architecture/), promoted there by `close-sprint`.

The delivered slices are covered by shipped enforcement: `check-seam-contract-parity.sh` proves every declared seam has a Shape and a Behavior suite on disk, `verify-and-assemble-pr` Step 3 requires the shared suite to have run against **both** sides of each touched seam, and `check-sprint-id-collision.sh` is an exact rather than heuristic check.

`TS-003` (adversarial seam review) is deliberately **not** gated, and that is a quality decision rather than a gap. A trace or existence check proxying for reviewer judgment manufactures false confidence — Goodhart's law applied to assurance. Its only honest enforcement is a different head rejecting the PR; the ledger records which head reviewed and never certifies the verdict. Do not add a gate here later without an ADR superseding that reasoning.

For `TS-005`, the risk tier is high and the test layer is Logic: given two sprint footprints overlapping on any one of the four disjointness axes, the check must name the axis and both sprints. Both new checks must be negative-tested against a deliberately constructed overlap before they are trusted, since a check that has only ever seen a disjoint tree is unproven.

Author fully via `create-quality-spec` when `TS-005`'s trigger is reached.
