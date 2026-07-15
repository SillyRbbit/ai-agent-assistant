# Next steps

Last updated: 2026-07-15

This file is the ordered implementation queue. Work only on the first item marked **Ready**. A verification-pending increment must close before later feature work begins.

## Completed increments

- Increment 2A — core interfaces and deterministic mocks: **Verified complete**.
- Increment 2B-0 — SQLite storage decision: **Complete**.
- Increment 2B-1 — SQLite dependency and migration skeleton: **Verified complete**.
- Increment 2B-1A — Rust 1.90 compatibility repair: **Verified complete**.
- Increment 2C — storage startup integration: **Verified complete**.
- Increment 2D — macOS menu-bar and window lifecycle: **Verified complete**.
- Increment 2E — React application shell: **Verified complete**.
- Increment 2F — mocked assistant interaction shell: **Verified complete**.
- Increment 2G — integration hardening: **Verified complete**.
- Increment 3A — in-memory conversation sessions: **Verified complete**.
- Increment 3B — mock context provenance: **Verified complete**.
- Increment 3C — simulated tool result: **Verified complete**.
- Phase 3 completion gap analysis: **Complete**.
- Increment 3D — bounded mock-loop completion: **Verified complete**.
- Phase 4 gateway and Responses security-boundary planning: **Complete**.
- Increment 4A — deterministic gateway protocol contract: **Verified complete**.
- Increment 4B — exact local tool-schema validation: **Verified complete**.
- Increment 4C — trusted policy-input binding: **Verified complete**.
- Increment 4D — exact approval binding: **Verified complete**.
- Increment 4E — trusted approval-decision source: **Verified complete**.
- Increment 4F - Cortexa product display rename: **Verified complete by project-owner direction**.
- Repository Workflow Increment 4G - automated post-increment gate: **Verified complete**.
- Increment 4H - typed approval-audit adapter: **Verified complete**.
- Increment 4I - remove generic audit scaffold: **Verified complete**.
- Repository Workflow Increment 4J - deletion-stable post-increment fingerprint: **Verified complete**.

## Queue status

Repository Workflow Increment 4G is **Verified complete**. It adds only the repository-local post-increment skill, deterministic Stop-hook validator, focused tests, report assets, and workflow documentation required to replace D-027's one-time exception. Its consolidated result is `PASS WITH ADVISORIES`; the advisory is the documented project-hook trust/bypass boundary.

Increment 4H typed approval-audit adapter is **Verified complete**. It adds one bounded transport-free in-memory Rust adapter that derives a closed redacted record from an exact terminal `ApprovalResolution`, revalidates the complete disposition/evidence matrix, and returns only a non-authorizing sequence receipt. It adds no durable persistence, runtime coordinator, dispatch, execution, IPC, UI, or live networking. Its consolidated result is `PASS WITH ADVISORIES`; the advisory is the explicit non-durable boundary and the absence of an approved next increment.

Repository Workflow Increment 4J is **Verified complete, published, and merged**. It corrects only the deterministic completion fingerprint for reviewed tracked deletions and adds positive and negative regression coverage. Its publication prerequisite for 4I reconstruction is satisfied.

Increment 4I remove generic audit scaffold is **Verified complete after reconstruction**. It deletes only the unused `audit::logger` and `audit::types` modules and removes their exports, preserving the typed `audit::approval` adapter unchanged. The corrected 4J gate remains intact, the stale-symbol scan has no matches, focused and complete checks pass, and no replacement abstraction, persistence, coordinator, dispatch, executor, IPC, UI, networking, dependency, migration, or permission was added.

Exact next task: wait for explicit project-owner direction to commit, push, and merge reconstructed 4I. No later product implementation item is marked **Ready**; do not infer or start another increment automatically.
