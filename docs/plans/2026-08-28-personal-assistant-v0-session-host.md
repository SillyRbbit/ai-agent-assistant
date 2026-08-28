# V0-2 — volatile Personal Assistant Rust session host

Status: Blocked; V0-1 is not implemented or verified
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: V0-1

## Goal

Extend V0-1's verified minimal host with Rust-issued presentation identity,
chronological updates, deterministic deadline state, cancellation state,
terminal cleanup accounting, and late-event rejection without adding transport
or Tauri IPC. Continue to enter the runtime only through
`AgentRuntime::start`.

## Exact files

- `src-tauri/src/lib.rs` for the exact closed handle/snapshot/update result
  reexports
- `src-tauri/src/personal_assistant_v0.rs`
- `src-tauri/tests/personal_assistant_v0_contract.rs`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-08-28-personal-assistant-v0-session-host.md`
- `docs/increments/personal-assistant-v0-session-host.md` (new)
- `docs/reviews/2026-08-28-personal-assistant-v0-session-host-post-increment-review.md`
  (new)

Changing `runtime.rs`, `gateway_request.rs`, `native_runtime.rs`, another
agent/workflow module, a manifest/lockfile, Tauri configuration, capability,
CSP, or frontend file is a stop condition.

## Interfaces

- Preserve V0-1's no-argument `PersonalAssistantV0Host::start_synthetic()` and
  its single call to `AgentRuntime::start`. Do not add an inherent Native start
  method, runtime mode selector, or another identity issuer. Extend its return
  value only with a Rust-issued presentation handle and closed snapshot.
- V0-2 adds no production provider-frame or typed-event ingress because no
  trusted transport caller exists yet. Private `cfg(test)` drivers and module
  tests exercise every lifecycle transition without creating a compiled
  dead-code seam. V0-7 must add the crate-private raw-frame ingress in the same
  increment as its sole trusted-transport caller; it may accept no caller
  identity or configuration.
- The exact public surface is equivalent to:

  ```rust
  pub fn start_synthetic(
      &mut self,
  ) -> Result<PersonalAssistantV0Start, PersonalAssistantV0Error>;
  pub fn snapshot(
      &mut self,
      handle: &PersonalAssistantV0PresentationHandle,
  ) -> Result<PersonalAssistantV0Snapshot, PersonalAssistantV0Error>;
  pub fn updates(
      &mut self,
      handle: &PersonalAssistantV0PresentationHandle,
      after_sequence: Option<u64>,
  ) -> Result<PersonalAssistantV0UpdateBatch, PersonalAssistantV0Error>;
  pub fn cancel(
      &mut self,
      handle: &PersonalAssistantV0PresentationHandle,
  ) -> Result<PersonalAssistantV0Snapshot, PersonalAssistantV0Error>;
  ```

  `PersonalAssistantV0PresentationHandle` has no public constructor.
  `PersonalAssistantV0Start`, snapshot, update, and batch are closed non-Serde
  Rust types whose states, sequence ranges, prefix/final equality, null-cursor
  recovery, and at-most-16 contiguous update semantics are exactly the V0-11
  rules. Start contains only the Rust-issued handle and sequence-0 Starting
  snapshot. Sequences are `u64` but bounded to 0..128. The snapshot is a closed
  enum with Starting, Streaming, Cancelling, Completed, Failed, and Cancelled
  variants; every non-Starting variant contains its bounded accepted-text
  prefix, Completed additionally contains the equal final answer, and Failed
  contains only the closed failure code/support correlation. Updates are the
  closed Started, TextDelta, Completed, Failed, and Cancelled variants.
  `PersonalAssistantV0UpdateBatch` contains at most 16 updates,
  `through_sequence`, exact `has_more`, and the authoritative snapshot. No
  optional state field, map, raw JSON, or serialization derive is used; V0-11
  later owns the versioned DTO projection.

- `snapshot`, `updates`, and `cancel` are mutable because deterministic deadline
  sampling may terminalize the session. They accept only the opaque
  presentation handle and expose no private run, gateway, provider, or
  credential identity.
- Because no transport exists, explicit cancellation closes the local run and
  records `CleanupNotRequired`. V0-7, and only V0-7, may replace that marker
  with a narrow transport abort/cleanup result after real I/O ownership exists.

## State and invariants

The host permits one process-wide session and exact states `Idle`, `Starting`,
`Streaming`, `Cancelling`, `Completed`, `Failed`, and `Cancelled`.

- Start is possible only from Idle or after complete terminal cleanup.
- The host derives `pa-v0-present-{generation:016x}` and, only on failure,
  `pa-v0-support-{generation:016x}` from V0-1's private process generation.
  Both are visible ASCII below 128 bytes, never reused in-process, and map to
  exactly one active session. They are opaque correlations, not random secrets
  or authorization credentials; caller handles are compared to the sole active
  value before state access.
- Accepted deltas append one chronological update and a bounded final-answer
  buffer. The 128-entry journal never evicts an update needed by an active
  consumer; capacity failure terminalizes closed rather than dropping history.
- Completion publishes one final answer equal to accepted deltas.
- Failure publishes one closed code and no upstream content.
- Cancel first closes content ingress, invokes `RuntimeRun::cancel` exactly
  once, records transport cleanup as not required, and publishes exactly one
  `cancelled` update. Repeated cancel returns the existing terminal snapshot.
- Deadline sampling occurs immediately before frame acceptance, snapshot/poll,
  and terminalization. Expiry closes ingress and enters the same resumable
  cancellation path. Network-enforced timers remain V0-7 responsibility.
- Production samples `std::time::Instant` synchronously through a private
  standard-library monotonic-clock adapter; `cfg(test)` supplies a private
  manual clock through the same reducer. Wall clock, sleep, timer threads, and
  caller-supplied time are forbidden.
- A terminal or different-session frame, stale cleanup result, wrong handle,
  sequence gap, identity mismatch, or overflow is rejected without state
  mutation.
- Dropping a transport-free host clears text/content only after terminal local
  cleanup is proved and never claims durable cleanup or audit. If cleanup is
  rejected or ambiguous, Drop moves the nonterminal Native run into V0-1's
  process-owned private quarantine slot and does not release the process lease
  or mark cleanup complete. No public access or background retry exists; both
  run and lease remain owned until process exit. V0-7 must add drop-time
  transport containment before any live request exists.
- All state is volatile. No SQLite, file, Keychain, audit store, log body,
  analytics, or background worker is added.

## Closed errors

Errors are an enum with fixed display copy for `busy`, `invalid_request`,
`invalid_handle`, `protocol_violation`, `limit_exceeded`, `deadline_exceeded`,
and `internal`. Debug/display never includes text, private identity, frame
bytes, native/provider errors, or a credential.

## Threats and tests

Tests must prove:

- one active session and no caller identity/configuration input;
- unique bounded presentation handles and private identity non-projection;
- exact success, provider failure, explicit cancellation during Starting and
  Streaming, idempotent repeated cancellation, and restart after terminal local
  cleanup;
- connect, idle, provider, and total deadline boundary behavior with a
  deterministic monotonic clock;
- cancellation/deadline versus started, delta, completion, and failure races;
- post-terminal, wrong-session, stale-sequence, gap, duplicate-terminal, and
  late-cleanup rejection;
- 128-update journal, 16-update batch, delta, output, handle, and sequence
  overflow boundaries;
- exact final answer and chronological ordering;
- drop-time volatile-state clearing without a durable-cleanup claim; and
- canary prompt/output/error/identity absence from debug and closed errors.

The `cfg(test)` lifecycle driver is unavailable in non-test builds and accepts
only closed application-owned fixture events through the same reducer and
state-transition functions that V0-7's future trusted ingress must call; it may
not duplicate test-only lifecycle logic. Public integration tests cover
start/snapshot/update/cancel surfaces that exist in production; module tests
cover injected success/failure/deadline/late-event branches. A dead-code allow,
public raw ingress, or production fixture driver is forbidden.

Existing Native runtime and orchestration contracts must remain passing. The
new host must not instantiate `AgentOrchestrator`, `MemoryStore`, tool registry,
policy, approval, audit, or any specialist workflow.

## Verification

```bash
cargo test --manifest-path src-tauri/Cargo.toml personal_assistant_v0
cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract
cargo test --manifest-path src-tauri/Cargo.toml --test personal_assistant_v0_contract
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
npm run verify
npm audit --audit-level=low
npm run security:scan
npm run repository:check
npm run docs:check
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Manual review confirms no transport, network, provider, credential, Tauri,
frontend, persistence, tool, file, permission, or background capability.
Target-Mac UI and external checks remain `Not run`.

## Rollback

Revert only the exact host/runtime/test and closeout files. No external state
exists. V0-1 remains a transport-free contract and existing runtime paths remain
unchanged.

## Stop conditions

Stop if implementation needs async/network code, a timer thread, a generic
coordinator, `AgentRuntime` mode selection, `AgentOrchestrator`, memory, tools,
Tauri state, caller identities, persistence, new dependencies, or a source file
outside the list.

## Acceptance criteria

- [ ] One volatile Rust-owned session proves all closed lifecycle branches.
- [ ] Cleanup failure cannot release or replace the active owner.
- [ ] No untrusted surface can select a trusted identity or profile.
- [ ] Focused, complete, security, and completion gates pass.

## Readiness

**Blocked.** Reassess only after V0-1 is verified and published or otherwise
accepted as the exact source baseline.
