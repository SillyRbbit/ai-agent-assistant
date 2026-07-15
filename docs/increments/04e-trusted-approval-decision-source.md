# Phase 4 Increment 4E - trusted approval-decision source

Last updated: 2026-07-14

Status: **Verified complete on target Mac**

## Goal

Add one Rust-owned macOS native approval interaction source for the existing exact `create_local_task@1` approval subject. The source must consume a manager-issued one-shot presentation, return a sealed exact-subject source outcome, and let the manager terminally approve, reject, cancel for edit/no-decision/source failure, or expire the subject without adding WebView authority, audit, dispatch, execution, provider continuation, persistence, or live orchestration.

## Planning result

- Increment 4D accepts a public `ApprovalChoice` plus `ApprovalId`. That proves local manager state only; it does not identify a trusted interaction source or prevent arbitrary Rust callers from supplying a choice.
- The React approval dialog is intentionally mock-only WebView state. No frontend event or Tauri command may be promoted into production approval authority.
- `get_app_info` remains the only custom Tauri command, `core:default` remains the only capability permission, and no native dialog or LocalAuthentication dependency exists.
- The product requires an exact preview with Approve, Reject, and Edit. The current manager can derive those facts, but its borrowed view cannot safely remain alive while a native interaction is pending and cancellation remains possible.
- The smallest coherent runtime increment adds one manager-issued, owned, one-shot presentation and one Rust-only macOS native message-dialog source. No Tauri dialog plugin is registered, so no dialog invoke command or WebView permission is added.
- The presentation carries a private in-memory manager-instance marker, exact approval ID and run/request/call identity, one bounded transient application-owned clone of the already-validated task title, and closed local preview facts. The source moves the marker into its sealed outcome, and the manager checks pointer identity before public IDs so an outcome cannot cross manager instances. The source's bounded formatted message and native rendering storage may briefly add transient title copies; none survives in the source outcome, logs, errors, debug output, audit, or persistence. The presentation grants no authority and can be issued only once for a pending subject.
- The native source consumes the presentation and returns a non-cloneable, non-serializable, redacted source outcome whose constructor is private to that source. The manager verifies the complete identity again before resolving it.
- A recognized Approve/Reject/Edit custom-button result is evidence only that the Rust-owned native source returned that button label from the dialog it created. `NativeNoDecision` and `SourceFailed` carry no user-intent evidence. No source outcome identifies the actor, proves biometric or device-owner authentication, establishes run liveness, or grants execution authority.
- Approve maps to `Approved`; Reject maps to `Rejected`; Edit maps to typed terminal cancellation. The dependency's `Cancel` result is conservatively `NativeNoDecision`, not proof that a person dismissed the prompt, because it can also represent an unmatched native result. Edit never mutates an existing subject and requires a fresh gateway call, validation, policy decision, approval ID, and presentation.
- Expiry, explicit run cancellation, identity mismatch, already-consumed state, and replay take precedence over any late native result. The selected dialog library cannot programmatically close an already-visible prompt, so a stale prompt may remain visible until dismissed even though its result is guaranteed to fail closed. Live orchestration is explicitly excluded.
- LocalAuthentication is not added. The only current subject is a reversible local task with no required permission. A later high-risk policy may require a separate exact-subject device-owner-authentication adapter after native Approve and before any execution gate.
- The exact execution plan is `docs/plans/04e-trusted-approval-decision-source.md`.

## Planning baseline

```text
branch: phase4/increment-4e
base: clean merged main at 1cf190f
origin/main: 1cf190f
Node.js: v26.3.0
npm: 11.16.0
Cargo: 1.90.0
Rust: 1.90.0
rustfmt: 1.8.0-stable
Clippy: 0.1.90
macOS: 26.5.2 (25F84)
npm run typecheck: passed
approval library baseline: 6 passed
approval-binding integration baseline: 2 passed
platform library baseline: 3 passed
menu-bar library baseline: 9 passed
menu-bar routing integration baseline: 3 passed
working tree before planning edits: clean
```

Repository inspection also confirmed:

- no production caller of `ApprovalManager` exists;
- no dialog plugin, native approval adapter, or LocalAuthentication adapter exists;
- no WebView command can reach approval state;
- the frontend dialog and choices remain explicitly mock-only;
- `src-tauri/capabilities/default.json` grants only `core:default`; and
- `unsafe` remains forbidden in the application crate.

## Implementation result

- Project-owner approval was received for the exact plan and eight-file runtime/test scope.
- The public raw `ApprovalChoice` path is removed. `ApprovalManager` issues one owned non-cloneable `ApprovalPresentation`, and only the macOS source can privately create the sealed non-cloneable `TrustedApprovalSourceOutcome` consumed by manager resolution.
- The manager marker moves manager -> presentation -> source outcome and is checked by `Arc::ptr_eq` before exact approval/run/gateway-request/function-call identity. Manager-retained issuance state remains one-shot and does not extend the 120-second deadline.
- `ApprovalDisposition` now carries closed `RunTerminated`, `EditRequested`, `NativeNoDecision`, and `SourceFailed` cancellation reasons plus structured source, optional recognized-button, `NotEvaluated` authentication, and optional fixed source-failure evidence.
- The macOS source builds the fixed-field-first/title-last message, enforces the exact presentation-safety set and 1,024-Unicode-scalar cap, configures Reject/Approve/Edit with Reject first/default, and maps every dependency result to a closed outcome.
- The standalone example exercises the public gateway -> schema -> policy -> approval manager -> native source -> manager resolution path from the process main thread. It performs no action or persistence and prints no task content.
- Sixteen approval unit tests and two approval-binding integration tests cover all planned identity, lifecycle, mapping, formatting, capacity, replay, and redaction behavior.

## Exact implementation files

Create:

```text
src-tauri/src/approvals/decision_source.rs
src-tauri/examples/native_approval_dialog.rs
```

Change:

```text
src-tauri/src/approvals/mod.rs
src-tauri/src/approvals/types.rs
src-tauri/src/approvals/manager.rs
src-tauri/tests/approval_binding.rs
src-tauri/Cargo.toml
src-tauri/Cargo.lock
```

Implementation closeout may update only:

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

Stop and request approval before changing any other file.

## Planned boundary

- Replace the public raw-choice manager path with a manager-issued one-shot presentation and a sealed source-produced outcome.
- Retain the existing one-pending, 1,024-subject manager-lifetime, 120-second monotonic TTL, and non-evicting replay limits.
- Permit exactly one presentation attempt per pending subject. A source failure terminally cancels that subject; retry requires a fresh validated call.
- Keep a private pointer-identical manager-instance marker plus exact run/request/call identity in both the manager and the source outcome, and compare every field before terminal resolution. The marker uses `Arc` only and adds no digest, randomness, global counter, dependency, or content.
- Keep the affected title bounded by the existing 200-Unicode-scalar schema contract and reject any presentation that cannot be derived from the registered closed preview.
- Build at most 1,024 Unicode scalar values of fixed native message copy. Render every trusted policy fact before the final clearly labeled task-title line, and reject the plan's exact closed zero-width/default-ignorable/line/bidirectional presentation set before display so untrusted title text cannot disappear, reorder, or fabricate policy rows. Ordinary non-ASCII text remains allowed.
- Show fixed product preview fields and fixed Approve, Reject, and Edit choices in one native macOS dialog.
- Configure Reject as the first/default native button. Return, Escape, close, no-decision, and failure paths must never approve.
- Keep the dialog source Rust-only. Add no Tauri command, event, plugin registration, capability permission, CSP change, or frontend path.
- Record source and no-authentication evidence on the terminal resolution as structured non-authorizing data. Store no OS error text, title, or arbitrary dialog copy in errors or debug output.
- Replace the undifferentiated cancellation disposition with exactly `RunTerminated`, `EditRequested`, `NativeNoDecision`, and `SourceFailed` reasons. No caller-supplied cancellation text is accepted.
- Preserve explicit manager cancellation while a dialog is outstanding. A late source result is rejected as consumed even if the native window could not be closed.
- Leave audit disconnected. A future typed audit adapter may receive opaque identity, source, authentication evidence, an optional recognized button, source-failure code, cancellation reason, and disposition fields, but not the raw title.

## Dependency decision

The implementation adds one exact macOS-target dependency:

```toml
[target.'cfg(target_os = "macos")'.dependencies]
rfd = { version = "=0.17.2", default-features = false }
```

Direct `rfd` use provides a native message dialog without registering the Tauri dialog plugin's WebView invoke commands or default dialog permissions. The dependency remains target-specific and is used only for the closed message-dialog adapter; file-open and file-save APIs are not wrapped, exported, registered, or granted to the WebView. Its native `unsafe` internals and raw-handle support are a new dependency trust boundary even though application `unsafe` remains forbidden; the wrapper passes no raw parent handle and exposes no dependency clone/debug path. Source, MIT license, target feature tree, duplicate graph, lockfile, native boundary, compatibility, and target-Mac behavior were reviewed. The lockfile diff adds only `rfd`; D-025 accepts this exact dependency boundary.

## Risks

- A WebView decision could be mistaken for production authority.
- A caller could present one subject and apply the choice to another.
- An outcome from one manager could collide with the same approval/run/request/call IDs in another manager that retained different content.
- A presentation or source outcome could be cloned, serialized, replayed, or reused after cancellation.
- Edit could mutate an already-approved subject rather than starting over.
- A native response could be overstated as device-owner authentication.
- LocalAuthentication availability could be mistaken for successful authentication or approval intent.
- Dialog text, title copies, OS errors, or dependency internals could leak through debug, logs, panic output, or audit.
- Schema-valid invisible/default-ignorable, bidirectional, or line-separator formatting could hide title content or visually spoof fixed native preview fields.
- Unicode confusables, font behavior, normalization, or visual width outside the exact deny set could remain ambiguous in a flat native message.
- A blocking native prompt could freeze the application or prevent timely cancellation.
- The selected native dialog API cannot programmatically close a prompt after run cancellation, leaving stale visible UI.
- A direct dependency could broaden platform support or expose file-dialog capability unintentionally.
- An approved resolution could be connected directly to execution before run-state, dispatch, executor, and audit gates exist.

The plan mitigates these risks with no WebView route, one owned presentation, a private pointer-identical manager marker, exact identity comparison, sealed non-cloneable outcomes, one-time terminal manager state, typed cancellation reasons, explicit no-authentication evidence, closed redacted errors, fixed-field-first display order, an exact closed zero-width/default-ignorable/line/bidirectional presentation set, a 1,024-character message cap, a macOS-target exact dependency, focused adversarial tests, a target-Mac dialog gate, and no production caller or execution conversion. The stale-visible-dialog limitation remains an explicit accepted risk for this unconnected source increment and must be revisited before live orchestration.

## Verification gate

Automated:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
npm run verify
npm audit --audit-level=low
git diff --check
```

Rust advisory gate after the exact lockfile exists:

```bash
CARGO_HOME=/private/tmp/ai-agent-assistant-cargo-audit-home cargo install cargo-audit --version 0.22.2 --locked --root /private/tmp/ai-agent-assistant-cargo-audit
CARGO_HOME=/private/tmp/ai-agent-assistant-cargo-audit-home /private/tmp/ai-agent-assistant-cargo-audit/bin/cargo-audit audit --file src-tauri/Cargo.lock
```

The temporary verification-tool install requires network approval and must not modify the repository. Increment 4E cannot complete if the RustSec scan is unavailable or has an unreviewed advisory.

Target-Mac manual gate:

```bash
cargo run --manifest-path src-tauri/Cargo.toml --example native_approval_dialog --locked
```

The project owner must confirm the native prompt uses the fixed `Cortexa approval` title, shows the exact task title last after every fixed preview field, exposes exactly Approve/Reject/Edit buttons, maps each button correctly when the example is rerun, makes Return/default activation reject, maps any available Escape/window-close result to `NativeNoDecision` rather than user intent or approval, renders no raw content after resolution, requests no operating-system permission, and performs no action. The complete diff must also pass code review, security review, exact temporary RustSec audit, dependency/scope/secret/generated-output checks, and documentation synchronization.

## Verification result

Passed:

- `cargo check --manifest-path src-tauri/Cargo.toml` and the locked library check;
- target `rfd` feature-tree and duplicate-tree review;
- rustfmt and Clippy for all targets/all features with warnings denied;
- focused approval tests: 16 library and two integration tests;
- `npm run verify`: 124 frontend tests, 92 Rust library tests, ten Rust integration tests, TypeScript, Vite production builds, and Tauri release no-bundle build;
- `npm audit --audit-level=low`: zero vulnerabilities after retrying the initial sandboxed DNS failure with network access; and
- stale-symbol, dependency, scope, code, and security review with no 4E runtime finding.

Failed, reviewed, and accepted as a scoped baseline exception:

- Exact temporary `cargo-audit 0.22.2` exited nonzero on RUSTSEC-2026-0194 and RUSTSEC-2026-0195 in pre-existing `quick-xml 0.39.4`. Increment 4E adds only `rfd`; the existing path is `plist 1.9.0 -> Tauri`. Source review found plain `quick_xml::Reader`, not `NsReader`, and no attribute iteration on that path. D-025 records the project owner's scoped reviewed baseline exception; it does not declare the advisories fixed or generally safe. No advisory ignore or dependency upgrade was added.

Manual verification results:

- The target-Mac manual gate passes. Approve -> `approved`, Reject -> `rejected`, Edit -> `cancelled: edit requested`, and Return/default -> `rejected`; Escape had no effect and no window-close control was available. The project owner confirmed fixed title/content/order, post-resolution terminal redaction, no action or persistence, and no permission prompt.

Checks not run:

- No packaging or shipping-app interaction gate was run because both remain explicit non-goals and no shipping Tauri path changed.

## Explicit non-goals

- No production orchestrator or Tauri application wiring.
- No WebView approval command, event, listener, capability, permission, CSP, or frontend change.
- No Tauri dialog plugin, generic native-dialog wrapper, file chooser, or file-system access.
- No LocalAuthentication, Touch ID, Apple Watch, device-password prompt, authentication policy, or authentication claim.
- No audit call, audit persistence, SQLite change, task persistence, dispatch, executor, tool implementation, or provider continuation.
- No gateway networking, credential, Keychain, identity, deployment, live model, or retention work.
- No multiple pending approvals, prompt retry, generic arbitrary-tool preview, preview editing, or mutable approval subject.
- No claim that approval ID, native response, authentication result, local disposition, or future digest is execution authority.
- No programmatic native-dialog cancellation guarantee in this increment.

## Rollback

Remove the native source and manual example, restore the raw transport-free choice API and existing cancellation shape, remove the exact target-specific dependency and lockfile entries, and restore the prior approval tests. Preserve the verified 4A-4D gateway, schema, policy, identity, preview, TTL, capacity, and replay behavior plus all Tauri, frontend, capability, CSP, storage, provider, and audit boundaries.

## Completion result

Implementation, focused and complete automated verification, dependency review, code review, security review, documentation synchronization, and the target-Mac manual gate are complete within the approved scope. D-025 records the exact native-source boundary and the project-owner-approved scoped RustSec baseline exception. Increment 4E implementation commit `b0a3036` was pushed, fast-forward merged into `main`, and published. No later increment was started.
