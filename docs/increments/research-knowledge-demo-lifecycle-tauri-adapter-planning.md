# Research/Knowledge demo lifecycle Tauri adapter planning

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-28

## Scope

This documentation-only increment defines a future narrow Tauri adapter around
the completed volatile lifecycle core. It creates no command, event, managed
state, WebView client, UI connection, capability, CSP, dependency, or runtime
behavior.

## Planned evidence

- An exact source ExecPlan for no-input command/event, ordering, cleanup,
  client narrowing, F-12, tests, target-Mac evidence, rollback, and stops.
- Current-state synchronization distinguishing that plan from implemented IPC
  and from a later connected Command Center presentation.
- Documentation, repository, security, and diff checks. Target-Mac/rendered
  checks are `Not run` because source is unchanged.

## Approval boundary

The owner approved planning only. Source implementation needs separate explicit
approval after this plan is reviewed as Ready.

## Verification

- `npm run docs:check`: Passed.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- Target-Mac, rendered UI, native event, and source-runtime checks: Not run;
  this increment changes documentation only.

## Final results

The documentation planning increment is complete with advisories. The exact
future adapter plan is Ready with advisories, but its source gate and all
implementation remain separately owner-approved.
