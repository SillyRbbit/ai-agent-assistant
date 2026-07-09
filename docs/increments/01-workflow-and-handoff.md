# Increment 1.2 — repository workflow and handoff system

## Goal

Add the repository-based operating system used to start, troubleshoot, review, end, and resume coding-assistant sessions without relying on chat history.

## Added

- Durable repository instructions.
- Current handoff, project status, next steps, decisions, changelog, and troubleshooting history.
- Security and code-review guides.
- Human and assistant usage instructions.
- Start, resume, end-session, and troubleshooting runbooks.
- Repository-scoped Codex skills.
- Reusable prompt library.
- Product brief and architecture baseline.
- Templates for future increments, decisions, handoffs, and troubleshooting entries.

## Scope

This is a documentation-only increment. It does not change runtime behavior, Tauri capabilities, dependencies, database state, or application permissions.

## Verification

Run:

```bash
npm run format:check
npm run typecheck
npm run test:unit
npm run build
```

Run the complete native check on the target Mac when practical:

```bash
npm run verify
```

## Expected result

- All Markdown files are formatted.
- Existing TypeScript and frontend tests remain unchanged and pass.
- The Vite production build remains successful.
- The native application continues to launch on macOS.
- A new assistant session can identify the current state and exact next task from repository files alone.

## Actual verification results

- `npm ci`: passed.
- `npx prettier --check .`: passed.
- `npm run lint:frontend`: passed.
- `npm run typecheck`: passed.
- `npx vitest run`: two tests passed.
- `npm run build`: passed.
- `npm audit --audit-level=low`: zero known vulnerabilities.
- Repository Markdown and skill metadata validation: passed.
- Rust and native Tauri checks: not rerun on the artifact-generation host because Cargo was unavailable. The application was already confirmed running on the target Mac before this documentation-only change.
