---
name: bind-work-to-a-version
description: >
  Attach closed iterations to a version so what it contains is a fact about the record rather than a
  hand-kept list. Teaches why only closed work binds, why an iteration binds exactly once, how the bump
  is proposed from configured rules, and why binding is reversible until cut. Runs in RELEASE.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, run_in_terminal]
---

# Skill: Bind Work to a Version (`skills/bind-work-to-a-version/`)

**Audience:** Maintainer. **Phase:** `RELEASE`, before cut.

```
praxis bind <ITER-ID> <VERSION> [--undo]
```

---

## The rule

**What a version contains is derived, never kept alongside.**

A hand-maintained release list and a derived one look identical right up until they disagree — and
afterwards nobody can tell which was right. So the release record is **machine-owned**: every field in
it comes from what was bound, and the file is composed whole on each bind rather than edited. There is
nothing in it a human wrote that a rewrite could lose.

Three refusals:

| Refused | Why |
| --- | --- |
| an iteration that has not closed | a version whose content includes work still in flight has no definite content — and the refusal names the unsettled claims |
| an iteration already bound elsewhere | the same work must not be counted in two releases |
| any change to a **cut** release | binding is reversible until the release is cut; cutting is not |

---

## Declare what kind of change it was

The bump is proposed from `config.kdl`'s `bump-proposal` rules, matched against each iteration's own
`contributes`:

```kdl
contributes "slice-outcome-added" \
    because="close-an-iteration, and no-silent-drop enforced"
```

The iteration declares this, not the tool. **A bump inferred from a diff is a guess about intent** —
only whoever did the work knows whether it added an outcome, remediated a gap, or broke a seam.

An iteration that declares nothing still binds. It appears in the release as
`proposal-incomplete-for`, because a proposal computed from half the work is not a proposal, and the
gap has to be visible rather than averaged away.

---

## The mapping is yours, not semver's

Which change kind maps to which position is a **binding**, read from `config.kdl`. Praxis is pre-1.0
and bumps the **minor** position for a breaking seam contract. A tool that hardcoded semver would
misreport its own author's releases.

What the engine supplies is the *ranking* between positions — that is what naming a scheme means. A
scheme it does not know proposes nothing rather than guessing.

---

## The record proposes; you decide

`proposed-bump` is a proposal with its reason and the iteration that decided it. Confirming the version
is the maintainer's, at cut. Nothing here chooses a version number.

---

## Unbinding

`--undo` while the release is `planned`. After it is cut, refused — and the emptiness of a release with
nothing bound is *stated* in the record rather than left blank, because a version with no content and a
version nobody has computed are not the same thing.

Related: `close-an-iteration` · `see-what-is-ready`.
