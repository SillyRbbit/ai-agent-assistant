# Execution plans

Use an execution plan for work that spans multiple modules, changes a trust boundary, or cannot be verified in one short edit-test cycle.

## Active plan

No execution plan is currently Active.

Phase 2 Increment 2D is verified complete. Increment 2E — React application shell — is the next Ready item in `NEXT_STEPS.md`. Create or activate its detailed plan only when the next implementation session begins.

## Completed plans

```text
docs/plans/02b-1-sqlite-migration-skeleton.md
docs/plans/02c-storage-startup.md
docs/plans/02d-menu-bar-window-lifecycle.md
```

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

## Next planning action

At the start of Increment 2E:

1. Confirm Increment 2D is committed and the working tree is clean.
2. Re-read the current security and UI constraints.
3. Resolve O-004, preferring React reducer plus context unless another dependency is justified.
4. Create or activate an Increment 2E execution plan before editing frontend behavior.
