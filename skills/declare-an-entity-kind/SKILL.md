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

## Related

- `cut-a-slice` — the shape a slice must carry is declared the same way.
