---
name: ask-the-record
description: >
  Ask the record a question about any declared kind, instead of reaching for grep. Teaches why the
  command is generic rather than a set of named questions, what `--count` is for, how the anti-join
  follows a declared edge, and which questions it deliberately cannot answer. Runs whenever you are
  about to grep the state root.
user-invocable: true
disable-model-invocation: false
tools: [read_file, grep_search, run_in_terminal]
---

# Skill: Ask the Record (`skills/ask-the-record/`)

**Audience:** An agent mid-task, and the loop it is running in. **When:** instead of `grep`.

```
praxis ask <kind> [--where field=value] [--show a,b] [--count]
                  [--unreferenced-by kind.field]
```

---

## Why it is generic

`praxis unbound-iterations` would be easier to use and is forbidden. It encodes a kind and a
field **the record declares** — the engine enforcing a vocabulary no record handed it, which is
`A4` — and it would answer nothing in a repository whose kinds are `cohort` and `experiment`.

So the kind comes from the schema, and this command never learns what any of it means. The
same code path answers over a vocabulary this engine has never heard of, which is what makes
it worth having in a project that is not this one.

## The four shapes

```
praxis ask thin-slice --count
praxis ask capability --where facet=product --show doing,not
praxis ask iteration --where state=closed --unreferenced-by release.binds
praxis ask doctrine-surface --where kind=probe --show path,serves
```

**`--count` prints the integer and nothing else.** The usage log is why: four commands were
run twenty-five times to read four integers, and two had their framing stripped by `sed` on
the way past. This is for the loop, not the reader.

**`--unreferenced-by` is the anti-join**, and it is clean only because `references=` is in the
schema. `iteration --where state=closed --unreferenced-by release.binds` is closed work no
release binds — the exact `for f in ITER.*.kdl; do grep …` that produced this slice.

## What it refuses, and why refusing beats answering

An undeclared kind or field is **refused**, naming what is declared.

A filter on a field that does not exist matches nothing — and looks exactly like a filter that
matched nothing. The empty answer is the dangerous one, so it is never given.

The same reasoning covers a kind that exists **only nested**:

```
$ praxis ask symptom --where state=present --count
praxis: `symptom` appears only nested inside another record, never at the root.
        This answers over root records, so 0 here would mean `cannot see them`
        while reading as `none`
```

Seven symptoms exist. `0` would have been a lie with a straight face.

---

## What it deliberately cannot do

| | |
|---|---|
| **Nested entities** | `finding` lives inside an iteration, `condition` inside a refusal. Reaching them needs a path syntax this does not have — and it says so rather than answering zero. |
| **Joins beyond a declared edge** | Anything the record does not declare as an edge would be the engine guessing what relates to what. |
| **A query language** | The usage log evidences a counter, a filter and one anti-join, from a sample of one session. A DSL would be inventing requirements. |
| **Writing** | An `ask` that changed the record would be a command wearing a question's clothes. |

## Related

- `ask-what-is-true` — the composed answer, when you do not know what to ask yet
- `see-what-is-ready` — the one question with a shape no filter expresses: what blocks each slice
- `declare-an-entity-kind` — a kind you declare becomes askable with nothing else changed
