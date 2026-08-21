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
praxis pick-up <SLICE-ID> [root] [--dry-run]
```

---

## The rule

**Selection is the commitment.** Choosing to work this is choosing not to work something else, and it
is the first moment the information needed to decide exists. There is exactly **one gate** in this
method and this is it — nothing later asks permission again.

Two outcomes, never both and never neither:

- **admitted** — an iteration record is written in state `open`, carrying the signed admission and
  every condition's verdict;
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
3. **If it is real and you proceed anyway**, a human signs the admission and **the refusal stays
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

## Who signs

The tree's own git identity, as `human:<name>`. **An agent cannot sign an admission** — not because a
rule refuses it, but because there is nowhere in the command to put an agent's name. An agent-signed
approval is the trust-transfer problem expressed as a signature.

Related: `see-what-is-ready` · `cut-a-slice` · `declare-an-entity-kind`.
