# Project status

Last updated: 2026-07-09

## Overall status

Phase 2 is active.

The repository now contains:

- Smallest runnable Tauri 2, React, TypeScript, Vite, and Rust application.
- Node.js 26.3.0 and npm 11.16.0 compatibility with strict engine enforcement.
- Repository operating system for assistant-driven development.
- Platform-neutral Rust core interfaces and deterministic mocks from Increment 2A.

## Verified complete

### Increment 1 — smallest runnable application

- Tauri 2 application shell.
- React frontend.
- Strict TypeScript configuration.
- Vite build.
- Typed Rust `get_app_info` IPC command.
- Rust typed startup errors.
- Formatting, linting, frontend tests, Rust tests, and build commands.

### Increment 1.1 — Node 26 compatibility

- Project accepts Node.js 26.3.0 and npm 11.16.0.
- Strict engine enforcement remains enabled.

### Increment 1.2 — repository working system

- Persistent project memory files.
- Handoff, status, next-step, decision, changelog, and troubleshooting documents.
- Assistant usage guide.
- Workflow runbooks.
- Prompt library.
- Repository-scoped skills.

### Increment 2A — platform-neutral core interfaces

Verified locally on macOS.

Implemented:

- AgentProvider
- ToolRegistry
- PolicyEngine
- ApprovalManager
- AuditLogger
- MemoryStore
- PlatformAdapter
- RiskClass
- PermissionKind

Verification reported passing:

- Cargo formatting check.
- Cargo Clippy with warnings denied.
- Cargo tests with 25 unit tests and 1 integration test passing.
- TypeScript check.
- Vite production build.
- Native Tauri development launch.

## Current security boundary

The product still has no:

- API keys
- OpenAI integration
- Backend gateway calls
- SQLite database
- New Tauri commands beyond existing app-info IPC
- Accessibility permission
- Screen-capture permission
- Apple Events integration
- Shell command execution
- OAuth
- Calendar, contact, reminder, notification, clipboard, or file tools

## Current architecture boundary

The frontend remains unchanged and cannot call core interfaces directly.

The new Rust interfaces are compiled into the trusted Rust crate but are not yet wired to the UI, persistence, real OS adapters, or real model providers.

## Next status gate

Before starting persistence work, complete Increment 2B-0: SQLite dependency and storage-design decision.
