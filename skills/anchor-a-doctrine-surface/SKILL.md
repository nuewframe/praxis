---
name: anchor-a-doctrine-surface
description: >
  Declare what a shipped skill, guardrail, agent, or probe exists to serve, in the same edit that
  ships it. Teaches why instruction with no anchor is improvisation the method mandated, why the
  two directions of the rule have different severities, and how to retire a surface rather than
  delete one. Runs whenever a file is added to skills/, instructions/, agents/, or scripts/check-*.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, replace_string_in_file, run_in_terminal]
---

# Skill: Anchor a Doctrine Surface (`skills/anchor-a-doctrine-surface/`)

**Audience:** Whoever ships instruction. **When:** in the same edit that ships it.

```
praxis audit-surfaces [--root praxis] [--from .]
```

---

## The rule about instruction

**A skill nobody can trace to a capability, a slice, or a read model is doctrine an agent follows
on the plugin's authority alone.**

That is `S1` — *agents improvise structure instead of following a method* — with the method's own
name on it. The agent is not improvising against the doctrine; it is following doctrine that
stopped corresponding to anything, which is worse, because it looks like compliance.

This plugin learned it the slow way. Twenty of its slices delivered twenty skills. Forty-five
other shipped surfaces described a workflow that had been replaced eighteen iterations earlier,
and **nothing objected** — not because the check was weak, but because the record held no node for
a shipped file at all. There were no dangling edges because there were no edges.

---

## What to write

One `doctrine-surface` per shipped file, filed under the state root:

```kdl
doctrine-surface "surface.skill.cut-a-slice" {
    path "skills/cut-a-slice/SKILL.md"
    kind "skill"
    serves "TS.260820.01"
}
```

- **`path`** — the file, from the tree root. The anchor is to a file, so an absent file is a broken
  promise rather than an untidy record.
- **`kind`** — `skill` · `guardrail` · `agent` · `probe`.
- **`serves`** — what this instruction exists **to produce or operate**. A slice, a capability, a
  read model, a frame, or an event storm. One or more; never none.

`serves` is a reference, so a surface whose reason was withdrawn goes dangling on its own. That is
deliberate: it means the record needs no second rule for *"is this skill still justified?"* — the
question is the same question as *"does the thing it serves still exist?"*

---

## The two directions are not symmetric

| Direction | Rule | Severity | Why |
|---|---|---|---|
| Declared, does not ship | `a-declared-surface-ships` | **Refuse** | The record promised a file on the plugin's behalf and cannot keep it. This can never be legitimate work-in-progress. |
| Ships, nobody declared | `every-shipped-surface-is-anchored` | **Report** | On the day the rule landed it named forty-five files. A rule that fails closed on its first run is one nobody can adopt. |

The second severity is **temporary and load-bearing**. `TS.260821.04` flips it to refuse when the
count reaches zero. If you are reading this and the count is not zero, the flip has not happened
and the report is still the honest answer.

---

## Retiring, not deleting

A surface that is no longer justified does not vanish from the record. It is **retired**:

```kdl
doctrine-surface "surface.retired.author-user-docs" {
    path "skills/author-user-docs/SKILL.md"
    kind "skill"
    serves "TS.260821.04"
    state "retired"
    retired-at "2026-08-22"
    stands-at "v0.7.1"
}
```

`stands-at` names the tag where the file still stands, so removal is recoverable **by name**. That
is what licenses removing it outright instead of leaving a shim: two spines loaded in one plugin
means an arriving agent reads two methods and picks one, which is the symptom this whole rule
attacks.

A retired surface whose file is **still in the tree** refuses. The retirement was recorded and
never carried out, which is the exact failure mode the retirement existed to fix.

---

## What this does not tell you

The audit says the record asks for the skill. It says **nothing** about whether the skill teaches
what the record means. A surface can be perfectly anchored and completely out of date — that is
what its slice's `docs` layer is for, and conflating the two turns a cleanup into a quarter.

---

## Related

- `declare-an-entity-kind` — `doctrine-surface` is a kind, declared on the schema like any other
- `witness-a-rule` — both surface rules carry witnesses; `every-shipped-surface-is-anchored` is the
  first rule in this schema whose witness declares a **world** (`given-shipped`) as well as a record
- `cut-a-slice` — a slice declaring a `doctrine` layer is a slice that will ship a surface
