# Hermes integration assessment

Status: Architecture assessment only; no implementation authorization
Assessment date: 2026-08-11
Repository baseline: `main` at `40f04b6`, with pre-existing uncommitted
documentation preserved
Upstream basis: Nous Research `hermes-agent` GitHub release `v2026.8.3`, source
version `0.20.0`, release commit
`3c27eb6234bf91b8ceee9e9071591b31e9b148cb`; PyPI distribution `0.19.0` from
tag `v2026.7.20`, commit
`3ef6bbd201263d354fd83ec55b3c306ded2eb72a`

This document evaluates a hypothesis. It does not claim that `AgentRuntime`,
`NativeAgentRuntime`, `HermesAgentRuntime`, a Hermes installation, a provider,
or a live native agent exists. At the time of this assessment, D-078 was the
accepted direction and the ADR created with this assessment remained Proposed.

> **Subsequent accepted decisions (2026-08-11):** D-079 accepts the small
> application-owned multi-runtime architecture, and D-080 conditionally selects
> managed local `hermes serve` plus TUI-gateway JSON-RPC/WebSocket for a later
> contained spike after native-runtime completion. Those decisions supersede
> this assessment's Proposed/no-transport-selected state and raw-stdio
> preference. The body remains historical assessment evidence; it does not
> authorize Hermes execution or an adapter.

> **Post-assessment transport result (2026-08-11):** The isolated
> [`HERMES_TRANSPORT_SPIKE.md`](../spikes/HERMES_TRANSPORT_SPIKE.md) disproved
> raw TUI-gateway stdio as a supported production integration mechanism for
> Hermes Agent `0.20.0` / tag `v2026.8.3`. The tagged release exposes no public
> raw-gateway launcher, no initial version or capability negotiation, and no
> gateway-shutdown RPC. The conditional recommendation below is retained as
> historical assessment evidence, but its raw-stdio preference is superseded.
> At that spike's closeout, no adapter could be implemented until the Proposed
> ADR was separately revised to evaluate a supported public transport, and this
> notice selected no replacement. D-079/D-080 above now record the additive
> decision outcome while keeping adapter implementation separately Blocked.

## 1. Executive summary

**Recommendation: CONDITIONAL GO.** Continue with a small application-owned
runtime boundary and a native-only implementation first. Do not integrate or
run Hermes until separate evidence proves process containment, a pinned
distribution, a narrow protocol projection, and the continued exclusivity of
Cortexa's Rust governance.

The recommended target is:

```text
Application and application services
              |
              v
Trusted Cortexa governance
  validation -> policy -> exact approval -> restricted execution -> audit
              |
              v
AgentRuntime (untrusted event and lifecycle port only)
  |-- NativeAgentRuntime (default, reference, explicit fallback, test path)
  `-- HermesAgentRuntime (optional, experimental, separately approved)
```

The recommended Hermes mechanism, if later approved, is a **version-pinned,
whole-process-contained managed subprocess** behind an application-owned
adapter. Prefer the officially documented TUI-gateway JSON-RPC transport over
stdio because it avoids a listening socket. A future spike must first verify a
supported, noninteractive launcher and a versioned schema. If only the
documented `hermes serve` WebSocket surface is viable, it must remain
loopback-only, use a per-launch secret held outside the WebView, and pass the
same narrow method/event allowlist. Neither path is authorized now.

- Qualitative complexity: **High**. The protocol translation is not the hard
  part; confinement, packaging, credentials, lifecycle, compatibility, and
  duplicated-governance prevention are.
- Most important risk: Hermes normally combines model routing, tools, memory,
  skills, plugins, subprocesses, approvals, and persistence inside one Python
  trust envelope. Its own security policy says OS isolation—not its in-process
  approval, redaction, scanner, or allowlist—is the load-bearing boundary.
- Most important preservation requirement: keep the current native Rust chain,
  its closed types, deterministic mocks, and tests intact. Hermes output must
  enter before independent Cortexa validation and may never acquire policy,
  approval, execution, credential, audit, or device authority.

This is not a recommendation to use Hermes's broad feature set. It is a
recommendation to preserve the option to consume a deliberately tiny,
untrusted subset after the native boundary exists and all blockers below are
closed.

## 2. Current architecture

### Domain boundaries

Cortexa is a local-first Tauri desktop application. Trusted Rust owns the
closed gateway protocol, local function-call validation, deterministic policy,
exact approval binding, typed approval audit, storage bootstrap, and narrow
platform adapters. React owns presentation and volatile demo state. Model,
provider, gateway, WebView, file, website, tool-result, and external-runtime
data are untrusted.

The source currently exposes three narrow Rust traits:

- `ToolRegistry` in `src-tauri/src/tools/registry.rs`;
- `PolicyEngine` in `src-tauri/src/policy/engine.rs`; and
- `ApprovalManager` in `src-tauri/src/approvals/manager.rs`.

These are domain seams, not a runtime abstraction. There is no current runtime
factory, coordinator, selector, dispatcher, executor, tool-result ingester, or
provider client.

### Provider abstractions

There is no current `AgentProvider`. D-032 removed the unused synchronous
`AgentProvider::complete`, its arbitrary-string request/response types, and its
mock. A future provider transport must be streaming, closed, bounded, and
separate from any runtime seam. Transitive `reqwest`, `hyper`, or `tokio`
lockfile entries come through Tauri and are not provider evidence. Likewise,
the npm lockfile's `hermes-parser` and `hermes-estree` packages are JavaScript
parser dependencies and are unrelated to Nous Hermes Agent.

### Native agent behavior and runtime lifecycle

`InitialGatewayTurn` at
`src-tauri/src/agent/gateway_request.rs:38` is the closest native composition
root. It:

- serializes one bounded `InitialGatewayRequest`;
- registers only `get_current_datetime@1` and `create_local_task@1` in an
  `InMemoryToolRegistry`;
- owns a `GatewayStreamValidator`, `InMemoryApprovalManager`, and
  `InMemoryApprovalAuditAdapter`;
- accepts caller-supplied normalized event bytes;
- independently validates a function call against the local schema;
- applies `DeterministicPolicyEngine`;
- creates one exact approval presentation when required;
- records one terminal approval resolution through the typed in-memory audit
  adapter; and
- provides local terminal, idempotent cancellation.

It performs no network I/O, model call, tool execution, persistence, or
continuation. `InitialGatewayTurn::new` is consumed only by its unit tests and
`src-tauri/tests/gateway_request_contract.rs`. It is therefore a verified
transport-free initial-turn security pipeline, not a complete agent runtime.

`GatewayStreamValidator` at
`src-tauri/src/agent/gateway_protocol.rs:253` validates protocol version,
identity, sequence, closed variants, event and byte limits, output limits,
function identity, cancellation, terminal state, and redacted closed failures.
`UntrustedFunctionCall` remains explicitly untrusted after gateway
normalization. `validate_function_call` in
`src-tauri/src/agent/function_call_validation.rs` creates a locally
schema-validated value that still grants no approval or execution authority.

No production run supervisor exists. Current deadlines and limits are frozen
contract values; no live timer drives them. Cancellation terminates the local
validator and any turn-owned pending call, but there is no transport or child
process to interrupt.

### Session model

React application state under `src/application/` holds volatile conversations,
messages, approvals, activity, retry, and run state. It is reset with the
frontend process and is part of the deterministic demonstration.

The Rust `storage` module provides a verified SQLCipher-capable SQLite
bootstrap, migrations, metadata, and a startup marker. It is not a conversation
store or product memory. D-033 removed the former arbitrary-content memory
scaffold. The Memory page is a visible empty-state placeholder and explicitly
claims nothing is stored.

### Tools, policy, approvals, and audit

`ToolSchema`, `ToolDefinition`, and `InMemoryToolRegistry` provide two closed
schema definitions and deterministic catalog behavior. No registered
implementation or executor exists, so even `PolicyOutcome::Allow` cannot cause
an effect.

`DeterministicPolicyEngine` consumes only locally validated `PolicyInput` and
derives a closed `PolicyDecision`. The decision is non-authorizing.
`InMemoryApprovalManager` binds one exact policy subject to one presentation and
one terminal resolution with replay, expiry, source, and identity checks.
`MacOsNativeApprovalDecisionSource` exists behind macOS conditional compilation
but has no production caller; its authentication evidence is explicitly
`NotEvaluated`.

`InMemoryApprovalAuditAdapter` records only a closed redacted terminal approval
projection and returns a sequence receipt. It is not durable and the receipt
does not authorize execution. There is no generic `AuditLogger`: D-030 removed
that arbitrary-string scaffold.

### Memory and platform behavior

There is no `MemoryStore` or memory-governance abstraction. Product memory
requires a separately approved bounded, user-controlled, encrypted lifecycle.
There is no generic `PlatformAdapter`: D-034 removed that unused scaffold.
Existing platform-specific behavior is narrow: Tauri startup/window/menu
handling, a disconnected macOS native approval source, and a disconnected
Cloudflare credential-availability probe that returns closed status without
exposing secret bytes.

### UI and Tauri boundary

`src-tauri/src/lib.rs` registers only the `get_app_info` Tauri command. The sole
native application event is the typed menu-route event. The main-window
capability at `src-tauri/capabilities/default.json` grants only `core:default`.
No agent, runtime, provider, approval, tool, memory, process, filesystem, or
network command crosses IPC.

`AppServices` in `src/App.tsx` injects `browserMockRunDriver` directly into the
React application. The frontend does not consume `InitialGatewayTurn`.

### Deterministic mocks

`MockRunDriver` uses browser timers to stream fixed text. `createMockRunScript`
states that no model is called and no tool is executed. `MOCK_LOOP_LIMITS`
sets network, file, search, and tool timeout capabilities to zero;
`MockToolResult` is always simulated and `executed: false`. The mock supports
stop, retry, approval-decision presentation, and final-answer UI behavior
without external I/O.

### Tests

Rust unit and integration tests cover closed protocol parsing, limits,
identity/sequence binding, malformed and duplicate JSON, local schemas,
deterministic policy, exact approval ownership, replay/termination, typed audit,
redaction, and cancellation. Key public suites are:

- `src-tauri/tests/gateway_request_contract.rs`;
- `src-tauri/tests/policy_input_binding.rs`;
- `src-tauri/tests/approval_binding.rs`; and
- `src-tauri/tests/approval_audit_binding.rs`.

Frontend suites including `src/application/mockRunDriver.test.ts` and
`src/App.test.tsx` cover deterministic streaming, stop, retry, approvals, and
rendered mock behavior. CI runs path-classified frontend, Rust, repository, and
audit checks on read-only workflows. There is no Hermes feature flag, runtime
feature flag, Hermes test, live-provider test, or runtime integration fixture.

## 3. Existing-work inventory

| Component               | Location                                                                                      | Responsibility                                                                 | Maturity and tests                                             | Current consumers                             | Architectural value                                     | Hermes overlap                                                                                          |
| ----------------------- | --------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ | -------------------------------------------------------------- | --------------------------------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| React application state | `src/application/ApplicationStateProvider.tsx`, `state.ts`, `conversations.ts`, `activity.ts` | Volatile UI state and deterministic transitions                                | Implemented demo state; extensive Vitest coverage              | `App`, conversation/activity/task views       | Separates presentation state from trusted Rust          | Hermes has persistent sessions and activity, but must not replace UI ownership automatically            |
| Frontend mock loop      | `mockRunDriver.ts`, `mockAssistantRun.ts`, `mockLoop.ts`, `mockToolResult.ts`                 | No-I/O streaming/approval/tool-result demonstration                            | Implemented mock; adjacent tests and `App.test.tsx`            | `AppServices`, `useMockAssistantRun`          | Stable visible fallback and deterministic UI fixture    | Hermes has a real loop; overlap is intentional and not deletion justification                           |
| Tauri boundary          | `src-tauri/src/lib.rs`, `capabilities/default.json`                                           | App startup, app-info command, menu lifecycle                                  | Shipping; smoke/menu tests                                     | Tauri application and typed frontend clients  | Narrow IPC and least privilege                          | Hermes Desktop has a much broader host bridge; it must not be copied into this boundary                 |
| Initial gateway turn    | `agent/gateway_request.rs` (`InitialGatewayTurn`)                                             | Bounded request plus validation, policy, approval, and typed audit composition | Verified transport-free code; unit and contract tests          | Tests only                                    | Strongest current native composition and trust sequence | Hermes provides full turn orchestration; a future native adapter should compose, not replace, this code |
| Gateway protocol        | `agent/gateway_protocol.rs` (`GatewayStreamValidator`)                                        | Normalize and validate untrusted streaming frames                              | Mature closed state machine with extensive negative tests      | `InitialGatewayTurn`, function/audit tests    | Versioned fail-closed external boundary                 | Hermes emits JSON-RPC events with different semantics; translation is required before this boundary     |
| Function validation     | `agent/function_call_validation.rs`                                                           | Revalidate gateway-normalized calls against local catalog                      | Verified success, mismatch, malformed, and redaction tests     | `InitialGatewayTurn`, policy/audit tests      | Prevents provider/runtime schema authority              | Hermes has its own tool schemas and dispatch; Cortexa validation remains independent                    |
| Tool catalog            | `tools/registry.rs`, `schema.rs`, `types.rs`                                                  | Closed local tool identity, schema, risk, permission metadata                  | Implemented catalog/schema; unit tests                         | `InitialGatewayTurn`, validation/policy tests | Application-owned capability inventory                  | Hermes has a large dynamic registry; only translated proposals for Cortexa tools may cross              |
| Policy                  | `policy/engine.rs`, `types.rs`                                                                | Deterministic classification from trusted typed input                          | Implemented and unit/integration tested                        | `InitialGatewayTurn`, approval/audit tests    | Application-owned authorization precursor               | Hermes approvals/policies are heuristics under its security model and cannot substitute                 |
| Approval manager        | `approvals/manager.rs`, `types.rs`                                                            | Exact subject, presentation, resolution, replay, expiry, termination           | Implemented in memory with unit/contract tests                 | `InitialGatewayTurn`, tests                   | Prevents model/runtime self-approval                    | Hermes exposes approval RPC; those methods must not bypass the manager                                  |
| macOS approval source   | `approvals/decision_source.rs`                                                                | Seal one native dialog result into manager-owned evidence                      | Implemented but disconnected; macOS tests; auth `NotEvaluated` | Tests only                                    | Narrow future native input source                       | Hermes has interactive approval prompts; no identity/evidence equivalence exists                        |
| Typed approval audit    | `audit/approval.rs`                                                                           | Closed redacted terminal approval record and receipt                           | Implemented in memory; unit/integration tests                  | `InitialGatewayTurn`, tests                   | Exact evidence without open strings                     | Hermes logs/events may inform future typed records but cannot be copied or treated as authorization     |
| SQLite bootstrap        | `storage/`                                                                                    | Migrations, metadata, connection safety, bootstrap marker                      | Implemented and unit/smoke tested                              | Startup and storage tests                     | Verified local persistence foundation                   | Hermes owns separate session SQLite and file memory; stores must remain isolated                        |
| Credential status probe | `credentials/cloudflare_access.rs`                                                            | Closed availability status for two fixed labels                                | Implemented proof, not wired; integration tests                | Tests only                                    | Demonstrates no-secret-return adapter style             | Hermes provider/tool secrets must never inherit this future credential boundary                         |
| Menu/platform adapter   | `menu_bar/`, `startup.rs`                                                                     | Window/menu lifecycle and typed route event                                    | Shipping and tested                                            | Tauri app                                     | Narrow target-gated OS ownership                        | Hermes terminal/file/browser/platform features overlap only functionally and remain prohibited          |
| Build and governance    | `Cargo.toml`, `package.json`, Tauri config, CI workflows, repository hooks                    | Pinned dependencies, strict lint, CSP/capabilities, read-only gates            | Active and tested                                              | Entire repository                             | Supply-chain and least-privilege baseline               | Hermes adds Python/Node/optional native dependency and packaging surfaces requiring separate review     |

## 4. Preservation matrix

| Component                                                                      | Classification                                      | Required treatment                                                                                                                 |
| ------------------------------------------------------------------------------ | --------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `GatewayStreamValidator` and closed gateway types                              | **KEEP AS-IS**                                      | Do not weaken limits or accept Hermes-native open payloads. Add a separate translation layer if later needed.                      |
| Function-call validation and local schemas                                     | **KEEP AND EXTEND**                                 | Add new Cortexa tools only through separately approved closed schemas; never import Hermes's registry as authority.                |
| `InitialGatewayTurn`                                                           | **ADAPT INTO NATIVE RUNTIME**                       | A future `NativeAgentRuntime` may compose it without renaming, moving, or broadening it into a generic provider API.               |
| Native runtime path as a whole                                                 | **KEEP AS FALLBACK**                                | Remain default and reference; “fallback” means explicit selection before a run, never automatic mid-run or provider failover.      |
| Deterministic frontend mock                                                    | **KEEP FOR TESTING**                                | Retain as visible no-I/O UI/test behavior until a separately approved UI/runtime increment supersedes specific consumers.          |
| `ToolRegistry`                                                                 | **KEEP AND EXTEND**                                 | Preserve local identity/schema/risk ownership; a future adapter may project a read-only subset outward.                            |
| `PolicyEngine`                                                                 | **KEEP AS-IS**                                      | Runtime/framework output never changes policy logic or supplies risk/permission metadata.                                          |
| `ApprovalManager` and native source                                            | **KEEP AS-IS**                                      | Hermes approval requests/responses cannot resolve Cortexa approval state. Authentication remains unproved.                         |
| Typed approval audit                                                           | **KEEP AND EXTEND**                                 | Add future closed runtime/process event families only with explicit design; do not restore a generic logger.                       |
| SQLite bootstrap                                                               | **KEEP AS-IS**                                      | Do not merge Hermes session or memory schemas into the application database.                                                       |
| Product memory                                                                 | **DEFER**                                           | Design a user-controlled memory-governance boundary independently; do not revive D-033's removed store.                            |
| Platform execution abstraction                                                 | **DEFER**                                           | Do not revive D-034 or expose Hermes terminal/file/browser/MCP capabilities. A restricted executor needs its own plan.             |
| D-032 provider transport                                                       | **REPLACE ONLY WITH EXPLICIT FUTURE JUSTIFICATION** | Any future provider transport must be a new closed streaming contract, distinct from runtime and from the deleted synchronous API. |
| Hermes adapter                                                                 | **WRAP BEHIND INTERFACE**                           | If approved, keep all Python, JSON-RPC, process, config, and upstream error types inside one adapter.                              |
| Hermes built-in tools, memory, skills, plugins, cron, messaging, and subagents | **DEFER**                                           | They are not part of the first experimental runtime and require separate capability-specific decisions.                            |

No component should be deleted merely because Hermes implements a similar
feature. Later removal still remains possible only through a separately
approved, evidence-backed increment.

## 5. Capability comparison

Classification describes verified capability location, not adoption priority.
“BOTH” may compare different maturity levels; “INTEGRATION REQUIRED” means both
sides have relevant boundaries whose semantics do not currently compose.

| Capability             | Classification           | Current Cortexa evidence                                                          | Verified Hermes `v2026.8.3` evidence                                                                               |
| ---------------------- | ------------------------ | --------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| Agent execution loop   | **HERMES**               | Deterministic UI mock and one transport-free initial turn; no live loop           | `AIAgent` owns prompt, provider, tool, retry, fallback, compression, callback, and persistence loop                |
| Model/provider routing | **HERMES**               | No provider transport or model selector                                           | Multiple providers/API modes, runtime resolution, model hot-swap, fallback configuration                           |
| Sessions               | **BOTH**                 | Volatile mock conversations; SQLite is not session storage                        | Persistent SQLite/FTS5 sessions, lineage, platform isolation, resume/branch APIs                                   |
| Streaming              | **BOTH**                 | Browser-timer mock and validated gateway frames; no live transport                | Message/tool/lifecycle streaming through TUI gateway, ACP, and HTTP/SSE                                            |
| Cancellation           | **BOTH**                 | Local idempotent terminal validator cancellation                                  | Session interrupt/stop and interactive interruption across documented surfaces                                     |
| Tools                  | **INTEGRATION REQUIRED** | Two closed schemas, deterministic policy, no executor                             | Large registry with dynamic dispatch and terminal/file/browser/code backends                                       |
| MCP                    | **HERMES**               | No MCP client or server                                                           | Stdio/HTTP MCP client with dynamic discovery; stdio MCP server surface                                             |
| Memory                 | **HERMES**               | No product memory store; UI placeholder only                                      | `MEMORY.md`, `USER.md`, provider plugins, write tools and session-end flows                                        |
| Skills                 | **HERMES**               | Repository Codex skills are developer workflow, not product capability            | Persistent executable/procedural skills, hub, external directories, plugins                                        |
| Skill generation       | **HERMES**               | None                                                                              | Agent-managed create/update/delete and self-improvement paths                                                      |
| Subagents              | **HERMES**               | Product multi-agent behavior prohibited; developer subagents are not product code | Delegation, child lifecycle, parallel tasks, tool RPC                                                              |
| Scheduled tasks        | **HERMES**               | None                                                                              | Cron jobs, scripts, platform delivery, background work                                                             |
| Messaging              | **HERMES**               | None                                                                              | Multi-platform long-running gateway and adapters                                                                   |
| Execution environments | **HERMES**               | No executor; narrow disconnected OS adapters only                                 | Local, container, remote, and cloud terminal/file backends plus code execution                                     |
| Observability          | **INTEGRATION REQUIRED** | Typed approval audit and mock Activity UI only                                    | Logs, session state, tool/lifecycle events, trajectories, hooks/webhooks; semantics are not Cortexa audit evidence |

Upstream capability evidence comes from the pinned
[architecture](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/website/docs/developer-guide/architecture.md),
[programmatic integration](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/website/docs/developer-guide/programmatic-integration.md),
[configuration](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/website/docs/user-guide/configuration.md),
and [MCP](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/website/docs/user-guide/features/mcp.md)
documents. Capabilities on upstream `main` that are not present at the pinned
release are not treated as available.

## 6. Target architecture

### Should `AgentRuntime` exist?

**Conditionally yes**, because application services need one framework-neutral
place to start, observe, and cancel either a native or external run without
learning Hermes types. The name must describe an untrusted event/lifecycle port,
not an owner of governance, tools, provider credentials, memory, or platform
effects. The ADR must be accepted before implementation.

If the next-phase contract cannot remain this small, stop. A giant runtime
interface would merely mirror Hermes and move application ownership across the
wrong boundary.

### Native runtime adaptation

No existing component can simply “become” a full `NativeAgentRuntime`.
`InitialGatewayTurn` is the correct verified nucleus to **compose behind** a
future native adapter, but it has no transport, run coordinator, executor, or
result loop. The adapter should preserve it intact and initially expose only
the behaviors the repository already proves. It must not rename it, inject
Hermes concerns into it, or revive D-032's provider contract.

### Smallest coherent contract

The conceptual application port has four responsibilities:

1. `describe`: return a closed `RuntimeDescriptor` with application-defined
   runtime ID, adapter version, protocol version, and bounded capabilities.
2. `start`: consume one closed, size-limited `RuntimeTurnRequest` and return one
   opaque, application-issued run handle.
3. `events`: deliver only closed `UntrustedRuntimeEvent` variants for that
   handle through an application-owned event sink/stream.
4. `cancel`: request terminal idempotent cancellation for that exact handle and
   return a closed outcome.

The concrete Rust async/stream mechanism is intentionally deferred. Selecting
an async library, blocking iterator, callback ownership model, or thread model
before a native implementation need would create a dependency-shaped decision.
The contract must not contain arbitrary provider request/response strings,
generic JSON escape hatches, generic RPC forwarding, tool execution, approval
resolution, storage access, secret values, or OS handles.

The initial event set should be limited to run-started, assistant-text delta,
untrusted tool proposal, run-completed, closed failure, and cancelled. Unknown
variants, versions, fields, identities, sizes, ordering, or late events fail
closed. Reasoning content and raw upstream errors need not cross.

### Separate interfaces

Keep these outside `AgentRuntime`:

- provider/gateway transport and credentials;
- tool catalog projection and tool proposal validation;
- deterministic policy;
- exact approval presentation and resolution;
- restricted execution and platform adapters;
- audit repositories and retention;
- session/conversation persistence;
- memory selection, consent, retention, edit, export, and deletion;
- process supervision and containment;
- runtime installation/update/package verification; and
- UI/runtime selection.

### Capability advertisement

`RuntimeCapabilities` should be a closed application-owned set, not arbitrary
strings or upstream JSON. Start with only capabilities the adapter contract can
prove, such as text streaming, cancellation, and untrusted tool proposals.
Missing or unknown capability data means unsupported. A capability declaration
never grants a permission. The host validates observed behavior against the
declared set and terminates on contradiction.

### Runtime selection and fallback

Trusted application configuration selects one runtime before a run. The native
runtime is the default. The model cannot select, change, or request a runtime;
the WebView may eventually submit a closed user choice but cannot construct an
adapter or command. Selection is fixed for the run and appears in typed audit
metadata.

There is no automatic cross-runtime fallback. A Hermes startup failure returns
a closed unavailable outcome. A later explicit owner action may start a new
native run with a new identity; it must never replay a possibly completed
mutation or continue a partially observed external run.

### Application-owned authority

Cortexa continues to own request limits, normalization, schemas, tool identity,
risk/permission metadata, policy, approval, authentication evidence,
cancellation state, restricted execution, audit, credentials, data retention,
platform access, runtime selection, disclosure, and user-visible truth. Hermes
may propose and stream; it may not decide or perform.

## 7. Governance boundary

### Boundary mapping

| Cortexa boundary          | Required Hermes interaction                                                                                                                                                                                                               |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ToolRegistry`            | Project only an exact, read-only allowlist of Cortexa schema descriptions if a later phase enables proposals. Never import Hermes tools or dispatch table. Every returned proposal is untrusted and revalidated locally.                  |
| `PolicyEngine`            | Evaluate only a locally validated call. Ignore Hermes risk, permission, allow/deny, explanation, or approval metadata.                                                                                                                    |
| `ApprovalManager`         | Create and resolve approval only from Cortexa-owned values and sources. Do not forward `approval.respond`, `sudo.respond`, or `secret.respond` as generic Hermes RPC. A Hermes approval has no Cortexa authority.                         |
| `AuditLogger`             | No such generic interface exists. Preserve D-030. Add only closed typed runtime/process records in a future approved increment; upstream logs are diagnostic input, not audit truth.                                                      |
| Memory governance         | No `MemoryStore` exists. Use a dedicated, initially ephemeral runtime home. Disable memory/profile/skill writes and never mount Cortexa SQLite or personal data until a separate memory design is accepted.                               |
| `PlatformAdapter`         | No generic interface exists. Preserve D-034. Do not expose terminal, file, browser, clipboard, MCP, code execution, messaging, cron, or device APIs.                                                                                      |
| Configuration and secrets | Rust owns a closed configuration projection. Pass the minimum through a dedicated channel to a contained process; no inherited environment, shell profile, user `~/.hermes`, repository secrets, WebView token, or ordinary log exposure. |
| Agent identity            | Treat runtime ID/version/process instance and run ID as opaque technical identity only. It is not a human identity, authenticated approver, provider identity, or authorization principal.                                                |
| Runtime lifecycle         | A Rust supervisor owns verified executable identity, spawn, readiness, protocol negotiation, deadlines, backpressure, cancellation, graceful shutdown, forced termination, child cleanup, restart policy, and redacted diagnostics.       |

### Plausible bypass paths

Every item below is blocked by default and requires explicit evidence before a
later phase can relax it:

1. Hermes built-in terminal, file, patch, browser, web, code-execution, voice,
   media, and messaging tools reaching the host directly.
2. MCP stdio children or HTTP servers inheriting environment, filesystem,
   network, or tool authority.
3. Python plugins, hooks, and skills importing arbitrary code inside the agent
   process with access to process credentials.
4. Agent-managed skill or memory writes persisting prompt-injected content,
   personal data, or executable instructions.
5. Hermes provider/model configuration receiving keys, choosing unapproved
   endpoints, using auxiliary models, or performing automatic fallback.
6. Broad TUI-gateway methods such as `config.set`, `command.dispatch`,
   `cli.exec`, `reload.env`, `reload.mcp`, `clipboard.paste`, `image.attach`,
   `prompt.background`, delegation, and approval/secret/sudo responses.
7. Treating Hermes's smart approval, redaction, Skills Guard, or tool allowlist
   as a containment or authorization boundary.
8. A loopback/WebSocket listener reachable by another local process, a leaked
   token, DNS rebinding, public bind, or unintended remote client.
9. Inherited current directory, environment, open file descriptors, shell
   startup files, home-directory access, Keychain access, or process-inspection
   rights.
10. Raw prompts, deltas, tool arguments, tracebacks, headers, tokens, memory,
    or personal data entering Hermes/Cortexa logs or crash reports.
11. Orphaned child processes, double completion, cancellation races, late
    events, automatic restart, or reuse of a stale session after the host
    believes a run terminal.
12. Protocol downgrade, unknown JSON-RPC methods/events, duplicate IDs,
    unbounded messages, method confusion, or upstream semantic drift.
13. Hermes sessions, `state.db`, `MEMORY.md`, `USER.md`, skills, cron, or logs
    silently becoming Cortexa's authoritative data.
14. The WebView connecting directly to Hermes or receiving the runtime's
    bearer/session token.
15. Automatic `hermes update`, mutable `main`, lazy dependencies, MCP package
    installers, plugins, skills, or self-modifying assets changing reviewed
    code after approval.
16. A failed Hermes run automatically falling back to native and replaying a
    request whose external outcome is unknown.

The upstream [security policy at the pinned
release](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/SECURITY.md)
explicitly says the default local terminal backend runs on the host, only
OS-level isolation is a security boundary against an adversarial LLM,
terminal-backend isolation does not contain in-process plugins/hooks/skills or
host code/MCP children, and whole-process wrapping is the supported posture for
untrusted inputs. Cortexa must enforce a boundary at least this strong outside
the Hermes process; it cannot delegate trust to upstream heuristics.

## 8. Integration options

### Comparison

| Criterion                 | A. Managed subprocess                                                                            | B. Hermes gateway/API                                                            | C. MCP                                                                                           | D. Embedded/tight coupling                                                  |
| ------------------------- | ------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------- |
| Verified surface          | TUI-gateway JSON-RPC over stdio/WebSocket; ACP stdio                                             | `hermes serve` JSON-RPC/WebSocket; OpenAI-compatible HTTP/SSE API                | Hermes client supports stdio/HTTP; `hermes mcp serve` is stdio                                   | Python `AIAgent` import documented                                          |
| Protocol stability        | Medium-low; documented but no compatibility/support window or explicit version negotiation found | Medium-low; documented endpoints, still fast-moving and broad                    | Medium for MCP framing, low for mapping full runtime lifecycle/governance                        | Low; Python internals and constructor semantics may change                  |
| Isolation                 | Best option only with whole-process OS containment and stdio                                     | Process isolation possible, but adds a listening network surface                 | Server isolation possible, but tool protocol is not a runtime boundary                           | Worst; same address/process trust or embedded interpreter                   |
| Crash containment         | Parent can supervise/kill process tree                                                           | Parent can supervise server but must reconcile sockets/sessions                  | Child/server can be killed, but lifecycle is tool-call-centric                                   | Python crash/GIL/native extension can impair desktop process                |
| Security                  | Can deny inherited env/files/network and allowlist messages                                      | Broad RPC/HTTP methods, auth/token, SSRF/listener exposure                       | Designed to expose tools/resources; risks creating a direct device-tool bridge                   | Violates desired trust separation; plugins/skills share process privilege   |
| Lifecycle control         | Strongest: explicit spawn/readiness/cancel/kill/cleanup                                          | Good but includes port selection, readiness/auth, reconnect, and server shutdown | Insufficient for full sessions, streams, approvals, and run cancellation without custom protocol | Coupled to Python interpreter and agent object lifecycle                    |
| Observability             | Host can normalize bounded stdout/stderr/events                                                  | Rich lifecycle/SSE/JSON-RPC events, but must redact and translate                | Tool-centric request/result telemetry                                                            | Direct internal access encourages coupling and raw-data leakage             |
| Portability               | Requires packaged Python runtime and OS containment per platform                                 | Same plus Web stack/network policy                                               | MCP libraries/servers add their own platform/runtime requirements                                | Rust/Python embedding, ABI, native wheels, and packaging are least portable |
| Upgradeability            | Exact executable/artifact can be pinned and replaced atomically                                  | Server/client compatibility must be coordinated                                  | MCP base protocol helps, but Hermes-specific semantics still drift                               | Highest source/API coupling and migration cost                              |
| Testability               | Excellent with a fake child speaking the same closed projection                                  | Good with local fake server; more network-state cases                            | Good for tool calls, poor for complete agent lifecycle                                           | Requires Python/runtime in tests or heavy mocks                             |
| Packaging                 | High complexity but separable as an optional component                                           | Higher: Python plus web extras/server/token/port management                      | Variable; may pull package managers and servers                                                  | Highest: interpreter, ABI, extensions, licenses inside app                  |
| Implementation complexity | High                                                                                             | High to very high                                                                | Medium for a narrow tool, high/incoherent for a runtime                                          | Very high                                                                   |
| Recommendation            | **Preferred, conditional**                                                                       | Secondary transport only if stdio is unavailable and auth is proved              | Not the primary runtime mechanism                                                                | No-Go for initial integration                                               |

### A. Managed subprocess

This best preserves language and failure isolation. The pinned release's
programmatic-integration guide explicitly documents TUI-gateway JSON-RPC over
stdio for custom hosts and exposes streaming, cancellation, session, approval,
tool, and readiness events. Cortexa must implement a much smaller projection,
not a generic JSON-RPC client.

Process separation alone is insufficient. A normal child inherits the user's
authority. Conditional approval requires a verified whole-process containment
profile, a dedicated runtime home, no host secrets, no unreviewed plugins or
skills, no automatic update, no built-in device tools, a process-tree kill
strategy, and protocol conformance fixtures.

### B. Hermes gateway or API

The pinned release documents:

- TUI gateway JSON-RPC over WebSocket;
- an OpenAI-compatible API server with HTTP/SSE run, events, approval, stop,
  capability, model, and health endpoints; and
- `hermes serve` as the headless backend used by Hermes Desktop.

These are real integration surfaces, but the network listener and broad method
catalog add token, bind, origin, reconnect, and local-client risks. The OpenAI
API also resembles provider transport more than the proposed runtime seam and
could blur D-032. Use only if a later spike proves stdio cannot meet the
lifecycle need.

### C. MCP-based integration

Hermes is both an MCP client and a stdio MCP server. MCP is useful for exposing
an exact tool/resource boundary, not for delegating Cortexa's complete runtime
lifecycle. Making Cortexa an MCP server for its tools would let Hermes request
them, but those requests would still need local schema, policy, approval,
execution, and audit; it also risks turning MCP into a generic model-to-device
bridge. MCP is deferred until a separately approved, least-privilege tool
increment has an actual use case.

### D. Embedded or tightly coupled integration

Upstream documents importing `run_agent.AIAgent` from Python. That is technically
feasible in a Python host, but Cortexa is Rust/Tauri and has no Python embedding
boundary. Embedding would add interpreter/ABI/native-wheel/GIL and shutdown
complexity, put Hermes plugins/skills inside a privileged process, and couple
Cortexa to volatile Python internals. It is **No-Go** for the initial
integration.

### Recommendation

Choose option A only after the native-only boundary plan completes and a
separate security-sensitive containment/protocol spike is approved. The
preferred wire is newline-delimited JSON-RPC over stdio with a host-side closed
allowlist. Do not expose raw JSON-RPC to application services or the WebView.
Do not proceed if an exact supported launcher, version negotiation strategy,
whole-process confinement, or tool/memory/skill disablement cannot be proved.

## 9. Current personal-project scope

Cortexa is currently a private, personally owned, local-first side project with
one technical user. The Hermes evaluation should therefore defer:

- SaaS control planes, tenant databases, tenant isolation, quotas, and service
  administration;
- multi-tenancy and organization/team concepts;
- billing, metering, subscriptions, entitlements, and commercial analytics;
- enterprise IAM, SSO, SCIM, RBAC matrices, and centralized policy;
- distributed orchestration, queues, fleets, worker pools, HA, failover, and
  disaster-recovery infrastructure;
- centralized runtime installation, configuration, patch, or fleet management;
- public skills/plugins/MCP/runtime marketplaces;
- commercial licensing systems;
- public cloud deployment pipelines and public gateway exposure; and
- remote multi-user messaging, scheduled autonomous work, and background
  subagent fleets.

Deferring these features does not justify hard-coding application logic to one
framework. The clean runtime port, closed capabilities, removable adapter, and
application-owned governance preserve extensibility without building a product
platform now. Existing accepted future consumer and enterprise roadmap states
remain possible; this assessment neither cancels nor implements them.

## 10. Future publication constraints

Preserve these decisions now so later publication remains practical:

1. Keep Hermes optional and removable; Cortexa must build and run natively
   without it.
2. Keep all upstream types, config paths, protocol payloads, errors, logs, and
   process details inside one adapter.
3. Preserve deterministic native mocks and contract tests so contributors need
   no provider key, network, Python, or Hermes install for normal validation.
4. Pin a reviewed Git commit and immutable distribution hash. Do not use
   `main`, `latest`, mutable installers, `hermes update`, lazy packages, or
   auto-update in a product build.
5. Maintain a dependency/license/SBOM inventory for the selected Python
   distribution, optional extras, native wheels, Node assets, installers, MCP
   servers, skills, and plugins. MIT covers Hermes source but not automatically
   every optional/transitive component or model/provider term.
6. Use a dedicated versioned runtime data directory with explicit migration,
   backup, deletion, and downgrade behavior; never silently adopt a user's
   global `~/.hermes`.
7. Keep app data, runtime data, provider credentials, logs, audit, and crash
   reports separately owned and documented.
8. Make external processing, model/provider choice, local process execution,
   optional download size, runtime version, and data retention visible before
   activation.
9. Keep runtime selection and fallback deterministic and exportable; no
   proprietary data format may prevent a user from returning to native.
10. Define supported OS/architecture/Python/runtime combinations and a security
    update policy before distribution.
11. Require reproducible package verification, rollback, uninstall, and removal
    of runtime data before claiming public support.
12. Resolve O-008 before public distribution and O-009 before trusted public
    macOS distribution. No commercial infrastructure is needed now.

## 11. Incremental migration plan

Every phase below is independent, owner-selected, reversible, and stops before
the next phase. Native remains the default and functional throughout.

### Phase 0 - accept or reject the architecture decision

- Objective: decide whether the narrow application-owned runtime port is worth
  implementing.
- In scope: review this assessment and Proposed ADR; resolve contract ownership,
  terminology, and acceptance criteria.
- Non-goals: code, dependency, Hermes, process, provider, UI, and behavior.
- Validation: architecture, security, and readiness review against D-030,
  D-032, D-033, D-034, and D-078.
- Rollback: leave the ADR Proposed or mark it Rejected; native code remains
  untouched.
- Stop condition: no implementation until the owner accepts a bounded decision
  and selects the separate Phase 1 plan.

### Phase 1 - native-only runtime boundary

- Objective: introduce the smallest closed runtime lifecycle/event port and a
  native adapter that preserves current behavior.
- In scope: application-owned descriptor/request/event/error/cancellation
  contracts, `NativeAgentRuntime` composition, deterministic contract tests,
  and architecture documentation.
- Non-goals: Hermes, external processes, provider transport, networking,
  credentials, runtime UI selection, new tools, execution, memory, or behavior.
- Validation: Rust format/Clippy/tests; contract tests for identities, limits,
  unknown variants, cancellation, late events, errors, and native parity; full
  repository verification required by the approved plan.
- Rollback: remove the new adapter/contracts and restore direct test
  construction of the unchanged `InitialGatewayTurn`.
- Stop condition: native parity and no new authority must pass before any
  external-runtime work. The proposed ExecPlan accompanying this assessment
  defines this phase and is not executed now.

### Phase 2 - deterministic external-runtime harness

- Objective: prove the adapter and supervisor against a local fake process, not
  Hermes.
- In scope: test-only child/fake transport, bounded framing, readiness,
  backpressure, crash, malformed message, cancellation, timeout, and orphan
  cleanup tests.
- Non-goals: third-party code, network, provider, tools, memory, secrets, UI,
  and production process activation.
- Validation: adversarial protocol fixtures, process-tree cleanup, redaction,
  no inherited secret/environment assertions, and native regression suite.
- Rollback: remove the fake adapter/supervisor; retain Phase 1 native boundary.
- Stop condition: any escape, orphan, leak, ambiguous terminal state, or giant
  contract blocks progression.

### Phase 3 - pinned Hermes containment and protocol spike

- Objective: verify one exact Hermes artifact can start, negotiate, stream a
  synthetic no-tool turn, cancel, and terminate inside the required OS boundary.
- In scope: separately approved disposable environment, immutable artifact/hash,
  dedicated empty runtime home, upstream self-update disabled, no host mounts,
  no inherited credentials, no built-in tools/MCP/memory/skills/plugins/hooks,
  and captured protocol fixtures.
- Non-goals: repository production dependency, product packaging, WebView/UI,
  real user content, real provider credentials, tool proposals, persistence,
  and release claims.
- Validation: version/provenance verification, protocol conformance, whole-
  process isolation tests, network/filesystem denial, crash/cancel/kill cleanup,
  and security review of the complete distribution.
- Rollback: delete the disposable runtime environment and its synthetic data;
  native application remains unchanged.
- Stop condition: no supported launcher, protocol drift, any device/secret
  reachability, or inability to disable mutable capabilities yields No-Go.

### Phase 4 - optional synthetic Hermes adapter

- Objective: add an off-by-default experimental adapter using only the proven
  closed event projection and owner-approved synthetic text.
- In scope: contained managed subprocess, explicit installed-version check,
  trusted Rust selection, synthetic-only run, closed errors, and redacted
  diagnostics.
- Non-goals: tools, memory, skills, plugins, MCP, subagents, cron, messaging,
  automatic provider/runtime fallback, personal content, and public release.
- Validation: native remains default; hermetic fake tests; separately approved
  target-Mac synthetic evidence; exact provider/external-processing gates if
  any network model is used.
- Rollback: disable/remove the optional adapter and runtime data; native remains
  fully functional.
- Stop condition: any trust-boundary bypass, unsupported artifact, pending
  disclosure/provider evidence, or native regression blocks activation.

### Phase 5 - narrowly broaden one capability at a time

- Objective: evaluate a demonstrated personal-project need, such as persistent
  sessions or one untrusted Cortexa tool proposal.
- In scope: exactly one capability with its own decision, plan, data model,
  security review, user control, tests, retention, and rollback.
- Non-goals: importing Hermes's general tool catalog, global home, memory,
  skills, MCP, marketplace, messaging, schedules, or subagents as a bundle.
- Validation: capability-specific negative tests plus the complete native and
  external runtime contract suite.
- Rollback: disable and remove only that optional capability without data loss
  or native impact.
- Stop condition: stop after one capability; do not infer authorization for the
  next.

## 12. Risks

| Category                    | Severity                | Risk                                                                                                                                            | Required mitigation / blocker                                                                                                       |
| --------------------------- | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Architecture                | High                    | Runtime could become a second application layer or duplicate provider, tool, approval, memory, and audit ownership                              | Accept only the small lifecycle/event port; keep every governance service separate; stop if adapter-specific needs leak outward     |
| Security                    | Critical if uncontained | Hermes's normal process can execute tools and load privileged Python code; upstream calls in-process controls heuristics                        | Whole-process OS containment, no host secrets/paths, tiny method allowlist, security proof before any product run                   |
| Process management          | High                    | Child leaks, orphan descendants, hangs, double terminal events, late output, or restart can violate run state                                   | Rust supervisor, process-group ownership, readiness/deadlines/backpressure, idempotent cancel, forced cleanup, no automatic restart |
| Python/Rust boundary        | High                    | Framing, encoding, async/threading, ABI, error, and shutdown mismatches create complex failure states                                           | Subprocess not embedding; bounded UTF-8 JSON framing; closed translation; fake-process tests; no raw JSON escape hatch              |
| Protocol stability          | High                    | TUI gateway is documented but no compatibility window or explicit protocol-version guarantee was identified                                     | Pin exact release/commit, vendor contract fixtures, negotiate closed version, reject unknowns, upgrade only in separate increment   |
| Dependency and supply chain | High                    | Installer, Python packages, optional extras, Node assets, lazy installs, MCP servers, plugins, and skills greatly expand reviewed code          | Immutable artifact/hash, no curl-to-shell in product, no self-update/lazy install, SBOM/license/advisory review, minimal extras     |
| Licenses                    | Medium                  | Hermes is MIT, but optional/transitive packages, models, providers, skills, and assets may carry other terms                                    | Full selected-distribution license inventory; preserve notices; resolve O-008 before publication                                    |
| Secrets                     | Critical if exposed     | `~/.hermes/.env`, `auth.json`, provider pools, inherited env, logs, or RPC secret prompts can bypass Keychain goals                             | Dedicated empty home, allowlisted secret broker only after approval, no WebView tokens, no global home, redacted closed errors      |
| Memory privacy              | High                    | Hermes stores sessions, memories, user profile, logs, skills, and search indexes outside Cortexa controls                                       | Disable initially; isolated ephemeral home; later explicit consent/retention/edit/delete/export/encryption design                   |
| Skill execution             | Critical if enabled     | Skills/plugins/hooks may execute Python with process privilege and agent-managed writes default permissive in upstream config                   | Disable and omit; no external directories; immutable reviewed runtime; separate future decision for any skill capability            |
| Version drift               | High                    | GitHub latest is 0.20.0 while PyPI remains 0.19.0; release page reported 1,150 later main commits; updates pull mutable code                    | Choose one exact channel and artifact; pin commit/hash; disable `hermes update`; maintain compatibility matrix and rollback         |
| Packaging                   | High                    | Python 3.11-3.13, native wheels, web/TUI extras, architecture support, signing, notarization, size, and uninstall complicate Tauri distribution | Keep optional/out-of-bundle initially; packaging decision and target-Mac evidence later; no publication claim                       |
| Testing                     | High                    | Happy-path mocks could miss protocol abuse, cancellation races, process escape, and upstream upgrade drift                                      | Fake-process adversarial suite, golden protocol fixtures, containment tests, native parity, no-network default tests                |
| Future publication          | Medium                  | Bundling or downloading an agent runtime changes disclosures, support, update, security-response, and data-boundary obligations                 | Removable adapter, transparent opt-in/version/data disclosure, license/SBOM/update/uninstall policy before distribution             |

### Upstream provenance and verified facts

- Official repository:
  [NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent).
- GitHub latest release inspected:
  [`v2026.8.3`](https://github.com/NousResearch/hermes-agent/releases/tag/v2026.8.3),
  titled Hermes Agent v0.20.0, released 2026-08-03, release commit
  [`3c27eb6234bf91b8ceee9e9071591b31e9b148cb`](https://github.com/NousResearch/hermes-agent/commit/3c27eb6234bf91b8ceee9e9071591b31e9b148cb).
- Pinned release metadata:
  [`pyproject.toml`](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/pyproject.toml)
  declares version 0.20.0, Python `>=3.11,<3.14`, MIT, and exact-pinned core
  dependencies.
- PyPI latest distribution inspected:
  [`hermes-agent` 0.19.0](https://pypi.org/project/hermes-agent/), uploaded
  2026-07-20 from tag `v2026.7.20` and commit
  `3ef6bbd201263d354fd83ec55b3c306ded2eb72a`, with Trusted Publishing
  provenance. It does not match GitHub's latest 0.20.0 release.
- License: [MIT](https://github.com/NousResearch/hermes-agent/blob/v2026.8.3/LICENSE).
- Documented install: official shell installer on Linux/macOS/WSL/Termux and
  PowerShell installer on Windows; the installer provisions Python and other
  tools. No installer or code was run for this assessment.
- Documented invocation: `hermes`, `hermes model`, `hermes tools`,
  `hermes config`, `hermes gateway`, `hermes setup`, `hermes update`,
  `hermes doctor`; programmatic surfaces include `hermes acp`, TUI-gateway
  JSON-RPC, `hermes serve`, and the HTTP/SSE API server.
- Protocols: ACP JSON-RPC/stdio; TUI-gateway JSON-RPC over stdio or WebSocket;
  OpenAI-compatible HTTP plus SSE; MCP client over stdio/HTTP and a stdio MCP
  server. No stable library ABI or formal TUI-gateway compatibility window was
  identified.
- Lifecycle: `AIAgent` performs provider selection, prompt assembly, model
  calls, tool loop, retry/fallback, callbacks, compression, and session
  persistence. Long-running gateway/serve modes add session and process
  lifecycle; TUI-gateway methods include interrupt and process stop.
- Configuration/state: by default under `~/.hermes/`, including `config.yaml`,
  `.env`, `auth.json`, `SOUL.md`, memories, skills, cron, sessions/state, and
  logs. CLI arguments and config/environment precedence affect behavior.
- Provider behavior: multiple provider and model modes, API keys/OAuth/custom
  endpoints, auxiliary calls, hot switching, and fallback exist. Cortexa may
  not inherit these choices implicitly.
- Tool behavior: upstream architecture documents a large dynamic registry and
  multiple terminal/browser/web/MCP backends; local terminal execution is a
  normal capability. Tool requests remain untrusted in Cortexa.
- Memory/skills: persistent memory and profile files are injected into context;
  skills can be loaded, created, updated, and deleted; plugins/skills may run
  Python in process. They remain disabled/deferred for Cortexa.
- Version risk: GitHub release, PyPI, and mutable `main` are already distinct
  channels. The official update flow may change source, dependencies,
  configuration, and running services. A future integration cannot track
  `main` or `latest` automatically.

### Unresolved blockers

The recommendation remains conditional until all of the following are resolved:

1. the Proposed ADR is accepted, amended, or rejected by the owner;
2. the native-only runtime boundary is implemented and verified without Hermes;
3. one exact upstream artifact/distribution channel and full hash are selected;
4. upstream auto-update, lazy dependency, plugin, skill, MCP, and self-modifying
   behavior can be disabled and verified;
5. a supported noninteractive stdio launcher or a secured loopback transport is
   proven for the pinned version;
6. a versioned closed method/event schema and compatibility policy are defined;
7. whole-process containment on supported target Macs is designed and proved;
8. child-process lifecycle, cancellation, orphan cleanup, redaction, and crash
   behavior pass adversarial tests;
9. provider/network/credential/disclosure requirements are separately decided;
10. no built-in tool, memory, skill, plugin, hook, cron, messaging, subagent,
    clipboard, file, shell, browser, or direct device path is reachable; and
11. packaging, transitive license, SBOM, signing/notarization, update, rollback,
    and uninstall implications are reviewed before distribution.

Until then, `HermesAgentRuntime` is a documentation-only concept and the native
path remains the sole reference direction.
