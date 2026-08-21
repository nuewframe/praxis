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
read-model "the-published-set-for-a-release" \
    publishable=#true \
    because="it is the archival record by construction — a release note is useful only frozen" \
    publishes-to="docs/releases/<version>/release-notes.md"
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

## A publishable view says where it lands

`publishes-to` is required, and `<version>` is substituted at publish time. A path ending in `/` is a
directory; the document inside is named for the view.

This is not bookkeeping. **The published set is derived from these declarations and from nothing
else** — so an engine that composes its own paths has quietly taken the decision back from the record.
That is exactly what happened here before this slice, and the record had been carrying the right
answer the whole time.

---

## The set must equal the declarations

A view declared publishable that nothing can compose **refuses the whole publish**. Not a shorter set,
not a warning — a set that quietly omits one of its members is a hand-maintained list with extra steps,
which is the thing being replaced.

Related: `publish-the-release-set` · `name-a-capability` · `cut-a-slice`.
