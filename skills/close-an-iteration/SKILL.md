---
name: close-an-iteration
description: >
  Close an iteration without dropping scope in silence. Teaches that a shortfall is recorded rather than
  quietly deleted, why a finding must NAME the claim it carries, and why deleting a claim to make a close
  succeed is itself refused. Runs at the end of ITERATE.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, replace_string_in_file, run_in_terminal]
---

# Skill: Close an Iteration (`skills/close-an-iteration/`)

**Audience:** Working agent. **Phase:** `ITERATE`, at the end.

```
praxis close <ITER-ID> [--outcome continue|pivot|stop]
```

---

## The rule

**Every claim is settled, or every shortfall is carried forward — or it does not close.**

An unmet claim that closes in silence and a met claim **look identical afterwards**. That is `S5`, the
symptom with the most day-to-day cost, and the refusal is the only thing that makes the two different.

Two ways for a claim to be accounted for, and only two:

```kdl
claim "C1" from-slice="TS.x" state="met" evidence="tests/…"
claim "C2" from-slice="TS.x" state="carried"
finding "F3" carries="C2" text="what was not reached, and where it is owed"
```

A finding that carries a claim must say so with `carries=`. **Prose is not accounting.** A finding that
explains the shortfall beautifully but never names the claim leaves the record unable to tell it apart
from a claim nobody thought about.

---

## What is not being judged

A recorded finding is **enough** to close. The gate refuses *silence*, never *shortfall*. Whether the
shortfall is acceptable is LEARN's question, and it is asked somewhere else by someone else.

So: do not argue with the refusal by improving the work. Record what happened.

---

## Why deleting a claim does not help

The claims are **frozen into the iteration when the gate opens it**. Removing one from the slice
afterwards does not remove it from the iteration — it makes the two disagree, and the disagreement is
refused by name.

This is the check that matters. Every other rule here is defeated by editing the claim list, which is
exactly how scope goes quietly in a Markdown checklist today. If the claim genuinely should not have
been cut, that is an amendment to the slice with a reason recorded, not a deletion during the close.

---

## What the machine writes, and what it does not

On acceptance the command writes `state`, `closed-at`, `outcome`, a computed accounting block, and a
trail entry. Every value there was **computed**.

It does **not** write why the iteration reached the conclusion it did, what the findings mean, or
whether to continue. Those are yours. The machine states what it checked; a machine-written judgement
would be the agent's own account of its work wearing the record's clothes.

---

## Closing is not reopening

A closed iteration is history. A later attempt at the same slice is a **new iteration** — which is
normal, and the record expects it. Nothing reopens.

Related: `attach-evidence` · `pick-up-a-slice` · `see-what-is-ready`.
