# Phase 4 Increment 4K - Remove legacy provider scaffold

Last updated: 2026-07-15

Status: **Verified complete**

## Goal

Remove the disconnected synchronous arbitrary-string provider scaffold before a
future gateway transport or coordinator can adopt it as the production provider
boundary.

## Planning result

- `agent::provider` exposes a synchronous `complete` trait, caller-authored
  request strings, arbitrary assistant response strings, and arbitrary mock
  failure strings.
- `agent::types` contains only the request and response values used by that
  scaffold.
- Repository search finds no caller outside those two files and their three
  embedded unit tests.
- The verified `agent::gateway_protocol` module is independent and already owns
  the closed, bounded, sequence-checked normalized response contract,
  cancellation state, and redacted failures required by D-021.
- The smallest coherent next increment is deletion, not redesign: remove the two
  legacy modules and their exports while leaving the verified gateway, schema,
  policy, approval, and audit chain unchanged.
- A future provider transport must receive a separately approved closed request
  contract and produce only validated normalized gateway events. It must not
  restore the current arbitrary-string synchronous interface.

## Source scope

Delete:

```text
src-tauri/src/agent/provider.rs
src-tauri/src/agent/types.rs
```

Change:

```text
src-tauri/src/agent/mod.rs
```

The exact design, risks, non-goals, verification, closeout files, and rollback are
recorded in
[`docs/plans/04k-remove-legacy-provider-scaffold.md`](../plans/04k-remove-legacy-provider-scaffold.md).

## Implemented scope

- Deleted `src-tauri/src/agent/provider.rs` and
  `src-tauri/src/agent/types.rs`.
- Removed only their exports from `src-tauri/src/agent/mod.rs`.
- Preserved `agent::gateway_protocol`, `agent::function_call_validation`, and
  every other source and test file unchanged.
- Added no replacement provider, request contract, transport, network,
  credential, coordinator, dispatch, executor, persistence, IPC, UI, dependency,
  capability, or permission.

## Baseline evidence

Passed on clean synchronized `main` at `99f9279`:

```text
python3 .codex/hooks/post_increment_gate.py status
  Increment 04i complete, valid: true, PASS WITH ADVISORIES
npm run typecheck
  passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::provider::
  3 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
  18 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
  2 passed
npm run format:check
  passed after planning edits
git diff --check
  passed after planning edits
rg -n "AgentProvider|MockAgentProvider|AgentProviderResponse|AgentRequest" src-tauri/src src-tauri/tests -g '!**/agent/provider.rs' -g '!**/agent/types.rs'
  no matches outside the proposed deleted files; required exit status 1
```

Repository search confirms `AgentProvider`, `MockAgentProvider`, `AgentRequest`,
`AgentProviderResponse`, and `AgentProviderError` have no current source or
integration caller outside the two proposed deleted modules and their embedded
tests. Historical backup documentation is not an executable caller.

Toolchains and platform:

```text
Node.js v26.3.0
npm 11.16.0
Cargo 1.90.0
rustc 1.90.0
rustfmt 1.8.0-stable
Clippy 0.1.90
arm64 macOS 26.5.2
Xcode Command Line Tools: /Library/Developer/CommandLineTools
```

## Planning file scope

Created:

```text
docs/increments/04k-remove-legacy-provider-scaffold.md
docs/plans/04k-remove-legacy-provider-scaffold.md
```

Updated:

```text
AGENTS.md
CHANGELOG.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/plans/README.md
```

No source, test, decision, security, troubleshooting, dependency, lockfile,
Tauri, frontend, storage, gateway, policy, approval, audit, coordinator, dispatch,
executor, IPC, capability, CSP, packaging, or permission file changes during
planning.

The `04i` marker was complete and valid on merged clean `main` before planning.
The nine documentation edits intentionally make that prior workspace fingerprint
stale, so status now reports completed Increment `04i` with `valid: false`. No
`04k` state is active, and no post-increment completion evidence is claimed during
approval-blocked planning.

The first caller-absence scan used non-path-aware exclusion globs and printed the
definitions in the proposed deleted files. The corrected path-aware command above
returned no matches. This was command syntax, not a source finding.

Implementation closeout may additionally change only:

```text
DECISIONS.md
docs/reviews/2026-07-15-04k-post-increment-review.md
src-tauri/src/agent/mod.rs
src-tauri/src/agent/provider.rs
src-tauri/src/agent/types.rs
```

## Final verification

Passed:

```text
python3 .codex/hooks/post_increment_gate.py begin --increment 04k
  active before source edits on the approved exact retry
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
  passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::
  18 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked
  2 passed
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
  passed
rg -n "AgentProvider|MockAgentProvider|AgentProviderResult|AgentProviderError|AgentRequest|AgentProviderResponse" src-tauri/src src-tauri/tests
  no matches; required exit status 1
npm run verify
  hook tests: 17 passed
  frontend tests: 124 passed
  Rust library tests: 92 passed
  Rust integration tests: 11 passed
  formatting, ESLint, Clippy, typecheck, Vite builds, and Tauri release no-bundle passed
npm audit --audit-level=low
  network-enabled retry passed with 0 vulnerabilities
git diff --check
  passed
```

Failed and resolved:

- The first sandboxed gate-state begin could not write ignored `.codex` state.
  The approved exact retry succeeded before source edits.
- The first sandboxed npm audit could not resolve the registry or write user-level
  npm logs. The approved network-enabled retry passed with zero vulnerabilities.
- The first planning caller scan used ineffective exclusion globs. The corrected
  path-aware baseline and final post-deletion scan found no unexpected caller.

Checks not run:

- No native launch or manual interaction was required because the removed
  scaffold had no production caller or user-visible/operating-system path.
- No RustSec audit was required because manifests and lockfiles are unchanged.

Manual verification pending: none. Exact scope, conflict, secret,
generated-output, complete-diff, architecture, code-health, and security reviews
have no blocking finding.

## Completion gates

- [x] Required repository, product, architecture, security, review, decision,
      troubleshooting, workflow, 4I, and actual source state reconciled.
- [x] Clean synchronized Git baseline, valid 04i marker, toolchains, platform,
      and focused baseline checks recorded.
- [x] Exact three-file source scope, closeout scope, risks, non-goals,
      verification, and rollback defined.
- [x] Project owner approves the exact plan and source/closeout file lists.
- [x] Post-increment state begins before source edits.
- [x] Source implementation completes without scope expansion.
- [x] Focused and complete automated checks pass.
- [x] Complete diff, code, security, scope, secret, and generated-output reviews
      pass.
- [x] Documentation and D-032 are synchronized with actual evidence.
- [x] The post-increment report passes and the 4K marker is complete and valid.

## Exact next task

Wait for explicit project-owner direction to commit, push, and merge Increment
4K. Do not start later work.
