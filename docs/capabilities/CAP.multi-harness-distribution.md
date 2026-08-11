# CAP.multi-harness-distribution: Multi-Harness Distribution & Session-Start Injection

**Domain Owner:** Praxis Core Maintainers  
**Status:** Active  
**Seam Contracts:** `harness-distribution@v1` (`.claude-plugin/plugin.json`, `.codex-plugin/plugin.json`, `.cursor-plugin/plugin.json`, `gemini-extension.json`)  

---

## 1. Domain Description & Bounded Context

- **Core Purpose:** Owns single-source distribution across six AI coding harnesses (Claude Code, Codex CLI/App, Cursor, Gemini CLI, OpenCode, and GitHub Copilot) from one `skills/` / `agents/` / `instructions/` tree.
- **Bounded Context:** Manages harness manifest declarations (`.claude-plugin/`, `.codex-plugin/`, `.cursor-plugin/`, `.opencode/`, `gemini-extension.json`), session-start context injection hooks (`hooks/session-start`), and overlay provisioning (`provision-project-overlay`).

---

## 2. User Experience & Living Journeys

- **User Personas Served:** Developers and AI agents operating in Claude Code, Codex, Cursor, Gemini, OpenCode, or Copilot.
- **Session-Start Context Journey:**
  1. User starts session or runs `/clear` in harness.
  2. `hooks/session-start` hook fires automatically.
  3. `skills/using-praxis/SKILL.md` is read in full and injected into agent session memory.
  4. Agent loads single-source personas (`agents/*.agent.md`) and always-on instructions (`instructions/*.instructions.md`) seamlessly.

---

## 3. Technical Architecture & System Topology

- **Architecture Layout:** `hooks/`, `.claude-plugin/`, `.codex-plugin/`, `.cursor-plugin/`, `.opencode/`, `gemini-extension.json`, `skills/provision-project-overlay/`
- **Manifest Adapters:**
  - `package.json` — Sole authored plugin version (`0.6.0`).
  - `.claude-plugin/plugin.json` & `marketplace.json` — Claude Code plugin manifest & marketplace index.
  - `.codex-plugin/plugin.json` — Codex CLI/App manifest adapter.
  - `.cursor-plugin/plugin.json` — Cursor manifest adapter.
  - `gemini-extension.json` — Gemini CLI extension manifest adapter.

---

## 4. Quality & NFR Invariants

- **Test Layer Mapping:**
  - *Manifest Parity Tests:* `bump-version.sh --check` verifying version equality across all 6 manifests.
  - *Version Audit Tests:* `bump-version.sh --audit` scanning for undeclared version literals.
  - *Injection Hook Tests:* POSIX shell syntax checks on `hooks/session-start`.
- **4 Production Readiness Anchors:**
  - *Observable:* `bump-version.sh --audit` outputs exact path:line locations for any version literal drift.
  - *Configurable:* `paths.product_root` in `praxis.config.yaml` rendered into host overlays.
  - *Scalable:* Zero content forks — 6 harnesses consume 1 single-source tree.
  - *Resilient:* `hooks/session-start` falls back gracefully if harness context injection API is unavailable.

---

## 5. Capability History & Lineage

- **Initiatives Delivered:**
  <!-- praxis:allow-version-literal reason="cites release initiative" -->
  - [INIT.praxis-v0.6.0-consolidation](../product/initiatives/INIT.praxis-v0.6.0-consolidation.md) — Unified distribution version bump to `0.6.0`.
- **Durable Decisions:**
  - [ADR.260725.17](../architecture/adr/ADR.260725.17-context-placement-beyond-one-repo.md) — Multi-repo context placement and precedence stack.
