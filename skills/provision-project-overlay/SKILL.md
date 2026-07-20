---
name: provision-project-overlay
description: Generate a project-specific `.github/` overlay (skills, agents, prompts, persona instructions) on top of an existing repo that has just installed the praxis plugin. Interview the human for stack, paths, persona aliases, and quality gates; write `praxis.config.yaml`; emit a managed set of overlay files from the plugin templates with placeholders substituted; optionally bootstrap `docs/project-context.md`, `docs/product/PRODUCT.md`, and a first ADR file that follows `create-adr` ID rules. Idempotent — re-running with `--reconfigure` re-runs the interview; re-running without it regenerates managed files from the current config and shows diffs for human-edited files before overwriting.
---

# Provision Project Overlay

## Use this when

- The plugin was just installed in an **existing** repository (greenfield uses `bootstrap-project` instead).
- The plugin shipped a new template version and the project should re-sync its overlays.
- A teammate adds a new persona alias or changes a doc path and wants the overlays updated consistently.

## Do NOT use this when

- The repository is empty and has no source code yet — use `bootstrap-project`.
- You are editing a single overlay file by hand. Hand-editing is allowed; this skill detects the diff and asks before overwriting.

## Concepts

- **Config file** — `praxis.config.yaml` at repo root. Source of truth for all substitutions. Survives plugin upgrades.
- **Templates** — files in `plugin/skills/provision-project-overlay/templates/` ending in `.tmpl`. Pure markdown / YAML with `{{placeholder}}` substitution.
- **Pointer-only overlays** — an overlay skill/instruction is a **thin pointer**: one paragraph naming the plugin skill it maps to ("read it first"), then project-specific paths, gates, and ownership. It must **never restate plugin doctrine** (tier tables, phase orders, the test pyramid, terminology) — that is what drifts. Two `validate-plugin.sh` lints enforce this: the terminology lint (forbidden legacy terms) and the placeholder-parity lint (every `{{…}}` resolves against the config). Keep overlays short; if one grows past ~40 lines, it is probably restating doctrine that belongs in the plugin skill.
- **Manifest** — `plugin/skills/provision-project-overlay/manifest.yaml` listing every managed `(template_path, target_path, condition)`. Files not in the manifest are never touched.
- **Substitution** — plain `{{key.nested}}` string replace. No conditionals, no loops. If a template needs branching, ship two templates and gate them with manifest `condition`. The one non-config token is `{{TODAY}}`, a **runtime placeholder** substituted with the current date (e.g. an ADR `Date:` line); it is allowlisted in `.praxis-canon.json` `specialPlaceholders`. Every other `{{…}}` must resolve against `praxis.config.yaml`.

---

## Steps

### Step 1 — Detect existing config

```
if exists(<repo-root>/praxis.config.yaml) and not flag --reconfigure:
    config = parse(praxis.config.yaml)
    skip to Step 3
else:
    proceed to Step 2
```

### Step 2 — Interview the human

Ask exactly these question groups. Wait for answers between groups.

**Group A — Project basics**

1. Project name (kebab-case, e.g., `acme-billing`)
2. One-line project description

**Group B — Stack** (capture for placeholders and stack-conventions overlay)

1. Primary language (typescript / python / go / rust / java / dotnet / other)
2. Runtime (deno / node / bun / python / jvm / dotnet / go-toolchain / rust-toolchain / other)
3. API framework (e.g., hono / express / fastapi / chi / axum / spring-boot / aspnet / none)
4. Database (e.g., surrealdb / postgres / mysql / mongodb / dynamodb / sqlite / none)
5. UI framework, if any (ionic-angular / react / vue / svelte / next / none)
6. Deployment target (cloud-run / lambda / k8s / single-binary / mobile-store / other)

If the human is unsure on any of these, push back. They shape the overlay; do not proceed with placeholders.

**Group C — Doc paths** (show defaults; accept Enter or override)

| Key                       | Default                   |
| ------------------------- | ------------------------- |
| `paths.product`           | `docs/product`            |
| `paths.architecture`      | `docs/architecture`       |
| `paths.guides`            | `docs/guides`             |
| `paths.adr`               | `docs/architecture/adr`   |
| `paths.waves`             | `docs/product/waves`      |
| `paths.sprints`           | `docs/product/sprints`    |
| `paths.engineering`       | `docs/engineering`        |
| `paths.project_context`   | `docs/project-context.md` |
| `paths.product_dashboard` | `docs/product/PRODUCT.md` |

**Group D — Quality gates** (show defaults derived from runtime; accept or override)

| Runtime    | Default `lint`                      | `format`                  | `type_check`            | `test`           |
| ---------- | ----------------------------------- | ------------------------- | ----------------------- | ---------------- |
| deno       | `deno lint`                         | `deno fmt`                | `deno check <entry>`    | `deno test`      |
| node / bun | `npm run lint`                      | `npm run format`          | `npm run typecheck`     | `npm test`       |
| python     | `ruff check .`                      | `ruff format .`           | `mypy .`                | `pytest`         |
| go         | `go vet ./...`                      | `gofmt -l .`              | (n/a)                   | `go test ./...`  |
| rust       | `cargo clippy`                      | `cargo fmt --check`       | `cargo check`           | `cargo test`     |
| jvm        | `./gradlew lint`                    | `./gradlew spotlessCheck` | `./gradlew compileJava` | `./gradlew test` |
| dotnet     | `dotnet format --verify-no-changes` | `dotnet format`           | `dotnet build`          | `dotnet test`    |

Optionally also: `e2e` command (omit if N/A).

**Group E — Persona aliases**

> "The plugin ships three role-based agents: `principal-engineer`, `product-manager`, `product-designer`. Want personality aliases (e.g., `rusty`, `manny`, `shelby`)? (default: no)"

If yes, ask one alias per role. If no, set `personas.use_aliases: false` and skip the alias-specific overlay files in Step 4.

**Group F — Bootstrap docs** (yes/no per artifact)

1. Generate `docs/project-context.md` skeleton? (default: yes if file does not exist)
2. Generate `docs/product/PRODUCT.md` skeleton? (default: yes if file does not exist)
3. Generate `docs/architecture/adr/ADR.<ID>-technology-stack.md` from the stack answers? (default: yes if file does not exist)
4. If yes, what is the first ADR `<ID>`? (must follow `create-adr` convention)
5. Generate `.claude/system-prompt.md` for Claude API / agentic use (Bedrock, Cursor, custom CLIs)? (default: no — only needed if the team uses Claude outside Claude Code)
6. Generate `.githooks/pre-commit` (anti-dumping + lint + format + type-check)? (default: yes; user must still run `git config core.hooksPath .githooks` to activate)

### Step 3 — Write `praxis.config.yaml`

Render `templates/praxis.config.yaml.tmpl` with the answers and write to `<repo-root>/praxis.config.yaml`. If file exists and `--reconfigure` was passed, show a diff and ask before overwriting.

### Step 4 — Emit overlay files

For each entry in `manifest.yaml`:

1. Evaluate `condition` (see manifest spec). Skip if false.
2. Read `template_path`. Apply substitution: every `{{key.path}}` is replaced with the value from `praxis.config.yaml` (dotted path lookup).
3. If the placeholder resolves to an alias-bearing path (e.g., `target_path: .github/agents/{{personas.aliases.principal_engineer}}.agent.md`), substitute in the **target path** as well.
4. Compute action vs the existing file:
   - **No file** → write
   - **File matches expected output** → skip (already current)
   - **File differs** → show unified diff and prompt: `[k]eep mine / [t]ake template / [m]erge manually`
5. Record outcome in a per-run summary.

### Step 4b — Copy the guardrail check scripts

Step 4 emits `scripts/verify.sh`, but that skeleton **calls** every `check-*.sh` guardrail script. Those scripts are shipped verbatim by the plugin (not templates — no substitution) and are **not** in `manifest.yaml`, so they must be copied here. Skipping this leaves `verify.sh` dead on arrival: step 4 (anti-dumping) fails with `exit 127` the first time anyone runs `bash scripts/verify.sh`.

Copy or symlink **every** `check-*.sh` from `<plugin-root>/scripts/` into the project's `scripts/`. Do not hand-pick a subset — `verify.sh` calls all of them, and a missing script breaks the quality gate on day one. (`validate-plugin.sh` is the one exception — it is a plugin self-test; copy it only if you want it.)

```bash
mkdir -p scripts
for src in "<plugin-root>"/scripts/check-*.sh; do
  cp "$src" scripts/
done
chmod +x scripts/check-*.sh scripts/verify.sh
```

Then run the parity check — every check `verify.sh` calls must exist on disk:

```bash
for s in $(grep -o 'check-[a-z-]*\.sh' scripts/verify.sh | sort -u); do
  [ -f "scripts/$s" ] || echo "MISSING: scripts/$s"
done
```

No output is the pass condition. Record the copied scripts in the per-run summary (`Scripts:` line).

### Step 5 — Validate inbound references

Scan these well-known anchor files (if present) for path references to managed overlay files:

- `<repo-root>/CLAUDE.md` (note: also generated by this skill as a thin pointer; if a human has expanded it, the diff-prompt in Step 4 protects their edits)
- `<repo-root>/GEMINI.md`
- `<repo-root>/CONTRIBUTING.md`
- `<repo-root>/.github/copilot-instructions.md`
- `<repo-root>/AGENTS.md`

For every reference like `.github/agents/<name>.agent.md` or `.github/skills/<name>/SKILL.md`, confirm the path exists. Report unresolved references — do not auto-edit anchor files other than the managed `CLAUDE.md` template (humans curate the rest).

### Step 6 — Run anti-dumping check

```bash
bash plugin/scripts/check-anti-dumping.sh .github
```

If it reports violations, surface them and stop. The user has hand-added something the plugin disallows.

### Step 7 — Print summary

Single block:

```
provision-project-overlay summary
─────────────────────────────────
Config:    praxis.config.yaml (created | unchanged | reconfigured)
Emitted:   <N> files
Skipped:   <N> files (already current)
Diffs:     <N> files (resolved: kept-mine=<a>, took-template=<b>, manual=<c>)
Scripts:   verify.sh + <N> check-*.sh copied (parity: ok | MISSING listed above)
Bootstrap: <list of bootstrap docs written>
Inbound:   <N> references checked, <M> unresolved (see list above)
Anti-dumping: clean | <N> violations
Next step: review praxis.config.yaml, commit when satisfied.
```

Do not auto-commit. Humans commit.

---

## Manifest spec (`manifest.yaml`)

```yaml
managed_files:
  - template: templates/praxis.config.yaml.tmpl
    target: praxis.config.yaml
    condition: always

  - template: templates/.github/instructions/README.md.tmpl
    target: .github/instructions/README.md
    condition: always

  - template: templates/.github/agents/role-engineer.agent.md.tmpl
    target: .github/agents/{{personas.aliases.principal_engineer}}.agent.md
    condition: personas.use_aliases == true

  # ... etc
```

Conditions are limited to:

- `always`
- `<key.path> == <literal>`
- `<key.path> != <literal>`
- `exists(<file-path>)`
- `not exists(<file-path>)`

Anything more complex is a sign the design is wrong — split the template.

---

## Substitution reference

| Placeholder                               | Source                                                                |
| ----------------------------------------- | --------------------------------------------------------------------- |
| `{{project.name}}`                        | `praxis.config.yaml` → `project.name`                              |
| `{{stack.runtime}}`                       | `praxis.config.yaml` → `stack.runtime`                             |
| `{{paths.adr}}`                           | `praxis.config.yaml` → `paths.adr`                                 |
| `{{personas.aliases.principal_engineer}}` | If `personas.use_aliases: true`, the alias; else `principal-engineer` |
| `{{quality_gates.test}}`                  | `praxis.config.yaml` → `quality_gates.test`                        |

Unrecognized placeholders are a hard error — do not emit silent empties.

---

## Stack conventions overlay

The skill emits `.github/instructions/stack-conventions.instructions.md` from a single generic template (`templates/.github/instructions/stack-conventions.instructions.md.tmpl`). The template has placeholders and a `## Stack-specific guidance (LLM-filled at provision time)` section that the running LLM fills with conventions appropriate to the answered stack. Examples to inform the fill:

- TypeScript + Deno + Hono → file-shape (entity / repository / service / api / mod), JSR imports, `query<T>()` pattern, single quotes
- Python + FastAPI → router / dependency / service / repository split, Pydantic schemas, ruff rules
- Go + chi → handler / service / repo, context propagation, table-driven tests
- Rust + axum → handler / service / repo, `Result<T, AppError>` pattern, `cargo clippy -- -D warnings`

The LLM does not invent rules — it transcribes well-known idioms for the given stack into the existing template structure (logging, error handling, file shape, quality gates). If the stack is unfamiliar, leave the section with `TODO: fill in <runtime> conventions` and warn the user.

---

## Anti-patterns

- **Conditional logic in templates.** Split templates instead.
- **Touching files not in the manifest.** Hand-curated personas (e.g., a custom `alex-research.md`) are sacred.
- **Auto-editing anchor files** (CLAUDE.md, copilot-instructions.md). Report; never write.
- **Silent overwrite of human edits.** Always diff and prompt.
- **Stack registry hard-coded into the plugin.** Stacks come from the user. The skill ships one generic stack template that the LLM fills.

---

## See also

- `bootstrap-project` — for empty repos (greenfield)
- `create-product-design-spec` — owns `templates/thin-slice-template.md`
- `lean-delivery-guardrails` — universal lean delivery rules referenced by the emitted methodology overlay
- `capability-driven-guardrails` — universal anti-dumping rules referenced by the emitted stack overlay
