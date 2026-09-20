---
name: anchor-the-work
description: >
  State the vision, the mission and the strategy a frame is worked under. Teaches why these are not
  checked for truth, why a mission carries a falsifier and a vision does not, why a strategy needs
  none at all, and how the work informs the strategy back. Runs before the first frame.
user-invocable: true
disable-model-invocation: false
tools: [read_file, grep_search, replace_string_in_file, run_in_terminal]
---

# Skill: Anchor the Work (`skills/anchor-the-work/`)

**Audience:** Whoever starts a product. **When:** before the first frame is written.

---

## The frame was the root, and it floated

Everything a Praxis record holds descends from a frame, and a frame is a **problem**.
`root-cause` says why the *problem* exists. Nothing said why the **product** does, or for whom,
or what would count as it having worked.

The consequence turned up exactly where you would expect: a published release could tell a
reader **how**, because usage is in the record — and could not tell them **why**.

## Three kinds, because they move on different schedules

```kdl
vision "trustworthy-by-construction" {
    is "a team can trust what an agent produced without re-deriving it"
}

mission "fidelity-is-computed" {
    is "make execution fidelity a property computed from the record"
    toward "trustworthy-by-construction"
    delivered-when "somebody who did not do the work can ask the record what happened
        and act on the answer, without asking the person who did it"
}

strategy "name-the-person-before-the-system" {
    serves "fidelity-is-computed"
    step "1" is="purpose — the outcome this exists to deliver" {
        satisfied-by "mission"
        satisfied-by "vision"
    }
    step "2" is="audience — the primary persona, named before anything is framed" {
        satisfied-by "persona"
    }
    …
}
```

A vision outlives every mission under it. A strategy is revised by findings while the mission
holds. **Collapsing them into one kind would mean amending the vision every time the approach
changed**, and the record would stop being able to say which one moved.

## What each is checked for

| | Falsifier | Because |
|---|---|---|
| `vision` | **none** | a world state cannot be tested. Asking whether a vision is *true* is the category error that kept all three out of the record |
| `mission` | **`delivered-when`, required** | this is what separates a mission from an aspiration. A mission nobody could be wrong about is a slogan |
| `strategy` | **none** | it is an **ordering**. What it earns is adherence, and conformance to a declared sequence is computable |

**The check is traceability, never truth.** `frame → strategy → mission → vision` is followed by
`dangling-relationship` with no rule of its own — the same check this record makes everywhere
else, that nothing is orphaned.

> a frame tracing to no strategy is work nobody can justify

---

## A strategy earns adherence instead of a falsifier

That is the whole reason a strategy carries none — and for a day it was an argument nothing
backed. `work-follows-the-declared-sequence` is the backing.

**Say what occupies each step.** `satisfied-by` names the *kinds*, and the rule reads them:

> step `"2"` is satisfied by nothing, and step `"5"` is already occupied by `thin-slice`.
> The strategy declares the order and this record is working it out of sequence.

Three properties, each chosen over an obvious alternative:

| | | Because |
|---|---|---|
| computed from what the record **holds** | not from when anything was written | a report derived from trail timestamps could never be cleared — this record's frame predates every persona by thirty iterations and always will. A state report clears the moment you write the missing thing, which is the only reason to tell somebody they are out of sequence |
| **reported**, never refused | not a failing check | being out of sequence is a fact about a project's *state*. A greenfield repository working down the steps is legitimately incomplete, and failing its check there means failing it during the hour it is most needed |
| the **strategy** declares the mapping | not the engine | an engine that knew what `audience` *meant* could diverge from the record and nobody could tell. Your strategy's steps are yours; declare their `satisfied-by` and the same rule works |

**A step with no `satisfied-by` is not checked.** A strategy is prose you write first and map to
kinds later. Requiring the mapping upfront would make declaring one a schema exercise, which is
how a strategy ends up unwritten.

**A step naming a kind the schema does not declare is reported too** — nothing can ever occupy
it, so it reads as checked while checking nothing.

## The arrow runs both ways

A strategy anchors the work, and **the work within a frame informs the strategy.** It is revised
by *appending*, naming the evidence that forced it — a finding, a walkthrough, a refusal.

A strategy the work can always revise constrains nothing. One it can never revise is a slogan
nobody revisits. What separates learning from rationalisation is whether the revision can name
what taught it.

This record's own strategy was **derived from the work**: thirty-odd slices, a withdrawn
release, and reading the published set as somebody who had not built it produced that order.
The loop ran once before anything existed that could record it had.

---

## What this does not do

**It cannot tell you the vision is right.** It makes the chain legible, so a reader can ask what
any piece of work is ultimately for and get an answer — and disagree with it.

## Related

- `name-a-persona` — the audience is step two, and the primary one is named before framing
- `mature-a-value` — how a strategy is revised without losing what it used to say
- `cut-a-slice` — everything downstream of a stated purpose and a named person
- `declare-an-entity-kind` — what a `satisfied-by` may name
