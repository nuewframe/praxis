---
name: witness-a-rule
description: >
  Declare a rule together with the record that demonstrates it refusing. Teaches why a witness lives in
  the record rather than in the engine, why "never fired" and "cannot fire" must read differently, and
  how to read the unwitnessed set. Runs whenever a rule is declared.
user-invocable: true
disable-model-invocation: false
tools: [read_file, file_search, grep_search, replace_string_in_file, run_in_terminal]
---

# Skill: Witness a Rule (`skills/witness-a-rule/`)

**Audience:** Whoever declares a rule. **When:** in the same edit that declares it.

```
praxis prove [root]
```

---

## The rule about rules

**A rule that has never been shown to refuse is indistinguishable from one that cannot.**

That is `S3` turned on the enforcement layer itself: a gate that was never invoked and a gate that
refused look identical afterwards — and so do a rule that cannot fire and a rule with nothing to fire
at. This frame learned it expensively. A correct rule reported nothing because its refusals were
computed and silently dropped, and it was caught only because the answer had been **predicted**.

So declaring a rule includes writing the record that violates it:

```kdl
rule "a-decision-names-its-falsifier" refuses="a decision naming nothing that would show it wrong" \
    witness=#"""
        iteration "ITER.witness" {
            on-slice "TS.witness"
            state "closed"
            decision "a preference wearing a decision's clothes" chose="this" over="that"
        }
        """#
```

---

## The witness lives in the record

Not in Rust. **A witness written in the engine is a second implementation of the same check**, and two
implementations agreeing proves only that one person wrote both. A witness written in the record is
evidence: the record declares the rule *and* demonstrates it.

The prover runs your witness through **the same functions that check the tree**. A witness checked by a
different path would prove that path works and nothing about the real one — which is exactly how the
dropped-refusal bug survived.

---

## Three ways to have no witness, and they read differently

| Reported as | Means |
| --- | --- |
| `the record declares no witness for it` | nobody has tried |
| `its witness is not a record the codec can read` | the witness is broken, not the rule |
| `its witness produced no refusal enforcing it` | **the rule is unimplemented, or its refusals are computed and dropped** |

The third is the one worth stopping for. It is what an unreachable rule looks like from outside.

---

## A witness must produce *this* rule's refusal

Not any refusal. A malformed record refuses for a dozen reasons, and if any of them counted, one bad
witness would witness every rule. The prover matches on the rule each refusal enforces.

**Some rules need context.** `dangling-relationship` is skipped when the corpus knows no entity of the
target kind — a guard against false positives on partial documents — so a witness declaring only the
dangling edge witnesses nothing. Give the witness the context it needs; it is a record, not a fragment.

---

## Reading the unwitnessed set

`praxis prove` reports it and **exits zero**. A rule declared today for a shape the record does not
hold yet is legitimate. What is not legitimate is nobody knowing which — so it reports rather than
refuses, and the number is the point.

When this was first run against its own repository, the answer was **13 of 40**.

Related: `declare-an-entity-kind` · `record-a-decision` · `cut-a-slice`.
