# What this plugin ships

what doctrine did this version ship, what asked for each piece, and which guarantees fail closed?

> Depicts **0.8.0**, and nothing else. Regenerated whole from
> the record; never edited in place.

## doctrine shipped

| surface | kind | serves | shipped in |
| --- | --- | --- | --- |
| skills/using-praxis/SKILL.md | skill | multi-harness-distribution | not derivable from what it serves |
| skills/event-storming/SKILL.md | skill | ES.260819.01 | not derivable from what it serves |
| skills/ask-what-is-true/SKILL.md | skill | TS.260820.03 | 0.8.0 |
| skills/name-a-capability/SKILL.md | skill | TS.260820.02 | 0.8.0 |
| skills/cut-a-slice/SKILL.md | skill | TS.260820.01 | 0.8.0 |
| skills/see-what-is-ready/SKILL.md | skill | TS.260820.04 | 0.8.0 |
| skills/pick-up-a-slice/SKILL.md | skill | TS.260820.05 | 0.8.0 |
| skills/attach-evidence/SKILL.md | skill | TS.260820.06 | 0.8.0 |
| skills/close-an-iteration/SKILL.md | skill | TS.260820.07 | 0.8.0 |
| skills/review-in-flight/SKILL.md | skill | TS.260820.08 | 0.8.0 |
| skills/cut-the-version/SKILL.md | skill | TS.260820.09 | 0.8.0 |
| skills/publish-the-release-set/SKILL.md | skill | TS.260820.10 | 0.8.0 |
| skills/verify-the-published-tree/SKILL.md | skill | TS.260820.11 | 0.8.0 |
| skills/declare-a-lifetime/SKILL.md | skill | TS.260820.12 | 0.8.0 |
| skills/see-the-dashboard/SKILL.md | skill | TS.260820.13 | 0.8.0 |
| skills/record-a-decision/SKILL.md | skill | TS.260820.14 | 0.8.0 |
| skills/document-usage/SKILL.md | skill | TS.260820.15 | 0.8.0 |
| skills/bind-work-to-a-version/SKILL.md | skill | TS.260820.16 | 0.8.0 |
| skills/promote-what-shipped/SKILL.md | skill | TS.260820.17 | 0.8.0 |
| skills/resolve-a-symptom/SKILL.md | skill | TS.260820.18 | 0.8.0 |
| skills/declare-an-entity-kind/SKILL.md | skill | TS.260821.01 | 0.8.0 |
| skills/witness-a-rule/SKILL.md | skill | TS.260821.02 | 0.8.0 |
| skills/anchor-a-doctrine-surface/SKILL.md | skill | TS.260821.03 | 0.8.0 |
| skills/declare-an-invariant/SKILL.md | skill | TS.260821.05 | 0.8.0 |
| agents/principal-engineer.agent.md | agent | principal-engineer | not derivable from what it serves |
| agents/product-manager.agent.md | agent | product-manager | not derivable from what it serves |
| agents/product-designer.agent.md | agent | product-designer | not derivable from what it serves |
| skills/adopt-the-method/SKILL.md | skill | TS.260821.10 | 0.8.0 |
| skills/write-durable-comments/SKILL.md | skill | comments-state-meaning | not derivable from what it serves |
| skills/name-a-persona/SKILL.md | skill | TS.260821.15 | not derivable from what it serves |
| skills/ask-the-record/SKILL.md | skill | TS.260821.11 | not derivable from what it serves |
| skills/mature-a-value/SKILL.md | skill | TS.260821.18 | not derivable from what it serves |
| skills/plan-before-building/SKILL.md | skill | TS.260821.19 | not derivable from what it serves |
| skills/anchor-the-work/SKILL.md | skill | TS.260821.16 | not derivable from what it serves |
| skills/define-seam-contract/SKILL.md | skill | seam-contract-parity | not derivable from what it serves |
| skills/design-capability-layout/SKILL.md | skill | no-dumping-grounds · port-adapter-parity | not derivable from what it serves |
| skills/implement-with-defensive-patterns/SKILL.md | skill | resilient-boundary · http-observability | not derivable from what it serves |
| skills/design-system-architecture/SKILL.md | skill | remote-config-externalization · stateless-request-path · resilient-boundary | not derivable from what it serves |
| skills/refactor-layered-to-capability/SKILL.md | skill | no-dumping-grounds | not derivable from what it serves |
| skills/test-by-ownership/SKILL.md | skill | no-skipped-tests | not derivable from what it serves |
| skills/prepare-project-for-ast/SKILL.md | skill | port-adapter-parity · seam-contract-parity | not derivable from what it serves |
| skills/bootstrap-project/SKILL.md | skill | multi-harness-distribution | not derivable from what it serves |
| skills/provision-project-overlay/SKILL.md | skill | multi-harness-distribution | not derivable from what it serves |
| skills/ingest-operational-feedback/SKILL.md | skill | conformance-probes | not derivable from what it serves |
| scripts/check-anti-dumping.sh | probe | no-dumping-grounds | not derivable from what it serves |
| scripts/check-no-skipped-tests.sh | probe | no-skipped-tests | not derivable from what it serves |
| scripts/check-no-sleep-waits.sh | probe | no-sleep-waits | not derivable from what it serves |
| scripts/check-config-externalized.sh | probe | remote-config-externalization | not derivable from what it serves |
| scripts/check-observability-at-seams.sh | probe | http-observability | not derivable from what it serves |
| scripts/check-stateless-request-path.sh | probe | stateless-request-path | not derivable from what it serves |
| scripts/check-resilient-boundary.sh | probe | resilient-boundary | not derivable from what it serves |
| scripts/check-port-adapter-parity.sh | probe | port-adapter-parity | not derivable from what it serves |
| scripts/check-seam-contract-parity.sh | probe | seam-contract-parity | not derivable from what it serves |
| scripts/check-escape-hatch-usage.sh | probe | escape-hatch-visibility | not derivable from what it serves |
| instructions/capability-driven-guardrails.instructions.md | guardrail | no-dumping-grounds · resilient-boundary · http-observability · port-adapter-parity | not derivable from what it serves |

## what it guarantees

| invariant | severity | kept by | languages |
| --- | --- | --- | --- |
| no-dumping-grounds | fails closed | scripts/check-anti-dumping.sh | every language — it reads structure, not text |
| no-skipped-tests | fails closed | scripts/check-no-skipped-tests.sh | typescript · javascript · java · kotlin · python · go · ruby · csharp · php · rust |
| no-sleep-waits | fails closed | scripts/check-no-sleep-waits.sh | typescript · javascript · java · kotlin · python · go · ruby · csharp · php · rust |
| remote-config-externalization | fails closed | scripts/check-config-externalized.sh | typescript · javascript · java · kotlin · python · go · ruby · csharp · php · rust |
| http-observability | fails closed | scripts/check-observability-at-seams.sh | typescript · javascript · java · kotlin · python · go · ruby · csharp · php · rust |
| stateless-request-path | fails closed | scripts/check-stateless-request-path.sh | typescript · javascript · java · kotlin · python · go · ruby · csharp · php · rust |
| resilient-boundary | fails closed | scripts/check-resilient-boundary.sh | typescript · javascript · java · kotlin · python · go · ruby · csharp · php · rust |
| port-adapter-parity | fails closed | scripts/check-port-adapter-parity.sh | every language — it reads structure, not text |
| seam-contract-parity | fails closed | scripts/check-seam-contract-parity.sh | every language — it reads structure, not text |
| escape-hatch-visibility | reports | scripts/check-escape-hatch-usage.sh | every language — it reads structure, not text |
| comments-state-meaning | reports | nothing — this guarantee has no keeper | not declared |

## retired, and where it still stands

| surface | stands at |
| --- | --- |
| scripts/check-contract-freshness.sh | v0.7.1 |
| skills/create-wave/SKILL.md | v0.7.1 |
| skills/create-initiative/SKILL.md | v0.7.1 |
| skills/derive-waves-from-history/SKILL.md | v0.7.1 |
| skills/start-thin-slice/SKILL.md | v0.7.1 |
| skills/create-sprint/SKILL.md | v0.7.1 |
| skills/close-sprint/SKILL.md | v0.7.1 |
| skills/intake-code-contribution/SKILL.md | v0.7.1 |
| skills/verify-and-assemble-pr/SKILL.md | v0.7.1 |
| skills/create-adr/SKILL.md | v0.7.1 |
| skills/create-capability-record/SKILL.md | v0.7.1 |
| skills/create-product-design-spec/SKILL.md | v0.7.1 |
| skills/create-product-architecture-spec/SKILL.md | v0.7.1 |
| skills/create-quality-spec/SKILL.md | v0.7.1 |
| skills/discovery-and-ambiguity-log/SKILL.md | v0.7.1 |
| skills/author-user-docs/SKILL.md | v0.7.1 |
| instructions/lean-delivery-guardrails.instructions.md | v0.7.1 |
| instructions/code-contribution-intake.instructions.md | v0.7.1 |
| scripts/check-sprint-disjointness.sh | v0.7.1 |
| scripts/check-sprint-id-collision.sh | v0.7.1 |
| scripts/check-design-approval-gate.sh | v0.7.1 |
| docs/capabilities/CAP.method-spine-and-execution.md | v0.7.1 |
| docs/capabilities/CAP.multi-harness-distribution.md | v0.7.1 |
| docs/capabilities/CAP.plugin-conformance-and-validation-probes.md | v0.7.1 |
| scripts/gen-coverage-matrix.sh | v0.7.1 |
| scripts/gen-doctrine-index.sh | v0.7.1 |

## not covered by this answer

| question | ask instead |
| --- | --- |
| whether a probe WORKS | `praxis prove` — this says a guarantee has a keeper, never that the keeper keeps it |
| surfaces whose `shipped in` reads `not derivable` | they serve an invariant, a capability or the frame rather than a slice, and no chain runs from those to a release. Storing a version on the surface would fix the column and break the fact |

## In detail

### depicts

0.8.0

