# ADR: Hermes ACP transport for contained evaluation

Status: Rejected after isolated spike
Date: 2026-08-11
Decision owners: Project owner
Related accepted decisions: D-078, D-079, D-080, D-081
Related evidence:
[`HERMES_TRANSPORT_SPIKE.md`](../spikes/HERMES_TRANSPORT_SPIKE.md) and
[`HERMES_SERVE_WEBSOCKET_SPIKE.md`](../spikes/HERMES_SERVE_WEBSOCKET_SPIKE.md)

This ADR evaluated ACP as a separate candidate transport after the raw
TUI-gateway stdio and managed `hermes serve` WebSocket mechanisms were rejected
for the pinned Hermes release. Its initial Proposed status authorized no
production adapter, dependency, Hermes execution, provider, credential,
process, or application behavior. The isolated spike returned NO GO and this
ADR was not accepted.

## Context

The evaluated upstream is exactly Hermes Agent package/application version
`0.20.0`, release tag `v2026.8.3`, source commit
`3c27eb6234bf91b8ceee9e9071591b31e9b148cb`. Mutable `main` and `latest` are
not compatibility evidence.

Hermes documents `hermes acp`, `hermes-acp`, and `python -m acp_adapter` as
public ACP launch forms. The pinned adapter wraps the synchronous Hermes
`AIAgent` in an asynchronous newline-delimited JSON-RPC stdio server. It
reserves stdout for ACP and routes human-readable logging to stderr. The
initialization handshake reports ACP protocol version, Hermes version,
capabilities, and authentication methods; sessions expose prompt, streamed
updates, cancellation, load/resume/fork/list, model configuration, and working
directory binding.

Structured framing and a public launcher make ACP materially different from
the rejected internal raw TUI-gateway stdio launcher. They do not establish
conversation-only execution or application-owned governance.

## Evaluated direction

Evaluate whether a future narrow Cortexa adapter can supervise the pinned ACP
process and project only initialization, one session, bounded text updates,
terminal completion, cancellation, and process shutdown into application-owned
types.

The isolated spike must use fixtures first. It may execute the operator-supplied
candidate only after exact provenance, required imports, process controls, and
capability containment pass. It may not install, update, repair, or mutate the
candidate and may not use the owner's normal Hermes profile or credentials.

## Required properties

ACP is acceptable only if all of the following are demonstrated:

- exact pinned version and dependency identity;
- bounded newline-delimited JSON-RPC framing with negotiated compatibility;
- protocol-only stdout and bounded redacted stderr;
- deterministic session, prompt, update, completion, failure, cancellation,
  EOF, crash, and shutdown behavior;
- a fresh owner-only Hermes home and working directory with an allowlisted
  environment and no normal profile, dotenv, sessions, memory, skills,
  plugins, or MCP configuration;
- no inherited credential or implicit provider use;
- no filesystem, shell, process, browser, network, Git, Keychain, package
  manager, updater, memory, skill, MCP, subagent, scheduler, or messaging
  authority outside the approved isolated fixture; and
- a load-bearing boundary that prevents Hermes from executing a tool before
  Cortexa's deterministic validation, policy, exact approval, restricted
  execution, cancellation, and audit gates.

Host-side rejection of a tool notification is not containment if Hermes has
already executed that tool internally. An ACP permission request is not a
substitute for Cortexa's application-owned approval binding.

## Comparison with rejected candidates

### Raw TUI-gateway stdio

Raw TUI-gateway stdio at the pinned release lacks a supported public raw server
launcher, initial version/capability negotiation, and a gateway shutdown RPC.
ACP has a public launcher, a structured initialization handshake, a reported
protocol version, standardized session methods, and structured update events.
ACP is therefore a distinct protocol and not a relabeling of the rejected raw
gateway mechanism.

### Managed `hermes serve` WebSocket

The managed WebSocket path adds a broad HTTP server, authenticated loopback
listener, dynamic port, WebSocket token, maintenance tasks, and dynamic network
containment. ACP avoids the listener and token by using a direct owned stdio
child. It still embeds the same broad Hermes runtime, configuration, provider,
state, tool, and subprocess authority, so eliminating the socket does not
eliminate the primary runtime-governance risk.

### OpenAI-compatible HTTP

The OpenAI-compatible surface is provider-shaped rather than a runtime control
protocol. It blurs the separately governed provider boundary, does not provide
the required runtime lifecycle contract, and does not remove the broad Hermes
process or tool authority. It is not selected by this evaluation.

## Security and portability questions

The spike must resolve:

- whether ACP session construction can truthfully select zero Hermes tools;
- whether tool calls are proposed before execution or are executed inside
  Hermes and merely reported afterward;
- whether all dotenv, managed-secret, credential, MCP, memory, skill, plugin,
  state, and update behavior can be disabled through supported controls;
- whether cancellation stops active model and child work with one terminal
  result;
- whether EOF or signal shutdown reaps the whole process tree, including
  detached descendants;
- whether the exact Python runtime and optional ACP dependency can be packaged
  immutably across target platforms; and
- whether a Rust supervisor can implement the closed projection without
  importing Hermes or generic JSON-RPC types into application/domain APIs.

## Evaluation findings

- **Protocol stability:** the public launcher and structured JSON-RPC framing
  are strengths, but the pinned adapter opts into the ACP SDK's unstable
  protocol and does not reject an incompatible client version during
  initialization.
- **Process isolation and crash containment:** stdio avoids a listener, but EOF,
  cooperative cancellation, and direct-child termination do not prove cleanup
  of tool processes or detached descendants. No ACP-specific containment-wide
  shutdown contract was found.
- **Framing and logs:** stdout is intentionally protocol-only and logging is
  directed to stderr. Pinned logging can include prompt prefixes and raw
  exception information, so stderr still requires strict bounds and redaction.
- **Session semantics:** initialization, session creation, prompt submission,
  structured updates, terminal results, and cancellation exist. There is no
  separate ACP health method or host-enforced prompt timeout.
- **Configuration, secrets, and filesystem behavior:** normal startup shares
  Hermes dotenv, managed-secret, provider, state, memory, skill, plugin, MCP,
  and working-directory behavior. A temporary home and sanitized environment
  are useful test controls, not an authority boundary.
- **Tools and children:** the hardcoded broad internal toolset is decisive. Its
  effects and children are Hermes-owned rather than projected as inert
  proposals for Cortexa to validate and execute.
- **Portability and packaging:** an adapter would need a complete immutable
  Python runtime, the exact optional ACP SDK, platform-specific process
  containment, and detached-descendant cleanup evidence. The supplied
  candidate satisfies none of those complete packaging requirements.
- **Rust integration complexity and testability:** a narrow Rust stdio client
  is mechanically testable with deterministic fixtures, but fixture success
  cannot prove the real runtime's authority, packaging, or cleanup properties.
- **Future publication:** publishing a Cortexa integration that relies on
  Hermes-owned privileged effects would weaken the documented trust model and
  create platform-dependent packaging and disclosure obligations. This release
  is therefore not an acceptable basis for a distributable adapter.

## Decision result

The spike returned NO GO. The public structured transport is materially better
than raw TUI-gateway stdio, but the pinned ACP session hardcodes the broad
`hermes-acp` toolset inside Hermes. No supported conversation-only/zero-tool ACP
mode or application-owned pre-execution gate for every effect exists. The
supplied candidate also lacks complete immutable runtime provenance and the
installed `agent-client-protocol==0.9.0` dependency. See
[`HERMES_ACP_TRANSPORT_SPIKE.md`](../spikes/HERMES_ACP_TRANSPORT_SPIKE.md).

Reject ACP for the exact evaluated release. Do not implement
`HermesAgentRuntime`, auto-select another transport, or reinterpret structured
tool-progress/permission messages as Cortexa-owned authorization.

## Consequences

- Native remains sole/default and unchanged.
- `HermesAgentRuntime` remains Draft/Blocked.
- No ACP dependency, process, selector, UI, provider, credential, or production
  source is authorized.
- Raw TUI-gateway stdio and managed `hermes serve` WebSocket remain rejected by
  their own evidence.
- No later transport is selected automatically.

## Rollback

This ADR is documentation only. Reverting it and its associated spike evidence
returns ACP to unselected while preserving the accepted native architecture and
both earlier transport NO-GO records.
