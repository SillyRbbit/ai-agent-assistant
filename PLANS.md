# Execution plans

Use an execution plan for work that spans multiple modules, changes a trust boundary, or cannot be verified in one short edit-test cycle.

## Active plan

Phase 2 Increment 2D remains active until target-Mac verification passes.

```text
docs/plans/02d-menu-bar-window-lifecycle.md
```

Source implementation, focused tests, frontend checks, static security scanning, and project-memory synchronization are complete. Remaining steps are target-Mac Rust verification, native menu-bar/window smoke testing, complete diff review, and final handoff closure.

## Completed plans

```text
docs/plans/02b-1-sqlite-migration-skeleton.md
docs/plans/02c-storage-startup.md
```

Increment 2C was verified on the target Mac before Increment 2D began.

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
| Increment 2D menu-bar/window lifecycle   | Active   | Project maintainer | 2026-07-13   |
