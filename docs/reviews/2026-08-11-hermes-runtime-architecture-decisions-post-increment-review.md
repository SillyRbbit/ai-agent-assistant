# Hermes runtime architecture decisions post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "python3 .codex/hooks/post_increment_gate.py begin --increment hermes-runtime-architecture-decisions",
    "python3 .codex/hooks/post_increment_gate.py status",
    "npm run docs:check",
    "./node_modules/.bin/prettier --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md docs/adr/ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md docs/architecture/HERMES_INTEGRATION_ASSESSMENT.md docs/plans/2026-08-11-hermes-agent-runtime-adapter.md docs/plans/2026-08-11-hermes-runtime-architecture-decisions.md docs/plans/2026-08-11-hermes-serve-websocket-spike.md docs/plans/2026-08-11-native-agent-runtime-boundary.md docs/reviews/2026-08-11-hermes-runtime-architecture-decisions-post-increment-review.md docs/spikes/HERMES_TRANSPORT_SPIKE.md",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "git status --short -- src src-tauri package.json package-lock.json .github .codex .agents",
    "awk '/^\\+#{1,6}[[:space:]]/{found=1} END{exit found}' ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md docs/adr/ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md docs/architecture/HERMES_INTEGRATION_ASSESSMENT.md docs/plans/2026-08-11-hermes-agent-runtime-adapter.md docs/plans/2026-08-11-hermes-runtime-architecture-decisions.md docs/plans/2026-08-11-hermes-serve-websocket-spike.md docs/plans/2026-08-11-native-agent-runtime-boundary.md docs/reviews/2026-08-11-hermes-runtime-architecture-decisions-post-increment-review.md docs/spikes/HERMES_TRANSPORT_SPIKE.md",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "docs/adr/ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md",
    "docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md",
    "docs/architecture/HERMES_INTEGRATION_ASSESSMENT.md",
    "docs/plans/2026-08-11-hermes-agent-runtime-adapter.md",
    "docs/plans/2026-08-11-hermes-runtime-architecture-decisions.md",
    "docs/plans/2026-08-11-hermes-serve-websocket-spike.md",
    "docs/plans/2026-08-11-native-agent-runtime-boundary.md",
    "docs/reviews/2026-08-11-hermes-runtime-architecture-decisions-post-increment-review.md",
    "docs/spikes/HERMES_TRANSPORT_SPIKE.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Roadmap",
      "effort": "Owner review and separately authorized publication of the completed documentation increment",
      "milestone": "Phase 2 publication handoff",
      "risk": "Starting native implementation while the accepted decisions and Ready plan remain uncommitted would break the clean-baseline and durable-authority requirements.",
      "severity": "Advisory",
      "summary": "The native plan is Ready, but execution is operationally blocked until this intentionally uncommitted Phase 2 scope is reviewed and published separately."
    }
  ],
  "increment_id": "hermes-runtime-architecture-decisions",
  "manual_verification": [
    {
      "check": "Pinned Hermes v0.20.0 / v2026.8.3 / 3c27eb6234bf91b8ceee9e9071591b31e9b148cb source conformance, transport semantics, and phase ordering were reviewed against official upstream evidence.",
      "required": true,
      "status": "Passed"
    },
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
      "command": "python3 .codex/hooks/post_increment_gate.py begin --increment hermes-runtime-architecture-decisions",
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
      "command": "git status --short -- src src-tauri package.json package-lock.json .github .codex .agents",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "awk '/^\\+#{1,6}[[:space:]]/{found=1} END{exit found}' ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md docs/adr/ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md docs/architecture/HERMES_INTEGRATION_ASSESSMENT.md docs/plans/2026-08-11-hermes-agent-runtime-adapter.md docs/plans/2026-08-11-hermes-runtime-architecture-decisions.md docs/plans/2026-08-11-hermes-serve-websocket-spike.md docs/plans/2026-08-11-native-agent-runtime-boundary.md docs/reviews/2026-08-11-hermes-runtime-architecture-decisions-post-increment-review.md docs/spikes/HERMES_TRANSPORT_SPIKE.md",
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
Increment: hermes-runtime-architecture-decisions
Branch: main

## Executive summary

Completed the owner-authorized documentation-only runtime architecture decision
increment. D-079 accepts the small application-owned multi-runtime target and
makes the native boundary plan Ready. D-080 accepts managed local `hermes serve`
plus a closed TUI-gateway JSON-RPC/WebSocket projection only as a conditional
contained-spike direction after native completion. Raw TUI-gateway stdio remains
NO-GO, ACP remains deferred, and the Hermes adapter remains Draft/Blocked. No
runtime source, test, dependency, Hermes execution, process, socket, credential,
provider, UI, or behavior changed. The result is PASS WITH ADVISORIES because
the intentionally uncommitted Phase 2 scope must be reviewed and published
separately before the Ready native implementation may begin.

## Scope and boundaries

The exact 16-path inventory contains only architecture decisions, current-state
documentation, future plans, changelog, and review evidence. It records an
accepted target without claiming implementation. Native remains default,
reference, deterministic test path, explicit fallback, and independently usable.
The future Hermes phases remain isolated behind whole-distribution provenance,
read-only/offline launch, endpoint-level network and Unix-socket denial, closed
protocol translation, tool prohibition, and containment-wide cleanup. No
production, test, manifest, lockfile, configuration, workflow, hook, skill, or
external state entered scope.

## Verification results

- Passed: the Phase 2 gate began from the clean synchronized Phase 1 commit
  `b44b988` and reports the expected active increment before finalization.
- Passed: the final documentation format and link check. A preliminary run found
  formatting only in the three new plans; the formatter corrected them before
  the final passing run.
- Passed: repository structure and memory consistency checks.
- Passed: secret/security scan.
- Passed: whitespace/error diff check.
- Passed: protected source, test, dependency, configuration, workflow, hook, and
  skill paths are absent from the changed set.
- Passed: no literal patch-marker prefix appears before a Markdown heading in
  any of the 16 paths.
- Passed: session-end inventory reports the expected documentation-only scope.
- Passed: no manual application check is required because no application path
  changed.

## Architecture findings

No blocking finding remains. Review against the pinned upstream source corrected
the WebSocket route owner and exact token-query URL, added POSIX `[pty]` to the
`[web]` prerequisite, modeled interrupt as cooperative acknowledgement followed
by a bounded redacted parser for the pinned human-oriented session-status text,
added mandatory `session.info`/`message.start` event shapes, and required cleanup
beyond a parent process group. D-079/D-080 now distinguish accepted target,
conditional evaluation, Ready native work, Blocked spike work, Draft/Blocked
adapter work, and current absence without reviving D-032's removed provider API.

## Security findings

No blocking finding remains. The accepted transport conditions now account for
the pinned lazy-dependency and update-check paths, read-only whole-distribution
provenance, exact fake-provider-only egress, denial of every other loopback and
Unix-domain socket destination, explicit denial of distribution-root,
isolated-home, machine-managed, and external dotenv/managed-secret sources,
secret query-URL redaction, background maintenance/change-watcher activity, and
containment-wide termination of detached descendants. These are future spike
stop conditions; no process, network, credential, filesystem, permission, or
dependency action occurred.

## Code-health findings

No findings. There is no source or test change. The documentation uses one
application-owned contract vocabulary, closed status labels, exact version/tag/
commit identifiers, bounded phase dependencies, and explicit rollback paths.
Formatting, links, and repository checks pass.

## Technical debt

None introduced. The unresolved distribution, containment, dependency, and
target-Mac choices are explicit future spike/adapter prerequisites rather than
hidden implementation debt. Each blocks its own future phase.

## Roadmap findings

Advisory: the native runtime boundary plan is the sole next Ready implementation
plan, but it is operationally Blocked until this completed Phase 2 documentation
scope is separately reviewed, committed, and pushed to restore a clean durable
baseline. The contained WebSocket spike remains Blocked until verified native
completion and fresh security/readiness review. The Hermes adapter remains
Draft/Blocked until both phases pass and exact dependencies/scope are approved.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Blocked

The exact next task is owner review and separately authorized publication of
this completed 16-path documentation-only Phase 2 scope. Do not begin the Ready
native implementation while these decisions are uncommitted. Do not begin the
WebSocket spike or Hermes adapter automatically.

## Exact files changed

- ARCHITECTURE.md
- CHANGELOG.md
- DECISIONS.md
- HANDOFF.md
- NEXT_STEPS.md
- PLANS.md
- PROJECT_STATUS.md
- docs/adr/ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md
- docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md
- docs/architecture/HERMES_INTEGRATION_ASSESSMENT.md
- docs/plans/2026-08-11-hermes-agent-runtime-adapter.md
- docs/plans/2026-08-11-hermes-runtime-architecture-decisions.md
- docs/plans/2026-08-11-hermes-serve-websocket-spike.md
- docs/plans/2026-08-11-native-agent-runtime-boundary.md
- docs/reviews/2026-08-11-hermes-runtime-architecture-decisions-post-increment-review.md
- docs/spikes/HERMES_TRANSPORT_SPIKE.md

## Exact commands executed

- Passed: git status --short --branch
- Passed: python3 .codex/hooks/post_increment_gate.py begin --increment
  hermes-runtime-architecture-decisions
- Passed: python3 .codex/hooks/post_increment_gate.py status
- Passed: npm run docs:check (preliminary formatting-only failure corrected;
  final run passed)
- Passed: ./node_modules/.bin/prettier --write ARCHITECTURE.md CHANGELOG.md
  DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md
  docs/adr/ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md
  docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md
  docs/architecture/HERMES_INTEGRATION_ASSESSMENT.md
  docs/plans/2026-08-11-hermes-agent-runtime-adapter.md
  docs/plans/2026-08-11-hermes-runtime-architecture-decisions.md
  docs/plans/2026-08-11-hermes-serve-websocket-spike.md
  docs/plans/2026-08-11-native-agent-runtime-boundary.md
  docs/reviews/2026-08-11-hermes-runtime-architecture-decisions-post-increment-review.md
  docs/spikes/HERMES_TRANSPORT_SPIKE.md
- Passed: npm run repository:check
- Passed: npm run security:scan
- Passed: git diff --check
- Passed: git status --short -- src src-tauri package.json package-lock.json
  .github .codex .agents
- Passed: awk '/^\+#{1,6}[[:space:]]/{found=1} END{exit found}' ARCHITECTURE.md
  CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md
  docs/adr/ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md
  docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md
  docs/architecture/HERMES_INTEGRATION_ASSESSMENT.md
  docs/plans/2026-08-11-hermes-agent-runtime-adapter.md
  docs/plans/2026-08-11-hermes-runtime-architecture-decisions.md
  docs/plans/2026-08-11-hermes-serve-websocket-spike.md
  docs/plans/2026-08-11-native-agent-runtime-boundary.md
  docs/reviews/2026-08-11-hermes-runtime-architecture-decisions-post-increment-review.md
  docs/spikes/HERMES_TRANSPORT_SPIKE.md
- Passed: python3 .codex/hooks/session_end_gate.py
