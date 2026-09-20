---
name: declare-an-entity-kind
description: >
  Declare a new kind of thing the record must hold, and the shape its records must carry, on the
  architecture's schema block. Extending the vocabulary is an amendment to the record, never a change
  to the engine. Use before writing records of a kind that does not yet exist.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, replace_string_in_file, run_in_terminal]
---

# Skill: Declare an Entity Kind (`skills/declare-an-entity-kind/`)

Use this when the model gains a kind of thing the record cannot yet hold.

**Audience:** Maintainer, Principal Engineer. **Phase:** `DISCOVER`.

---

## The rule

**The engine may not encode what the record declares.** Every entity kind, every field, every
cardinality and every closed vocabulary lives on the architecture's `schema` block and is loaded at
runtime. So adding a kind is an **amendment to a decision**, not an edit to a program — and a rule the
record does not state is a rule nothing may enforce.

The corollary is the part people skip: **a kind nobody declared cannot be checked.** It is not
"unchecked but fine". It is invisible, and it will stay invisible until something tries to build
against it.

That is not hypothetical. Three entities — `iteration`, `read-model` and `phase` — were each written
into the record, used by several capabilities, and survived a storm, two walks and an architecture
before anything noticed they had never been declared.

## Declare it before you write records of it

Add the kind, say what it **is**, and give it the fields its records must carry. Then write the
records. The order matters: declaring afterwards means everything written in between went unchecked,
and you will not know what you missed.

## Do not guess the shape — let the record tell you

When declaring a kind that already has records in the tree, do not recall what those records usually
carry. Turn the check on and read what it refuses:

```console
$ praxis check praxis
praxis: 254 refusal(s) in praxis
```

Each refusal names a field the records use and the schema has not declared. Declaring them turns 254
into zero **without touching the engine**, which is the rule above, demonstrated.

## Say whether a field holds an entity or a value

A field that **holds** a contained entity must say so:

```kdl
field "phase" each="0..n" holds="phase"
```

Without `holds`, a contained entity is indistinguishable from a plain field, the container skips it,
and it goes unchecked. That is precisely how `phase` went missing.

`holds` nests; `references` points across the graph by id. They are not the same and a field is
rarely both.

## A kind with no fields is reported, not refused

Declaring a kind and giving it no shape is allowed — some kinds carry everything as properties, and
property shapes are not yet describable. But it is **reported on every run**, because a kind that
refuses nothing and a kind that was never checked must not look the same to a reader.

Leaving a kind shapeless is a decision. Make it deliberately, and say why on the declaration.

---

## What this skill refuses to do

- **It does not decide whether the kind should exist.** That is the architecture's argument.
- **It does not let you add the rule to the engine instead.** If the checker needs a rule the record
  does not state, the record is what changes.


## A field is read in whichever form you write it

**Both of these are the same fact, and both check:**

```kdl
read-model "x" { publishable #false }        // as a child node
read-model "x" publishable=#false            // as a property
```

**So are these:**

```kdl
owns-event "A"                               owns-event "A" "B"
owns-event "B"
```

`each=` counts **values**, not nodes. Two values is two, written either way, and a field with
no values at all is still missing — counting values did not turn cardinality into a formality.

### Why this is worth stating

Until `TS.260823.04` the schema declared a field and the **engine** decided what shape it took,
differently in different places. An author reading `praxis schema` could not tell which, and the
refusal did not say:

| written | said |
|---|---|
| `publishable #false` | *does not say whether it survives being frozen* — while it said so on the line above |
| one `owns-event` node per event | *appears 2 times; the schema requires exactly one* |
| four `followed=` properties | three of four approaches silently read as unfollowed |

All three are valid KDL. The validation review hit every one inside twenty minutes while
building a first record by **following this skill** — and the fastest way past them was to copy
an existing record and mutate it, which is transcription, the one move `adopt-the-method` spends
its longest section arguing against.

**A checker teaches.** Every refusal is doctrine delivered at the moment it is needed, and a
refusal that misdescribes the fault teaches the author to route around the checker — which
produces exactly the improvised structure this method exists to prevent.

`D1` on `ITER.260823.07` chose reading both forms over declaring a form per field: nothing in
the record needs to insist on a form, and a method already carrying 27 kinds and 158 field names
does not need vocabulary for a distinction nobody wants enforced.

### Counting values found nine wrong cardinalities

Turning it on refused the record immediately. `owns-event`, `keeps-consistent`, `attacks`,
`from-cluster`, `affected`, `baseline`, `deciders`, `held-by` and `exercised-by` were all
declared `each="1"` or `each="0..1"` while the record had **always** carried lists. Node-counting
made nine years of that invisible in a schema whose whole job is to say what a record may be.

## An identity prefix is declared on the kind

```kdl
entity "capability" id-prefix="CAP." { … }
```

`CAP.` used to be `trim_start_matches("CAP.")` in four engine call sites — a naming convention
only the engine knew, applied to every repository whether it used that prefix or not, and
explained to nobody when references stopped resolving. A repository may now choose its own, and
`praxis schema` prints it.

## Related

- `cut-a-slice` — the shape a slice must carry is declared the same way.
