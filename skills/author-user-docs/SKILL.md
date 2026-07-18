---
name: author-user-docs
description: >
  Author or refresh user-facing documentation (the TEACH phase) from the durable capability
  record. Product-designer-owned. Turns validated architecture truth into Diátaxis-structured
  guides — tutorials, how-tos, reference, explanation — homed in docs/guides/. Capability guides
  are sourced from one capability record; journey tutorials span capabilities and are sourced
  from wave product-design journeys.
user-invocable: true
disable-model-invocation: false
---

# Skill: Author User Docs (TEACH)

Use this skill to turn a validated capability into user-facing teaching. This is the **TEACH** phase of the build-measure-learn loop — the phase that consumes the durable architecture record:

```
PLAN → BUILD → MEASURE → LEARN → TEACH
wave    slices   ship+obs  capability   user guides
(bet)                      record       (docs/guides/)
                           (truth) ─feeds→ this skill
```

**Owner:** Product Designer (authoritative voice of the user). The Principal Engineer reviews technical accuracy; they do not author.

**Teach validated behavior, never hypotheses.** Only author or refresh a guide once the behavior it describes has shipped and the capability record reflects it. If the record is still a bet, stop — there is nothing validated to teach yet.

---

## Project Conventions

This skill assumes the host project defines:

- A **capability record** at `docs/architecture/<capability>/` (the technical truth; see `design-system-architecture`).
- A **guides home** at `docs/guides/` (commonly configured as `paths.guides`).
- Wave `product-design.md` (user journeys and voice; see `create-product-design-spec`).

If those don't exist, the upstream artifacts are not ready — return to LEARN before teaching.

---

## The two altitudes

Cross-capability teaching has no per-capability home, so guides split the same way the architecture record does:

| Altitude          | What it teaches                             | Source                                                                  | Home                        |
| ----------------- | ------------------------------------------- | ----------------------------------------------------------------------- | --------------------------- |
| Capability guide  | Concepts + how-tos for one capability       | `docs/architecture/<capability>/` + wave `product-design.md`            | `docs/guides/<capability>/` |
| Journey tutorial  | End-to-end walkthrough spanning capabilities | wave `product-design.md` journeys + multiple capability records + system overview | `docs/guides/tutorials/`    |

---

## The genre boundary — record is source, guide is rendering

The capability record is **engineer-facing source**: how the capability is built and what it observably does. The guide is **user-facing rendering**: the same truth expressed as a task or concept in the user's language.

- The guide **derives from and links to** the record. It never restates mechanics (schemas, resilience tables, topology).
- When the record drifts, the guide is **re-derived**, not annotated with history.
- Never teach from sprint notes or a wave `product-architecture.md` (that is the bet, not the truth).

---

## Diátaxis — the four quadrants

Every guide is organized by the four Diátaxis modes. Keep them separate; do not blend a tutorial into a reference.

| Quadrant        | Serves        | Answers                          | Voice                     |
| --------------- | ------------- | -------------------------------- | ------------------------- |
| **Tutorial**    | Learning      | "Take me from zero to a win"     | Guided, concrete, safe    |
| **How-to**      | A task        | "How do I accomplish X?"         | Goal-oriented, terse      |
| **Reference**   | Information   | "What exactly are the inputs/outputs/states?" | Dry, complete, accurate |
| **Explanation** | Understanding | "Why does it work this way?"     | Discursive, context-rich  |

**Reference** is curated by hand from the capability record's contracts and seams for now. Auto-generation from seam contracts is a future capability — do not hand-copy a contract you cannot keep in sync; link to it.

---

## Steps

### Step 1 — Confirm there is validated behavior to teach

Read the capability record and the wave README. Confirm the behavior shipped and the record reflects current truth. If the change was a pure refactor with no user-observable effect, **stop** — there is nothing to teach.

### Step 2 — Choose the altitude

- One capability, its concepts and tasks → **capability guide** in `docs/guides/<capability>/`.
- A flow that crosses capabilities (e.g. sign up → browse → order → pay) → **journey tutorial** in `docs/guides/tutorials/`.

A change often produces both: refresh the capability guide, then check whether any tutorial that traverses this capability needs updating.

### Step 3 — Author or refresh, by quadrant

Create or update the guide using the structure below. Write in clean present tense as the current correct experience — no "changed in", no sprint or date history in the body.

```markdown
---
title: <User-facing title>
capability: <capability>            # omit for a cross-capability tutorial
source: docs/architecture/<capability>/   # the record this guide is derived from
last-validated: YYYY-MM-DD          # date the behavior was last confirmed shipped
---

# <Title>

## Concepts (Explanation)
<Why this exists, the mental model, the key terms — in user language.>

## Get started (Tutorial)
<A guided, end-to-end path to a first success. Concrete, safe, reproducible.>

## How-to
### How to <task>
<Terse, goal-oriented steps.>

## Reference
<Inputs, outputs, states, limits — curated from the capability record's contracts.
Link to the machine-readable contract; do not hand-copy it.>
```

For a journey tutorial, lead with the Tutorial quadrant end-to-end across capabilities, and link out to each capability guide for depth.

### Step 4 — Verify accuracy against the source

Re-read the capability record and confirm every claim in the guide matches current truth. Hand to the Principal Engineer for a technical-accuracy pass. Update `last-validated`.

### Step 5 — Cross-link

- Link the guide from the capability record (`docs/architecture/<capability>/README.md`) so the source points at its rendering.
- Link capability guides from any tutorial that traverses them.
- Register the guide in `docs/guides/README.md` (the guides index) if the project keeps one.

---

## Quality Checklist

- [ ] There is validated, user-observable behavior to teach (not a hypothesis, not a pure refactor)
- [ ] Correct altitude chosen (capability guide vs. journey tutorial)
- [ ] All four Diátaxis quadrants considered; each kept distinct
- [ ] Front-matter carries `source:` and `last-validated:`
- [ ] No mechanics restated from the record — reference links to contracts instead of copying them
- [ ] Present tense, no sprint/date history in the body
- [ ] Technical-accuracy review by the Principal Engineer complete
- [ ] Guide cross-linked from the capability record (and tutorials from capability guides)

---

## Anti-Patterns

- Teaching from the wave `product-architecture.md` or sprint notes — that is the bet, not the truth
- Blending quadrants — a tutorial that turns into an API dump, or reference padded with narrative
- Hand-copying a contract into reference where it will silently drift; link to the machine-readable source
- Authoring guides for a hypothesis before the behavior shipped
- Annotating a guide with change history instead of re-deriving it in clean present tense
- Leaving a cross-capability tutorial orphaned — every tutorial has a Product-Designer owner and a wave journey source
