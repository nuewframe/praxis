# What the CLI is actually asked

A running log of every `praxis` invocation, and — more importantly — **the question behind
it**. Raw material for deciding what a query surface has to answer.

Not a record: this directory sits beside `.render/` and `.cache/` under the state root, and
`load()` reads only `*.kdl`, so nothing here is checked or published.

Append as you go. Do not tidy — the shape of what gets asked twice is the finding.

---

## How to read the pattern column

| Pattern | Meaning |
|---|---|
| **read** | answers a question, writes nothing |
| **gate** | decides and records the decision, either way |
| **write** | changes the record |
| **chain** | only meaningful in a sequence with others |

---

## 2026-08-22 — one session, ~120 invocations

### Reads that ran over and over

| Invocation | The question | Pattern | Times |
|---|---|---|---|
| `praxis check` | is the record still coherent after that edit? | read | **~40** |
| `praxis audit-surfaces` | how many shipped surfaces are unanchored now? | read | ~12 |
| `praxis prove` | how many rules are witnessed now? | read | ~8 |
| `praxis ready` | what can I start, and what blocks the rest? | read | ~6 |
| `praxis check-invariants` | what do we guarantee, and what keeps each? | read | ~5 |
| `praxis truth` | what does the record already know? | read | 3 |
| `praxis review <ITER>` | what has this iteration promised and shown? | read | 1 |
| `praxis schema` | what governs this repository? | read | 2 |

**The tell:** `praxis check` ran roughly forty times, and *every time* the interesting output
was one number — refusals, reports, entity kinds. The other 99% of the output was scrolled
past. Four commands were run purely to read a counter:

```
praxis check              → "21 entity kinds declared"
praxis audit-surfaces     → "50 of 50 anchored"
praxis prove              → "21 of 48 witnessed"
praxis check-invariants   → "8 of 11 have a probe"
```

Four processes, four full corpus loads, four parses of every `.kdl` file in the tree — to
produce four integers. That is the strongest signal in this log.

### Questions asked with grep, because the CLI could not answer them

Each of these is a query someone wanted and had to hand-roll:

| What was wanted | What was actually run |
|---|---|
| which closed iterations are unbound? | `for f in ITER.*.kdl; grep 'state "closed"' … grep -v REL` |
| which surfaces serve the frame? | `grep -rn 'serves "FRAME' praxis/` |
| how many symptoms are still present? | `grep -c 'state="present"' FRAME.*.kdl` |
| what does each probe scan? | `grep -o "--include='\*\.[a-z]*'" scripts/check-*.sh` |
| which skills have no anchor? | `comm -23 all.txt anchored.txt` — before `audit-surfaces` existed |
| what does this entity declare? | `grep -n 'entity "iteration"' -A 8` |
| where is this rule implemented? | `grep -rn "MisfiledRecord" crates/` |
| which findings are open? | `grep -n 'finding "A' -A 3` |

**The tell:** every one of these is a traversal of the graph the engine already holds in
memory. `Corpus` has `slices`, `attempts`, `releases`, `symptoms`, `capabilities`, `views`,
`decisions`, `surfaces`, `invariants`, `roles` — and no way to ask it anything.

### Gates — the ones that refused

Refusals are the interesting invocations, because they are the ones that taught something.

| Invocation | Outcome |
|---|---|
| `pick-up TS.260821.03 TS.260821.04` | **refused** — `dependencies-delivered`: 04 needs 03 |
| `close ITER.260822.06` | **refused** — `a-close-names-its-attester` |
| `close ITER.260822.06 --attested-by agent:principal-engineer` | **refused** — self-attestation |
| `cut-release 0.8.0` | **refused** — published set uncommitted |
| `cut-release 0.8.0` (again) | **refused** — `the-maintainer-confirms-the-version` |

Five refusals, five different rules, and each one wrote a `REF.` record. The gate surface is
the part of this CLI that behaves best.

### Writes

| Invocation | Times |
|---|---|
| `pick-up <SLICES>` | 7 |
| `close <ITER> --attested-by <id>` | 7 |
| `bind <ITER> <VERSION>` | 7 |
| `publish <VERSION>` | ~8 |
| `accept` | 1 |
| `cut-release <VERSION>` | 2 (both refused) |

`publish` ran eight times for one version — every time the record changed in a way that might
alter a projection, with no way to ask *whether* it would.

---

## What was done with the output

The invocation says what was asked. This says what was **used** — which is where the mismatch
between the interface and the need actually shows.

### `praxis check` — ~40 runs

| What was read | What it drove |
|---|---|
| the final line, `ok` or `N refusal(s)` | continue, or stop and fix |
| the first `×` block, when non-zero | the next edit |
| everything else | **scrolled past** |

Almost every run was consumed as a **boolean**. On the ~8 runs that refused, exactly one
refusal was read — the first — and fixed, then the command was re-run. Nobody ever read
refusals two through eleven; when the schema move produced 64 at once, the reaction was to
find the *cause*, not to read the list.

**Consumption shape:** `bool`, then on false, `first(refusals)`. The list is a poor fit for
how it is used.

### `praxis audit-surfaces` / `prove` / `check-invariants` — ~25 runs

| What was read | What it drove |
|---|---|
| the summary line only | whether a claim could be marked met |
| the itemised list | twice, both times to build a work list |

These were read as **progress meters**: 23 → 34 → 35 → 45 → 48 → 50. The number went into
iteration evidence almost verbatim. The per-item output was used exactly twice out of
twenty-five, and both times it was immediately piped through `grep`/`sed` to strip the prose:

```
praxis audit-surfaces | grep UNANCHORED | sed 's/praxis: //;s/ — UNANCH.*//'
```

**Consumption shape:** a scalar, tracked over time. The prose framing was actively removed.

### `praxis ready` — 6 runs

| What was read | What it drove |
|---|---|
| the `ready` section | which slice to pick up next |
| the `blocked` section | once — and it found the deadlock I had created |

The blocked section paid for itself in one run: it showed `TS.260821.05` blocked on
`TS.260821.04` while 04 carried claims against 05, which is a cycle no amount of reading the
files would have surfaced. **That is the highest-value single output in this log.**

**Consumption shape:** a partition, and the *blocked* half is the valuable one.

### `praxis close` / `pick-up` / `bind` — 21 runs

Read as: did it happen, and what is the new id? The claim-by-claim `C1 — met` lines were
skimmed for a `not met`, never read in full when all passed.

The **refusals** were read completely, every word, and acted on immediately. A refusal is the
only output in this CLI that gets full attention — because it is the only one that is
surprising.

### `praxis truth` / `review` — 4 runs

Read for one section each, never whole:

```
praxis truth   | sed -n '/capabilities ──/,/releases ──/p'
praxis review  | head -30
```

Both were immediately sliced. **Nothing consumed a whole read model in this session** — every
projection was narrowed at the shell.

### `praxis publish` — 8 runs

| What was read | What it drove |
|---|---|
| the file list | confirmation it wrote what was expected |
| byte-identity | via `git diff --exit-code`, not via the tool |

The question being asked was *"did this change the output?"* and the tool cannot answer it —
so `git` answered it instead, twice.

---

## What the consumption pattern says

1. **Most output is consumed as a scalar or a boolean.** Forty checks, twenty-five meters,
   and the prose is stripped by `sed` when it gets in the way. The CLI writes for a reader;
   it is being used by a loop.
2. **Refusals are the exception, and get read in full.** They are surprising, they name a
   fix, and they are acted on immediately. This is the part that works.
3. **Read models were narrowed at the shell every single time.** Not once was a whole
   projection consumed. A section selector belongs in the tool.
4. **"Would this change anything?" was asked repeatedly and never answerable.** It was
   resolved with `git diff` for publish, and with re-running `check` for everything else.
5. **The one high-value structured output — `ready`'s blocked section — was structured
   precisely because it answers a question that has no scalar form.** Worth noticing which
   outputs earn their length.

---

## What this suggests, as of the first entry

Stated as observations, not requirements — the point of the log is to see whether these hold up.

1. **A counter surface.** Four commands exist mostly to print one integer each. `praxis
   count <what>` or a single status line would replace most of forty invocations.
2. **A traversal surface.** Every grep above walks a graph the `Corpus` already holds.
   Unbound-closed, serves-X, findings-open, symptoms-present are all one-liners against it.
3. **A dry-run for projections.** `publish --dry-run` exists; *"would publishing change
   anything?"* does not, and that is the question that was actually being asked.
4. **The chain has an order, and only refusals teach it.** `bind → publish → commit →
   cut → verify → promote → resolve`. Two of the five refusals above were the tool teaching
   the sequence, which is good — but nothing states it up front.

---

## 2026-08-22, later — the release chain, end to end

First run of `bind → publish → commit → cut → verify → promote`. Walked as
`WALK.260822.02`.

| Invocation | The question | Outcome |
|---|---|---|
| `bind ITER.x 0.8.0` × 7 | attach today's work to the version | each re-proposed the bump |
| `cut-release 0.8.0` | cut it | **refused** — published set uncommitted |
| `cut-release 0.8.0` | again, after committing | **refused** — nobody confirmed the bump |
| `cut-release 0.8.0 --confirm` | with the decision made | cut at `8ddfd00` |
| `check` | did the cut hold together? | 6 refusals — 5 of them the next command |
| `promote 0.8.0` | fold what shipped into what each capability is | 5 capabilities, sought → active |
| `verify-published` | was anything hand-edited? | verified, and see AS3 |

### What was done with the output

`praxis check` immediately after the cut returned six refusals, and **five of them were the
tool naming the next step** — `promoted-truth-is-derived`, once per capability. That is the
only time in this log where a refusal list was read as an instruction rather than as a defect.
Consumption shape: `first(refusals)` was wrong here; the *shape* of the list was the signal.

The two `cut-release` refusals taught the sequence. Nothing states it up front, and both were
read in full and acted on immediately — consistent with every other refusal in this log.

### The traversal that was grepped again

```
for f in ITER.*.kdl; do
  grep -q 'state "closed"' $f && ! grep -q "$id" REL.0.8.0.kdl && echo $id
done
```

Closed iterations no release binds. Same anti-join as the first entry, wanted again within the
hour. `TS.260821.11` names it as C4.

### New: a question with no command at all

*"Which symptoms could 0.8.0 resolve?"* — every symptom has attacking slices, and answering it
meant seven greps over `attacks`. There is no `resolve` command in `--help`; symptom resolution
is a record edit plus `accept`. **The chain's last step is the one with no interface.**
