# Implementation plans

Store active or completed multi-step implementation plans in this directory.

Naming convention:

```text
YYYY-MM-DD-short-kebab-case-title.md
```

Create a plan from `docs/templates/INCREMENT_TEMPLATE.md`. Link active plans from `PLANS.md` and the current `HANDOFF.md`.

Increment 4O bound initial gateway turn is verified complete in the current uncommitted workspace under [`04o-bound-initial-gateway-turn.md`](04o-bound-initial-gateway-turn.md). Its exact two-file source/test scope binds the verified request bytes and response validator without transport, credentials, continuation, orchestration, or execution authority. No later implementation increment is Ready. Increment 4N is verified complete, published, and merged under [`04n-bounded-initial-gateway-request.md`](04n-bounded-initial-gateway-request.md).

Do not use a plan to hide unbounded scope. Each plan should still describe one coherent increment with explicit non-goals and verification.
