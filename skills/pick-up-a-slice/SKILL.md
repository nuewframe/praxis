---
name: pick-up-a-slice
description: >
  Take a slice through the one gate. Teaches that selection IS the commitment, what the gate evaluates
  and what it refuses to decide for you, how a refusal is recorded as a fact, and what to do when the
  gate refuses work you believe should proceed. Runs at the start of ITERATE.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, run_in_terminal]
---

# Skill: Pick Up a Slice (`skills/pick-up-a-slice/`)

**Audience:** Working agent, Maintainer. **Phase:** `ITERATE`. Run `see-what-is-ready` first.

```
praxis pick-up <SLICE-ID>... --asked-by <IDENTITY> [--root <ROOT>] [--dry-run]
```

---

## The rule

**Selection is the commitment.** Choosing to work this is choosing not to work something else, and it
is the first moment the information needed to decide exists. There is exactly **one gate** in this
method and this is it — nothing later asks permission again.

Two outcomes, never both and never neither:

- **admitted** — an iteration record is written in state `open`, carrying the admission, who asked
  for it, and every condition's verdict;
- **refused** — a `REF.` record is written naming the conditions that failed, and **nothing opens**.

---

## Why the refusal is written down

A gate that was never invoked and a gate that refused **look identical afterwards**. That is `S3`, and
it is the reason a refusal is a file rather than a message on a terminal that scrolls away.

So the refusal is a record: checked like any other, kept like any other, never deleted. If you want to
know whether the gate ran, you look.

---

## Read what it did not decide

Both records — admission and refusal — carry the conditions the gate **did not decide**:

- `verdict="not-computed"` — declared, and no check answers it yet. Not a pass.
- `verdict="left-to-the-maintainer"` — yours. The tool shows; it does not rank.

An admission that lists only what passed claims more than it checked. Read those lines before you
start, because they are the part nobody checked for you.

---

## When the gate refuses and you disagree

This happens, and there is a right way through it. In order:

1. **Read the condition.** It names the specific thing — which dependency, which iteration holds it.
   Most refusals are correct and the answer is to go work that instead.
2. **Ask whether the block is real or an accounting artefact.** A dependency can be genuinely
   satisfied while the record has not caught up. If so, **fix the accounting** — that is a finding and
   usually a real defect.
3. **If it is real and you proceed anyway**, a human is named as having asked and **the refusal stays
   standing.** Record both: the `REF.` file, and a create-vet condition naming it with the reason.
   A reader must be able to see that the gate ran, what it said, and who decided otherwise.

**Never** weaken a condition to make one refusal go away. Every future refusal gets less trustworthy
to save you one argument, and the conditions are the only reason anyone believes the gate at all.

---

## What it will not do

- **It does not judge the work.** Preconditions at open, accounting at close, the work itself at
  neither.
- **It does not queue.** A refusal is not a promise to admit later; ask again when the condition changes.
- **It does not start the work.** The iteration opens in state `open`. The tree moves between the ask
  to create and the ask to start, which is the whole reason there are two vets — so starting is a
  second ask, and a gate that recorded both at one moment would be recording a check that did not
  happen.
- **It never overwrites a record.** Composed in memory, written to a staging name, moved into place.
  Whole or nothing.

---

## Who asked

**`--asked-by` is required and has no default. Name whoever asked.**

The ask is the permission. Somebody said "pick this up" and that is the authority — this method does
not add a second confirmation, and a prompt inside the tool would only prove the tool ran twice.

What it will not do is decide **whose** ask it was. Until `TS.260823.01` the identity came from
`git config user.email`, stripped and prefixed `human:` — which reports whose machine the command ran
on. An agent working in the maintainer's shell therefore produced a signed human approval nobody
typed, in the one gate every other rule in this method assumes held.

`close --attested-by` had made this argument for two versions already:

> *An identity the tool supplied would prove the tool ran, which nobody doubted.*

The same sentence is true of the admission, and both ends of an iteration now say so.

**The word is `asked`, not `signed`.** Nothing here is signed, and the schema never claimed it was —
its reason for the approval field has always read *"the human's ask is the only gate, and it is
recorded or it did not happen."* The engine wrote `signed` over it for nineteen iterations, so a
reader asking *who decided this* got a word promising a witness nobody produced.

| Refused | Why |
|---|---|
| no `--asked-by` | an admission on nobody's word |
| `--asked-by ""` | the same, said longer |
| `--asked-by agent:anything` | an agent admitting its own work is this frame's root cause with the method's name on it |

A bare name is read as a human. Requiring the `human:` prefix would teach people to type it as a
formality, and a formality is exactly what the git identity had become.

**What this does not claim.** The record holds who was *named*. It cannot hold whether they asked,
agreed, or read the slice — and a mechanism pretending otherwise would repeat, one level up, the
fault this replaced.

Related: `see-what-is-ready` · `cut-a-slice` · `declare-an-entity-kind`.
