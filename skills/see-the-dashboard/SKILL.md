---
name: see-the-dashboard
description: >
  See where the product stands right now, composed on demand from several read models and never
  committed. Teaches why a dashboard is the clearest case the publication test rejects, and why a
  document may compose many views while each view keeps exactly one owner. Ask at any moment.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, run_in_terminal]
---

# Skill: See the Dashboard (`skills/see-the-dashboard/`)

**Audience:** Maintainer. **When:** any moment someone asks where the product actually is.

```
praxis dashboard [root]
```

---

## The rule

**A dashboard's entire value is being current, so it is never committed.**

A committed dashboard is a claim that starts decaying the moment it is written, and the reader who
finds it has no way to tell how far it has decayed. It is the clearest case the publication test
rejects, and it is rejected **by name**.

So this is composed when you ask and thrown away after. There is no file, no cache and no regeneration
step — ask twice across a change and you get two answers, because the only input is the record.

---

## One document, several views, one owner each

The dashboard puts three results side by side:

| Part | Owner |
| --- | --- |
| what is currently true | `CAP.delivery-record` |
| what is ready to pick up | `CAP.work-admission` |
| what the next version holds | `CAP.release-binding` |

A **document** composes several read models. A **read model** still has exactly one owner. Composition
is *ordering*, not interpretation — nothing here reaches across an ownership boundary, and a capability
appearing twice is refused rather than rendered.

If a composite view ever needed a cross-capability owner, the one-owner rule would be broken and the
capability boundaries would be drawn wrong.

---

## The same result, two lifetimes

The release part is the *same* read model that gets published into a release directory. Composed on
demand it carries **the moment**; published it carries **the version** and no clock at all.

That one result can do both is what makes `read-model@v1` a seam rather than a formatting convention.

---

## What its absence means

If the record declares no admission conditions, the readiness part is **left out** rather than computed
from an assumed gate — an empty gate is not an open one. You notice because the owner is missing from
the document, not because a section is empty.

---

## What this replaces

A committed product dashboard can be retired once this exists, without losing anything: the answer it
was approximating is now available whenever anyone asks. Retiring it *before* this exists would lose
the approximation and offer nothing in its place.

Related: `ask-what-is-true` · `see-what-is-ready` · `declare-a-lifetime`.
