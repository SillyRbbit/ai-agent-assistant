# Working-session handoff

Last updated: 2026-06-18

## Current phase

Phase 2 — repository setup and working Tauri shell.

Current increment: Increment 1.2 — repository operating system, session continuity, reusable prompts, and assistant skills.

## Last completed work

- Created the smallest runnable Tauri 2, React, TypeScript, Vite, and Rust application.
- Added Node.js 26.3.0 and npm 11.16.0 compatibility while retaining strict engine enforcement.
- Resolved local macOS Rust toolchain discovery by adding Homebrew `rustup` to `PATH`.
- Confirmed the application now launches and runs on Henry's Apple Silicon MacBook Pro.
- Added persistent repository memory, session workflows, prompt files, Codex skills, troubleshooting records, and next-step planning documents.

## Working branch

Unknown in the packaged snapshot. The archive was not a Git repository when this handoff was generated.

After opening the local checkout, record the branch with:

```bash
git branch --show-current
```

If Git has not been initialized, follow the first-time Git instructions in `ASSISTANT_USAGE.md` before beginning implementation work.

## Confirmed local environment

```text
Platform: macOS on Apple Silicon
Node.js: 26.3.0
npm: 11.16.0
Rust toolchain: 1.90.0-aarch64-apple-darwin
Package manager: npm
```

Homebrew `rustup` must be present on `PATH`:

```bash
export PATH="$(brew --prefix rustup)/bin:$PATH"
```

## Application status

### Working

- `npm ci`
- `npm run tauri -- dev`
- Native main window
- React rendering
- Strict TypeScript compilation
- Typed `get_app_info` Tauri IPC command
- Rust core connection indicator
- Frontend unit tests
- Rust unit and integration tests when Cargo is available
- Formatting and lint command definitions

### Not implemented yet

- Menu-bar entry and hide/show lifecycle
- Sidebar navigation and application pages
- Agent provider interface and mocked streaming runtime
- Tool registry interface
- Policy engine interface
- Approval manager interface
- Audit logger interface
- Memory store interface
- Platform adapter interface
- SQLite storage and migrations
- Tool activity card
- Approval dialog
- Settings page
- Permission Center shell

### Intentionally prohibited or deferred

- Production model credentials
- Accessibility
- Screen capture
- Apple Events
- Unrestricted shell execution
- Broad filesystem access
- Autonomous external or destructive actions

## Next recommended task

Implement **Phase 2 Increment 2A: platform-neutral core interfaces and deterministic mock implementations**.

The increment should add interfaces for:

- Agent provider
- Tool registry
- Policy engine
- Approval manager
- Audit logger
- Memory store
- Platform adapter

Keep the UI behavior unchanged except for tests or wiring required to prove the interfaces compile. Do not add SQLite or menu-bar behavior in the same increment.

Acceptance criteria and the follow-on sequence are in `NEXT_STEPS.md`.

## Exact resume prompt

Copy and paste `prompts/resume-work.md`, or use this compact prompt:

```text
Resume Cortexa from the repository state. Read AGENTS.md, HANDOFF.md, PROJECT_STATUS.md, NEXT_STEPS.md, DECISIONS.md, TROUBLESHOOTING_LOG.md, SECURITY.md, and the relevant increment documents before changing files. Confirm the actual Git status and toolchain. Then implement only Phase 2 Increment 2A: platform-neutral interfaces and deterministic mocks for AgentProvider, ToolRegistry, PolicyEngine, ApprovalManager, AuditLogger, MemoryStore, and PlatformAdapter. Preserve current UI behavior, add focused Rust tests, use typed errors with no unwrap or panic in production paths, run the relevant checks, and update all handoff documents with actual results.
```

## Files added or changed in the latest documentation increment

- `AGENTS.md`
- `ASSISTANT_USAGE.md`
- `CHANGELOG.md`
- `CODE_REVIEW.md`
- `CONTRIBUTING.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `README.md`
- `SECURITY.md`
- `TROUBLESHOOTING_LOG.md`
- `.agents/skills/**/SKILL.md`
- `docs/product/*.md`
- `docs/workflows/*.md`
- `docs/templates/*.md`
- `docs/plans/*.md`
- `docs/increments/01-workflow-and-handoff.md`
- `prompts/*.md`

## Verification for the latest documentation increment

Verification performed on the artifact-generation host:

| Command                                          | Result               | Notes                                                                             |
| ------------------------------------------------ | -------------------- | --------------------------------------------------------------------------------- |
| `npm ci`                                         | Passed               | 275 packages installed; zero npm audit vulnerabilities                            |
| `npx prettier --check .`                         | Passed               | All matched files use Prettier formatting                                         |
| `npm run lint:frontend`                          | Passed               | ESLint completed with zero warnings                                               |
| `npm run typecheck`                              | Passed               | Strict TypeScript project build completed                                         |
| `npx vitest run`                                 | Passed               | One test file and two tests passed                                                |
| `npm run build`                                  | Passed               | Vite production build completed                                                   |
| Markdown path and skill metadata validation      | Passed               | 46 repository Markdown files and 8 skills validated                               |
| Rust formatting, Clippy, tests, and native build | Not run on this host | Cargo was unavailable and the host could not resolve the Rustup download endpoint |

Target-macOS status: the user confirmed that `npm run tauri -- dev` launches and works on Henry's Apple Silicon MacBook Pro before this documentation-only increment. Run the complete native check on that Mac after applying the update:

```bash
npm run verify
```

## Warnings and open questions

- The packaged repository was not initialized as Git. The local working copy may already be under version control; verify before running `git init`.
- The next implementation increment should not combine interfaces, SQLite, menu-bar behavior, and the full UI shell. Keeping those separate will make failures easier to isolate.
- SQLite library selection remains undecided and must be documented before adding the dependency.
- The product gateway and OpenAI Responses integration remain future phases and must not be introduced during Phase 2.
