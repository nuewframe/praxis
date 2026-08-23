---
name: adopt-the-method
description: >
  The one front door. Put a repository under the delivery graph without copying it — `praxis adopt`
  writes the binding, a verify entry point naming only probes that ship, a pre-commit hook and one
  engineering-doctrine pointer, and nothing about the product. Teaches what the method carries, what
  a repository may add, what it may never redefine, and why nothing else was written for you. Runs
  when a project first installs Praxis, and on every new kind after that.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, replace_string_in_file, run_in_terminal]
---

# Skill: Adopt the Method (`skills/adopt-the-method/`)

**Audience:** A repository that is not Praxis. **When:** on install, and on every new kind.

```
praxis adopt --repository <owner>/<name>   put this repository under the method
praxis check                               with nothing copied
praxis view what-you-must-declare          the seven kinds you write next
praxis schema --print                      the method's record, whole
```

---

## Run this first

```console
$ praxis adopt --repository acme/checkout \
      --language typescript --test "npm test" --lint "eslint ." --format "prettier --check ."
```

Four files, and that is the whole of it:

| File | What it is |
|---|---|
| `praxis/config.kdl` | the binding. `governed-by`, the layer vocabulary your slices may declare, how this repository verifies, where things live |
| `scripts/verify.sh` | the gate. Its probe steps are **generated from the record of what this plugin ships**, so it cannot name one that is absent |
| `.githooks/pre-commit` | the cheap half of that gate. Activate with `git config core.hooksPath .githooks` |
| `.github/instructions/capability-structure.instructions.md` | the engineering doctrine, as a **pointer** to the plugin's skills plus the handful of facts true of your stack |

Then `praxis check` passes, over a record that holds a binding and nothing else. That is the
correct starting state, and `praxis view what-you-must-declare` names what comes next.

`praxis adopt --dry-run` prints what the current templates would produce without writing. Run
it after a plugin upgrade and diff it against your tree.

**There was no greenfield/brownfield split, and the two skills that made one are retired.**
`bootstrap-project` and `provision-project-overlay` scaffolded a wave directory (retired), a
sprint placeholder (retired), fifty-one overlay templates, and a `verify.sh` calling two
probes this plugin does not ship — one of them described in its own comment as hard-fail with no warn mode. Neither
named the delivery graph once. The split was never the useful axis: what a repository needs on
arrival is the same whether or not it already has code, and what differs is everything below,
which is judgement rather than scaffolding.

### What adoption will not do

**It does not write your record.** No frame, no storm, no capability, no slice. Adoption
produces the binding and the shape; the rest is the work, and a tool that derived it from what
your repository already claims would import those claims without their evidence. The longest
section of this skill is about exactly that.

It also **will not migrate a repository already running the retired spine.** If you have wave
and sprint documents from `v0.7.1`, those artifacts no longer map, and what to do with them is
a separate question with its own evidence. Adopt beside them and let the record catch up.

---

## Start here — the least you must declare

```console
$ praxis view what-you-must-declare
```

**Seven kinds, not twenty-seven.** `config`, `frame`, `symptom`, `event-storm`, `cluster`,
`capability`, `thin-slice` — and the view names the rest with the moment each arrives.

It is **computed, never listed.** It starts at the unit of work and follows every *required*
reference: a slice must name a capability, a capability the storm it was derived from, a storm
its frame. Add the config that binds you to the method, and that is the closure. A kind gaining
a required edge changes the answer with nothing to update — a hand-written minimum is a second
copy of the schema, and it drifts.

This section leads because of the order the questions actually arrive in. What follows —
*what you may never redefine* — is the right rule and an answer to a question you do not have
yet. Until `TS.260823.09` it came first, and the honest consequence was that the fastest route
into this method was to copy somebody's record and change the words. That is transcription,
which the longest section below argues nobody should do, and the size of the vocabulary is what
made it the rational choice.

**There is no lite profile, and the rest is not optional forever.** A kind outside the minimum
is one the work has not called for *yet*. A method a repository can weaken locally reports
whatever that repository wanted to hear.

---

## What you do not copy

**Nothing.** The method's entity kinds, its rules, and its admission conditions travel with the
engine.

That was not true until `TS.260821.10`. Adopting meant lifting a 946-line `schema` block and an
`admits` block out of Praxis's own discovery folder — which means every repository owns a
different version of the method and nobody can say which one is running it. That is the
second-copy problem, applied to the definition of the record itself.

## Two carriers, one file

`praxis/method/DELIVERY-GRAPH.v1.kdl` is the method. It reaches you twice, because neither route
alone reaches everybody:

| You have | How the method reaches you |
|---|---|
| The plugin, no binary | The file ships in the plugin tree. Read it. |
| The binary, no plugin | It is embedded, from that same file. |

`include_str!` points at the shipped file, so there is nothing to keep in sync. It is **data** —
parsed by the same reader as any record, subject to its own rules, and self-describing: the
method declares the `method` kind that holds it. An engine that *knew* what a `frame` was could
diverge from the record and nobody could tell, which is what `A4` forbids.

## Bind to it

```kdl
config {
    repository "acme/checkout"
    governed-by "delivery-graph@v1"
    …
}
```

`praxis check` refuses a version the engine does not carry. A binding to a method you do not have
is a repository checked against rules nobody can see, which is worse than no binding — it reads
as one.

---

## Extend, never redefine

Your repository declares the kinds **your** records need and the method does not name:

```kdl
notional-architecture "NA.260822.01" {
    …
    schema {
        entity "cohort" {
            field "slug" each="1"
        }
    }
}
```

The `schema` block is optional. A project with no kinds of its own writes nothing rather than an
empty block.

**Redefining a kind the method declares is refused.** Not warned — refused, and `praxis check`
fails. You may add; you may not remove a kind, loosen a cardinality, or downgrade a rule from
refuse to report. That is `config may bind and may never declare`, one level up: a method a
repository can weaken locally is a method that reports whatever each repository wanted to hear.

Praxis is held to this too. `invariant` and `doctrine-surface` are **its** extensions, not the
method's, because they describe a repository that ships instruction and `acme/checkout` does not.

---

---

## Adopting on a repository that already shipped

**Build the record new. What is already there is evidence, not record.**

This is the one instruction that decides whether a brownfield adoption is worth anything, and
it is the one that sounds like extra work.

The obvious move is to transcribe: capability docs become capability records, ADRs become
decisions, release history becomes releases. Every one of those artifacts makes claims, and
transcribing them **imports the claims without their evidence** — so the record looks checked
from its first commit and is not.

That is `S2` — *an artifact looks identical whether the agent reasoned hard or pattern-matched
a template* — said about a whole repository at once. A transcribed graph refuses nothing,
because everything in it was already agreed before it arrived.

### What to do instead

| Artifact | What it is | What to do with it |
|---|---|---|
| capability / architecture docs | somebody's claims, unchecked | storm the events behind them; derive capabilities from clusters |
| ADRs | real decisions, real arguments | read them; record the ones that still bind, with what they rejected |
| release history | what shipped, and what was claimed about it | bind what the record can compute; **withdraw** what it cannot |
| tickets, RFCs, READMEs | discovery data | walk one path through them and see what survives |

### Restate what it taught. Do not record where it lived.

**There is no `informed-by`, and there deliberately is not one.** The obvious move — keep a
pointer to the document each fact came from — records the half guaranteed not to survive. A
path rots; a path marked as provenance rots while looking deliberate, and a dead pointer
outlives the file it named as a confident lie.

So the substance goes into the record in the record's own words, and the location goes
nowhere:

| Instead of | Write |
|---|---|
| `informed-by=` pointing at the old queueing ADR | the decision itself, with what it rejected and what would show it wrong |
| `evidence=` naming a test file and the function inside it | `evidence="a refund replayed twice moves the balance once"` |
| "see the capability doc" | the capability's `doing`, its `not`, and the four gate tests it passes |

`W16` was thirty-four citations into unfollowable Markdown, read as a documentation problem.
It was the same fault one level down, and `the-record-cites-what-it-holds` now **refuses** a
field naming a file — a hundred and fifty-six of them stood in this repository's own record.

What the record may cite is **its own ids**. `TS.260821.17`, `AG1` and `surface.skill.cut-a-slice`
are followable, because `dangling-relationship` refuses one the record does not hold. That is
exactly the check a path cannot have.

### The evidence, from this repository

`TS.260821.06` ported three Markdown capability records without meaning to run an experiment:

- **two ported and gained** four gate tests and a `keeps-consistent` each — neither of which
  the Markdown originals declared, because a Markdown capability record declares no gate test
  it can fail
- **one did not port at all.** `CAP.method-spine-and-execution` had been hollow since the
  delivery graph replaced the spine, and nothing said so for two versions

The port was slower than a transcription would have been, and it found a dead capability.

### You do not need a rule for this

A transcribed capability has no cluster, no gate tests and no exclusion — the shape check
refuses it already. The doctrine exists because somebody who has not read it will invent the
transcription anyway, and a refusal cannot explain why the faster path was the wrong one.

---

## What this does not give you

That adopting **works**. This removes the copying. Whether the rest of the loop survives contact
with a repository nobody here has seen is what `W18` owes, and `TS.260821.10` deliberately does
not claim it.

---

## Related

- `declare-an-entity-kind` — the shape of an extension, once you need one
- `ask-what-is-true` — the first thing to run in a repository you did not set up
- `cut-a-slice` — what the method wants a slice to carry
- `attach-evidence` — the same rule, said about one claim at a time
- `write-durable-comments` — a citation and a story, told apart
