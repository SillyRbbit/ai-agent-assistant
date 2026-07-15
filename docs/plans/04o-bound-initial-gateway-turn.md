# Execution plan - Increment 4O bound initial gateway turn

Status: **Complete; verified and uncommitted**

Owner: Project maintainer

Last updated: 2026-07-15

## Goal and outcome

Bind the verified Increment 4N initial request bytes to one correctly configured
`GatewayStreamValidator` so a future trusted Rust transport cannot independently
choose response correlation IDs, allowed function names, or tool-contract version
for that request.

The result is one non-cloneable, transport-free `InitialGatewayTurn`. It owns the
closed request bytes and response-validation state, exposes only borrowed request
bytes plus bounded stream operations, and derives the response tool contract from
the exact local `ToolSchema` catalog. It is not a transport, coordinator,
authorization result, approval, dispatch token, or execution capability.

## User-visible outcome

None. Increment 4O adds no Tauri registration, IPC route, frontend caller, live
model traffic, persistence, credential access, or operating-system interaction.

## Existing behavior and constraints

- Increment 4N fixes run ID, gateway-request ID, `cortexa_desktop_mvp@1`, and all
  conservative limits in one closed initial request body.
- `GatewayStreamValidator::new` separately accepts expected run/request IDs,
  caller-supplied allowed function names, and a caller-supplied tool-contract
  version.
- No production caller exists, but a future caller could construct a valid
  request and a differently configured validator. That would split one logical
  gateway turn across two independently supplied trust inputs.
- The local catalog contains exactly `get_current_datetime@1` and
  `create_local_task@1`. The initial request advertises only the fixed tool-set
  identity, while the response validator must enforce those exact local names
  and their common version.
- The lower-level public validator remains useful for protocol fixtures and
  downstream validation tests. Increment 4O does not redesign it.
- O-006 and O-007 continue to block authenticated transport and live provider
  traffic.

## Exact bound-turn contract

`InitialGatewayTurn::new` accepts only one opaque `run_id`, one opaque
`gateway_request_id`, and one owned user-selected text value. It constructs the
existing closed initial request and an internal `GatewayStreamValidator` from the
same identities. Allowed response function names are derived only from
`ToolSchema::GetCurrentDatetimeV1` and `ToolSchema::CreateLocalTaskV1`; the
expected version is derived from that catalog and construction fails closed if
the catalog versions do not agree with `INITIAL_GATEWAY_TOOL_SET_VERSION`.

The public turn exposes only:

- `request_bytes(&self) -> &[u8]` for a later trusted Rust transport;
- `status(&self) -> GatewayStreamStatus`;
- `accept_frame(&mut self, frame: &[u8]) -> GatewayProtocolResult<ValidatedGatewayEvent>`; and
- `cancel(&mut self) -> bool`.

It does not expose its internal request or validator, does not expose mutable
identity or tool configuration, and does not implement `Clone`, `Serialize`, or
`Deserialize`. Its custom `Debug` output redacts request content and body bytes.
The existing `GatewayProtocolError` remains the frame-validation error. A new
fixed request error variant may report only that internal validator configuration
failed; it must not retain selected content, request bytes, or identities.

`InitialGatewayRequest` becomes private to `gateway_request.rs`, and its raw
constructor and byte accessor cease to be public. `InitialGatewayTurn` becomes
the only public initial request-construction path. `GatewayStreamValidator::new`
remains public for the lower-level protocol boundary and existing tests.

## Exact source and test scope

Change only:

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

The request module owns all implementation and unit-test changes. The public
integration test proves the new crate-boundary contract. Do not change
`gateway_protocol.rs`, `tools/schema.rs`, module exports, manifests, or any other
source/test path. Stop and request approval before expanding this two-file scope.

## Exact implementation closeout scope

In addition to the two source/test paths, closeout may change only:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04o-bound-initial-gateway-turn.md
docs/plans/04o-bound-initial-gateway-turn.md
docs/plans/README.md
docs/reviews/2026-07-15-04o-post-increment-review.md
```

`DECISIONS.md` is limited to a durable record of the bound initial-turn
construction boundary and public API narrowing. No security, product, workflow,
troubleshooting, dependency, manifest, lockfile, Tauri, frontend, storage,
capability, entitlement, or permission file may change without separate approval.

## Implementation steps

1. After approval, run `python3 .codex/hooks/post_increment_gate.py begin --increment 04o` before source edits.
2. Make `InitialGatewayRequest` and its constructor/accessor private, preserving its exact wire behavior and focused tests.
3. Add non-cloneable `InitialGatewayTurn` construction that derives one request and validator from the same IDs and exact local tool catalog.
4. Add only the four bounded public operations, redacted debug output, and a fixed content-free internal-configuration error.
5. Update the public contract test for request bytes, matching/mismatching identities, both allowed tools, unknown/version-mismatched tools, cancellation, and redaction.
6. Run focused and complete verification, review the complete diff and trust boundaries, synchronize closeout documentation, record the durable decision, and finalize the mandatory gate.

## Security and privacy considerations

- Selected user text may appear only in borrowed request bytes. It must not
  appear in `Debug`, errors, logs, audit, fixtures, or validator configuration.
- Request and response correlation identities must be sourced once and remain
  private after construction.
- Allowed function names and version must come from the exact local tool catalog,
  never from a WebView, model, gateway event, or future transport caller.
- A schema-valid function call remains untrusted and non-authorizing; existing
  local schema, policy, approval, audit, and execution boundaries remain required.
- Cancellation is only local terminal validator state. It does not abort a
  network operation because no transport exists.
- Exposing content-bearing request bytes remains a deliberate future transport
  boundary; no logging or persistence is added.

## Risks

- The wrapper could grow into a runtime coordinator. The exact four-method
  surface and two-file scope exclude deadlines, retries, turn counting,
  continuation, policy, and dispatch.
- Making `InitialGatewayRequest` private narrows a public Rust API. The crate is
  not published and repository callers are covered by search and tests, but an
  unsupported external consumer would need to adopt `InitialGatewayTurn`.
- The fixed tool-set identity could drift from the local catalog or deployed
  gateway. Derivation plus tests bind the local names/version; deployment review
  remains mandatory before traffic.
- `GatewayStreamValidator::new` stays public for lower-level use, so arbitrary
  test fixtures can still configure it. Future initial transport code must use
  the bound turn and must not reconstruct a detached validator.
- A future caller could log or retain returned request bytes. Redacted
  debug/errors and the absence of a current caller bound this increment only.

## Explicit non-goals

- HTTP, TLS, streaming transport, gateway service/deployment, endpoint
  configuration, DNS, transport abort, retries, deadlines, or runtime scheduling.
- Gateway authentication, credentials, Keychain, LocalAuthentication, OAuth,
  principal identity, retention approval, or provider/OpenAI SDK parameters.
- Continuation requests, tool-result return, second model turn, retry requests,
  history, memory, context selection, files, attachments, or external services.
- Runtime coordinator, run state machine, policy changes, approval changes,
  audit persistence, dispatch, executor, or real tool execution.
- Tauri commands/events/plugins, WebView integration, UI, SQLite, dependencies,
  CSP, capabilities, entitlements, or permissions.
- Redesigning or privatizing the lower-level `GatewayStreamValidator` constructor.

## Test plan

Preserve the six exact request tests. Extend the public integration contract to
construct only `InitialGatewayTurn`, inspect exact request bytes, accept matching
response frames, reject mismatched run/request IDs, accept each exact local tool
in separate turns, reject an unknown function and wrong contract version, prove
cancellation is terminal and idempotent, and prove debug/errors omit sentinel
selected content and serialized body. Preserve all 18 protocol and nine tool
tests.

No manual verification is required because the increment has no production
caller or user-visible, network, credential, native, persistence, or
operating-system behavior.

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

Then run `$post-increment-gate`, finalize the exact 4O review report, and require
a complete valid marker with `PASS` or `PASS WITH ADVISORIES`.

## Rollback or failure strategy

Before commit, restore the two source/test files to `d7c4b69` and revert only the
declared 4O planning/closeout documentation. After commit, revert the single 4O
commit. No migration, data, dependency, credential, compatibility identifier, or
remote resource requires rollback.

Any need for a third source/test path, dependency, transport, credential,
continuation, coordinator, Tauri route, product/security document, or permission
change stops the increment for project-owner approval.

## Acceptance criteria

- [x] Project owner approves the exact two-file source/test plan and closeout scope.
- [x] Mandatory 04o gate state begins before source edits.
- [x] One public turn constructs request bytes and validator state from the same identities.
- [x] Response function names and version derive only from the exact local tool catalog.
- [x] Raw initial-request construction is no longer public; the lower-level validator remains available.
- [x] Matching frames, mismatches, both tools, wrong tools/version, cancellation, and redaction are covered.
- [x] No transport, credential, continuation, coordinator, Tauri, persistence, execution, dependency, capability, or permission path is added.
- [x] Focused and complete checks and reviews pass.
- [x] Closeout documentation and the durable decision match actual evidence.
- [x] The post-increment report passes and the 04o marker is complete and valid.

## Planning baseline

Passed on clean synchronized `main` at `d7c4b69` before planning edits:

```text
python3 .codex/hooks/post_increment_gate.py status
  Increment 04n complete, valid: true, PASS WITH ADVISORIES
npm run typecheck
  passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
  18 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::
  9 passed
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
  1 passed
rg -n 'InitialGatewayTurn|BoundInitialGateway|PreparedInitialGateway|into_stream_validator|into_turn' src-tauri/src src-tauri/tests
  no existing bound-turn abstraction; required exit status 1
```

Git `HEAD`, local `main`, and `origin/main` all resolved to
`d7c4b69b36f83dfd1fd680b8bbe9ac9fc9aa3c5f`. Toolchains are Node.js `v26.3.0`,
npm `11.16.0`, Cargo and rustc `1.90.0`, rustfmt `1.8.0-stable`, and Clippy
`0.1.90` on arm64 macOS `26.5.2` with Xcode Command Line Tools at
`/Library/Developer/CommandLineTools`.

## Implementation and verification result

Implemented exactly within the two-file source/test scope. The public
`InitialGatewayTurn` privately owns `InitialGatewayRequest` and
`GatewayStreamValidator`, builds both from one pair of opaque IDs, derives the
allowed names and common version from the exact local `ToolSchema` variants, and
fails closed if that version disagrees with `cortexa_desktop_mvp@1`. The raw
request type and constructor are private. Only borrowed request bytes, stream
status, validated frame acceptance, and local cancellation are public.

Passed:

```text
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
  18 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::
  9 passed
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
  6 passed
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
npm run verify
  17 hook, 124 frontend, 92 Rust library, and 17 Rust integration tests passed
  lint, typecheck, Vite builds, and Tauri release no-bundle passed
npm audit --audit-level=low
  network-enabled retry passed with 0 vulnerabilities
git diff --check
```

The first sandboxed gate-begin command could not write ignored state; its approved
elevated retry succeeded before source edits. The first sandboxed npm audit could
not resolve the registry or write npm logs; its approved network-enabled retry
passed. One test helper initially translated request construction failure into an
unrelated protocol error; review corrected the helper to preserve the original
typed error before final verification.

No manual verification is required because no production caller, Tauri route,
WebView behavior, network, credential, native API, persistence, permission, or
operating-system behavior changed. D-036 records the final bound-turn and public
API narrowing decision. The mandatory review result is `PASS WITH ADVISORIES`;
the advisories are the theoretical unsupported external consumer of the private
request API and the intentionally public lower-level validator for protocol
fixtures.
