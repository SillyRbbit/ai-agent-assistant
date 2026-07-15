# Handoff

Last updated: 2026-07-14

## Current state

Phase 3 and Phase 4 Increments 4A through 4E are verified complete on the target Mac. Increment 4E is complete within the approved eight-file runtime/test scope on branch `phase4/increment-4e`, based on merged `main` at `1cf190f`.

Increment 4E completion evidence:

- the complete repository gate and focused automated checks pass;
- the JavaScript dependency audit passes;
- exact temporary RustSec scanning ran and its two pre-existing `quick-xml 0.39.4` findings have the scoped D-025 baseline disposition; and
- the project-owner native-dialog interaction matrix passed.

The changes are uncommitted and unpushed. D-025 is accepted, no advisory ignore or dependency remediation change was added, and no later increment has started.

## Completed implementation

- Removed public `ApprovalChoice` and the raw `ApprovalManager::decide` path.
- Added one owned, non-cloneable, non-serializable, redacted `ApprovalPresentation` issued only once by the pending manager.
- Added one sealed, non-cloneable, non-serializable `TrustedApprovalSourceOutcome` whose constructor is private to the macOS decision source.
- Moved one private `Arc` marker through manager -> presentation -> source outcome, compare it with `Arc::ptr_eq` before exact approval/run/gateway-request/function-call identity, and require that the one presentation was issued.
- Preserved one pending approval, 1,024 lifetime subjects, a 120-second monotonic TTL, no TTL extension on presentation, cancellation/expiry precedence, and non-evicting replay rejection.
- Added closed `RunTerminated`, `EditRequested`, `NativeNoDecision`, and `SourceFailed` cancellation reasons.
- Added precise native-dialog source, optional recognized-button, `NotEvaluated` authentication, and optional fixed source-failure evidence without title, actor-identity, run-liveness, audit, dispatch, or execution claims.
- Added a macOS-only direct `rfd` adapter with fixed `AI Agent Assistant approval` title and exact Reject/Approve/Edit custom-button order. Reject is first/default.
- Mapped only recognized custom results to Approve, Reject, or Edit. `Cancel` is `NativeNoDecision`; every unexpected result is a fixed `SourceFailed(UnexpectedDialogResult)`.
- Rendered every trusted preview fact before the final affected-title row, rejected the exact approved presentation-format set, allowed ordinary non-ASCII text, and capped the complete message at 1,024 Unicode scalar values.
- Added a standalone main-thread example using the public gateway -> schema -> policy -> approval -> native source -> manager-resolution path. It performs no action or persistence and prints only bounded approval identity plus a fixed terminal label.
- Added 16 approval unit tests and two approval-binding integration tests for exact layout, every denied code point, ordinary non-ASCII, message limit, every dialog result, cross-manager collision, identity mismatch, replay, late cancellation, expiry equality, Edit freshness, capacity, overflow, and redaction.

## Dependency review

Added exactly:

```toml
[target.'cfg(target_os = "macos")'.dependencies]
rfd = { version = "=0.17.2", default-features = false }
```

- `rfd 0.17.2` is MIT licensed.
- The lockfile diff adds only `rfd`; its native transitive crates were already resolved.
- The macOS target feature tree and duplicate graph were reviewed.
- The dependency's AppKit, raw-handle, and native `unsafe` internals are a new trust boundary, but application `unsafe` remains forbidden.
- The wrapper does not use `set_parent`, raw handles, dependency `Debug`, file-open/save APIs, a Tauri plugin, JavaScript API, command registration, or WebView capability.

## Exact files changed

Created runtime/test files:

```text
src-tauri/src/approvals/decision_source.rs
src-tauri/examples/native_approval_dialog.rs
```

Changed runtime/test files:

```text
src-tauri/src/approvals/mod.rs
src-tauri/src/approvals/types.rs
src-tauri/src/approvals/manager.rs
src-tauri/tests/approval_binding.rs
src-tauri/Cargo.toml
src-tauri/Cargo.lock
```

Changed closeout documentation:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/04e-trusted-approval-decision-source.md
docs/plans/04e-trusted-approval-decision-source.md
```

D-025 records the exact native-source boundary and the project-owner-approved scoped RustSec baseline exception. `TROUBLESHOOTING_LOG.md` remains unchanged because it was outside the approved closeout file list and no repository defect was diagnosed.

## Verification classification

### Passed

```text
cargo check --manifest-path src-tauri/Cargo.toml
  passed
cargo check --manifest-path src-tauri/Cargo.toml --lib --locked
  passed
cargo tree --manifest-path src-tauri/Cargo.toml --target aarch64-apple-darwin --locked -p rfd -e features
  passed; resolved features reviewed
cargo tree --manifest-path src-tauri/Cargo.toml --target aarch64-apple-darwin --duplicates --locked
  passed; duplicates reviewed
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
  passed
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
  passed
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::
  16 passed; 0 failed
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
  2 passed; 0 failed
npm run verify
  passed
  frontend: 124 passed
  Rust library: 92 passed
  Rust integration: 10 passed
  TypeScript, Vite production builds, and Tauri release no-bundle build passed
npm audit --audit-level=low
  passed after network-enabled retry; 0 vulnerabilities
```

Also passed:

- exact dependency source and MIT-license review;
- lockfile review proving Increment 4E adds only `rfd`;
- stale `ApprovalChoice` and raw `.decide` Rust-symbol search;
- focused code review and security review with no 4E runtime-boundary finding; and
- final `npm run format:check` and `git diff --check`;
- secret-pattern scan of all 17 changed files with no matches;
- exact scope/status review with no generated files, databases, build output, or unrelated paths; and
- complete tracked and untracked diff review with no accidental scope expansion.

### Failed

Exact RustSec scan:

```text
CARGO_HOME=/private/tmp/ai-agent-assistant-cargo-audit-home \
  /private/tmp/ai-agent-assistant-cargo-audit/bin/cargo-audit \
  audit --file src-tauri/Cargo.lock
  exited 1
```

Reported:

- RUSTSEC-2026-0194 in `quick-xml 0.39.4`: quadratic duplicate-attribute checking.
- RUSTSEC-2026-0195 in `quick-xml 0.39.4`: unbounded namespace declarations in `NsReader`.
- 18 allowed warnings for unmaintained or unsound transitive crates.

Review established that these findings predate Increment 4E: the 4E lockfile diff adds only `rfd`, while `quick-xml` is reached through existing `plist 1.9.0 -> Tauri`. The existing `plist` source uses plain `quick_xml::Reader`, not `NsReader`, and does not iterate attributes, so the cited APIs appear unreachable through that path. This is a source-based reachability assessment, not a passing RustSec result. No ignore, Tauri/plist upgrade, or lockfile remediation was added.

D-025 records the project owner's scoped baseline exception for these two findings. It permits Increment 4E completion without calling the scanner passed, declaring the advisories fixed, or treating `quick-xml` as generally safe. Re-review is mandatory if the affected APIs become reachable or the dependency path changes.

The first sandboxed `npm audit --audit-level=low` attempt also failed on DNS resolution. The required network-enabled retry passed with zero vulnerabilities, so the final npm audit result is passed.

The first secret-scan invocation omitted the `rg --` option terminator and failed because the private-key pattern began with hyphens. The corrected command passed with no matches; this was command syntax, not a repository finding.

### Checks not run

- No packaged application gate was run; packaging is outside Increment 4E and `npm run verify` already passed the Tauri release no-bundle build.
- No shipping Tauri/WebView interaction test was run because Increment 4E adds no shipping-app wiring.
- No non-macOS cross-target runtime launch was run; `cfg(target_os = "macos")` and the fixed fallback compiled under all-target Clippy/build checks.
- No RustSec ignore or dependency remediation command was run because the approved disposition is a documented baseline exception with no repository suppression or dependency change.

### Manual verification

The command below compiled and opened the native prompt:

```bash
cargo run --manifest-path src-tauri/Cargo.toml --example native_approval_dialog --locked
```

Owner results are now recorded: Approve -> `approved`, Reject -> `rejected`, Edit -> `cancelled: edit requested`, and Return/default -> `rejected`. Escape had no effect and no window-close control was available. The final Edit run exited successfully.

The project owner confirmed the fixed window title, exact trusted-fields-first/title-last content, post-resolution terminal redaction, no action or persistence, no shipping-app change, and no operating-system permission prompt all passed.

## Residual risks

- The visible native prompt cannot be programmatically closed after manager cancellation. Manager state invalidates the subject immediately and rejects every late outcome, but stale visible UI remains possible. Production orchestration is excluded.
- Native interaction proves only that this Rust-owned source returned a recognized button. It does not establish who interacted, LocalAuthentication, device-owner presence, active-run state, dispatch eligibility, or execution authority.
- The exact display deny set is intentionally not a complete Unicode confusable, normalization, font, or visual-width analysis.
- The two pre-existing RustSec advisories remain present under D-025's scoped reviewed baseline exception and require re-review if reachability or the dependency path changes.
- The target-platform manual gate is complete. Escape/close behavior is recorded above and did not create an approval path.

## Exact next task

Review and publish the completed Increment 4E branch only when the project owner explicitly requests commit and push. Do not start another increment until the project owner selects and approves its exact planning scope.

Do not edit runtime files, start a later increment, commit, or push unless explicitly requested.

## Ready-to-paste resume prompt

```text
Use $documentation-sync.

Resume from HANDOFF.md on branch phase4/increment-4e. Re-read the required repository, security, decision, increment, and plan documents and reconcile them with Git. Increment 4E is verified complete under D-025's scoped reviewed RustSec baseline exception; do not rerun the native gate or change runtime files unless evidence changes. Review the complete uncommitted diff and final status, then commit and push only if the project owner explicitly requests it. Do not start another increment until its exact planning scope is selected and approved.
```
