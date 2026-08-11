# Hermes ADR transport revision post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "python3 .codex/hooks/post_increment_gate.py begin --increment hermes-adr-transport-revision",
    "python3 .codex/hooks/post_increment_gate.py status",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "awk '/^\\+#{1,6}[[:space:]]/{found=1} END{exit found}' CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md TROUBLESHOOTING_LOG.md docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md docs/plans/2026-08-11-hermes-adr-transport-revision.md docs/reviews/2026-08-11-hermes-adr-transport-revision-post-increment-review.md",
    "python3 .codex/hooks/session_end_gate.py",
    "./node_modules/.bin/prettier --write TROUBLESHOOTING_LOG.md docs/reviews/2026-08-11-hermes-adr-transport-revision-post-increment-review.md"
  ],
  "files_changed": [
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md",
    "docs/plans/2026-08-11-hermes-adr-transport-revision.md",
    "docs/reviews/2026-08-11-hermes-adr-transport-revision-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Roadmap",
      "effort": "Owner architecture decision and separately bounded evidence plan",
      "milestone": "Future runtime decision",
      "risk": "Implementing a runtime or selecting a Hermes transport without accepted evidence would violate the raw-stdio NO-GO and current trust-boundary controls.",
      "severity": "Advisory",
      "summary": "No runtime implementation increment is Ready."
    }
  ],
  "increment_id": "hermes-adr-transport-revision",
  "manual_verification": [
    {
      "check": "No manual application check is required because this increment changes documentation only and no runtime path changed.",
      "required": false,
      "status": "Passed"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "git status --short --branch",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/post_increment_gate.py begin --increment hermes-adr-transport-revision",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/post_increment_gate.py status",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "awk '/^\\+#{1,6}[[:space:]]/{found=1} END{exit found}' CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md TROUBLESHOOTING_LOG.md docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md docs/plans/2026-08-11-hermes-adr-transport-revision.md docs/reviews/2026-08-11-hermes-adr-transport-revision-post-increment-review.md",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-08-11
Increment: hermes-adr-transport-revision
Branch: main

## Executive summary

Completed the documentation-only revision of the Proposed multi-runtime ADR.
It removes the failed raw TUI-gateway stdio preference, records that mechanism
as rejected for the evaluated release, and keeps native-only, Hermes ACP, and
Hermes serve unselected. No source, dependency, runtime, process, credential,
provider, UI, or external behavior changed. All required documentation checks
passed. The quality-gate result is PASS WITH ADVISORIES because no runtime
implementation increment is Ready. The owner-authorized precommit review also
corrected an accidental literal `+` before the TS-018 heading and reran the
required checks.

## Scope and boundaries

The approved goal was to reconcile the ADR with the completed transport spike
without accepting the ADR or selecting another transport. The changed-file
inventory stays within architecture documentation, project memory, the active
plan, review evidence, changelog, and local workflow troubleshooting. No trust
boundary changed; the documents retain deterministic Rust ownership and require
a later owner decision, containment, provenance, protocol, capability, and
packaging evidence before any Hermes adapter can be considered.

## Verification results

- Passed: git status showed the expected documentation-only working set on main.
- Passed: the required gate began as hermes-adr-transport-revision after
  explicitly approved elevated local workspace permission.
- Passed: post-increment status reported the expected active increment before
  finalization.
- Passed: npm run docs:check after the first run reported formatting only in the
  new review and troubleshooting entry; the formatter corrected those files.
- Passed: npm run repository:check.
- Passed: npm run security:scan.
- Passed: git diff --check.
- Passed: the accidental-patch-marker check found no literal leading `+` before
  a Markdown heading in any of the nine increment paths.
- Passed: python3 .codex/hooks/session_end_gate.py; no conflicts, staged paths,
  or unexpected paths were reported.
- Passed: ./node_modules/.bin/prettier --write TROUBLESHOOTING_LOG.md
  docs/reviews/2026-08-11-hermes-adr-transport-revision-post-increment-review.md.
- Passed: no manual application check is required because no application path
  changed.

## Architecture findings

No findings. The ADR remains Proposed, distinguishes rejected, unselected, and
conceptual states, keeps external-runtime types adapter-local, and preserves
native/default/fallback and deterministic-Rust ownership. No current capability
is overstated and no new abstraction or dependency was introduced.

## Security findings

No findings. The change adds no IPC, process launch, networking, capability,
credential, secret, filesystem, storage, permission, model, tool, policy,
approval, audit, or supply-chain behavior. The revised ADR retains the
requirements for immutable provenance, whole-process containment, restricted
capabilities, redaction, cancellation, and closed protocol translation.

## Code-health findings

No findings. There is no source or test change. Markdown formatting, links, and
repository consistency passed their documented checks. The plan, current-state
records, and review distinguish current, planned, rejected, and unselected
behavior.

## Technical debt

None introduced. The intentional roadmap block is recorded separately below;
it is not implementation debt created by this documentation-only increment.

## Roadmap findings

Advisory: no runtime implementation increment is Ready. Raw TUI-gateway stdio
remains rejected at Hermes Agent 0.20.0 / v2026.8.3. Native-only, Hermes ACP,
and Hermes serve are unselected; an owner architecture decision, separately
bounded evidence plan, and fresh readiness review are required before any
runtime source work.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Blocked

The exact next task is an owner-selected architecture decision: retain
native-only behavior, accept a narrow native-runtime direction without Hermes,
or authorize a separately planned comparison of one supported public Hermes
surface. Do not select a transport by inference, accept the current Proposed
ADR, execute Hermes, or begin adapter work.

## Exact files changed

- CHANGELOG.md
- HANDOFF.md
- NEXT_STEPS.md
- PLANS.md
- PROJECT_STATUS.md
- TROUBLESHOOTING_LOG.md
- docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md
- docs/plans/2026-08-11-hermes-adr-transport-revision.md
- docs/reviews/2026-08-11-hermes-adr-transport-revision-post-increment-review.md

## Exact commands executed

- Passed: git status --short --branch
- Passed: python3 .codex/hooks/post_increment_gate.py begin --increment hermes-adr-transport-revision
- Passed: python3 .codex/hooks/post_increment_gate.py status
- Passed: npm run docs:check
- Passed: npm run repository:check
- Passed: npm run security:scan
- Passed: git diff --check
- Passed: awk '/^\+#{1,6}[[:space:]]/{found=1} END{exit found}' CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md TROUBLESHOOTING_LOG.md docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md docs/plans/2026-08-11-hermes-adr-transport-revision.md docs/reviews/2026-08-11-hermes-adr-transport-revision-post-increment-review.md
- Passed: python3 .codex/hooks/session_end_gate.py
- Passed: ./node_modules/.bin/prettier --write TROUBLESHOOTING_LOG.md
  docs/reviews/2026-08-11-hermes-adr-transport-revision-post-increment-review.md
