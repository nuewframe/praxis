---
name: define-seam-contract
mode: architect
tools: [read_file, file_search, grep_search, semantic_search, create_file, replace_string_in_file]
description: >
  Define a Seam Contract for a boundary — the machine-checkable description of how one
  unit depends on another's promise. Architect-mode skill: produces a machine-readable
  Shape, registers the shared Behavior test-suite location, and assigns a frozen
  `<name>@vN` contract id in the project seam manifest (`.seam-contracts.json`). The
  frozen version is what lets a dependent slice build against a promise instead of
  waiting for the producer's internals — converting "wait for merge / snapshot rots"
  into "build against a frozen contract." Pairs with `check-seam-contract-parity.sh`
  (the conformance gate) and `create-product-architecture-spec` (where seams are declared).
user-invocable: true
disable-model-invocation: false
---

# Skill: Define Seam Contract

Use this skill when a wave or slice crosses a **seam** — any boundary where one unit
depends on another unit's promise — and that boundary needs to be honored by both sides
*executably*, not asserted in prose.

This is the keystone of the **Executable Seams First** plan (`docs/plans/executable-seams-first.md`):
quality is a property of the seam, and a Seam Contract is the one new first-class artifact
that converts a seam from *asserted-correct in markdown* to *executed-correct in the pipeline*.

---

## What counts as a seam

A seam is any boundary where one unit depends on another's promise:

| Seam kind | Example | Default Shape form (D2) |
| --------- | ------- | ----------------------- |
| `http`  | A public HTTP/REST API or RPC endpoint | **OpenAPI** (`.openapi.yaml` / `.openapi.json`) |
| `event` | A published message / event on a topic or queue | **JSON-Schema** (`.schema.json`) |
| `port`  | A capability ↔ adapter Port (already half-covered by `check-port-adapter-parity.sh`) | **native typed interface** (`*.ports.<ext>`) |
| `cli`   | A command-line contract another tool scripts against | usage/spec doc or schema |

A project MAY override the default Shape form per kind in `praxis.config` (decision D2),
but the form must stay machine-readable so the parity gate can verify a Shape exists.

**Not every function is a seam.** Only boundaries *named in `product-architecture.md`*
carry a contract (the §7 anti-bureaucracy rule). If it isn't a declared boundary between
owners, it doesn't need a Seam Contract.

---

## The three parts of a Seam Contract

| Part | What it is | Where it lives |
| ---- | ---------- | -------------- |
| **Shape** | The machine-readable interface — typed signature, OpenAPI, JSON-Schema, event schema | A file under the project contracts dir (or alongside the Port) |
| **Behavior** | The **shared** contract test suite both sides must pass — timeout, retry, idempotency, error envelope, ordering | A `*.contract.test.*` suite run against both sides |
| **Version** | A frozen `<name>@vN` id a dependent builds against, pinned *before* the producer's internals finish | An entry in `.seam-contracts.json` |

The **Behavior** suite is the same machinery as the Adapter Contract layer in
`test-by-ownership` — one suite, run against both the consumer's expectation and the
producer's implementation (consumer-driven). Reuse it; do not invent a parallel concept.

---

## Step 1 — Identify the boundary and its owners

State, in one or two sentences:

- **Producer** — the unit that makes the promise (owns the implementation).
- **Consumer(s)** — the unit(s) that depend on the promise.
- **What crosses** — the data/operation and its direction.

If two units both "sort of own" the seam, stop — resolve ownership in
`product-architecture.md` first. A contract with ambiguous ownership rots.

---

## Step 2 — Assign a frozen contract id

The id is `<name>@v<N>`:

- `<name>` — kebab-case, unique within `.seam-contracts.json` (e.g. `publish-api`, `charge-event`).
- `@vN` — integer version starting at `v1`.

**Versioning rule (why the version is not bureaucracy):** a dependent slice depends on
`<name>@vN`, *not* on the producer slice. The id is frozen the moment it is published —
before the producer's internals are done — so the dependent can start early and is immune
to unrelated merges. Bump to `@v(N+1)` **only on an incompatible Shape change**; keep the
old version published until every consumer has migrated. Never mutate a published version
in place.

Pick the lowest free version for a new seam (`@v1`). Existing published ids are immutable.

---

## Step 3 — Write the Shape

Produce the machine-readable Shape in the default form for the seam kind (Step’s table
above), unless `praxis.config` overrides it. Keep it to the contract surface — request/
response shape, error envelope, event fields, method signatures — not the implementation.

Write it to the project contracts directory (default `docs/product/contracts/`,
overridable). Suggested file naming:

```
<name>.v<N>.openapi.yaml      # http
<name>.v<N>.schema.json       # event
<base>.ports.<ext>            # port  (lives in the capability folder, not the contracts dir)
<name>.v<N>.cli.md            # cli
```

The Shape must be non-empty and parseable in its form (the parity gate checks existence
and non-emptiness; CI schema-validation, where present, checks well-formedness).

---

## Step 4 — Register the shared Behavior suite

Name the location of the **shared** contract test suite — the one suite both sides run.
It does not have to be fully written yet (TDD: it can start red), but its path must be
declared so the parity gate can see it and the producer/consumer know where to converge.

Convention: `*.contract.test.*` placed alongside the capability it verifies, e.g.
`src/publishing/publish-api.contract.test.ts`.

The suite must cover the Behavior properties that matter at this seam:

- Error envelope shape on failure
- Timeout / retry behavior at the boundary
- Idempotency under retry (where the operation claims it)
- Ordering / at-least-once / exactly-once semantics for events
- Authorization failure mode

---

## Step 5 — Record the seam in the manifest

Add (or update) the seam entry in `.seam-contracts.json` at the repo root. Create the
file if it does not exist:

```json
{
  "mode": "warn",
  "contractsDir": "docs/product/contracts",
  "seams": [
    {
      "id": "publish-api@v1",
      "kind": "http",
      "shape": "docs/product/contracts/publish-api.v1.openapi.yaml",
      "behavior": "src/publishing/publish-api.contract.test.ts"
    }
  ]
}
```

- `mode` — `warn` (default) until a wave closes with every declared seam in parity, then
  promote to `enforce` (decision D3). Warn-first prevents a wall of legacy hits from
  making the team disable the gate.
- `shape` / `behavior` — exact path or glob; at least one non-empty match must exist.

---

## Step 6 — Verify parity locally

Run the conformance gate before handing off:

```
bash scripts/check-seam-contract-parity.sh .
```

It must report the seam as in parity (Shape present, Behavior suite present). The gate
proves the *structural* half — both halves exist. The **executable** half (the suite
actually ran and passed against both sides) is enforced by the `verify` entry point and
the Acceptance ↔ Test traceability check in `verify-and-assemble-pr` Step 3.

---

## Quality Checklist

- [ ] The seam is a boundary between distinct owners, named in `product-architecture.md`
- [ ] Producer and consumer(s) are explicit; ownership is unambiguous
- [ ] The id is `<name>@vN`, kebab-case, unique, and frozen
- [ ] The Shape is machine-readable in the default form for its kind (or a `praxis.config` override)
- [ ] The shared Behavior suite path is declared (one suite, run against both sides)
- [ ] The manifest entry exists in `.seam-contracts.json`
- [ ] `check-seam-contract-parity.sh` reports the seam in parity
- [ ] A dependent slice can name `<name>@vN` as its dependency instead of the producer slice

---

## Anti-Patterns

- Treating every function as a seam — only declared boundaries carry contracts
- A Shape in prose only — it must be machine-readable so the gate can verify it exists
- Two separate test suites (one per side) instead of one shared, consumer-driven suite
- Mutating a published `@vN` in place instead of cutting `@v(N+1)`
- Depending on the producer *slice* instead of the frozen *contract version* — that is the
  exact coupling the version exists to remove
- Declaring a seam but leaving `mode: enforce` on day one against a legacy codebase — burn
  down or opt-out existing hits under review first, then promote
