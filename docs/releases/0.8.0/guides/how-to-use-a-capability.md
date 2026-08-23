# How to use a capability

how do I use what 0.8.0 can do?

> Depicts **0.8.0**, and nothing else. Regenerated whole from
> the record; never edited in place.

## how to use what shipped

| capability |
| --- |
| claim-settlement |
| delivery-record |
| document-projection |
| release-binding |
| work-admission |

## shipped without a guide

_Nothing here — every capability this version bound work for carries usage prose._

## written, not yet shipped

| capability | why it is not here |
| --- | --- |
| conformance-probes | 0.8.0 bound no slice realizing it |

## In detail

### claim-settlement

`praxis close <ITER> --outcome continue|pivot|stop` — every claim settled, or every shortfall carried by a finding that NAMES it, or the close is refused.

`praxis review <ITER> [--markdown]` — preview what an iteration promised against what it has shown, at any moment, writing nothing. Layers the slice declared and nothing reached are listed, not omitted.

### delivery-record

`praxis check [root]` — refuse any record that does not match the shape the record itself declares.

`praxis truth` — what this repository already knows, so an arriving agent does not re-derive it. Read the last section: it names what the answer does not cover.

Adopting on a repository that already shipped: the record is built NEW, and what is already there is discovery data. Capability docs, ADRs and release history are stormed, walked and argued with — a claim transcribed is a claim imported without its evidence, and a record that looks checked from its first commit is worth less than an empty one.

`what did this come from` is answered by the record's own derivation, never by a pointer at the document it was read from. A capability names the cluster it came from and the storm that cluster is in; a slice names its storm; a decision names what it rejected and what would show it wrong. Every one of those is an id the checker follows and refuses when it dangles. There is deliberately no `informed-by`: a path is the half guaranteed not to survive, and a dead pointer outlives the file as a confident lie.

### document-projection

`praxis publish <VERSION> [--dry-run]` — regenerate the published set whole, before the cut. Membership comes from the lifetime declarations and the paths come with it.

`praxis verify-published` — prove no published document was hand-edited, comparing each release's directory against the commit its index names.

### release-binding

`praxis bind <ITER> <VERSION> [--undo]` — attach closed work to a version. Only closed iterations bind, each binds once, and a cut release does not change.

`praxis cut-release <VERSION> --confirm` — cut the version and write its index node in one record. Refused until the published set exists and is committed.

`praxis promote <VERSION>` — fold what a cut release shipped into what each capability says it is. Derived from bound work and recomputed by a rule.

### work-admission

`praxis ready` — which slices could be started right now, and what would refuse each of the rest. Read the `not decided for you` section first: it names the conditions the gate declares and did not compute.

`praxis pick-up <SLICE>` — take a slice through the one gate. Either an iteration opens, or a refusal record is written naming the condition that failed. Selection is the commitment.

