---
name: resolve-a-symptom
description: >
  Close a symptom by naming a release that demonstrably attacked it. Teaches that a frame shrinks only
  by a release, why resolution is computed from what shipped rather than asserted, and what to do when
  the check refuses a claim you believe. Runs in RELEASE, after the cut.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, replace_string_in_file, run_in_terminal]
---

# Skill: Resolve a Symptom (`skills/resolve-a-symptom/`)

**Audience:** Maintainer. **Phase:** `RELEASE`, after the cut.

```kdl
symptom "S3" state="resolved" resolved-by="0.9.0" \
    text="how much process a unit of work received is declared by the agent that received it"
```

---

## The rule

**A frame shrinks by a release, never by an opinion.**

`resolution-names-a-release` accepts a resolution only when both are true:

1. the record holds a **cut** release at that version — binding work to it is not shipping it;
2. that release **bound a slice whose `attacks` names this symptom**.

Resolution is therefore *computed from what shipped* rather than asserted by whoever is closing it.
That is the frame's own principle applied to the frame's own bookkeeping — and a method that exempts
its own progress reports from the standard it sets is the root cause with a nicer surface.

`partially-resolved` is held to the same standard. **It is a state, not a hedge.**

---

## When it refuses something you believe

This is the check most likely to contradict a maintainer, and that is the point. Work through it in
this order:

1. **Did a slice in that release actually attack the symptom?** If the work happened but no slice
   says `attacks`, the gap is in the slice, and amending it is the fix.
2. **Is the release cut?** A planned release has shipped nothing.
3. **Is the claim older than the record?** Then it is not *false* — it is **uncomputable**, which is a
   different thing and needs a different answer.

For the third case, **withdraw rather than grandfather**:

```kdl
symptom "S1" state="present" \
    previously-claimed="partially-resolved by 0.5.0" \
    withdrawn-because="0.5.0 predates the delivery graph: no cut release at that version, and \
        therefore no slice whose `attacks` names S1"
```

This repository had to do exactly that to its own S1. The narrowing may well have happened — the
problem is that nobody can check, and this frame's entire argument is that **an assertion nobody can
check is not evidence.** The first thing self-application cost it was a claimed win.

---

## What this does not do

- **It does not close the frame.** A frame has no terminal state; its symptoms resolve individually.
- **It does not judge whether the fix was good.** It judges whether something shipped that was aimed at
  the symptom.

Related: `cut-the-version` · `bind-work-to-a-version` · `promote-what-shipped`.
