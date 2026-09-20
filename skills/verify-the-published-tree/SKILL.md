---
name: verify-the-published-tree
description: >
  Prove every published document is byte-identical to what was published. Teaches why the comparison is
  against the commit a release names rather than the current record, why an unreadable commit fails
  instead of skipping, and why a check that fails continuously is the real risk. Runs on push, in CI.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, run_in_terminal]
---

# Skill: Verify the Published Tree (`skills/verify-the-published-tree/`)

**Audience:** Automation, Maintainer. **Phase:** on push — nobody invokes it.

```
praxis verify-published [root]
```

---

## The rule

**A published document is never edited. It is superseded.**

Each release's directory is compared, byte for byte, against **the commit its own index node names**.
If a document differs, the check fails and *names the document*.

---

## Compare against the claim, never against the record

This is the part that is easy to get wrong and expensive to get wrong.

An archival document is **supposed** to disagree with a record that has moved on. It depicts one
version and is false for every other — that is the entire property. So:

- **Not** against the current record. That fails the moment work resumes, continuously and *correctly*,
  which trains everyone to ignore the gate.
- **Not** by re-rendering. Rendering rules change as the method learns, so re-rendering a past release
  with today's logic reports drift that never occurred.

What it compares against is what git already holds at the commit the release names. Nothing else.

**C2 is the load-bearing claim**, and it is load-bearing for a reason that has nothing to do with
drift: *a check that fails continuously is a check everyone disables, and that failure mode is likelier
than the thing the check exists to catch.*

---

## Unreadable is not clean

A commit that cannot be read — shallow clone, unfetched history, missing tag — **fails**. It is never
skipped, and never treated as a pass.

A check that silently skips is how verification gets disabled without anyone deciding to disable it.
That is why CI checks out with `fetch-depth: 0`: full history is what makes the check *answerable*, not
what makes it lenient.

---

## What it proves, and what it does not

- **Proves:** a published document was not hand-edited after the fact.
- **Does not prove:** that the document still agrees with the record. It is not supposed to.

State the scope when you cite it. A check whose scope nobody can state is worse than a narrower one
that is honest about its edges.

---

## It does not fix anything

Drift is named and refused. Repairing it is a decision — usually the answer is that the edit belongs in
the *next* release's document, which is what a version line is for.

Related: `publish-the-release-set` · `cut-the-version`.
