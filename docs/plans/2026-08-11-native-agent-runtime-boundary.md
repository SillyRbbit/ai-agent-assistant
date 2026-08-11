# Native agent runtime boundary

Status: Proposed; Blocked pending owner acceptance of the multi-runtime ADR and
a fresh readiness review
Owner: Project owner
Last updated: 2026-08-11

This is the proposed next-phase ExecPlan. It is not active, grants no authority,
and was not executed during the Hermes architecture assessment.

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
- D-078 permits only a conceptual runtime direction until a separate plan and
  approval authorize implementation.

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
- The proposed ADR is
  `docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md`; it must be accepted through
  the repository decision process before this plan can become Ready.

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
- `DECISIONS.md` only to record the owner-accepted ADR before implementation
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

### Proposed contract shape

The final Rust syntax is chosen during Milestone 1 and must remain dependency-
free. Conceptually the port supports only:

- `describe() -> RuntimeDescriptor`;
- `start(RuntimeTurnRequest) -> RuntimeRun`;
- delivery/acceptance of closed `UntrustedRuntimeEvent` values for one run; and
- `cancel(RuntimeRunId) -> RuntimeCancellationOutcome`.

A reviewer must be able to explain every method using current native behavior.
If a method exists only because Hermes might need it, remove it or stop the
increment.

### Closed types

- `RuntimeId` and protocol/capability identifiers are closed enums or validated
  newtypes, never arbitrary adapter-supplied strings.
- `RuntimeTurnRequest` contains only current bounded run/request identity and
  selected content. It contains no provider, endpoint, credential, schema,
  risk, permission, approval, execution, or framework field.
- `RuntimeCapabilities` is a closed bounded set. It initially describes only
  verified native behavior; a declaration never grants permission.
- `UntrustedRuntimeEvent` contains only the minimum closed variants needed for
  current response start/text, untrusted proposal, completion, failure, and
  cancellation behavior. Unknown variants/fields/versions fail closed.
- `RuntimeError` uses closed redacted variants and does not retain selected
  content, arguments, upstream bytes, or arbitrary failure strings.

### Ownership

- `AgentRuntime` owns runtime lifecycle/event production only.
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

### Separation from provider transport

The port must not contain a `complete(prompt) -> String` shape, provider/model
selection, HTTP request construction, arbitrary response body, or automatic
fallback. A future provider transport is a separate D-032/D-060 decision and
must still feed independent local validation.

## Implementation milestones

- [ ] Milestone 0 - owner gate and clean baseline
  - accept or amend the Proposed ADR through `DECISIONS.md`;
  - run readiness review;
  - require no overlapping uncommitted work;
  - begin exactly one `native-agent-runtime-boundary` gate.
- [ ] Milestone 1 - contract proof on paper and in compile-failing tests
  - write the exact current-behavior-to-method matrix;
  - choose closed types without new dependencies;
  - add negative/compile-oriented contract tests first;
  - stop if the contract needs framework, provider, executor, memory, storage,
    or generic JSON/RPC concepts.
- [ ] Milestone 2 - native wrapper
  - add `NativeAgentRuntime` as a thin composition root;
  - forward only existing verified behavior;
  - retain all `InitialGatewayTurn` coverage and add native parity cases.
- [ ] Milestone 3 - adversarial and regression coverage
  - verify invalid IDs, limits, unknown variants, out-of-order/late events,
    cancellation, double terminal state, redaction, and capability mismatch;
  - prove no Tauri/frontend/provider/execution behavior changed.
- [ ] Milestone 4 - architecture and completion evidence
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

None. This plan remains Proposed. The future owner-approved run must record:

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
- A future Hermes adapter needs process supervision and protocol translation,
  but adding either now would expand this native-only plan.

## Progress

- 2026-08-11: Proposed from the Hermes integration assessment. No milestone has
  begun and no implementation file has changed.

## Acceptance criteria

- [ ] Owner accepts or amends the multi-runtime ADR through a durable decision.
- [ ] Fresh readiness review classifies this exact plan Ready or Ready with
      advisories.
- [ ] The contract contains only descriptor, start, closed untrusted events,
      and cancellation responsibilities necessary for current native behavior.
- [ ] `NativeAgentRuntime` composes the unchanged `InitialGatewayTurn`.
- [ ] All direct existing tests and new native parity/adversarial tests pass.
- [ ] No behavior, UI, Tauri IPC/capability, provider, network, dependency,
      process, execution, credential, memory, persistence, or external action
      is added.
- [ ] No arbitrary JSON/string/RPC/provider escape hatch or generic authority
      interface is introduced.
- [ ] Documentation says the boundary is unwired and not a live agent runtime.
- [ ] Architecture, security, code-health, technical-debt, readiness,
      session-end, and post-increment reviews pass.
- [ ] Native remains default/reference/explicit-fallback direction, with no
      automatic fallback implementation.

## Final results

Not started. This plan was created as an output of a documentation-only
assessment and must not be marked Active until the owner accepts the ADR,
selects this task, reconciles the working tree, and begins a new gate.

## Documentation updates

When separately approved and completed:

- [ ] `DECISIONS.md` records the accepted/amended ADR before implementation.
- [ ] `ARCHITECTURE.md` distinguishes implemented-unwired boundary from mocked,
      planned, and shipping behavior.
- [ ] `PLANS.md`, `HANDOFF.md`, `PROJECT_STATUS.md`, `NEXT_STEPS.md`, and
      `CHANGELOG.md` record actual evidence only.
- [ ] `docs/increments/native-agent-runtime-boundary.md` records scope/results.
- [ ] A valid post-increment review records the complete diff and checks.
