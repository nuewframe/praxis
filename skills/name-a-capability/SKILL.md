---
name: name-a-capability
description: >
  Name a capability by deriving it from a cluster of events in a storm, never by asserting it. Teaches
  clustering on consistency boundaries, the four gate tests each with its reason, and the exclusion that
  makes a boundary real. Runs after an event storm, inside DISCOVER.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, create_file, replace_string_in_file, run_in_terminal]
---

# Skill: Name a Capability (`skills/name-a-capability/`)

Use this after a storm, when the events have been swept and it is time to say what the system must be
able to **do**.

**Audience:** Maintainer, Product Manager, Principal Engineer. **Phase:** `DISCOVER`.

---

## The rule, and why it is the whole point

**A capability owns a cluster of events from a storm, or it is an assertion.**

A capability named straight off a problem statement is a claim made by whoever produced it — which is
the trust-transfer problem reproduced *inside* the method meant to solve it. So the derivation is
recorded, and the checker refuses a capability that does not carry one:

```
× untraced: missing `from-cluster` — the cluster IS the derivation. Naming a
│ storm without the cluster inside it proves nothing
```

Naming a cluster that does not exist is refused too. Presence is not a trace.

## Cluster on what must stay consistent, not on what sounds related

Group the storm's events by **what must stay consistent together**. Not by subject matter, not by who
does them, not by which part of the system you imagine owning them.

The test that does the work: *if this fact changed, which other facts must change with it?* Those
belong together. That is a consistency boundary, and it is what a capability is.

Two consequences people trip on:

- **A cluster is not a layer.** "Everything about validation" is a subject, not a boundary.
- **One event has exactly one owner.** If two capabilities both claim an event, the boundary between
  them is drawn wrong, and the checker says so by naming both claimants. Resolve it by moving the
  boundary, never by letting both keep it.

An event **no** capability owns is reported rather than refused. It belongs to nothing, which is a gap
in the model rather than a malformed record — and a gap you want to see rather than one you want to
fail on.

## Apply the four gate tests, and record the reason each passes

A candidate becomes a capability only by passing all four, and the **reason** is recorded, not just the
verdict:

1. **Names a doing.** Not a noun someone liked. What can the system now *do*?
2. **Fundable on its own terms.** Worth building with nothing else present.
3. **Slices deliver independently.** You can name several slices, each useful before the next exists.
4. **States an exclusion.** What it is *not*, as something else's job — never scope restated.

A fifth clause, learned the hard way: **a candidate that keeps no state of its own is a slice, not a
capability.** Verification felt like a peer of projection until this was applied; it keeps nothing
consistent that projection does not already keep.

Record the reasoning, not the verdict. A gate test whose reason is recorded can be re-run by someone
else; one that only carries "pass" is an opinion with a checkmark next to it.

## The exclusion is the load-bearing field

`not` is where boundaries actually get drawn. It fails when it restates scope — *"it does not do
anything outside its remit"* says nothing. It works when it names a **specific** thing that a
**specific** other capability owns.

If you cannot write the exclusion, you have not found the boundary yet. Go back to the clusters.

---

## What this skill refuses to do

- **It does not merge or split clusters for you.** Judgement stays with the maintainer; only the trace
  is enforced.
- **It does not decide build order.** That is the slice set's argument, cut in the same phase.

## Related

- `event-storming` — the storm a capability must be derived from.
- `cut-a-slice` — what to do once the capability exists.
