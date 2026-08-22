# The decisions that shaped this

what was decided, what were the alternatives, and what tested it?

> Depicts **0.8.0**, and nothing else. Regenerated whole from
> the record; never edited in place.

## what was decided

| decision | forced by | chose | would be shown wrong by |
| --- | --- | --- | --- |
| the KDL parser | ITER.260820.01 | kdl-rs, as a document model. Not knuffel, and not any derive-based deserializer | a derive-based codec that can validate against a schema loaded at RUNTIME. If one exists, the disqualifying property was never derive and this decision was about maturity, which is a different argument with a different answer |
| where the shape check lives | ITER.260820.01 | a praxis-core validator over a parsed document, never a codec-level refusal | a codec-level refusal that names the MISSING FIELD rather than the syntax error. If a codec can say `missing realizes`, the layering here is one indirection nobody needed |
| a delivered slice is not marked delivered | ITER.260821.03 | leave TS.260820.04 with no `state`, and let the derivation from its iterations stand | a reader who needs the stored state because deriving it is too slow, or a case where the derivation is genuinely ambiguous and the record has to say which reading it meant |
| the machine gate implements half of the doctrine, and says so | ITER.260821.04 | `pick-up` writes an iteration in state `open` carrying the CREATE vet only | a `start` command landing and the second vet turning out to add nothing — if it never catches a tree that moved, the two-stage vet was ceremony and one command should do both |
| the seal covers the body and not the amendments | ITER.260821.15 | compute the seal over title, chose, over, because and falsified-by only | an amendment that contradicts the body rather than extending it. If that happens the split is wrong, because a contradiction would pass unsealed |
| prose written before its truth ships is named, not dropped | ITER.260821.16 | a `written, not yet shipped` section listing usage whose release has not promoted it | that section growing large enough that nobody reads it — at which point it is a backlog wearing a document's clothes, and belongs somewhere that tracks backlogs |
| the unverifiable claim is withdrawn, not grandfathered | ITER.260821.17 | move S1 to `present` and record previously-claimed and withdrawn-because | a frame where withdrawing pre-record claims loses so much true history that readers start keeping it somewhere else — at which point the record has made itself less useful than the prose it replaced |
| what a persona serves | ITER.260822.04 | anchor the three personas to FRAME.260819.01, the weakest anchor in the vocabulary | a fourth persona, or a persona that needs to say something the frame cannot hold — at which point three surfaces pointing at one frame stops being an argument and becomes the dumping ground it currently resembles |

## what it rejected

| decision | alternative |
| --- | --- |
| the KDL parser | knuffel · a maintained derive-based successor · serde with a generic value model · a hand-rolled parser |
| where the shape check lives | a codec-level refusal · a two-stage parse that types the document as it reads it |
| a delivered slice is not marked delivered | writing `state "delivered"` on it, as the three slices before it carry |
| the machine gate implements half of the doctrine, and says so | writing it in state `working` with both vets recorded at the same moment |
| the seal covers the body and not the amendments | sealing the whole node, amendments included |
| prose written before its truth ships is named, not dropped | omitting it, which is what `only publish what shipped` literally implies |
| the unverifiable claim is withdrawn, not grandfathered | a grandfather clause exempting claims made before the delivery graph existed |
| what a persona serves | a `persona` entity kind, so a persona could serve the phases it owns |

## what tested it

| decision | finding | from |
| --- | --- | --- |
| the machine gate implements half of the doctrine, and says so | T3 | ITER.260821.04 |

## corrections

_Nothing here — nothing recorded — no decision has been amended._

