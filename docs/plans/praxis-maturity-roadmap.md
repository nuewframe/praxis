# Plan — Praxis Maturity Roadmap

**Status:** Implemented (Phases 0–5 landed; two items pragmatically deferred, noted below)
**Author:** Principal Engineer (architect mode)
**Scope:** Resolve the defects and structural shortcomings found in the post-fix review — the ones that survive after the critical/high/medium/low review items were closed. Sequenced by leverage, not by severity label.
**Deliverable of this doc:** a roadmap only. No skill/script/template changes until a phase is approved.

## Execution notes (2026-07-20)

- **Phase 4.1** (extract per-language markers to a shared registry consumed by the probes) was **deferred** as an internal cleanup: Phase 0.2 already unified the `--include` extension set across all text probes, so the generated coverage matrix shows full coverage and the registry refactor is modest-ROI / real-regression-risk on six working probes.
- **Phase 2** was **lighter than planned**: the audit found the overlays are already thin pointers (19–32 lines), so the work was to codify the pointer-only principle and rely on the new terminology + placeholder-parity lints to enforce non-divergence, rather than rewrite templates.
- Everything else shipped as specified, each with a CI guard. Every phase commit passes `validate-plugin.sh` (now 11 checks) + `test-probes.sh` + `gen-coverage-matrix.sh --check`.

---

## 1. The meta-pattern

Every remaining issue is one of three kinds. The plan is organized around the kind, because the *fix strategy* differs by kind — treating them all as "bugs to patch" is what produced the drift in the first place.

| Kind | Meaning | Strategy |
|---|---|---|
| **Defect** | Concretely wrong; a correct version exists | Fix + add a guard so it can't recur |
| **Erosion** | Correct today, but the architecture guarantees it drifts | Change the architecture (single source of truth), not just the instance |
| **Inherent tradeoff** | Not fully solvable; overselling it is the real defect | Scope the claim honestly + give adopters a dial |

The single deepest root cause is **doctrine duplicated across four surfaces** (skills, instructions, agent personas, overlay templates) with no mechanical parity check. Phases 1–2 attack that root; the rest follow.

---

## 2. Guiding principles (the decisions that shape every phase)

1. **One canonical home per fact; everything else references or is generated.** No fact (tier table, Major-path order, alias name, terminology, phase chain) is stated in two editable places.
2. **Mechanism over prose wherever a gate can be mechanized.** A gate that only a human can enforce must say so; a gate a script can enforce must ship that script and run it in CI.
3. **The overlay is generated, never hand-copied.** Divergence is impossible if the second copy doesn't exist.
4. **Honesty over aspiration.** "Universal" and "mechanical gates" are scoped to what actually ships enforced, with a documented capability matrix.
5. **The plugin eats its own dog food.** Every rule Praxis imposes on adopters runs against Praxis itself in CI.

---

## 3. Phase 0 — Stop the bleeding (concrete defects + CI)

Highest leverage, lowest risk. These are the review's confirmed defects plus the safeguard that makes the rest durable.

### 0.1 Fix the persona-alias placeholder mismatch
- **Problem:** 21 template files use `personas.aliases.principal-engineer` (hyphen); config + manifest use `principal_engineer` (underscore). The alias feature is dead when `use_aliases: true`.
- **Work:** rewrite the 62 hyphen refs to underscore across the 21 overlay templates. Choose underscore (matches config + manifest + the majority-canonical surfaces).
- **Guard:** add a `provision` self-check (and a plugin CI lint) that every `{{personas.aliases.X}}` placeholder in a template resolves against a key in `praxis.config.yaml.tmpl`. (See Phase 1 drift lint.)
- **Done:** grep shows zero hyphenated alias placeholders; a rendered dry-run with aliases on produces no literal `{{…}}`.

### 0.2 Fix the two probe coverage gaps
- **Problem:** `check-no-skipped-tests.sh` and `check-no-sleep-waits.sh` scan only 9 extensions and use JS/Python-centric patterns; Go/Rust/Ruby/C#/PHP skipped tests and sleeps pass clean. `time.sleep` won't even match Go's `time.Sleep` (case-sensitive).
- **Work:** unify the file-extension list across all six text probes (single shared list), and extend the two patterns with per-language markers: Go `t.Skip(`/`b.Skip(`/`time.Sleep(`, Rust `#[ignore]`/`thread::sleep`/`tokio::time::sleep`, Ruby `skip`/`sleep `, C# `[Skip`/`[Ignore]`/`Thread.Sleep`, PHP `markTestSkipped`/`sleep(`.
- **Done:** a fixture repo per language with a skipped test + a sleep is flagged by both probes; unit fixtures live under a `scripts/__fixtures__/` and run in CI.

### 0.3 Small correctness fixes
- `check-anti-dumping.sh`: detect malformed JSON explicitly (`jq empty` pre-check) → exit 2 with "invalid JSON", not the misleading "no scanPaths".
- Document (don't silently ship) that the four production-readiness probes are **warn-first**; add a one-line "flip to enforce when clean" to each probe's non-conforming output and to `create-quality-spec`.

### 0.4 Add CI (the deferred item 9)
- **Work:** a `.github/workflows/ci.yml` that on push/PR runs `validate-plugin.sh`, `check-anti-dumping.sh`, and the probe fixtures; matrix on macOS (bash 3.2) + Linux (bash 5) to lock the declared floor.
- **Done:** CI is green on `main`; a PR that reintroduces any Phase-0 defect fails.

**Exit criterion for Phase 0:** all concrete defects fixed, each with a CI guard that fails on reintroduction.

---

## 4. Phase 1 — Kill semantic drift (the root cause)

The validator catches *syntactic* drift (fences, frontmatter, inventory). It does not catch *semantic* drift — the Major-path order, alias↔config parity, terminology, ownership. This phase makes the canonical facts machine-checkable.

### 1.1 Declare the canonical-facts registry
- Create `.praxis-canon.json` (or reuse an existing manifest) enumerating single-source facts and their canonical home, e.g.:
  - `major_path_order` → `skills/start-thin-slice/SKILL.md` Step 5
  - `tier_table` → `intake-code-contribution` Step 0
  - `test_layers` → `test-by-ownership`
  - `terminology` → a small glossary (`educated theory`, not `bet`; `four-condition disjointness`, etc.)
- Every other file must **reference** these, not restate them.

### 1.2 Add drift lints to `validate-plugin.sh`
- **Terminology lint:** forbidden legacy terms (`\bbet\b` in the wave sense, `three-axis`, `Component` as a test layer) fail unless in an allowlisted context.
- **Placeholder-parity lint:** every `{{path.key}}` in any `.tmpl` resolves against `praxis.config.yaml.tmpl` (catches 0.1's class permanently).
- **Cross-file order lint:** the Major-path skill chain, extracted from each of the 3 sources by regex, must be identical (or the two non-canonical sources must literally contain the canonical reference sentence rather than a restated chain).
- **Done:** flipping any single source out of sync fails CI; negative-tested.

### 1.3 Convert restatements to references
- Sweep the skills/instructions/personas for restated tables/orders that a lint now guards; replace with a one-line pointer to the canonical home. (This is the durable version of what item 3 did for the Major path.)

**Exit criterion:** a doctrine change made in one place either propagates by reference or fails CI — no silent divergence path remains.

---

## 5. Phase 2 — Thin the overlay (collapse the diverging copy)

The overlay is a second, hand-maintained copy of the plugin. It is where the alias bug and the missing `create-quality-spec` lived. Two options, pick one:

- **Option A (preferred) — pointer-only overlay.** Each overlay skill becomes a 5–8 line pointer: "the full skill is the plugin's `<name>`; here are this project's paths + gates." No restated doctrine, so nothing to drift. Drop the renamed variants (`create-system-architecture`, `write-tests`, `implement-capability-change`) or keep them as *pure* aliases whose body is only a name-map + paths.
- **Option B — generate the overlay.** A `provision` step renders overlays from the plugin skills + `praxis.config.yaml`, so the overlay is a build artifact, never edited.

Recommendation: **A now, B later.** A removes the drift surface immediately with low risk; B removes the maintenance entirely but needs a generator.

- **Done:** no overlay template restates plugin doctrine; the placeholder-parity + terminology lints (1.2) run over the overlay too.

---

## 6. Phase 3 — Make enforcement honest and, where possible, real

The headline promise is "mechanical gates," but on the primary harness the gates are prose the agent self-attests to. This phase closes the gap between claim and mechanism.

### 3.1 Separate "mechanically enforced" from "self-attested"
- Audit every gate and tag it: **script-enforced** (runs in `verify`/CI), **human-signed** (approval lines), or **agent-attested** (envelope, conformance block).
- Publish the tags in `using-praxis` so adopters know exactly what is enforced vs trusted. This is the honest version of "mechanical."

### 3.2 Ship a reference enforcement wiring
- A `.githooks/pre-push` + the Phase-0 CI that actually *runs* the check scripts, so at least the script-enforceable gates fail closed without requiring MPM.
- Document the MPM wiring for the agent-attested gates (delegation, separate reviewer head) as the *upgrade* path, not the baseline — and stop implying the baseline enforces them.

### 3.3 Strengthen the one gate that catches forgery
- Make the **separate reviewer head** the default in `verify-and-assemble-pr` for Major, downgraded to "explicit fresh-diff reviewer-mode switch" only when a second head is unavailable — and record which was used (already partially there; make it a hard checklist item the PR narrative must carry).

**Exit criterion:** every gate is labeled by how it's enforced; the script-enforceable subset runs in CI/hooks with zero manual steps; docs no longer overstate enforcement.

---

## 7. Phase 4 — Scope "universal" honestly

"Universal — language-, framework-, and runtime-agnostic" is true for *portability* and false for *enforcement coverage*.

### 4.1 Per-language pattern registry
- Extract every probe's language-specific markers into one `scripts/patterns/<lang>.json` registry the probes consume. Adding a language is one file, not six edits.

### 4.2 Coverage matrix
- Ship `docs/coverage-matrix.md`: rows = probes, columns = languages, cells = supported/partial/none. CI regenerates it from the registry so it can't lie.

### 4.3 Re-word the claim
- Change "universal" to "multi-language, with documented per-probe coverage (see coverage matrix); language-agnostic in doctrine, best-effort in static enforcement."

**Exit criterion:** the claim matches the matrix; the matrix is generated, not hand-written.

---

## 8. Phase 5 — Adoption dial + upgrade story

Two inherent tradeoffs remain: **ceremony cost** (D) and **copies rot** (G). These are dialed, not eliminated.

### 5.1 Adoption profiles
- `praxis.config.yaml` gains `profile: lite | standard | full`:
  - **lite** — Trivial/Standard only; no hypothesis card, no four-anchor block, single-doc waves. For solo/small repos.
  - **standard** — current default.
  - **full** — adds the parallel-safe coordination artifacts + adversarial second head. For multi-agent at scale.
- Gates read the profile; the ceremony a two-file change sees is proportional to the profile. This makes "route around it" unnecessary instead of tempting.

### 5.2 Upgrade command
- `provision --upgrade`: re-syncs the generated overlay, check scripts, and `verify.sh` from the installed plugin version, diffing human-edited files. Record the plugin version in `praxis.config.yaml` so drift between a repo and the plugin is visible.
- **Done:** bumping the plugin and running `--upgrade` pulls script fixes (like today's seam-parity fix) into an adopted repo with a reviewable diff.

---

## 9. What we are NOT solving (and how we stay honest)

A mature plan names the limits:

- **Runtime enforcement on a bare harness is out of scope.** Praxis is artifact discipline; forcing an agent to run intake before coding needs an orchestration runtime. We make this explicit (3.1) rather than implying the plugin does it.
- **Self-attested gates can always be forged by a sufficiently sloppy generator.** We shrink the trusted surface (3.3, script-enforcement) but do not claim to eliminate it.
- **Harness fragmentation is inherent to multi-harness support.** We document per-harness enforcement (3.1) instead of pretending parity.

---

## 10. Sequencing & dependencies

```
Phase 0 (defects + CI) ──► Phase 1 (drift lints) ──► Phase 2 (thin overlay)
                                     │
                                     ├──► Phase 3 (enforcement honesty)
                                     └──► Phase 4 (coverage matrix)
Phase 5 (profiles + upgrade) depends on Phase 2 (generated/pointer overlay).
```

- **0 first** — it's the safety net; nothing else regresses silently after it.
- **1 before 2** — the drift lints must exist before we thin the overlay, so the thinning is guarded.
- **3 and 4 are parallelizable** once 1 lands.
- **5 last** — profiles and upgrade assume the overlay is thin/generated.

## 11. Definition of done (roadmap-level)
- Every concrete defect fixed **with a CI guard**.
- No fact is editable in two places without a lint failing.
- The overlay restates no doctrine.
- Every gate is labeled by its enforcement mechanism; the script-enforceable set runs in CI.
- The "universal" claim matches a generated coverage matrix.
- A solo repo can adopt at `lite`; an adopted repo can pull plugin fixes via `--upgrade`.

## 12. Rollback
Each phase is independently revertable; Phases 0–4 are additive (lints, CI, fixes, docs). Phase 5 changes `provision` behavior and is gated behind the `profile`/version keys, defaulting to today's behavior when absent.
