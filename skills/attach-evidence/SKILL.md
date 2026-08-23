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

Evidence written at close is written from memory of what the work meant. Evidence written at the
moment the layer is reached says what was shown, while you are still looking at it:

```kdl
layer "enforcement" state="evidenced" \
    evidence="six tests, one of them asserting the rule goes silent when the record stops \
        declaring it" \
    reaches="evidence-names-its-layer, failing closed on an undeclared layer"
```

## Settle it on a run, not on a sentence

```
praxis evidence <ITERATION> --claim <ID> [--slice <TS-ID>] [--by <selector>]
```

**Praxis runs your verification and records what came back** — the command, the exit status,
and a digest of what it printed. `TS.260823.02`.

This exists because the sentence was never checkable. `attach-evidence` used to say, in this
file, *"It does not judge the evidence"* — so `evidence="46 tests pass"` read identically
whether 46 passed or none existed. The validation review closed an iteration with three
evidenced layers and a met claim in a repository containing **no application code**, and
nothing objected. That is `S2` reproduced inside the mechanism built to compute it away.

**What runs is the record's, never yours.** The repository declares its verification once:

```kdl
config {
    verification {
        runner "cargo test --quiet"
        select-with "--test"
    }
}
```

`--by` **selects within** that runner; it never becomes a command. An engine that ran whatever
the caller passed would be a command runner driven by the agent it exists to check — that
agent could name `true` and settle every claim in the record.

| | |
|---|---|
| a non-zero exit | **refuses.** The claim stays pending and the status is named. A red run is a fact, and it is not evidence the claim holds |
| a selector a runner cannot take | **refused, not dropped.** A dropped selector runs the whole suite and records it as the one test |
| a claim id in a multi-slice iteration | **refused without `--slice`.** A claim id is unique within a *slice*; an iteration over three carries three `C1`s, and settling whichever comes first is the failure `a-claim-id-is-unique-in-its-iteration` names |
| re-witnessing a settled claim | the superseded run is **matured**, not overwritten. Behaviour evolves; the latest is truth and the chain back is the maturation |

**A repository that declares no verification keeps prose evidence**, and the record shows those
claims as reported rather than witnessed. That fallback is a stated fallback — not a default to
drift into.

### What the digest does and does not claim

It is not a signature, and it is not hard to forge: anyone who can edit the record can run the
same function. What it catches is the value **written from memory** — the agent does not know
the digest until the run has happened, and that is the property prose never had.

It also does not prove the run tests the behaviour the claim describes. Four claims settled
against one test file share one digest, and that is honest: it witnesses *that a run happened
and passed*, never *that it proves this*. Choosing a run that proves the right thing stays
yours, and no checker can take it.

## Evidence states WHAT was shown, never where to look

**A path is not a citation this record can follow.** `the-record-cites-what-it-holds` refuses a
field naming a file, and `evidence` is where it used to happen most:

| Refused | Write instead |
|---|---|
| `evidence="tests/evidence_layers.rs::c1_evidence_naming_an_undeclared_layer_is_refused"` | `evidence="evidence naming an undeclared layer is refused"` |
| `evidence="skills/attach-evidence/SKILL.md, registered"` | `evidence="surface.skill.attach-evidence, registered"` |
| `evidence=` naming a published Markdown file by path | `evidence="the 0.8.0 capability guide"` |

The rename that breaks the first column is invisible: the claim goes on reading as settled while
pointing at nothing. The second column survives it, and says what the test proved rather than
where somebody can go to watch it pass. The repository already knows where things live, and git
knows when they moved — the copy the record keeps is the one that rots.

**What you MAY cite is an id.** `TS.260821.17`, `AG1`, `C3`, `surface.skill.attach-evidence` — the
record holds those, and `dangling-relationship` refuses one it does not. That is exactly the check
a path cannot have.

If you cannot say what was shown, the layer is not evidenced yet, and saying so is the honest
close.

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

- **It does not judge whether the verification is any good.** A weak test that passes witnesses the
  run; whether it proves the behaviour is a review question, at close. What it no longer accepts is
  a claim settled on nothing at all.
- **It does not produce the evidence.** That is the work itself.

Related: `pick-up-a-slice` · `cut-a-slice` · `see-what-is-ready` · `adopt-the-method` (why a path is never a citation) · `write-durable-comments` (the same test, for comments).
