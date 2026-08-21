# `test-run/` — journey walks and spikes

A journey walk is the method's own instrument for falsifying a model before it is compiled.
ADR.260819.02's F24 settled what a walk is: **running the blueprint is not a description of the
loop, it is an iteration of the loop**, and its output is real work. So walks are recorded here
rather than narrated in a document and discarded.

This directory is committed for three reasons: the findings are review evidence, the graphs a walk
materializes become golden-file fixtures for the Rust engine, and a walk nobody can re-read is a
walk nobody can check.

```
test-run/
├── walk-001/     Stages 1 → 2: intake through the creation of the first vertical slice
└── spike/        Throwaway instruments. Not the engine, not on the stack.
```

## What a walk is, and is not

**Is:** two altitudes run together — *product design* (what a human or agent invokes at each stage
and what comes back) and *anticipated data* (the concrete nodes and edges the stage materializes,
with real values). ADR.260819.02 found that ten of its twenty-eight findings appeared only at the
second altitude, so a walk that stays abstract under-reports by roughly half.

**Is not:** a test suite, a simulation, or a rehearsal that gets thrown away. A walk that produces
no finding has not proved the model correct; it has proved the walk shallow.

## What a walk produces

| Output | Where it lands |
| --- | --- |
| The stage record and its findings | `walk-NNN/walk.md` |
| Terminal transcripts of commands that do not exist yet | `walk-NNN/walk.md` |
| Real graph nodes the walk decided | `praxis/` — the walk creates actual work, not copies |
| Findings that change a decision | `amendment` blocks on the governing ADR |
| Findings that are work | `praxis/gaps/` |

Markdown is fine *here*. `test-run/` is not the state root: the no-Markdown rule (amendment A4)
governs `praxis/`, where every prose artifact is KDL and every `.md` in the repository is
generated. A walk record is neither state nor projection — it is the instrument's own log.

Commands shown in a transcript are **designed, not implemented**. Writing the terminal output a
stage should produce is how a stage that reads cleanly in the abstract turns out to need a command
nobody designed.
