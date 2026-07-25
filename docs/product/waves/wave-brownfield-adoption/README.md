# WAVE: Brownfield Adoption

> **Planning-stage document — an educated theory, not yet the truth.** The best approach given what we know today; current-state architecture lives in [docs/architecture/](../../../architecture/), promoted there by `close-sprint`.

**Status:** 🔄 In Progress\
**Goal:** A team whose product already exists — possibly across many packages or many repositories — can adopt the method against what they have, instead of being offered only a greenfield entry point.

---

## Tracking Rules

- Track each thin-slice by intended user outcome and current state only.
- If a thin-slice is reopened or corrected, keep the same slice ID and add one short tracking note next to that slice.
- Keep implementation history in sprint files and version control. This README stays focused on product intent.

---

## Value Theme

Almost nobody adopts a delivery method on an empty repository. They adopt it on a product that already ships, with history the method never saw, spread across a layout the method did not choose. Praxis currently answers that situation with `bootstrap-project` (greenfield) and `refactor-layered-to-capability` (legacy *code*, not legacy *intent*) — and for everything else, "keep an archive and start fresh." That is the weakest guidance in the method, and it sits directly on the most common adoption path.

---

## Scope

- A path for deriving wave structure from a product that already shipped, without fabricating the history it never recorded.
- Guidance for where context lives when one repository is not the unit of the product.

**Out of scope:**

- Migrating legacy code structure into capability slices — `refactor-layered-to-capability` already owns that, and it is a different problem from migrating intent.
- Greenfield scaffolding — `bootstrap-project` owns that.

---

## Thin-Slices

### TS-001: Adopting waves on a product that already shipped

> **Status:** ⚪ Not Started

**User Value:** As a team adopting Praxis on an existing product, I need a documented path for deriving my wave structure from what I have already built, so that I get an intent map without inventing acceptance criteria for work that shipped years ago.

**Acceptance Criteria:**

- [ ] Given an existing product with release history and architecture documentation, when the path is followed, then it produces a wave set derived from coherent value themes rather than one wave per code module
- [ ] Given a wave whose work is finished, when it is authored, then it carries a README with evidence-cited slices and no hypothesis card, acceptance criteria, or educated-theory documents
- [ ] Given a wave whose work is open, when it is authored, then it carries the full four-document set
- [ ] Given a candidate delivered slice with no citable release entry, ADR, or capability-record passage, when the path is followed, then the slice is dropped or recorded as undocumented prior work — never given invented evidence
- [ ] Given a reader encountering a derived wave, when they open it, then they can tell it was reconstructed after delivery without being told separately

**Dependencies:** None. Decided in [ADR.260725.10](../../../architecture/adr/ADR.260725.10-brownfield-wave-retrofit.md) (status: Accepted). The decision is already applied to Praxis's own tree; this slice generalizes it into a path others can follow.

**Tracking note:** The pattern has exactly one instance — Praxis's own retrofit. The evolution policy requires real-repo validation before a rule ships, and dogfooding does not substitute. This slice generalizes the pattern only after it has been applied to a project that is not Praxis.

---

### TS-002: Context placement when one repository is not the product

> **Status:** ✅ Complete

**User Value:** As a team whose product spans many packages or many repositories, I need to know where project context, waves, and architecture records live so that no single location is a partial truth presented as the whole.

**Acceptance Criteria:**

- [x] Given a monorepo of many packages, when the guidance is followed, then it is clear which context is repository-wide and which is per-package, and how an agent resolves precedence between them
- [x] Given a product spanning several repositories, when the guidance is followed, then wave and dashboard placement is defined without any one repository claiming to hold the whole product's intent
- [x] Given content that legitimately belongs in two places, when the guidance is followed, then it is generated from one source rather than hand-synced

**Dependencies:** None.

**Tracking note:** Resolved as **precedence, not generation** ([ADR.260725.17](../../../architecture/adr/ADR.260725.17-context-placement-beyond-one-repo.md)). A package tier joins the precedence stack; a package declares only what differs and inherits the rest. A product spanning repositories names where the whole lives via the optional `paths.product_root`, and a repository that is a part must declare that scope rather than presenting a fragment as the entire picture.

The third criterion is satisfied by **dissolution rather than construction**, which is worth stating plainly instead of quietly ticking: under precedence a fact lives at the level where it is true, so it never lives in two places and there is nothing to generate. The dual-home generator this slice expected to consume is therefore retired as a concept rather than deferred again — `TS-007` of [wave-self-conformance](../wave-self-conformance/README.md) had already narrowed away from it for the same reason.

**Honest limit:** read-time resolution is a *behavior*, not an artifact. Praxis can check that a package context file parses; it cannot prove an agent walked the tree. That sits in the agent-attested tier — the weakest of the three — and the mitigation is placement in the always-on summary, not a gate. The one mechanically enforced part is that a part-repository must declare its scope.

---

## Success Criteria

Wave is complete when:

- [ ] All thin-slices are ✅ Complete
- [ ] Journey tests pass for all primary scenarios — for this wave, the path has been applied end-to-end to at least one project that is not Praxis
- [ ] User guides updated (TEACH) for capabilities whose user-observable behavior changed — via `author-user-docs`
- [ ] Product dashboard updated to reflect completion

---

## Dependencies

- **Requires:** [wave-method-spine](../wave-method-spine/README.md) — there must be a wave pattern before there is a path for adopting it late.
- **Enables:** the real-repo validation the evolution policy demands. Most candidate validation projects are brownfield, so an adoption path for them is a precondition for meeting that bar honestly.
