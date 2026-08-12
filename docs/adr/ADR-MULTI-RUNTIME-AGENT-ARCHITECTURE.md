# ADR: Multi-runtime agent architecture

Status: Accepted
Date: 2026-08-11
Decision owners: Project owner
Related accepted decisions: D-078, D-079, D-080
Assessment:
[`HERMES_INTEGRATION_ASSESSMENT.md`](../architecture/HERMES_INTEGRATION_ASSESSMENT.md)

The project owner accepted this architecture through D-079. Acceptance makes
the small application-owned runtime target authoritative and makes the separate
native boundary plan Ready; it does not implement a runtime or authorize code,
dependencies, Hermes execution, a provider, a process, a credential, UI, or a
behavior change in this documentation increment.

> **Transport result and subsequent decision (2026-08-11):** The isolated
> [`HERMES_TRANSPORT_SPIKE.md`](../spikes/HERMES_TRANSPORT_SPIKE.md) found that
> Hermes Agent package/application version `0.20.0`, release tag `v2026.8.3`,
> at commit `3c27eb6234bf91b8ceee9e9071591b31e9b148cb` has no supported public raw
> TUI-gateway stdio launcher, initial version/capability negotiation, or
> gateway-shutdown RPC. Raw TUI-gateway stdio is therefore rejected as a
> supported production contract at that release. D-080 conditionally selects
> managed local `hermes serve` plus the documented TUI-gateway
> JSON-RPC/WebSocket surface for a later contained spike after native-runtime
> completion. ACP remains deferred. No Hermes adapter is authorized or Ready.

## Context

Cortexa currently has no `AgentRuntime`, `NativeAgentRuntime`,
`HermesAgentRuntime`, runtime selector, live provider, live agent coordinator,
dispatcher, executor, or product memory store.

The verified Rust path consists of a transport-free `InitialGatewayTurn` that
serializes one bounded request, validates normalized gateway events and local
tool schemas, applies deterministic policy, binds an exact approval, records a
closed in-memory approval audit, and cancels locally. It is used by tests, not
the shipping Tauri command surface. The visible React agent flow is a separate
deterministic no-I/O mock. Neither is a complete native runtime.

D-032 removed a legacy synchronous, arbitrary-string `AgentProvider` API. D-030,
D-033, and D-034 similarly removed unused generic audit, memory, and platform
scaffolds. A new runtime abstraction must not revive those interfaces or turn a
framework into a second authority layer.

D-078 permits planning an application-owned runtime seam with a native default
and an optional experimental Hermes adapter. It requires all runtime output to
remain untrusted and keeps validation, policy, exact approval, restricted
execution, cancellation, and audit in deterministic Rust.

Official Nous Research evidence for Hermes Agent package/application version
`0.20.0`, release tag `v2026.8.3`, shows a broad Python runtime with provider
routing, sessions, streaming, tools, MCP, memory, skills, plugins, subagents,
cron, messaging, multiple execution backends, and several programmatic
protocols. The documented TUI gateway can be driven by a custom host through
JSON-RPC over stdio or WebSocket. The same method surface also exposes
approvals, secrets, configuration, CLI execution, tools, and subagent controls.
Hermes's security policy says OS-level isolation is the only load-bearing
boundary against an adversarial LLM; in-process approvals, redaction, scanners,
and allowlists are heuristics.

## Decision

Introduce a small, application-owned `AgentRuntime` port whose only purpose is
to describe, start, observe, and cancel an untrusted runtime run.

Conceptually:

```text
Application services
       |
       v
Cortexa governance and data ownership
       |
       v
AgentRuntime
  |-- NativeAgentRuntime
  `-- HermesAgentRuntime (optional, experimental, later)
```

The port would expose only:

- a closed runtime descriptor and bounded capability set;
- one bounded start request producing an opaque application-issued run handle;
- a closed stream/sink of untrusted lifecycle, text, proposal, terminal, and
  failure events; and
- exact terminal idempotent cancellation.

The specific Rust concurrency/stream mechanism is deliberately not decided by
this ADR. The contract may not expose arbitrary JSON, arbitrary strings,
provider-native types, generic RPC, tool execution, approval resolution,
secrets, storage handles, OS handles, or framework configuration.

`NativeAgentRuntime` would be implemented first and would preserve and compose
the current `InitialGatewayTurn` and typed native boundaries. It would be the
default, reference implementation, deterministic contract-test path, explicit
fallback, and independently usable runtime. It would not imply that the current
native path already has transport, coordination, or execution.

`HermesAgentRuntime` may be considered only in a later separately approved
increment after the native boundary is verified and D-080's contained transport
spike passes. It would use one immutable, version-pinned Hermes artifact in
whole-process OS containment, managed by a Rust supervisor. All Hermes-specific
process, protocol, configuration, compatibility, and error types would remain
in the adapter. No raw RPC relay would escape the adapter.

Runtime selection would be trusted, explicit, and fixed before each run. Native
would be the default. A runtime failure would not cause automatic failover;
returning to native would start a new explicitly authorized run so an ambiguous
external outcome cannot be replayed.

## Hermes transport status

D-080 selects one transport direction for evaluation only: a Rust-supervised,
managed local `hermes serve` child bound to loopback, with a closed projection
of the documented TUI-gateway JSON-RPC/WebSocket surface. The selection is
conditional on a separately executed spike after the native boundary completes;
it is not production-adapter approval.

- **Raw TUI-gateway stdio:** rejected as a supported production transport for
  Hermes Agent package/application version `0.20.0`, release tag `v2026.8.3`.
  The completed spike found no public launcher, no initial
  protocol/version/capability negotiation, and no gateway-shutdown RPC. Its
  fixture-only mechanics are not Hermes conformance, containment, or packaging
  evidence.
- **Hermes serve:** conditionally selected for a contained spike only. The spike
  must prove authenticated numeric-loopback startup, whole-distribution
  provenance, preinstalled `[web]`/POSIX `[pty]` support, suppression of the
  pinned lazy-install/update paths, denial of every pinned dotenv/managed-secret
  source, endpoint-level network and Unix-socket denial, containment-wide
  cleanup of detached descendants, bounded WebSocket translation, secret-safe
  configuration, isolated state, prohibited-capability failure, and target-Mac
  lifecycle evidence. The exact conditional constraints are authoritative in
  [`ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md`](ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md).
- **Hermes ACP:** deferred as a documented fallback. It is neither selected nor
  implemented and requires a new owner decision before any assessment or code.
- **Native-only:** remains the default, reference, deterministic test path,
  explicit fallback, potential standalone runtime, and valid long-term outcome
  if the Hermes spike or adapter does not pass.

No option above authorizes Hermes installation or execution in this increment,
a dependency, process, socket, credential, provider, UI, runtime selector, or
adapter. Later phases do not begin automatically.

## Alternatives

### 1. Keep native-only with no runtime abstraction

Benefits:

- smallest present architecture;
- no new abstraction before a second runtime exists; and
- no risk of a misleading interface or framework coupling.

Costs:

- later external-runtime work would be more likely to reach directly into
  application services or the existing gateway turn;
- native and external contract parity could not be tested independently; and
- the owner-approved portability direction in D-078 would remain only prose.

This remains a valid outcome if the proposed boundary cannot stay small or no
second runtime demonstrates value.

### 2. Adopt the small application-owned runtime port

Benefits:

- protects application and governance ownership;
- makes native preservation explicit and testable;
- contains framework churn in adapters;
- permits a deterministic fake process and contract fixtures; and
- preserves optionality without adopting Hermes now.

Costs:

- adds abstraction before live native orchestration exists;
- requires careful naming so mocked/planned/current states remain accurate; and
- can drift into a giant interface if future framework features are copied into
  it.

Selected and accepted for the native-first architecture under D-079.

### 3. Use Hermes's TUI gateway or HTTP API directly from application services

Benefits:

- uses documented upstream programmatic surfaces;
- exposes rich sessions, streaming, tool, approval, and lifecycle features; and
- reduces initial translation work.

Costs:

- leaks Hermes concepts and a broad method catalog across the application;
- lets upstream approval, secret, CLI, tool, memory, and session semantics
  compete with Cortexa ownership;
- makes a protocol upgrade a cross-application migration; and
- creates a likely Rust/WebView-to-Hermes bypass.

Rejected under this proposal.

### 3a. Adopt raw TUI-gateway stdio as the Hermes adapter transport

Rejected for the evaluated Hermes Agent package/application version `0.20.0`,
release tag `v2026.8.3`. The completed spike found that its raw gateway requires
an internal Python-module entry point and lacks a supported public launcher,
initial version/capability negotiation, and gateway-shutdown RPC. A
deterministic fake proves only host-side framing and lifecycle mechanics; it is
not evidence that the upstream interface is safe, stable, or packageable as a
production contract.

Any future upstream release with a public, version-negotiated, capability-bounded
raw transport would require a new assessment, immutable provenance, containment
evidence, and an owner-selected decision before reconsideration.

### 3b. Evaluate contained managed local `hermes serve`

Selected conditionally under D-080 for a separately gated spike after the
native boundary is complete. The spike must treat the broad server as untrusted,
use loopback-only authenticated WebSocket access and whole-process containment,
and return NO-GO if tool restriction, lifecycle, compatibility, or isolation
cannot be proved.

### 3c. Evaluate Hermes ACP

Deferred as a documented fallback. ACP is not selected or implemented and may
be evaluated only after a new owner decision.

### 4. Treat Hermes as the provider transport

Benefits:

- could reuse upstream model routing and provider support.

Costs:

- conflates agent orchestration with the separate gateway/provider boundary;
- risks reviving the D-032 shape;
- could move provider credentials and fallback policy out of Cortexa's accepted
  boundaries; and
- conflicts with D-060 through D-064 staged provider decisions.

Rejected under this proposal.

### 5. Expose Cortexa tools to Hermes through MCP

Benefits:

- uses a standard tool protocol and upstream MCP support.

Costs:

- MCP is a tool/resource protocol, not the complete runtime lifecycle needed
  here;
- a broad MCP server could become a generic model-to-device executor;
- every call would still need Cortexa schema, policy, approval, restricted
  execution, and audit; and
- MCP server installation and environment handling expand supply-chain and
  secret risk.

Deferred for a future one-tool use case; not the runtime mechanism.

### 6. Embed Python or import `AIAgent` in the desktop process

Benefits:

- direct API access and no separate transport.

Costs:

- interpreter/ABI/native-wheel/GIL and shutdown coupling;
- plugins, hooks, and skills share a privileged process;
- weaker crash and security containment;
- difficult Tauri packaging and portability; and
- volatile Python internal API becomes an application dependency.

Rejected for the initial integration.

### 7. Replace the native path with Hermes

Benefits:

- superficially avoids maintaining two paths.

Costs:

- discards verified typed trust boundaries and deterministic tests;
- creates framework lock-in and removes the safe default/fallback;
- adopts a materially different security model; and
- violates D-078 and the assessment's preservation requirement.

Rejected.

## Consequences

Acceptance means:

- a separately approved first implementation phase may add only the narrow
  runtime contract and native adapter;
- the native path remains the default and must preserve behavior and tests;
- provider transport, process transport, governance, tools, execution, memory,
  persistence, and UI selection remain separate;
- framework capabilities cannot expand the runtime contract without another
  explicit decision;
- the conditionally selected Hermes spike and any future adapter carry high
  process, protocol, packaging,
  dependency, and security cost even though the top-level interface is small;
- normal tests remain network-free and Hermes-free through native and fake
  implementations; and
- every external-runtime activation remains optional, explicit, observable,
  reversible, and separately approval-bound.

The main architectural downside is timing: Cortexa would introduce a port
before it has a shipping native coordinator. The next-phase plan must therefore
prove the abstraction through concrete native parity and delete or redesign it
if it cannot remain coherent without speculative methods.

## Preservation guarantees

Acceptance of this ADR would guarantee:

1. `InitialGatewayTurn`, `GatewayStreamValidator`, local schema validation,
   `ToolRegistry`, `PolicyEngine`, `ApprovalManager`, typed approval audit,
   deterministic mocks, and their tests are not removed or weakened by the
   runtime-boundary phase.
2. No deleted generic provider, audit, memory, or platform scaffold is revived.
3. Native remains the default, reference, deterministic test path, explicit
   fallback, and independently usable path.
4. “Fallback” means a new explicit selection, never automatic runtime or
   provider failover and never replay after an ambiguous outcome.
5. External framework types and arbitrary upstream payloads remain adapter-local.
6. Runtime output enters as untrusted input before local validation and policy.
7. A runtime descriptor or capability declaration is informational and never
   grants permission or authority.
8. No provider, network, credential, process, execution, memory, MCP, skill,
   subagent, cron, messaging, UI, or platform capability is authorized by the
   runtime interface itself.
9. No behavior-changing phase begins automatically after the native boundary.
10. A later evidence-backed removal increment remains possible; preservation
    is not an instruction to retain obsolete code forever.

## Security boundary

Trusted Rust remains the sole owner of:

- request construction and limits;
- protocol/version/identity/sequence validation;
- local tool identity, schema, risk, and permission metadata;
- deterministic policy;
- exact approval subject, presentation, resolution, replay, expiry, and
  authentication evidence;
- restricted execution and platform adapters;
- cancellation truth and terminal run state;
- credentials and external-processing disclosure;
- typed audit and retention; and
- session, memory, and application data governance.

An external runtime is an untrusted planner/event producer. Its built-in tool
allowlist, approval prompt, redaction, scanner, model routing, memory, or audit
does not satisfy a Cortexa gate.

A future Hermes process must be whole-process contained because upstream
explicitly treats OS isolation as the load-bearing security boundary. The
contained process receives no global home, inherited secrets, application
database, Keychain access, repository write access, built-in device tools,
MCP, memory, skills, plugins, hooks, cron, messaging, subagents, clipboard, or
generic RPC. The host must reject unknown or forbidden methods/events and own
spawn, readiness, backpressure, deadlines, cancellation, forced termination,
descendant cleanup, version verification, redaction, and diagnostic limits.

The WebView must never connect to Hermes or receive its process token. Any
event rendered in React first crosses a closed typed Rust IPC boundary.

## Deferred decisions

This ADR deliberately does not decide:

- the exact Rust trait syntax, async runtime, stream, callback, thread, or
  channel implementation;
- whether the final public name remains `AgentRuntime` after implementation
  review;
- provider/gateway transport or any model/provider selection;
- the exact Hermes distribution channel, artifact hash, optional extras, or
  installer used by a future contained spike;
- the concrete macOS whole-process containment technology;
- whether a later owner decision should evaluate Hermes ACP after the selected
  `hermes serve` path;
- TUI-gateway protocol versioning, schema generation, or compatibility window;
- runtime installation, update, rollback, repair, removal, or packaging UI;
- provider credentials, disclosure, live networking, or user data;
- any Hermes tool, MCP, memory, skill, plugin, hook, subagent, schedule,
  messaging, voice, browser, file, terminal, code, or platform capability;
- durable sessions, product memory, audit persistence, or execution;
- runtime selection UI;
- OpenClaw evaluation or migration; and
- public distribution, licensing inventory, signing, notarization, support,
  or commercial infrastructure.

Each requires a later decision or independently approved increment when there
is a demonstrated need.

## Acceptance criteria

Acceptance was recorded after the project owner confirmed all of the following:

- [x] The small runtime port has one clear application owner and does not
      duplicate provider transport or governance.
- [x] The proposed native adapter composes rather than rewrites
      `InitialGatewayTurn` and preserves all existing contracts and tests.
- [x] Runtime requests, events, capabilities, errors, identities, sizes,
      ordering, terminal state, and cancellation are closed and bounded.
- [x] The concrete design contains no arbitrary JSON/string/RPC escape hatch.
- [x] Tool, policy, approval, execution, audit, memory, storage, secrets, and
      platform responsibilities remain separate and application-owned.
- [x] Native stays default/reference/explicit-fallback and no automatic
      cross-runtime or provider fallback is introduced.
- [x] The next implementation plan includes native parity, negative tests,
      rollback, no dependency additions, and no behavior/UI change.
- [x] The owner accepts that Hermes remains separately blocked on immutable
      provenance, whole-process containment, protocol compatibility,
      lifecycle, credentials, packaging, and security evidence. D-080 selects
      `hermes serve`/WebSocket for a contained spike only, not an adapter.
- [x] D-030, D-032, D-033, D-034, D-060 through D-064, D-065, and D-078 remain
      intact unless an additive accepted decision explicitly supersedes them.
- [x] The accepted architecture and conditional transport decisions are recorded
      as D-079 and D-080 in `DECISIONS.md`.

## Acceptance record

The project owner accepted this ADR on 2026-08-11 and selected the native
runtime boundary as the next executable implementation plan. The owner also
kept Hermes execution in a later separately gated phase and prohibited any
automatic progression from native implementation to the contained spike or
adapter.
