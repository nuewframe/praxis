---
name: name-a-persona
description: >
  Name who the product is for, before anything is framed. Teaches why one persona is enough to
  start, why the rest emerge from the work and must cite what surfaced them, and why `judges-by`
  is the field that turns a description into a claim. Runs at the start of discovery, and again
  whenever the work meets somebody nobody had named.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, replace_string_in_file, run_in_terminal]
---

# Skill: Name a Persona (`skills/name-a-persona/`)

**Audience:** Whoever starts a product, and whoever finds a new reader mid-frame.
**When:** before the first frame, and whenever the work meets somebody unnamed.

---

## Why this is first

Discovery has three activities — storm, architecture, walkthrough — and **every one is about
the system.** The record can say what a capability owns, what a slice claims and what a rule
refuses, and until this kind existed it could not say who any of it was for.

The consequence is specific. `useful-alone` is the field that says *what you get if this ships
and nothing after it does*. **It has always had an unstated subject.** So has `needed-by`,
which named a role — `reader`, `adopter`, `maintainer` — with nothing behind it.

## One is enough

```kdl
persona "the-team-building-a-digital-product" {
    is "engineers, designers and product managers working through an LLM coding agent"
    came-for "delivery they can trust without re-reading everything the agent produced"
    does-not-care "how the engine is built, or why any of it is in KDL"
    judges-by "they can pick up work somebody else left, and act on the record without asking them"
    state "primary"
}
```

**The primary persona is known before framing** — it is the reason discovery is happening at
all. You do not need the others yet, and a record naming none is *reported*, never refused.

Enumerating personas upfront is a week spent on people nobody has met.

## The rest emerge, and cite what found them

```kdl
persona "the-adopter" {
    is "somebody deciding whether to put this method into a repository that is not ours"
    came-for "what it does and what it will refuse them, before installing anything"
    does-not-care "this repository's own frame, or how the method was arrived at"
    judges-by "they can run it against their own repository and read the refusals without a guide"
    state "emergent"
    found-by "WALK.260822.01"
}
```

`found-by` is **required when emergent**. Discovered means discovered *by* something — a
walkthrough, a finding, a slice. A persona added later with no origin is indistinguishable
from one somebody invented to justify a decision already made.

That is the same discipline a strategy revision is held to, for the same reason.

---

## The four fields, and which one does the work

| Field | What it must contain |
|---|---|
| `is` | who they are, in a sentence |
| `came-for` | the outcome they arrived wanting — **not** what the product does |
| `does-not-care` | what they will not read. The half a document gets wrong by *including* |
| `judges-by` | how they would know they got it |

**`judges-by` is the one that matters.** A persona nobody can be wrong about is a paragraph.
Without it you cannot say a document served them, and "we improved it" stays an assertion.

`does-not-care` is the field people skip and shouldn't: it is why two personas need two
documents rather than one longer one.

## Not a role

`role` (`TS.260821.08`) is **what you may attest** — it gates whether `praxis close` accepts
you. A persona is **what you came for**. One person occupies several roles in a week and
remains one persona.

---

## What this does not give you

That the personas are **right**. It makes them checkable and contradictable, which is the most
a record can do. A wrong persona stated beats a right one assumed — the wrong one can be argued
with.

## Related

- `cut-a-slice` — `useful-to` names whose value a slice claims
- `declare-a-lifetime` — `needed-by` now names a persona, so a view for nobody dangles
- `record-a-decision` — where a disagreement about who this is for gets settled
