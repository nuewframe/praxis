---
name: cut-a-slice
description: >
  Cut an atomic vertical slice from a capability, in a form a checker can refuse rather than a form the
  author asserts. Names the one command or one read model, the one actor, the events it produces, the
  layers it must reach, and the claims that will settle it. Runs after an event storm, inside DISCOVER.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, create_file, replace_string_in_file, run_in_terminal]
---

# Skill: Cut a Slice (`skills/cut-a-slice/`)

Use this when a capability exists and a piece of it is worth doing before the rest.

**Audience:** Maintainer, Product Manager, Principal Engineer.
**Phase:** `DISCOVER`. Cutting is how you learn what a capability is, so it happens *inside* discovery
rather than after it — a capability no slice can be cut from is evidence the capability is wrong.

---

## The rule this skill exists to enforce

**Verticality is not an adjective you apply to your own work.** A slice claimed vertical that cannot be
shown to have cut anything is symptom `S4` of the trust-transfer frame. The answer is not more careful
prose. It is to write the slice in a shape that something else can refuse.

So the discipline is: **declare before you build, and declare what would prove you wrong.**

---

## Name the command before the solution

A slice is anchored to something that actually happens in the domain — one command an actor issues, or
one view an actor must see before they can act. Two kinds, and no third:

- A **command slice** — an actor does something, and a fact becomes true.
- A **view slice** — an actor needs to see something before they can act. Nothing is written.

If you cannot name the one command or the one view, you have not cut a slice. You have named a project.
Three commands in one file is three slices, and it will be discovered at close rather than at cut.

**A refusable command names both outcomes.** If the command can be refused, the refusal is a fact too —
a gate that was never invoked and a gate that refused look identical afterwards, and that difference is
the entire point. Name the refusal event alongside the success event.

## Name the one actor

Not a role list. The one person or agent who issues this command. If the answer is "the team", the slice
is a phase.

## Declare the layers before you know what they cost

A slice is vertical when it reaches **every layer it requires** — not every layer that exists. Most
slices legitimately touch a subset, and a slice that reaches one layer is reported rather than rejected.

What makes verticality testable is that you name the subset *first*, and say what reaching each layer
means for this slice specifically. The set seals at the first iteration. **It cannot be trimmed at close
to make "done" cheaper** — that is the whole reason it is declared early rather than described late.

## State the claims, and name their evidence at cut time

Each claim says what will be true when this is done, and names the evidence that will settle it —
**now, before the work starts.** Naming the evidence afterwards lets the worker pick convenient proof.

A slice with no claim cannot go unmet, so nothing can ever object to it.

---

## Do not memorise the fields — run the checker

The exact fields, their cardinalities and their vocabularies are **declared in the record**, on the
notional architecture's `schema` block. They are not restated here on purpose: doctrine that duplicates
a fact the graph owns becomes a second writer for it, and the two drift.

```console
$ praxis check praxis
praxis: praxis ok — 10 entity kinds declared
```

A slice that is missing something is refused by name, with the reason the schema itself gives:

```
× TS.999999.01: missing `layer` — verticality is the declared set, and a
│ slice with none asserts nothing
   ╭─[praxis/…/TS.999999.01.a-horizontal-fragment.kdl:1:1]
```

If you find yourself asking "what fields does a slice need?", the answer is the checker's, not this
document's. Ask it.

---

## What this skill refuses to do

- **It does not judge whether the slice is worth doing.** That is the capability's business, and the
  gate's at pickup.
- **It does not admit work.** Cutting costs nothing and commits to nothing — selection at pickup is the
  commitment.
- **It does not restate the schema.** See above; that is the point.

## Related

- `event-storming` — the storm a slice's command must appear in.
- `create-capability-record` — a slice with no capability is unfunded work.
