# Fixture-based Research and Knowledge workflow

Status: Verified complete with advisories; publication pending
Date: 2026-08-12
Gate ID: `agent-research-knowledge-workflow`
Plan: `docs/plans/2026-08-11-research-knowledge-workflow.md`
Decision: D-086, preserving D-079 and D-082 through D-085
Baseline: clean synchronized `main` at `5e53f55`

## Goal

Implement exactly one deterministic, Rust-only Personal Assistant -> Research
Agent -> Knowledge & Document Agent -> Personal Assistant synthesis workflow
above the unchanged `AgentRuntime`, using only application-supplied fixtures
and without introducing a provider, retrieval path, general workflow engine,
persistence, IPC, or UI.

## Implemented boundary

- `ResearchKnowledgeWorkflowRequest` owns a bounded objective and immutable
  catalog of one to eight deterministic fixture sources. Each source has a
  canonical opaque ID, bounded label, and bounded untrusted evidence.
- `ResearchResult` is a strict V1 structured value with bounded findings,
  closed confidence, known source references, unresolved questions,
  limitations, and optional follow-up. Unknown fields, URLs, reasoning,
  malformed content, unknown or duplicate IDs, and bound violations fail
  closed.
- `KnowledgeResult` is a strict V1 transformation with thematic sections,
  extracted facts, contradictions, summary, optional pending reusable-
  knowledge proposal, and artifact outline. Its references must already exist
  in the validated Research result.
- The final Personal synthesis is a strict bounded fixture-disclosed V1 result;
  its references must be issued by the same application catalog. It cannot
  claim live research, expose reasoning, use an inconsistent complete/partial
  status, or add arbitrary citations.
- `AgentOrchestrator` alone creates both specialist tasks as sequential
  depth-one siblings under the Personal root. Research never spawns Knowledge.
  Successful execution uses four runtime runs, three tasks, two
  non-replenishing children, one active child, and zero automatic retries.
- Workflow events expose Research started/completed, Knowledge organization
  started/completed, synthesis started, partial failure, cancelled, and
  completed. Bounded redacted attribution records are observational only and
  expose no runtime authority.
- Research and Knowledge may use only their exact live private/task memory.
  Task-temporary memory is cleaned at terminal state. Reusable Knowledge
  remains `PendingReview` and never becomes approved shared memory
  automatically.

## Failure and provenance behavior

Research failure skips Knowledge and starts truthful fallback synthesis when
possible. Knowledge failure or incomplete output preserves only validated
Research data for partial synthesis. Missing references are explicitly
partial; unknown references are invalid. Cancellation is child-first, starts
no later stage, and leaves no live orphan. A failed cancellation preserves the
retryable live state. Knowledge or synthesis start failure cannot reverse an
already accepted terminal runtime event and is recorded through the closed
continuation-failure status.

Fixture IDs are correlation and provenance identifiers, not claims of factual
truth. The deterministic demo identifies the evidence as fixture-based. No live
external research, retrieval, provider, or factual verification occurred.

## Preserved boundaries

Generic delegation remains exactly Personal Assistant to Research; direct
Research-to-Knowledge and specialist spawning remain denied. The separate
D-085 Personal-to-Knowledge approved-document route is unchanged.
`AgentRuntime` and `NativeAgentRuntime` are unchanged; Native remains the only
default. No Coding, QA, Security, Cloud, Systems Operations, or Workflow
Automation agent was activated. No dependency, manifest, lockfile, Tauri
command, React consumer, storage schema, network, filesystem discovery,
provider, credential, tool execution, permission, Hermes integration, or
user-visible behavior was added.

## Evidence

- Research/Knowledge units: 12 passed.
- Orchestrator units: 10 passed.
- Public D-086 workflow contract: 18 passed, including deterministic
  success, strict provenance, partial failure, cancellation, bounds, start
  failures, terminal preflight, memory cleanup, attribution, event order, and
  Native regression.
- Existing memory/document, generic orchestration, governance,
  definition/registry, runtime, and gateway contracts passed.
- Rust formatting, all-target/all-feature check, strict Clippy, all-target
  tests, complete `npm run verify`, documentation, repository, security, diff,
  and session-end gates passed.
- No manual application check was required because the new Rust boundary is
  unwired and no Tauri/React behavior changed.

Quality result: `PASS WITH ADVISORIES`. The accepted implementation advisory is
a future code-health opportunity to replace the private continuation-failure
getter comparison with a more explicit typed internal application outcome.
This does not affect the closed public result or permit false success. No later
plan is Ready; next-increment readiness is `Blocked`.

## Rollback

Before publication, remove the workflow module and contract, restore the
declared orchestrator/module/Knowledge-definition/test-fixture changes, and
revert only D-086/current-state documentation. Preserve D-079 and D-082 through
D-085 and every historical increment/review. There is no data, migration,
dependency, credential, provider, process, network, IPC, or external state to
reverse. After publication, use one bounded revert plus an additive
superseding decision rather than rewriting D-086 history.
