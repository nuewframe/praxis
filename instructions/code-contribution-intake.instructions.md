---
applyTo: "**"
description: "Always-on code contribution intake gate: before implementation for user stories, features, behavior changes, or non-trivial code contributions, anchor work to initiative (INIT.<name>.md), thin-slice, sprint, current code state, and test posture."
---

# Code Contribution Intake Guardrails

Use these rules before writing or modifying implementation code for any user story, feature, thin-slice, behavior-changing contribution, or non-trivial refactor. The host project owns the final word; project-specific instructions may narrow or strengthen these rules.

## 1. Start From Product Intent

Do not treat a direct implementation request as sufficient product context.

- Identify the active initiative (`docs/product/initiatives/INIT.<initiative-name>.md`) and thin-slice (`TS-NNN`).
- Confirm the thin-slice has testable acceptance criteria in the initiative file.
- If no initiative exists, stop and use `create-wave` to initialize `INIT.<initiative-name>.md`.
- If the request changes a completed thin-slice, keep the original thin-slice ID (`TS-NNN`) and treat it as a correction or reopen.

## 2. Verify Initiative Specs

Before sprint planning or implementation, confirm the initiative file (`INIT.<name>.md`) or living capability record (`docs/capabilities/CAP.<name>.md`) is specific enough:

- `INIT.<initiative-name>.md` — intent, thin-slices, acceptance criteria, status, UX, architecture educated theory, QA invariants.
- If specs are missing or too vague, stop and refine through the matching skill (`create-product-design-spec`, `create-product-architecture-spec`, `create-quality-spec`).

## 3. Require A Sprint Bridge

Implementation starts from a sprint (`docs/product/sprints/SPRINT.<YYMMDD>-<slug>.md`), not from a chat prompt.

- If no sprint exists for the selected thin-slice, use `create-sprint`.
- If a sprint exists, read it and obey its immutable scope.
- Do not add scope to a live sprint. Close it and create a new sprint if the bridge is wrong.

## 4. Correlate Against Current Engineering Reality

Before changing files, inspect and summarize the current state that constrains the work:

- Codebase touchpoints: capabilities, files, modules, public contracts, living capability records (`docs/capabilities/CAP.<name>.md`)
- Existing tests: relevant Logic, Composition, Adapter Contract, Integration boundary, Journey coverage
- Toolchain: language/runtime, lint, format, type-check, test commands
- Integrations: providers, feature flags, environment variables, external services
- Active ADRs and known debt/hazards near the change site

## 5. Classify Test Impact Before Code

For every impacted behavior, decide the test posture before implementation:

| Situation | Required posture |
| --- | --- |
| Behavior changes or new behavior is added | Write or update the lowest-layer test first, run it, and observe `RED`. |
| Behavior must be preserved while internals change | Run existing test first and observe `GREEN` before editing. |
| Behavior is underspecified | Stop and clarify the initiative/sprint before writing tests or code. |
| Existing tests fail before work begins | Record baseline failure and decide with human whether it blocks sprint. |
