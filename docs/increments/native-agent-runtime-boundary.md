# Native agent runtime boundary

Status: Verified complete with advisories
Date: 2026-08-11
Gate ID: `native-agent-runtime-boundary`
Plan: `docs/plans/2026-08-11-native-agent-runtime-boundary.md`
Baseline: clean synchronized `main` at `701c061`

## Goal

Introduce the smallest application-owned runtime event/lifecycle foundation and
adapt the verified native initial-turn path behind `NativeAgentRuntime` without
changing user-visible behavior or integrating Hermes.

## Approved boundaries

- Add only the closed Rust runtime contract, native composition wrapper, private
  deterministic contract fixture, focused tests, and approved closeout docs.
- Preserve `InitialGatewayTurn`, gateway validation, local schemas, policy,
  approval, audit, the visible React mock, and Tauri behavior.
- Add no Hermes/OpenClaw code, dependency, provider, model, network, process,
  credential, Tauri/React wiring, selector, automatic fallback, or execution.
- Keep tools, policy, approval, audit, memory, platform access, and secrets
  outside `AgentRuntime`.

## Implemented contract

- `AgentRuntime::{describe,start}` constructs one typed bounded run.
- `RuntimeRun::{run_id,identity,status,accept_event,cancel}` provides one closed
  untrusted event sink and exact local run lifecycle.
- `RuntimeRunIdentity` retains only redacted run/request correlation, allowing
  events to be constructed after the consuming start call without retaining
  selected content.
- `RuntimeCapabilities` is a fixed closed representation; declarations grant no
  authority. Native reports streaming text and does not report shared tool
  proposals.
- Event/request/error models use bounded application-owned types. Debug output
  redacts identity, selected text, response text, tool-call IDs, and arguments.
- `RuntimeError` and `NativeAgentRunError` remain closed and typed; no raw
  upstream payload or arbitrary failure string is retained.

## Native mapping

- `NativeAgentRuntime::start` constructs exactly one unchanged
  `InitialGatewayTurn` and owns it through `NativeAgentRun`.
- The shared event lane privately serializes only closed start, text, terminal,
  and failure events into the existing normalized frame and delegates all
  sequence/identity/content/limit validation to the existing turn.
- The concrete native lane delegates exact request bytes, frame acceptance,
  governance-bearing native events, trusted macOS approval resolution, and
  audited run-termination cleanup without genericizing those types.
- Concrete frames and shared events are mutually exclusive per run.
- Generic cancellation delegates stream cancellation or the existing audited
  pending-approval run-termination cleanup and returns no approval/audit data.

## Deterministic mock

`MockAgentRuntime` is private to `agent_runtime_contract.rs`. It uses only fixed
application-owned events and standard in-memory state. It has no clock, thread,
filesystem, network, process, provider, model, or Hermes prerequisite. It covers
deterministic success, unavailable/start/event failure, invalid state,
capability contradiction, cancellation, failure terminalization, late events,
and duplicate terminal behavior through the shared contract.

## Exact implementation files

- `src-tauri/src/agent/runtime.rs`
- `src-tauri/src/agent/native_runtime.rs`
- `src-tauri/src/agent/mod.rs`
- `src-tauri/tests/agent_runtime_contract.rs`

The existing `gateway_request.rs`, `gateway_protocol.rs`,
`function_call_validation.rs`, public gateway contract, frontend, Tauri,
manifests, lockfile, capabilities, and configuration remain unchanged.

## Focused evidence

- `agent_runtime_contract`: 20 passed, 0 failed.
- unchanged `gateway_request_contract`: 10 passed, 0 failed.
- unchanged gateway-protocol units: 18 passed, 0 failed.
- unchanged gateway-request units: 10 passed, 0 failed.
- focused strict Clippy: passed.
- Formatting: passed after the final focused edits.

- all-target Rust: 150 passed, 0 failed, 1 explicitly opt-in real-Hermes probe
  ignored.
- `npm run verify`: passed, including 124 frontend tests, hooks/repository tests,
  strict lint/typecheck/build, integration tests, and Tauri release build.
- `npm run security:scan`, `npm run docs:check`, `npm run repository:check`,
  `git diff --check`, and session-end inventory: passed.

Quality result: `PASS WITH ADVISORIES`. Root `AGENTS.md` still calls the runtime
names planned concepts; that repository-governance path was outside this
increment and should be updated in a separate bounded documentation change
before later Hermes implementation work.

## Security and architecture result

Independent review initially found that a descriptor/start/cancel-only trait was
not a usable D-079 runtime boundary. The corrected contract now accepts and
observes closed application-owned events through both native and mock runs.
Review also drove fixed capability storage, fail-closed mock errors, exact
pending-approval cancellation, bidirectional lane isolation, post-start
correlation, event bounds, and identifier/content redaction. Governance types
remain concrete to Native and no executor, permission, credential, IPC,
provider, network, process, dependency, or framework path was added.

## Rollback

Delete `runtime.rs`, `native_runtime.rs`, and the new contract test; restore
`agent/mod.rs`; and revert only this increment's current-state documentation.
Retain the unchanged `InitialGatewayTurn`, all prior native tests, accepted
D-079/D-080 decisions, and historical evidence. There is no database, external
process, credential, user-data, dependency, or runtime migration to reverse.
