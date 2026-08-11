---
name: derive-waves-from-history
description: Derive a wave structure for a product that already shipped, from the record it already has — release history, ADRs, capability records, commit log — without inventing the acceptance criteria and hypotheses that were never written. Use when adopting Praxis on an existing product rather than scaffolding a new one. Produces derived waves for delivered work and full four-document waves for open work, and makes the difference visible to a reader.
user-invocable: true
disable-model-invocation: false
---

# Skill: Derive Waves From History

Use this when a product **already ships** and needs an intent map, rather than a new repository needing scaffolding.

Praxis's other two entry points do not cover this. `bootstrap-project` is greenfield. `refactor-layered-to-capability` migrates legacy **code**, not legacy **intent**. Between them sat the most common adoption path with no answer better than "keep an archive and start fresh".

The governing decision is [ADR.260725.10](../../docs/architecture/adr/ADR.260725.10-brownfield-wave-retrofit.md): **deriving an intent map from records that already exist invents nothing; rewriting a shipped plan into a wave complete with acceptance criteria it never had asserts validated learning that never happened.** Every rule below serves that line.

---

## The one failure this skill exists to prevent

A derived wave and a fabricated one **have the same shape**. Both are markdown with slice headings and evidence lines. No script can tell them apart, which is why this is a discipline and not a probe.

The pressure is real: a wave with a citation on every slice looks finished, and a wave that says "no evidence found" looks like unfinished work. **It is not.** Showing a gap is the deliverable. If following this skill ever feels like it would read better with one more citation, that is the moment it is about to go wrong.

---

## When NOT to use this

- The repository is new or nearly empty → `bootstrap-project`.
- The problem is code layout, not intent → `refactor-layered-to-capability`.
- The product's waves are already maintained and current → nothing to derive.

---

## Step 1 — Find the record before deciding anything

Inventory what actually exists, and write the inventory down. You are deriving *from* this; if it is thin, the derived waves must be thin.

- Release history: `CHANGELOG.md`, tags, release notes.
- Decisions: ADRs anywhere in the tree.
- Current-state truth: capability records, architecture READMEs.
- Delivery record: the commit log, especially messages carrying slice ids.
- Prior intent: PRDs, specs, vision documents, design docs.

**Record what is missing as much as what is present.** A product with 40 commits and no ADRs derives differently from one with four ADRs and thirteen capability records.

## Step 2 — Establish how many roots the product has

Do not assume one tree.

A product may be several generations in one repository, several packages in a monorepo, or several repositories. In each case **no single `docs/product/initiatives/` holds the product's intent**, and a dashboard inside any one of them is a partial truth presented as the whole.

- **One root** → the dashboard is `docs/product.md` and it is complete.
- **Several roots in one repository** → the dashboard sits **above** them and names each root; each root keeps its own waves.
- **Several repositories** → set `paths.product_root` per [ADR.260725.17](../../docs/architecture/adr/ADR.260725.17-context-placement-beyond-one-repo.md); a repository that is a part says so and points at the whole.

> **Observed case.** LogicKit is five successive generations in one repository — `alpha.1`, `alpha.3`, `alpha.4`, `alpha.5` — two of which carry their own waves. There is no repository-wide waves directory, and the dashboard had to sit above all of them.

## Step 3 — Derive waves from value themes, never from structure

A wave is a **coherent value theme**: an outcome a user gets. It is not a module, a package, a service, a release, or a generation.

Mapping structure to waves one-to-one produces the implementation-bucket anti-pattern `create-wave` prohibits. The tempting wrong answers are the ones the tree hands you for free.

> **Observed case.** LogicKit's five generations are the obvious wrong answer — five generations did **not** become five waves. Generations are re-approaches to the same product; the value themes run *across* them.

Ask of each candidate: *what could a user do afterwards that they could not do before?* If there is no answer, it is not a wave.

## Step 4 — Classify each wave: delivered or open

This decides what documents it gets, and it is the rule most likely to be broken by good intentions.

| | **Delivered** — work is finished | **Open** — work continues |
| --- | --- | --- |
| Documents | `README.md` **only** | Full four: README, product-design, product-architecture, qa |
| Slices | Each cites its evidence | Each carries acceptance criteria |
| Hypothesis card | **None** | Yes |
| Acceptance criteria | **None** | Yes |

A delivered wave gets no hypothesis card and no acceptance criteria **because none were written at the time**. Adding them retroactively asserts validated learning that never happened. This is the whole distinction; if you take one rule from this skill, take this one.

## Step 5 — Cite evidence, or say there is none

Every slice in a delivered wave carries a citation to something that **already exists**: a release entry, an ADR, a capability-record passage, a commit.

When a candidate slice has no citable evidence, there are exactly two honest outcomes:

1. **Drop it.** It may not have happened the way memory says.
2. **Record it as undocumented prior work** — named, with the absence stated plainly.

**Never** give a slice invented evidence. Never infer a citation from resemblance.

> **Observed case.** LogicKit's waves number slices `TS-NNN`; its commits number them `ST-NNN`. `ST-006 explicit acceptance record` obviously corresponds to `TS-003: Explicit Acceptance Record` — obviously enough that mapping all nine felt like diligence. Nothing in the repository declares that mapping, so it was **not applied**; the dashboard shows the gap and names who could close it. Inference that feels obvious is the exact shape fabrication takes.

> **Observed case.** LogicKit has no `alpha.2`. Nothing says why. It is recorded as observed and unexplained, not smoothed over.

## Step 6 — Make the reconstruction visible

A reader must be able to tell a derived record from a maintained one **without being told separately**. Every derived wave and dashboard opens with a banner saying so, why it carries no criteria, and what it was reconstructed from.

Praxis's own derived waves are the worked example — see any wave marked `Delivered†` in [`docs/product.md`](../../docs/product.md).

## Step 7 — Reconcile existing wave documents against the evidence

Late adoption rarely means *no* Praxis. It usually means **some Praxis, unmaintained**. Existing wave documents may declare a status their own repository contradicts.

When declared status and evidence disagree:

- **Show both.** Record what the document declares and what the evidence says.
- **Do not silently overwrite** the declared status with your inference.
- **Do not silently keep it** either — an uncontested stale status is the drift you were brought in to surface.
- Name who can resolve it.

> **Observed case.** All three of LogicKit's `alpha.5` waves declare every slice `⚪ Not Started`, while the commit log ends at *"ST-009 executed obligations and the working API — epic complete"* and thirteen capability records exist for the capabilities those waves name. The dashboard shows the contradiction rather than resolving it, because resolving it requires the delivery history someone else holds.

## Step 8 — Write the dashboard, and let it state its own limits

Register every wave. Then say plainly what the dashboard does **not** know: which statuses are contested, which waves have no evidence either way, and what the next honest step is.

A derived dashboard makes a product visible for the first time. It does not make it current, and it should not imply that it does.

---

## Quality Checklist

- [ ] Record inventoried before any wave was named, including what is absent
- [ ] Number of roots established; dashboard placed above them if more than one
- [ ] Waves derived from value themes — no wave corresponds one-to-one to a module, package, release, or generation
- [ ] Every delivered wave carries a README only, with **no** hypothesis card and **no** acceptance criteria
- [ ] Every open wave carries all four documents
- [ ] Every delivered slice cites existing evidence, or is dropped, or is named as undocumented prior work
- [ ] No citation inferred from resemblance
- [ ] Gaps in the record recorded as observed, not explained
- [ ] Derived-record banner on every derived wave and on the dashboard
- [ ] Contradictions between declared status and evidence shown, not resolved by inference
- [ ] Dashboard states its own limits

---

## Anti-Patterns

- Giving a delivered wave acceptance criteria or a hypothesis card so it "matches the others"
- Inferring a citation because the titles line up
- One wave per module, package, service, release, or generation
- Silently correcting a stale status to what you believe is true
- Silently leaving a stale status uncontested
- Explaining a gap in the record instead of recording it
- A dashboard that reads as current when it is reconstructed
- Deriving a rich wave set from a thin record — the derived structure cannot be more certain than what it came from
