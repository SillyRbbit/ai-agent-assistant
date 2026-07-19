# Implementation plans

Store active or completed multi-step implementation plans in this directory.

Naming convention:

```text
YYYY-MM-DD-short-kebab-case-title.md
```

Create a plan from `docs/templates/INCREMENT_TEMPLATE.md`. Link active plans from `PLANS.md` and the current `HANDOFF.md`.

The verified-complete documentation-only O-006 Phase 1 identity decision is defined in
[`o006-phase1-microsoft-personal-identity.md`](o006-phase1-microsoft-personal-identity.md).
It records Microsoft personal identity as the sole Phase 1 provider under
D-062 while deferring Google, Apple, persistent sessions, automatic email
linking, and all implementation. Its result is `PASS WITH ADVISORIES`; it was
published through PR #37 and squash-merged at `c458f27`, and no publication
action remains.

The verified-complete and published documentation-only
O-006/O-007 decision-record increment is defined under
[`o006-o007-staged-gateway-identity-retention-decisions.md`](o006-o007-staged-gateway-identity-retention-decisions.md).
It records current absence, a provider-neutral consumer/prosumer Phase 1, a
later enterprise Phase 2, separate identity-provider, Azure-first portable
cloud-hosting, and future trusted AI model-provider boundaries, and
provider-specific verified-ZDR controls without adding or authorizing product
implementation. Its approved amendment uses gate
`o006-provider-boundary-amendment` and is verified complete with advisories,
published through PR #35 from source commit `4b474b4`, and squash-merged at
`853da62`. No publication action remains.

Meta risk-based GitHub Actions validation is locally verified with publication and hosted execution pending under [`meta-risk-based-ci.md`](meta-risk-based-ci.md). It replaces blanket persistent-runner checks with two read-only GitHub-hosted workflows and preserves the complete local final increment gate without changing application behavior or dependencies.

Meta Increment 1 branding and identity foundation is verified complete under [`meta-01-branding-foundation.md`](meta-01-branding-foundation.md). It establishes canonical logo assets, brand guidance, the repository-local `$branding` skill, README/favicon references, and the official sidebar mark without changing behavior, compatibility identifiers, dependencies, permissions, or production Tauri icons.

Meta Increment 2 engineering operating system is verified complete under [`meta-02-engineering-operating-system.md`](meta-02-engineering-operating-system.md). It consolidates repository engineering guidance only and changes no product behavior.

Meta Increment 3 Codex automation and post-increment quality gates is verified complete and squash-merged at `ad9042c` under [`meta-03-codex-automation.md`](meta-03-codex-automation.md). It adds shared safe inspection and focused review workflows without changing product behavior.

Meta Increment 5 repository health and GitHub hygiene is verified complete and squash-merged at `6b149fa` under [`meta-05-repository-health.md`](meta-05-repository-health.md). It adds read-only repository quality automation and governance without changing product behavior.

Meta Increment 6 is the documentation-only Product Readiness Audit, squash-merged at `5281fac` with result `NOT READY (57/100)` and recorded under [`../reviews/2026-07-16-product-readiness-audit.md`](../reviews/2026-07-16-product-readiness-audit.md).

Meta Increment 7 verified application icon rollout is verified complete with advisories and squash-merged through PR #19 at `96ba6ae` under [`meta-07-verified-application-icon-rollout.md`](meta-07-verified-application-icon-rollout.md). Exactly the existing 16 Tauri icon files derive from the canonical source; debug and release app bundles use Cortexa, with the approved raw `tauri dev` generic-icon advisory in D-051 and ICNS semantic-verification rule in D-052.

Repository Workflow Increment trusted self-hosted runner routing is verified complete with advisories and squash-merged through PR #24 at `eaf6c9f` under [`repository-self-hosted-runner.md`](repository-self-hosted-runner.md). Final branch commit `cfa976f` passed CI, Documentation, and Security on runner 21. The approved private-only Rust portability correction changes no public contract or target-Mac behavior.

Increment 4U bind initial approval run-termination is verified complete, published, and merged at `61525bf` under [`04u-bind-initial-approval-run-termination.md`](04u-bind-initial-approval-run-termination.md). Its exact two-file source/test scope lets the turn resolve only its privately retained pending approval through the existing manager. Native invocation or closure, proactive expiry, audit, runtime coordination, transport, dispatch, and execution remain excluded.

Increment 4V bind initial terminal approval audit is verified complete and squash-merged through PR #23 at `6e6f91d` from reconstructed source commit `ec919e9` under [`04v-bind-initial-terminal-approval-audit.md`](04v-bind-initial-terminal-approval-audit.md). Its exact source/test and closeout scope passed local and hosted verification, and the `04v` marker remains valid. It adds no durable persistence, runtime coordination, transport, dispatch, or execution.

Increment 4T bind terminal initial approval resolution is verified complete, published, and merged at `244a1d8` under [`04t-bind-terminal-initial-approval-resolution.md`](04t-bind-terminal-initial-approval-resolution.md). A clean archive of that commit reproduces the stored valid `04t` fingerprint; the live marker is stale only because later planning files are present.

Do not use a plan to hide unbounded scope. Each plan should still describe one coherent increment with explicit non-goals and verification.
