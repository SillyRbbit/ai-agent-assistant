# Execution plan - Increment 4N bounded initial gateway request

Status: **Complete; verified and uncommitted**

Owner: Project maintainer

Last updated: 2026-07-15

## Goal and outcome

Add one transport-free Rust contract that constructs the first bounded desktop-to-gateway request described by D-021. The contract carries only fixed protocol identity, opaque run and request identity, one user-selected text input, one fixed server-recognized tool-set identity, and the already approved conservative limits.

The output is product-protocol bytes suitable for a later authenticated HTTPS adapter. It is not a network client, provider request, authentication credential, runtime coordinator, policy result, approval, audit record, dispatch token, or execution capability.

## User-visible outcome

None. Increment 4N has no Tauri registration, IPC route, frontend caller, model traffic, gateway deployment, credential access, persistence, or operating-system interaction.

## Existing behavior and constraints

- Increment 4A validates a closed normalized gateway-to-Rust event stream but defines no desktop-to-gateway request envelope.
- D-021 requires a closed request with protocol version, opaque run/correlation identity, bounded user-selected content, one server-recognized tool-set identifier, and fixed limits.
- Existing gateway protocol constants are the authoritative Phase 4 protocol, identity, request-size, loop, frame, output, retry, and deadline values.
- The existing opaque-ID validator is private to `gateway_protocol.rs`; duplicating it would allow request and response identity rules to drift.
- The local catalog contains exactly `get_current_datetime@1` and `create_local_task@1`. The desktop must send only a fixed tool-set identity, never schemas or caller-selected tools.
- No authenticated gateway transport, identity provider, Keychain abstraction, production model, provider retention approval, continuation contract, runtime coordinator, or trusted context selector exists.
- O-006 and O-007 block live traffic but do not block a transport-free local request contract.

## Exact request contract

`InitialGatewayRequest::new` accepts only one opaque `run_id`, one opaque `gateway_request_id`, and one owned user-selected text value for the initial turn. It reuses existing opaque-ID validation, rejects content with no non-whitespace character, performs an early byte-bound check, serializes through private `serde::Serialize` wire structs, and rejects any final JSON body larger than `MAX_GATEWAY_REQUEST_BYTES` after escaping.

The closed serialized object contains exactly:

- `protocol_version`, fixed to `GATEWAY_PROTOCOL_VERSION`;
- `run_id` and `gateway_request_id`;
- `request_kind`, fixed to `initial_user_turn`;
- `model_turn`, fixed to `1`;
- `retry_attempt`, fixed to `0`;
- `input`, with fixed type `user_selected_text` and the exact selected text;
- `tool_set`, with fixed identifier `cortexa_desktop_mvp` and version `1`; and
- `limits`, populated only from the existing Phase 4 request, turn, event, function-call, argument, output, retry, and deadline constants.

The request contains no gateway URL, provider or model name, authorization value, credential, principal identity, OpenAI parameter, tool definition, JSON schema, hosted tool, MCP server, shell tool, local permission, policy outcome, approval state, audit state, or execution field. The gateway remains responsible for authentication, authorization, strict server-side definitions, and forced Responses settings including foreground streaming, `store: false`, `background: false`, and disabled parallel tool calls.

The public value is non-cloneable and does not implement `Serialize` or `Deserialize`. Its custom `Debug` output redacts selected text and serialized bytes. It exposes borrowed request bytes only for a later trusted Rust transport. Typed errors contain fixed variants and numeric limits only, never selected content or serialized bytes.

## Exact source and test scope

Create:

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

Change:

```text
src-tauri/src/agent/gateway_protocol.rs
src-tauri/src/agent/mod.rs
```

The `gateway_protocol.rs` change is limited to making `is_valid_opaque_id` visible to its sibling request module. The `mod.rs` change adds only the new module export. Stop and request approval before changing any other source or test file.

## Exact implementation closeout scope

In addition to the four source/test paths, closeout may change only:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04n-bounded-initial-gateway-request.md
docs/plans/04n-bounded-initial-gateway-request.md
docs/plans/README.md
docs/reviews/2026-07-15-04n-post-increment-review.md
```

`DECISIONS.md` is limited to proposed D-035 recording the fixed initial request boundary. No product, security, workflow, troubleshooting, dependency, manifest, lockfile, Tauri, frontend, storage, capability, entitlement, or permission file may change without separate approval.

## Implementation steps

1. After approval, run `python3 .codex/hooks/post_increment_gate.py begin --increment 04n` before source edits.
2. Add the private wire representation, public non-cloneable request, fixed constructor, byte accessor, redacted debug output, and closed typed errors.
3. Reuse the existing opaque-ID predicate with sibling-only visibility and export the new module.
4. Add unit coverage for exact JSON, fixed constants, identity/content rejection, escaped-size enforcement, non-ASCII preservation, and redaction.
5. Add one public integration test proving only the closed initial request can be constructed through the crate boundary.
6. Run focused and complete verification, review the exact diff and trust boundaries, synchronize closeout documentation, append D-035, and finalize the mandatory gate.

## Security and privacy considerations

- Selected user text may appear only in returned request bytes. It must not appear in `Debug`, errors, logs, audit records, tests, or documentation fixtures.
- The final serialized-byte limit must be enforced after escaping; checking only source text length is insufficient.
- IDs reuse the response protocol's ASCII-graphic, non-empty, 128-byte rule so both directions cannot drift.
- The fixed tool-set identity is gateway authorization input, not local execution authority, and cannot be supplied by the WebView or model.
- No public general serializer or arbitrary field map may permit provider parameters, credentials, schemas, or tools.
- The type remains disconnected from Tauri and networking. O-006 and O-007 continue to block authenticated transport and live provider traffic.

## Risks

- A future caller could log or misroute content-bearing request bytes. Redacted debug/errors, no current caller, and no transport wiring bound this increment.
- JSON escaping can expand content beyond the request limit. Exact post-serialization enforcement and boundary tests are mandatory.
- Request and response identity rules could drift. Reusing the sibling-only validator avoids duplicate policy.
- The fixed tool-set identity could diverge from a future gateway. A deployment increment must verify exact server-side agreement before traffic.
- Scope could expand into authentication, transport, continuation, context selection, or provider-specific fields. Those require separate approval.

## Explicit non-goals

- HTTP, TLS, streaming transport, gateway service/deployment, endpoint configuration, DNS, retry execution, or cancellation orchestration.
- OpenAI SDK/types, direct Responses requests, model selection, provider parameters, raw provider events, or provider credentials.
- Gateway tokens, refresh/session credentials, Keychain, LocalAuthentication, principal identity, OAuth, or retention approval.
- Continuation requests, tool-result return, second model turn, retry requests, history, memory, device context, attachments, files, or external-service context.
- Runtime coordinator, run state machine, policy changes, approval changes, audit persistence, dispatch, executor, or real tools.
- Tauri commands/events/plugins, WebView integration, UI, SQLite, dependencies, capabilities, CSP, entitlements, or permissions.

## Test plan

Focused tests must prove exact field names and fixed values; deterministic JSON; all limits match existing constants; valid IDs and non-ASCII/multiline content are preserved; invalid IDs plus empty, whitespace-only, and oversized content fail closed; final size is checked after escaping; debug and errors omit sentinel content; forbidden provider, credential, schema, authority, and execution fields are absent; and the existing 18 gateway plus nine tool-catalog tests remain passing.

No manual verification is required because the increment has no production caller or user-visible, network, credential, native, or persistence behavior.

## Verification commands

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
npm run verify
npm audit --audit-level=low
git diff --check
python3 .codex/hooks/post_increment_gate.py status
```

Then run `$post-increment-gate`, finalize the exact 4N review report, and require a complete valid marker with `PASS` or `PASS WITH ADVISORIES`.

## Rollback or failure strategy

Before commit, delete the two new files, remove only the new module export, and restore `is_valid_opaque_id` to private visibility. Revert only declared 4N documentation. After commit, revert the single 4N commit. No migration, data, credential, remote resource, dependency, or compatibility identifier requires rollback.

Any need for a fifth source/test path, dependency, product/security change, live endpoint, credential, Tauri route, continuation request, or runtime caller stops the increment for project-owner approval.

## Acceptance criteria

- [x] Project owner approves the exact four-file source/test plan and closeout scope.
- [x] Mandatory 04n gate state begins before source edits.
- [x] The exact closed initial request serializes deterministically within 64 KiB.
- [x] IDs, selected content, tool-set identity, and all existing limits are validated as documented.
- [x] Debug and errors expose no selected content or serialized body.
- [x] No transport, credential, provider-specific, continuation, runtime, Tauri, persistence, execution, dependency, capability, or permission path is added.
- [x] Focused and complete checks and reviews pass.
- [x] Closeout documentation and D-035 match actual evidence.
- [x] The post-increment report passes and the 04n marker is complete and valid.

## Planning baseline

Passed on clean synchronized `main` at `1f03d1e` before planning edits:

```text
python3 .codex/hooks/post_increment_gate.py status
  Increment 04m complete, valid: true, PASS WITH ADVISORIES
npm run typecheck
  passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
  18 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::
  9 passed
rg -n 'struct (Initial)?GatewayRequest|enum (Initial)?GatewayRequest|mod gateway_request|TOOL_SET_ID|tool_set_id' src-tauri/src src-tauri/tests
  no existing request envelope; required exit status 1
```

Toolchains are Node.js `v26.3.0`, npm `11.16.0`, Cargo and rustc `1.90.0`, rustfmt `1.8.0-stable`, and Clippy `0.1.90` on arm64 macOS `26.5.2` with Xcode Command Line Tools at `/Library/Developer/CommandLineTools`.

## Actual results

Implemented exactly within the four-file source/test scope. `InitialGatewayRequest`
uses private wire structs, shared identity validation, fixed initial-turn and
tool-set values, every existing conservative limit, final post-escaping size
validation, content-redacted debug output, and closed typed errors. Six focused
unit tests and one public integration test cover the complete planned matrix.

Focused rustfmt, request, gateway, tool, integration, and Clippy checks pass.
Complete `npm run verify` passes with 17 hook, 124 frontend, 92 Rust library,
and 12 Rust integration tests plus lint, typecheck, frontend builds, and Tauri
release no-bundle. The network-enabled npm audit retry reports zero
vulnerabilities. Exact scope, conflict, secret, generated-output, architecture,
code-health, security, complete-diff, and documentation reviews have no blocking
finding. No manual verification is required.

D-035 records the closed request boundary. The consolidated result is `PASS WITH
ADVISORIES`; the content-bearing byte accessor has no runtime caller, and the
fixed tool-set identity requires future deployed-gateway agreement before live
traffic. Both are intentional and block neither completion nor later bounded
planning.

## Documentation updates

All declared closeout documents are synchronized. No troubleshooting entry is
required because implementation failures were test-code corrections rather than
environment or product defects. No product or security document changed because
D-021, the architecture baseline, and `SECURITY.md` already define this boundary.
