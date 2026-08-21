# Capabilities and what they own

what must the system be able to do, and which events does each keep consistent?

> Depicts **0.8.0**, and nothing else. Regenerated whole from
> the record; never edited in place.

## capabilities

| capability | derived from | events owned |
| --- | --- | --- |
| claim-settlement | evidence-and-settlement | 7 |
| delivery-record | problem-record | 19 |
| document-projection | projection | 3 |
| release-binding | release-binding | 3 |
| work-admission | iteration-control | 7 |

## what each owns

| capability | event |
| --- | --- |
| claim-settlement | EvidenceAttached |
| claim-settlement | ClaimSettled |
| claim-settlement | FindingRaised |
| claim-settlement | FindingTriaged |
| claim-settlement | DecisionRecorded |
| claim-settlement | DecisionAccepted |
| claim-settlement | AcceptanceRefused |
| delivery-record | IntentRaised |
| delivery-record | SymptomRecorded |
| delivery-record | RootCauseConverged |
| delivery-record | ConvergenceRefused |
| delivery-record | SymptomResolved |
| delivery-record | FrameReopened |
| delivery-record | StormHeld |
| delivery-record | PathWalked |
| delivery-record | OptionRejected |
| delivery-record | CapabilityNamed |
| delivery-record | SliceCut |
| delivery-record | SliceCutRefused |
| delivery-record | ClaimStated |
| delivery-record | LifetimeDeclared |
| delivery-record | UsageDocumented |
| delivery-record | EntityDeclared |
| delivery-record | EntityDeclarationRefused |
| delivery-record | ClaimWithdrawn |
| delivery-record | RecordAmended |
| document-projection | DocumentsProjected |
| document-projection | DocumentStamped |
| document-projection | DriftDetected |
| release-binding | WorkBound |
| release-binding | ReleaseCut |
| release-binding | TruthPromoted |
| work-admission | SliceVetted |
| work-admission | PickupRefused |
| work-admission | DependencyFrozen |
| work-admission | IterationClosed |
| work-admission | CloseRefused |
| work-admission | IterationStarted |
| work-admission | StartRefused |

## Notes

**depicts** — 0.8.0

