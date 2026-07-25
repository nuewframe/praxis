# Enforcement — Capability Record

The `scripts/` tree (18 `.sh` files, one `.py` module, plus `data/` and `__fixtures__/`) is the generic, project-agnostic enforcement tooling this plugin ships. It is the mechanism half of the plugin's trust-transfer problem: the doctrine in `skills/` says what disciplined work looks like; the scripts here are what can actually check it without relying on an agent's self-report.

## `validate-plugin.sh` — the plugin's own self-test

`scripts/validate-plugin.sh` runs 16 checks: SKILL.md frontmatter (including the guard that a `tools:` value must be a single-line flow sequence, since a multi-line one is valid YAML that silently prevents the skill registering in Claude Code), every JSON file parses, every YAML file parses, cross-references resolve, manifest versions are in parity, every enforcement script parses and is executable, inventory parity (every skill/script/instruction on disk is referenced in the canonical docs), agent frontmatter, fenced-code balance, terminology drift (`.praxis-canon.json` forbidden legacy terms, scanned across `skills`, `instructions`, `agents`, and `docs`), template placeholder parity, required-phrase presence (enforcing `.praxis-canon.json`'s `requiredPhrases` — e.g. that README.md discloses the enforcement split), version single-source (delegating to `bump-version.sh --audit`), link resolution, CHANGELOG structure, and self-conformance declaration parity.

The last three are the drift classes that were previously caught only by hand. **Link resolution matters because inventory parity proves a name is *mentioned*, never that a link *resolves*** — a reorganization can satisfy the first while stranding every reference. Its three exclusions are load-bearing: fenced blocks (template content whose relative paths resolve from the destination document), `<placeholder>` path segments, and external schemes. Its fence tracking honours the opening marker's length; a naive three-backtick toggle produces false negatives.

The count is code-sourced, not header-sourced. The script's header comment has undercounted before, which is why the number here is stated against the executing checks.

Check #16 — **self-conformance declaration parity** — is the one that closes the loop this capability record describes. It reads `.self-conformance.json` and fails when a shipped `check-*.sh` is undeclared, declared twice, names a script that does not exist, or is exempted with `runs: false` and no reason. An unexplained exemption is the defect it removes; without it, a probe could ship and simply never run against this repo, with nothing recording whether that was a decision or an oversight.

## The three enforcement postures

Twelve `check-*.sh` probes ship for host projects. Each falls into exactly one of three postures:

### Warn-first, mode-promotable

Each probe below reads its own `.{name}.json` config, defaults to `mode: warn` (report, exit 0), and only fails the build once a project flips that file to `mode: enforce`:

- `check-config-externalized.sh` — Configurable anchor (hardcoded remote URLs, endpoints, secret literals)
- `check-observability-at-seams.sh` — Observable anchor (a boundary call with no log/metric/trace/correlation-id)
- `check-stateless-request-path.sh` — Horizontally-scalable anchor (node-local mutable state on the request path)
- `check-resilient-boundary.sh` — Resilient anchor (a boundary call with no timeout/retry/circuit-breaker/fallback)
- `check-seam-contract-parity.sh` — every seam in `.seam-contracts.json` has a machine-readable Shape and a shared Behavior suite on disk
- `check-sprint-id-collision.sh` — no two active sprint files share an id token (exact, not heuristic, but still warn-first via `.sprint-coordination.json` until a project promotes it)

### Hard-fail, no warn mode

These probes have no `mode: enforce` config to flip because they never shipped a warn mode to begin with — they fail outright:

- `check-anti-dumping.sh`
- `check-no-skipped-tests.sh`
- `check-no-sleep-waits.sh`
- `check-port-adapter-parity.sh`
- `check-design-approval-gate.sh` — the newest of the twelve, and deliberately hard-fail with no opt-out by design (see ADR.260720.01, below): unlike every other probe in this repo it ships with no `.{name}.json` config at all, because there is nothing to configure.

### Informational, never fails

- `check-escape-hatch-usage.sh` — reports every `praxis:allow-*` marker it finds by file:line, but always exits 0. Its job is visibility for a human reviewer, not gatekeeping. Its vocabulary is **seven** markers, defined once in the script's `MARKERS` array; the header comment used to restate the list with nothing keeping the two in agreement, and no longer does.

## Declared exceptions — citation is not assertion

Two of the checks are literal scanners: version single-source (#13, delegating to `bump-version.sh --audit`) and terminology drift (#10). Neither could tell a document that **asserts** a literal from one that **cites** it, and the answer used to be path allowlists — 8 entries in `.version-bump.json` and 3 in `.praxis-canon.json`, most of them directory-wide. That inverted default-deny: exempting `docs/architecture/adr/` meant a future ADR making a genuinely stale claim about the *current* plugin version passed silently, which is the precise defect check #13 exists to catch, disabled across the tree holding the most durable decisions.

Both lists are gone. [ADR.260725](../adr/ADR.260725-inline-declared-exceptions.md) replaced them with three layers, and `scripts/citation_scan.py` is the single implementation both scanners consume — one home, so Layer 1 cannot drift between them:

- **Layer 1 — structural, no configuration.** A literal inside a fenced block, a blockquote, or an inline code span is a citation by position and is not reported. The fence rule is check #14's, moved into the shared module rather than copied; it honours the opening marker's length, because a naive three-backtick toggle produced a real false negative. Code-span detection is column-precise: a line carrying a cited literal in backticks *and* a bare assertion is still reported.
- **Layer 2 — declared, for the prose residue.** A citation in running prose carries `<!-- praxis:allow-version-literal reason="…" -->` or `<!-- praxis:allow-term reason="…" -->` on the preceding line. **The reason is mandatory and its absence is itself a failure** — an unexplained exemption is the defect being removed, not a smaller version of it.
- **Layer 3 — accumulation is visible.** Both markers are registered in `check-escape-hatch-usage.sh`, so each use appears in the PR diff report, and the marker count joins the Trust Receipt. An exemption set that grows in silence is debt; one that reports its size every PR is a decision the reviewer keeps making.

Migrating this way changed what the checks can see. Under the old rules a planted stale current-version claim in an ADR passed; under the new rules it fails. In practice **Layer 1 absorbed almost the whole migration** — the citations that surfaced were version tokens that read better as code literals anyway, and backticking them is what a careful author would have done regardless. That matters for the open question ADR.260725 deliberately left undecided: a marker *range* form for a paragraph of several citations was never needed, because the one shape that would have required it — a literal inside a markdown table row, where a comment line would break the table — is solved by Layer 1 instead. The range form stays uninvented.

Two of the three retired terminology entries turned out to be dead weight: `CHANGELOG.md` and `.praxis-canon.json` sit outside the scanned directories, so they were exempting files the check never read.

Check **#4** (cross-references) consumes the same module. It checks a repository *file* path named in markdown prose — backticked or bare, under `docs/` as well as the four code prefixes — which link resolution cannot see: #14 proves a link works and says nothing about the far more common path in running text. Three rules keep its signal meaningful. It matches file paths only, because bare directory names are where illustrative host-repo structure concentrates and matching them buried the real defects under examples. It excludes markdown link constructs, because a prose path resolves from the repo root while a link target resolves from the linking file's directory — conflating the two reported a correct link as broken. And it applies fence, blockquote, and `praxis:allow-path` exemption but deliberately **not** code-span exemption: for a path, backticks are the ordinary notation, so exempting them would excuse nearly every reference the check exists to validate. That asymmetry with the version scanner is the point — the same module serves both because the citation *positions* differ per literal kind.

Check #10 also had to change shape before Layer 2 was possible at all. It scanned whole files and reported the file, so there was no line for a marker to attach to; it is now line-scoped and reports `file:line`.

## The two generators

Two scripts keep documentation honest against reality rather than hand-maintained:

- `gen-coverage-matrix.sh` derives `docs/coverage-matrix.md` from each text probe's actual `--include` glob, so the coverage claim cannot drift from what the probes really scan.
- `gen-tier-table.sh` derives the tier-classification table rendered into three skill/agent surfaces from one JSON source, `scripts/data/tier-classification.json` (see ADR.260720.02, homed in the `skills` capability record — the generator pattern proven here with `gen-coverage-matrix.sh` is the precedent that pilot reuses).

Both run in `--check` mode in `.github/workflows/ci.yml`.

## CI wiring

`.github/workflows/ci.yml` runs, on every push and pull request, across both an Ubuntu (bash 5) and a macOS (bash 3.2, the declared floor) runner: `validate-plugin.sh`, `.github/run-self-conformance.sh` (every shipped gate, below), `test-probes.sh` (the probe language-coverage self-test against `__fixtures__/`), `gen-coverage-matrix.sh --check`, `gen-tier-table.sh --check`, and a `bash -n` syntax sweep over `scripts/*.sh` and `.github/*.sh`.

## Running the gates it ships

A plugin that ships a gate it never runs against itself is making two different claims with one script. `.self-conformance.json` closes that gap: it declares, for each of the twelve probes, whether Praxis executes it in its own CI — and when it does not, a **mandatory** `reason`. `.github/run-self-conformance.sh` reads that manifest, executes every `runs: true` gate, and prints an explicit `n/a — <reason>` line for the rest, so the full twelve-gate verdict is visible in the build rather than ten of them being silently absent.

The runner accumulates rather than short-circuits: a gate failure is recorded and reporting continues, so one red gate never hides the eleven behind it. It lives under `.github/` rather than `scripts/` because it is Praxis's own CI tooling, not generic host-facing enforcement that ships to adopters.

The split today is **10 run, 2 reasoned `n/a`**:

- **`check-no-skipped-tests.sh` and `check-no-sleep-waits.sh` are `n/a`.** Both target a product test suite Praxis does not ship. Their only in-tree matches are the deliberate dirty fixtures under `scripts/__fixtures__/`, which `test-probes.sh` already asserts must exit 1 — a stronger proof of those probes than a scoped vacuous pass. Giving them an exclusion flag was rejected: changing a host-facing probe's scan semantics to accommodate this repo's fixture layout would export Praxis's problem into every adopter's build.
- **Five of the ten that run pass vacuously**, and say so in a `note` field: the four production-readiness anchors and `check-port-adapter-parity.sh` scan globs Praxis has no files under. They are still executed, because that proves the scripts *run* on both the bash 3.2 floor and bash 5 — which `bash -n` does not — but the manifest records that the pass proves execution, not that the anchor holds on real runtime code. The gate is not permitted to read as a stronger claim than it is.
- **`check-seam-contract-parity.sh` is declared `runs: true` even though it self-skips today** (Praxis declares no `.seam-contracts.json`), so the gate activates automatically the moment Praxis declares its first seam rather than the exemption going stale.
- **`check-design-approval-gate.sh` runs unconditionally**, so its verdict appears on every build. With no Major-tier sprint on the branch it reports the empty-input verdict; the populated path is exercised whenever a Major sprint is present.

## ADR index

| ADR | Purpose |
| --- | --- |
| [ADR.260720.01: Design Approval git pre-push hook gate](../adr/ADR.260720.01-design-approval-git-hook-gate.md) | Builds `check-design-approval-gate.sh`, the one probe in this repo that is hard-fail with no opt-out by design — the first gate Praxis demonstrably fails closed without an orchestration runtime. |
| [ADR.260725: Declared exceptions move inline](../adr/ADR.260725-inline-declared-exceptions.md) | Replaces the path-based allowlists in `.version-bump.json` and `.praxis-canon.json` with structural citation detection plus inline reasoned `praxis:allow-*` markers, restoring default-deny at line granularity and making exemption growth visible in the Trust Receipt. **Status: Accepted** — implementation is `TS-008` of wave-self-conformance. |

**Cross-capability note.** [ADR.260720.02](../adr/ADR.260720.02-generated-tier-table.md) (generated tier table) is homed in the `skills` capability record, but the generator pattern it establishes was proven using this capability's own `gen-coverage-matrix.sh` as precedent — the same generate / `--write` / `--check` shape, applied to a second fact.
