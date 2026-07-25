# Praxis — Product

Product overview and dashboard for Praxis itself. This is the `docs/product/README.md` anchor that Praxis instructs host repos to keep: the live view of what is being built, for whom, and where each wave stands.

**Product intent lives here. Engineering truth lives in [`../architecture/README.md`](../architecture/README.md).** Wave documents are *educated theory*; the architecture tree is validated truth, promoted there by `close-sprint`.

---

## What Praxis is, in product terms

**User:** an engineer or product owner working through an LLM coding agent (Claude Code, Codex, Cursor, Gemini CLI, OpenCode, Copilot).

**Outcome they want:** delivery artifacts an agent produced that a human team can build on without re-deriving the reasoning.

**The problem being solved:** trust transfer. An agent-generated architecture document looks identical whether the agent reasoned hard or pattern-matched a template. Praxis makes the difference visible. See [`../project-context.md`](../project-context.md) § 2.

---

## Wave dashboard

| Wave | Intent | Status | Slices |
| ---- | ------ | ------ | ------ |
| [wave-method-spine](waves/wave-method-spine/README.md) | One ordered method carries work from intent to closed, instead of the agent improvising a process. | ✅ Delivered† | 4 — 4 ✅ |
| [wave-multi-harness-reach](waves/wave-multi-harness-reach/README.md) | Identical agent behavior across six harnesses from one source tree. | ✅ Delivered† | 3 — 3 ✅ |
| [wave-executable-seams](waves/wave-executable-seams/README.md) | Build against a frozen promise instead of waiting for a merge. | 🔄 In Progress† | 5 — 4 ✅, 1 ⚪ |
| [wave-production-readiness](waves/wave-production-readiness/README.md) | Runtime posture decided once at the wave, conformed mechanically per slice. | ✅ Delivered† | 4 — 4 ✅ |
| [wave-trust-transfer](waves/wave-trust-transfer/README.md) | A human can tell whether the agent reasoned or pattern-matched. | ✅ Delivered† | 5 — 5 ✅ |
| [wave-self-conformance](waves/wave-self-conformance/README.md) | Every convention and gate Praxis prescribes is one it demonstrably follows and runs against itself. | 🔄 In Progress | 11 — 9 ✅, 2 ⚪ |
| [wave-brownfield-adoption](waves/wave-brownfield-adoption/README.md) | A product that already ships can adopt the method against what it has. | ⚪ Not Started | 2 — 2 ⚪ |

**† Delivered before wave adoption.** These waves are *derived records*, reconstructed from release history, ADRs, and the capability records after the work shipped. They carry a README only — no hypothesis card, no acceptance criteria, no educated-theory documents — because none were written at the time, and inventing them retroactively would assert validated learning that never happened. Each slice cites its evidence instead. The derivation rules, and the distinction between deriving from truth and fabricating history, are recorded in [ADR.260725.10](../architecture/adr/ADR.260725.10-brownfield-wave-retrofit.md).

**Honest status:** the open waves are where the real gaps are. **Self-conformance is not real-repo validation:** the [evolution policy](../project-context.md) is explicit that dogfooding proves internal consistency, not that a rule improves fidelity on real work, and that bar remains unmet. Most candidate validation projects are brownfield, which is why [wave-brownfield-adoption](waves/wave-brownfield-adoption/README.md) is a precondition for meeting it honestly rather than a side quest. Provenance of the self-conformance wave, including the deviations accepted along the way, is recorded in [ADR.260724](../architecture/adr/ADR.260724-wave-category-relaxation.md).

---

## Where planning intent lives

All planning intent lives in [`waves/`](waves/), authored with [`create-wave`](../../skills/create-wave/SKILL.md). There is no separate planning directory.

Praxis previously kept pre-adoption engineering plans under `docs/plans/`. That archive is gone: the delivered work those plans described is now cited as evidence by the derived waves above, and the one commitment that was still open — the sprint footprint and disjointness probes — is authored as `TS-005` of [wave-executable-seams](waves/wave-executable-seams/README.md), with its full design in that wave's [product-architecture.md](waves/wave-executable-seams/product-architecture.md).

The removal was a migration, not a discard: nothing open was dropped, and nothing shipped was rewritten into wave shape. That second half is the constraint that governs the retrofit — deriving an intent map from records that already exist invents nothing, while rewriting a shipped plan into a wave complete with acceptance criteria it never had would assert learning that never happened. See [ADR.260725.10](../architecture/adr/ADR.260725.10-brownfield-wave-retrofit.md).

This closes a real drift rather than reframing it. [`../architecture/README.md`](../architecture/README.md) previously opened by stating that planning-stage intent lives in `docs/plans/` — contradicting the authority statement `bootstrap-project` Step 7 mandates for every host repo. Calling both "true" was the more comfortable reading, but the architecture tree's first line is a single-valued claim about where intent lives, and it named the wrong directory while omitting the `promoted by close-sprint` clause. It now matches the mandated template exactly, and `waves/` is the single home for planning intent.
