# Plan — TEACH Phase (User Docs from the Capability Record)

**Status:** Approved (decisions settled; implementing)
**Author:** Product Designer (owner) with Principal Engineer (technical-accuracy review)
**Scope:** Make TEACH a real phase in the build-measure-learn loop — turn the validated capability record into user-facing teaching — instead of the current documented-hook-only state.

---

## 1. Problem statement

The architecture artifact model ends at LEARN: the capability record (`docs/architecture/<capability>/`) is the durable truth, and `close-sprint` promotes validated learning into it. The record is already written to be legible to a docs author and is *named* as the upstream source for user docs — but nothing consumes it. TEACH is a promise, not a phase:

- No doc home for user-facing guides.
- No skill to author or refresh them.
- No trigger that produces them.
- Not represented in the lifecycle map, dashboard, or scaffolding.

## 2. The model — two altitudes (mirrors the architecture split)

Cross-capability teaching content has no per-capability home, exactly as cross-capability topology had none. So TEACH mirrors the architecture altitudes:

| Altitude | Artifact | Source | Home |
|---|---|---|---|
| Capability guide | Concepts + how-tos for one capability | `docs/architecture/<capability>/` (technical truth) + wave `product-design.md` (user voice) | `docs/guides/<capability>/` |
| Journey tutorial | End-to-end walkthrough spanning capabilities | wave `product-design.md` journeys + multiple capability records + system overview | `docs/guides/tutorials/` |

Organizing framework: **Diátaxis**, full four quadrants — **tutorial** (learning), **how-to** (task), **reference** (information), **explanation** (understanding). Reference is manually curated from the capability record's contracts/seams now; auto-generation from seam contracts is a future hook, out of scope here.

**Genre boundary (anti-duplication):** the capability record is *engineer-facing source* (how it is built + observable behavior); the guide is *user-facing rendering* (task/concept in user language). The guide derives from and links to the record — it never restates mechanics. Record drifts → guide is re-derived, not annotated.

## 3. Ownership & trigger

- **Owner:** Product Designer authors (authoritative voice of the user). Principal Engineer reviews technical accuracy. No new persona.
- **Trigger:** `close-sprint` gains a TEACH step that fires **only when user-observable behavior changed** — pure refactors produce no doc churn. Wave success criteria adds "user guides updated." Teaching rides *validated* behavior, never hypotheses.

## 4. Freshness — soft gate, no mechanical probe

Prose cannot be diffed against behavior; a `check-docs-*.sh` script would give false confidence. Enforcement is a soft gate:

- Guide front-matter carries `source: docs/architecture/<capability>/` and a `last-validated` note.
- `close-sprint` re-derives the guide on user-observable change (checklist item + anti-pattern for a stale guide).

No new enforcement script.

## 5. Deliverables

**New skill** — `skills/author-user-docs/SKILL.md` (product-designer-owned): reads a capability record (and, for tutorials, wave journeys), emits/refreshes Diátaxis-structured guides in `docs/guides/`. Source-anchored, present-tense, same tone rules as the record (intent/current-state, no sprint history).

**Ripple (mirror of the architecture sweep):**

| File | Change |
|---|---|
| `skills/author-user-docs/SKILL.md` | New skill |
| `skills/close-sprint/SKILL.md` | TEACH step + checklist item + stale-guide anti-pattern (fires on user-observable change) |
| `skills/create-wave/SKILL.md` | Success criteria includes user-guide update; pairing note |
| `skills/using-praxis/SKILL.md` | Add `author-user-docs` to the lean-delivery skill index + a lifecycle line |
| `instructions/lean-delivery-guardrails.instructions.md` | `applyTo` adds `docs/guides/**` |
| `skills/bootstrap-project/SKILL.md` | Scaffold `docs/guides/` with a capability-guide stub |
| `skills/provision-project-overlay/SKILL.md` | Add `paths.guides` (`docs/guides`) to the interview table |
| `skills/provision-project-overlay/templates/praxis.config.yaml.tmpl` | Add `guides:` path |
| `skills/provision-project-overlay/manifest.yaml` + overlay template | `author-user-docs` overlay skill |
| `README.md` | Catalog row for the new skill |
| `/memories/repo/` | Record the TEACH altitudes |

## 6. Pre-mortem guardrails

- **Drift:** front-matter `source:` anchor + close-sprint re-derivation on user-observable change. Prose freshness is a human gate by design.
- **Orphaned journeys:** tutorials owned by the Product Designer, sourced from wave journeys — cross-capability guides have a home and an owner.
- **Churn:** trigger gated on user-observable behavior change only.
- **Duplication:** strict genre boundary — record is source, guide is rendering.

## 7. Out of scope

Auto-generated API reference from seam contracts; a docs-site/build pipeline; localization. Future hooks only.

## 8. Settled decisions

1. Doc home: `docs/guides/`.
2. Diátaxis depth: full four quadrants.
3. Freshness: soft gate only (no script).
4. Skill name: `author-user-docs`.
5. Owner: Product Designer.
