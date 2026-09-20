---
name: declare-a-lifetime
description: >
  Decide whether a read model survives being frozen, and record why. Teaches the publication test and
  its two outcomes, why there is no repository-wide default, and why an unjustified declaration is
  refused in both directions. Runs when a view is named, in PLAN or DISCOVER.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, replace_string_in_file, run_in_terminal]
---

# Skill: Declare a Lifetime (`skills/declare-a-lifetime/`)

**Audience:** Maintainer. **When:** the moment a read model is named — not at publication time.

---

## The publication test

> **If a reader finds this two years from now, stamped with an old release, does it still do its job?**

Two outcomes, and both are decisions:

| Declaration | Means |
| --- | --- |
| `publishable=#true` | it is still useful **frozen** — it becomes history rather than decaying |
| `publishable=#false` | its value is **being current**; project it on demand and never commit it |

```kdl
read-model "the-published-set-for-a-release" needed-by="the-reader-of-a-release" \
    answers="what can you do now, and what is still missing?" \
    publishable=#true \
    because="it is the archival record by construction — a release note is useful only frozen"
```

---

## Both directions need a reason

`publish-only-what-survives-freezing` refuses a view with **no declaration**, and refuses a declaration
in **either** direction with no `because`.

Deciding *not* to publish something is as much a judgement about perishability as deciding to — and an
unjustified judgement is indistinguishable from an oversight. Six months later nobody can tell "we
thought about this" from "nobody got to it", and that is the whole failure this rule exists to prevent.

---

## There is no default

Not per repository, not per file type. **Perishability is a property of the question**, not of the
format it happens to be written in. A dashboard and a release note are both Markdown tables; one is
wrong the moment it is frozen and the other is only useful frozen.

The clearest case the test rejects is a dashboard. If you find yourself reaching for "documents are
publishable", you are deciding by file type.

---

## A publishable view says who it is for

`needed-by` names the persona. It is required of anything published, and
`a-published-view-names-its-reader` reports a view that names nobody.

This is not bookkeeping either, and it was ignored for longer than anything else in the record.
`needed-by` sat on every read model from the day the storm was written and **no projection ever
read it** — so a release published seven documents in alphabetical order, four of them about how
the tool is built, and a reader who wanted outcomes opened the architecture because it sorted
first.

A view that names nobody is still published, and goes out under **`addressed to nobody`**. Hiding
it would hide the fault with it.

---

## A release is one document, not a directory of them

A reader arrives with a reason. Seven siblings in alphabetical order ask them to already know the
answer in order to find it.

So a release is **one story**, and the story declares its own order:

```kdl
read-model "where-to-start" needed-by="the-reader-of-a-release" \
    answers="what is this, what can it do, and where do I begin?" \
    publishable=#true \
    because="it IS the release" \
    publishes-to="docs/releases/<version>/README.md" {
    part "1" is="why this exists"  from="what-this-product-means"
    part "2" is="what it can do"   from="capabilities-and-what-they-own"
    part "3" is="how to start"     from="how-to-use-a-capability"
    part "4" is="what changed"     from="the-published-set-for-a-release"
}
```

- **A view a story carries needs no `publishes-to`.** It lands where the story lands, and that is
  the only case where a publishable view may omit one.
- **The index quotes each part's own `answers`, verbatim.** A publisher that rewords its source is
  a second author and a reader cannot tell which of the two they are reading.
- **A part naming a view the engine cannot compose refuses the publish.** A story with a missing
  chapter is worse than none: it reads as complete.
- **The glossary carries the words the story uses**, in the order a reader learns them — a
  container before what it contains, and a kind before the kind that references it. The full
  vocabulary is `praxis schema --print`, which is a command rather than a release artifact.

---

## Not published is not unanswerable

Flipping a view to `publishable=#false` takes it out of the release directory and nothing else. It
is still declared, still composed from the record, and still asked for:

```
praxis view the-decisions-that-shaped-this
```

Praxis did this to three of its own views. The decisions, the architecture and the shipped-doctrine
inventory are **engineering** — a reader of a release did not come for them, and frozen at a version
they are the wrong answer anyway: how the tool is built *now* is what anybody asking that wants.

Record the flip with `matured "publishable"`, naming what taught it. A lifetime that changed and
does not say why reads as one that was always this way.

---

## A publishable view says where it lands

`publishes-to` is required of anything a story does not carry, and `<version>` is substituted at
publish time. A path ending in `/` is a directory; the document inside is named for the view.

This is not bookkeeping. **The published set is derived from these declarations and from nothing
else** — so an engine that composes its own paths has quietly taken the decision back from the record.
That is exactly what happened here before this slice, and the record had been carrying the right
answer the whole time.

---

## The set must equal the declarations

A view declared publishable that nothing can compose **refuses the whole publish**. Not a shorter set,
not a warning — a set that quietly omits one of its members is a hand-maintained list with extra steps,
which is the thing being replaced.

Related: `publish-the-release-set` · `name-a-persona` (who a view is for) · `mature-a-value` (recording a lifetime that changed) · `cut-a-slice`.
