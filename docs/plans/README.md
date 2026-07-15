# Implementation plans

Store active or completed multi-step implementation plans in this directory.

Naming convention:

```text
YYYY-MM-DD-short-kebab-case-title.md
```

Create a plan from `docs/templates/INCREMENT_TEMPLATE.md`. Link active plans from `PLANS.md` and the current `HANDOFF.md`.

Increment 4N bounded initial gateway request is verified complete in the current uncommitted workspace under [`04n-bounded-initial-gateway-request.md`](04n-bounded-initial-gateway-request.md). It adds only a transport-free first-turn request envelope and exact public-boundary coverage, and no later increment is Ready. Increment 4M is verified complete, published, and merged under [`04m-remove-legacy-platform-scaffold.md`](04m-remove-legacy-platform-scaffold.md).

Do not use a plan to hide unbounded scope. Each plan should still describe one coherent increment with explicit non-goals and verification.
