# How it fits together

what does each part do, what does it refuse to do, and what does it keep true?

> Depicts **0.8.0**, and nothing else. Regenerated whole from
> the record; never edited in place.

## what it does

| capability | does | refuses to do |
| --- | --- | --- |
| conformance-probes | keep every guarantee this plugin makes about code it is loaded into paired with something that enforces it, at a severity an adopter can read | it does not decide WHETHER a guarantee is worth making, and it never proves a probe works — only that a guarantee has a keeper |
| multi-harness-distribution | deliver one skills/agents/instructions tree to six harnesses from a single source, and inject the router at session start in each | it does not decide WHAT the doctrine says. It owns how doctrine reaches an agent, never what the agent is told |

## how it is built

| capability | does | refuses to do |
| --- | --- | --- |
| claim-settlement | settle every stated claim against evidence, or refuse to let the work close | it does not judge whether the work should have been started, and it does not produce the evidence. It refuses to let a claim close without it |
| delivery-record | hold everything the method produces — problems, discoveries, capabilities, slices and claims — as typed, queryable, trailed facts | it enforces nothing about DOING the work. It refuses a malformed fact, never a bad decision |
| document-projection | compose any declared read model as a whole document, mark what lifetime it has, and prove a published one still matches the release it claims | it never mutates the record, never edits a document toward a new state, and does not decide what any view CONTAINS — only how a view becomes a document |
| release-binding | bind finished iterations to a version, cut it, and promote what shipped into what each capability says it is | it does not write documents. It moves truth; document-projection renders it |
| work-admission | decide whether a slice may be started right now, and record the decision either way | it does not judge whether the work was done well. It judges readiness and disjointness, at one moment, and then it is finished |

## what each keeps true

| capability | invariant |
| --- | --- |
| claim-settlement | every stated claim against its evidence or its recorded shortfall |
| claim-settlement | a finding against the iteration that raised it |
| claim-settlement | a decision against the work that forced it |
| conformance-probes | every invariant the profile enables against the surfaces that enforce it |
| conformance-probes | each invariant's declared severity against what its probe actually does |
| delivery-record | the published set against the declared lifetimes it is derived from |
| delivery-record | a symptom's state against the release that resolved it |
| delivery-record | a capability's existence against the storm cluster it owns |
| delivery-record | a slice's shape against the command, actor, event and read model it must name |
| document-projection | a published release directory against the commit its release index names |
| document-projection | the published set against the lifetime declarations it is derived from |
| multi-harness-distribution | every harness manifest against the single source tree they are rendered from |
| multi-harness-distribution | the injected router against the file on disk |
| release-binding | a version's content against the iterations bound to it |
| release-binding | current truth against the last release that moved it |
| work-admission | a slice's pickup against every other iteration in flight |
| work-admission | an iteration's frozen dependencies against what it stands on |
| work-admission | a refusal against the condition that caused it |

## decisions behind it

| decision | chosen over |
| --- | --- |
| the KDL parser | knuffel · a maintained derive-based successor · serde with a generic value model · a hand-rolled parser |
| where the shape check lives | a codec-level refusal · a two-stage parse that types the document as it reads it |
| a delivered slice is not marked delivered | writing `state "delivered"` on it, as the three slices before it carry |
| the machine gate implements half of the doctrine, and says so | writing it in state `working` with both vets recorded at the same moment |
| the seal covers the body and not the amendments | sealing the whole node, amendments included |
| prose written before its truth ships is named, not dropped | omitting it, which is what `only publish what shipped` literally implies |
| the unverifiable claim is withdrawn, not grandfathered | a grandfather clause exempting claims made before the delivery graph existed |
| what a persona serves | a `persona` entity kind, so a persona could serve the phases it owns |

## Notes

**depicts** — 0.8.0

