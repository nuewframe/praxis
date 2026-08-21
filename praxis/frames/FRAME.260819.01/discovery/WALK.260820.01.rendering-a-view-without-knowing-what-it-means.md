# WALK.260820.01 — Rendering a view without knowing what it means

**Frame:** FRAME.260819.01 · **Storm:** ES.260819.01 · **Date:** 2026-08-20
**Kind:** user journey walkthrough with system thinking overlaid — discovery's *depth* activity.
The storm was breadth: the whole timeline, shallow. This is one path, all the way down.

## What this walk is for

`ES.260819.01` finding **E10** narrowed `CAP.document-projection` to mechanism and gave it
zero read models, on a coupling argument: if projection knew what *"ready to pick up"* meant,
changing the vet rules would force a change in projection.

It shipped with a falsifier: **if projection ever needs a per-read-model code path, the split
is fake.** That falsifier is now an acceptance criterion (`TS.260820.10` C3). This walk runs
the path that would trigger it, because a capability boundary that has never been walked is
a boundary someone drew on a whiteboard.

**Path chosen:** `TS.260820.04` (work-admission owns the content of *what-is-ready-to-pick-up*)
→ `TS.260820.08` (the same mechanism renders a claim-settlement view) → `TS.260820.10`
(publish) → `TS.260820.11` (verify). It is the riskiest path because it crosses the contested
boundary twice, in both lifetimes, and then tries to prove the result years later.

---

## The walkthrough

### Step 1 — The working agent asks what is ready

> *"What can I pick up right now?"*

**Crosses:** actor → engine.

The question is not a document request. It is a question about admissibility, and only
`work-admission` can answer it: readiness is *dependencies frozen · no live iteration on this slice ·
work disjoint from what is in flight*.

**System thinking:** the first thing this walk kills is the obvious design. If projection ran a
*query* over the graph to build this view, it would have to evaluate those three predicates —
and it would know what "ready" means. **The falsifier fires immediately on a query-based seam.**

So the seam cannot be a query. It has to be a *typed result*.

### Step 2 — work-admission evaluates and emits

`work-admission` evaluates every slice against its own conditions and emits a result. Not
Markdown, not a template — a structure describing what to show, with no opinion about how it
looks.

**Crosses:** capability → projection. **This is the seam.** Named `read-model@v1` below.

**System thinking:** the direction of the dependency is what matters. Projection depends on the
*shape* of the result, which is stable. It does not depend on `work-admission` at all. Change
the vet rules and the result's contents change; its shape does not; projection does not
recompile. That is the coupling argument surviving contact.

### Step 3 — projection renders it whole

Projection receives the result and produces a document: heading, sections, tables, stored prose.
It marks the lifetime — here `working`, so the document announces itself as a preview of a
version that does not exist yet — and writes it to the gitignored working path.

**Crosses:** projection → filesystem.

**System thinking:** projection never inspects a field name. It walks a generic structure. Ask
it to render `what-this-iteration-claims-and-has-shown` (`TS.260820.08`, content owned by
`claim-settlement`) and nothing in projection changes — different result, same walk. **Two
capabilities, one renderer, zero per-view code.** C3 is satisfiable.

### Step 4 — the reviewer reads, and discards

Nothing enters the committed tree. The published tree stays written-once, which is the only
reason it can be immutable per release.

### Step 5 — a release is cut, and the same mechanism publishes

`TS.260820.09` cuts the release. `TS.260820.10` publishes: the same capabilities emit the same
kind of result, the same renderer walks it, and the only differences are the **stamp** (which
release this depicts) and the **destination** (committed, not gitignored).

**System thinking:** working and archival are one mechanism with two lifetimes, not two
pipelines. If they were two pipelines they would drift, and a preview would eventually disagree
with the record it previewed.

### Step 6 — eighteen months later, verification runs — and the walk breaks

`TS.260820.11` says: *"regenerate what the record would have produced for that release, and
compare."*

**It cannot.** Regenerating a release-N document requires evaluating release-N's *rules*.
`work-admission`'s conditions have changed since — that is the whole point of a method that
learns. Re-rendering an old release with today's logic produces a legitimately different
document, and the check reports drift that did not happen.

Worse, it is the failure mode `verify-against-the-claim` was written to prevent. That policy
pins verification to the claimed release precisely so the check does not fail continuously and
train everyone to ignore it. Pinning the *data* while letting the *code* float reintroduces the
same failure through the back door.

**The mitigation the walk reaches:** verification does not re-render. At publish, the emitted
result and the document are committed together and hashed. Verification re-runs only the
**pure, versioned** step — result → document — and compares. That step has no capability logic
in it, so it is stable across time in a way the evaluation never can be.

This narrows what `verify-published` actually proves, honestly: **a published document was not
hand-edited after the fact.** It does not prove the document still agrees with today's record —
and it must not, because an archival document is *supposed* to disagree with a record that has
moved on.

---

## Data shapes

### `read-model@v1` — the seam between a content-owning capability and projection

```kdl
// Emitted by the capability that owns the view's content. Projection consumes it
// without ever reading a field name. Adding a read model adds no renderer code —
// that is TS.260820.10 C3, and E10's falsifier stated as a test.
result "what-is-ready-to-pick-up" {
    title    "Ready to pick up"
    owner    "CAP.work-admission"
    lifetime "working"                  // working | archival
    as-of    "2026-08-20T11:04:00-05:00"
    depicts  "unreleased"               // a version string when lifetime=archival

    section "Admissible now" {
        column "Slice" "Capability" "Command"
        row "TS.260820.01" "delivery-record" "cut-slice"
    }
    section "Blocked" {
        column "Slice" "Blocking condition"
        row "TS.260820.05" "dependency TS.260820.04 is not frozen"
    }
    prose "note" from="CAP.work-admission/principle-note"   // stored text, never generated
    empty "Blocked" says="nothing is blocked"               // silence is never absence
}
```

**Constraints the walk found necessary:**

1. **Cells are values, never markup.** The moment a capability may emit Markdown into a cell,
   presentation ownership leaks back across the seam and projection becomes a passthrough that
   only pretends to own the mechanism.
2. **Prose is referenced, not inlined** — `from=` points at stored text in the record. Prose a
   human wrote is data; prose a renderer composed is presentation.
3. **Empty sections declare their own emptiness.** Otherwise a reader cannot distinguish
   "nothing is blocked" from "blocking was not computed" — which is the trust-transfer problem
   at table granularity.
4. **`depicts` is mandatory when `lifetime="archival"`.** This is `stamp-on-publish` enforced
   in the shape rather than in the renderer.

### `published-document@v1` — what gets committed at publish

```kdl
published "docs/product.md" {
    depicts     "0.8.0"
    result-hash "sha256:…"      // the emitted result, committed beside the document
    render-hash "sha256:…"      // the document as written
    renderer    "v1"            // pure result → document. Versioned, because verification re-runs it
    published-at "2026-08-20T12:00:00-05:00"
}
```

---

## Layers this path touches

| Layer | What this path needs from it |
|---|---|
| `doctrine` | the skill teaches asking before choosing, reviewing in flight, and never editing a published document |
| `enforcement` | one implementation of the admission conditions shared by view and gate; `stamp-on-publish` enforced in the result shape; verification comparing hashes, not re-evaluating |
| `harness` | verification runs on push with no human invocation |
| `docs` | the working preview (discarded) and the archival set (committed with the tag) |

The engine sits under all four as a seam, not a capability: `read-model@v1` and
`published-document@v1` are its contracts.

---

## Findings

**W1 — the seam is a typed result, not a query.** A query-based seam fires E10's falsifier
immediately, because projection would have to evaluate admission predicates to answer it. The
capability evaluates; projection renders. *Obliges: record `read-model@v1` as a seam contract.*

**W2 — E10 holds, conditionally.** Two capabilities' views render through one code path with no
per-view branch. The condition is constraint 1 above: **cells carry values, never markup.**
Relax it and the split becomes fiction while still passing C3. *Obliges: `TS.260820.10` C3 must
assert no-markup-in-cells, not merely that a new view renders.*

**W3 — working and archival are one mechanism, two lifetimes.** They differ by stamp and
destination only. Confirms the E7/E10 narrowing rather than challenging it.

**W4 — `verify-published` cannot re-render, and `TS.260820.11` step 3 is wrong.** Regenerating a
past release's document requires that release's *rules*, not just its data. Rules change, so
re-rendering reports drift that did not occur — reintroducing exactly the continuous-failure
mode `verify-against-the-claim` exists to prevent. Verification must compare committed hashes
and re-run only the pure, versioned result → document step. *Obliges: amend `TS.260820.11`, and
narrow what the check claims to prove.*

**W5 — `render --check` proves less than the ADR implied.** It proves a published document was
not hand-edited. It does not prove the document still agrees with the record — and it must not,
because an archival document is supposed to disagree with a record that has moved on. Saying so
plainly is better than a check whose scope nobody can state. *Obliges: state the narrowed claim
wherever verification is described.*

**W6 — the storm's `verify-published` command has the wrong `asks`.** It reads *"does the
published tree still match what the record would produce for the release it claims?"* Per W4
that question is unanswerable. *Obliges: amend ES.260819.01.*

---

## Correction — W4's mitigation superseded (2026-08-20)

W4's **diagnosis** stands: a past release cannot be re-rendered, because rules change.

Its **mitigation** — commit the emitted result, hash it, version the renderer — is withdrawn.
It built a second tamper-evidence system next to a content-addressed one.

The walk also argued for one path per document, and that was wrong. Under one-path-per-document
a past release exists only in history, so comparing the working tree against it is meaningless:
HEAD holds a *different* release at the same path. Under **versioned paths**
(`docs/releases/<version>/`) every release coexists in HEAD and each is **independently**
checkable against the commit its release index names.

So verification is a git comparison, per release directory, and never a re-render. W5's
narrowing survives intact — the check proves a published document was not hand-edited, and does
not prove it still agrees with a record that has moved on.

Recorded as `ES.260819.01` **E20** (layout) and **E21** (the release index is a pointer into
git, never a copy of it — a mirror would be a second clock, and the storm already declares git
as the only clock the record has).
