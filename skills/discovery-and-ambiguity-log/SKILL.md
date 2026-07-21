---
name: discovery-and-ambiguity-log
mode: architect
tools: [read_file, file_search, grep_search, semantic_search, create_file, replace_string_in_file]
description: Phase 1 of the principal-engineer workflow (architect mode). Use before any non-trivial change to map context, define SLOs/SLAs, and produce a structured Ambiguity Log. Stops the work and asks for clarification before proceeding to design. Architect mode forbids edits to source code; writes are limited to docs and ADR paths.
---

# Discovery and Ambiguity Log

## Use this when

- Starting any non-trivial feature, refactor, or system change.
- A ticket or request lacks edge cases, security boundaries, or non-functional requirements.
- You're tempted to "just start coding" — that's the signal you skipped this phase.

## Do NOT use this when

- The change is a one-line fix with obvious behavior.
- A prior discovery log already exists and is current for this work.

## The iron rule

```
NO DESIGN OR CODE UNTIL THE AMBIGUITY LOG IS RESOLVED
```

If the user pushes to skip, push back. Resolving ambiguity now costs minutes; resolving it after implementation costs days.

## Steps

### Step 1 — Context audit

Map what already exists.

1. Identify the **target capability** this work belongs to. If none exists, propose a new capability name (single noun phrase, business-domain language).
2. List the **adjacent capabilities** that will be touched, called, or affected.
3. List the **external dependencies** (databases, queues, third-party APIs, file systems).
4. Note the **runtime and deployment context** (single process, distributed, edge, batch).
5. Note any **existing patterns in this codebase** that the work should follow.

### Step 2 — Non-functional requirements (SLO/SLA)

Define explicitly. If the user can't answer, that's an entry in the Ambiguity Log.

| Dimension     | Question                            | Example answer                                |
| ------------- | ----------------------------------- | --------------------------------------------- |
| Throughput    | Requests per second at peak?        | 50 RPS sustained, 200 RPS burst               |
| Latency       | p95 / p99 budget?                   | p95 ≤ 200ms, p99 ≤ 500ms                      |
| Availability  | Target uptime?                      | 99.9% monthly                                 |
| Consistency   | Strong, eventual, read-your-writes? | Strong for writes, eventual for read replicas |
| Durability    | Data loss tolerance?                | Zero loss; replicate before ack               |
| Recovery      | RTO / RPO?                          | RTO 1h, RPO 5min                              |
| Scale horizon | 6-month projection?                 | 10× current load                              |

### Step 3 — Security and compliance boundaries

- What data crosses a trust boundary?
- What is PII / regulated / secret?
- What is the authentication and authorization model?
- What is logged vs. minimized?
- Any compliance regime (SOC2, GDPR, HIPAA, PCI)?

### Step 4 — Failure surface

- What breaks when each external dependency goes down?
- What is the degraded behavior?
- Where are the retry boundaries?
- Where are idempotency keys required?

### Step 5 — Produce the Ambiguity Log

Write a Markdown table. One row per unresolved question.

```markdown
## Ambiguity Log

| # | Question                                    | Why it matters                       | Default if unanswered         |
| - | ------------------------------------------- | ------------------------------------ | ----------------------------- |
| 1 | What is the p99 latency budget?             | Drives sync vs. async design choice  | Assume 500ms; flag for review |
| 2 | Can the user retry the same request safely? | Determines need for idempotency keys | Assume no; require keys       |
| 3 | Is event ordering required across users?    | Determines partition strategy        | Assume per-user only          |
```

### Step 6 — Stop

Output:

1. **Context map** — capabilities touched, dependencies, runtime.
2. **SLO/SLA table** — filled where known, empty rows flagged.
3. **Ambiguity Log** — one row per open question with `default if unanswered`.

Then say: **"Please resolve the Ambiguity Log before I proceed to Phase 2 (system architecture)."**

Do not write architecture, code, or tests yet.

## Anti-patterns

- Filling in SLOs by guessing without flagging them.
- Marking the log "resolved" without a human answering each question.
- Treating the log as one-shot — if new ambiguity surfaces in later phases, return here and update.
- Skipping context audit because "I already know this codebase."

## Output template

```markdown
# Discovery — <feature or change name>

## Context Map

- Target capability: `<name>` (new | existing)
- Adjacent capabilities: …
- External dependencies: …
- Runtime: …
- Existing patterns to follow: …

## SLO / SLA

| Dimension | Target | Source |
| --------- | ------ | ------ |
| …         | …      | …      |

## Security and Compliance

- Trust boundaries: …
- Regulated data: …
- AuthN / AuthZ: …
- Logging policy: …

## Failure Surface

- Dependency `<X>` down → <degraded behavior>
- Retry boundaries: …
- Idempotency requirements: …

## Ambiguity Log

| # | Question | Why it matters | Default if unanswered |
| - | -------- | -------------- | --------------------- |
| 1 | …        | …              | …                     |

**Awaiting human resolution before Phase 2.**
```
