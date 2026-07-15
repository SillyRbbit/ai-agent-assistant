# Phase 4 Increment 4M - Remove legacy platform scaffold

Last updated: 2026-07-15

Status: **Verified complete; uncommitted**

## Goal

Remove the disconnected Phase 2 Rust platform scaffold before future macOS tools,
permissions, secret storage, or local authentication work can treat its
caller-authored generic capability map as authoritative operating-system evidence.

## Planning result

- `PlatformMetadata` accepts arbitrary operating-system, architecture, and family
  strings even though the live application already exposes compile-time target
  metadata through the independent typed `AppInfo` IPC boundary.
- `MockPlatformAdapter::with_capability` lets any caller assign `Available`,
  `Disabled`, or `Unavailable` to broad capabilities without an OS query, resource
  scope, provenance, observation time, requestability, user initiation, dependent
  feature, or last-use evidence.
- The generic enum combines ordinary integrations and privileged capabilities but
  omits required future boundaries such as secret storage, local authentication,
  selected-resource scope, and capability-specific failure semantics.
- Repository search finds no source or integration caller outside the platform
  module and its three embedded unit tests. The only external reference is the
  one-line crate-root export.
- The shipping Permission Center is fixed frontend placeholder data and the live
  Settings diagnostics use `AppInfo`; neither depends on the Rust platform mock.
- Product and architecture requirements for capability-specific platform adapters
  remain intact. Their trusted contracts belong to separately approved Phase 6
  and Phase 7 increments.

## Source scope

Delete:

```text
src-tauri/src/platform/adapter.rs
src-tauri/src/platform/mod.rs
src-tauri/src/platform/types.rs
```

Change:

```text
src-tauri/src/lib.rs
```

The `lib.rs` change removes only `pub mod platform;`. Exact design, risks,
non-goals, verification, closeout files, and rollback are recorded in
[`docs/plans/04m-remove-legacy-platform-scaffold.md`](../plans/04m-remove-legacy-platform-scaffold.md).

## Baseline evidence

Passed on clean synchronized `main` at `ecd49be`:

```text
python3 .codex/hooks/post_increment_gate.py status
  Increment 04l complete, valid: true, PASS WITH ADVISORIES
npm run typecheck
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked platform::
  3 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked app_info::
  1 passed
cargo test --manifest-path src-tauri/Cargo.toml --test smoke --locked
  1 passed
npx vitest run src/App.test.tsx -t "renders the Permission Center without a permission request control"
  1 passed; 24 skipped
rg -n "PlatformAdapter|MockPlatformAdapter|PlatformResult|PlatformError|PlatformMetadata|PlatformCapability|CapabilityStatus|CapabilityReport" src-tauri/src src-tauri/tests -g '!src-tauri/src/platform/**'
  no callers outside the proposed deleted module; required exit status 1
rg -n "pub mod platform" src-tauri/src/lib.rs
  one expected crate-root export
```

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
docs/increments/04m-remove-legacy-platform-scaffold.md
docs/plans/04m-remove-legacy-platform-scaffold.md
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

No source, test, decision, security, product, troubleshooting, dependency,
lockfile, Tauri, frontend, app-info, Permission Center, storage, gateway, policy,
approval, audit, IPC, capability configuration, CSP, packaging, or permission file
changes during planning.

Implementation closeout may additionally change only:

```text
DECISIONS.md
docs/reviews/2026-07-15-04m-post-increment-review.md
src-tauri/src/lib.rs
src-tauri/src/platform/adapter.rs
src-tauri/src/platform/mod.rs
src-tauri/src/platform/types.rs
```

## Completion gates

- [x] Required repository, product, architecture, security, review, decision,
      troubleshooting, workflow, 4L, and actual source state reconciled.
- [x] Clean synchronized Git baseline, valid 04l marker, toolchains, platform,
      caller search, and focused baseline checks recorded.
- [x] Exact four-file source scope, closeout scope, risks, non-goals,
      verification, and rollback defined.
- [x] Project owner approves the exact plan and source/closeout file lists.
- [x] Post-increment state begins before source edits.
- [x] Source implementation completes without scope expansion.
- [x] Focused and complete automated checks pass.
- [x] Complete diff, code, security, scope, secret, and generated-output reviews
      pass.
- [x] Documentation and D-034 are synchronized with actual evidence.
- [x] The post-increment report passes and the 4M marker is complete and valid.

## Completion evidence

- The three platform files are deleted and `src-tauri/src/lib.rs` loses only the
  platform export.
- The final legacy-symbol scan returns no matches with required exit status 1.
- The app-info unit test, public metadata smoke test, and focused Permission
  Center test pass unchanged.
- Clippy passes with warnings denied.
- Complete `npm run verify` passes with 17 hook, 124 frontend, 86 Rust library,
  and 11 Rust integration tests plus formatting, lint, typecheck, frontend builds,
  and Tauri release no-bundle.
- Network-enabled `npm audit --audit-level=low` reports zero vulnerabilities.
- No manual verification is required because no production or user-visible path
  changed.
- Complete source, scope, conflict, secret, generated-output, architecture,
  security, code-health, and documentation reviews have no blocking finding.
- D-034 preserves future capability-specific adapters and authoritative, scoped,
  fresh permission evidence.
- The consolidated result is `PASS WITH ADVISORIES`; advisories are deferred
  platform design and theoretical unsupported external use of the removed public
  module.

## Exact next task

Wait for explicit project-owner direction to commit, push, and merge verified
Increment 4M. Do not start later planning or implementation.
