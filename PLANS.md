# Execution plans

Use an execution plan for work that spans multiple modules, introduces a dependency, changes a trust boundary, or cannot be verified in one short edit-test cycle.

## Active plan

No implementation plan is active.

The next ready work is Phase 2 Increment 2B-1 in `NEXT_STEPS.md`. It introduces a native SQLite dependency and multiple storage modules, so the next session should create or update an implementation plan before editing source files.

Recommended plan path:

```text
docs/plans/02b-1-sqlite-migration-skeleton.md
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

Keep plans current while working. Mark completed steps, record deviations, and close the plan with actual verification results.

## Plan status values

- **Draft** — still being designed.
- **Ready** — enough information exists to implement.
- **Active** — implementation is in progress.
- **Blocked** — a missing decision, dependency, environment issue, or failing prerequisite prevents progress.
- **Complete** — acceptance criteria and verification are complete.
- **Superseded** — replaced by another plan; retain the link to the replacement.

## Active-plan index

| Plan | Status | Owner | Last updated |
| ---- | ------ | ----- | ------------ |
| None | —      | —     | 2026-07-13   |
