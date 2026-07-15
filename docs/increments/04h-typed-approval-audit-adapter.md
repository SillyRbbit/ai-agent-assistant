# Phase 4 Increment 4H - Typed approval-audit adapter

Last updated: 2026-07-15

Status: **Verified complete**

## Goal

Implement one bounded in-memory adapter that records a closed redacted projection
of a terminal `ApprovalResolution` without creating durable-audit, dispatch, or
execution authority.

## Planning result

- The existing generic audit scaffold accepts arbitrary event, summary, and details
  strings and applies only token-pattern redaction. It has no production caller and
  is not suitable for exact approval evidence.
- The verified approval resolution already exposes exact opaque identity, locally
  derived tool and policy facts, disposition, and optional closed interaction
  evidence. It also exposes a title-bearing preview that the adapter must never
  call.
- The proposed adapter derives a private-field, non-serializable record directly
  from the resolution, revalidates the exact current subject and complete
  disposition/evidence matrix, and stores no title or arbitrary text.
- The proposed in-memory boundary permits 1,024 records, one record per exact
  approval/run/request/call key, checked deterministic sequencing, and no eviction.
- A successful receipt is correlation evidence only. It proves no run liveness,
  user identity, device-owner authentication, durable write, dispatch eligibility,
  execution, or tool result.
- Existing generic audit code remains unchanged and disconnected. Persistence,
  coordinator wiring, execution, IPC, UI, gateway networking, credentials, and new
  permissions remain out of scope.

## Proposed implementation scope

Create:

```text
src-tauri/src/audit/approval.rs
src-tauri/tests/approval_audit_binding.rs
```

Change:

```text
src-tauri/src/audit/mod.rs
src-tauri/src/approvals/decision_source.rs
```

The `decision_source.rs` change is limited to test coverage. The exact design,
closeout file list, failure matrix, verification commands, risks, non-goals, and
rollback are recorded in
[`docs/plans/04h-typed-approval-audit-adapter.md`](../plans/04h-typed-approval-audit-adapter.md).

## Implemented scope

- Added private-field, non-serializable `ApprovalAuditRecord`, sequence, receipt,
  closed typed errors, and `InMemoryApprovalAuditAdapter` types.
- The adapter borrows one manager-produced resolution, revalidates exact
  `create_local_task@1`, risk, permission, policy, disposition, and interaction
  evidence before mutation, and never calls `ApprovalResolution::preview()`.
- Records retain only bounded opaque approval/run/request/call identity and closed
  local facts. Tool identity is represented internally by a closed enum rather
  than a caller-authored string.
- One adapter retains at most 1,024 records, rejects one exact
  approval/run/request/call subject twice, never evicts, and uses checked
  contiguous sequence assignment.
- Record and adapter debug output redact exact identity. Errors contain only fixed
  variants and the fixed capacity limit.
- The new integration test proves the public gateway -> schema -> policy ->
  approval -> run-termination -> typed-audit path. Native-source tests cover every
  valid button/no-decision/source-failure result plus extra and contradictory
  evidence rejection without opening a dialog.
- The generic arbitrary-string audit scaffold remains unchanged and disconnected.
  No production caller, persistence, runtime coordinator, executor, dispatch, IPC,
  UI, gateway, credential, capability, or permission path was added.

## Planning verification

Passed on clean `codex/phase4-increment-4h` at `74692c1`:

```text
npm run typecheck
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::
  4 passed
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
  2 passed
npm run format:check
  passed after a targeted Prettier correction to the two new Markdown files
git diff --check
  passed
```

## Final verification

Passed:

```text
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked audit::approval::
  6 passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::decision_source::
  11 passed
cargo test --manifest-path src-tauri/Cargo.toml --test approval_audit_binding --locked
  1 passed
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
npm run verify
  hook tests: 15 passed
  frontend tests: 124 passed
  Rust library tests: 99 passed
  Rust integration tests: 11 passed
  formatting, ESLint, Clippy, typecheck, Vite builds, and Tauri release no-bundle build passed
npm audit --audit-level=low
  0 vulnerabilities on the network-enabled retry
git diff --check
```

Failed and resolved:

- The first planning `npm run format:check` found layout-only drift in the two new
  Markdown files. Targeted Prettier formatting corrected them and the exact check
  passes.
- The first sandboxed `npm audit --audit-level=low` could not resolve
  `registry.npmjs.org` and could not write user-level npm logs. The approved
  network-enabled retry passed with zero vulnerabilities; no repository file
  changed.
- The first post-increment finalization validated the report but could not write
  ignored state under the sandbox-protected `.codex` directory. The approved exact
  retry completed the marker, and status is complete and valid.

Checks not run:

- No native application or dialog interaction gate was required. Production
  dialog code outside its test module and all shipping application paths are
  unchanged.
- No Rust dependency audit was required because Cargo manifests and the lockfile
  are unchanged.

Manual verification:

- None required. Increment 4H is transport-free and has no user-visible or
  operating-system permission behavior.

## Planning file scope

Created:

```text
docs/increments/04h-typed-approval-audit-adapter.md
docs/plans/04h-typed-approval-audit-adapter.md
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

No runtime, test, dependency, lockfile, Tauri, frontend, storage, provider,
gateway, policy, approval-authority, audit-persistence, dispatch, execution,
capability, CSP, packaging, or permission file changed during planning.

Final closeout additionally changes:

```text
DECISIONS.md
docs/reviews/2026-07-14-04h-post-increment-review.md
src-tauri/src/approvals/decision_source.rs
src-tauri/src/audit/mod.rs
src-tauri/src/audit/approval.rs
src-tauri/tests/approval_audit_binding.rs
```

## Completion gates

- [x] Required repository, product, security, review, decision, troubleshooting,
      approval, audit, and prior-increment state reconciled.
- [x] Current branch, clean baseline, recent commits, and focused baseline checks
      recorded.
- [x] Exact proposed runtime/test and closeout file scopes defined.
- [x] Goal, closed facts, evidence matrix, bounds, typed failures, risks, non-goals,
      verification, and rollback defined.
- [x] Project owner approves the exact plan and file lists.
- [x] Runtime/test implementation is complete without scope expansion.
- [x] Focused and complete automated checks pass.
- [x] Complete diff, code, security, scope, secret, and generated-output reviews
      pass.
- [x] Documentation and D-029 are synchronized with actual implementation evidence.
- [x] Post-increment report passes and the 4H completion marker is valid.

## Exact next task

Increment 4H is complete. Wait for the project owner to select and approve one
bounded next plan. Do not infer or begin another increment, commit, push, or merge
without explicit direction.
