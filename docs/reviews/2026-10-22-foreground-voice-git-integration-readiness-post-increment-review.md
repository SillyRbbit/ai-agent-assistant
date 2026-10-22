# Foreground voice Git integration readiness

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "foreground-voice-git-integration-readiness",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "commands_executed": [
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "docs/plans/2026-10-22-foreground-voice-git-integration.md",
    "docs/reviews/2026-10-22-foreground-voice-git-integration-readiness-post-increment-review.md"
  ],
  "findings": [],
  "verification": [
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
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Preservation and scope review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Architecture, security, code-health and readiness review",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

## Executive summary

Owner-authorized integration of already accepted voice. Original acceptance is sealed and valid through exact-content commit cffa36c5d0edd6a6cfc79ef8fb5f3aa1450a1520. This is a new current publication admission, not rebinding old acceptance.

## Scope and boundaries

Reconcile exact main91ebfe8562c8bee675fb032c4e5f7ef0ed39d909, preserve both histories in seven appended-document conflicts, additive integration records, required checks/review and authorized push/PR/squash merge. No voice/chat/provider/RAG product edit, acceptance replay, installation, resource release, HP or cleanup.

## Verification results

Read actual external preparation receipts before begin. Required product verification649 frontend/554 Rust and native acceptance are unchanged and reused with exact source bindings. Main91ebfe8 has six successful checks. New current documentation checks do not replace old product evidence.

## Architecture findings

Upstream PR154/155 add retention documentation and read-only inventory tooling; no product/migration/context overlap. RAG owns its separate persistent-memory checkout and native-acceptance gate leaf; dashboard owns its publication leaf. Neither is modified.

## Security findings

Protect all accepted src/src-tauri, package/lock, lifecycle hooks, workflows, original voice plan/readiness/final review and immutable admission. Import only exact published upstream records/tooling. No protected artifact or data read/deletion.

## Code-health findings

Complete31-path accepted diff reviewed and preserved. Stage/index/blob/mode/tree/sole-parent verified independently; original marker and actual Stop pass postcommit. No blocking source finding.

## Technical debt

Retain all original voice advisories and unknown failures. RAG parity and physical audibility/whole-host network assurances are not claimed.

## Roadmap findings

Only existing accepted voice Git integration. No new capability or successor.

## Completion decision

Preparation only. Normal schema2 begin is required before merging current main or extending current records. Required checks must actually pass.

## Next-increment readiness

Ready with advisories after passing preparation receipts and supported ordinary admission.

## Exact files changed

Only the plan and this readiness report during preparation.

## Exact commands executed

See machine manifest and git-integration-01 commands.jsonl. Historical attempts remain unchanged.
