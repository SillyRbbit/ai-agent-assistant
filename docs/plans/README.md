# Implementation plans

Store active or completed multi-step implementation plans in this directory.

Naming convention:

```text
YYYY-MM-DD-short-kebab-case-title.md
```

Create a plan from `docs/templates/INCREMENT_TEMPLATE.md`. Link active plans from `PLANS.md` and the current `HANDOFF.md`.

Increment 4P schema-bound initial gateway events is verified complete in the current uncommitted workspace under [`04p-schema-bound-initial-gateway-events.md`](04p-schema-bound-initial-gateway-events.md). Its exact two-file source/test scope makes the bound initial turn own exact local function-call schema validation without transport, credentials, continuation, policy, orchestration, or execution authority. No later increment is Ready. Increment 4O is verified complete, published, and merged under [`04o-bound-initial-gateway-turn.md`](04o-bound-initial-gateway-turn.md).

Do not use a plan to hide unbounded scope. Each plan should still describe one coherent increment with explicit non-goals and verification.
