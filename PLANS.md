# Execution plans

Use an execution plan for work that spans multiple modules, introduces a dependency, changes a trust boundary, or cannot be verified in one short edit-test cycle.

## Active plan

Phase 2 Increment 2B-1 is active until target-Mac verification passes.

```text
docs/plans/02b-1-sqlite-migration-skeleton.md
```

Implementation and focused storage-harness verification are complete. The remaining plan steps are target-Mac lockfile resolution, full Tauri-crate verification, and native application launch.

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

## Active-plan index

| Plan                                     | Status | Owner              | Last updated |
| ---------------------------------------- | ------ | ------------------ | ------------ |
| Increment 2B-1 SQLite migration skeleton | Active | Project maintainer | 2026-07-13   |
