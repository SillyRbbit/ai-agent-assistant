# Implementation plans

Store active or completed multi-step implementation plans in this directory.

Naming convention:

```text
YYYY-MM-DD-short-kebab-case-title.md
```

Create a plan from `docs/templates/INCREMENT_TEMPLATE.md`. Link active plans from `PLANS.md` and the current `HANDOFF.md`.

Increment 4M remove legacy platform scaffold is verified complete in the current uncommitted workspace under [`04m-remove-legacy-platform-scaffold.md`](04m-remove-legacy-platform-scaffold.md). It deletes only the disconnected generic Rust platform module and its crate export, and no later increment is Ready. Increment 4L is verified complete, published, and merged under [`04l-remove-legacy-memory-scaffold.md`](04l-remove-legacy-memory-scaffold.md), and Increment 4K is verified complete, published, and merged under [`04k-remove-legacy-provider-scaffold.md`](04k-remove-legacy-provider-scaffold.md).

Do not use a plan to hide unbounded scope. Each plan should still describe one coherent increment with explicit non-goals and verification.
