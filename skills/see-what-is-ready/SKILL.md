---
name: see-what-is-ready
description: >
  Ask the record which slices could be started right now, and what would refuse each of the rest, before
  choosing what to work on. Teaches reading the answer — including the conditions nobody computed, which
  are the ones that matter — and what to do when the ready set is short. Runs before pick-up, inside ITERATE.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, run_in_terminal]
---

# Skill: See What Is Ready (`skills/see-what-is-ready/`)

Use this **before** choosing a slice, never after. Selection is the commitment, and this is the
information the commitment needs.

**Audience:** Working agent, Maintainer. **Phase:** `ITERATE`, before `pick-up`.

---

## Ask

```
praxis ready [root]
```

It is computed on demand and never committed. `right now` is in the question: frozen, this answer is
not merely stale, it is **wrong**, so there is no file to read instead and no cached copy to go looking
for. If you find one, it is a bug.

---

## The rule

**A gate that shows only its verdict teaches nothing. A gate that shows its conditions teaches the
next agent how work is admitted.**

So the answer has two halves, and most readers only look at the first:

| Section | What it is for |
| --- | --- |
| `ready` | the slices nothing blocks — the answer to the question you asked |
| `blocked` | every other slice, and **the specific thing** blocking it |
| `delivered` | done, which is not the same as blocked and must never look like it |
| `not decided for you` | conditions the gate declares and **did not decide** |
| `record disagrees with itself` | a slice's own `state` against what its iterations show |

---

## Read the fourth section first

`not decided for you` is the section that exists because of `S3`. A gate that was never invoked and a
gate that refused look identical afterwards — and at the granularity of a single condition, *"nothing
blocks this"* and *"blocking was not computed"* look identical too. So an uncomputed condition is never
a pass:

- a condition the record declares with `uncomputed=` says why, and appears here;
- a condition the record declares with `judgement=#true` appears here, because the tool shows and does
  not rank — selection stays the maintainer's;
- a condition naming a `check=` the **engine does not have** appears here naming that check, so a rule
  declared and never implemented cannot pass silently.

A slice in `ready` is a slice nothing *computable* blocks. It is not a slice that is safe to start.
Read what was not decided, then decide it yourself.

---

## When the ready set is short

A short `ready` set is not a sign that there is nothing to do. It is a sign about the **shape of the
dependencies**, and it is worth reading that way:

1. Look at what the blocked slices are blocked *by*. If almost all of them are blocked by
   `dependencies-delivered`, the backlog is a **queue, not a set** — the work is serial no matter how
   many agents are available.
2. Look at what those dependencies *name*. A dependency naming a **slice** blocks until that whole
   slice is delivered. A dependency naming a **frozen seam** blocks only until the contract is frozen —
   which is usually already true.
3. That is the argument for `depends-on` naming a seam rather than a slice, and it is not a matter of
   notation. It is the difference between a chain and a fan.

Do not "fix" a short ready set by weakening a condition. Fix it by cutting the dependency at a contract.

---

## What this does not do

- **It does not pick.** Seeing is not choosing; the commitment happens at pick-up.
- **It does not rank or recommend.** There is no "next" and no priority column, on purpose.
- **It does not judge the work.** Readiness and disjointness only — never whether a slice is any good.

---

## Where the conditions come from

Not from the engine. They are declared on the notional architecture, under the capability that owns
admission:

```kdl
admits "an iteration on a slice" {
    condition "dependencies-delivered" check="dependencies-delivered" \
        because="an iteration standing on undelivered work is standing on nothing"
    condition "work-is-disjoint" uncomputed="no seam manifest exists, so what two iterations \
        would touch cannot be compared"
}
```

Adding a condition is an **amendment to the record**, and it takes effect the moment it is declared —
as an uncomputed one, named in the answer, until a check for it exists. That ordering is deliberate:
the gap is visible from the moment it is admitted, rather than after someone remembers to write it up.

Related: `cut-a-slice` · `declare-an-entity-kind` · `name-a-capability`.
