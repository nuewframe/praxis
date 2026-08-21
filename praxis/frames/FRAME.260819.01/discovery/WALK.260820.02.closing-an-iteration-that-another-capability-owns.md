# WALK.260820.02 — Closing an iteration that another capability owns

**Frame:** FRAME.260819.01 · **Storm:** ES.260819.01 · **Date:** 2026-08-20
**Kind:** user journey walkthrough with system thinking overlaid — discovery's *depth* activity,
second iteration. `WALK.260820.01` walked the **read** side. This walks the **write** side.

## What this walk is for

`E7` found that every actor who must *trust* an artifact is read-side and issues no commands,
and `WALK.260820.01` walked that side to its end: four capabilities emit results, one mechanism
renders them, one seam (`read-model@v1`) carries everything across. That seam held.

The write side has no equivalent and has never been walked. Traversing the committed graph
surfaces two edges that cross a capability boundary in the direction nothing has modelled:

| Slice | Realizes | Writes / feeds | Owned by |
|---|---|---|---|
| `TS.260820.07` close-an-iteration | `CAP.claim-settlement` | `IterationClosed`, `CloseRefused` | `CAP.work-admission` |
| `TS.260820.17` promote-what-shipped | `CAP.release-binding` | `capabilities-and-what-they-own`, `what-is-currently-true` | `CAP.delivery-record` |

Neither is necessarily wrong. Both mean a capability must cause a change to state it does not
own, and the record says nothing about through what. A boundary that is crossed by two of
eighteen slices and has no declared seam is not a boundary — it is an omission.

**Path chosen:** `TS.260820.05` (pick up) → `.06` (attach evidence) → `.07` (close) → `.16`
(bind) → `.09` (cut) → `.17` (promote truth) → `.18` (resolve a symptom). One unit of work from
admission to the symptom it retires. It is the riskiest path available because it touches all
five capabilities in sequence, crosses the two undeclared edges, and ends by asserting a fact
about a *release* from inside the capability that owns *problems*.

**Also under test:** whether `CAP.delivery-record` is one capability. It owns 15 of 31 events
and 4 of 8 read models, and every other capability reads or writes through it. That is either
the correct centre of the model or a shared database wearing a capability's name.

---

## The walkthrough

### Step 1 — The working agent picks up a slice

`work-admission` must decide: are the dependencies frozen, is another iteration open on this slice,
is the work disjoint from what is in flight. The first two questions are about facts it owns
(`SliceVetted`, `DependencyFrozen`, `IterationClosed`). The third is not: *what this slice
depends on* and *what layers and claims it declares* are `SliceCut` and `ClaimStated`, owned by
`delivery-record`.

So the gate cannot decide from its own state. It needs another capability's facts, and the only
seam the record declares — `read-model@v1` — is the wrong instrument: that seam exists so a
*renderer* can compose a document without knowing what a view means. Here a capability needs the
facts themselves in order to *judge*.

Two ways to get them:

- **Read the store.** `work-admission` reaches into how slices are held. Then a change to how a
  slice is stored forces a change in the gate — the same coupling argument `E10` used to narrow
  projection, arriving on the write side instead of the read side.
- **Be handed a typed fact set.** `delivery-record` answers a named query with values. The gate
  computes its verdict from what it was handed plus its own state, and nothing else.

The second is the only one compatible with the frame's principle. A verdict computed from a
typed input can be **recomputed** from the record later; a verdict computed by reaching into a
store depends on the store's shape at the moment it ran, and *"fidelity is a property computed
from the record"* becomes untestable for the one gate the method has.

> **The seam this step needs: `fact-set@v1`.** A capability declares the queries it depends on;
> the record answers them with typed values. No capability reads another's internals.

### Step 2 — Evidence attaches, and the layer set comes from somewhere else

`TS.260820.06` C1 refuses evidence naming a layer the slice never declared. The declared layer
set lives on the slice (`delivery-record`); the refusal is `claim-settlement`'s. Same shape as
step 1, no new problem — the second consumer of `fact-set@v1`, which is what makes it a seam
rather than a one-off.

### Step 3 — The close, and the edge that has no seam

`TS.260820.07` is where the walk breaks.

The agent asks to close. `no-silent-drop` refuses if any claim is unsettled and uncarried. The
slice realizes `claim-settlement` and produces `IterationClosed` and `CloseRefused` — which
`work-admission` owns. Something has to give, and there are three ways out:

1. **Move the events to `claim-settlement`.** Then the *iteration* — one entity — is opened by one
   capability and closed by another. The storm clusters by *what must stay consistent together*;
   an entity whose lifecycle spans two owners is the exact thing that principle forbids.
2. **Merge the two capabilities.** `CAP.work-admission` records this as a predicted merge that
   *did not* happen, because they are complementary exclusions. Nothing in this walk disturbs
   that argument.
3. **Separate the judgement from the act.** `claim-settlement` decides what is settled and what
   is carried, and emits that as a fact set. `work-admission` owns the iteration, consumes the fact
   set, and closes or refuses. The judgement never performs; the owner never judges.

Option 3 is the only one that keeps one entity under one owner *and* keeps the two exclusions
intact. It also generalises: it is `fact-set@v1` again, carrying a judgement instead of a
description. Nothing new is needed on the seam — which is the argument for it.

But it moves something. `no-silent-drop` is listed on `CAP.claim-settlement`, and under option 3
the refusal happens at `work-admission`. That forces a rule the record does not yet state:

> **A policy is enforced by the capability that owns the act it refuses** — not by the one that
> owns the concept it is about.

Checked against the other nine policies, this reassigns exactly one (`no-silent-drop`). Every
other policy already sits on the capability owning the refused act, including the two that
consume another capability's facts (`resolution-names-a-release`, `guide-follows-promoted-truth`).
A rule that reassigns one of ten is a rule the model was already following without saying so.

And `TS.260820.07` is then filed under the wrong capability: it realizes `work-admission`.
The slice's *content* is right — this is `E25` in reverse, where the gate test was right and the
slice was wrong; here the slice is right and the capability record is wrong.

### Step 4 — Binding refuses work that did not close

`TS.260820.16` C1 refuses an unclosed iteration, *naming its unsettled claims*. `release-binding`
therefore needs both the iteration's closure state (`work-admission`) and the claim ids that are
unsettled (`claim-settlement`) — a third and fourth consumer, and the first that needs fact sets
from **two** capabilities to produce one diagnostic.

Nothing new breaks. Worth recording only because it is where a shortcut would be tempting:
`release-binding` could ask *"is this iteration bindable?"* and let someone else decide. That would
put the binding rule outside the capability that owns binding, and the step-3 rule forbids it.

### Step 5 — The cut, and the first thing that is not a fact

`TS.260820.09` C2: the cut and its index node are **one operation — a failure leaves neither**.
The cut also writes a tag into git, which is not part of the record at all.

This is the first place the walk needs something no seam describes: a **unit of change that
applies whole or not at all**, spanning more than one fact and, here, more than one system. It
recurs immediately — `TS.260820.17` C1 requires the record be *byte-unchanged* after a failed
promotion, and `TS.260820.07` C3 refuses a claim deleted mid-iteration, which is only checkable if
the iteration's claim set is pinned at some boundary.

Three acceptance criteria on three slices in three capabilities are all settling against the
same unstated structure. That is an architectural fact, not an implementation detail:

> **Every command produces exactly one change set. It applies whole or not at all, and it
> carries its own trail entry.** The change set — not the file, not the node — is the unit of
> atomicity in the model.

### Step 6 — Promotion writes into records another capability owns

`TS.260820.17` derives what changed from the bound work and moves current truth on every
affected capability record. Capability records belong to `delivery-record`. This is the second
undeclared edge, and step 3's answer does not obviously reach it: no judgement is being handed
over — an actual change to another capability's state is.

Unless ownership is finer-grained than the file. Ask *who may author this field*, and the answer
is unambiguous: **only** promotion may write a capability's current truth. `TS.260820.17` C2
refuses a hand-edit outside promotion — the acceptance criterion is already stating the rule.

> **Single-writer is per fact, not per file or per entity.** `delivery-record` owns the
> capability record; `release-binding` owns the authorship of its current-truth field. One
> writer per fact, and the entity may have several.

With that, the second undeclared edge is not an edge: it is `release-binding` authoring facts it
owns, which happen to live in a record another capability owns. Nothing crosses.

### Step 7 — A symptom is resolved, and the clock is external

`TS.260820.18` marks a symptom resolved; `resolution-names-a-release` requires the named release
to have bound a slice whose `attacks` names that symptom. `delivery-record` owns the act and
consumes a `release-binding` fact set — step 1's seam, fifth consumer, no new structure.

What *is* new: `E21` says a release node is an index **into** git, never a copy of it, because a
mirror would be a second clock. So the model has an external system that owns history, and the
record holds pointers into it that it cannot itself validate. That is a relationship the
architecture has to name, or the boundary will be crossed silently the first time something
finds it convenient to cache a commit's contents.

---

## What this says about `CAP.delivery-record`

The size is real: 15 events, 4 read models, and a consumer in every other capability. The walk
tested whether that makes it a capability or a database.

It is not a database. Six policies sit on it, and each refuses an act it owns — a capability
that does not trace to a storm cluster, a slice that does not have the shape, symptoms that do
not converge, a symptom resolved without an attacking release, a view whose lifetime is
undeclared, a guide ahead of promoted truth. Those are refusals, and a store does not refuse.

But it is carrying something that is not a capability. Every one of the five capabilities needs
facts held, validated for shape, changed atomically, trailed, and answerable as a typed set.
`delivery-record` looks three times the size of its peers because that substrate has been read
into it. `WALK.260820.01` already said it in one line — *"the engine sits under all four as a
seam, not a capability"* — and this walk reaches the same place from the other side.

**Verdict: `CAP.delivery-record` stands, and the substrate beneath all five is factored out of
it.** What leaves is not its facts; it is the implied ownership of everyone else's.

The merge that built it survives, but its *argument* does not. `merge-reason` says you would
never ship a Praxis that records frames but not slices — a **fundability** argument, and
fundability is gate test 2. The clustering principle is **consistency**, and a symptom's
resolution must stay consistent with a *release*, not with a slice's shape. The merge reaches
the right answer through the wrong test, which is worth exactly as much as `E25`'s converse.

---

## Data shapes

### `fact-set@v1` — the seam between two capabilities

```kdl
// Emitted by the capability that OWNS the facts, in answer to a named query the
// consuming capability declares. The consumer computes its own verdict from this
// plus its own state, and reads nothing else. Same discipline as read-model@v1:
// values only, and silence is never absence.
facts "claims-of-iteration" for="CAP.work-admission" {
    owner "CAP.claim-settlement"
    as-of "2026-08-20T14:02:00-05:00"
    query "claims-of-iteration" iteration="PASS.260820.03"

    fact "C1" settled=#true  evidence="TEST.260820.11"
    fact "C2" settled=#false carried-by="F7"
    fact "C3" settled=#false carried-by=#null      // the one that refuses the close

    empty "none" says="this iteration states claims"    // an unclaimed iteration is a fact, not a blank
}
```

**Constraints the walk found necessary:**

1. **The consumer declares the query; the owner answers it.** A consumer that composes its own
   query over another capability's facts is reading internals with extra steps.
2. **Values only, and no verdict field.** The owner may state that C3 is unsettled. It may not
   state that the close is refused — that is the consumer's act, and a fact set carrying a
   verdict is an instruction wearing a fact's shape.
3. **Answered as of a moment, and never cached across a change set.** A gate that decided from a
   stale fact set is a gate whose refusal cannot be recomputed.
4. **Empty answers declare their own emptiness** — carried over from `read-model@v1` constraint
   3, for the same reason at a different granularity.

### `change-set@v1` — the unit of atomic change

```kdl
change-set {
    by      "agent:principal-engineer"
    command "close-iteration"
    at      "2026-08-20T14:02:11-05:00"

    writes "PASS.260820.03" field="state"     was="open"   now="closed"
    writes "PASS.260820.03" field="closed-at" was=#null    now="2026-08-20T14:02:11-05:00"
    // every write names the fact and the writer that owns it. A write whose author does
    // not own that fact is refused before anything is applied.

    trail "closed with C2 carried by F7"
}
```

**Constraints:**

1. **One command, one change set.** Not one per file touched — `promote-truth` moves truth on
   many capability records and is still one.
2. **Whole or nothing**, with the record byte-unchanged on failure (`TS.260820.17` C1).
3. **Every write names the fact it changes**, so the per-fact writer rule is checkable before
   application rather than by review afterwards.
4. **A change set that touches the external history is still one change set.** `TS.260820.09` C2
   demands the tag and the index node succeed or fail together; the model must say this is
   required, and leave *how* to whatever binds it.

---

## Layers this path touches

| Layer | What this path needs from it |
|---|---|
| `doctrine` | the skill teaches that a capability asks for facts and never reads another's state, and that judgement is handed over as a fact, never as an instruction |
| `enforcement` | per-fact writer checked before a change set applies; `no-silent-drop` enforced at the iteration owner; atomicity fault-injectable |
| `harness` | none on this path — it is entirely internal to a repository |
| `docs` | refusals, carried claims and promoted truth all survive into the release projection |

---

## Findings

**W7 — the write side has no seam, and needs one.** Five of eighteen slices require facts owned
by another capability in order to decide. `read-model@v1` cannot serve them: it exists so a
renderer can compose without understanding, and these consumers must understand in order to
judge. *Obliges: record `fact-set@v1` as a seam contract, with the no-verdict constraint.*

**W8 — `TS.260820.07` realizes the wrong capability.** The iteration is one entity; `work-admission`
opens it, so `work-admission` closes it. `claim-settlement` supplies the settlement fact set and
does not perform. *Obliges: re-file `TS.260820.07` under `CAP.work-admission`, and restate
`work-admission`'s `not` — "and then it is finished" is false of a capability that owns an iteration
through its whole life.*

**W9 — enforcement location was never stated, and one policy is misplaced.** A policy is
enforced by the capability that owns the act it refuses. Applying it reassigns exactly one of
ten (`no-silent-drop`, to `work-admission`), which is evidence the model already worked this way
without declaring it. *Obliges: state the rule; move the one policy.*

**W10 — single-writer is per fact, not per file.** `TS.260820.17` C2 already refuses a
current-truth edit made outside promotion, so the acceptance criterion states the rule the
capability records do not. With it, the promotion edge disappears rather than needing a seam.
*Obliges: state per-fact authorship in the architecture; no slice changes.*

**W11 — three acceptance criteria in three capabilities settle against an unnamed structure.**
`TS.260820.09` C2, `TS.260820.17` C1 and `TS.260820.07` C3 are all asserting atomicity over a
unit nothing has defined. *Obliges: name the change set as the unit of atomic change, and make
the per-fact writer rule checkable inside it.*

**W12 — `CAP.delivery-record` survives; the substrate does not belong to it.** It refuses six
classes of malformed derivation, which a store does not do. What made it look three times the
size of its peers is the record substrate every capability needs, read into the one capability
that happened to hold the most facts. *Obliges: model the substrate beneath all five, owned by
none.*

**W13 — the merge argument in `CAP.delivery-record` uses the wrong test.** `merge-reason` argues
fundability; clusters group by consistency. The verdict survives the correction, so nothing
splits — but the reasoning on the record is not the reasoning that justifies it.
*Obliges: restate `merge-reason` on the consistency argument, and carry the falsifier: if
problem facts ever change on a different cadence or by a different writer than derivation facts,
the merge is wrong.*

**W14 — an external system owns history, and the record cannot validate its own pointers.**
`E21` forbids mirroring it; `TS.260820.09` C3 asserts the recorded commit resolves and contains
that release's directory — a claim the record can only settle by asking the external system.
*Obliges: name the external history as a first-class participant with one direction of trust,
so nothing later caches its contents for convenience.*

**W15 — nothing on this path touches `harness`.** Every write-side slice reaches doctrine,
enforcement and docs, and none reaches the six install paths. Consistent with `E11`, and worth
recording: the layer set a slice must reach is a property of the *path*, not a fixed checklist.
*Obliges: none.*

---

## Verification pass — the architecture checked against all eighteen slices

The walk produced `NA.260820.01`. Traversing every slice against it, rather than re-reading it,
produced two findings the walk did not.

**W16 — `needs-read-model` and `feeds-read-model` do not mean what the edge shape suggests.**
Read naively, the graph shows eight cross-capability edges. Five are not edges at all:
`needs-read-model` is what the **actor** must see before issuing a command — it reaches them
through projection, and `TS.260820.16` reading `what-is-currently-true` is a maintainer looking at
a dashboard, not `release-binding` reaching into `delivery-record`. And `feeds-read-model` points
**backwards**: when a slice of A feeds a view owned by B, A writes nothing into B — A authors
facts that B's view is computed from, so the real edge is B consuming a fact set from A. Without
both rules the two genuine write-side problems are lost among five that are not problems.
*Obliges: state both rules in the architecture; no slice changes.*

**W17 — one seam edge was missing, and applying W16 found it.** Every `feeds-read-model` crossing
an ownership boundary must have a matching consumption in the other direction. `TS.260820.12`
(declare lifetime) and `TS.260820.18` (resolve a symptom) both feed `the-published-set-for-a-release`,
which `release-binding` owns — so `release-binding` consumes lifetime and problem facts from
`delivery-record`, and nothing declared it. *Obliges: `lifetime-facts@v1` and `problem-facts@v1`,
now recorded.* It also settles a duplication that was about to happen: a release's `resolves` is
**derived** from the symptoms that name it, never stored beside them — `E21`'s argument one scale
down.
