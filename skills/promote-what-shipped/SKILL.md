---
name: promote-what-shipped
description: >
  Fold what a cut release shipped into what each capability says it is, in one operation. Teaches why
  promoted truth is derived rather than written, why a hand-edit is recomputable rather than merely
  discouraged, and why promotion is part of releasing and not a follow-up task. Runs in RELEASE.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, run_in_terminal]
---

# Skill: Promote What Shipped (`skills/promote-what-shipped/`)

**Audience:** Maintainer. **Phase:** `RELEASE`, immediately after the cut.

```
praxis promote <VERSION> [--dry-run]
```

---

## The rule

**Nothing promoted is hand-written.**

What moves is *derived*: cut release → bound iterations → the slice each was on → the capability that
slice realizes. A capability with shipped work leaves `sought` and becomes `active`, and gains a
`shipped` line naming the version, the iteration and the slice.

If it did not come from bound work, **it is not promotion — it is an assertion about the past**, which
is this frame's root cause with a version number attached.

---

## Why a hand-edit is caught rather than forbidden

`promoted-truth-is-derived` **recomputes** every capability's promoted block and refuses any that
disagrees. The command and the check call the same derivation, so there is no second place for them to
disagree from.

This is the answer to the cache problem: *a derived field written once and never rechecked is a cache;
one recomputed by a rule is a projection with a proof attached.*

---

## The rule runs in both directions

A capability that moved **further than the record derives** is refused. So is one that **should have
moved and did not**.

That second half has a consequence worth knowing before it surprises you: **after a cut, `praxis check`
fails until promotion runs.** Promotion stops being a thing you might remember and becomes a step the
record will not let you skip — which is the point, because the frame's own argument is that the chore
beside the work is the chore that gets dropped when the work is late.

---

## Atomic, in two layers

1. The **whole change set is computed before anything is written**. There is no shape in which half of
   it exists.
2. Every original is held, and any write failure **restores every file already written**.

A half-promoted record is worse than an unpromoted one: it is wrong in a way nobody can see. And
because promotion writes the whole derivation rather than a delta, running it again after an
interruption *repairs* the record instead of compounding the damage.

---

## Promoting twice does nothing

Not an error, not a double application — a no-op that says so. Idempotence here is not a nicety; it is
what makes "run it again" the safe response to any doubt about whether it ran.

Related: `cut-the-version` · `bind-work-to-a-version` · `publish-the-release-set`.
