# PR #57 Linux Clippy portability remediation

Status: Active
Owner: Project owner
Date: 2026-08-25
Gate ID: `pr57-linux-clippy-portability`
Plan:
[`2026-08-25-pr57-linux-clippy-portability.md`](../plans/2026-08-25-pr57-linux-clippy-portability.md)
Baseline: `28a0c46b24d2865a165dc6e11ec082e6dec61fae`
Pull request: `#57`

## Goal

Restore strict Linux all-target Clippy by matching private Rust compilation
scope to existing macOS-only consumers, without changing public behavior,
authority, credential handling, or the completed deterministic acceptance
increment.

## Approved scope

- One macOS-only orchestrator test import.
- One private macOS-only approval identity matcher.
- Private Cloudflare reader and validation helpers compiled on macOS or in
  tests, while the public non-macOS probe remains fail-closed.
- Exact focused, complete target-Mac, remote Linux, security, and closeout
  evidence.

## Explicit boundaries

No lint suppression, workflow bypass, dependency update, public API change,
approval or execution behavior change, real credential ingestion, provider,
network, IPC, permission, capability, or external effect is authorized. The
separate npm advisory remediation may begin only after this increment closes.

## Current status

The exact attribute-only correction is implemented. Focused approval and
credential tests, strict all-target/all-feature Clippy, all-target Rust tests,
complete `npm run verify`, and independent interim architecture, security, and
code review pass locally. The first full verification attempt stopped only on
formatting in this new record; Prettier corrected it and the complete rerun
passed.

The gate remains active and quality remains `FAIL` only because the reviewed
correction has not yet been published for the required Linux and target-Mac
Rust validation. The separate npm advisory remediation has not started, and PR
#57 remains unmerged.
