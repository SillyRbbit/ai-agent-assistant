# Phase 4 Increment 4F - Cortexa product display rename

Status: Complete
Owner: Project maintainer
Last updated: 2026-07-14

## Goal

Rename the assistant's former human-facing product name to `Cortexa` without changing compatibility-sensitive repository, package, crate, executable, bundle-identifier, event, command, storage, or database identifiers.

## User-visible outcome

- The application window, native application menu, right-side menu-bar item, Settings application value, sidebar brand, native approval dialog, and browser metadata show `Cortexa`.
- Repository documentation, prompts, and local skill descriptions consistently use `Cortexa` as the product name.
- Existing behavior, security boundaries, persistence, and integration contracts remain unchanged.

## Scope

- Replace every reviewed tracked occurrence of the exact former product phrase with `Cortexa`.
- Change the sidebar monogram from `A` to `C`.
- Update focused React and Rust assertions for the new display value.
- Add a durable display-name-only decision and synchronize project memory.

## Explicit non-goals

- No rename of the GitHub repository or local repository directory.
- No rename of npm package `ai-agent-assistant` or its lockfile entries.
- No rename of Cargo package `ai-agent-assistant`, library crate `ai_agent_assistant_lib`, or the executable target.
- No change to bundle identifier `com.aiagentassistant.desktop`.
- No database, migration, storage path, menu-route event, Tauri command, Rust module, type, variable, or test-target identifier rename.
- No dependency, capability, CSP, permission, network, credential, model, gateway, audit, approval-authority, execution, or persistence change.
- No new product copy where existing generic Settings or placeholder copy contains no former product name.

## Existing behavior and constraints

- At baseline, Tauri `productName`, the main-window title, Rust `AppInfo`, native menu labels, tray tooltip, native approval title, and sidebar exposed the former display name.
- Settings renders `AppInfo.name`; it has no separate product-name literal.
- The composer placeholder is generic and has no product-name literal.
- Internal compatibility identifiers intentionally contain `ai-agent-assistant` or `assistant` and must remain stable.
- `AGENTS.md` requires `$post-increment-gate`, but no matching repository skill currently exists. D-027 records the project owner's one-time 4F sequencing exception without claiming that the gate ran; the rule remains mandatory for later implementation increments.

## Files expected to change

Application, native display surface, and tests:

```text
index.html
src/App.test.tsx
src/components/ApplicationSidebar.tsx
src-tauri/Cargo.toml
src-tauri/src/app_info.rs
src-tauri/src/approvals/decision_source.rs
src-tauri/src/main.rs
src-tauri/src/menu_bar/action.rs
src-tauri/src/menu_bar/tauri_adapter.rs
src-tauri/src/startup.rs
src-tauri/tauri.conf.json
src-tauri/tests/smoke.rs
```

Repository instructions, skills, prompts, and project memory:

```text
.agents/skills/code-review/SKILL.md
.agents/skills/documentation-sync/SKILL.md
.agents/skills/resume-session/SKILL.md
.agents/skills/security-review/SKILL.md
.agents/skills/session-end/SKILL.md
.agents/skills/session-start/SKILL.md
.agents/skills/troubleshoot/SKILL.md
.agents/skills/verified-increment/SKILL.md
AGENTS.md
ASSISTANT_USAGE.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
README.md
SECURITY.md
TROUBLESHOOTING_LOG.md
prompts/end-of-session-handoff.md
prompts/resume-work.md
prompts/start-work.md
```

Product, historical, increment, and plan documentation:

```text
docs/backups/2026-07-09-increment-2a-doc-sync/HANDOFF.md
docs/increments/02d-menu-bar-window-lifecycle.md
docs/increments/04e-trusted-approval-decision-source.md
docs/increments/04f-cortexa-product-display-rename.md
docs/plans/04e-trusted-approval-decision-source.md
docs/plans/04f-cortexa-product-display-rename.md
docs/plans/README.md
docs/product/ARCHITECTURE_BASELINE.md
docs/product/PRODUCT_BRIEF.md
```

## Implementation steps

- [x] Confirm clean merged `main`, review all exact old-name occurrences, and classify compatibility identifiers.
- [x] Run `npm run build` as the smallest pre-edit application build baseline.
- [x] Update application and native display strings plus focused tests.
- [x] Update every reviewed tracked documentation, prompt, and skill occurrence.
- [x] Run targeted tests and the complete requested automated verification.
- [x] Run the native development application and obtain project-owner manual confirmation.
- [x] Complete documentation synchronization and record the project-owner D-027 gate deferral.

## Security and privacy considerations

The change is presentation-only. It adds no untrusted input, serialization, IPC route, native capability, permission, network access, credential, persistence, log content, or execution authority. Existing fixed and redacted error copy remains fixed and redacted; only its product prefix changes.

## Risks

- A visible product label could be missed.
- A broad replacement could rename a compatibility-sensitive identifier.
- Historical documentation could retain conflicting product copy.
- Tauri product metadata could be mistaken for a package or executable rename.
- The required post-increment gate is unavailable in the current repository and is deferred under D-027 to the next clean branch.

## Verification commands

```bash
npm run format:check
npm run lint
npm run typecheck
npm run test:unit
npm run test:integration
npm run build
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
git grep -n -i -E 'AI[[:space:]]+Agent[[:space:]]+Assistant'
git diff --check
npm run tauri -- dev
```

The old-name search must return no tracked matches. A separate compatibility review must confirm that package, crate, executable, bundle ID, repository paths, storage paths, event names, and command identifiers remain unchanged.

## Manual verification

- Main window and standard application menu show `Cortexa`.
- Right-side menu-bar title/tooltip and Open/Quit labels show `Cortexa`.
- Settings shows `Cortexa` as the application value.
- Sidebar shows `Cortexa`; the fixed native approval title is covered by the focused Rust test.
- Existing navigation, menu routing, mocked assistant interaction, lifecycle, and diagnostics remain unchanged.
- No operating-system permission prompt appears.

## Rollback or failure strategy

Revert only the display strings, monogram, focused assertions, and rename documentation. No migration, identifier compatibility step, dependency rollback, or data conversion is required.

## Exit criteria

- Every exact former display-name occurrence is removed from tracked content.
- Every preserved compatibility identifier is reviewed and unchanged.
- All requested automated commands pass.
- The project owner confirms every manual check.
- Code, security, scope, secret, generated-output, and complete-diff review pass.
- Project memory matches actual evidence.
- The mandatory post-increment gate produces PASS or PASS WITH ADVISORIES, unless an explicit project-owner decision records a one-time sequencing exception without claiming the gate ran.

## Verification result

Passed:

- focused frontend and Rust tests: 25 React app, one app-info, nine menu-bar, 16 approval, and one smoke test;
- every requested formatting, lint, typecheck, unit, integration, build, Clippy, and all-target Cargo command;
- 124 frontend tests, 92 Rust library tests, and ten Rust integration tests;
- exact former-name, preserved-identifier, package/executable metadata, secret-pattern, scope, generated-output, and diff checks;
- code review and security review with no finding; and
- target-Mac launch plus project-owner window, application-menu, status-item menu, sidebar, Settings, regression, and no-permission-prompt confirmation.

Resolved command failures:

- The first `npm run format:check` found only `index.html` Prettier layout drift. `npx prettier --write index.html` fixed it and the required check passed on rerun.
- The first `npm run tauri -- dev` found port 1420 occupied by a stale standalone project Vite process. After identifying and stopping its `npm run dev` parent, the exact launch command succeeded. TS-013 records the diagnosis.

Not run and explicitly deferred:

- `$post-increment-gate` is unavailable because no matching skill or report workflow exists in the repository. The project owner explicitly confirmed 4F complete and directed commit, push, and merge, with skill creation deferred to the next clean branch. D-027 records this one-time exception; no gate result is claimed and the mandatory rule remains in force for later implementation increments.
