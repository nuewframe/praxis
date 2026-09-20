---
name: ask-what-is-true
description: >
  Ask the record what this repository already knows before reconstructing it. Teaches arriving agents to
  ask rather than re-derive, how to read what the answer says it does NOT cover, and why this view is
  computed on every ask and never committed. Run it first, in any phase.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, run_in_terminal]
---

# Skill: Ask What Is True (`skills/ask-what-is-true/`)

**Audience:** Any arriving agent. **Phase:** first, before anything else.

```
praxis truth [root]
```

---

## The rule

**Ask before deriving.**

You arrive with none of the previous agent's context. The expensive mistake is not getting something
wrong — it is spending the first half of a session reconstructing, from five documents, something the
record can answer in one question. That is `S7`, and it is the symptom that costs most and is noticed
least, because re-deriving *feels* like work.

The answer covers: capabilities and what each owns, releases and what they bind, which slices are
delivered and which are outstanding, and what closed iterations left owed.

---

## Read the last section

**`not covered by this answer`** names the questions this view does *not* answer and what to ask
instead. A view that does not state its edges is one you will over-trust — the difference between an
answer and an impression.

It does not tell you what is in flight, what could be started, or why anything was decided. Those are
other questions with other answers, and they are named so you do not mistake this answer's silence for
the record's.

---

## It is computed, never stored

There is no cached summary and no regeneration step. Change the record and the next ask reflects it —
because the only input is the record. If you ever find a written file claiming to be this view, it is a
bug, and it is the exact bug the whole frame exists to attack.

It stamps the **moment** it was computed, which is the opposite of a published document — those name
the version they depict and carry no clock. Same result type, opposite requirement, and it is
`publishable` that decides which.

---

## `nothing recorded` is an answer

An empty area says *"nothing recorded"* and why. It never returns an empty space.

A reader who cannot tell "there are no releases" from "releases were not computed" is reading the
trust-transfer problem at table granularity — which is the same reason a refusal is a file and an
uncomputed condition is named.

Related: `see-what-is-ready` · `pick-up-a-slice`.
