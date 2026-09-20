---
name: declare-an-invariant
description: >
  Declare what this plugin guarantees about code it is loaded into, and anchor the probe that keeps
  it. Teaches why "enabled" and "enforced" are different facts, why omitting is a binding rather
  than a gap, and why a probe's severity is the only thing an adopter actually needs from it. Runs
  whenever a probe is written or a guarantee is claimed.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, replace_string_in_file, run_in_terminal]
---

# Skill: Declare an Invariant (`skills/declare-an-invariant/`)

**Audience:** Whoever writes a probe, or claims a guarantee. **When:** in the same edit.

```
praxis check-invariants [--root praxis]
```

---

## The thing an invariant is

**An invariant is what a probe is FOR. The probe is how.**

Nothing connected the two until `TS.260821.05`. `praxis/config.kdl` had said
`enable-all-fail-closed #true` since the binding was written, and omitted three probes by name.
Neither the enabling nor the omitting referred to anything the record held — the ids were
strings. What actually enforced them was ten shell scripts and their header comments, and no
version of this plugin could state what it guaranteed without somebody reading all ten.

That is why twenty-five doctrine surfaces had no anchor after `TS.260821.04`. Not because they
were stale — because the record had no word for what they did.

---

## What to write

```kdl
invariant "no-sleep-waits" {
    protects "no source path waits for correctness by sleeping"
    severity "refuse"
    language "typescript"
    language "python"
}
```

- **`protects`** — the guarantee, in a sentence. A guarantee that cannot be stated in one is not
  a guarantee.
- **`severity`** — `refuse` or `report`. **This is the field that matters most to an adopter.**
  The difference between a gate and a notice is the whole of what they need to know, and
  describing a probe as stronger than it is was already a named failure in this repository's own
  README.
- **`language`** — which languages the probe actually covers. Static enforcement is best-effort
  per language, and *which* languages is a fact about the probe rather than a hope.
- **`structural`** — set this instead of listing languages when the probe reads structure rather
  than text. `check-port-adapter-parity.sh` parses; it applies to every language its parser
  handles, and saying so is a different claim from listing nine.

Then anchor the probe to it (`anchor-a-doctrine-surface`):

```kdl
doctrine-surface "surface.probe.check-no-sleep-waits" {
    path "scripts/check-no-sleep-waits.sh"
    kind "probe"
    serves "no-sleep-waits"
}
```

---

## Three states, and only one of them is a problem

| State | What it means | Reported? |
|---|---|---|
| **Kept** | Enabled, and a shipped surface enforces it | No |
| **Omitted** | This repository's profile says it does not apply — Praxis has no HTTP surface | **No.** Omitting is a binding, not a gap |
| **Unkept** | Enabled, and nothing the record holds enforces it | Yes — `an-enabled-invariant-is-enforced` |

Keeping omitted and unkept apart is the point. If a profile that has answered the question gets
reported anyway, adopters learn to ignore the report — which is how a rule stops being read.

The rule **reports** rather than refuses: an invariant declared before its probe is written is a
legitimate order of work. What is not legitimate is nobody knowing which.

---

## Config may bind. Config may never declare.

`praxis/config.kdl` said exactly that in a comment for the whole life of the binding, and nothing
could tell omitting an invariant from inventing one. `profile.omit-probe` now **references**
`invariant`, so `dangling-relationship` refuses an id the record does not hold.

That is the general shape: a sentence in a comment becomes enforceable by being an edge.

---

## What this does not tell you

That the probe **works**. `praxis check-invariants` proves the guarantee has a keeper, never that
the keeper keeps it — which is what `witness-a-rule` is for, one level down. A probe anchored to
an invariant and silently broken looks exactly like a working one from here.

---

## Related

- `anchor-a-doctrine-surface` — the probe's half of the anchor
- `witness-a-rule` — proving the enforcer can actually fire
- `declare-an-entity-kind` — `invariant` and `profile` are kinds, declared on the schema
