---
name: plan-before-building
description: >
  Produce the approach before the implementation, and record what it was chosen over. Teaches what
  each of the five phases must produce, why design-system only works before the code exists, why
  teach and the doctrine layer are different things, and how to correct an approach mid-flight.
  Runs when an iteration opens, before the first edit.
user-invocable: true
disable-model-invocation: false
tools: [read_file, grep_search, replace_string_in_file, run_in_terminal]
---

# Skill: Plan Before Building (`skills/plan-before-building/`)

**Audience:** Whoever works an iteration. **When:** before the first edit, not at close.

---

## What each phase produces

| Phase | Produces |
|---|---|
| `design-ux` | what the actor must see and decide before acting, and what they will **not** read |
| `design-system` | the **approach** — what will be built and what it was chosen over |
| `implement` | the thing itself, naming the approach it followed |
| `learn` | what the work taught, including where the approach turned out wrong |
| `teach` | what the **end user** reads: the version's docs, the feature's why and how |

## Why design-system only works before

An approach written at close is a **summary of what was built**. It cannot be argued with while
there is still time to change what gets built, which is the only moment arguing helps.

This record proves it against itself. Every `design-system` entry in `FRAME.260819.01` is
past-tense, written by whoever did the work, at close. `ITER.260822.13`'s said *"only the head
of a chain is checked"* — a decision made **during** implementation, attested as a design
phase, and wrong. The better approach surfaced in conversation after the code shipped.

```kdl
phase "design-system" state="complete" {
    approach "carry the plan in the iteration" {
        over "a design document per slice, under docs/"
        over "an approval gate between design and implement"
        because "the approach belongs where the claims it will settle already are.
            A separate file is a second thing to keep in step"
    }
}
phase "implement" state="complete" followed="carry the plan in the iteration"
```

**`over` is required.** An approach with nothing rejected is a description of the only thing
anybody thought of, not a choice.

## When the plan turns out wrong

**Mature it. Do not edit it.**

```kdl
matured "approach" {
    from "three approaches, recorded before any code existed"
    to "four — the fourth discovered while implementing"
    at "2026-08-22"
    because "the plan did not decide whether a field is a child node or a property"
    taught-by "ITER.260822.14"
}
```

Discovering the design was wrong while building is the **normal case** and must stay cheap. A
rule that punished it would teach people to write the approach at close, which is the
behaviour being fixed. What matters is that the plan and the discovery stay
distinguishable — that is the whole value of having written it down first.

## teach is not the doctrine layer

A skill teaches an **agent** and is the `doctrine` layer. A version's docs teach the **reader
of a release**. Every iteration in this frame named one skill file under both — one artifact
counted twice — and the end user was served by the teach phase zero times before this slice.

`taught` names the persona reached. That is a reference: naming a reader the record does not
hold dangles, the same as any other edge.

---

## What this does not do

**It cannot tell you whether the approach was good.** It makes the choice visible and
contradictable. A bad approach recorded beats a better one nobody wrote down, because only the
first can be argued with before it ships.

It also does not gate. The same agent may design and build — what is refused is implementing
with **no approach recorded**. Who may *attest* the result is `close-an-iteration`'s subject
and a different rule entirely.

## Related

- `mature-a-value` — an approach that turns out wrong is matured, never overwritten
- `close-an-iteration` — where the phases are accounted for
- `name-a-persona` — `taught` names one, and it must be a persona the record holds
