# Implementation plans

Store active or completed multi-step implementation plans in this directory.

Naming convention:

```text
YYYY-MM-DD-short-kebab-case-title.md
```

Create a plan from `docs/templates/INCREMENT_TEMPLATE.md`. Link active plans from `PLANS.md` and the current `HANDOFF.md`.

Meta Increment 1 branding and identity foundation is verified complete under [`meta-01-branding-foundation.md`](meta-01-branding-foundation.md). It establishes canonical logo assets, brand guidance, the repository-local `$branding` skill, README/favicon references, and the official sidebar mark without changing behavior, compatibility identifiers, dependencies, permissions, or production Tauri icons.

Meta Increment 2 engineering operating system is verified complete under [`meta-02-engineering-operating-system.md`](meta-02-engineering-operating-system.md). It consolidates repository engineering guidance only and changes no product behavior.

Meta Increment 3 Codex automation and post-increment quality gates is verified complete and squash-merged at `ad9042c` under [`meta-03-codex-automation.md`](meta-03-codex-automation.md). It adds shared safe inspection and focused review workflows without changing product behavior.

Meta Increment 5 repository health and GitHub hygiene is verified complete but uncommitted and unpublished under [`meta-05-repository-health.md`](meta-05-repository-health.md). It adds read-only repository quality automation and governance without changing product behavior.

Meta Increment 6 verified application icon rollout is Ready under [`meta-06-verified-application-icon-rollout.md`](meta-06-verified-application-icon-rollout.md). It requires verified Meta Increment 5 completion and separate project-owner approval before generating or replacing the exact existing Tauri icon family.

Increment 4U bind initial approval run-termination is verified complete, published, and merged at `61525bf` under [`04u-bind-initial-approval-run-termination.md`](04u-bind-initial-approval-run-termination.md). Its exact two-file source/test scope lets the turn resolve only its privately retained pending approval through the existing manager. Native invocation or closure, proactive expiry, audit, runtime coordination, transport, dispatch, and execution remain excluded.

Increment 4V bind initial terminal approval audit is Proposed under [`04v-bind-initial-terminal-approval-audit.md`](04v-bind-initial-terminal-approval-audit.md). The merged 4U prerequisite is satisfied and the plan is reconciled to `61525bf`, but it remains blocked on explicit queue selection and separate project-owner approval. It adds no durable persistence, runtime coordination, transport, dispatch, or execution.

Increment 4T bind terminal initial approval resolution is verified complete, published, and merged at `244a1d8` under [`04t-bind-terminal-initial-approval-resolution.md`](04t-bind-terminal-initial-approval-resolution.md). A clean archive of that commit reproduces the stored valid `04t` fingerprint; the live marker is stale only because later planning files are present.

Do not use a plan to hide unbounded scope. Each plan should still describe one coherent increment with explicit non-goals and verification.
