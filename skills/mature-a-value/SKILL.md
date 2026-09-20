---
name: mature-a-value
description: >
  Record a value that changed as a change, rather than overwriting it. Teaches the three moves an
  iteration makes on a record, why change is the one that usually leaves no trace, and why a
  maturation must cite what taught it. Runs whenever you improve something the record already says.
user-invocable: true
disable-model-invocation: false
tools: [read_file, grep_search, replace_string_in_file, run_in_terminal]
---

# Skill: Mature a Value (`skills/mature-a-value/`)

**Audience:** Whoever changes something the record already said. **When:** in the same edit.

---

## The three moves

An iteration does one of three things to a record:

| Move | Leaves a trace? |
|---|---|
| **Adds** a value | trivially — a new node is a new node |
| **Removes** a value that did not work | yes — `stands-at`, `withdrawn-because`, `previously-cut` |
| **Changes** a value for a better one | **usually not.** It gets overwritten. |

Two of the three were covered in this record and the third was not. Twenty-six retired
surfaces name the tag they still stand at; a withdrawn release keeps the index it was cut at.
And `every-shipped-surface-is-anchored` flipped from `report` to `refuse` leaving a **note**,
while `CAP.conformance-probes` was narrowed on port with its prior claim simply gone.

Both changes were right. **Neither was a record** — and afterwards, *"we changed our mind"* and
*"we were always right"* look identical.

## What to write

```kdl
matured "severity" {
    from "reports"
    to "refuses"
    at "2026-08-22"
    because "it named forty-five files on the day it landed, and a rule that fails closed
        on its first run is one nobody can adopt. The count reached zero"
    taught-by "ITER.260822.04"
}
```

The first argument is **the field that changed**. `from` is the half that gets lost and the
half that makes the change legible — a change that does not say what it was is a new value
wearing a change's clothes.

**`taught-by` is required.** A change citing nothing is a rewrite, which is exactly the
discipline a strategy revision and an emergent persona are already held to.

## Two rules

- **`a-value-agrees-with-its-maturation`** — if the maturation says the value became X and the
  record holds Y, one of them is wrong and a reader cannot tell which. That is *worse* than not
  recording the change, because it reads as an account of what happened.
- Every field of a maturation is required. Absence is how a change becomes a rewrite quietly.

## Maturing more than once is the point

```kdl
matured "doing" { from "the first thing"  to "the second thing" … }
matured "doing" { from "the second thing" to "the third thing"  … }
```

Only the **latest** is checked against the current value. A field that matured twice has a
stale first `to` by construction — that is what iterating *is*, and refusing it would make a
chain of improvements look like a defect. The earlier entries are history, and history is why
the shape exists.

---

## What this does not do

**It cannot see a change nobody recorded.** From one snapshot it is not possible, and doing it
from git would point outside the record — which the durability principle forbids, and which
would make this rule depend on the least durable thing available.

It makes the recorded changes trustworthy and gives the rest somewhere to go.

It is also **not a history of every edit**. A change that taught nobody anything is a typo, and
typos are not values.

## Related

- `declare-a-lifetime` · `cut-the-version` — removal already matures forward; this is the third move
- `record-a-decision` — a decision is amended, never rewritten, for the same reason
- `name-a-persona` — an emergent persona cites what found it, which is the same discipline
