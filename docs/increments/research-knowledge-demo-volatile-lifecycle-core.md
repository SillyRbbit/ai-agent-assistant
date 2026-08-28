# Research/Knowledge demo volatile lifecycle core

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-27

## Goal and outcome

Add the smallest Rust-backed lifecycle prerequisite for a later truthful
interactive demo: one manually stepped, process-local owner drives the existing
sealed D-086 Research -> Knowledge workflow through `NativeAgentRuntime`. The
host remains unwired to Tauri and React and has no user-visible behavior.

## Implemented scope

- `ResearchKnowledgeDemoHost` exposes only no-argument `start`, `advance`,
  `cancel`, and `snapshot` methods through a public Rust-only contract.
- Production construction fixes the native runtime, D-086 objective, two
  synthetic sources, success script, response identities, and every runtime
  event. A synthesis-failure script and fault runtime exist only under tests.
- A closed v1 snapshot retains the exact
  `DEMO MODE · SIMULATED AGENT DATA` disclosure, separate-proof statement,
  fixture provenance, Rust-issued epoch/revision, eight-entry journal cap, and
  finite lifecycle/event vocabulary. It serializes no task, run, request,
  context, objective, source, result, path, URL, or raw error.
- Cancellation remains child-first and idempotent. Returned-runtime mismatch
  and cancellation faults retain cleanup ownership, block restart, and retry
  only through no-argument cancellation.
- Persistent destructor-time cleanup failure retains the owner until process
  exit and sets a private process-wide sentinel. Every replacement `start`,
  `advance`, and `cancel` then fails closed as `cleanup-pending`; production has
  no reset path.

## Boundaries preserved

No `src-tauri/src/agent/**` implementation changed. No Tauri command, event,
managed state, capability, CSP, permission, client, React component, timer,
thread, async task, polling, worker, provider, model, network, credential,
tool, approval dispatch, persistence, durable audit, filesystem, dependency,
generic workflow engine, or device effect was added. The current read-only
Command Center projection, Conversations mock, and sealed Rust workflow remain
separate deterministic proofs.

## Focused evidence

- Lifecycle module: Passed, 9/9; 0 failed, 0 ignored.
- Public Rust-only lifecycle contract: Passed, 1/1; 0 failed, 0 ignored.
- Strict Rust Clippy and rustfmt: Passed.
- Sealed-agent acceptance: Passed, 468/468; 0 failed, 0 ignored.
- Returned-identity quarantine, cancellation retry, process-wide Drop
  quarantine, replacement blocking, redaction, bounds, success, failure,
  stage cancellation, restart, and late-step rejection are covered.
- Duplicate-live identity is structurally unavailable in this sequential host;
  exact shared-orchestrator rejection remains covered by the existing runtime
  and bounded-parallelism contracts.

## Target-Mac evidence

- Pinned Node 26.3.0, npm 11.16.0, Rust/Cargo 1.90.0 ran on macOS 26.6 build
  25G72 arm64.
- Complete `npm run verify`: Passed with 28 hook tests, 57 repository tests,
  247 frontend tests, 261 Rust library tests, 243 Rust integration tests, the
  frontend production build, and the Tauri release no-bundle build.
- One explicitly opt-in Hermes version probe remained intentionally ignored.
- Rendered UI, IPC, viewport, theme, reduced motion, focus, scroll, zoom, and
  native resize checks: Not run and not required because this increment has no
  Tauri or user-visible boundary.

## Review and advisories

Independent architecture, security, code, technical-debt, readiness, and
quality review pass with advisories. The process-wide sentinel is only a
fail-closed destructor guard, not a concurrency coordinator; a future Tauri
adapter must own one synchronized managed host and separately review command
ordering. A replacement host's `snapshot()` remains host-local and may be
`idle` while its mutating operations accurately return `cleanup-pending`. Two
Low items are deferred: fixed JSON duplicates existing acceptance fixtures, and
the containment path assumes the frozen vocabulary remains below the journal
cap. No later adapter or presentation increment is planned, Ready, or
authorized by this work.

## Completion state

The bounded source and complete verification pass. Documentation, repository,
secret, diff, session-end, and post-increment marker checks complete the same
source gate. The result is `PASS WITH ADVISORIES`; no required check failed and
nothing was committed, pushed, merged, released, or published.
