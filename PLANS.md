# Execution plans

Use an execution plan for work that spans multiple modules, introduces a dependency, changes a trust boundary, or cannot be verified in one short edit-test cycle.

## Active plan

Phase 2 Increment 2C is active until target-Mac verification passes.

```text
docs/plans/02c-storage-startup.md
```

Source implementation, focused tests, static security scanning, and project-memory synchronization are complete. Remaining steps are target-Mac Rust verification, frontend verification, two native launches, complete diff review, and final handoff closure.

## Completed plans

```text
docs/plans/02b-1-sqlite-migration-skeleton.md
```

Increment 2B-1 and its Rust 1.90 compatibility repair were verified on the target Mac before Increment 2C began.

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

Keep plans current while working. Mark completed steps, record deviations, and close the plan with actual verification results.

## Plan status values

- **Draft** — still being designed.
- **Ready** — enough information exists to implement.
- **Active** — implementation is in progress or verification remains.
- **Blocked** — a missing decision, dependency, environment issue, or failing prerequisite prevents progress.
- **Complete** — acceptance criteria and verification are complete.
- **Superseded** — replaced by another plan; retain the link to the replacement.

## Plan index

| Plan                                     | Status   | Owner              | Last updated |
| ---------------------------------------- | -------- | ------------------ | ------------ |
| Increment 2B-1 SQLite migration skeleton | Complete | Project maintainer | 2026-07-13   |
| Increment 2C storage startup integration | Active   | Project maintainer | 2026-07-13   |
