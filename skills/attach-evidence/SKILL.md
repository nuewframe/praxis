---
name: attach-evidence
description: >
  Back a layer's claim with an artifact instead of your own word for it. Teaches naming the declared
  layer at the moment it is reached, why evidence outside the slice's declared set is refused, and why a
  layer you did not reach must stay visible as unevidenced rather than quietly disappear. Runs during ITERATE.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, create_file, replace_string_in_file, run_in_terminal]
---

# Skill: Attach Evidence (`skills/attach-evidence/`)

**Audience:** Working agent. **Phase:** `ITERATE`, as each layer is reached — not at close.

---

## The rule

**Evidence names the declared layer it evidences, or it is refused.**

The slice set the granularity when it was cut. That is what makes *"how much of this was actually
reached"* a question the record can answer rather than a judgement the working agent reports about
their own work — which is `S2`, and the reason this rule exists.

Two failures, and they are not symmetrical:

| What happened | What the record does |
| --- | --- |
| evidence names a layer the slice never declared | **refused** — the slice declares `doctrine · enforcement`, evidence naming `harness` is evidence for something nobody asked about |
| a declared layer has no evidence | **reported** — carried as unevidenced, never dropped |

An iteration is allowed to close having reached less than it hoped. It is not allowed to do so
**silently**. A layer that vanishes from the accounting was never reached *and* never refused, and
afterwards those two look identical.

---

## Attach it when you reach it, not at close

Evidence written at close is written from memory of what the work meant. Evidence written at the moment
the layer is reached names the artifact that exists right then:

```kdl
layer "enforcement" state="evidenced" \
    evidence="tests/evidence_layers.rs — six tests, and one asserting the rule goes silent \
        when the record stops declaring it" \
    reaches="evidence-names-its-layer, failing closed on an undeclared layer"
```

`evidence` names **something that exists** — a test, a diagnostic, a generated file, a skill. Not a
description of work done. If you cannot point at the artifact, the layer is not evidenced yet, and
saying so is the honest close.

---

## Sealing the layer set

The rule only reports a missing layer once `layer-set sealed=#true`. Before that, the iteration is
still deciding what it will reach, so an absent layer is not yet a gap — which is what lets
`praxis pick-up` open an iteration carrying no layers at all.

**Seal at open, not at close.** A layer set sealed after the work is a list of what you managed, not a
declaration of what you set out to reach — and the gap between those two is the only thing worth
measuring here.

---

## What this does not do

- **It does not judge the evidence.** Absence and mismatch, never quality. A weak test evidences the
  layer; whether it is a good test is a review question, at close.
- **It does not produce the evidence.** That is the work itself.

Related: `pick-up-a-slice` · `cut-a-slice` · `see-what-is-ready`.
