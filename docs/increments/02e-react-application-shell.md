# Phase 2 Increment 2E — React application shell

Last updated: 2026-07-13

Status: **Verified complete**

## Goal

Replace the proof-of-connection screen with the platform-neutral React application shell and consume the existing closed native menu-route event without expanding native authority.

## Baseline confirmed before implementation

- Increment 2D project memory was read.
- The reconstructed verified Increment 2D baseline was on local branch `main` with a clean working tree.
- Node.js `v22.16.0` and npm `10.9.2` were available.
- Cargo, Rust, rustfmt, and Clippy were unavailable on the artifact host.
- `npm ci`, strict TypeScript, and the Vite production build passed before frontend edits.

## Planned files before editing

```text
src/App.tsx
src/App.test.tsx
src/styles.css
src/test/setup.ts

src/application/ApplicationStateProvider.tsx
src/application/context.ts
src/application/navigation.ts
src/application/state.ts
src/application/state.test.ts
src/application/useApplicationState.ts
src/application/useCoreConnection.ts
src/application/useMenuRouteSubscription.ts

src/components/ApplicationSidebar.tsx
src/components/PageState.tsx

src/features/conversations/ConversationWorkspace.tsx
src/features/permissions/PermissionCenter.tsx
src/features/settings/SettingsPage.tsx
src/features/shared/PageHeader.tsx
src/features/shared/PlaceholderPage.tsx
src/features/tasks/TasksPage.tsx

src/infrastructure/tauri/menu-route-client.ts
src/infrastructure/tauri/menu-route-client.test.ts

CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
TROUBLESHOOTING_LOG.md
docs/increments/02e-react-application-shell.md
docs/plans/02e-react-application-shell.md
```

No Rust, Tauri configuration, capability, CSP, package manifest, or lockfile was planned or changed.

## Implemented design

### State model

D-014 resolves O-004 by using React's built-in reducer and context.

The state contains only:

```text
activeRoute
composerDraft
```

It remains local and volatile. No state is read from or written to SQLite, browser storage, files, accounts, or a network service.

### Closed navigation

The application defines exactly seven routes:

```text
conversations
tasks
memory
activity
integrations
permissions
settings
```

Sidebar metadata is deterministic and shared by rendering and reducer tests.

### Native menu event boundary

The frontend listens only for:

```text
assistant-menu-route
```

The Tauri event payload enters as `unknown`. It is accepted only when it is an exact object with one `route` property containing:

```text
new_request
tasks_placeholder
```

Unknown strings, primitives, arrays, empty objects, wrong types, and objects with additional properties are ignored.

`new_request` opens Conversations and clears the in-memory draft. `tasks_placeholder` opens Tasks.

### Application shell

The shell provides:

- persistent sidebar navigation,
- a main toolbar and local-core status,
- conversation workspace with empty state,
- controlled but disabled composer,
- Tasks, Memory, Activity, and Integrations placeholders,
- Settings with typed `get_app_info` diagnostics,
- Permission Center placeholder rows without request actions,
- responsive light and dark presentation at the current minimum Tauri window size.

### Testability

`App` accepts narrow injected services for:

```text
AppInfoLoader
MenuRouteSource
```

Production defaults use the existing Tauri IPC client and native event source. Tests use deterministic in-memory implementations and do not require Tauri, a network, OS permissions, or machine state.

## Files changed

```text
src/App.tsx
src/App.test.tsx
src/styles.css
src/test/setup.ts
src/application/ApplicationStateProvider.tsx
src/application/context.ts
src/application/navigation.ts
src/application/state.ts
src/application/state.test.ts
src/application/useApplicationState.ts
src/application/useCoreConnection.ts
src/application/useMenuRouteSubscription.ts
src/components/ApplicationSidebar.tsx
src/components/PageState.tsx
src/features/conversations/ConversationWorkspace.tsx
src/features/permissions/PermissionCenter.tsx
src/features/settings/SettingsPage.tsx
src/features/shared/PageHeader.tsx
src/features/shared/PlaceholderPage.tsx
src/features/tasks/TasksPage.tsx
src/infrastructure/tauri/menu-route-client.ts
src/infrastructure/tauri/menu-route-client.test.ts
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
TROUBLESHOOTING_LOG.md
docs/increments/02e-react-application-shell.md
docs/plans/02e-react-application-shell.md
```

## Focused tests

Frontend tests cover:

- deterministic route order,
- navigation to all seven routes,
- unchanged navigation state reuse,
- composer draft updates,
- New Request routing and draft clearing,
- Tasks placeholder routing,
- acceptance of both closed native route values,
- rejection of eight malformed or extended payload shapes,
- conversation workspace and composer,
- every sidebar page,
- typed Rust diagnostics in Settings,
- loading and error presentations,
- Permission Center without request buttons,
- bounded native-listener failure presentation,
- native listener cleanup on unmount.

Result:

```text
3 test files passed
30 tests passed
0 failed
```

## Artifact-host commands and results

```text
npm ci                                      passed
npx prettier --check .                      passed
npm run lint:frontend                       passed
npm run typecheck                           passed
npx vitest run                              passed — 30 tests
npm run build                               passed
npm audit --audit-level=low                 passed — 0 vulnerabilities
git diff --check                            passed
```

Build result:

```text
49 modules transformed
CSS: 12.17 kB (3.23 kB gzip)
JavaScript: 208.78 kB (65.12 kB gzip)
```

Unavailable on this host:

```text
cargo fmt
cargo clippy
cargo test
native Tauri launch
macOS lifecycle/manual verification
```

## Security review

- No new dependency.
- No new Tauri command.
- No capability or CSP change.
- No Rust or storage change.
- No persistence.
- No API key, model, gateway, OAuth, or network access.
- No permission request.
- No shell, broad filesystem, Accessibility, screen capture, Apple Events, microphone, calendar, contacts, reminders, notifications, or clipboard API.
- External event payloads are strictly narrowed from `unknown`.
- The native listener's thrown error detail is not rendered to the user.

## Target-Mac verification pending

Run:

```bash
npm run format:check
npm run lint
npm run typecheck
npm run test:unit
npm run test:integration
npm run build
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run tauri -- dev
```

Manual acceptance:

- shell and all sidebar destinations render,
- composer is present and non-functional,
- Settings diagnostics work,
- Permission Center produces no OS prompt,
- New Request routes to Conversations and clears the draft,
- Tasks routes to Tasks,
- native close, reopen, Dock, and quit behavior remains intact,
- storage startup remains idempotent,
- no permission prompt appears.

## Completion result

Increment 2E is not yet complete. Source implementation and frontend artifact-host checks pass; the target-Mac gate remains required. Increment 2F stays blocked.
