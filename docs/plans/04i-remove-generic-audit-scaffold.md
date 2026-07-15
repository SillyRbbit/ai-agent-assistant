# Execution plan - Increment 4I remove generic audit scaffold

Status: **Complete after reconstruction**

Owner: Project maintainer

Last updated: 2026-07-15

## Execution status

The project owner approved reconstruction of the exact source and closeout scopes
on corrected 4J `main`. The original `cf9d701` commit is preserved under
`codex/phase4-increment-4i-pre-fingerprint-fix`; its changes were applied to a fresh
branch with `git cherry-pick --no-commit`. Implementation still deletes only the
two generic modules and removes their exports from `audit::mod`. Focused checks,
complete repository verification, review, documentation reconciliation, and the
corrected mandatory gate pass without scope expansion.

## Goal and outcome

Delete the unused public caller-authored generic audit API before future runtime
wiring can mistake it for a trusted, structured, redacted audit boundary.

The outcome is a Rust audit namespace containing only the verified typed
`audit::approval` module. No replacement abstraction or new product capability is
introduced.

## User-visible outcome

None. The generic scaffold has no production caller, and the shipping application
does not expose audit persistence or history from Rust.

## Existing behavior and constraints

- `audit::types::AuditEventInput` publicly accepts arbitrary actor, event type,
  summary, and details values.
- `audit::logger::AuditLogger` accepts that input, and its in-memory and no-op
  implementations return clonable arbitrary-string records.
- The redactor tokenizes on whitespace and masks only a short marker list. It is a
  deterministic mock utility, not a safe structured audit boundary.
- Repository search finds no production or integration caller. The only executable
  references are the two generic modules and four embedded unit tests.
- `audit::approval` has no dependency on the generic modules. It privately derives
  a closed record from an exact terminal approval resolution and has focused unit
  and public-boundary integration coverage.
- D-029 deliberately left the generic scaffold disconnected and non-production.
  If implementation is approved, a new D-030 will record its removal without
  rewriting the historical D-029 entry.
- The crate is the application core and no repository evidence identifies a
  supported external consumer of this unused public scaffold.

## Why deletion is the smallest increment

Keeping an arbitrary-string API for hypothetical future use creates a plausible
bypass around the typed audit design. Generalizing it, adapting it, or introducing
a repository trait would require defining run, policy, execution, result,
retention, persistence, and failure semantics that are not yet approved.

Deletion removes that ambiguity in three source paths. Future audit event families
can be introduced independently as closed typed records when their authoritative
inputs and retention rules exist.

## Exact source scope

Delete:

```text
src-tauri/src/audit/logger.rs
src-tauri/src/audit/types.rs
```

Change:

```text
src-tauri/src/audit/mod.rs
```

The `mod.rs` change removes only:

```rust
pub mod logger;
pub mod types;
```

It must preserve:

```rust
pub mod approval;
```

## Exact implementation closeout scope

In addition to the three source paths above, implementation closeout may change
only:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04i-remove-generic-audit-scaffold.md
docs/plans/04i-remove-generic-audit-scaffold.md
docs/plans/README.md
docs/reviews/2026-07-15-04i-post-increment-review.md
```

`DECISIONS.md` is limited to proposed D-030 recording that future trusted audit
events require closed typed inputs rather than restoration of this generic API.
Stop and request approval before changing any other file.

## Implementation steps

1. After project-owner approval, run:

   ```bash
   python3 .codex/hooks/post_increment_gate.py begin --increment 04i
   ```

2. Delete `audit/logger.rs` and `audit/types.rs`.
3. Remove only their module exports from `audit/mod.rs`.
4. Confirm every generic scaffold symbol is absent from current Rust source and
   tests while historical planning records remain unchanged.
5. Run focused typed approval-audit tests, Clippy, and the complete repository
   verification suite.
6. Review the complete diff against `SECURITY.md` and `CODE_REVIEW.md`, synchronize
   closeout documentation, append D-030, and complete the mandatory gate.

## Test plan

Focused verification must prove:

- the typed approval-audit unit tests still pass unchanged;
- the public gateway -> schema -> policy -> approval -> typed-audit integration
  test still passes unchanged;
- the native decision-source tests that exercise sealed source evidence through
  the typed adapter still pass unchanged;
- no generic audit type, trait, implementation, redactor, module export, or caller
  remains in current Rust source or tests;
- no source path outside the exact three-file plan changes; and
- complete repository verification passes after the four deleted embedded tests
  are removed from the expected Rust library count.

No new test file is justified: this increment removes an unused module, and the
remaining typed boundary already has unit and public integration coverage.

## Verification commands

Focused during implementation:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::approval::
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::decision_source::
cargo test --manifest-path src-tauri/Cargo.toml --test approval_audit_binding --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
rg -n "AuditEventInput|AuditEvent|AuditLogger|InMemoryAuditLogger|NoopAuditLogger|redact_secret_like_content" src-tauri/src src-tauri/tests
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

Then run `$post-increment-gate`, create and finalize the exact 4I report, and
confirm status is complete and valid.

## Manual verification

None required. The removed scaffold has no production caller, Tauri registration,
IPC path, user-visible state, persistence, or operating-system interaction.

## Risks and mitigations

- **Public API removal:** an unsupported external crate consumer could import the
  public scaffold even though repository search finds no internal caller.
  Mitigation: the application crate is not treated as a published compatibility
  surface; rollback restores exactly two files and two exports.
- **Typed audit regression:** an over-broad module edit could remove or alter
  `audit::approval`. Mitigation: `mod.rs` changes by two lines only, the typed file
  remains byte-for-byte unchanged, and focused unit/integration tests run.
- **Future audit pressure:** later work might reintroduce arbitrary strings for
  convenience. Mitigation: D-030 records that future event families require
  separately reviewed closed typed contracts.
- **Evidence-count drift:** removal intentionally deletes four generic unit tests.
  Mitigation: closeout records actual test counts and never describes the lower
  count as a regression.
- **Historical documentation confusion:** completed plans describe the scaffold as
  existing at their historical checkpoints. Mitigation: preserve those records
  and update only current-state memory plus the new decision.

## Explicit non-goals

- A replacement generic audit trait, enum, event union, or redactor.
- Durable audit storage, SQLite tables, migrations, encryption, retention,
  deletion, export, or audit-history UI.
- An authoritative run coordinator or production caller.
- Dispatch, executor, local-task creation, provider continuation, or tool results.
- New typed records for run, proposal, policy, execution, cancellation, or final
  outcomes.
- Changes to `audit::approval`, approval manager, native decision source, policy,
  schema validation, gateway protocol, or provider mocks.
- Tauri commands/events, WebView changes, IPC, networking, gateway deployment,
  credentials, Keychain, LocalAuthentication, capabilities, CSP, packaging,
  entitlements, or operating-system permissions.
- Dependencies, manifests, lockfiles, storage code, database files, or generated
  artifacts.

## Rollback or failure strategy

Restore `src-tauri/src/audit/logger.rs` and `src-tauri/src/audit/types.rs` exactly
from corrected baseline `a2b9803`, and restore their two exports in
`src-tauri/src/audit/mod.rs`.
Revert only the 4I planning/closeout documents and D-030. No migration, persisted
data, dependency, capability, credential, or external state requires rollback.

If any repository caller is discovered before deletion, stop and revise the plan
rather than widening it to migrate that caller automatically.

## Acceptance criteria

- The exact two generic audit files are deleted and only their exports are removed
  from `audit/mod.rs`.
- `audit::approval` and all other source files remain unchanged.
- Current Rust source and tests contain no generic audit scaffold symbol or module
  export.
- Existing typed approval-audit unit, source-evidence, and integration checks pass.
- The implementation adds no replacement abstraction or production caller.
- No runtime behavior, persistence, network, credential, dependency, migration,
  IPC, UI, capability, or permission changes.
- The exact three-file source scope and declared closeout scope are preserved.
- Focused checks, full repository verification, dependency audit, complete diff,
  code review, security review, documentation synchronization, and mandatory
  post-increment gate pass.
- D-030 is accepted and Increment 4I is marked complete only after project-owner
  approval and all gates pass.

## Planning baseline evidence

Passed on clean synchronized `main` at `e3af5a4`:

```text
npm run typecheck
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::
  10 passed
cargo test --manifest-path src-tauri/Cargo.toml --test approval_audit_binding --locked
  1 passed
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

At the planning checkpoint, changes were documentation-only and source
implementation remained blocked on project-owner approval.

## Completion result

The exact three-file source scope is complete. `audit::approval` remains
byte-for-byte unchanged, while current Rust source and tests contain no generic
audit scaffold symbol or export. The four removed tests belonged only to the
deleted implementation; six typed adapter tests, eleven native-source tests, and
one public approval-audit integration test pass unchanged.

`npm run verify` passes with 17 hook tests, 124 frontend tests, 95 Rust library
tests, 11 Rust integration tests, lint, typecheck, Vite production builds, and the
Tauri release no-bundle build. The network-enabled npm audit retry reports zero
vulnerabilities. Complete scope, diff, architecture, code-health, security,
secret, generated-output, and conflict reviews have no blocking finding.

Reconstruction started from merged 4J `main` at `a2b9803`. The original commit is
preserved at `cf9d701`; its change was applied without committing. Eight expected
shared-document conflicts were reconciled from corrected `main`, while every 4J
hook, test, security, troubleshooting, increment, plan, and report file remained
unchanged. The corrected fingerprint accepts the reviewed deletions and produces
a complete valid 04i marker.

No manual check applies because no production caller, Tauri registration, IPC,
UI, persistence, or operating-system behavior changed. D-030 records the removal
and the requirement for separately approved closed typed future audit events. The
post-increment report result is `PASS WITH ADVISORIES`; the advisory is the
intentional absence of an approved next increment and the separately deferred
durable audit/coordinator design.

## Exact next task

Increment 4I is complete. Wait for the project owner to select and approve one
bounded next plan. Do not infer or begin another increment, commit, push, or merge
without explicit direction.
