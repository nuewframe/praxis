# Enforcement — Capability Record

The `scripts/` tree (17 `.sh` files, plus `data/` and `__fixtures__/`) is the generic, project-agnostic enforcement tooling this plugin ships. It is the mechanism half of the plugin's trust-transfer problem: the doctrine in `skills/` says what disciplined work looks like; the scripts here are what can actually check it without relying on an agent's self-report.

## `validate-plugin.sh` — the plugin's own self-test

`scripts/validate-plugin.sh` runs 12 checks: SKILL.md frontmatter, every JSON file parses, every YAML file parses, cross-references resolve, manifest versions are in parity, every enforcement script parses and is executable, inventory parity (every skill/script/instruction on disk is referenced in the canonical docs), agent frontmatter, fenced-code balance, terminology drift (`.praxis-canon.json` forbidden legacy terms), template placeholder parity, and required-phrase presence (the last one enforces `.praxis-canon.json`'s `requiredPhrases` — e.g. that README.md discloses the enforcement split).

The script's own header comment currently lists only 11 numbered checks. The 12th — required-phrase presence — exists in the code (the `requiredPhrases` check against `.praxis-canon.json`) but was never added to the header list. The header undercounts by one; the actual, code-sourced count is 12.

## The three enforcement postures

Eleven `check-*.sh` probes ship for host projects. Each falls into exactly one of three postures:

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
- `check-design-approval-gate.sh` — the newest of the eleven, and deliberately hard-fail with no opt-out by design (see ADR.260720.01, below): unlike every other probe in this repo it ships with no `.{name}.json` config at all, because there is nothing to configure.

### Informational, never fails

- `check-escape-hatch-usage.sh` — reports every `praxis:allow-*` marker it finds by file:line, but always exits 0. Its job is visibility for a human reviewer, not gatekeeping.

## The two generators

Two scripts keep documentation honest against reality rather than hand-maintained:

- `gen-coverage-matrix.sh` derives `docs/coverage-matrix.md` from each text probe's actual `--include` glob, so the coverage claim cannot drift from what the probes really scan.
- `gen-tier-table.sh` derives the tier-classification table rendered into three skill/agent surfaces from one JSON source, `scripts/data/tier-classification.json` (see ADR.260720.02, homed in the `skills` capability record — the generator pattern proven here with `gen-coverage-matrix.sh` is the precedent that pilot reuses).

Both run in `--check` mode in `.github/workflows/ci.yml`.

## CI wiring

`.github/workflows/ci.yml` runs, on every push and pull request, across both an Ubuntu (bash 5) and a macOS (bash 3.2, the declared floor) runner: `validate-plugin.sh`, `check-anti-dumping.sh` (the plugin scanning itself), `test-probes.sh` (the probe language-coverage self-test against `__fixtures__/`), `gen-coverage-matrix.sh --check`, `gen-tier-table.sh --check`, and a `bash -n` syntax sweep over every script in `scripts/`.

## ADR index

| ADR | Purpose |
| --- | --- |
| [ADR.260720.01: Design Approval git pre-push hook gate](../adr/ADR.260720.01-design-approval-git-hook-gate.md) | Builds `check-design-approval-gate.sh`, the one probe in this repo that is hard-fail with no opt-out by design — the first gate Praxis demonstrably fails closed without an orchestration runtime. |

**Cross-capability note.** [ADR.260720.02](../adr/ADR.260720.02-generated-tier-table.md) (generated tier table) is homed in the `skills` capability record, but the generator pattern it establishes was proven using this capability's own `gen-coverage-matrix.sh` as precedent — the same generate / `--write` / `--check` shape, applied to a second fact.
