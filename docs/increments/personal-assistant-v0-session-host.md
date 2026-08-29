# Personal Assistant v0 session-host increment

Status: Verified complete
Owner: Henry Dang
Date: 2026-08-28
Baseline: `8e382e813b42c1615e2319b369ca7561f164f0a3`
Plan: [2026-08-28-personal-assistant-v0-session-host.md](../plans/2026-08-28-personal-assistant-v0-session-host.md)

## Goal

Implement V0-2's smallest transport-free volatile Personal Assistant Rust
session owner: opaque presentation correlation, chronological bounded updates,
deterministic deadlines, cancellation cleanup ownership, restart, and late-
event rejection around the existing sealed V0-1 Native run.

## Scope

- Preserve the no-argument sealed `start_synthetic()` and sole
  `AgentRuntime::start` boundary.
- Add Rust-issued opaque handles plus closed non-Serde start, snapshot, update,
  batch, failure, and error types.
- Own at most one process-wide run, 128 updates, 16 updates per page, and exact
  connect/idle/provider/total monotonic deadlines.
- Retain ambiguous rejected-run or cancellation cleanup with its process lease
  and full volatile session record; reject late, foreign, stale, gapped, or
  contradictory results without mutation.
- Exercise deterministic success, failure, cancellation, limit, deadline, and
  cleanup branches through a private finite fixture driver, the real Native
  runtime event boundary, and the production-private record reducer.

## Non-goals

No user text, production response-frame ingress, transport, provider/model
request, network, signed identity, credential, Keychain, Tauri/WebView, UI,
persistence, memory, tool, approval dispatch, durable audit, filesystem,
background autonomy, device action, dependency, manifest, lockfile, capability,
CSP, permission, external state, commit, or publication.

## Acceptance evidence

- Public callers can supply only the opaque Rust-issued presentation handle and
  optional bounded poll cursor; they cannot choose a trusted run, request,
  agent, profile, runtime, provider, model, fixture, or workflow identity.
- The production-private reducer owns journal sequence, phase, prefix/final
  equality, delta/output bounds, deadline refresh state, and terminal updates.
  The event driver and application-owned success/failure fixtures do not compile
  into non-test builds.
- Exact returned runtime identity/status is checked before acceptance. Cleanup
  release requires consistent identity, cancellation outcome, and terminal
  status; contradictory or ambiguous cleanup remains Cancelling/quarantined.
- Focused tests prove success, every closed provider failure, cancellation from
  Starting/Streaming, deadline races, restart, pagination, limits, late-event
  rejection, identity/sequence faults, cleanup retry, Drop, and quarantine.
- Complete local verification and all builds pass. Manual scope and
  trust-boundary review found no source, dependency, IPC, permission, network,
  persistence, tool, filesystem, or device expansion.

## Result

`PASS WITH ADVISORIES`. V0-2 is locally verified on
`codex/personal-assistant-v0-session-host`, uncommitted and unpublished.
Success/provider-failure/streaming outcomes remain fixture-only; production
proves only fixed synthetic start, snapshot/poll, deadline terminalization,
cancellation cleanup, and restart. Target-Mac UI and external-system checks are
`Not run` because no such boundary exists. Those optional checks and Blocked
next-increment readiness produce the advisory classification; there is no
source finding. V0-3 remains Blocked by D-076 and TS-017; no successor
increment is Ready.
