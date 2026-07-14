# Execution plans

Use an execution plan for work that spans multiple modules, changes a trust boundary, or cannot be verified in one short edit-test cycle.

## Active plan

There is no Active plan. **Increment 2G — integration hardening** is Complete.

Implementation, automated release verification, native launch, and project-owner manual acceptance are complete. Phase 2 is verified complete.

Most recently completed plan:

```text
docs/plans/02g-integration-hardening.md
```

## Completed plans

```text
docs/plans/02b-1-sqlite-migration-skeleton.md
docs/plans/02c-storage-startup.md
docs/plans/02d-menu-bar-window-lifecycle.md
docs/plans/02e-react-application-shell.md
docs/plans/02f-mocked-assistant-interaction-shell.md
docs/plans/02g-integration-hardening.md
```

Increments 2C and 2D were verified on the Apple Silicon target Mac.

## Plan rules

A plan must contain:

- Goal and user-visible outcome.
- Scope and explicit non-goals.
- Existing behavior and constraints.
- Files expected to change.
- Ordered implementation steps.
- Security and privacy considerations.
- Tests and verification commands.
- Rollback or failure strategy.
- Exit criteria.
- Documentation updates.

## Plan status values

- **Draft** — still being designed.
- **Ready** — enough information exists to implement.
- **Active** — implementation is in progress or verification remains.
- **Blocked** — a prerequisite prevents progress.
- **Complete** — acceptance criteria and verification are complete.
- **Superseded** — replaced by another plan.

## Plan index

| Plan                                     | Status   | Owner              | Last updated |
| ---------------------------------------- | -------- | ------------------ | ------------ |
| Increment 2B-1 SQLite migration skeleton | Complete | Project maintainer | 2026-07-13   |
| Increment 2C storage startup integration | Complete | Project maintainer | 2026-07-13   |
| Increment 2D menu-bar/window lifecycle   | Complete | Project maintainer | 2026-07-13   |
| Increment 2E React application shell     | Complete | Project maintainer | 2026-07-13   |
| Increment 2F mocked interaction shell    | Complete | Project maintainer | 2026-07-13   |
| Increment 2G integration hardening       | Complete | Project maintainer | 2026-07-13   |

## Phase 2 Increment 2E — complete

Verified on 2026-07-13. The React application shell, closed menu-route handling, Settings diagnostics, and Permission Center placeholders passed all required automated and manual checks.

## Phase 2 Increment 2F — complete

Goal: add a deterministic, mocked assistant interaction flow to the verified application shell.

Planned boundaries:

- In-memory conversation messages only.
- Deterministic mock streaming and stop behavior.
- Tool activity card presentation.
- Trusted mock approval dialog.
- No network, API key, real tool execution, new Tauri command, OS permission, or persistence expansion.

Implementation and `npm run verify` pass on the target Mac. Native Tauri launch passes with idempotent storage startup. The project owner confirmed streaming, Stop, approve/reject/edit, small-window, lifecycle, and no-permission-prompt checks passed.

## Phase 2 Increment 2G — complete

Goal: complete bounded cancellation, error-state, audit-view, and release-verification hardening without production model access or privileged automation.

The typed driver, bounded failure and Retry, redacted in-memory Activity feed, and focused tests are implemented. `npm run verify`, `npm audit --audit-level=low`, native launch, and project-owner manual acceptance all pass.

## Phase 3 planning — ready

Reconcile the Phase 3 product brief with the completed Phase 2 mock loop and define the first bounded Phase 3 increment. Do not implement features until a dedicated plan and exact file set are approved.
