---
name: record-a-decision
description: >
  Record a choice an iteration could not make implicitly, bound to the work that forced it, with the
  alternatives it rejected and what would show it wrong. Teaches the falsifier requirement and
  append-only correction. Runs during ITERATE, at the moment of choosing.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, replace_string_in_file, run_in_terminal]
---

# Skill: Record a Decision (`skills/record-a-decision/`)

**Audience:** Working agent. **Phase:** `ITERATE`, at the moment the choice is made — not at close.

```kdl
decision "the machine gate implements half of the doctrine, and says so" \
    chose="`pick-up` writes an iteration in state `open`, carrying the create vet only" \
    over="writing it `working` with both vets recorded at the same moment" \
    because="the record says an iteration is vetted twice BECAUSE the tree moves in between" \
    falsified-by="a `start` command landing and the second vet turning out to add nothing" \
    state="accepted"
```

---

## Four things, and two of them are usually missing

| | Why it is refused without it |
| --- | --- |
| `chose` | what was decided |
| `over` | **a decision with no alternatives is a preference** |
| `because` | the argument, which is what a reader is actually looking for |
| `falsified-by` | **a decision nothing could falsify was never a choice between real options** |

A preference recorded as a decision is the hardest kind to argue with later, because there is nothing
to argue *against*. When this frame turned the rule on, it refused **every decision it had ever
recorded** — six, across four iterations, none of them falsifiable.

---

## Write the falsifier as an observation, not a caveat

Not *"this might be wrong if circumstances change."* Name **the thing that would happen**:

- *"a derive-based codec that can validate against a schema loaded at runtime"*
- *"a `start` command landing and the second vet never catching a tree that moved"*
- *"a reader who needs the stored state because deriving it is too slow"*

If you cannot write one, you have not made a decision yet — you have expressed a preference, and the
honest move is to say so or to keep arguing.

---

## Bound to the iteration, not filed beside it

A decision nests inside the iteration that **forced** it. A decisions folder loses the one thing that
makes a decision readable two years on: what was happening that made it necessary.

---

## Corrected only by appending

Once `accepted`, the body is sealed. Correct it with an `amendment` child naming what changed and why —
the original stays readable, and `an-accepted-decision-is-append-only` refuses a rewrite.

The seal covers the body and **deliberately not the amendments**, so appending is not an edit. As with
every seal here: it makes a rewrite *visible*, it does not prevent one.

---

## What tested it

A finding that tested a decision says so with `tests="<the decision's title>"`. That is what makes the
findings **reachable from the decision** in the published document — otherwise the record holds the
evidence and the claim in two places that never meet.

Related: `close-an-iteration` · `declare-a-lifetime` · `review-in-flight`.
