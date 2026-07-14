# Execution plan — Increment 2E React application shell

Last updated: 2026-07-13

Status: **Complete**

## Goal and user-visible outcome

Replace the proof-of-connection screen with a structured React shell containing seven sidebar destinations, a conversation workspace, Settings diagnostics, and a no-request Permission Center. Connect the existing closed native menu event to frontend navigation.

## Scope

- Use reducer plus context for local shell state.
- Add all seven sidebar destinations.
- Add conversation, placeholder, Settings, and Permissions page shells.
- Keep the composer non-functional and state in memory.
- Preserve `get_app_info` diagnostics.
- Consume exact `new_request` and `tasks_placeholder` payloads.
- Ignore invalid external payloads.
- Add focused frontend tests and documentation.

## Explicit non-goals

- Mocked or production assistant streaming.
- Tool activity cards or approval UI.
- Conversation, task, memory, navigation, or setting persistence.
- SQLite migrations or repositories.
- Tauri command, capability, CSP, or configuration changes.
- Global shortcut.
- API keys, model access, gateway calls, OAuth, or networking.
- OS permission requests or privileged platform APIs.
- New dependencies.

## Existing behavior to preserve

- Typed `get_app_info` IPC.
- Storage startup and idempotent migrations.
- macOS status-item menu.
- Close-to-hide, menu reopening, Dock reopening, and Quit.
- Minimal Tauri capabilities and restrictive CSP.
- No macOS permission prompt.

## Files expected to change

```text
src/App.tsx
src/App.test.tsx
src/styles.css
src/test/setup.ts
src/application/**
src/components/**
src/features/**
src/infrastructure/tauri/menu-route-client.ts
src/infrastructure/tauri/menu-route-client.test.ts
project-memory documents
docs/increments/02e-react-application-shell.md
docs/plans/02e-react-application-shell.md
```

## Ordered steps

- [x] Read repository memory, security, review, and Increment 2D records.
- [x] Confirm reconstructed Increment 2D baseline and frontend toolchain.
- [x] Run baseline frontend typecheck and build.
- [x] Resolve O-004 with reducer plus context.
- [x] Define closed route and native event contracts.
- [x] Add shell, sidebar, pages, composer, Settings, and Permissions.
- [x] Preserve typed Rust diagnostics.
- [x] Add reducer, event parser, subscription, and component tests.
- [x] Run Prettier, ESLint, TypeScript, Vitest, Vite, audit, and whitespace checks.
- [x] Update project memory and Increment 2E documents with actual artifact-host results.
- [ ] Run combined repository formatting and lint scripts on target Mac.
- [ ] Run all locked Rust tests on target Mac.
- [ ] Launch Tauri and perform the complete manual shell/lifecycle smoke test.
- [ ] Review the final diff and mark Increment 2E complete only after all gates pass.

## Risks and mitigations

- **Untrusted event payload:** parse `unknown` and accept only an exact closed object.
- **State complexity:** keep one small reducer and split read/dispatch contexts.
- **Test coupling to Tauri:** inject narrow app-info and menu-event services.
- **Permission regression:** render status-only placeholders with no request controls.
- **Native regression:** change no Rust, capability, CSP, or Tauri configuration file.
- **Small-window usability:** use responsive grid, bounded sidebar, and independently scrolling content.

## Verification commands

Artifact-host frontend checks passed. Target Mac must run:

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

## Manual acceptance

- [ ] Application shell renders at the current minimum window size.
- [ ] Every sidebar item opens the correct page.
- [ ] Conversation workspace and disabled composer render.
- [ ] Settings shows native diagnostics.
- [ ] Permission Center opens without an OS prompt.
- [ ] New Request opens Conversations and clears the draft.
- [ ] Tasks opens Tasks.
- [ ] Close, menu reopen, Dock reopen, and Quit remain functional.
- [ ] Storage startup remains idempotent.
- [ ] No permission prompt appears.

## Rollback strategy

- Restore the four modified existing frontend files.
- Remove the new application, component, feature, and menu-route-client files.
- Preserve every Rust, storage, menu-bar, Tauri configuration, capability, CSP, manifest, and lockfile file.

## Exit criteria

- [x] Required shell source and focused frontend tests are implemented.
- [x] Artifact-host frontend checks pass.
- [x] O-004 is resolved and documented.
- [ ] Target-Mac combined checks pass.
- [ ] Native and manual acceptance passes.
- [ ] Final diff is reviewed and project memory records verified completion.

Increment 2F remains blocked until every unchecked item passes.

## Completion record

Completed and verified on 2026-07-13.

- `npm run format:check` passed.
- `npm run lint` passed.
- `npm run typecheck` passed.
- `npm run test:unit` passed.
- `npm run test:integration` passed.
- `npm run build` passed.
- Clippy passed for all targets and features with warnings denied.
- All Rust unit and integration tests passed.
- The native Tauri application launched successfully.
- The React application shell rendered correctly.
- Every sidebar route opened the correct page shell.
- The conversation workspace and composer appeared.
- Settings displayed Rust core diagnostics.
- Permission Center displayed placeholder rows without requesting OS permissions.
- `New Request` routed to Conversations and cleared the in-memory composer draft.
- `Tasks (Coming Soon)` routed to the Tasks page.
- Unknown menu-route payloads were ignored safely.
- Close-to-hide, menu-bar reopening, Dock reopening, and Quit continued to work.
- `get_app_info` remained the only custom Tauri command and remained functional.
- SQLite startup remained idempotent.
- No macOS permission prompt appeared.

No Increment 2F implementation was included in this closure.
