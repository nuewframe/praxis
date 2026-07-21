---
applyTo: "**"
description: "Always-on code contribution intake gate: before implementation for user stories, features, behavior changes, or non-trivial code contributions, anchor work to wave, thin-slice, sprint, current code state, and test posture."
---

# Code Contribution Intake Guardrails

Use these rules before writing or modifying implementation code for any user story, feature, thin-slice, behavior-changing contribution, or non-trivial refactor. The host project owns the final word; project-specific instructions may narrow or strengthen these rules.

## 1. Start From Product Intent

Do not treat a direct implementation request as sufficient product context.

- Identify the active wave and thin-slice from the wave `README.md`.
- Confirm the thin-slice has testable acceptance criteria.
- If no wave or thin-slice exists, stop and use `create-wave` / `create-product-design-spec` before implementation planning.
- If the request changes a completed thin-slice, keep the original thin-slice ID and treat it as a correction or reopen.

## 2. Verify The Wave Specs

Before sprint planning or implementation, confirm the wave has the four-document pattern:

- `README.md` - intent, thin-slices, acceptance criteria, status
- `product-design.md` - user-visible journeys, ambiguity, recovery, error states
- `product-architecture.md` - ownership, contracts, data/control flow, security, failure behavior
- `qa.md` - risk tiers, test layer map, security coverage, definition of done

If any spec is missing or too vague to guide the work, stop and author/refine it through the matching skill. Do not fill the gap by guessing in code.

## 3. Require A Sprint Bridge

Implementation starts from a sprint, not from the chat prompt.

- If no sprint exists for the selected thin-slice, use `create-sprint`.
- If a sprint exists, read it and obey its immutable scope.
- Do not add scope to a live sprint. Close it and create a new sprint if the bridge is wrong.
- The sprint must include the engineering current-state snapshot, gap analysis, implementation plan, test plan, and acceptance criteria.

## 4. Correlate Against Current Engineering Reality

Before changing files, inspect and summarize the current state that constrains the work:

- Codebase touchpoints: capabilities, files, modules, public contracts
- Existing tests: relevant Logic, Composition, Adapter Contract, Integration boundary, Journey coverage
- Toolchain: language/runtime, lint, format, type-check, test commands
- Integrations: providers, feature flags, environment variables, external services
- Active ADRs and known debt/hazards near the change site

The implementation plan closes the gap between the wave target state and this current reality.

## 5. Classify Test Impact Before Code

For every impacted behavior, decide the test posture before implementation:

| Situation                                         | Required posture                                                                                                                                                |
| ------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Behavior changes or new behavior is added         | Write or update the lowest-layer test first, run it, and observe `RED` for the expected reason.                                                                 |
| Behavior must be preserved while internals change | Run the relevant existing test first and observe `GREEN` before editing. If no test exists, add a characterization test and observe `GREEN` before refactoring. |
| Behavior is underspecified                        | Stop and clarify the wave/sprint/spec before writing tests or code.                                                                                             |
| Existing tests fail before work begins            | Record the baseline failure and decide with the human whether it blocks the sprint. Do not hide it inside the contribution.                                     |

Use `test-by-ownership` to choose the layer. The same behavior may appear at multiple layers only when each layer asserts a different property.

## 6. Produce The Contribution Envelope

Before implementation, state the intake result concisely:

```markdown
## Code Contribution Intake

Wave: [wave]
Thin-slice: [TS-NNN]
Sprint: [sprint file]
Wave specs: README / product-design / product-architecture / qa
Current code touchpoints: [files/capabilities]
Behavior impact: changes behavior | preserves behavior | mixed
Test posture:

- Red-first: [tests]
- Green-baseline: [tests]
- Missing coverage: [tests to add]

Implementation boundary: [what is in scope]
Verification commands: [format/lint/type/test/e2e as applicable]
```

Once the envelope is clear, continue with `implement-with-defensive-patterns`, then `verify-and-assemble-pr`.

## Anti-Patterns

- Starting from a chat prompt without finding the wave and thin-slice
- Writing implementation code before a sprint exists
- Treating `qa.md` as optional because tests will be written later
- Changing behavior without first observing a failing test for the changed behavior
- Refactoring without first establishing a green baseline
- Adding assertions at multiple layers for the same property
- Updating sprint scope mid-flight instead of closing and reopening the bridge
