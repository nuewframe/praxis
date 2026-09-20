---
name: publish-the-release-set
description: >
  Regenerate every published document whole for one version, before it is cut. Teaches why a published
  document is never edited toward a new state, why an archival document must carry no wall-clock stamp,
  and why publishing precedes the cut. Runs in RELEASE, between binding and cutting.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, run_in_terminal]
---

# Skill: Publish the Release Set (`skills/publish-the-release-set/`)

**Audience:** Maintainer. **Phase:** `RELEASE`, after binding and **before** the cut.

```
praxis publish <VERSION> [--dry-run]
```

---

## The rule

**Every document is generated afresh and whole. Nothing is ever spliced.**

A document that is spliced can be corrupted; a document that is regenerated cannot. There is no
search-and-replace path in this command, at any point, for any reason — and there should never be one,
because the moment a substitution exists, "regenerated whole" becomes a claim rather than a property.

Each document lands under `docs/releases/<version>/` and depicts **exactly that version**. A later
release publishes a new document; it never rewrites this one.

---

## Publish *before* you cut

This ordering was forced by building, not chosen for taste. `TS.260820.09` requires the release index's
commit to **contain** this release's published directory — and a commit taken at cut time cannot
contain documents written afterwards. So:

```
praxis bind …    →  praxis publish <V>  →  git commit  →  praxis cut-release <V> --confirm
```

`cut-release` refuses until `docs/releases/<version>/` exists **and is committed**. The containment is
true by construction rather than by hoping the steps ran in the right order.

Publishing for a release that is already cut is refused for the same reason.

---

## An archival document carries no clock

There is no "generated at" line, and the composing function takes no timestamp at all. This looks like
a small thing and is not:

- verification of a published tree is a **comparison** (`TS.260820.11`);
- a wall-clock reading anywhere in a document makes every re-render differ from the last;
- so a document stamped with the moment can never be verified against a fresh render.

The working views do the opposite — `praxis ready` stamps the moment, because *"right now"* is the
whole of its question. Same result type, opposite requirement, and `publishable` is what decides.

---

## The renderer must not know what a view means

Rendering takes a `read-model@v1` result and nothing else. There are two renderers over that one type —
terminal and Markdown — and **no branch on a view's name in either**. That is finding `E10`'s falsifier
held as an acceptance criterion: if projection ever needs a code path specific to one view, this
capability and the content-owning ones merge and the split was fiction.

Both halves matter. A renderer with no per-view branch can still be fake if a capability emits markup
in a cell, so cells carry values and the check rejects markup in any of them.

Related: `bind-work-to-a-version` · `cut-the-version` · `see-what-is-ready`.
