# Implementation plans

Store active or completed multi-step implementation plans in this directory.

Naming convention:

```text
YYYY-MM-DD-short-kebab-case-title.md
```

Create a plan from `docs/templates/INCREMENT_TEMPLATE.md`. Link active plans from `PLANS.md` and the current `HANDOFF.md`.

Increment 4U bind initial approval run-termination is verified complete with uncommitted changes and a valid `04u` marker under [`04u-bind-initial-approval-run-termination.md`](04u-bind-initial-approval-run-termination.md). Its exact two-file source/test scope lets the turn resolve only its privately retained pending approval through the existing manager. Native invocation or closure, proactive expiry, audit, runtime coordination, transport, dispatch, and execution remain excluded. Publication requires explicit project-owner direction.

Increment 4V bind initial terminal approval audit is Proposed under [`04v-bind-initial-terminal-approval-audit.md`](04v-bind-initial-terminal-approval-audit.md). It would bind exact native and run-termination resolutions to the turn's private typed in-memory audit adapter, but it remains blocked on 4U publication/merge, plan reconciliation, and separate project-owner approval. It adds no durable persistence, runtime coordination, transport, dispatch, or execution.

Increment 4T bind terminal initial approval resolution is verified complete, published, and merged at `244a1d8` under [`04t-bind-terminal-initial-approval-resolution.md`](04t-bind-terminal-initial-approval-resolution.md). A clean archive of that commit reproduces the stored valid `04t` fingerprint; the live marker is stale only because later planning files are present.

Do not use a plan to hide unbounded scope. Each plan should still describe one coherent increment with explicit non-goals and verification.
