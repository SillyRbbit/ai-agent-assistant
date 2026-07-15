# Execution plan - Increment 4M remove legacy platform scaffold

Status: **Complete; verified and uncommitted**

Owner: Project maintainer

Last updated: 2026-07-15

## Goal and outcome

Delete the unused Phase 2 Rust platform module before future macOS integrations
or permission enforcement can mistake caller-authored generic capability status
for authoritative operating-system evidence.

The outcome removes the current Rust `platform` namespace entirely. The product's
cross-platform architecture and future macOS adapter requirements remain intact
and must be implemented through separately approved capability-specific contracts.

## User-visible outcome

None. The Rust platform scaffold has no production caller, Tauri registration,
IPC path, frontend connection, native framework call, permission request, secret
storage integration, or local-authentication integration. Existing `AppInfo`
diagnostics and fixed Permission Center placeholders remain unchanged.

## Existing behavior and constraints

- `PlatformMetadata::new` accepts arbitrary operating-system, architecture, and
  family strings and derives `Clone` and raw-value `Debug`.
- `MockPlatformAdapter::with_capability` accepts any closed enum capability and
  lets a caller assign `CapabilityStatus::Available` without observing the OS.
- The map contains no resource scope, permission source, observation time,
  freshness, requestability, user-initiation evidence, dependent feature,
  last-use time, denial/restriction reason, or capability-specific error.
- `PlatformCapability` combines Calendar, Reminders, Contacts, Notifications,
  Files, Accessibility, Screen Recording, Automation, and Microphone under one
  generic status contract. It does not represent application/URL integration,
  clipboard, Keychain, selected-file security scope, or LocalAuthentication
  despite those being distinct product boundaries.
- `MockPlatformAdapter::default` reports only Calendar and Files as disabled. It
  neither describes actual platform support nor makes a permission query.
- Repository search finds no caller outside the platform module and its three
  embedded tests. The only external reference is `pub mod platform;` in the crate
  root.
- `current_app_info()` independently derives live architecture and OS target from
  `std::env::consts`; the Settings and smoke-test paths use that typed boundary.
- The Permission Center independently renders fixed no-request frontend rows and
  has focused coverage proving it exposes no permission request control.
- Product requirements still mandate capability-specific OS adapters and a
  user-controlled Permission Center in later phases. Deleting this mock does not
  remove or revise those requirements.

## Why deletion is the smallest increment

Repairing the generic scaffold would require capability-specific OS APIs,
permission semantics, selected-resource scope, provenance, observation freshness,
request flows, error contracts, thread/executor ownership, Keychain and
LocalAuthentication boundaries, Tauri IPC design, and Permission Center state.
Those are Phase 6 and Phase 7 capabilities and cannot be safely inferred during
Phase 4 cleanup.

Deleting four source paths removes the misleading unused surface while leaving
the live app-info, Permission Center, Tauri capability configuration, and all
native boundaries unchanged. Future adapters can start from product requirements
instead of inheriting a generic fabricated-status API.

## Exact source scope

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

The `lib.rs` change removes only:

```rust
pub mod platform;
```

Every other crate export must remain unchanged.

## Exact implementation closeout scope

In addition to the four source paths above, implementation closeout may change
only:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04m-remove-legacy-platform-scaffold.md
docs/plans/04m-remove-legacy-platform-scaffold.md
docs/plans/README.md
docs/reviews/2026-07-15-04m-post-increment-review.md
```

`DECISIONS.md` is limited to proposed D-034 recording that future OS integration
requires separately approved capability-specific adapters and authoritative,
scoped, fresh permission evidence rather than restoration of this generic
caller-authored status map. Stop and request approval before changing any other
file.

## Implementation steps

1. After project-owner approval, run:

   ```bash
   python3 .codex/hooks/post_increment_gate.py begin --increment 04m
   ```

2. Delete the three files under `src-tauri/src/platform/`.
3. Remove only `pub mod platform;` from `src-tauri/src/lib.rs`.
4. Confirm every legacy platform symbol and crate module export is absent from
   current Rust source and tests while historical records remain unchanged.
5. Run focused app-info, smoke, and Permission Center checks, Clippy, and the
   complete repository verification suite.
6. Review the complete diff against `SECURITY.md` and `CODE_REVIEW.md`, synchronize
   closeout documentation, append D-034, and complete the mandatory gate.

## Test plan

Focused verification must prove:

- typed application metadata remains available through the unchanged Rust API;
- the public desktop metadata smoke test remains passing;
- the fixed Permission Center remains status-only and exposes no request control;
- no legacy platform type, trait, mock, capability status, report, module export,
  or current caller remains;
- no source path outside the exact four-file plan changes; and
- complete repository verification passes after the three deleted embedded tests
  are removed from the expected Rust library count.

No new test file is justified: this increment removes a disconnected module, and
the preserved app-info, public smoke, and Permission Center paths already have
focused coverage.

## Verification commands

Focused during implementation:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked app_info::
cargo test --manifest-path src-tauri/Cargo.toml --test smoke --locked
npx vitest run src/App.test.tsx -t "renders the Permission Center without a permission request control"
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
rg -n "PlatformAdapter|MockPlatformAdapter|PlatformResult|PlatformError|PlatformMetadata|PlatformCapability|CapabilityStatus|CapabilityReport|pub mod platform" src-tauri/src src-tauri/tests
```

The final `rg` command must return no matches. Exit status `1` means the required
absence check passed; any matching line fails scope completion.

Complete gate:

```bash
npm run verify
npm audit --audit-level=low
git diff --check
python3 .codex/hooks/post_increment_gate.py status
```

Then run `$post-increment-gate`, create and finalize the exact 4M report, and
confirm status is complete and valid.

## Manual verification

None required. The removed scaffold has no production caller, Tauri registration,
IPC path, user-visible state, native framework call, permission request, secret
store, local authentication, network transport, or operating-system interaction.

## Risks and mitigations

- **Public API removal:** an unsupported external crate consumer could import the
  public platform module even though repository search finds no internal caller.
  Mitigation: the application crate is not a supported external API; rollback
  restores exactly three files and one export.
- **Architecture confusion:** deleting the scaffold could be misread as deleting
  the platform-adapter requirement. Mitigation: proposed D-034 and current product
  documents preserve capability-specific future adapters and permission evidence.
- **App-info regression:** an over-broad crate-root edit could alter the live
  diagnostic boundary. Mitigation: `lib.rs` changes by one line only,
  `app_info.rs` remains byte-for-byte unchanged, and focused unit/public tests run.
- **Permission UI regression:** platform cleanup could accidentally change the
  independent fixed Permission Center. Mitigation: frontend files are outside
  scope and the focused no-request-control test runs unchanged.
- **Premature adapter design:** cleanup could expand into EventKit, Keychain,
  LocalAuthentication, permissions, or resource scopes. Mitigation: every
  replacement capability is an explicit non-goal and requires separate approval.
- **Evidence-count drift:** removal intentionally deletes three platform unit
  tests. Mitigation: closeout records actual counts and does not describe the
  lower count as a regression.

## Explicit non-goals

- Replacement platform metadata, adapter traits, capability maps, permission
  status types, mocks, or native framework wrappers.
- EventKit, Contacts, NSWorkspace, UserNotifications, Keychain,
  LocalAuthentication, Accessibility, ScreenCaptureKit, Apple Events, microphone,
  clipboard, file access, selected-resource scope, or URL/application integration.
- Permission request/open-settings controls, observation polling, last-used
  tracking, onboarding, settings synchronization, or Permission Center changes.
- Tauri commands/events/plugins, frontend state, IPC, capabilities, CSP,
  packaging, entitlements, or operating-system permission declarations.
- Changes to app-info, menu-bar, storage, gateway, function validation, tools,
  policy, approval, audit, provider transport, coordinator, dispatch, or executor.
- Dependencies, manifests, lockfiles, database files, or generated artifacts.
- Rewriting historical Increment 2A records that describe the scaffold at their
  original checkpoint.

## Security and privacy considerations

The change removes a public constructor path that can label a privileged
capability `Available` without authoritative OS evidence. It adds no replacement
permission, credential, personal-data, native-framework, or authority flow. The
implementation must not modify Tauri capabilities, entitlements, permission
descriptions, Keychain, LocalAuthentication, frontend permission presentation,
logs, audit, policy, approval, dispatch, or execution.

## Rollback or failure strategy

Restore `src-tauri/src/platform/adapter.rs`, `src-tauri/src/platform/mod.rs`, and
`src-tauri/src/platform/types.rs` exactly from baseline `ecd49be`, and restore
`pub mod platform;` in `src-tauri/src/lib.rs`. Revert only the 4M
planning/closeout documentation and D-034. No migration, persisted data,
dependency, capability, credential, permission, or external state requires
rollback.

If any current repository caller is discovered before deletion, stop and revise
the plan rather than widening it to migrate that caller automatically.

## Acceptance criteria

- The exact three platform files are deleted and only `pub mod platform;` is
  removed from `lib.rs`.
- App-info, Permission Center, Tauri configuration, and all other source files
  remain unchanged.
- Current Rust source and tests contain no legacy platform scaffold symbol or
  module export.
- Existing app-info unit, public smoke, and Permission Center checks pass.
- The implementation adds no replacement platform abstraction or runtime caller.
- No runtime behavior, OS query, permission, persistence, network, credential,
  dependency, migration, IPC, UI, capability configuration, or entitlement
  changes.
- The exact four-file source scope and declared closeout scope are preserved.
- Focused checks, full repository verification, dependency audit, complete diff,
  code review, security review, documentation synchronization, and mandatory
  post-increment gate pass.
- D-034 is accepted and Increment 4M is marked complete only after project-owner
  approval and all gates pass.

## Planning baseline evidence

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

At the planning checkpoint, changes were documentation-only and the previously
valid `04l` marker became stale because the planning documents changed. The
project owner then approved this exact source and closeout scope. Mandatory `04m`
state began before source edits.

## Implementation and verification result

The implementation deletes exactly the three platform files and removes only
`pub mod platform;` from `src-tauri/src/lib.rs`. Repository search finds no
legacy platform symbol or module export in current Rust source or tests. App-info,
Permission Center, Tauri configuration, tests, manifests, lockfiles, and all other
trust boundaries remain unchanged.

Passed:

```text
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked app_info::
  1 passed
cargo test --manifest-path src-tauri/Cargo.toml --test smoke --locked
  1 passed
npx vitest run src/App.test.tsx -t "renders the Permission Center without a permission request control"
  1 passed; 24 skipped
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
rg -n "PlatformAdapter|MockPlatformAdapter|PlatformResult|PlatformError|PlatformMetadata|PlatformCapability|CapabilityStatus|CapabilityReport|pub mod platform" src-tauri/src src-tauri/tests
  no matches; required exit status 1
npm run verify
  17 hook, 124 frontend, 86 Rust library, and 11 Rust integration tests passed;
  formatting, lint, typecheck, builds, and Tauri release no-bundle passed
npm audit --audit-level=low
  0 vulnerabilities on the network-enabled retry
git diff --check
```

No manual verification applies. D-034 records the separately approved future
platform boundary. The consolidated result is `PASS WITH ADVISORIES`; advisories
are the intentionally deferred capability-specific platform design and
theoretical unsupported external consumer of the removed public module.

## Exact next task

Wait for explicit project-owner direction to commit, push, and merge verified
Increment 4M. Do not start later planning or implementation.
