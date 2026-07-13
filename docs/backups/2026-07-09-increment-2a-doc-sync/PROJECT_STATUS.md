# Project status

Last updated: 2026-06-18

## Objective

Build a standalone, native-feeling, local-first AI personal executive assistant. The macOS application will help a CEO gather information, organize work, propose actions, and execute only narrowly scoped actions allowed by deterministic policy and human approval.

## Current milestone

Phase 2 — repository setup and working Tauri shell.

The smallest native application is running on the target macOS workstation. Repository continuity and assistant operating procedures are now established. Feature implementation beyond the initial typed IPC proof has not started.

## Architecture baseline

- React and TypeScript run in the Tauri WebView.
- Rust is the trusted local application core.
- The WebView is not an authorization boundary and must never receive generic tool execution access.
- The model will be an untrusted planner behind an `AgentProvider` abstraction.
- Tool validation, policy, approval, execution, and audit remain deterministic local responsibilities.
- Operating-system behavior remains behind platform adapters.
- SQLite will hold local application state; credentials and tokens will not be stored there.
- Production model calls will later use an authenticated gateway; no production API key will be shipped in the client.

See `docs/product/ARCHITECTURE_BASELINE.md` for the working architecture summary.

## Technology status

| Area                | Selected technology              | Status                |
| ------------------- | -------------------------------- | --------------------- |
| Desktop shell       | Tauri 2                          | Running               |
| Frontend            | React 19, TypeScript 5.9, Vite 7 | Running               |
| Local core          | Rust 1.90                        | Running               |
| Persistence         | SQLite                           | Not started           |
| macOS native bridge | Swift only where required        | Not started           |
| Agent provider      | Mock first, Responses API later  | Interface not started |
| Production gateway  | Authenticated streaming gateway  | Future phase          |
| Secrets             | macOS Keychain                   | Future phase          |

## Capability status

| Capability                           | Status                | Notes                                        |
| ------------------------------------ | --------------------- | -------------------------------------------- |
| Native main window                   | Complete              | Main Tauri window launches                   |
| Typed frontend-to-Rust IPC           | Complete              | `get_app_info` proof command                 |
| Strict TypeScript                    | Complete              | Project references and strict checks enabled |
| Rust typed startup errors            | Complete              | No panic-based startup path                  |
| Formatting and linting               | Complete              | Prettier, rustfmt, ESLint, Clippy            |
| Unit and integration tests           | Complete for scaffold | React and Rust smoke coverage                |
| Menu-bar entry                       | Not started           | Phase 2                                      |
| Sidebar and page shell               | Not started           | Phase 2                                      |
| Mock streaming assistant             | Not started           | Phase 2/3 boundary                           |
| Core interfaces                      | Not started           | Next increment                               |
| SQLite foundation                    | Not started           | Follows interfaces                           |
| Tool activity and approvals UI       | Not started           | Phase 2/3 boundary                           |
| Settings and Permission Center shell | Not started           | Phase 2                                      |
| Real tools and permissions           | Not started           | Later phases                                 |
| Responses API                        | Not started           | Phase 4                                      |

## Security status

### Present controls

- Restrictive Tauri capability file.
- No shell, opener, or unrestricted filesystem plugin.
- No API key or `.env` dependency.
- Content Security Policy is configured.
- Production Rust lint rules deny panic-style shortcuts and unsafe code.
- Node engine compatibility is explicit and strictly enforced.

### Controls not yet implemented

- Tool schemas and semantic validation.
- Deterministic action-risk policy.
- Approval binding and nonce handling.
- Audit persistence and redaction.
- SQLite encryption strategy.
- Keychain integration.
- Permission Center behavior.

## Development health

The application is confirmed to run on Henry's Apple Silicon MacBook Pro with Node.js 26.3.0, npm 11.16.0, and Rust 1.90.0. The authoritative local verification result should be updated in `HANDOFF.md` at the end of every session.

## Known risks

- Combining too many Phase 2 concerns in one change would obscure failures and make security review harder.
- SQLite crate and encryption choices are unresolved.
- Platform-neutral interfaces can become over-generalized if designed before concrete mock use cases; keep the first contracts minimal.
- Documentation can drift unless the end-session workflow is followed.

## Next milestone gate

Phase 2 Increment 2A is complete when all seven required core interfaces compile, have deterministic mock implementations, use typed errors, have focused tests, and require no new privileged capability or production credential.

## Project memory map

- Current handoff: `HANDOFF.md`
- Priorities: `NEXT_STEPS.md`
- Decisions: `DECISIONS.md`
- Changes: `CHANGELOG.md`
- Troubleshooting history: `TROUBLESHOOTING_LOG.md`
- Product brief: `docs/product/PRODUCT_BRIEF.md`
- Architecture baseline: `docs/product/ARCHITECTURE_BASELINE.md`
