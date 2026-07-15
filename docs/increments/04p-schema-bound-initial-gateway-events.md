# Phase 4 Increment 4P - Schema-bound initial gateway events

Last updated: 2026-07-15

Status: **Verified complete**

## Goal

Make the verified bound initial gateway turn own exact local schema validation so
no raw normalized function call or caller-selected registry is exposed through
the future trusted initial-turn path.

## Planning result

- Planning began from clean synchronized `main` at `87be00e`; Increment 4O is
  committed, pushed, and fast-forward merged, and its completion marker was valid
  before these planning edits.
- Focused baseline verification passes with TypeScript typecheck, six request
  tests, 18 protocol tests, six function-validation tests, nine tool tests, six
  public request-contract tests, and two policy-binding tests.
- Repository search finds no existing schema-bound initial event abstraction.
- The current bound turn returns `ValidatedGatewayEvent`, whose function variant
  contains raw JSON in `UntrustedFunctionCall`; local validation still requires a
  separately caller-supplied registry.
- A two-file event/error wrapper and public-boundary test close that initial-turn
  gap without changing lower-level protocol, validation, registry, policy,
  approval, audit, transport, or runtime APIs.
- Mandatory 04p gate state began before source edits. The implementation and
  complete verification are finished in the current uncommitted workspace.

## Proposed source and test scope

Change only:

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

The request module will construct one private exact registry, convert normalized
events into a closed `InitialGatewayEvent`, consume function calls through the
existing local validator, and close terminally on schema failure. The integration
test will prove the public contract. Exact behavior, risks, non-goals,
verification, closeout, and rollback are in
[`docs/plans/04p-schema-bound-initial-gateway-events.md`](../plans/04p-schema-bound-initial-gateway-events.md).

## Planning baseline

Passed before documentation edits:

```text
git status --short --branch
  clean main tracking origin/main
git rev-parse HEAD main origin/main
  all 87be00e6b06767cc4a276a683546e097958cf6fa
python3 .codex/hooks/post_increment_gate.py status
  Increment 04o complete, valid: true, PASS WITH ADVISORIES
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
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
  2 passed
```

Toolchains are Node.js `v26.3.0`, npm `11.16.0`, Cargo and rustc `1.90.0`,
rustfmt `1.8.0-stable`, and Clippy `0.1.90` on arm64 macOS `26.5.2` with Xcode
Command Line Tools at `/Library/Developer/CommandLineTools`.

## Planning file scope

Create:

```text
docs/increments/04p-schema-bound-initial-gateway-events.md
docs/plans/04p-schema-bound-initial-gateway-events.md
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

- The wrapper mirrors normalized event variants; exhaustive conversion must make
  future protocol drift a compile failure.
- The private registry and validator tool set must derive from the same fixed
  schema array.
- Schema failure occurs after protocol acceptance, so wrapper-owned terminal
  failure must override status and reject every later operation.
- Public event/error API narrowing could affect a theoretical unsupported
  external Rust consumer.
- Lower-level protocol and registry APIs remain available; future initial
  transport code must use the schema-bound turn.
- Output and typed arguments remain content-bearing and require redacted debug
  behavior.

## Non-goals

Networking, gateway deployment, provider SDKs or parameters, authentication,
credentials, Keychain, live traffic, retries, deadlines, transport abort,
continuation, tool results, context selection, runtime coordination, policy,
approval, audit writes or persistence, dispatch, execution, Tauri, WebView,
SQLite, dependencies, capabilities, entitlements, and permissions are excluded.

## Completion gates

- [x] Required repository memory, workflow, product, architecture, security,
      decisions, troubleshooting, 4O, and actual source state reconciled.
- [x] Clean synchronized Git baseline, valid 04o marker, toolchains, abstraction
      scan, and focused checks recorded.
- [x] Exact two-file source/test scope, closeout scope, risks, non-goals,
      verification, and rollback defined.
- [x] Project owner approves the exact plan and source/closeout file lists.
- [x] Mandatory 04p gate state begins before source edits.
- [x] Source implementation completes without scope expansion.
- [x] Focused and complete automated checks pass.
- [x] Complete diff, code, security, scope, secret, and generated-output reviews pass.
- [x] Documentation and durable decision match actual evidence.
- [x] The post-increment report passes and the 04p marker is complete and valid.

## Completion evidence

- The exact source/test implementation changes only
  `src-tauri/src/agent/gateway_request.rs` and
  `src-tauri/tests/gateway_request_contract.rs`.
- The turn owns one private exact registry, returns only closed schema-bound
  events, and terminally fails after local schema rejection.
- Focused verification passes with six request, 18 protocol, six
  function-validation, nine tool, eight public contract, and two policy-binding
  tests plus rustfmt and Clippy with warnings denied.
- Complete `npm run verify` passes with 17 hook, 124 frontend, 92 Rust library,
  and 19 Rust integration tests plus builds and Tauri release no-bundle. The
  network-enabled npm audit reports zero vulnerabilities.
- Exact-scope, conflict, secret, generated-output, architecture, security,
  code-health, complete-diff, and documentation reviews have no blocking finding.
  No manual verification is required.
- D-037 records the durable trust-boundary decision. The consolidated result is
  `PASS WITH ADVISORIES`; no later implementation increment is Ready.

## Rollback

Before commit, restore the two source/test files to `87be00e` and revert only the
declared 4P documentation. After commit, revert the single 4P commit. No data,
migration, dependency, credential, compatibility identifier, or remote resource
requires rollback.

## Exact next task

Wait for explicit project-owner direction to commit, push, and merge verified
Increment 4P. Do not start later planning or implementation.
