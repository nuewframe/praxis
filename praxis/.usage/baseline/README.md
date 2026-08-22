# Baseline — what the published set looked like before

The four documents `0.8.0` froze when it was first cut, kept so that "we improved it" is a
comparison rather than an assertion.

`0.8.0-withdrawn-cut/` is the published set as it stood at commit `8ddfd00`, before the cut
was withdrawn (see `praxis/releases/REL.0.8.0.kdl` for why). Nothing regenerates it. It is
not a record and not a projection — it is evidence.

## What it is a baseline for

`TS.260821.12` claims the published set becomes worth reading. That claim is settled by tests,
and tests assert that a sentence is present. They cannot say whether the document is better.
This can:

```
diff -r praxis/.usage/baseline/0.8.0-withdrawn-cut docs/releases/0.8.0
```

## What the baseline actually contained

| Document | Lines | What a reader got |
|---|---|---|
| `release-notes.md` | 170 | a table of `ITER.x · TS.y — slug · slice-outcome-added`, with the last column identical on all 27 rows |
| `capabilities/…` | 40 | seven capability ids and an event count each |
| `decisions/…` | 90 | the decisions, which is the one document that read well |
| `guides/…` | 26 | *"this release promoted no capability's truth, so there is no shipped surface to describe"* — false when written, and twelve `usage` lines sat unused |

Not one of them said what the product **does**.

## The test to apply to the replacement

Hand it to somebody who has read nothing else. If they cannot say what this is for, what it
can do, and how to start, the new set has not earned the withdrawal.
