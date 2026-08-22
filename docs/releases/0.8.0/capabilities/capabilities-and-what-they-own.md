# Capabilities and what they own

what must the system be able to do, and which events does each keep consistent?

> Depicts **0.8.0**, and nothing else. Regenerated whole from
> the record; never edited in place.

## what the product can do

| capability | derived from | events owned |
| --- | --- | --- |
| conformance-probes | evidence-and-settlement | 2 |
| multi-harness-distribution | projection | 3 |

## how it is built

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
| conformance-probes | InvariantEnforced |
| conformance-probes | InvariantUnenforced |
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
| multi-harness-distribution | RouterInjected |
| multi-harness-distribution | ManifestPublished |
| multi-harness-distribution | OverlayProvisioned |
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

## In detail

### depicts

0.8.0

