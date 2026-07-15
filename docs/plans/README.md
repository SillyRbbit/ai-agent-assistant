# Implementation plans

Store active or completed multi-step implementation plans in this directory.

Naming convention:

```text
YYYY-MM-DD-short-kebab-case-title.md
```

Create a plan from `docs/templates/INCREMENT_TEMPLATE.md`. Link active plans from `PLANS.md` and the current `HANDOFF.md`.

Increment 4S bind terminal initial approval presentation is verified complete with uncommitted changes under [`04s-bind-terminal-initial-approval-presentation.md`](04s-bind-terminal-initial-approval-presentation.md). Its exact two-file source/test scope consumes terminal `RequireApproval` through the existing exact approval manager without native interaction, approval resolution, audit, transport, orchestration, dispatch, or execution authority. Increment 4R is verified complete, published, and merged under [`04r-bind-terminal-initial-policy.md`](04r-bind-terminal-initial-policy.md). No later increment is Ready.

Do not use a plan to hide unbounded scope. Each plan should still describe one coherent increment with explicit non-goals and verification.
