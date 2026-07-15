# Phase 4 Increment 4Q - Terminally release initial function call

Last updated: 2026-07-15

Status: **Verified complete; uncommitted**

## Goal

Keep a schema-validated initial-turn function call private until the normalized
gateway response reaches terminal completion, and discard it on failure or
cancellation.

## Implementation result

- Planning began from clean synchronized `main` at `8c1a2e0`; Increment 4P is
  committed, pushed, fast-forward merged, and its completion marker was valid
  before these planning edits.
- Focused baseline verification passes with TypeScript typecheck, six request
  tests, 18 protocol tests, six function-validation tests, nine tool tests, eight
  public request-contract tests, and two policy-binding tests.
- Project-owner approval was received, and mandatory `04q` gate state began
  before source edits.
- The bound turn now owns one private optional pending call. A valid function
  frame returns `None` while status remains `Streaming`; accepted terminal
  completion takes and releases the exact call once.
- Accepted gateway failure and successful cancellation discard the pending call.
  Transactional malformed, mismatched, and out-of-sequence frames retain it for
  the correct terminal frame.
- Text completion and local schema failure remain unchanged. Content-bearing
  arguments stay bounded, ephemeral, non-serializable, and absent from debug and
  errors.
- Lower-level protocol, schema validator, registry, policy, approval, audit,
  transport, and runtime APIs remain unchanged.

## Exact source and test scope

Change only:

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

The request module privately buffers one bounded schema-validated call, releases
it only on terminal response completion, and discards it on failure or
cancellation. The integration test proves the public contract. Exact behavior,
risks, non-goals, verification, closeout, and rollback are in
[`docs/plans/04q-terminally-release-initial-function-call.md`](../plans/04q-terminally-release-initial-function-call.md).

## Planning baseline

Passed before documentation edits:

```text
git status --short --branch
  clean main tracking origin/main
git rev-parse HEAD main origin/main
  all 8c1a2e0c082e5ef996c3f0e29f56785a884e3b94
python3 .codex/hooks/post_increment_gate.py status
  Increment 04p complete, valid: true, PASS WITH ADVISORIES
npm run typecheck
  passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
  18 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::
  9 passed
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
  8 passed
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
  2 passed
```

Toolchains are Node.js `v26.3.0`, npm `11.16.0`, Cargo and rustc `1.90.0`,
rustfmt `1.8.0-stable`, and Clippy `0.1.90` on arm64 macOS `26.5.2` with Xcode
Command Line Tools at `/Library/Developer/CommandLineTools`.

## Planning file scope

Create:

```text
docs/increments/04q-terminally-release-initial-function-call.md
docs/plans/04q-terminally-release-initial-function-call.md
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

No source, test, security, product, decision, troubleshooting, workflow,
dependency, manifest, lockfile, Tauri, frontend, storage, capability, entitlement,
or permission file changes during planning.

## Risks

- The optional-event return narrows a public Rust API and requires callers to
  treat `None` only as an accepted pending call.
- Typed argument lifetime extends until terminal completion, so every failure and
  cancellation path must discard the bounded pending value.
- Transactional protocol errors after buffering must retain the call for the
  correct contiguous terminal frame without releasing it early.
- Lower-level protocol and registry APIs remain available; future initial
  transport code must use the terminal-release turn.
- Output and typed arguments remain content-bearing and require redacted debug
  behavior.

## Non-goals

Policy, approval, audit, dispatch, execution, networking, gateway deployment,
provider SDKs or parameters, authentication, credentials, Keychain, live traffic,
retries, deadlines, transport abort, continuation, tool results, context
selection, runtime coordination, Tauri, WebView, SQLite, dependencies,
capabilities, entitlements, and permissions are excluded.

## Completion gates

- [x] Required repository memory, workflow, product, architecture, security,
      decisions, troubleshooting, 4P, and actual source state reconciled.
- [x] Clean synchronized Git baseline, valid 04p marker, toolchains, caller scan,
      and focused checks recorded.
- [x] Exact two-file source/test scope, closeout scope, risks, non-goals,
      verification, and rollback defined.
- [x] Project owner approves the exact plan and source/closeout file lists.
- [x] Mandatory 04q gate state begins before source edits.
- [x] Source implementation completes without scope expansion.
- [x] Focused and complete automated checks pass.
- [x] Complete diff, code, security, scope, secret, and generated-output reviews pass.
- [x] Documentation and durable decision match actual evidence.
- [x] The post-increment report passes and the 04q marker is complete and valid.

## Verification result

Passed:

```text
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
  18 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::
  9 passed
cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked
  9 passed
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
  2 passed
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
npm run verify
npm audit --audit-level=low
  found 0 vulnerabilities
```

No manual verification is required because there is no production caller,
user-visible behavior, network, credential, native, persistence, capability, or
permission change.

## Rollback

Before commit, restore the two source/test files to `8c1a2e0` and revert only the
declared 4Q documentation. After commit, revert the single 4Q commit. No data,
migration, dependency, credential, compatibility identifier, or remote resource
requires rollback.

## Exact next task

Wait for explicit project-owner direction to commit, push, and merge verified
Increment 4Q. Do not start later planning or implementation.
