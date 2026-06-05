# Plan — Executable Seams First, Parallelism as Emergent

**Status:** Shipped — B3-config (`check-config-externalized.sh`), the Bundle A keystone (`define-seam-contract` + `check-seam-contract-parity.sh` + architecture-spec seam declaration + verify wiring), and Bundle B B1/B2 (wave Production-Readiness posture + per-slice conformance block + intake gate). Next per build order: remaining B3 probes (observability, statelessness, resilience) → C → D.
**Author:** Principal Engineer (architect mode)
**Supersedes the roadmap of:** `thin-slice-loop-hardening.md` (Bundles 1–3, already shipped) — this is the Phase 2 direction.
**Deliverable of this doc:** an implementation plan only. No skill/script/template changes until approved.

---

## 1. Thesis

> **Quality is a property of the seam, not the slice. Speed is already provided by GenAI. The method's job is to convert seam-correctness from *asserted* to *executed* — and let parallelism fall out as a safe consequence of machine-checkable contracts.**

Three claims this plan is built on, settled during sparring:

1. **The seam is the quality unit.** The four runtime anchors — observable, configurable, horizontally scalable, resilient — are all *cross-boundary* properties. They live at seams (Port/Adapter, contract, capability edge), not inside a slice. Optimizing the *slice* for throughput works against them.
2. **GenAI is the speed leverage — not the process.** We do not need a parallel scheduler to go fast; the model is already fast. Spending method-complexity on a throughput scheduler buys little and risks quality. Method-complexity should buy *quality assurance the model cannot fake*.
3. **Parallelism is an emergent permission, not a designed-in goal.** Once a seam's contract is machine-checkable, crossing it wrong is a red build. *That* is what makes two slices safe to build at once — not a coordinator. Build executable contracts; parallelism becomes a safe side-effect.

### The GenAI inversion (why this ordering is right for AI specifically)

Classic methods assume **humans are the bottleneck**, so they optimize for human throughput (parallel teams, scheduling). With GenAI the bottleneck inverts:

| | Human-era constraint | GenAI-era constraint |
|---|---|---|
| Scarce | Implementation hours | **Trustworthy verification** |
| Cheap | — | Generating code, tests, docs, slices |
| Failure mode | Too slow to build | **Plausible-but-wrong, fast** |
| Right lever | Parallelize builders | **Make correctness executable** |

GenAI makes *generation* nearly free and *self-attestation* nearly worthless (a model will happily write `Idempotency: N/A ✓` and a green-looking test that asserts nothing). So the highest-value method investment is the one thing the model cannot fake: **an executable gate that fails the build when a seam is wrong.** Everything in this plan follows from that.

---

## 2. What this plan is NOT

To kill the throughput instinct cleanly:

- **No parallel scheduler.** No L1 inter-slice coordinator, no DAG engine, no program-level orchestrator. GenAI provides the speed; we provide the safety rails.
- **No mandatory intra-slice (L2) step-DAG.** The 5-slice trace showed thin slices barely benefit intra-slice; a mandatory DAG is ceremony. Independent steps may still run concurrently when obvious — but that's the agent's discretion, not a required artifact.
- **Parallelism is permitted, never required.** When contracts are executable and resources disjoint, the human (or an orchestration runtime like MPM) MAY run slices concurrently. Praxis makes it *safe*; it does not make it *happen*.

This keeps the lean/ephemeral ethos intact and concentrates all new complexity on executable verification.

---

## 3. The core construct — the Seam Contract

A **Seam Contract** is the machine-checkable description of a boundary, plus the executable suite that proves both sides honor it. It is the single new first-class artifact this plan introduces.

A seam is any boundary where one unit depends on another's promise:
- A **Port** (capability ↔ adapter) — already half-built in Praxis via Port/Adapter parity.
- A **public API / event / CLI contract** (capability ↔ consumer).
- A **cross-capability call** (capability ↔ capability public surface).

A Seam Contract has three parts:

| Part | Form | Praxis status today |
|---|---|---|
| **Shape** | Machine-readable interface — typed signature, OpenAPI/JSON-Schema, protobuf, event schema | ⚠️ Implied in `product-architecture.md` prose; not machine-readable |
| **Behavior** | The shared contract test suite that both sides must pass (timeout, retry, idempotency, error envelope, ordering) | ✅ Exists as concept — Adapter Contract layer in `test-by-ownership` |
| **Version** | A pinned identifier a dependent slice builds against (`contract@v1`), frozen independently of the producer slice's completion | ❌ Absent — dependencies point at slices/snapshots, not frozen contract versions |

**Why version matters and isn't bureaucracy:** today a slice depends on *another slice* (or its frozen engineering snapshot), so it can't start until that slice merges, and the snapshot rots the moment anything else merges. If instead a slice depends on `contract@v1` — published and frozen *before* the producer's internals are done — the dependent can start early AND is immune to unrelated merges. The version is exactly what converts "wait for merge / snapshot rots" into "build against a frozen promise." It is the mechanism that makes parallelism emergent.

---

## 4. The four anchors become executable probes

Each runtime anchor gets one cheap, language-pluggable executable probe wired into the project `verify` entry point. None needs to be perfect; each converts an anchor from *asserted in a checklist* to *proven by the pipeline*. All ship as Praxis enforcement scripts (the strongest existing rail — we add to it, we don't invent softer markdown).

| Anchor | Probe (fails the build when…) | Form |
|---|---|---|
| **Configurable** | a request/boundary path contains a hardcoded URL, host, port, or secret literal instead of reading config | `scripts/check-config-externalized.sh` (grep-class, allowlist-driven) |
| **Observable** | a new boundary/cross-process call has no structured log with a correlation ID and no metric | `scripts/check-observability-at-seams.sh` |
| **Horizontally scalable** | a request-scoped path introduces node-local mutable state (module-level mutable singletons, in-process caches on the hot path) without an opt-out annotation | `scripts/check-stateless-request-path.sh` |
| **Resilient** | an external/boundary call has no declared timeout and no fallback/retry policy | `scripts/check-resilient-boundary.sh` |

**Honesty about the probes:** these are heuristic, not proofs. They are deliberately *conservative* (flag + require an explicit, reviewed opt-out comment) so they catch the boundary-blind violation the 5-slice trace exposed (TS-3's node-local auth cache) without blocking legitimate exceptions. Each opt-out is a reviewable artifact — the bias firewall is that silence fails, not that the agent promises.

---

## 5. Work breakdown — sequenced so each step has standalone value

Ordered by leverage. Each bundle is shippable alone; later bundles depend on earlier ones, never the reverse.

### Bundle A — Make the seam executable (the keystone) — **highest leverage**

The one investment that raises quality *and* de-risks future parallelism in a single move.

- **A1. Seam Contract as an artifact.** New skill `define-seam-contract` (architect-mode): given a boundary, produce the machine-readable Shape, register the shared Behavior suite location, and assign a frozen `contract@vN` id. Writes to a project contracts directory; collision-safe ids mirroring `create-adr`.
- **A2. Contract conformance in `verify`.** Extend `verify-and-assemble-pr` Step 3: every touched seam's shared contract suite must run and pass against **both** sides (consumer-driven). This generalizes the existing Port/Adapter parity gate from Ports to all seams. Add `scripts/check-seam-contract-parity.sh` (every declared seam has a Shape + a Behavior suite that ran).
- **A3. `product-architecture.md` declares seams + version.** Extend `create-product-architecture-spec`: the wave names its seams and the contract version each carries. This is where the cross-boundary view gets an owner *before* slices fork (the antidote to boundary-blind parallel decisions).

**Standalone value even with zero parallelism:** contract-conformance turns "TS-3 honors TS-1's contract" from a sentence into a green test. That is a quality gain on serial work alone.

### Bundle B — Promote the four anchors to a Production-Readiness spine — **second leverage**

- **B1. Wave-level posture.** Extend `create-product-architecture-spec`: the wave declares its posture per anchor — *where does state live* (statelessness boundary), the *observability/correlation contract*, the *config & secrets strategy*, the *cross-slice failure model*. Decided with the whole in view; slices conform, they don't re-decide.
- **B2. Per-slice conformance, not re-derivation.** Extend `create-sprint` resilience checklist into a **Production-Readiness conformance** block: "this slice preserves the wave's stateless/observable/config/resilience posture" — with the seam(s) it touches named. Conformance to a central decision, not a per-slice litigation (avoids `N/A` theater).
- **B3. Executable probes.** Ship the four `scripts/check-*.sh` from §4, wired into `verify`. This is the move that makes B1/B2 real rather than asserted — and it closes the two anchors (configurable, horizontally scalable) that are absent today.

**Standalone value:** even one probe (recommend **observability** or **config** first) converts an anchor from prose to pipeline.

### Bundle C — Adversarial seam review — **quality multiplier, costs speed deliberately**

- **C1. Seam-behavior review lens.** Extend reviewer mode in `verify-and-assemble-pr`: beyond the structural matrix, the reviewer must attack *behavior at each touched seam* — "show the test where the upstream circuit opens mid-call," "prove this handler is idempotent under retry," "where is the correlation ID across this path." These map to existing coverage scenarios but become an **adversarial gate run by a different head/persona**, not implementer self-check.
- **C2. Property over example at high-risk seams.** Extend the AC↔test matrix (shipped in Bundle 2 of Phase 1): for risk-tier-high ACs at a seam, require a **property/contract test** (idempotency ∀ keys, concurrent op linearizable) rather than a single example. Sharpens, doesn't replace, the matrix.

**Explicit tension accepted:** C *slows* each slice. That is the correct trade when the anchor is quality and GenAI already supplies speed. This is the bundle that most embodies the decision "speed from AI, quality from method."

### Bundle D — Parallelism as emergent permission (smallest, last) — **enabling, not driving**

Only the *safety* primitives, no scheduler:

- **D1. Three-axis disjointness rule.** Document (in `using-praxis` + capability guardrails) that two units may run concurrently only if disjoint across **capability/files**, **persistent resources** (tables, topics), and **shared config keys** — and each depends only on a **frozen `contract@vN`**, never on the other's in-flight internals. (The 5-slice trace proved capability-disjointness alone is unsafe.)
- **D2. Collision-safe coordination artifacts.** Collision-safe sprint ids (mirror the ADR scheme); per-slice status records instead of one shared README table edit; so N concurrent slices don't collide on the coordination layer before they collide on code.
- **D3. Snapshot staleness re-anchor at intake.** Extend `intake-code-contribution`: if the sprint sat queued while siblings merged, re-check whether any seam/contract the slice depends on changed since freeze; re-anchor before coding. The price of parallelism touching the immutable bridge — paid as a check, not by breaking immutability.

**Why last:** none of this is worth building until seams are executable (Bundle A). With A in place, D is small — disjointness + collision-safe ids + a staleness check — and parallelism becomes *permitted and safe*, exercised by the human or an orchestration runtime, never forced by Praxis.

---

## 6. Naming the conflations we are deliberately keeping separate

| Concept | What it is | Where it lives |
|---|---|---|
| **Build-time parallelism** | Many slices built at once | Emergent (Bundle D), never required |
| **Runtime horizontal scale** | Produced code scales across nodes | An *anchor*, probed (Bundle B3) |
| **Intra-slice step concurrency** | Independent steps within one slice | Agent discretion; no mandatory artifact |

These three were collapsed in earlier sparring. The plan keeps them apart so no downstream gate blurs "built in parallel" with "scales in parallel."

---

## 7. Pre-mortem on THIS plan

Six months out, it failed. Why, and the mitigation baked in:

| Failure mode | Mitigation |
|---|---|
| Probes are too noisy → teams disable them | Conservative + explicit reviewed opt-out comment; silence fails, not legitimate exceptions. Ship one probe first, tune, then expand. |
| Seam Contracts become bureaucratic overhead | Only *declared* seams (named in `product-architecture.md`) carry contracts; not every function. Reuse existing Port/Adapter suite machinery. |
| Contract versioning ignored, deps still point at slices | The conformance gate (A2) fails if a dependent built against an unversioned/unfrozen seam — versioning is enforced, not encouraged. |
| Adversarial review (C) seen as friction and skipped | It is a *gate by a different persona*, not a checkbox; PR rejected without it. Tension is intentional and owned. |
| We secretly rebuild a scheduler | §2 + §5 Bundle D scope-lock: safety primitives only, no orchestration. |
| Probes give false confidence (heuristic ≠ proof) | Documented as heuristics; Bundle C's property tests carry the real proof at high-risk seams. Probes catch the careless; properties catch the subtle. |

The top historical quality failure — *"forty seams asserted-correct in markdown, never executed-correct"* — is addressed head-on by Bundle A, which is why it is the keystone.

---

## 8. Sequencing summary

1. **Bundle A** — executable Seam Contract (artifact + conformance gate + architecture declaration). The keystone; raises quality on serial work and is the precondition for safe parallelism.
2. **Bundle B** — four anchors as wave posture + per-slice conformance + executable probes. Closes the two absent anchors.
3. **Bundle C** — adversarial seam review + property tests at high-risk seams. The deliberate speed-for-quality trade.
4. **Bundle D** — emergent-parallelism safety primitives (disjointness, collision-safe ids, staleness re-anchor). Small, last, enabling only.

**Resolved start point** (see §9 D1): ship **one anchor probe first — `check-config-externalized.sh` (B3-config)** — to prove the executable-gate pattern end-to-end (script + `verify` wiring + reviewed opt-out + warn→fail promotion) against real code, *then* build the keystone **seam contract conformance gate (A2)** with the pattern already proven and copy-able.

**Resolved build order:** `B3-config → A2 (keystone) → rest of A → B1/B2 → remaining B3 probes → C → D`.

---

## 9. Resolved decisions (sign-off)

Four decisions, signed. These are the answers to the open questions and govern implementation.

### D1 — Start point: anchor probe first, then keystone

**Decision:** Ship `check-config-externalized.sh` (B3-config) as the first executable gate; build A2 (seam contract conformance) immediately after, reusing the proven gate mechanic.

**Rationale:** The thesis is *convert asserted to executed*. A single grep-class probe proves that mechanic — script, `verify` wiring, reviewed opt-out convention, tuning — in near-zero design surface against real code. The Seam Contract is higher-leverage but heaviest (new artifact, id scheme, conformance gate, architecture-spec changes); building the probe pattern first makes A2's "executable gate wired into `verify` with a reviewed opt-out" mechanic already proven when it is built. **Config** specifically: most deterministic to detect, highest security value, and one of the two anchors absent today — the fastest unambiguous win.

### D2 — Contract Shape format: standardized per interface kind, project-overridable

**Decision:** Praxis declares a default Shape form per interface kind — **OpenAPI** for HTTP, **JSON-Schema** for events/messages, **native typed interface** for Ports — and `praxis.config` MAY override per project.

**Rationale:** Fully project-declared formats guarantee drift and make `check-seam-contract-parity.sh` impossible to write generically (it cannot validate "a Shape exists" if every project invents the format). A fixed default per kind makes the parity check mechanical; the override valve preserves polyglot reality. Mirrors how Praxis already treats `verify` — convention with a config escape hatch.

### D3 — Probe strictness: warn-first, mechanical promotion to fail-closed

**Decision:** Each probe ships **warn-first**. Promotion criterion is mechanical, not optional: once a wave closes with zero un-opted-out hits, the probe flips to **fail-closed** for that project.

**Rationale:** Fail-closed on day one against an existing codebase produces a wall of legacy hits → teams disable the probe (the §7 top failure mode). Warn-first lets the team burn down or opt-out existing hits *with review* before the gate bites. Warn-forever is theater — so promotion is a defined criterion, not a preference. This hardens the §7 mitigation from "conservative" to "conservative *and* sequenced."

### D4 — Adversarial review persona: separate context preferred, same-agent mode-switch as recorded fallback

**Decision:** Bundle C's adversarial seam review defaults to a **genuinely separate session/agent** (or an orchestration runtime dispatching a second head). When unavailable, permit a same-agent **explicit reviewer-mode switch with a fresh read of the diff** — and record in the ledger which path was used.

**Rationale:** The value of C is adversarial *independence* — a head that did not write the code attacking the seam. Mandating a separate agent universally makes the gate unusable in a solo session, so it gets skipped (C's failure mode). The fallback keeps it always-runnable; recording the path keeps it honest and lets a reviewer weight a same-agent review appropriately. Fits Praxis's existing reviewer-is-a-different-head framing without inventing runtime orchestration it disclaims.

### D5 — Meta-loop: new gates are themselves under plugin validation

**Decision (added):** Every new `scripts/check-*.sh` (the four anchor probes and `check-seam-contract-parity.sh`) is registered the same way the existing four enforcement scripts are, and is covered by `validate-plugin.sh`.

**Rationale:** The rails that enforce code quality must themselves be under the plugin's own validation — otherwise a broken or unregistered gate silently stops gating. Small, but it closes the meta-loop: the quality system validates its own quality instruments.
