# WAVE: Self-Conformance

> **Planning-stage document — an educated theory, not yet the truth.** The best approach given what we know today; current-state architecture lives in [docs/architecture/](../../../architecture/), promoted there by `close-sprint`.

**Status:** 🔄 In Progress\
**Goal:** An adopter can trust that every convention and gate Praxis prescribes is one Praxis demonstrably follows and runs against itself.

---

## Tracking Rules

- Track each thin-slice by intended user outcome and current state only.
- If a thin-slice is reopened or corrected, keep the same slice ID and add one short tracking note next to that slice.
- Keep implementation history in sprint files and version control. This README stays focused on product intent.

---

## Value Theme

Praxis's product is **trust transfer** — an adopter believes a green check because the discipline behind it is real. A plugin that prescribes conventions it violates, and ships gates it never runs on itself, spends that trust rather than earning it.

---

## Scope

- The conventions Praxis prescribes to host repos hold in Praxis's own tree (doc homes, dashboard naming, architecture-tree authority statement).
- The enforcement scripts Praxis ships execute against Praxis in CI, or declare an explicit, reasoned `n/a`.
- The self-check suite catches the defect classes that were found by hand during the course correction that produced this wave: stale version literals, broken links, malformed CHANGELOG structure, undeclared skill invocability.
- Facts that Praxis states in more than one place are single-sourced or generated, not hand-synced.

**Out of scope:**

- **Monorepo and multi-repo context placement.** One `project-context.md` cannot describe twelve packages, and a product spanning repos makes `docs/product/waves/` in any single repo a partial truth. Genuinely unsolved design work, not merely unscheduled — it gets its own wave.
- **Real-repo validation of the method itself.** The evolution policy's bar (running a full wave → sprint → close cycle against an external project) is what this wave *unblocks*, not what it delivers. Self-conformance proves internal consistency only, and the policy is explicit that this does not substitute.

---

## Thin-Slices

### TS-001: `create-wave` stops imposing a category taxonomy

> **Status:** ✅ Complete

**User Value:** As a team authoring a wave, I need to name it after the outcome it delivers so that recording product intent does not first require fitting it into someone else's org taxonomy.

**Acceptance Criteria:**

- [x] Given `create-wave`, when a wave is authored, then no category is required in the folder name or the README title
- [x] Given the change, when it ships, then a `CHANGELOG.md` entry states that existing category-prefixed wave folders remain valid
- [x] Given the change is a rule change, when it ships, then an ADR exists at `status: Accepted` recording the decision and its governance basis
- [x] Given the evolution policy, when a future relaxation is proposed, then the policy states how to classify it without re-litigating this case

**Dependencies:** None. **Blocks:** TS-002.

---

### TS-002: Praxis's own docs follow the conventions Praxis prescribes

> **Status:** ✅ Complete

**User Value:** As an adopter reading Praxis's own repo to learn the method, I need its tree to match the tree it tells me to build so that I can use it as a worked example instead of finding the two contradict each other.

**Tracking note:** The pre-adoption plans directory this slice originally described has since been removed rather than merely described correctly — its delivered work is cited as evidence by the derived waves, and its one open commitment moved to `TS-005` of [wave-executable-seams](../wave-executable-seams/README.md). The criterion is stated as the durable constraint it was protecting.

**Acceptance Criteria:**

- [x] Given `docs/architecture/README.md`, when its first line is read, then it matches the authority statement `bootstrap-project` Step 7 mandates, including the `promoted by close-sprint` clause and the `docs/product/waves/` path
- [x] Given a wave exists under `docs/product/waves/`, when the product dashboard is read, then that wave is registered there
- [x] Given work that shipped before wave adoption, when it is brought into the wave structure, then it is cited as evidence and never rewritten into a wave complete with acceptance criteria it never had
- [x] Given any statement of fact in a living capability record, when it is read, then it matches the code it describes

**Dependencies:** TS-001

---

### TS-003: The self-check suite catches what was found by hand

> **Status:** ✅ Complete

**User Value:** As a maintainer changing Praxis, I need the suite to fail on the defect classes that previously slipped through so that I am not the mechanism by which broken links and malformed release notes are caught.

**Acceptance Criteria:**

- [x] Given a broken relative markdown link anywhere in the repo, when `validate-plugin.sh` runs, then it fails and names `file:line`
- [x] Given a fenced code block or a `<placeholder>` template path, when the link check runs, then it is not reported
- [x] Given a `CHANGELOG.md` whose version headings are missing, duplicated, or out of order, when `validate-plugin.sh` runs, then it fails and names each offence
- [x] Given each new check, when it is added, then a deliberate defect is introduced and confirmed to fail the build, then reverted

**Dependencies:** None

---

### TS-004: A skill's invocability is declared, not inherited

> **Status:** ✅ Complete

**User Value:** As an adopter installing Praxis on any harness, I need each skill to state whether a human or the model may invoke it so that behavior does not silently vary with the harness's defaults.

**Acceptance Criteria:**

- [x] Given any `skills/*/SKILL.md`, when its frontmatter is parsed, then both `user-invocable` and `disable-model-invocation` are present
- [x] Given the lean-delivery `applyTo` glob, when a reader asks why top-level `docs/waves/**` and `docs/sprints/**` appear, then the document explains they are override-only and names the config keys that reach them
- [x] Given doctrine terminology, when it drifts in `docs/`, then the terminology check fails — verified by negative test

**Dependencies:** None

---

### TS-005: Praxis runs the gates it ships

> **Status:** ⚪ Not Started

**User Value:** As an adopter deciding whether to trust a Praxis gate, I need evidence that the gate runs green against Praxis itself so that "we ship this check" and "this check passes" are not two different claims.

**Acceptance Criteria:**

- [ ] Given the enforcement scripts applicable to a repo with no runtime code, when CI runs, then each executes against Praxis rather than only being syntax-checked by `bash -n`
- [ ] Given a gate that targets host-repo runtime code Praxis does not have, when CI runs, then it reports an explicit reasoned `n/a` rather than being silently absent
- [ ] Given `check-design-approval-gate.sh`, when CI runs on a branch carrying a Major-tier sprint, then the gate's verdict is visible in the build

**Dependencies:** None. Gate findings may block other slices from reporting green.

---

### TS-006: The product dashboard has one name

> **Status:** ⚪ Not Started

**User Value:** As an adopter following Praxis's own tree as a template, I need the dashboard filename Praxis uses to be the filename Praxis tells me to use so that I do not have to guess which of two conventions is current.

**Acceptance Criteria:**

- [ ] Given any skill, agent, or instruction that names the product dashboard, when it is read, then it names one path, matching the one in Praxis's own tree
- [ ] Given the rename, when it ships, then the `CHANGELOG.md` entry states whether adopters must migrate and how

**Dependencies:** TS-002

---

### TS-007: A fact Praxis states twice is stored once

> **Status:** ⚪ Not Started

**User Value:** As a reader of Praxis's docs, I need duplicated content to agree with itself so that I do not act on whichever copy I happened to open.

**Acceptance Criteria:**

- [ ] Given the skill index and the guardrail and persona lists, when they appear on more than one surface, then each surface is generated from one source or reduced to a pointer
- [ ] Given a hand-edit to generated content, when CI runs, then the build fails

**Dependencies:** TS-002. **Named risk:** properly single-sourcing content that legitimately lives in two places may require the dual-home generator currently deferred to its own wave. If that dependency materializes, narrow this slice to pointer-reduction and re-scope the generator rather than hand-syncing.

---

### TS-008: A sanctioned literal explains itself where it appears

> **Status:** ⚪ Not Started

**User Value:** As a maintainer whose document legitimately cites a version or a retired term, I need to declare that one occurrence at the line where it appears so that excusing it does not disable the check for every document in the directory.

**Acceptance Criteria:**

- [ ] Given a literal inside a fenced block, blockquote, or inline code span, when the version or terminology check runs, then it is treated as a citation and not reported — using the same fence-tracking implementation as check #14, not a second copy
- [ ] Given a literal in running prose with an inline `praxis:allow-*` marker carrying a non-empty `reason=`, when the check runs, then it is allowed
- [ ] Given such a marker with a missing or empty `reason=`, when the check runs, then it fails — an unexplained exemption is the defect being removed
- [ ] Given a stale claim about the *current* plugin version anywhere in `docs/architecture/adr/`, when the check runs, then it fails (it does not today, because the whole directory is exempt)
- [ ] Given the new markers, when a PR touches one, then `check-escape-hatch-usage.sh` reports it and the Trust Receipt carries the count
- [ ] Given the migration, when the path allowlists are deleted, then a `--report-only` parity pass has already shown the new rules match the old ones on the clean tree

**Dependencies:** None. Decided in [ADR.260725](../../../architecture/adr/ADR.260725-inline-declared-exceptions.md) (status: Accepted). Major-tier, so implementation additionally requires a signed Design Approval block in the sprint that carries it.

**Named risk:** `bump-version.sh --audit` is load-bearing for releases, so its blast radius exceeds the defect. Change `--audit` matching only; leave `--check` and `--write` untouched.

---

### TS-009: A path named in prose is checked, not just a path in a link

> **Status:** ⚪ Not Started

**User Value:** As a maintainer moving or deleting a directory, I need a reference written as prose or backticks to fail the build the same way a markdown link does, so that a shipped skill cannot keep pointing at a file that no longer exists.

**Acceptance Criteria:**

- [ ] Given a backticked or bare repository path in any markdown file that does not resolve, when `validate-plugin.sh` runs, then it fails and names `file:line`
- [ ] Given a path under `docs/`, when it is referenced in prose, then it is checked — the cross-reference check currently covers only `skills/`, `agents/`, `instructions/`, and `scripts/` prefixes
- [ ] Given a path that is illustrative rather than real (a host-repo example, a `<placeholder>` segment, a path inside a fenced block), when the check runs, then it is not reported
- [ ] Given the new coverage, when it is added, then a deliberately broken prose path is confirmed to fail the build, then reverted

**Dependencies:** None.

**Tracking note:** Evidenced rather than hypothetical. Removing the pre-adoption plans directory left `skills/define-seam-contract/SKILL.md` pointing at a deleted file, and neither existing check caught it — check #14 validates markdown links only, and the cross-reference check matches only the four code-directory prefixes. A shipped skill carried a dangling reference that had to be found by grep.

---

## Success Criteria

Wave is complete when:

- [ ] All thin-slices are ✅ Complete
- [ ] Journey tests pass for all primary scenarios — for this wave, that means `create-wave` runs end-to-end producing a category-free wave, and the session-start hook still emits valid JSON carrying all three guardrail names
- [ ] User guides updated (TEACH) for capabilities whose user-observable behavior changed — via `author-user-docs`
- [ ] Product dashboard updated to reflect completion

---

## Dependencies

- **Requires:** nothing external. Every slice is documentation, skill prose, or self-test tooling; no runtime boundary, no third-party integration.
- **Enables:** real-repo validation of the method. The evolution policy makes external validation the non-negotiable step before a minor bump, and a plugin that visibly violates its own conventions cannot be honestly put in front of an external project first.
