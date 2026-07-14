# Changelog

All notable repository changes are documented here. Entries distinguish verified work from implementation awaiting target-platform checks.

## Unreleased

### Added

- Phase 3 Increment 3B typed fixed-copy mock context provenance bound to exact run and conversation IDs.
- Accessible per-run context disclosure showing the current request as used and all unaccessed source categories as not used.
- Focused provenance validation, privacy, Retry, empty-session, and conversation-restoration coverage, bringing the frontend suite to 92 tests.
- Documentation-only Phase 3B gap analysis and proposed plan for volatile mock context provenance disclosure.
- Phase 3 Increment 3A volatile conversation sessions with deterministic IDs, bounded titles, and per-session messages and mock tool activity.
- Accessible New conversation and newest-first conversation-history controls in the application sidebar.
- Focused conversation-model, reducer, native-route, busy-state, restoration, and interaction coverage, bringing the frontend suite to 83 tests.
- Documentation-only Phase 3 gap analysis and proposed Increment 3A plan for volatile in-memory conversation sessions.
- Verified Phase 2 Increment 2G typed mock-run driver with explicit idempotent cancellation.
- Bounded mock failure presentation and deterministic Retry without duplicating the user message.
- Redacted in-memory Activity feed for accepted run, Stop, failure, approval-request, and approval-decision events.
- Focused privacy, stale-event, driver, cancellation, failure, Retry, and Activity tests.
- Verified Phase 2 Increment 2F deterministic in-memory assistant interaction shell.
- Fixed mock assistant streaming, Stop behavior, conversation messages, and tool activity presentation.
- Mock-only approval preview with deterministic approve, reject, and edit decisions that execute no tool.
- Focused script, reducer, cancellation, and interaction tests, bringing the frontend suite to 47 tests.
- Verified Phase 2 Increment 2E React application shell.
- Reducer-and-context navigation for Conversations, Tasks, Memory, Activity, Integrations, Permissions, and Settings.
- Conversation workspace with a controlled in-memory composer.
- Settings diagnostics, Permission Center placeholders, and reusable empty, loading, and error presentations.
- Strict frontend listener for the closed `assistant-menu-route` native event.
- Thirty focused frontend tests across reducer, event parsing, navigation, diagnostics, Settings, and Permissions.
- Verified Phase 2 Increment 2C managed `Storage` startup integration.
- Typed `app_initialized` metadata bootstrap with idempotent migrations.
- Verified Phase 2 Increment 2D macOS menu-bar and window lifecycle.
- macOS-only Tauri `tray-icon` support using the existing bundled icon as a temporary template icon.
- Fixed menu actions for Open AI Agent Assistant, New Request, Tasks (Coming Soon), and Quit AI Agent Assistant.
- Main-window close-to-hide behavior and macOS Dock/reopen restoration.
- Closed-enum `assistant-menu-route` backend event for later React-shell integration.
- Platform-neutral menu action and lifecycle policy contracts.
- Focused deterministic unit and integration tests for routing, failure behavior, close policy, and reopen policy.
- Increment 2D record and execution plan.

### Changed

- Stored mock provenance with each volatile conversation and counted provenance-bearing sessions as non-empty.
- Advanced Increment 3B to verification pending after the full automated gate and native development launch passed.
- Marked Increment 3B verified complete after project-owner native interaction, restoration, layout, and regression checks passed.
- Selected mock context provenance as the smallest remaining Phase 3 gap; implementation remains blocked on project-owner approval of the exact file plan.
- Bound active and retryable mock runs to exact conversation IDs so asynchronous events cannot mutate another selected transcript.
- Made native New Request create or select an empty conversation while idle and only focus the active conversation while a run or approval is pending.
- Marked Increment 3A verified complete after full automated verification, native launch, and project-owner manual acceptance passed.
- Replaced direct timer ownership in the React hook with an injectable closed mock-run driver.
- Replaced the Activity placeholder with a volatile session feed containing fixed summaries and opaque run IDs only.
- Hardened late chunk, completion, and failure handling against inactive or mismatched runs.
- Marked Phase 2 verified complete after automated, native-launch, manual interaction, lifecycle, diagnostics, storage, and no-permission-prompt checks passed.
- Made Phase 3 gap analysis and increment planning the next Ready task.
- Enabled the conversation composer for non-empty local mock requests and kept all resulting state volatile.
- Updated Activity copy to distinguish current-conversation mock activity from future persisted audit history.
- Recorded successful full repository verification, native launch, streaming, Stop, approval-decision, layout, lifecycle, storage, and no-permission-prompt checks for Increment 2F.
- Made Increment 2G integration hardening the next Ready increment.
- Replaced the proof-of-connection page with the platform-neutral React shell while retaining `get_app_info` diagnostics.
- Resolved O-004 with React reducer plus context and no new state-management dependency.
- Routed menu-bar New Request to Conversations with a cleared draft and Tasks to the Tasks page.
- Marked Increment 2E verified complete after its target-Mac gate passed.
- Marked Increment 2C verified complete based on target-Mac results: 41 Rust unit tests, 3 Rust integration tests, TypeScript, Vite, and native launch passed.
- Marked Increment 2D verified complete based on target-Mac Rust, frontend, native launch, menu-action, close, reopen, Dock, quit, storage, and permission checks.
- Updated `npm run test:integration` to execute every Rust integration-test target rather than only the original smoke test.
- Changed the Rust entrypoint to build the Tauri app explicitly so macOS `RunEvent::Reopen` can restore the hidden main window.
- Updated project memory to make Increment 2E — React application shell — the next ready increment.
- Documented that the custom right-side menu-bar status item is distinct from the standard left-side macOS application menu.

### Security

- The provenance constructor accepts validated opaque IDs only, uses a closed fixed source list, and cannot receive request text or personal content.
- The new disclosure is explicitly labeled as frontend mock data rather than trusted audit evidence and adds no authorization or execution path.
- Increment 3B changes no dependency, lockfile, Rust, IPC, Tauri, SQLite, capability, CSP, credential, network, packaging, or operating-system permission file.
- The Phase 3B plan limits provenance to volatile fixed-copy WebView presentation with opaque IDs and explicitly excludes request text, real context collection, trusted audit claims, persistence, networking, and native capability changes.
- Increment 3A keeps all conversation titles, messages, and mock tool activity in volatile WebView memory and copies none of that content into Activity.
- Conversation creation and selection fail closed during streaming or pending approval; Retry eligibility is cleared when leaving the failed conversation.
- Increment 3A adds no Rust, IPC, Tauri command, capability, CSP, dependency, lockfile, persistence, credential, network, packaging, or operating-system permission change.
- Increment 2G Activity records exclude request text, tool arguments, tool results, and underlying error details.
- Driver failures map to one bounded user-facing reason; startup exceptions are not rendered or logged.
- Increment 2G adds no Rust, IPC, Tauri, capability, CSP, dependency, persistence, credential, network, or OS permission change.
- Increment 2F adds no model network, credential, dependency, Rust, IPC, Tauri command, capability, CSP, persistence, or operating-system permission change.
- Mock approval decisions remain inside untrusted WebView memory, are labeled as non-executing, and cannot authorize or invoke a local action.
- Mock timer events are bound to an active run identifier; stale events and decisions outside the valid state fail closed.
- Added no Rust, SQLite, Tauri command, capability, CSP, dependency, persistence, credential, networking, or operating-system permission change in Increment 2E.
- Native event payloads enter as `unknown` and are ignored unless they exactly match one closed route object.
- Permission Center is status-only and contains no control that can request operating-system access.
- Kept `get_app_info` as the only custom Tauri command.
- Added no WebView-invokable capability, CSP change, plugin, global shortcut, API key, OAuth flow, model networking, database schema, or product-data persistence.
- Added no Accessibility, screen capture, Apple Events, microphone, shell, or broad filesystem access.
- Menu routes use closed enums, unknown menu IDs are ignored, and no arbitrary content is executed.
- Target-Mac verification produced no macOS permission prompt.

## 0.1.0 — 2026-06-18

### Added

- Smallest runnable Tauri 2 desktop application.
- React 19, TypeScript 5.9, and Vite 7 frontend.
- Typed `get_app_info` command across the Tauri IPC boundary.
- Typed Rust startup error propagation.
- Restrictive Content Security Policy and minimal Tauri capability set.
- ESLint, Prettier, rustfmt, Clippy, Vitest, Rust unit tests, and Rust integration smoke test.

### Changed

- Added compatibility for Node.js 26.3.0 and npm 11.16.0 while preserving strict engine enforcement.

### Security

- No API keys, shell plugin, opener plugin, broad filesystem plugin, Accessibility, screen capture, or Apple Events access.

## 2026-07-13 — Phase 2 Increment 2E verified

- Marked the React application shell as verified complete on the target Mac.
- Recorded successful frontend, Rust, native-launch, menu-routing, lifecycle, diagnostics, storage-idempotence, and no-permission-prompt checks.
- Marked Phase 2 Increment 2F as Ready.
