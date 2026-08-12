# Native multi-agent end-to-end demonstrations

Status: Blocked; draft follow-on, not approved for implementation
Owner: Project owner
Last updated: 2026-08-11
Blocked on: the selected workflow's verified backend, governance, data/tool,
UI, disclosure, and environment-specific prerequisites
Roadmap:
[`NATIVE_MULTI_AGENT_ROADMAP.md`](../roadmap/NATIVE_MULTI_AGENT_ROADMAP.md)

## Goal

Future goal: demonstrate selected native multi-agent workflows end to end with
bounded deterministic or separately approved inputs and exact capability
disclosure.

## Provisional scope

- Select one already verified workflow family per demonstration.
- Prefer deterministic no-I/O fixtures before any separately approved provider,
  tool, document, repository, cloud, or platform evidence.
- Show exact agent/task lineage, orchestrator sequencing, progress,
  cancellation, governed decisions, results, and synthesis.
- Distinguish simulated, fixture-backed, read-only, approved effect, and
  unavailable capabilities.
- Preserve reproducible scripts, expected outputs, failure cases, cleanup, and
  rollback evidence.

## Explicit non-goals

- No demo-only bypass, hidden live model, real credential, unrestricted file or
  network access, privileged action, autonomous effect, or capability claim
  beyond verified evidence.
- No agent activation solely for presentation.
- No release, public deployment, benchmark, or production-readiness claim.

## Required work before Ready

Choose the exact workflow and environment, resolve every external dependency and
disclosure, define deterministic fixtures and manual evidence, specify failure
and cleanup, run architecture/security/privacy review, and record rollback.

## Authority and rollback

This draft grants no demo execution or external-state authority. Removing it
leaves all verified workflow components independently intact.
