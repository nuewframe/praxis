---
name: review-in-flight
description: >
  Preview what an iteration has promised and what it has actually shown, at any moment, writing nothing.
  Teaches reviewing in flight rather than at a checkpoint, reading the gap between promised and shown,
  and why a preview is never mistakable for a record. Runs during ITERATE.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, run_in_terminal]
---

# Skill: Review In Flight (`skills/review-in-flight/`)

**Audience:** Reviewer, Maintainer. **Phase:** `ITERATE`, at any moment.

```
praxis review <ITER-ID> [--markdown]
```

---

## The rule

**Review the gap, not the artifact.**

`S2` is a reviewer judging work by how *finished it looks*. What makes that possible is that "what was
promised" and "what was shown" arrive separately and get reconciled in the reviewer's head — where the
polish of the artifact does the reconciling.

This puts them in one answer, side by side, with the gaps named:

| Section | What to look for |
| --- | --- |
| what it promised | every claim, its state, and **what has actually been shown for it** |
| what it has reached | every layer **the slice declared** — including the ones nothing has reached |
| how far it has got | the phases, and what each produced |
| what is still open | findings, and which claim each carries |

---

## The second section is the one that lies least

Layers come from **the slice**, not from the iteration. So a layer the slice declared and the iteration
has not touched appears as `unevidenced` — it does not quietly vanish from the table.

A layer that vanishes was never reached *and* never refused, and afterwards those two look identical.
A layer the iteration claims that the slice never declared shows up too, marked `NOT DECLARED`.

---

## Ask whenever. Review cadence is not release cadence

There is no checkpoint to wait for and no gate here — **reading is not approving**. Ask in the middle
of the work, twice in an hour, on someone else's iteration. The answer reflects the record at the
moment you asked, and asking again gives a different answer when the work has moved.

---

## A preview is not a record

The header says so before anything a reader might quote. Structurally:

- the composing function returns a result with **no file path in it**, so a preview has nowhere in the
  published tree to land;
- `--markdown` writes only to the working projection path, which is **gitignored** — a preview cannot
  be committed by accident;
- it is never `publishable`, because its value is being current, and a committed preview is a record of
  a version that does not exist.

Discard it after reading. Nothing is lost — ask again.

Related: `attach-evidence` · `close-an-iteration` · `ask-what-is-true`.
