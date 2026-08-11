# CAP.plugin-conformance-and-validation-probes: Plugin Conformance & Validation Probes

**Domain Owner:** Praxis Core Maintainers  
**Status:** Active  
**Seam Contracts:** `validation-probes@v1` (`scripts/validate-plugin.sh`)  

---

## 1. Domain Description & Bounded Context

- **Core Purpose:** Owns automated quality verification probes (`scripts/check-*.sh`), plugin validation probe (`scripts/validate-plugin.sh`), version single-source audit, cross-reference scanners, and test-suite fixtures.
- **Bounded Context:** Enforces structural anti-dumping rules, seam contract parity, port/adapter parity, production readiness anchors, link resolution, and self-conformance declaration parity.

---

## 2. User Experience & Living Journeys

- **User Personas Served:** Developers, Principal Engineers, CI Pipelines.
- **Validation Suite Execution:**
  1. Developer or CI runs `bash scripts/validate-plugin.sh`.
  2. Probe executes 15 automated composition & structural checks in $< 2\text{sec}$.
  3. Returns `validate-plugin: all checks passed` (exit code 0) or outputs exact `file:line: error` breakdown.

---

## 3. Technical Architecture & System Topology

- **Architecture Layout:** `scripts/validate-plugin.sh`, `scripts/check-*.sh`, `scripts/citation_scan.py`, `scripts/test-probes.sh`, `scripts/test-citation-scan.sh`
- **15 Automated Validation Checks:**
  1. `SKILL.md` frontmatter validity & single-line `tools:` key check.
  2. JSON syntax & schema validation.
  3. YAML syntax validation.
  4. Markdown cross-reference check (`citation_scan.py` with `praxis:allow-path`).
  5. Manifest version parity (`bump-version.sh --check`).
  6. Enforcement script execution check.
  7. Inventory parity (skills, scripts, instructions present in `README.md` & `docs/product.md`).
  8. Agent frontmatter validity (`agents/*.agent.md`).
  9. Fenced code block balance check.
  10. Terminology discipline check (`citation_scan.py` with `praxis:allow-term`).
  11. Template placeholder parity check.
  12. Required phrases check (`.praxis-canon.json`).
  13. Version single-source audit (`bump-version.sh --audit`).
  14. Relative markdown link resolution check.
  15. CHANGELOG structure validation (`## [version]` descending order check).
  16. Self-conformance declaration parity check.

---

## 4. Quality & NFR Invariants

- **Test Layer Mapping:**
  - *Probe Self-Tests:* `scripts/test-probes.sh` testing dirty fixtures in `scripts/__fixtures__/`.
  - *Citation Scanner Self-Tests:* `scripts/test-citation-scan.sh` testing fence, blockquote, and inline marker parsing.
  - *Composition Probe:* `scripts/validate-plugin.sh` verifying whole plugin integrity.
- **4 Production Readiness Anchors:**
  - *Observable:* Fast-fail output with exact line numbers and remediation instructions.
  - *Configurable:* `allowed_missing` set in `validate-plugin.sh` for illustrative/deleted historical paths.
  - *Scalable:* POSIX shell + Python standard library execution with zero external NPM/pip dependencies.
  - *Resilient:* `citation_scan.py` marker-length fence awareness preventing false negatives.

---

## 5. Capability History & Lineage

- **Initiatives Delivered:**
  <!-- praxis:allow-version-literal reason="cites release initiative" -->
  - [INIT.praxis-v0.6.0-consolidation](../product/initiatives/INIT.praxis-v0.6.0-consolidation.md) — Self-conformance declaration parity (Check #16) and `docs/product.md` inventory parity.
- **Durable Decisions:**
  - [ADR.260725](../architecture/adr/ADR.260725-inline-declared-exceptions.md) — Declared exceptions move inline (`praxis:allow-*` markers).
