# Increment 1 — Smallest runnable application

## Goal

Create the smallest useful Tauri 2 desktop application that proves the React WebView can invoke a typed Rust command without introducing privileged plugins, credentials, persistence, or macOS permissions.

## Included

- Tauri 2 window named `main`
- React 19 and strict TypeScript
- Vite development and production builds
- Typed `get_app_info` IPC command
- Explicit Rust startup error propagation
- CSP limited to bundled assets, Tauri IPC, and the local Vite development socket
- ESLint, Prettier, rustfmt, Clippy, Vitest, Rust unit tests, and a Rust integration test

## Intentionally deferred

- SQLite
- Menu-bar entry
- Agent interfaces and mock streaming
- Sidebar, conversation UI, approvals, settings, and Permission Center
- Accessibility, screen capture, Apple Events, shell access, and credentials

These remain deferred until this shell compiles and runs successfully.
