# Execution plan - Increment 4E trusted approval-decision source

Status: **Complete**

Owner: Project maintainer

## Execution status

The project owner approved this exact plan and eight-file runtime/test scope. Implementation is complete without scope expansion:

- one manager-issued owned presentation and one sealed macOS source outcome replace the public raw-choice path;
- private pointer-identical manager binding and exact approval/run/request/call/presentation checks are implemented;
- exact preview layout, presentation-safety filtering, 1,024-Unicode-scalar limit, Reject-first button order, closed native-result mapping, typed cancellation, redaction, and no-authentication evidence are implemented;
- exact macOS-target `rfd = "=0.17.2"` is present with default features disabled; and
- the main-thread non-executing example and 18 focused approval tests are present.

Focused checks, rustfmt, Clippy, `npm run verify`, `npm audit --audit-level=low`, dependency review, code review, security review, and the project-owner native-dialog interaction matrix pass. Exact `cargo-audit 0.22.2` remains nonzero on two reviewed pre-existing `quick-xml 0.39.4` advisories; D-025 records the project-owner-approved scoped baseline exception without an advisory ignore or dependency change. Increment 4E is verified complete.

## Gap analysis

The verified Increment 4D manager owns one exact approval subject and consumes it once, but its public `decide(id, ApprovalChoice)` API accepts a raw Rust choice from any caller. There is no production caller, so `Approved` currently proves only that a test or future trusted Rust caller supplied that enum while the subject was pending and unexpired.

The React `ApprovalDialog` cannot close this gap. It is intentionally mock-only, runs inside the untrusted WebView, and has no IPC route to the Rust approval manager. Promoting its choices would let compromised rendered content or frontend state assert authority. `get_app_info` remains the only custom Tauri command and the current capability grants only `core:default`.

The product brief requires a preview showing what will happen, target, exact affected data, date/time, recipients, reversibility, required permission, main risk, and Approve/Reject/Edit choices. Increment 4D already derives those facts from the retained policy decision, but exposes them through a borrowed view. A native interaction cannot safely retain that borrow while the manager remains available for run cancellation and expiry.

The smallest coherent next increment is therefore one Rust-owned, macOS-native choice source plus the minimum manager handoff required to use it safely. It remains disconnected from Tauri commands, live orchestration, audit, dispatch, execution, persistence, and provider continuation.

## Primary-source findings

- The [Tauri dialog plugin documentation](https://v2.tauri.app/plugin/dialog/) exposes Rust and JavaScript APIs and documents default dialog permissions. Its [Rust plugin source](https://docs.rs/tauri-plugin-dialog/latest/src/tauri_plugin_dialog/lib.rs.html) registers open, save, and message invoke handlers. Registering that plugin would add an unnecessary WebView-facing command family even if capabilities attempted to deny it.
- [`rfd` 0.17.2](https://docs.rs/rfd/0.17.2/rfd/) provides native message dialogs without requiring Tauri plugin registration. Its closed [`MessageButtons`](https://docs.rs/rfd/0.17.2/rfd/enum.MessageButtons.html) and [`MessageDialogResult`](https://docs.rs/rfd/0.17.2/rfd/enum.MessageDialogResult.html) contracts provide three custom button labels plus a distinct `Cancel` result. Its macOS guidance recommends opening dialogs on the main thread. The proposed dependency is macOS-target-only and the application will wrap only its synchronous message-dialog API.
- [Apple Local Authentication](https://developer.apple.com/documentation/localauthentication) evaluates local authentication policy. [`deviceOwnerAuthentication`](https://developer.apple.com/documentation/localauthentication/lapolicy/deviceownerauthentication) may use available device-owner methods, but a successful evaluation does not itself display or bind the exact action preview.
- The [official Tauri plugin support page](https://v2.tauri.app/plugin/) describes its biometric plugin as Android/iOS, not a desktop macOS approval primitive. No existing repository adapter supplies LocalAuthentication.
- [RustSec](https://rustsec.org/) identifies `cargo-audit` as the lockfile advisory scanner. No Rust advisory tool is installed or configured in this repository; current exact [`cargo-audit` 0.22.2](https://docs.rs/crate/cargo-audit/0.22.2) supports the repository's Rust toolchain.

These findings support a direct Rust native-message-dialog source now and a separately approved LocalAuthentication adapter only when policy requires device-owner authentication for a higher-risk exact subject.

## Goal and outcome

Replace the raw choice input with a manager-issued one-shot `ApprovalPresentation` and a source-produced sealed `TrustedApprovalSourceOutcome` for `create_local_task@1`.

The user-visible outcome is limited to the manual verification harness: a native macOS prompt displays the exact validated task title and fixed preview facts, offers Approve, Reject, and Edit, and performs no action. The shipping Tauri application remains unchanged because no production orchestrator is in scope.

## Trust and authorization path

The complete path after this increment is:

```text
untrusted normalized gateway call
  -> exact local schema validation
  -> deterministic policy retaining exact input
  -> one pending ApprovalManager subject
  -> one manager-issued owned presentation
  -> Rust-owned macOS native dialog
  -> sealed exact-binding native source outcome
  -> manager rechecks identity, pending state, and monotonic expiry
  -> terminal non-authorizing ApprovalResolution
```

There is still no path from the terminal resolution to audit, dispatch, execution, persistence, WebView, or provider continuation. Neither the WebView nor model can construct a presentation or source outcome.

## Manager-issued one-shot presentation

Add an owned, non-cloneable, non-serializable, debug-redacted `ApprovalPresentation` created only by the manager for its current pending approval.

It contains:

```text
approval ID
private in-memory manager-instance marker
verified run ID
verified gateway-request ID
validated function-call ID
local tool name and contract version
closed policy, risk, and permission facts
one bounded copy of the validated local-task title
fixed target, schedule, recipient, reversibility, and risk facts
remaining monotonic lifetime at issuance
```

The manager-instance marker is a private `Arc` allocation owned by the manager and cloned only into its one presentation. The source moves that same private marker into the sealed outcome; callers cannot read, construct, serialize, or replace it. The manager checks `Arc::ptr_eq` before any ID comparison. This prevents an outcome from one manager instance from resolving another manager's same-numbered approval without introducing randomness, a digest, a global counter, or action content.

The presentation is correlation and display data, not authority. The manager marks the subject as presentation-issued before returning it. A second issuance attempt fails closed. The title copy is unavoidable for an owned native handoff, remains bounded by the existing 200-Unicode-scalar schema contract, is consumed by the source, and is not retained in the returned source outcome.

The manager must keep the original policy decision and deadline. Issuing a presentation does not pause or extend the 120-second TTL. The presentation cannot resolve itself and has no manager reference, executor conversion, serialization, or public constructor.

## Native source and outcome

Add a macOS-gated `MacOsNativeApprovalDecisionSource` in `src-tauri/src/approvals/decision_source.rs`. It consumes one `ApprovalPresentation`, derives all copy locally, and displays one synchronous native message dialog through direct `rfd` use. The module export, dependency import, native implementation, and interactive example body are all `cfg(target_os = "macos")`; the example has a fixed non-interactive non-macOS fallback so all-target builds remain portable. The window title is fixed to `AI Agent Assistant approval`; untrusted content cannot alter it. The source is called only from the standalone example on the process main thread in this increment. Shipping-app scheduling and modality remain future orchestration work.

The dialog must show these trusted facts first:

- action: create one local task;
- target: local task list;
- schedule/date-time: none;
- recipients: none;
- reversibility: reversible;
- required permission: none;
- main risk: creates a local task; and
- fixed Approve, Reject, and Edit choices.

The exact affected task title is the only untrusted display field. Put it in a clearly labeled final message row after every trusted policy fact; never interpolate it into the native window title, explanatory copy, or button labels. Before display, reject this exact closed set:

```text
U+00AD
U+034F
U+061C
U+115F-U+1160
U+17B4-U+17B5
U+180B-U+180F
U+200B-U+200F
U+2028-U+202E
U+2060-U+206F
U+3164
U+FE00-U+FE0F
U+FEFF
U+FFA0
U+FFF9-U+FFFB
U+1BCA0-U+1BCA3
U+1D173-U+1D17A
U+E0000-U+E0001
U+E0020-U+E007F
U+E0100-U+E01EF
```

This presentation-only set covers soft hyphen, grapheme joiner, fillers, zero-width and bidirectional formatting, line/paragraph separators, variation selectors, shorthand controls, musical annotation controls, and tag characters that could hide content or reorder/fabricate preview rows. Ordinary non-ASCII text remains allowed. A rejection produces a sealed `SourceFailed(UnsafePresentationFormatting)` outcome and terminally cancels the subject; it does not modify the accepted schema contract. A display-unsafe title requires a fresh request with safely renderable text.

The manager retains the original typed title, the presentation owns one bounded clone, the source builds one bounded message string, and the native framework may retain transient rendering storage. Those copies can briefly coexist. The source must drop its presentation and message storage immediately after the native result or source failure, retain no title in `TrustedApprovalSourceOutcome`, and make no unsupported memory-zeroization claim. No copy enters logs, errors, debug output, audit, persistence, or terminal output.

Configure exactly `MessageButtons::YesNoCancelCustom("Reject", "Approve", "Edit")`. `rfd`/AppKit treats the first button as the default, so the default is fail-closed Reject rather than Approve. Map only these recognized results:

| `MessageDialogResult`       | Closed source outcome | Manager disposition           |
| --------------------------- | --------------------- | ----------------------------- |
| `Custom("Approve")`         | Approve               | `Approved`                    |
| `Custom("Reject")`          | Reject                | `Rejected`                    |
| `Custom("Edit")`            | Edit                  | `Cancelled(EditRequested)`    |
| `Cancel`                    | NativeNoDecision      | `Cancelled(NativeNoDecision)` |
| `Yes`, `No`, `Ok`, or other | SourceFailed          | `Cancelled(SourceFailed)`     |

`Cancel` covers any operating-system dismissal result the library reports, but the dependency can also use it as a fallback for an unmatched native return. It therefore records no user intent or user-presence evidence. Do not assume macOS exposes a close control or that Escape is available; the target-Mac gate must record actual behavior. Return/default activation must resolve Reject, and no Escape, close, no-decision, or failure path may ever map to Approve.

The source consumes the presentation and privately constructs one `TrustedApprovalSourceOutcome` containing the private manager-instance marker, approval ID, exact run/request/call binding, one closed outcome, source kind, and fixed no-authentication evidence. It retains no title and exposes no public constructor or field mutation. The value does not implement `Clone`, `Copy`, serialization, deserialization, or raw derived `Debug`.

## Source-outcome and evidence semantics

The manager replaces `decide(id, ApprovalChoice)` with a method that consumes `TrustedApprovalSourceOutcome`. Before any terminal transition it checks:

1. a request is pending;
2. the private manager-instance marker is pointer-identical;
3. the approval ID matches;
4. run ID matches;
5. gateway-request ID matches;
6. function-call ID matches;
7. a presentation was issued exactly once;
8. the request is not cancelled or consumed; and
9. current manager time is strictly before the existing deadline.

Manager-instance, source-kind, or identity mismatch returns a redacted error without mutating an unrelated pending subject; the consumed mismatched outcome cannot be reused. Its originating subject has already issued its only presentation and can only be cancelled or expire. For an exact matching subject, deadline equality is expired before outcome mapping, matching Increment 4D.

An interaction-backed resolution may expose closed structured evidence for the three recognized button outcomes:

```text
source: MacOsNativeDialog
native button: Approve | Reject | Edit
authentication: NotEvaluated
```

`MacOsNativeDialog` plus a button means only that this Rust-owned adapter received one recognized custom-button result from the native dialog it created from one manager-issued presentation. It is product-level local interaction evidence, not a cryptographic user-presence assertion. `NativeNoDecision` and `SourceFailed` carry no button or user-intent evidence. `NotEvaluated` explicitly means no Touch ID, Apple Watch, device password, biometric, or device-owner identity claim was made.

## Cancellation, expiry, edit, and replay

- Preserve explicit manager cancellation and add a closed cancellation reason rather than an arbitrary string.
- The closed cancellation reasons are exactly `RunTerminated`, `EditRequested`, `NativeNoDecision`, and `SourceFailed`. `ApprovalDisposition::Cancelled` retains one of those reasons; no caller supplies descriptive text.
- The public orchestration hook is a fixed run-termination cancellation by approval ID and can produce only `RunTerminated`. The sealed native source outcome alone can produce `EditRequested`, `NativeNoDecision`, or `SourceFailed`.
- Run end or run cancellation consumes the exact pending subject even while a native prompt is outstanding.
- Expiry consumes the subject at or after the existing monotonic deadline.
- A source outcome arriving after cancellation, expiry, or another terminal transition returns `AlreadyConsumed`; it cannot recreate or approve the subject.
- Edit is a terminal cancellation reason. It does not alter the title, policy decision, presentation, or approval. Any edited request must re-enter through a fresh gateway call, strict protocol validation, exact local schema validation, policy evaluation, approval ID, and presentation.
- A dependency `Cancel` result is terminal `NativeNoDecision`; it never defaults to Reject, Edit, or Approve and makes no user-intent claim.
- Unsafe presentation formatting, message-limit overflow, or unexpected result mapping is terminal `SourceFailed`; no prompt retry is allowed for that subject.
- A source outcome is consumed by value and cannot be cloned. The manager's existing non-evicting subject tombstone rejects recreation for the manager lifetime.
- The selected native dialog API does not provide a programmatic close handle. Manager cancellation therefore invalidates the subject immediately, but a visible stale prompt may remain until the person dismisses it. Its eventual result fails closed. No live orchestration may use this source until that UX limitation is explicitly accepted or replaced.

## Closed limits

Retain:

```text
pending approvals per manager: 1
distinct subjects per manager lifetime: 1,024
approval lifetime: 120 monotonic seconds
presentations per subject: 1
native prompts per presentation: 1
native buttons: Approve, Reject, Edit
closed source outcomes: Approve, Reject, Edit, NativeNoDecision, SourceFailed
complete native message: at most 1,024 Unicode scalar values
authentication attempts: 0
audit writes: 0
execution attempts: 0
```

The existing title maximum remains 200 Unicode scalar values. All other dialog text is fixed application copy. The implementation must calculate the complete message limit before calling the dependency and fail closed if fixed-copy changes ever exceed it. It must not introduce arbitrary labels, error messages, icon paths, parent handles, URLs, or file-dialog parameters.

## Closed failures and redacted errors

Add exactly these closed source-failure codes inside a sealed `SourceFailed` outcome:

- `UnsafePresentationFormatting`;
- `MessageLimitExceeded`; and
- `UnexpectedDialogResult`.

Extend closed manager errors only as needed for:

- presentation already issued;
- presentation unavailable or expired;
- source-outcome identity mismatch;
- manager-instance mismatch; and
- source kind mismatch.

Failures and errors may expose fixed codes, bounded numeric approval IDs, and fixed limits. They must not expose the title, prompt text, run/request/call strings, OS error text, dependency debug output, paths, window labels, or personal content. Production code must not log or print them. The manual example may print only fixed outcome labels and bounded approval IDs.

## LocalAuthentication boundary

Do not add LocalAuthentication in Increment 4E. The only registered approval subject is `create_local_task@1`, classified as a reversible local action with `PermissionKind::None`; the product does not require device-owner authentication for that current action.

For future higher-risk subjects:

1. deterministic policy must state that device-owner authentication is required for that exact subject;
2. the native preview and explicit Approve intent must occur first;
3. a separate trusted adapter must call `canEvaluatePolicy` only as an availability check;
4. only successful `evaluatePolicy(.deviceOwnerAuthentication)` may create a closed `DeviceOwnerAuthenticated` claim;
5. cancellation, fallback, lockout, unavailable policy, context invalidation, or evaluation failure must not approve;
6. the manager/orchestrator must recheck the same approval ID, run/request/call identity, pending state, TTL, and run liveness after authentication; and
7. the authentication claim remains evidence for policy/dispatch, not execution authority by itself.

That adapter, its dependency and platform implementation, exact fallback policy, entitlements, usage disclosure, tests, and audit shape require a separate approved increment.

## Audit boundary

Increment 4E makes no audit call and does not modify the generic audit scaffold. A later typed adapter may consume terminal resolution facts such as opaque run/request/call/approval IDs, local tool/version, risk/permission, policy reason, interaction source, no-authentication/authentication evidence, an optional recognized native button, source-failure code, cancellation reason, disposition, and bounded timing.

It must not copy the task title, prompt text, raw arguments, OS errors, or dependency output into generic strings. Audit completion will still not authorize dispatch or execution.

## Dependency decision

Add exactly:

```toml
[target.'cfg(target_os = "macos")'.dependencies]
rfd = { version = "=0.17.2", default-features = false }
```

Why:

- it supplies the required native message dialog on the target Mac;
- direct use avoids registering Tauri plugin commands, JavaScript initialization, or dialog permissions;
- the exact version follows repository dependency policy;
- macOS target scoping avoids changing non-macOS application dependencies; and
- only a closed message-dialog wrapper is exposed.

The runtime implementation must inspect the resolved lockfile, license, enabled features, duplicate native crates, Rust 1.90 compatibility, and audit result before accepting D-025. File open/save APIs remain unused and inaccessible. If the exact dependency does not pass those checks, stop and request a revised plan; do not substitute another library or add unsafe AppKit code.

`rfd` is a native dependency with platform `unsafe` internals, including raw window-handle transport and AppKit bindings; the application crate's `unsafe_code = "forbid"` does not review dependency internals. Treat the exact crate source and resolved native graph as a new trust boundary. The wrapper must not call `set_parent`, expose raw handles, clone or format the dependency dialog object, or let its raw-content `Debug` implementation reach logs or errors. D-025 must record this accepted dependency boundary only after source, license, tree, audit, and target-Mac behavior review pass.

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

Implementation closeout may change only:

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

No `src-tauri/src/lib.rs`, Tauri configuration, capability, CSP, frontend, storage, policy, schema, registry, audit, executor, provider, or gateway file may change. Stop and request approval before expanding either list.

## Implementation steps

1. Add the exact macOS-target dependency and inspect the lockfile and dependency tree before writing source code.
2. Add one-shot presentation, closed interaction evidence, typed cancellation reasons, and sealed source-outcome types with redacted debug behavior.
3. Update the manager to issue one presentation, remove raw `ApprovalChoice`, consume only a sealed source outcome, verify every identity field, and preserve cancellation/expiry/replay precedence.
4. Add the macOS-gated native source and portable example fallback with fixed preview construction and closed result mapping. Add no Tauri plugin or WebView surface.
5. Extend focused unit and public boundary tests for issuance, exact binding, each button, no-decision/source-failure cancellation, expiry, run cancellation, replay, mismatch, and redaction.
6. Add the non-executing manual example, ensuring it runs the source on the process main thread, prints no title or raw error, and cannot invoke a tool. `cargo clippy --all-targets` must compile it, while normal tests do not launch it.
7. Run focused checks, the full gate, dependency audit, target-Mac manual gate, scope/secret/generated-output checks, code review, security review, and documentation synchronization.
8. Record D-025 and closeout evidence only if every automated and manual completion gate passes.

## Automated test plan

Unit and integration coverage must prove:

- only one presentation can be issued per pending subject;
- presentation data is derived from the exact retained policy decision;
- the presentation carries a private manager-instance marker plus exact approval/run/request/call identity;
- an outcome from another manager fails before ID resolution even when every public ID collides;
- title content is bounded and absent from debug and errors;
- the complete message is bounded, fixed fields precede the title, every code point in the closed presentation-format set fails before display, and ordinary non-ASCII text still passes;
- the first/default native button is Reject, Return cannot approve, and Escape/close/no-decision paths cannot approve;
- raw `ApprovalChoice` no longer exists as a public manager input;
- only the native source can construct a trusted source outcome;
- Approve, Reject, Edit, NativeNoDecision, and SourceFailed map to the exact terminal result;
- Edit requires a fresh validated subject and cannot mutate/reuse the old subject;
- source failure terminally cancels and cannot retry the same subject;
- manager/source/identity mismatch fails without approving or consuming another manager's pending subject, and the mismatched outcome cannot be reused;
- exact deadline equality and later times expire before source-outcome application;
- run cancellation wins over a late native source outcome;
- every terminal outcome rejects source-outcome replay and subject recreation;
- button/source/authentication evidence is exact, no-decision/failure paths carry no user-intent evidence, and `NotEvaluated` cannot be confused with successful authentication;
- errors and custom debug output exclude title, prompt, identity strings, and OS details;
- existing one-pending, 1,024-subject, overflow, and redaction behavior remains; and
- no source type exposes serialization, cloning, IPC, audit, dispatch, or execution conversion.

Exercise sealed outcome construction through private source-module mapping tests, including synthetic `MessageDialogResult` values; do not add a public constructor, Cargo feature, environment bypass, or integration-test-only production API. The public `approval_binding` integration test should cover gateway-to-presentation identity and fail-closed run cancellation. The standalone example is the only full public native-source-to-manager acceptance path.

## Verification commands

Focused during implementation:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked approvals::
cargo test --manifest-path src-tauri/Cargo.toml --test approval_binding --locked
```

Complete gate:

```bash
npm run verify
npm audit --audit-level=low
git diff --check
git status --short
```

Install the exact RustSec scanner into temporary storage only, then audit the resolved Rust lockfile:

```bash
CARGO_HOME=/private/tmp/ai-agent-assistant-cargo-audit-home cargo install cargo-audit --version 0.22.2 --locked --root /private/tmp/ai-agent-assistant-cargo-audit
CARGO_HOME=/private/tmp/ai-agent-assistant-cargo-audit-home /private/tmp/ai-agent-assistant-cargo-audit/bin/cargo-audit audit --file src-tauri/Cargo.lock
```

This verification tool is not a repository or production dependency. Its installation requires network access and explicit approval when executed. If the temporary install or advisory update cannot run, report the Rust advisory scan as not run and do not mark Increment 4E complete.

Review the exact dependency tree and lockfile without modifying them:

```bash
cargo tree --manifest-path src-tauri/Cargo.toml --target aarch64-apple-darwin --locked
cargo tree --manifest-path src-tauri/Cargo.toml --target aarch64-apple-darwin --duplicates --locked
```

Required target-Mac manual gate:

```bash
cargo run --manifest-path src-tauri/Cargo.toml --example native_approval_dialog --locked
```

Rerun the example to verify Approve, Reject, and Edit. Press Return without selecting a button and confirm the result is Reject, never Approve. Attempt Escape and any available window-close control; record whether a no-decision path is available and confirm any `Cancel` result is `NativeNoDecision` with no user-intent evidence, never approval. Confirm the fixed window title, trusted-field-first/title-last preview order, exact content, no raw-content output after interaction, no action or persistence, no permission prompt, and no shipping Tauri/WebView behavior change.

The complete diff must then pass:

- exact file-scope and stale-symbol checks;
- secret, credential, generated-output, database, and build-artifact checks;
- dependency and license review;
- RustSec lockfile audit with no unreviewed advisory ignore;
- code review;
- security review; and
- documentation synchronization review.

Current evidence:

- Approval library tests: 16 passed.
- Approval-binding integration tests: two passed.
- `npm run verify`: passed with 124 frontend tests, 92 Rust library tests, ten Rust integration tests, TypeScript, Vite production builds, and Tauri release no-bundle build.
- `npm audit --audit-level=low`: passed with zero vulnerabilities after a network-enabled retry of the sandboxed DNS failure.
- Exact temporary `cargo-audit 0.22.2`: failed on RUSTSEC-2026-0194 and RUSTSEC-2026-0195 in pre-existing `quick-xml 0.39.4`; the 4E lockfile diff adds only `rfd`, and source review found the cited duplicate-attribute and `NsReader` APIs unused on the existing `plist -> Tauri` path. D-025 records the scoped reviewed baseline exception. No ignore or remediation change was added.
- Native example: passed the target-Mac manual gate. Approve -> `approved`, Reject -> `rejected`, Edit -> `cancelled: edit requested`, and Return/default -> `rejected`; Escape had no effect and no window-close control was available. The project owner confirmed fixed title/content/order, post-resolution terminal redaction, no action or persistence, and no permission prompt.

## Risks and mitigations

- **WebView authority confusion:** no frontend or IPC file changes; the source is callable only from Rust.
- **Subject substitution:** presentation and source outcome retain a private pointer-identical manager marker plus exact approval/run/request/call binding, all rechecked by the manager.
- **Duplicate authority:** presentation and source outcome are one-shot, non-cloneable, non-serializable values; manager tombstones remain non-evicting.
- **Title leakage:** every application-owned copy is explicitly bounded and transient; source copies drop after interaction, and outcomes, errors, debug, audit, persistence, and terminal output exclude title content.
- **Preview spoofing or invisible content:** trusted facts precede the final untrusted title row; the closed zero-width/default-ignorable/line/bidi display set fails, ordinary non-ASCII remains allowed, and the full message is capped at 1,024 characters.
- **Residual Unicode rendering ambiguity:** the closed set is not a complete confusable, font, normalization, or visual-width analysis. Title-last layout limits policy-row spoofing, and production integration must reassess whether a richer structured native view is required.
- **Edit ambiguity:** Edit is terminal cancellation and requires full revalidation from a fresh call.
- **Authentication overclaim:** every source outcome records `NotEvaluated`; no device-owner-authenticated variant is produced.
- **Stale dialog:** cancellation consumes manager state immediately and late results fail. The visible prompt may remain, and live orchestration is excluded pending a separate UX decision.
- **Dependency breadth:** exact macOS target scoping, no Tauri plugin registration, no file-dialog wrapper, dependency-tree review, and audit limit exposure.
- **Native dependency internals:** review the exact source and resolved AppKit/raw-handle graph; keep application `unsafe` forbidden and expose no dialog object, raw handle, clone, or debug path.
- **UI/platform mismatch:** a target-Mac main-thread example verifies actual native labels, button mapping, available dismissal behavior, focus, and no permission prompt before completion.
- **Default-action risk:** Reject is configured first and the target-Mac gate proves Return, Escape, and window close cannot approve.
- **Premature execution:** no resolution conversion, dispatch, executor, audit, provider, storage, or production caller is added.

## Explicit non-goals

- Shipping application wiring or a production approval coordinator.
- WebView input, UI changes, Tauri commands/events/plugins, capabilities, CSP, or permission changes.
- Programmatic cancellation or replacement of an already-visible native prompt.
- LocalAuthentication or any actor-identity, biometric, device-password, or strong user-presence claim.
- Audit, persistence, tasks, dispatch, execution, provider continuation, or tool results.
- Gateway networking, credentials, identity, deployment, provider retention, or live model work.
- Multiple pending approvals, parallel prompts, prompt retry, arbitrary tool previews, or editable native fields.
- Generic native dialog, file-open/save, URL, shell, Accessibility, screen capture, Apple Events, microphone, or broad filesystem APIs.
- A complete Unicode confusable, normalization, font-rendering, or visual-width engine.

## Rollback

Delete the new source and manual example, restore the Increment 4D manager/type API and tests, remove the exact target-specific dependency and its lockfile graph, and restore documentation to make 4E planning Ready again. Do not disturb the verified gateway, schema, policy, exact approval subject, preview, TTL, limits, replay tombstones, Tauri shell, frontend, storage, audit, provider, capability, CSP, or permissions.

## Acceptance criteria

- One exact manager-issued presentation is the only input accepted by the native source.
- One sealed source outcome is the only input that can produce a source-backed manager resolution.
- Approval/run/request/call identity is retained and rechecked end to end.
- Cross-manager outcome substitution fails even when every public ID collides.
- The native dialog renders every required preview field and closed choice on the target Mac.
- Non-macOS all-target builds compile without the target-specific dependency or an interactive prompt.
- Reject is the native default; Return, Escape, close, no-decision, and failure paths cannot approve.
- Native preview construction is fixed-field-first, title-last, rejects every code point in the exact presentation-format set while allowing ordinary non-ASCII text, and enforces the 1,024-character complete-message cap.
- Approve, Reject, Edit, native no-decision, source failure, run cancellation, expiry, mismatch, and replay all have typed fail-closed outcomes.
- Edit cannot mutate or reuse the subject.
- Native interaction evidence makes no identity, biometric, device-owner-authentication, run-liveness, or execution claim.
- LocalAuthentication is explicitly deferred with a precise future binding contract.
- No title or raw OS/dependency error enters debug, error, audit, logs, or terminal output.
- The exact native dependency source, license, features, duplicate graph, unsafe boundary, and target-Mac behavior are reviewed before D-025 is accepted.
- Exact temporary `cargo-audit 0.22.2` has no unreviewed Increment 4E advisory: its two pre-existing findings have the scoped D-025 baseline disposition, and no advisory is ignored.
- No WebView, Tauri command/event/plugin, capability, CSP, permission, audit, dispatch, executor, persistence, provider, network, or credential path is added.
- Focused tests, full verification, dependency audit, target-Mac manual checks, code review, security review, and documentation synchronization all pass.
- D-025 and completion state are recorded only after approval and successful implementation verification.

## Completion result

The target-Mac interaction gate passes, and D-025 records the project-owner-approved scoped disposition for the reviewed baseline RustSec findings. Every Increment 4E completion gate is closed with the scanner's nonzero result explicitly preserved. This plan is Complete. Implementation commit `b0a3036` was pushed, fast-forward merged into `main`, and published. No later increment was started.
