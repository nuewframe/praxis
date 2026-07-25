# Product Architecture — Executable Seams

> **Planning-stage document — an educated theory, not yet the truth.** The best approach given what we know today; current-state architecture lives in [docs/architecture/](../../../architecture/), promoted there by `close-sprint`.

The delivered architecture of this wave is already truth, not theory: the Seam Contract construct, `.seam-contracts.json`, and the parity gate are described in the capability records — [docs/architecture/skills/README.md](../../../architecture/skills/README.md) and [docs/architecture/enforcement/README.md](../../../architecture/enforcement/README.md). This document does not restate them.

What follows is the design for the **one open slice**, `TS-005`. It is authored here because this wave owns that slice; it is the only home for this design.

---

## Thesis for the open slice

Parallelism in Praxis is a **permission, not a plan**. The method never schedules concurrent work; it establishes when concurrent work is *safe*, so a human running parallel worktrees or an orchestration runtime fanning out can proceed without corrupting shared state. `TS-005` converts the last two safety rules from prose discipline into computation.

---

## The four-condition disjointness rule (currently prose)

Two units may be built concurrently only if they are disjoint across **all** of:

1. **Capability and files** — no overlapping capability slice or file glob.
2. **Persistent resources** — no shared table, topic, queue, cache, or migration.
3. **Shared configuration keys** — no key written by both.
4. **Contract dependency** — each depends only on a frozen `<name>@vN` contract, never on the other's in-flight internals.

Condition 4 is the one that makes the rest work, and capability-disjointness alone is explicitly insufficient — a five-slice trace demonstrated that two slices in different capabilities still collide through a shared resource or config key.

---

## Why this is not yet mechanical: the missing input

Conditions 1–3 are set intersections and condition 4 is a hash comparison. Both are trivially computable — **against inputs that do not exist**. Neither can be evaluated today because no sprint declares its footprint in machine-readable form.

**The sprint footprint** is the artifact `TS-005` must introduce: a structured block per sprint declaring

- touched capabilities and file globs,
- persistent resources (tables, topics, queues, caches, migrations),
- configuration keys written,
- depended-on `<name>@vN` contracts.

With that block, two probes follow directly:

| Probe | Computation | Input |
| --- | --- | --- |
| `check-sprint-disjointness.sh A B` | set intersection across conditions 1–3, plus condition 4's in-flight-internals test | two sprint footprints |
| `check-contract-freshness.sh` | hash comparison of each depended-on contract against `.seam-contracts.json` | one footprint + the seam manifest |

Both follow the established probe mechanic: warn-first, mechanical promotion to `enforce` via a config file, reviewed opt-out, `scanPaths` fallback, bash 3.2 compatible, wired through `verify.sh` and covered by `validate-plugin.sh`.

---

## Why it is deferred, and the trigger that ends the deferral

Building the footprint artifact plus two probes before a single real concurrent run is **speculative generality**. The value is exactly zero until a second concurrent slice exists, and the footprint's correct shape is best learned from a real dispatch rather than guessed. Designed now, built on trigger.

**Named build trigger:** the first time two slices are dispatched concurrently against this method — a human running parallel worktrees, or an orchestration runtime fanning out. Until then, the four conditions stand as prose discipline in `using-praxis`, the capability-driven guardrails, and the intake gate's re-anchor check.

This is deferral against a stated condition, not an open-ended "later". If concurrent dispatch happens and these probes do not exist, the deferral has failed and the slice is overdue.

---

## The enforceability split this wave established

The reason `TS-005` is deferred while its sibling shipped, and why `TS-003` is deliberately never gated, is one principle applied three ways. It is recorded here because it governs future enforcement decisions in this wave:

| Kind of rule | Example | Honest enforcement |
| --- | --- | --- |
| **Exact invariant on artifacts that already exist** | duplicate sprint id — the filenames already carry it | Ship the gate now. Zero judgment, zero new input. Delivered as `check-sprint-id-collision.sh`. |
| **Computation on inputs that must first be declared** | disjointness, contract freshness | Design now, build on a named trigger. Mechanizable in principle; premature without the input artifact. |
| **Judgment** | whether an adversarial seam review was genuinely adversarial | Refuse the gate. A trace check proxies presence for quality and invites checkbox theater — the exact failure mode inverted. Enforce by a different head rejecting the PR; record *which head*, never certify the verdict. |

---

## Seams

This wave introduces the seam **construct** rather than consuming a boundary of its own. The seams it declares are the `<name>@vN` contracts in `.seam-contracts.json`. `TS-005` adds no runtime boundary: the footprint is a build-time artifact and both probes are single-shot CLI checks, so no timeout, retry, or fallback posture applies.
