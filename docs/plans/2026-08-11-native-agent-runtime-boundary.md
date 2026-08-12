# Native agent runtime boundary

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-11

This is the next executable implementation ExecPlan under D-079. It is not
active, grants no editing authority by itself, and was not executed during the
architecture-decision documentation increment.

## Goal

Introduce the smallest application-owned runtime lifecycle/event boundary and
a native implementation that composes the verified Rust initial-turn path,
without integrating Hermes or changing shipping behavior.

## User-visible outcome

None. Cortexa continues to display and run the existing deterministic frontend
mock. No live model, provider, external process, tool execution, memory,
runtime selector, or new UI becomes available.

## Scope

- Define one closed Rust runtime descriptor, capability set, turn request, run
  identity, untrusted event set, terminal outcome, and error set.
- Define one narrow `AgentRuntime` lifecycle/event port only if the contract
  maps coherently to current native behavior.
- Add `NativeAgentRuntime` as a composition wrapper around the unchanged
  `InitialGatewayTurn` and existing typed boundaries.
- Add a deterministic test-only `MockAgentRuntime` inside the runtime contract
  suite. It uses no network, model, provider, Hermes installation, process, or
  external I/O and supports fixed success, failure, capability, invalid-state,
  and cancellation scenarios.
- Add deterministic native contract tests for parity, closed types, limits,
  cancellation, terminal state, and failure behavior.
- Document the implemented boundary accurately as source/test infrastructure,
  not a shipping live runtime.
- Complete the required architecture, security, code-health, technical-debt,
  readiness, documentation, and post-increment gates.

## Explicit non-goals

- No Hermes code, package, installer, process, configuration, protocol, fixture,
  data directory, or dependency.
- No OpenClaw code or evaluation.
- No Python, subprocess, WebSocket, HTTP, SSE, ACP, MCP, JSON-RPC, async-runtime,
  channel, or networking dependency.
- No provider/gateway transport, provider selection, model selection, model
  request, credential, Keychain access, identity, disclosure, or traffic.
- No Tauri command/event/capability/CSP/permission or frontend/UI change.
- No runtime-selection UI, settings, feature flag, or automatic fallback.
- No tool implementation, dispatch, execution, result ingestion, file access,
  browser, shell, clipboard, operating-system action, or permission expansion.
- No memory, session persistence, audit persistence, database migration,
  background work, subagent, scheduler, or messaging.
- No rewrite, rename, relocation, weakening, or deletion of existing native
  boundaries, tests, or deterministic mocks.
- No revival of D-030, D-032, D-033, or D-034 scaffolds.
- No branch, commit, push, PR, release, or publication without separate owner
  direction.

## Existing behavior and constraints

- `InitialGatewayTurn` is a concrete transport-free object used only by tests.
  It owns concrete in-memory registry, approval manager, and approval-audit
  instances and accepts normalized frame bytes.
- It is not wired to Tauri, React, networking, a provider, or an executor.
- The frontend mock and Rust initial-turn path are disconnected and must stay
  behaviorally unchanged in this phase.
- `ToolRegistry`, `PolicyEngine`, and `ApprovalManager` are existing narrow
  traits. They must not be absorbed into a giant runtime trait.
- There is no generic audit logger, product memory store, platform adapter, or
  current provider trait.
- Native cancellation is local, idempotent, and terminal. No process or
  transport currently exists to cancel.
- D-079 accepts the application-owned runtime architecture, and the owner has
  selected this exact plan as the sole next Ready implementation phase. This
  run began gate `native-agent-runtime-boundary` from a clean synchronized
  `main` baseline at `701c061`.

## Current-state evidence

- `src-tauri/src/agent/gateway_request.rs:38` defines `InitialGatewayTurn`.
- `src-tauri/src/agent/gateway_protocol.rs:253` defines
  `GatewayStreamValidator` and closed protocol limits/events.
- `src-tauri/src/agent/function_call_validation.rs` independently validates
  normalized calls against local schemas.
- `src-tauri/src/lib.rs` registers only `get_app_info`; no runtime crosses IPC.
- `src-tauri/capabilities/default.json` contains only `core:default`.
- `src/App.tsx` injects `browserMockRunDriver`; `mockLoop.ts` gives the mock zero
  network/file/search capability and `mockToolResult.ts` marks all results
  simulated and unexecuted.
- Public native boundaries are covered by
  `gateway_request_contract.rs`, `policy_input_binding.rs`,
  `approval_binding.rs`, and `approval_audit_binding.rs`.
- The accepted ADR is
  `docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md`, recorded durably as D-079.
- D-080 keeps the contained Hermes WebSocket spike Blocked until this plan is
  verified complete; no Hermes concern is needed to implement this phase.
- The complete call-site inventory found no production caller of
  `InitialGatewayTurn`: construction occurs only in its unit tests and
  `src-tauri/tests/gateway_request_contract.rs`. React continues to use the
  separate `browserMockRunDriver`, and Tauri exposes only `get_app_info`.
- The exact native symbols being adapted are `InitialGatewayTurn::{new,
request_bytes,status,accept_frame,cancel,
cancel_pending_approval_for_run_termination}` plus the macOS-only
  `resolve_approval_source_outcome`. `InitialGatewayEvent` contains both
  lifecycle/text results and governance-owned `PolicyEvaluated` and
  `ApprovalPresentationReady` results.
- `GatewayStreamValidator`, `GatewayStreamStatus`, `validate_function_call`,
  `ToolRegistry`, `PolicyEngine`, `ApprovalManager`, and typed approval audit
  remain unchanged and continue to own their current responsibilities.

## Files expected to change

Product and test scope:

- `src-tauri/src/agent/runtime.rs` (new)
- `src-tauri/src/agent/native_runtime.rs` (new)
- `src-tauri/src/agent/mod.rs`
- `src-tauri/tests/agent_runtime_contract.rs` (new)
- `src-tauri/tests/gateway_request_contract.rs` only if a minimal consumer move
  is required to prove parity; existing assertions and direct boundary coverage
  must remain

Architecture and closeout scope:

- `ARCHITECTURE.md`
- `PLANS.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `NEXT_STEPS.md`
- `CHANGELOG.md`
- this plan
- `docs/increments/native-agent-runtime-boundary.md` (new)
- `docs/reviews/YYYY-MM-DD-native-agent-runtime-boundary-post-increment-review.md`
  (new)

If implementation requires another product, test, manifest, lockfile,
capability, workflow, or configuration path, stop and request scope approval.

## Affected components

- Rust `agent` domain namespace;
- native initial-turn construction and test consumers;
- public Rust contract tests; and
- architecture/current-state documentation.

The Tauri application, frontend, provider, platform, credentials, storage,
tools, policy, approval, and audit implementations are behaviorally unaffected.

## Interfaces and invariants

### Final contract shape

Milestone 1 selected two small dependency-free traits because the current unit
of work is one bounded turn, not a runtime-owned session registry:

- `describe() -> RuntimeDescriptor`;
- `start(RuntimeTurnRequest) -> RuntimeRun` through an associated run type; and
- `RuntimeRun::{run_id,status,accept_event,cancel}` for one closed untrusted
  event sink and exact local run lifecycle.

A reviewer must be able to explain every method using current native behavior.
If a method exists only because Hermes might need it, remove it or stop the
increment.

The common event contract is an application-owned sink/acceptance boundary. It
contains closed start, text, proposal, completion, and failure values and never
exposes normalized JSON, provider/framework payloads, policy decisions,
approval presentations, or audit types. The native adapter translates supported
shared events privately into the unchanged normalized gateway frame and maps
only lifecycle/text/failure acceptance back to application-owned types.

The current native turn returns `PolicyEvaluated` and
`ApprovalPresentationReady` only after its concrete registry, schema, policy,
approval, and audit boundaries have run. The common native descriptor therefore
does **not** claim `UntrustedToolProposals`; such an event fails closed as a
capability mismatch and terminalizes the shared run. The concrete
`NativeAgentRun::{accept_frame,cancel_pending_approval_for_run_termination}`
lane preserves exact current tool/policy/approval behavior without putting
governance types on `AgentRuntime`. Mixing a concrete frame lane into the
shared-event lane is rejected before shared processing.

### Current behavior-to-method matrix

| Implemented symbol              | Current behavior mapped                                                                                                                  | Explicitly not claimed                                       |
| ------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| `AgentRuntime::describe`        | closed Native identity, in-process availability, local-boundary health, and bounded capabilities                                         | provider/model/network readiness                             |
| `AgentRuntime::start`           | constructs exactly one `InitialGatewayTurn` from the owned typed request                                                                 | session registry, connection, retry, transport, or execution |
| `RuntimeRun::{run_id,status}`   | identity retained by the wrapper and one-to-one `GatewayStreamStatus` mapping                                                            | persistence or cross-run coordination                        |
| `RuntimeRun::accept_event`      | privately serializes closed shared lifecycle/text/failure values and delegates acceptance to the unchanged validator                     | raw JSON/RPC, provider payloads, governance, or execution    |
| `RuntimeRun::cancel`            | delegates local stream cancellation and, when present, the existing audited run-termination cancellation of a run-owned pending approval | process cancellation or approval resolution authority        |
| `NativeAgentRun::request_bytes` | borrows the exact request serialized and owned by `InitialGatewayTurn`                                                                   | provider send or arbitrary transport bytes                   |
| `NativeAgentRun::accept_frame`  | delegates normalized untrusted-frame validation and preserves exact `InitialGatewayEvent`/error results                                  | generic runtime authority or raw frame bypass                |
| native approval methods         | delegate the existing exact pending-approval termination and macOS source-resolution paths                                               | approval ownership in `AgentRuntime`                         |

### Closed types

- `RuntimeId` and protocol/capability identifiers are closed enums or validated
  newtypes, never arbitrary adapter-supplied strings.
- `RuntimeTurnRequest` contains only current bounded run/request identity and
  selected content. It contains no provider, endpoint, credential, schema,
  risk, permission, approval, execution, or framework field.
- `RuntimeCapabilities` is a closed bounded set. It initially describes only
  verified native behavior; a declaration never grants permission.
- `UntrustedRuntimeEvent` contains only closed response-start, text-delta,
  untrusted-proposal, completion, and failure values. Tool arguments are a
  bounded redacted string only at this explicit untrusted serialization
  boundary; `serde_json::Value` is not a domain contract.
- `RuntimeEventAcceptance` returns only application-owned lifecycle, text,
  proposal, completion, or failure values. It cannot carry policy, approval,
  audit, execution, credential, provider configuration, or framework types.
- `RuntimeError` uses closed redacted variants and does not retain selected
  content, arguments, upstream bytes, or arbitrary failure strings.

### Ownership

- `AgentRuntime` owns only the currently provable local runtime descriptor and
  one-turn closed event/lifecycle foundation.
- `InitialGatewayTurn` and the gateway validator continue to own independent
  protocol, sequence, size, and content validation.
- `ToolRegistry` owns local tool identity/schema/risk/permission metadata.
- `PolicyEngine` owns deterministic policy.
- `ApprovalManager` owns exact approval state.
- typed audit owns its closed evidence families.
- no runtime owns or exposes execution, credentials, memory, platform access,
  Tauri IPC, or the WebView.

### Native parity

- `NativeAgentRuntime` composes `InitialGatewayTurn`; it does not subclass,
  duplicate, rename, move, or generalize it.
- Direct `InitialGatewayTurn` unit/contract coverage remains.
- The adapter adds no network, thread, timer, persistence, process, or I/O.
- Cancellation and terminal outcomes remain exact and idempotent.
- Debug and error output remains redacted.
- Native is recorded as the default/reference/explicit-fallback direction only;
  no selector or fallback code is added.
- `NativeAgentRun::cancel` owns only local run termination. The existing
  pending-approval termination path is invoked only as terminal run cleanup;
  its audit-bearing result remains concrete and cannot enter the generic
  cancellation result.

### Deterministic mock

- `MockAgentRuntime` is a private test fixture in
  `src-tauri/tests/agent_runtime_contract.rs`, not production scaffolding.
- It implements the exact application-owned contract with fixed scripts and no
  clock, thread, filesystem, network, subprocess, model, Hermes, or provider.
- It declares only explicit closed capabilities and fails when a script emits a
  capability contradiction.
- It supports deterministic unavailable, start failure, event failure,
  cancellation, late-event, and duplicate-terminal scenarios.
- It never replaces or alters the existing visible frontend mock; the two mocks
  exercise different boundaries.

### Separation from provider transport

The port must not contain a `complete(prompt) -> String` shape, provider/model
selection, HTTP request construction, arbitrary response body, or automatic
fallback. A future provider transport is a separate D-032/D-060 decision and
must still feed independent local validation.

## Implementation milestones

- [x] Milestone 0 - owner gate and clean baseline
  - confirm accepted D-079 and D-080 remain current;
  - confirm the documentation closeout classified this exact plan Ready;
  - require no overlapping uncommitted work;
  - begin exactly one `native-agent-runtime-boundary` gate.
- [x] Milestone 1 - contract proof on paper and in negative tests
  - write the exact current-behavior-to-method matrix;
  - choose closed types without new dependencies;
  - add negative contract tests first;
  - stop if the contract needs framework, provider, executor, memory, storage,
    or generic JSON/RPC concepts.
- [x] Milestone 2 - native wrapper
  - add `NativeAgentRuntime` as a thin composition root;
  - forward only existing verified behavior;
  - retain all `InitialGatewayTurn` coverage and add native parity cases.
- [x] Milestone 3 - adversarial and regression coverage
  - verify invalid IDs, limits, unknown variants, out-of-order/late events,
    cancellation, double terminal state, redaction, and capability mismatch;
  - prove no Tauri/frontend/provider/execution behavior changed.
- [x] Milestone 4 - architecture and completion evidence
  - label the boundary implemented but unwired;
  - run required verification/reviews;
  - synchronize current project memory;
  - finalize the post-increment marker.

## Security and privacy considerations

- Runtime output remains untrusted at every stage.
- No generic bytes/JSON/RPC method may skip gateway and local schema validation.
- Runtime-reported risk, permission, policy, approval, actor, authentication,
  audit, or execution facts are ignored or unrepresentable.
- Selected content and arguments remain absent from Debug/errors/audit.
- No secret, environment, path, handle, credential, provider endpoint, or
  personal data is added to a runtime descriptor or capability set.
- Unknown capability, event, identity, version, ordering, or terminal state
  fails closed.
- The phase adds no external process; process containment is therefore not
  claimed or tested as implemented capability.
- A security review must explicitly confirm the new abstraction cannot become
  a direct model-to-device or WebView-to-device path.

## Test plan

Focused Rust tests must cover:

- exact native descriptor and bounded capability declaration;
- deterministic `MockAgentRuntime` success, failure, capability, invalid-state,
  cancellation, late-event, and duplicate-terminal scripts with no I/O;
- valid construction from current run/request/content inputs;
- invalid/empty/oversized identities and content;
- current text-only completion parity;
- valid and invalid local tool-proposal parity;
- unknown tool and version rejection;
- policy and approval ownership unchanged;
- local cancellation idempotence and late-event rejection;
- run/request identity mismatch;
- duplicate/out-of-order/unknown event rejection;
- closed failure mapping and Debug/Display redaction;
- capability contradiction failure;
- no conversion from runtime event/capability/error to approval, execution, or
  audit authority; and
- unchanged existing gateway, policy, approval, audit, frontend, storage, and
  application suites.

No network, provider, Hermes, Python, subprocess, UI, or manual native check is
required because this phase adds none. If a reviewer finds a user-visible or
target-Mac behavior change, stop and expand the plan only with owner approval.

## Verification commands

Run focused checks during implementation, then run the required complete gate
once after the final product/doc edit:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo check --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run verify
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Before claiming completion, also prove the diff contains no Hermes/OpenClaw,
Python, dependency, manifest, lockfile, Tauri capability/configuration,
frontend, workflow, installer, or runtime-data path.

## Risks

| Risk                                      | Severity | Mitigation                                                                                       |
| ----------------------------------------- | -------- | ------------------------------------------------------------------------------------------------ |
| Premature abstraction                     | High     | Require one-to-one current behavior mapping; stop if methods exist only for a future framework   |
| Provider/runtime conflation               | High     | Exclude provider/model/transport fields and review explicitly against D-032                      |
| Duplicated governance                     | High     | Keep tool, policy, approval, audit, execution, memory, secrets, and platform interfaces separate |
| Misleading current capability             | Medium   | Document the boundary as implemented but unwired; do not call it a live agent runtime            |
| Public API churn                          | Medium   | Keep modules crate-local where possible; expose only what public contract tests require          |
| Unnecessary dependency/concurrency choice | Medium   | Standard library and existing dependencies only; defer async/transport implementation            |
| Native regression                         | High     | Preserve direct tests and add wrapper parity; no consumer/UI wiring                              |
| Future giant capability enum              | Medium   | Closed minimal capabilities with separate additive decisions                                     |

Any High finding that cannot be corrected within the declared paths blocks
completion and requires owner direction.

## Rollback or failure strategy

- Remove `runtime.rs`, `native_runtime.rs`, and the new runtime contract test.
- Restore `agent/mod.rs` and any minimally adapted test construction to the
  pre-increment form.
- Retain `InitialGatewayTurn` and all existing source/tests unchanged.
- Revert only current project-memory entries created for this increment; do not
  rewrite historical evidence.
- No database, external process, credential, user data, dependency, or runtime
  migration exists, so rollback is source-only.
- If the abstraction proves speculative, record that evidence and keep the ADR
  rejected/deferred rather than preserving misleading unused code.

## Decisions made

- D-079 accepts the small application-owned runtime architecture and this
  native-first phase.
- `MockAgentRuntime` is test-only contract infrastructure in
  `agent_runtime_contract.rs`; no production mock abstraction is added.
- D-080 leaves all Hermes execution in a later blocked spike.

The future implementation run must still record:

- accepted ADR/decision ID;
- final boundary name and visibility;
- exact closed contract and current-behavior mapping;
- whether one or two traits are required for runtime and run lifecycle;
- why the contract is not a provider API; and
- why each capability/event variant is necessary now.

## Discoveries

- The current native composition root is test-only and concrete; a wrapper can
  prove portability but cannot honestly claim a live native runtime.
- The frontend mock is a separate application service and should not be routed
  through the Rust boundary in this phase.
- The shared native lane can safely accept lifecycle, text, completion, and
  failure events through the unchanged validator. It cannot honestly expose a
  pre-governance tool proposal because the current concrete turn retains that
  call and later returns policy or approval results. Native therefore omits the
  shared proposal capability while preserving the exact concrete tool lane.
- Exact run cancellation must also close a pending approval created by the
  concrete turn. The wrapper delegates that cleanup to the existing typed,
  audited run-termination method and exposes no approval or audit data through
  `AgentRuntime`.
- A future Hermes adapter needs process supervision and protocol translation,
  but adding either now would expand this native-only plan.

## Progress

- 2026-08-11: Proposed from the Hermes integration assessment. No milestone has
  begun and no implementation file has changed.
- 2026-08-11: The owner accepted D-079 and selected this exact plan. A fresh
  documentation readiness review classified it Ready; implementation has not
  begun and no source, test, dependency, or behavior changed.
- 2026-08-11: Reconfirmed a clean synchronized `main` baseline at `701c061`,
  passed the 10-test public gateway contract and 10-test gateway-request unit
  baseline, and began gate `native-agent-runtime-boundary`.
- 2026-08-11: Completed the exact symbol/call-site inventory. Selected an
  associated-run `AgentRuntime`/`RuntimeRun` foundation, a closed shared event
  sink, and a separate concrete native frame/governance lane.
- 2026-08-11: Added the native wrapper and private deterministic mock contract
  suite. Focused evidence currently passes 20 tests plus the unchanged 10-test
  public gateway contract, 18 gateway-protocol units, and 10 gateway-request
  units; complete-gate validation remains pending.
- 2026-08-11: Independent review found and the implementation corrected a
  missing shared event path, noncanonical capability storage, nonterminal mock
  failures, generic cancellation that could leave a run-owned approval pending,
  one-way lane isolation, event construction that required retaining the consumed
  request, unbounded output text, thin negative coverage, and identifier Debug
  exposure. Focused tests and strict Clippy pass after the corrections.
- 2026-08-11: The all-target Rust suite passed 150 tests with one explicitly
  opt-in real-Hermes version probe ignored. `npm run verify`, frontend tests,
  hooks/repository tests, the Tauri release build, docs/repository checks,
  security scan, session-end inventory, and post-increment marker all pass.

## Acceptance criteria

- [x] Owner accepts or amends the multi-runtime ADR through a durable decision.
- [x] Fresh readiness review classifies this exact plan Ready or Ready with
      advisories.
- [x] The contract contains only descriptor, start, closed untrusted events,
      and cancellation responsibilities necessary for current native behavior.
- [x] `NativeAgentRuntime` composes the unchanged `InitialGatewayTurn`.
- [x] All direct existing tests and new native parity/adversarial tests pass in
      the complete applicable suite.
- [x] No behavior, UI, Tauri IPC/capability, provider, network, dependency,
      process, execution, credential, memory, persistence, or external action
      is added.
- [x] No arbitrary JSON/string/RPC/provider escape hatch or generic authority
      interface is introduced.
- [x] Documentation says the boundary is unwired and not a live agent runtime.
- [x] Architecture, security, code-health, technical-debt, readiness,
      session-end, and post-increment reviews pass.
- [x] Native remains default/reference/explicit-fallback direction, with no
      automatic fallback implementation.

## Final results

Implementation is verified complete with advisories. The
new application-owned contract, default native composition wrapper, and private
deterministic mock exist only in Rust source/test infrastructure. Focused
runtime and unchanged gateway regressions pass; the all-target Rust suite passed
150 tests with one explicitly opt-in real-Hermes version probe ignored; and
`npm run verify`, the Tauri release build, docs/repository checks, security scan,
and session-end/post-increment gates pass. No Hermes, provider, network, process,
dependency, Tauri/React wiring, runtime selector, automatic fallback, or
user-visible behavior was added.

Quality result: `PASS WITH ADVISORIES`. The implementation has no remaining
source/test finding. Root `AGENTS.md` still describes the runtime names as
planned concepts; updating that repository-governance wording was outside this
approved path inventory and should be a separate bounded documentation change
before later Hermes implementation work.

## Documentation updates

When separately approved and completed:

- [x] `DECISIONS.md` records the accepted ADR before implementation.
- [x] `ARCHITECTURE.md` distinguishes implemented-unwired boundary from mocked,
      planned, and shipping behavior.
- [x] `PLANS.md`, `HANDOFF.md`, `PROJECT_STATUS.md`, `NEXT_STEPS.md`, and
      `CHANGELOG.md` record actual evidence only.
- [x] `docs/increments/native-agent-runtime-boundary.md` records scope/results.
- [x] A valid post-increment review records the complete diff and checks.
