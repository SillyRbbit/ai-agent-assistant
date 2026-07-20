# Repository governance - Codex instruction hierarchy

Status: Verified complete with advisories
Date: 2026-07-20
Gate ID: `repository-governance-codex-instruction-hierarchy`
Plan: `docs/plans/repository-governance-codex-instruction-hierarchy.md`
Baseline: clean synchronized `main` at `36ce9ab`

## Goal

Establish a concise root instruction entry point and one detailed, authoritative
Codex instruction document without changing product scope, behavior, or the
approved Phase 4 roadmap.

## Delivered scope

- Replaced the oversized root `AGENTS.md` with a concise mandatory entry point
  that preserves essential trust, Git, validation, and completion safeguards.
- Added `docs/governance/MASTER_PROMPT.md` as the durable hierarchy, planning,
  validation, documentation, and final-response authority below platform and
  system instructions.
- Kept all twenty-three reusable prompt/template assets in their existing
  locations, added the shared governing preamble, and retained their
  task-specific goals, validation, and stop conditions.
- Recorded D-065 and synchronized only the live ARB-002A publication wording
  required to keep project memory accurate.

## Boundaries preserved

- No application source, test, dependency, lockfile, workflow, hook, skill,
  CI, Tauri, IPC, database, runtime, deployment, credential, or UI path changed.
- No prompt was deleted, renamed, or relocated.
- Multi-agent work remains a documented future concept; no subagent,
  orchestration, or model-switching automation was added.
- Stage B, Stage C, Stage D, and ARB-002 runtime work remain blocked.

## Verification

Final command results and the exact changed-file inventory are recorded in the
post-increment review. The completion marker is valid only after every required
documentation-tier command and review passes.

## Rollback

Before publication, restore only this bounded documentation scope. After
publication, revert the single bounded documentation commit and supersede
D-065 additively if needed. No product or operational rollback applies.
