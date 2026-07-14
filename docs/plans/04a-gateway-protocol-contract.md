# Execution plan - Increment 4A deterministic gateway protocol contract

Last updated: 2026-07-14

Status: **Complete - verified 2026-07-14**

## Phase 4 gap analysis

The verified application proves a complete bounded frontend mock loop, while the Rust foundation contains only a synchronous `AgentProvider::complete` interface. `AgentRequest` carries a run ID, user message, and tool names; `AgentProviderResponse` returns final text or tool proposals. There is no stream contract, gateway authentication boundary, protocol version, event sequencing, cancellation token, deadline, request or output size enforcement, provider-error redaction, correlation identity, or provider audit split. `ToolSchema` is still a placeholder string, so no current Rust path can safely treat model arguments as an executable tool call.

The product brief requires OpenAI Responses through an authenticated gateway, but a live HTTP client would combine too many new trust boundaries at once. The smallest independently verified Phase 4 increment is therefore a transport-free Rust protocol contract and state validator driven only by deterministic JSON fixtures.

| Requirement          | Verified repository state                                                     | Phase 4 gap                                                                                                  |
| -------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| Credential ownership | No production credential or gateway token exists.                             | OpenAI credentials must remain gateway-only; future gateway tokens need a Rust/Keychain boundary.            |
| Responses stream     | The frontend streams fixed timer chunks; Rust provider output is synchronous. | No normalized event envelope, version, sequence, or terminal-state contract exists.                          |
| Function calls       | Rust has tool names, placeholder schemas, and unvalidated argument strings.   | Calls need two-stage gateway/Rust validation and must remain non-actionable until exact local schemas exist. |
| Cancellation         | The frontend mock driver has idempotent timer cancellation.                   | No desktop-gateway/provider transport cancellation or late-event contract exists.                            |
| Limits               | Phase 3 has frontend-only mock limits.                                        | Production-bound request, event, argument, output, event-count, timeout, and network limits are undefined.   |
| Error handling       | Mock failures use fixed frontend copy.                                        | Provider and gateway failures need closed redacted codes and opaque correlation IDs.                         |
| Audit                | Frontend Activity is volatile; Rust audit is an in-memory foundation.         | Gateway operational telemetry and local trusted action audit are not separated.                              |

## Goal and outcome

Add a versioned, closed Rust representation of normalized gateway stream events plus a deterministic state validator that rejects malformed, oversized, mismatched, out-of-order, duplicate, late, or non-actionable function-call data. Prove the boundary with fixtures without creating a network path or changing user-visible behavior.

## Protocol contract

Each normalized JSON frame has exactly these envelope fields:

```text
protocol_version
run_id
gateway_request_id
sequence
event
```

Protocol version `1` starts at sequence `0` and requires contiguous increments. IDs are opaque, ASCII, 1-128 characters, and must match the validator's expected run and gateway request. The event union is closed:

- `response_started`: exactly once and first; carries one bounded opaque provider response ID.
- `output_text_delta`: bounded non-empty text after start and before a terminal event.
- `function_call_completed`: at most once; carries a bounded call ID, registered name, exact tool-contract version, and completed argument JSON string.
- `response_completed`: exactly one successful terminal event.
- `response_failed`: terminal with one closed error code, retryability, and optional bounded retry delay.

For the first contract, a provider turn yields assistant text or one completed function call, never both. Cancellation is a local validator transition rather than a gateway frame. Unknown event variants, fields, enum values, non-contiguous sequences, duplicate starts or terminals, and events after a terminal fail closed.

The normalized protocol is intentionally stricter than the upstream API. The future gateway adapter may ignore additive fields on a recognized OpenAI event because OpenAI documents additional fields as backward-compatible, but it must reject unknown upstream event types or invalid required fields and must never forward raw events.

## Function-call validation boundary

The gateway must eventually:

1. Select a server-owned tool-set version rather than accepting arbitrary OpenAI tool definitions from the desktop.
2. Send only function tools with `strict: true`, every object property required, `additionalProperties: false`, and `parallel_tool_calls: false`.
3. Reject hosted tools, MCP tools, shell tools, unknown functions, unexpected output-item types, argument overflow, malformed JSON, and incomplete calls.
4. Emit only a completed normalized function-call event.

Increment 4A then validates the normalized frame again in Rust:

- The name must be in the validator's exact allowed-name set.
- The tool-contract version must match the expected version.
- Arguments must fit the byte limit and decode as exactly one JSON object with no duplicate keys.
- No risk, permission, approval, or execution field is accepted from the gateway.
- The result remains an untrusted protocol value and is never converted into `ToolCallProposal`.

Exact per-tool JSON Schema validation is not possible while `ToolSchema` remains a placeholder. A later approved prerequisite must replace that placeholder and revalidate arguments before policy. Until then, every received function call is non-actionable.

## Credential and gateway responsibilities

- The OpenAI API credential exists only in the gateway's server-side secret store and authorization header.
- A future desktop gateway access token is audience-bound, has a maximum 15-minute lifetime, and is held in Rust memory. Any refresh or session credential is read only through platform secret storage and stored in Keychain rather than SQLite or the WebView.
- The WebView cannot supply a gateway origin, model, provider parameter, authorization header, tool definition, or tool-set version.
- The gateway authenticates and authorizes the desktop principal; enforces app/protocol/model/tool-set allowlists, quotas, and size/time/rate limits; selects exact tool definitions; forces `stream: true`, `store: false`, and `background: false`; injects the provider credential; captures provider request IDs for restricted operations; and normalizes the stream.
- The gateway cannot authorize, approve, execute, or claim the result of a local tool.

Gateway identity provider and deployment platform selection remains O-006 and does not block Increment 4A because this increment adds no transport or credential.

`store: false` minimizes Responses application-state storage but does not eliminate OpenAI's default abuse-monitoring retention. O-007 must select and verify the OpenAI project retention mode and user disclosure before live provider traffic. Increment 4A sends no data externally and does not require that deployment decision.

## Cancellation and deadlines

The first live integration will use foreground streaming. OpenAI's Responses cancel endpoint applies only to background responses, while background mode stores response data for polling. Cancellation therefore uses transport propagation:

1. Rust marks the run cancelling and idempotently aborts its gateway request.
2. The gateway observes disconnect/cancellation, aborts its upstream foreground stream, and stops normalization.
3. Rust discards every late frame and records a local cancelled outcome.

Increment 4A exposes an idempotent local `cancel` transition and tests that all later frames fail. It does not define a cancellation frame, asynchronous task, or network handle, and it does not claim that the provider stopped computation.

## Conservative limits

Freeze the following protocol constants in Increment 4A:

- Protocol version: `1`.
- Maximum model turns per run: `2`.
- Maximum function calls per run: `1`; parallel calls prohibited.
- Maximum retry attempts: `1`, only when the gateway proves it did not forward the request upstream or returns an explicit rate-limit response before `response_started`. Timeouts and unknown delivery outcomes are not retried.
- Maximum gateway requests per run: `3`.
- Maximum serialized gateway request: `65,536` bytes.
- Maximum normalized event frame: `16,384` bytes, enforced before JSON decoding.
- Maximum completed function arguments: `8,192` bytes.
- Maximum assistant output per turn: `8,192` Unicode code points.
- Maximum normalized events per turn: `256`.
- Future transport limits: 10-second connection timeout, 20-second idle timeout, 60-second provider-turn deadline, and 120-second run deadline.

Increment 4A enforces the event, argument, output, event-count, turn, and tool-call portions that can be exercised without transport. Future gateway and Rust clients must independently enforce applicable request, network, retry, and deadline limits.

## Error redaction

The normalized error codes are closed to `unauthenticated`, `forbidden`, `rate_limited`, `request_rejected`, `provider_unavailable`, `provider_timeout`, `protocol_violation`, `limit_exceeded`, `cancelled`, and `internal`.

Only code, retryability, optional retry delay capped at 60 seconds, and opaque gateway request identity may cross to Rust. Human provider messages, raw bodies, headers, URLs, stack traces, prompts, output, function arguments, and credentials are structurally absent. Rust maps codes to fixed user-facing copy later; it never renders a provider message.

## Audit boundary

Gateway operational records may contain an opaque principal, gateway and provider request IDs, app/protocol/tool-set/model versions, timestamps, authentication outcome, status/error code, latency, rate-limit metadata, and aggregate token usage. They exclude prompt, output, arguments, results, authorization headers, and credentials by default and have a maximum seven-day retention unless a later compliance decision approves another period.

The local trusted audit owns run lifecycle, normalized proposal identity, locally derived tool/risk/permission metadata, deterministic policy, approval, restricted execution, cancellation, and final outcome. It excludes raw prompts, output, arguments, results, and errors unless a later field-specific decision proves the data is necessary and redacted. Frontend Activity remains presentation-only.

## Explicit non-goals

- HTTP, SSE, WebSocket, or OpenAI SDK code.
- A gateway server, deployment, domain, identity provider, cloud account, model selection, or production API call.
- API keys, gateway tokens, OAuth, Keychain access, secret-store implementation, or authorization headers.
- Changes to `AgentProvider`, `AgentRequest`, `AgentProviderResponse`, `ToolSchema`, the tool registry, policy, approvals, audit storage, or executor behavior.
- Converting normalized calls into `ToolCallProposal` or executing any function.
- Provider continuation, tool-result submission, persistence, context collection, attachments, voice, or user-visible UI.
- Tauri commands/events, WebView IPC, capabilities, CSP, plugins, packaging, or operating-system permissions.
- Background Responses, server-side response storage, OpenAI hosted tools, MCP tools, shell tools, parallel calls, or arbitrary provider parameters.

## Exact implementation files

Create:

```text
src-tauri/src/agent/gateway_protocol.rs
```

Change:

```text
src-tauri/src/agent/mod.rs
src-tauri/Cargo.toml
src-tauri/Cargo.lock
```

`serde_json = "=1.0.150"` becomes a direct production dependency. It is already present transitively in the lockfile, but direct parsing of untrusted normalized JSON requires an explicit pinned dependency. Focused tests remain inside `gateway_protocol.rs`; no fixture file is needed because fixed inline JSON keeps the input corpus reviewable beside the validator.

Update only during implementation closeout:

```text
CHANGELOG.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04a-gateway-protocol-contract.md
docs/plans/04a-gateway-protocol-contract.md
```

No file outside this list may change without stopping for project-owner approval.

## Implementation steps

1. Add the exact direct `serde_json` dependency and regenerate only the root dependency entry in `Cargo.lock`.
2. Define immutable limits, protocol IDs, the closed event/error enums, validated outputs, and typed errors.
3. Reject oversized frames before JSON decoding and deserialize normalized envelopes with unknown-field rejection.
4. Implement the deterministic per-turn validator for identity, contiguous sequence, start/terminal state, one text-or-call outcome, output/event/tool-call limits, and late-event rejection.
5. Implement duplicate-key-aware JSON-object validation for completed function arguments and keep the result explicitly untrusted and non-actionable.
6. Add focused table-driven tests for every accepted transition and rejection class, including proof that typed errors and debug output contain no raw frame, text delta, arguments, or provider detail.
7. Run focused Rust checks, the complete repository gate, dependency audit, diff checks, code review, and security review.
8. Synchronize only the listed closeout documents with actual evidence; manual native behavior remains unchanged because the module is not wired to Tauri.

## Risks and mitigations

- **The normalized protocol is mistaken for the OpenAI schema:** use product-owned event names and document the gateway adapter as the only upstream translator.
- **Strict parsing breaks on additive OpenAI fields:** apply closed parsing only to the normalized product protocol; the future upstream adapter ignores additive fields on recognized event types and strips them.
- **Provider strict mode is treated as authorization:** keep every function call untrusted and non-actionable until independent local schema, policy, and approval checks pass.
- **Raw content leaks through errors:** typed errors carry only enum/limit/sequence metadata and tests assert raw frames, deltas, and arguments are absent from display/debug output.
- **Duplicate JSON keys change argument meaning:** use a duplicate-aware object parser and reject before any schema or policy work.
- **Cancellation overclaims provider state:** distinguish local transport cancellation from provider-side cancellation in event and audit semantics.
- **A parsing dependency expands supply-chain risk:** pin the already-locked `serde_json 1.0.150`, add no new transitive package, run audit, and review the lock diff.
- **The increment drifts into transport:** prohibit HTTP/async/Tauri/provider changes and stop before editing any file outside the exact list.

## Implementation and verification result

The project owner approved the exact plan on 2026-07-14. The implementation added only the transport-free Rust protocol module, its module export, and the exact direct `serde_json = "=1.0.150"` dependency. The lockfile changed only by adding the already-locked package to the root crate dependency list.

The validator bounds frames before decoding, rejects unknown fields and variants, enforces expected identity and contiguous sequence, accepts one start followed by text or one completed function call and one terminal event, and makes local cancellation idempotent and terminal. Function calls remain private-field `UntrustedFunctionCall` values with no `ToolCallProposal` conversion or executor path. Argument parsing rejects malformed, non-object, oversized, and recursively duplicate-key JSON. Typed errors retain only closed status, sequence, and limit metadata.

Seventeen focused protocol tests pass. Final `npm run verify` passes with 124 frontend tests, 67 Rust library tests, six Rust integration tests, production frontend builds, and the Tauri release no-bundle build. `npm audit --audit-level=low` reports zero vulnerabilities, `git diff --check` passes, and code/security review found no defects or scope expansion. No native manual interaction check was required because the module is not wired to Tauri.

## Test plan

- Accept one started/text-delta/completed sequence and one started/function-call/completed sequence.
- Reject protocol version, run ID, gateway request ID, and provider response ID violations.
- Reject unknown fields, unknown event variants, invalid enums, malformed JSON, non-ASCII or oversized IDs, and oversized frames before decoding.
- Reject missing start, duplicate start, sequence gaps/duplicates, empty deltas, text after a call, a call after text, a second call, duplicate terminal, and any frame after local cancellation or another terminal state.
- Reject too many events, too much accumulated output, oversized arguments, non-object arguments, malformed arguments, and duplicate argument keys.
- Reject unknown tool names and mismatched tool-contract versions.
- Prove a parsed call has no risk, permission, approval, execution, or `ToolCallProposal` conversion.
- Prove closed errors accept only bounded retry metadata and cannot contain raw provider messages.
- Prove error display/debug output excludes frame content, text deltas, function arguments, and credentials.
- Preserve all existing 124 frontend and 50 Rust library tests, six Rust integration tests, and production builds.

## Verification commands

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol
npm run verify
npm audit --audit-level=low
git diff --check
git diff -- src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/src/agent/mod.rs src-tauri/src/agent/gateway_protocol.rs
```

Review gates:

- Run `$code-review` against the complete diff.
- Run `$security-review` because the change defines a future credential, provider, tool-call, error, and audit boundary.
- Confirm no Tauri command, invoke handler, capability, CSP, network client, URL, credential, database, permission, generated file, or build output was added.
- No native UI interaction check is required because Increment 4A is an unreferenced portable Rust module; `npm run verify` still proves the native release build.

## Rollback

Remove `gateway_protocol.rs`, remove its module export, remove the direct `serde_json` dependency, and regenerate the lockfile so the root package dependency list returns to the verified Phase 3 state. Do not alter the existing mock provider, frontend mock loop, tool registry, policy, approval, audit, storage, Tauri configuration, capabilities, or CSP during rollback.

## Acceptance criteria

- [x] The exact protocol version, closed event/error unions, and conservative limits are represented in portable Rust.
- [x] Every untrusted frame is bounded before decoding and validated against expected identity, sequence, state, and terminal rules.
- [x] Completed function arguments are bounded, duplicate-free JSON objects tied to one allowed tool name and contract version.
- [x] Function calls remain explicitly non-actionable and cannot become a `ToolCallProposal`.
- [x] Errors are closed and redacted by construction; tests prove raw provider/frame content is absent.
- [x] Cancellation semantics distinguish transport abort from confirmed provider cancellation.
- [x] No network, credential, gateway, IPC, WebView, tool execution, persistence, capability, CSP, packaging, or permission path is added.
- [x] Only the exact approved implementation and closeout files changed after approval; the pre-existing approved Phase 4 planning-document diff was preserved.
- [x] Focused tests, `npm run verify`, dependency audit, diff checks, code review, and security review pass.
- [x] Closeout documentation records actual evidence, remaining work, risks, and the next exact task.

## Approval gate

The project owner approved this exact goal, file list, dependency, limits, validation semantics, verification gate, and rollback on 2026-07-14. Increment 4A is verified complete. Later tool-schema, transport, credential, gateway, or provider work still requires a separate approved increment.

## Official OpenAI references reviewed

- [API authentication](https://developers.openai.com/api/reference/overview#authentication): API credentials must stay out of client-side apps and be loaded from server-side environment or key management.
- [Function calling](https://developers.openai.com/api/docs/guides/function-calling#strict-mode): strict mode requirements, required properties, `additionalProperties: false`, and disabling parallel function calls.
- [Streaming Responses](https://developers.openai.com/api/docs/guides/streaming-responses): typed semantic stream events and common lifecycle events.
- [Responses cancellation](https://developers.openai.com/api/reference/resources/responses/methods/cancel): the cancel endpoint applies only to background responses.
- [API overview](https://developers.openai.com/api/reference/overview#backwards-compatibility): additive response properties and stream event types are backward-compatible changes; request IDs support operations and troubleshooting.
- [Data controls](https://developers.openai.com/api/docs/guides/your-data#default-usage-policies-by-endpoint): Responses storage defaults, `store: false` implications, and background-mode disk storage.
