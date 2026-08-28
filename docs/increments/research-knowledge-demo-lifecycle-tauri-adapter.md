# Research/Knowledge demo lifecycle Tauri adapter

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-28

## Approved scope

The owner approved the exact lifecycle Tauri adapter source increment described
by the corresponding ExecPlan: one application-owned volatile host, four
no-input commands, one bounded notification event, an initially unconnected
runtime-narrowed client, and atomic F-12 protection. Command Center connection,
caller-selected identity, external effects, persistence, background work,
capability/CSP changes, dependencies, and agent internals remain out of scope.

## Blocking evidence

The required focused Rust compile failed before any adapter test could run.
Tauri managed state requires `Send + Sync + 'static`, but
`ResearchKnowledgeDemoHost` transitively owns `Box<dyn ApprovalClock>` through
the governance approval manager, and the private `ApprovalClock` trait is not
`Send`. Therefore `Mutex<ResearchKnowledgeDemoHost>` cannot satisfy Tauri's
managed-state bound.

The approved file list cannot correct that private prerequisite. Unsafe Send
assertions, thread-local ownership, a worker, a queue, or a second host would
either weaken the boundary or expand the approved architecture and were not
attempted.

## Rollback and verification

- The partial adapter module, contract test, and `lib.rs` registration were
  removed immediately after the compile failure.
- `git diff -- src-tauri/src/lib.rs` is empty; no product-source change remains.
- `cargo test --manifest-path src-tauri/Cargo.toml
research_knowledge_demo_lifecycle`: Passed, 9 passed, 0 failed, 0 ignored.
- Adapter-focused and adapter-contract tests: Failed/Not run at compilation;
  the managed-state trait bound prevented test execution.
- Client, F-12, full verification, and target-Mac checks: Not run because the
  source increment stopped at the unapproved prerequisite.

The required post-increment workflow subsequently ran. Baseline frontend,
repository, acceptance, full verification, security, diff, release no-bundle
build, and target-Mac process startup pass. The adapter contract target still
fails because it does not exist, the focused adapter name filter executes zero
tests, target-Mac visual permission inspection is pending, and native lifecycle
observation is Not run. The consolidated result is
[`FAIL`](../reviews/2026-08-28-research-knowledge-demo-lifecycle-tauri-adapter-blocked-post-increment-review.md);
the gate remains active and unfinalized.

## Smallest next decision

Before the adapter can resume, the owner must separately approve amending the
plan and source scope to include `src-tauri/src/approvals/manager.rs` for only:

- `ApprovalClock: Send`;
- a deterministic Send-safe replacement for the test-only `Rc<Cell<Instant>>`
  clock, such as `Arc<Mutex<Instant>>`;
- a compile-time Send assertion and existing approval/governance tests proving
  no semantic or authority change.

The owner approved this exact prerequisite on 2026-08-28. Implementation may
resume within the amended ExecPlan; the source gate remains active. No broader
approval, policy, concurrency, UI, capability, CSP, dependency, or effect change
is authorized.

## Implemented result

- The private `ApprovalClock` now requires `Send`; the test clock uses
  `Arc<Mutex<Instant>>`; compile-time assertions cover the approval manager and
  lifecycle host.
- One Tauri-managed `Mutex<ResearchKnowledgeDemoHost>` owns the sole process
  lifecycle. Snapshot, start, advance, and cancel commands accept only injected
  Tauri state/app handles and no caller-selected values.
- Successful mutations emit one fixed snapshot notification after releasing the
  mutex. Emission failure returns the closed unavailable error while preserving
  the committed host for explicit snapshot recovery.
- The initially unconnected client invokes only the four literal commands,
  listens to the one literal event, narrows `unknown` to an exact frozen v1 DTO,
  rejects concurrent/stale/contradictory data, and requires explicit recovery
  for gaps or newer epochs.
- F-12 guards the exact imports, invocation count/names, command signatures,
  handler registrations, event/listener/emitter allowlist, prohibited boundary
  tokens, unchanged capabilities, and unchanged production/development CSP.

## Focused verification

- Approval-manager tests: Passed, 7 passed, 0 failed, 0 ignored.
- Existing lifecycle-core tests: Passed, 9 passed, 0 failed, 0 ignored.
- Adapter tests: Passed, 5 passed, 0 failed, 0 ignored.
- Public adapter contract: Passed, 1 passed, 0 failed, 0 ignored.
- Client tests: Passed, 30 passed, 0 failed.
- Strict typecheck and frontend/Rust lint: Passed.
- Repository/F-12 tests: Passed, 61 passed, 0 failed.
- One Cargo incremental-cache `dep-graph.part.bin` failure occurred before
  source diagnostics and passed on exact unchanged retry; no cache deletion or
  source workaround was used.
- Complete `npm run verify`: Passed with 28 hook tests, 61 repository tests, 277
  frontend tests, 267 Rust library tests, 244 Rust integration tests, one
  intentional ignored Hermes probe, the production frontend build, and the
  Tauri release no-bundle build.
- `npm run security:scan` and `git diff --check`: Passed.
- Target-Mac `npm run tauri -- dev`: Passed; Vite and Rust started, the native
  Cortexa window was visible, and no permission prompt appeared.
- Direct observation of the deliberately unconnected lifecycle commands/event:
  Not run; approved UI tooling has no connected surface for them.

## Completion result

`PASS WITH ADVISORIES`. The adapter is volatile, visibly simulated at its DTO
boundary, and unconnected to React. The Command Center, Conversations mock,
read-only projection, lifecycle adapter, and Rust acceptance workflows remain
separate deterministic proofs. The adapter adds no provider, model, network,
credential, tool execution, approval dispatch, persistence, filesystem access,
background autonomy, capability/CSP change, dependency, or device effect.

The next connected presentation increment is `Blocked` pending a separate
owner-approved plan. That plan must reconcile the current fixture-only failure
proof before claiming an interactive Rust-backed failure scenario.
