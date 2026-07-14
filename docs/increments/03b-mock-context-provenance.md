# Phase 3 Increment 3B — mock context provenance

Last updated: 2026-07-13

Status: **Verified complete**

## Goal

Add a clearly labeled, volatile mock disclosure showing exactly which fixed context categories the deterministic assistant run used, without collecting real context or claiming trusted provenance.

## Planning result

- The verified loop already covers messages, streaming, Stop, approval decisions, failure, Retry, stale-event rejection, tool activity, and per-conversation restoration.
- The product brief requires the user to see what information the assistant used.
- Decision D-017 identifies conversation identity as a prerequisite for context provenance.
- A fixed frontend mock disclosure is smaller than tool-result modeling or broader loop expansion and does not cross a native trust boundary.
- The exact scope, risks, file list, tests, verification gate, and rollback are recorded in `docs/plans/03b-mock-context-provenance.md`.

## Implemented behavior

- One typed provenance record per mock run, scoped to its volatile conversation.
- Current request marked used; prior messages, saved memory, device data, and external services marked not used.
- Opaque IDs and fixed copy only; no request text or personal content in provenance.
- Explicit mock-only labeling; no trusted audit claim.
- The constructor rejects malformed run and conversation IDs.
- Submit and Retry each append one record without duplicating request messages.
- Conversation selection restores only the owning records.
- Accessible fixed-copy cards identify their run and state that they are not trusted audit evidence.
- Provenance-bearing sessions are not treated as empty.

## Planning baseline

```text
git status --short --branch: clean phase3/increment-3b
Node.js: v26.3.0
npm: 11.16.0
Cargo: 1.90.0
Rust: 1.90.0
rustfmt: 1.8.0-stable
Clippy: 0.1.90
npm run typecheck: passed
npm run test:unit: passed
frontend tests: 83 passed
Rust library tests: 50 passed
```

No runtime, dependency, lockfile, Rust, Tauri, IPC, capability, CSP, persistence, credential, network, packaging, or permission file changed during planning.

## Implementation files

Created:

```text
src/application/contextProvenance.ts
src/application/contextProvenance.test.ts
src/features/conversations/ContextProvenanceCard.tsx
docs/increments/03b-mock-context-provenance.md
docs/plans/03b-mock-context-provenance.md
```

Changed:

```text
src/application/conversations.ts
src/application/conversations.test.ts
src/application/state.ts
src/application/state.test.ts
src/App.tsx
src/App.test.tsx
src/features/conversations/ConversationWorkspace.tsx
src/styles.css
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
```

No dependency, lockfile, Rust, Tauri, IPC, capability, CSP, persistence, credential, network, packaging, or permission file changed.

## Verification evidence

```text
targeted Vitest: 4 files, 66 tests passed
npm run typecheck: passed
npm run lint:frontend: passed
npm run verify: passed
full Vitest: 8 files, 92 tests passed
Rust library tests: 50 passed
Rust integration tests: 6 passed
Vite production build: passed
Tauri release no-bundle build: passed
npm audit --audit-level=low: 0 vulnerabilities
git diff --check: passed
native Tauri development launch: passed
storage startup: idempotent, 2 migrations already applied
```

Project-owner native confirmation passed for the fixed source/status disclosure, request-content exclusion, mock-only trust labeling, per-conversation restoration, no duplication after Stop or decisions, minimum-window layout, and existing native regressions.

## Approval gate

The exact file plan was approved before implementation. No file outside the approved list changed.
