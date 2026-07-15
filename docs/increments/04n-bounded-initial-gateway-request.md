# Phase 4 Increment 4N - Bounded initial gateway request

Last updated: 2026-07-15

Status: **Verified complete; uncommitted**

## Goal

Define one transport-free, content-bounded Rust request envelope for the first desktop-to-gateway model turn without adding networking, authentication, credentials, provider-specific parameters, runtime orchestration, IPC, persistence, or execution authority.

## Planning result

- Git is clean and synchronized on `main` at `1f03d1e`; Increment 4M is committed, pushed, fast-forward merged, and its completion marker remains valid.
- The normalized gateway response protocol has 18 passing focused tests and owns the authoritative Phase 4 constants.
- The exact local two-tool catalog has nine passing focused tests.
- Repository search finds no desktop-to-gateway request envelope, constructor, tool-set identifier, authenticated transport, or runtime caller.
- D-021 requires a closed request contract before live transport. O-006 and O-007 block live traffic but not this transport-free value.
- An initial-turn-only request is smaller than selecting a gateway platform, adding credentials, designing continuation, or introducing a runtime coordinator.
- The valid 04m marker was confirmed before planning edits. At that planning checkpoint, the nine documentation changes made its workspace fingerprint stale as expected and no 04n gate state had begun.

## Proposed source and test scope

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

The protocol change is limited to sibling visibility for `is_valid_opaque_id`; the module index adds one export. Exact behavior, risks, non-goals, verification, closeout, and rollback are in [`docs/plans/04n-bounded-initial-gateway-request.md`](../plans/04n-bounded-initial-gateway-request.md).

## Planning baseline

Passed before documentation edits:

```text
git status --short --branch
  clean main tracking origin/main
git rev-parse HEAD main origin/main
  all 1f03d1edfaef8d18d8d73a176f259f11aab5e945
python3 .codex/hooks/post_increment_gate.py status
  Increment 04m complete, valid: true, PASS WITH ADVISORIES
npm run typecheck
  passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
  18 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::
  9 passed
```

Toolchains are Node.js `v26.3.0`, npm `11.16.0`, Cargo and rustc `1.90.0`, rustfmt `1.8.0-stable`, and Clippy `0.1.90` on arm64 macOS `26.5.2` with Xcode Command Line Tools at `/Library/Developer/CommandLineTools`.

## Planning file scope

Create:

```text
docs/increments/04n-bounded-initial-gateway-request.md
docs/plans/04n-bounded-initial-gateway-request.md
```

Change:

```text
AGENTS.md
CHANGELOG.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/plans/README.md
```

No source, test, security, product, decision, troubleshooting, workflow, dependency, manifest, lockfile, Tauri, frontend, storage, capability, entitlement, or permission file changes during planning.

## Risks

- Selected content must appear in request bytes but nowhere in debug output, errors, logs, audit, or fixtures.
- JSON escaping can make a body exceed 64 KiB after a naive input-length check.
- Request and response identity validation can drift if duplicated.
- A future gateway may use a different tool-set identifier; no live transport may rely on this value before deployment review.
- Authentication, continuation, context selection, or runtime work could expand the increment beyond an independently verifiable contract.

## Non-goals

Networking, gateway service/deployment, provider SDKs, model selection, credentials, Keychain, identity, retention approval, retries, cancellation orchestration, continuation, tool results, context selection, coordinator, policy, approval, audit persistence, dispatch, execution, Tauri, WebView, SQLite, dependencies, capabilities, entitlements, and permissions are excluded.

## Completion gates

- [x] Required repository memory, workflow, product, architecture, security, decisions, troubleshooting, 4M, and actual source state reconciled.
- [x] Clean synchronized Git baseline, valid 04m marker, toolchains, caller scan, and focused checks recorded.
- [x] Exact four-file source/test scope, closeout scope, risks, non-goals, verification, and rollback defined.
- [x] Project owner approves the exact plan and source/closeout file lists.
- [x] Mandatory 04n gate state begins before source edits.
- [x] Source implementation completes without scope expansion.
- [x] Focused and complete automated checks pass.
- [x] Complete diff, code, security, scope, secret, and generated-output reviews pass.
- [x] Documentation and D-035 match actual evidence.
- [x] The post-increment report passes and the 04n marker is complete and valid.

## Completion evidence

- `InitialGatewayRequest` accepts only run ID, gateway-request ID, and selected
  initial-turn text, then returns a non-cloneable borrowed-byte boundary.
- Private serialization fixes protocol version, request kind, turn, retry,
  `cortexa_desktop_mvp@1`, and all existing conservative limit values.
- Shared opaque-ID validation, blank/source-size rejection, and final
  post-escaping 64 KiB enforcement fail closed with typed content-free errors.
- Debug output redacts the complete body; no content reaches logs, audit,
  persistence, IPC, or a production caller.
- Six request unit tests, 18 gateway protocol tests, nine tool tests, one public
  request integration test, rustfmt, and Clippy with warnings denied pass.
- `npm run verify` passes with 17 hook, 124 frontend, 92 Rust library, and 12
  Rust integration tests plus lint, typecheck, frontend builds, and Tauri release
  no-bundle.
- The network-enabled npm audit retry reports zero vulnerabilities.
- Exact source, closeout, conflict, secret, generated-output, architecture,
  code-health, security, and complete-diff reviews have no blocking finding.
- No manual verification is required because no production or user-visible path
  changed.
- D-035 records the closed request boundary; O-006 and O-007 continue to block
  authenticated transport and live provider traffic.
- The consolidated result is `PASS WITH ADVISORIES`; advisories are the absent
  runtime caller and required future deployed-gateway tool-set agreement.

## Exact next task

Wait for explicit project-owner direction to commit, push, and merge verified
Increment 4N. Do not start later planning or implementation.
