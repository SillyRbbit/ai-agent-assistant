# Native multi-agent desktop UI

Status: Blocked; draft follow-on, not approved for implementation
Owner: Project owner
Last updated: 2026-08-11
Blocked on: stable verified backend catalog, task, governance, cancellation,
attribution, and disclosure contracts plus a frontend readiness review
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Future goal: present the agent catalog, activation state, bounded task lineage,
progress, results, cancellation, and approval explanations through narrow typed
Tauri IPC without granting the WebView execution authority.

## Provisional scope

- Clearly distinguish available, disabled, preview-only, running, terminal,
  and failed states without calling an unwired agent operational.
- Show exact agent/task attribution, parent lineage, progress, bounded results,
  and cancellation controls.
- Explain application policy and approval outcomes without presenting an
  advisory agent as the decision-maker.
- Preserve accessibility, keyboard navigation, disclosure, redaction, and
  deterministic frontend fixtures.
- Narrow Rust-owned commands/events derived only after backend contracts settle.

## Explicit non-goals

- No React import of runtime/framework payloads, direct WebView-to-device
  execution, arbitrary agent activation, tool authority, credentials, raw logs,
  or secrets.
- No polished marketplace, user-defined agents, remote workers, or public
  plugin UI.
- No backend, provider, workflow, or tool implementation through this draft.

## Required work before Ready

Record exact IPC types, command/event allowlists, state ownership, stale/replay
handling, cancellation behavior, content limits, disclosure copy, accessibility
tests, screenshots/manual checks, security review, and rollback.

## Authority and rollback

This draft grants no IPC or UI implementation authority. Removing it preserves
the existing deterministic React mock and current command surface.
