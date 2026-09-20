---
name: document-usage
description: >
  Write how a capability is used while you are building it, attached to the capability record. Teaches
  why usage is written inside the iteration rather than at release time, and why a guide for behaviour
  that never shipped is refused. Runs during ITERATE.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, replace_string_in_file, run_in_terminal]
---

# Skill: Document Usage (`skills/document-usage/`)

**Audience:** Working agent. **Phase:** `ITERATE`, while the surface is being built.

```kdl
capability "work-admission" {
    usage "`praxis ready` — which slices could be started right now, and what would refuse \
        each of the rest. Read the `not decided for you` section first."
}
```

```
praxis guide <CAPABILITY> <VERSION>
```

---

## Write it while you build it

Not at release time. Release time is when usage is **least accurate and most rushed** — written from
memory of what the work meant, by whoever is publishing that day, under time pressure. That is the
chore-beside-the-work pattern this frame exists to remove, and it is the first thing dropped when the
work runs late.

---

## It lives in the record, not in a file the record points at

A pointer is a second thing to keep in step, and it is the one that rots. The prose is **data on the
capability**, which is why the guide can be composed with no filesystem read at all.

---

## A guide for what never shipped is refused

`guide-follows-promoted-truth`: a capability's usage enters version N's guide only if N **promoted that
capability's truth**.

A guide describing behaviour that never shipped is **worse than no guide** — it is a document that is
confidently wrong, and a reader pinned to that version has no way to tell. So asking for one is
refused, by name, with the reason.

---

## Three outcomes, and two of them are gaps

| Section | Means |
| --- | --- |
| how to use what shipped | promoted, and documented |
| shipped without a guide | promoted, and nobody wrote it — **reported, never omitted** |
| written, not yet shipped | documented ahead of the release that will carry it |

The third one matters more than it looks. Prose written before its truth ships is not *wrong* — it is
**early**. Naming it as timing rather than dropping it is what stops the next person writing it again.

And a silently missing guide is indistinguishable from one nobody needed, which is the same shape as
every other reported gap here.

---

## What this does not do

- **It does not judge the prose.** Absence, and shipping-status. Never quality.
- **It does not render.** The guide is prose in the record until document-projection composes it.

Related: `promote-what-shipped` · `declare-a-lifetime` · `name-a-capability`.
