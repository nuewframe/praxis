# Capabilities and what they own

what must the system be able to do, and which events does each keep consistent?

> Depicts **0.8.0**, and nothing else. Regenerated whole from the
> record; never edited in place.

## capabilities

| capability | derived from | events owned |
| --- | --- | --- |
| claim-settlement | evidence-and-settlement | 1 |
| delivery-record | problem-record | 1 |
| document-projection | projection | 1 |
| release-binding | release-binding | 1 |
| work-admission | iteration-control | 1 |

## what each owns

| capability | event |
| --- | --- |
| claim-settlement | EvidenceAttached |
| delivery-record | IntentRaised |
| document-projection | DocumentsProjected |
| release-binding | WorkBound |
| work-admission | SliceVetted |

## Notes

**depicts** — 0.8.0

