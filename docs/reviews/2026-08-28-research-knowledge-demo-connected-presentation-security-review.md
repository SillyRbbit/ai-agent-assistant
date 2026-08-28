# Research/Knowledge connected presentation security review

Date: 2026-08-28
Change: `research-knowledge-demo-connected-presentation`
Reviewer: Codex security-review workflow with independent source re-review

## Result

`PASS WITH ADVISORIES`

No Critical, High, Medium, or Low security finding remains. The source slice
preserves its fixed synthetic boundary and adds no external authority. Residual
advisories cover the unchanged `core:default` event-emission surface, deliberate
F-12 digest maintenance, and native raw-debug checks unavailable to approved UI
tooling.

## Scope and threat model

The untrusted actors are the WebView, event payloads, and all values returned
over Tauri IPC. Trusted authority remains in deterministic Rust: one
application-owned `ResearchKnowledgeDemoHost`, `AgentOrchestrator`, the sole
`NativeAgentRuntime`, exact returned-identity validation, and rejected-run
cleanup ownership. The WebView supplies no agent, task, root, run, request,
profile, runtime, workflow, objective, source, fixture, script, stage, outcome,
or event frame.

The affected entry points are the existing no-input snapshot, start, advance,
and cancel commands; the fixed `research-knowledge-demo-lifecycle-v1`
notification; and one prop-free lifecycle panel mounted only in the selected
Research and Knowledge scenario. Assets are Rust-owned identities and volatile
lifecycle ownership. Outputs are a closed v1 state vocabulary, bounded epoch
and revision, at most eight content-free journal entries, exact disclosure and
proof-boundary copy, or one closed unavailable error.

Current behavior is a visibly simulated process-local proof. The Command Center
fixture projection, read-only Rust projection, lifecycle panel, Conversations
mock, and Rust acceptance workflows remain separate. Provider/model transport,
real tools, approvals, persistence, durable audit, filesystem/device access,
background autonomy, credentials, and live content remain prohibited and
absent.

## Data flow and privileges

An explicit user action invokes a fixed command with no arguments. Rust owns
the host, private fixture scripts, completed-run schedule, identities, and
runtime. Returned snapshots are serialized across IPC and parsed from `unknown`
with exact keys, literal identity/provenance/disclosure values, bounded integer
limits, sequential revisions, and a closed state-specific journal grammar.
Requested-operation validation binds each start, advance, or cancel response to
the previously accepted snapshot. Only an accepted command response commits
presentation state.

Notifications carry the same untrusted DTO. Malformed, older, and same-revision
notifications are inert. Any parser-valid newer notification sets
`recoveryRequired` without replacing the accepted snapshot or calling the React
listener; the next explicit operation fails closed unless an explicit snapshot
response reconciles state. This prevents a WebView-emitted grammar-valid success,
failure, or cancellation event from rendering an outcome or creating native
authority.

All state remains process-local. No file, SQLite row, browser storage, log,
network request, credential, permission, approval, tool call, operating-system
effect, timer, worker, queue, or dependency was added. F-07 production and
development CSPs, Tauri capabilities, and command/event registration remain
unchanged. Errors never expose raw Rust, transport, identity, content, path, or
cleanup detail.

## Checklist evidence

- Native agent governance: applicable and passed; F-01 returned-runtime
  identity validation and F-02 cleanup quarantine remain on every start path.
- Sealed Research and Knowledge workflow: applicable and passed; only the fixed
  D-086 synthetic fixture is reachable.
- Deterministic Command Center and read-only projection: applicable and passed;
  fixture controls remain IPC-free and the lifecycle panel is a separate proof.
- Volatile Research/Knowledge lifecycle core: applicable and passed; no-input
  construction, closed DTOs, cancellation, schedule ownership, cleanup, and
  replacement blocking remain bounded.
- Tauri IPC, CSP, capabilities, and plugins: applicable and passed with the
  unchanged-event-emission advisory below; four no-input commands, one event,
  no capability/plugin/CSP change.
- Credentials, gateway/provider, tools/policy/approval, SQLite/local data,
  filesystem/OS, audit/logging: inspected and not exercised; no such input,
  output, permission, state, or dependency was added.
- Dependency/supply chain: applicable only as a no-change check; manifests and
  lockfiles are unchanged, and npm audit reports zero vulnerabilities.
- GitHub automation and release security: no workflow/release change; remote
  runs are not available for the uncommitted state, and nothing was published.

## Findings

### Advisory — unchanged WebView event-emission permission can force recovery

- Location: `src-tauri/capabilities/default.json` and
  `src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.ts`
- Impact: a compromised WebView that can emit the fixed event may set the
  lifecycle client to recovery-required and make the panel fail closed on its
  next operation. It cannot mutate the Rust host, commit presentation state,
  select an outcome, or authorize an action.
- Evidence: listener tests prove unsolicited valid successor and terminal
  events leave `current` unchanged and call no presentation callback. Exact
  response validation remains required for every operation.
- Smallest mitigation: consider a separately approved capability-narrowing
  increment only if Tauri permits removal of WebView event emission without
  disrupting required core behavior.
- Blocking: no; this is fail-closed denial of availability, not authority gain.

### Advisory — F-12 exact source digests require deliberate maintenance

- Location: `scripts/repository_health.py`
- Impact: a future legitimate lifecycle client or panel edit must update the
  reviewed SHA-256 baseline atomically with adversarial tests. This is intended
  review friction; an unreviewed equivalent alias cannot silently pass.
- Smallest mitigation: keep the current review procedure and update the digest
  only inside a separately approved boundary-affecting increment.
- Blocking: no.

## Adversarial and regression tests

- Exact DTO keys, literals, integer bounds, journal bounds, revision order, and
  state grammar: Passed.
- Caller-forged identity/content/additional keys and contradictory requested
  operation responses: Passed closed.
- Older, same-revision, gapped, newer-epoch, malformed, impossible-journal, and
  valid-but-unsolicited successor/terminal notifications: Passed without
  presentation mutation; recovery is required only for parser-valid newer data.
- Concurrent requests, late results after disposal, operation failure, listener
  failure, cleanup-pending, cancellation at every active stage, and schedule
  continuity across cancellation: Passed.
- Alternate lifecycle consumers, aliased/computed/raw Tauri transport, command
  arguments, outcome selectors, extra listeners/emitters, browser network/
  storage/timer/worker surfaces, capability/CSP drift, and guarded-source drift:
  Passed rejection in F-12 repository tests.
- Traversal, symlink, SQLite, credential-redaction, expiry, and operating-system
  permission cases: Not applicable because this increment accepts no paths,
  credentials, stored data, expiry value, or privileged operation.

## Verification

- `npx vitest run src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.test.ts src/features/command-center/ResearchKnowledgeLifecyclePanel.test.tsx src/features/command-center/CommandCenterPage.test.tsx`: Passed, 81/81.
- `cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle`: Passed, 16/16 lifecycle tests, zero ignored.
- `cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle_tauri`: Passed, 6/6 adapter tests, zero ignored.
- `cargo test --manifest-path src-tauri/Cargo.toml --test research_knowledge_demo_lifecycle_tauri_contract`: Passed, 1/1.
- `npm run test:repository`: Passed, 76/76.
- `python3 scripts/repository_health.py ui-native-boundary`: Passed.
- `npm run verify`: Passed; complete lint, test, frontend build, and Tauri
  release no-bundle build completed without warnings.
- `npm run security:scan`: Passed.
- `npm audit --audit-level=low`: Passed with zero vulnerabilities after the
  sandboxed DNS attempt failed and the same command was retried with approved
  network access.
- `git diff --check`: Passed.
- Target-Mac `npm run tauri -- dev` startup and no-permission-prompt inspection:
  Passed.
- Source-current raw-debug lifecycle interaction, alternate native theme and
  reduced motion, page zoom, and native resize: Not run; approved tooling could
  not bind to the raw debug executable.

## Residual risk and follow-up

The project owner accepts only a local, synthetic, volatile demonstration. The
unchanged event-emission permission and exact-digest maintenance are advisories
for a future explicitly approved boundary increment, not authority for work now.
Native raw-debug presentation evidence should be repeated when approved tooling
can bind to that executable. No next source increment is selected or Ready.
