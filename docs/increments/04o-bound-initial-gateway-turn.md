# Phase 4 Increment 4O - Bound initial gateway turn

Last updated: 2026-07-15

Status: **Verified complete; uncommitted**

## Goal

Bind the verified initial request bytes and response validator into one
transport-free Rust turn so a future trusted caller cannot independently choose
request/response identities or allowed tool-contract configuration.

## Planning result

- Git is clean and synchronized on `main` at `d7c4b69`; Increment 4N is
  committed, pushed, fast-forward merged, and its completion marker is valid.
- Focused baseline verification passes with six request tests, 18 protocol tests,
  nine tool tests, one public request-contract test, and TypeScript typecheck.
- Repository search finds no existing bound initial-turn abstraction.
- The initial request and stream validator currently accept correlation and tool
  configuration through separate public constructors.
- A two-file wrapper and public-boundary test close that local configuration gap
  without adding transport, credentials, continuation, or orchestration.
- The valid 04n marker was confirmed before planning edits. These planning changes
  are expected to make its workspace fingerprint stale; no 04o gate state has
  begun and no source implementation has started.

## Proposed source and test scope

Change only:

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

The request module will add `InitialGatewayTurn`, make the raw initial request
private, derive response names/version from the exact local tool catalog, and
delegate bounded stream validation and cancellation. The integration test will
prove the public contract. Exact behavior, risks, non-goals, verification,
closeout, and rollback are in
[`docs/plans/04o-bound-initial-gateway-turn.md`](../plans/04o-bound-initial-gateway-turn.md).

## Planning baseline

Passed before documentation edits:

```text
git status --short --branch
  clean main tracking origin/main
git rev-parse HEAD main origin/main
  all d7c4b69b36f83dfd1fd680b8bbe9ac9fc9aa3c5f
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
```

Toolchains are Node.js `v26.3.0`, npm `11.16.0`, Cargo and rustc `1.90.0`,
rustfmt `1.8.0-stable`, and Clippy `0.1.90` on arm64 macOS `26.5.2` with Xcode
Command Line Tools at `/Library/Developer/CommandLineTools`.

## Planning file scope

Create:

```text
docs/increments/04o-bound-initial-gateway-turn.md
docs/plans/04o-bound-initial-gateway-turn.md
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

- The wrapper must remain a narrow request/validator binding and not become a run
  coordinator.
- Making the raw request private narrows a public Rust API and could affect an
  unsupported external consumer.
- The fixed request tool-set and local catalog could drift; local names/version
  must be derived and tested, while deployment agreement remains deferred.
- Content-bearing request bytes remain exposed to a future trusted transport and
  must stay absent from debug, errors, logs, persistence, and audit.
- The lower-level public validator remains independently constructible for tests;
  future initial transport work must use the bound turn.

## Non-goals

Networking, gateway deployment, provider SDKs or parameters, authentication,
credentials, Keychain, live traffic, retries, deadlines, transport abort,
continuation, tool results, context selection, runtime coordination, policy,
approval, audit persistence, dispatch, execution, Tauri, WebView, SQLite,
dependencies, capabilities, entitlements, and permissions are excluded.

## Completion gates

- [x] Required repository memory, workflow, product, architecture, security,
      decisions, troubleshooting, 4N, and actual source state reconciled.
- [x] Clean synchronized Git baseline, valid 04n marker, toolchains, abstraction
      scan, and focused checks recorded.
- [x] Exact two-file source/test scope, closeout scope, risks, non-goals,
      verification, and rollback defined.
- [x] Project owner approves the exact plan and source/closeout file lists.
- [x] Mandatory 04o gate state begins before source edits.
- [x] Source implementation completes without scope expansion.
- [x] Focused and complete automated checks pass.
- [x] Complete diff, code, security, scope, secret, and generated-output reviews pass.
- [x] Documentation and durable decision match actual evidence.
- [x] The post-increment report passes and the 04o marker is complete and valid.

## Completion evidence

- `InitialGatewayTurn` is the only public initial request-construction path and
  does not implement `Clone`, `Serialize`, or `Deserialize`.
- Request bytes and response validation derive from one pair of opaque IDs.
- Exact allowed function names and version derive from the two local `ToolSchema`
  variants and must agree with `cortexa_desktop_mvp@1`.
- Public operations are limited to borrowed request bytes, status, normalized
  frame acceptance, and local terminal cancellation.
- Debug and typed errors retain no selected content, request bytes, or identities.
- Six preserved request tests, 18 protocol tests, nine tool tests, six public
  contract tests, rustfmt, Clippy, complete `npm run verify`, npm audit, scope,
  code, security, documentation, and mandatory gate reviews pass.
- No production caller or user-visible behavior changed, so no manual verification
  is required.
- D-036 records the durable boundary. The result is `PASS WITH ADVISORIES` for
  public API narrowing and the intentionally retained lower-level validator
  constructor.

## Rollback

Before commit, restore the two source/test files to `d7c4b69` and revert only the
declared 4O documentation. After commit, revert the single 4O commit. No data,
migration, dependency, credential, compatibility identifier, or remote resource
requires rollback.

## Exact next task

Wait for explicit project-owner direction to commit, push, and merge verified
Increment 4O. Do not start later planning or implementation.
