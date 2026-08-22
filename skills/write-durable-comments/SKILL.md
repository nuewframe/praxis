---
name: write-durable-comments
description: >
  Decide what belongs in a comment and what belongs in the record. Teaches the what-versus-when
  test, where a changelog actually lives, the one case where history is legitimately a comment,
  and how to tell a citation from a story. Runs whenever you write or edit a comment, a config
  block, or a file header.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, replace_string_in_file]
---

# Skill: Write Durable Comments (`skills/write-durable-comments/`)

**Audience:** Anyone writing a comment. **When:** as you write it, and as you edit around one.

---

## The test

> **If the sentence stops being true after the next refactor, it is a changelog.
> If it stays true as long as the code does, it is a comment.**

A comment that says **when** something changed is invalidated by the next change — and nobody
re-reads it, so it decays into a confident false statement sitting next to working code.

A comment that says **what** a thing means is invalidated only by the thing changing, which is
the edit that makes you read it anyway. It cannot rot without you noticing.

## What it looks like when it goes wrong

```kdl
// Restored by TS.260821.10, after six days pointing at nothing. The schema had been
// carried on ADR.260819.02, which is withdrawn: an ADR is a TECHNICAL decision, and
// what a frame IS belongs to the record itself. The comment that stood here promised
// the binding would come back when the schema had a home that was not a decision.
governed-by "delivery-graph@v1"
```

Eight lines, all true, none of them telling you what `governed-by` **is**. Written in this
repository and caught within the hour (`ITER.260822.07/AR3`).

```kdl
// The method this repository is governed by. `praxis check` refuses a version the
// engine does not carry. `praxis schema` prints what it means.
governed-by "delivery-graph@v1"
```

Three lines: what it holds, what enforces it, where to look. Still true in a year.

---

## Where the changelog actually lives

Every fact in that first block already had a home:

| The fact | Its home |
|---|---|
| It was restored | the slice — `TS.260821.10` |
| It had been broken for six days | the finding — `AK2` |
| An ADR is the wrong carrier for a vocabulary | the decision that withdrew `ADR.260819.02` |
| Exactly what changed, and when | the commit |

**Prose that duplicates a home it already has is a second copy** — and the second copy is the one
that goes stale, because nothing checks it. That is the whole argument of `FRAME.260819.01`,
applied to comments.

---

## The one case where history IS the comment

When the history is *why the code looks wrong*. A reader about to "fix" something needs the
near-miss:

```rust
// Swapped as a parsed fragment, not as a value: a value set through the API is
// written as a bare identifier, so `state "open"` would become `state closed` —
// valid KDL, and a diff that looks like the file changed shape.
```

That is not a changelog. It is a statement about the present that happens to be *learned* from a
past mistake, and it stays true as long as the code does.

## Citation, not story

Naming a finding is fine. Retelling it is not.

- **Story** — *"AG1 happened because refusals about nested entities were computed and then
  silently dropped, and it was only caught because the answer had been predicted."*
- **Citation** — *"attribution must match kind AND id at any depth, or a correct rule reports
  nothing (`AG1`)."*

The citation states a rule about the code and points at where the reasoning is kept. The story
puts the reasoning in two places, and only one of them is checked.

---

## Placement

| Where | What belongs |
|---|---|
| **File header** | what this file is, what belongs in it, what deliberately does not |
| **Declaration** | what the field means; why a cardinality or a default is what it is |
| **Function** | the invariant it maintains, or the failure it prevents |
| **Inline** | only where a reader would otherwise change it back |

Never: *"Restored by…"*, *"used to be…"*, *"fixed in 0.8.0"*, *"changed from A to B"*, or a name
and a date.

---

## What this does not tell you

Whether a given sentence passes. `comments-state-meaning` is declared with **no probe**, and
`praxis check-invariants` reports it as unkept — accurately. Whether a sentence states meaning or
narrates change is a judgement, and a grep for *"used to"* would catch the honest cases and miss
every paraphrase.

This codebase does not fully comply. Several comments written the same week as this skill retell
findings they could have cited. That is worth knowing before you read them as exemplars.

---

## Related

- `declare-an-invariant` — `comments-state-meaning` is one, and it has no keeper yet
- `record-a-decision` — where the *why* goes, with the alternatives it rejected
- `close-an-iteration` — where a finding goes, so a comment can cite it instead of retelling it
