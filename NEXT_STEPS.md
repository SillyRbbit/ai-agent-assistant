# Next steps

Last updated: 2026-07-14

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

## Queue status

No increment is currently marked **Ready**. Increment 4E is verified complete under D-025's scoped reviewed baseline exception. The exception adds no advisory ignore and must be revisited if the affected `quick-xml` APIs become reachable, the `plist -> Tauri` dependency path changes, or separate dependency remediation is approved.

Exact next task: the project owner selects and approves the exact scope of the next documentation-only planning task. Do not start another increment before that approval.
