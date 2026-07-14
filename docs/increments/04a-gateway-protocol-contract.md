# Phase 4 Increment 4A - deterministic gateway protocol contract

Last updated: 2026-07-14

Status: **Verified complete**

## Goal

Establish and fixture-test one versioned, closed, transport-free Rust contract for normalized product-gateway events before any credential, network client, gateway deployment, OpenAI request, IPC, or tool execution is introduced.

## Planning result

- Phase 3 is verified complete, but the Rust `AgentProvider` remains synchronous and has no stream, gateway, cancellation, limit, correlation, redacted-error, or audit-boundary contract.
- Production OpenAI credentials belong only to the authenticated gateway; a future gateway access token has a maximum 15-minute lifetime and stays in Rust memory, while any refresh or session credential belongs to platform secret storage, never the WebView or SQLite.
- The gateway is the only OpenAI adapter. It authenticates and limits requests, selects exact strict tool definitions, forces foreground `store: false` streaming, and converts recognized provider events into a versioned product protocol without gaining local execution authority.
- OpenAI events are typed but externally evolving. The gateway may ignore additive fields on recognized events, while unknown types and invalid required fields fail. The normalized gateway-to-Rust contract rejects unknown fields and variants.
- Function calls require independent gateway and Rust validation. Increment 4A validates protocol shape, identity, JSON object structure, duplicate keys, allowed names, and tool-contract version but deliberately keeps calls non-actionable while local `ToolSchema` is a placeholder.
- Foreground cancellation is an idempotent local terminal transition followed by transport abort propagation, not a gateway event, the background Responses cancel endpoint, or proof that provider computation stopped.
- `store: false` does not remove provider abuse-monitoring retention; O-007 must resolve the OpenAI project retention mode and user disclosure before live traffic.
- Gateway operational telemetry and local trusted action audit are separate, redacted records.
- The exact plan is `docs/plans/04a-gateway-protocol-contract.md`.

## Planning baseline

```text
branch: phase4/planning
base: merged main at f56cab2
working tree before planning: clean
Node.js: v26.3.0
npm: 11.16.0
Cargo: 1.90.0
Rust: 1.90.0
rustfmt: 1.8.0-stable
Clippy: 0.1.90
npm run typecheck: passed
npm run test:unit: passed
frontend tests: 124 passed
Rust library tests: 50 passed
```

Official OpenAI authentication, function-calling, streaming, cancellation, API compatibility/request-ID, and data-control documentation was reviewed on 2026-07-14. The official developer-docs MCP was installed for future Codex sessions; because a restart is required before newly installed MCP tools become callable, this session used the corresponding official OpenAI web documentation.

## Exact planned implementation files

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

Closeout updates are limited to:

```text
CHANGELOG.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04a-gateway-protocol-contract.md
docs/plans/04a-gateway-protocol-contract.md
```

No file outside this list may change without project-owner approval.

## Planned dependency

Add exact direct dependency `serde_json = "=1.0.150"`. The package is already present transitively in `src-tauri/Cargo.lock`; direct use is required to parse and structurally validate untrusted normalized JSON. The implementation must review the complete lockfile diff and add no other package.

## Planned security properties

- Protocol version `1`, exact expected IDs, contiguous sequence starting at zero, exactly one start, and exactly one terminal event.
- Closed normalized text, completed function call, completed, and failed events with unknown-field rejection, plus a local idempotent cancelled state.
- Bounds applied before JSON decoding and during output, argument, event-count, turn, call, retry, and request accounting.
- Duplicate-free JSON object arguments, exact allowed tool name, and exact tool-contract version.
- No risk, permission, policy, approval, execution, provider message, raw body, credential, or authorization field in the protocol.
- Function calls remain untrusted protocol data and cannot become executable proposals in this increment.
- Typed errors expose only closed metadata and never retain raw frames, text, arguments, or provider details.
- No network, gateway, credential, IPC, Tauri, WebView, persistence, capability, CSP, packaging, or permission change.

## Risks

- Provider schema leakage or drift if the normalized contract mirrors raw OpenAI events too closely.
- False trust in provider strict mode or gateway validation.
- Raw content retained in parse errors or debug output.
- Duplicate JSON keys changing function-argument meaning.
- Cancellation language overstating provider-side state.
- Scope expansion from protocol work into transport, credentials, or tool-schema replacement.

Mitigations and exact tests are specified in the execution plan.

## Verification gate

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol
npm run verify
npm audit --audit-level=low
git diff --check
```

The complete diff must also pass `$code-review` and `$security-review`. No native manual interaction gate is planned because the new module is transport-free and not wired to Tauri; the full release no-bundle build remains required.

## Rollback

Delete the new module, remove its module export and direct dependency, and restore the root package dependency entry in `Cargo.lock`. Preserve every verified Phase 3 mock, Rust interface, Tauri, storage, capability, CSP, and permission behavior.

## Approval gate

The project owner approved the exact plan on 2026-07-14. No commit or push was requested or performed.

## Actual implementation

- Added `src-tauri/src/agent/gateway_protocol.rs` with protocol version and conservative limit constants, closed normalized event/failure types, redacted typed errors, and a transactional `GatewayStreamValidator`.
- Frames are size-bounded before `serde_json` decoding. Envelopes and events reject unknown fields and variants, require exact opaque run/request identity, and enforce contiguous sequence and terminal state.
- The validator accepts one started text response or one completed function call. Local cancellation is idempotent, terminal, and rejects every later frame without claiming provider cancellation.
- `UntrustedFunctionCall` has private fields and read-only accessors. It validates allowed name and exact contract version plus bounded, duplicate-free JSON-object arguments, but has no risk, permission, approval, policy, executor, or `ToolCallProposal` conversion.
- Added the exact direct `serde_json = "=1.0.150"` dependency. The lockfile added no package and changed only the root dependency list.
- Added 17 focused inline tests for accepted transitions, every planned rejection class, cancellation, limits, recursive duplicate keys, closed failures, and display/debug redaction.

## Verification result

Passed:

```text
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol
  17 passed; 0 failed; 50 filtered out
npm run verify
  frontend: 10 files, 124 tests passed
  Rust library: 67 passed
  Rust integration: 6 passed
  TypeScript, Vite production build, and Tauri release no-bundle build passed
npm audit --audit-level=low
  0 vulnerabilities
git diff --check
code review
security review
```

The first focused compilation exposed a reversed `matches!` invocation, and the first added overlength-ID test exposed one missing test-module constant import. Both local implementation defects were corrected before the final passing gates. The first sandboxed audit attempt could not resolve the npm registry; the approved network retry passed. No repository troubleshooting entry was needed.

Not run:

- Native manual interaction, because the module is unreferenced by Tauri and changes no UI or native behavior.

## Remaining work

- Replace placeholder `ToolSchema` with an exact locally owned schema contract and validate each untrusted call against it before any policy conversion.
- Add authenticated gateway transport, credential storage, and live provider behavior only through later separately approved increments.
- Resolve O-006 before live gateway networking and O-007 before live provider traffic.
