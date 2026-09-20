---
name: cut-the-version
description: >
  Cut a planned version and write the index that points into git, in one operation. Teaches why the
  index points rather than copies, why the record proposes a bump and a human confirms it, and what the
  seal on a cut release does and does not claim. Runs in RELEASE, after binding.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, run_in_terminal]
---

# Skill: Cut the Version (`skills/cut-the-version/`)

**Audience:** Maintainer. **Phase:** `RELEASE`, after everything intended for it is bound.

```
praxis cut-release <VERSION> --confirm
```

---

## The rule

**A release without an index is a release nothing can be verified against.**

The cut and the index node are **one operation** — version, tag, commit, what it binds, what it
resolves. Not because a two-step version would usually work, but because a release that exists with no
index is a point on the version line nothing downstream can pin to, and publishing, verification and
symptom resolution all pin to it.

Atomicity here is **by construction, not by cleanup**: the whole record is composed before anything is
written. There is no half-cut state to recover from, because there were never two operations.

---

## The index points; it never copies

The index node names a **commit**. It does not carry the diff, the message, the file list, or anything
else git already holds. A second copy of history whose fidelity cannot be checked without consulting
the first is the problem this whole frame exists to attack — putting one inside the release record
would be the neatest possible way to lose the argument.

---

## `--confirm` is not a formality

The record **proposes** a bump from the configured rules and the bound work's `contributes`. Choosing
the version is the maintainer's. Without `--confirm` the cut is refused and says what it proposed, so
the confirmation is a decision someone made rather than a flag someone always passes.

Refused as well: a version binding nothing, a version already cut, and any bound iteration that is not
closed — named individually.

---

## What the seal does, and what it does not

A cut release carries a `seal` computed over its version, commit, bindings and resolutions.
`cut-release-is-immutable` recomputes it and refuses a mismatch.

**It makes an edit visible. It is not a signature.** Anyone who can edit the record can run the same
function and recompute it. What it catches is the edit nobody meant to make and nobody would otherwise
notice — which is the failure mode this frame is actually about. Do not describe it as tamper-proof;
describe it as tamper-evident, and only against accident.

It is deliberately order-independent over what is bound, so that a rewrite which reorders bindings does
not read as tampering.

---

## After the cut

The release is immutable. Nothing rebinds to it, `--undo` is refused, and the index node is never
edited. If something was left out, it belongs to the next version — which is what a version line is
for.

Related: `bind-work-to-a-version` · `close-an-iteration`.
