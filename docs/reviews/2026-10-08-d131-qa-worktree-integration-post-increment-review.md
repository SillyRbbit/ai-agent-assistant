# D-131 QA worktree integration review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "d131-qa-worktree-integration",
  "commands_executed": [
    "/usr/bin/python3 -B -m unittest discover -s .codex/hooks/tests -p test_*.py -v",
    "/usr/local/bin/python3 -B -m unittest discover -s scripts/tests -p test_*.py -v",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "/usr/bin/python3 -B .codex/hooks/session_end_gate.py",
    "/usr/bin/python3 -B .codex/hooks/post_increment_gate.py status",
    "/usr/bin/python3 -B .codex/hooks/post_increment_gate.py stop"
  ],
  "files_changed": [
    ".agents/skills/documentation-sync/SKILL.md",
    ".agents/skills/post-increment-gate/SKILL.md",
    ".agents/skills/quality-gate/SKILL.md",
    ".agents/skills/readiness-review/SKILL.md",
    ".agents/skills/session-end/SKILL.md",
    ".agents/skills/verified-increment/SKILL.md",
    ".codex/hooks/lifecycle_closure.py",
    ".codex/hooks/post_increment_gate.py",
    ".codex/hooks/tests/test_post_increment_gate.py",
    "AGENTS.md",
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "CODE_REVIEW.md",
    "DECISIONS.md",
    "ENGINEERING_GUIDE.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PRODUCT_REQUIREMENTS.md",
    "PROJECT_STATUS.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/architecture/current/01-system.svg",
    "docs/architecture/current/02-sequence.svg",
    "docs/architecture/current/03-collaboration.svg",
    "docs/architecture/current/04-context.svg",
    "docs/architecture/current/05-lifecycle.svg",
    "docs/architecture/current/06-observability.svg",
    "docs/architecture/current/07-security.svg",
    "docs/architecture/current/08-delivery.svg",
    "docs/architecture/current/09-future.svg",
    "docs/architecture/current/README.md",
    "docs/architecture/current/capabilities.md",
    "docs/architecture/current/diagrams.json",
    "docs/architecture/current/external-projects.md",
    "docs/architecture/current/overview.pdf",
    "docs/architecture/current/render_diagrams.py",
    "docs/architecture/current/security.md",
    "docs/architecture/current/sources.json",
    "docs/architecture/current/viewer.html",
    "docs/architecture/current/walkthrough.md",
    "docs/governance/MASTER_PROMPT.md",
    "docs/plans/2026-10-07-current-architecture-walkthrough.md",
    "docs/plans/2026-10-07-isolated-bot-action-workflow.md",
    "docs/plans/2026-10-08-d131-qa-worktree-integration.md",
    "docs/reviews/2026-10-07-current-architecture-walkthrough-post-increment-review.md",
    "docs/reviews/2026-10-08-d131-qa-worktree-integration-post-increment-review.md",
    "docs/templates/INCREMENT_TEMPLATE.md",
    "docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md",
    "docs/templates/READINESS_REVIEW_TEMPLATE.md",
    "docs/workflows/END_SESSION.md",
    "docs/workflows/RESUME_SESSION.md",
    "docs/workflows/START_SESSION.md",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src-tauri/Cargo.lock",
    "src-tauri/Cargo.toml",
    "src-tauri/examples/native_approval_dialog.rs",
    "src-tauri/src/agent_adapter.rs",
    "src-tauri/src/agent_chat.rs",
    "src-tauri/src/agent_chat_tauri.rs",
    "src-tauri/src/approvals/decision_source.rs",
    "src-tauri/src/approvals/manager.rs",
    "src-tauri/src/approvals/types.rs",
    "src-tauri/src/audit/approval.rs",
    "src-tauri/src/codex_connection.rs",
    "src-tauri/src/collaboration.rs",
    "src-tauri/src/collaboration_action.rs",
    "src-tauri/src/collaboration_tauri.rs",
    "src-tauri/src/isolated_action/mod.rs",
    "src-tauri/src/isolated_action/sandbox.rs",
    "src-tauri/src/isolated_action/tests.rs",
    "src-tauri/src/isolated_action/workspace.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/storage/collaboration.rs",
    "src/features/collaboration/ActionReview.tsx",
    "src/features/collaboration/CollaborationPage.css",
    "src/features/collaboration/CollaborationPage.test.tsx",
    "src/features/collaboration/CollaborationPage.tsx",
    "src/features/command-center/collaborationProjection.test.ts",
    "src/infrastructure/tauri/collaboration-client.test.ts",
    "src/infrastructure/tauri/collaboration-client.ts"
  ],
  "findings": [
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "Action acceptance remains incomplete and legacy-admitted.",
      "risk": "Governance success must not be presented as native milestone completion.",
      "effort": "Separate evidence-supported diagnosis and authorized acceptance",
      "milestone": "isolated-bot-action-workflow",
      "blocks_completion": false,
      "blocks_next_increment": true
    }
  ],
  "manual_verification": [
    {
      "check": "Architecture, security, code-health and preservation review",
      "required": true,
      "status": "Passed"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "verification": [
    {
      "command": "/usr/bin/python3 -B -m unittest discover -s .codex/hooks/tests -p test_*.py -v",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "/usr/local/bin/python3 -B -m unittest discover -s scripts/tests -p test_*.py -v",
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
      "command": "/usr/bin/python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "/usr/bin/python3 -B .codex/hooks/post_increment_gate.py status",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "/usr/bin/python3 -B .codex/hooks/post_increment_gate.py stop",
      "required": false,
      "status": "Manual verification pending"
    }
  ]
}
-->

Date: 2026-10-08
Branch: codex/live-provider-qa
Review scope: selective governance integration, not isolated-action completion.

## Executive summary

PASS WITH ADVISORIES for the authorized governance integration. The active action
milestone remains incomplete; its raw state is unchanged and no completion marker
was issued. This standalone review does not claim a separate ordinary milestone
was begun. Same-task begin passed before editing without mutating state.

## Scope and boundaries

Ported Desktop's general closure/admission helper and D-132 absence handling;
selectively integrated it with the QA gate rather than copying Desktop's gate.
Preserved D-133's schema-4 mechanism, positional acceptance-request API, legacy
report/test bodies, fingerprint encoding and hook configuration. Mixed routes,
mixed lineages and conversion of active legacy state reject. Git-resolved index
ownership enables closure backups for actual linked worktrees without moving them.

Policy, six skills, three templates, workflow guidance and additive current
records describe objective-based operation. The integration delta is29 paths;
counts are informational. Snapshot and current hashes are recorded separately at
`/private/tmp/cortexa-d131-integration-rin1mbe4`. No product, dependency, credential,
permission, native execution, request, install, Git integration or publication.

## Verification results

- Passed:32 focused general closure/admission tests and8 focused integration tests.
- Passed:145 full hook tests, including unchanged legacy and D-133 coverage.
- Passed:95 repository tests with installed `/usr/local/bin/python3`3.12.1.
- Retained initial failure: system Python cannot execute `zip(strict=...)` in the
  unchanged repository checker;95 tests reported169 errors. Interpreter selection
  corrected the tooling route, with no product/test implementation change.
- Passed: documentation, repository, security, whitespace and session inventory.
- Passed: snapshot/preservation and review schema validation; receipts retained.
- Status: active isolated-bot-action-workflow. Full Stop command executed exit0
  but returned `decision: block`; it is NOT accepted. Manifest marks acceptance
  pending and optional for this standalone integration review, never for action
  completion. User explicitly prohibited finalizing incomplete action acceptance.

Unchanged product verification is inherited from
`/private/tmp/cortexa-example-config-71q_jh9c/FINAL-HANDOFF.md`: recorded verify exit0,
612 frontend,490 Rust library and255 integration tests, builds/Clippy/formatting.
Those tests/builds were not repeated. Existing opt-in skips and Vite/Node warnings
remain. Native Reject, exact Approve/apply and terminal-failed restart/no-replay
are inherited; correction, drift and active cancellation/recovery are not passed.

## Architecture findings

No integration blocker found. Gate remains deterministic and checkout-local;
closure/admission helpers own distinct lineage, while D-133 retains its dedicated
route. Objective-based reports enforce criteria/path attribution and protected
boundaries without a routine file-count gate. General admission cannot adopt a
D-133 maintenance pointer. Product trust boundaries are byte-unchanged.

## Security findings

No new blocker found. Fixed schemas, path/hash checks, original failure/report
preservation, protected/destructive path enforcement, absent-path handling and
atomic receipts remain. Index discovery uses Git metadata without shell parsing;
existing regular-file/symlink/size checks and exact backup equality apply. Tests
cover tampering, unsafe/missing index paths, changed staged index, mixed routes,
protected paths and false completion. No secret, app database or runtime access.
Same-user hashes remain workflow evidence, not authentication or semantic proof.

## Code-health findings

No integration blocker found. All original hook test classes are AST-identical;
D-133 implementation/tests and common fingerprint code remain byte-identical.
Focused tests exercise real linked Git worktrees plus negative fixtures. The
Desktop general helper is reused with narrow index/coexistence corrections.
No obsolete legacy validators were silently removed or reinterpreted.

## Technical debt

Advisory: inherited action acceptance is incomplete. Risk: conflating offline
integration with live correctness. Effort: separate bounded diagnosis and native
coverage; milestone isolated-bot-action-workflow. It does not block this integration
but blocks a later dependent milestone. Local receipts are not resistant to a
malicious same-user rewrite; this accepted limitation remains unchanged.

## Roadmap findings

No roadmap reorder or parked work resumed. Ledger50: three additional requests
remain, insufficient for another four-dispatch reservation. Diagnose retained
`qa_contract_schema_data` evidence without inventing missing field/value detail;
propose any product diagnostic change and sufficient paid budget separately.

## Completion decision

PASS WITH ADVISORIES for selective governance integration only. Do not finalize,
close, convert or fabricate passing state for the active action milestone.
Historical source/artifact seals stay unchanged. The first external preservation
check incorrectly compared the authorized hook delta to the historical full-source
binding; its failure is retained. Corrected comparison validates original bound
bytes from the snapshot and current unchanged product/dist/bundle bytes, with the
governance delta explicitly attributed. No historical validator was rewritten.

## Next-increment readiness

Blocked for a new dependent milestone. Safe next work is read-only diagnosis of
the remaining action rejection. D-131 allows routine work within authorized scope,
but does not authorize extra paid usage or excluded product/security changes.
ECC hooks/MCP disabled; native workaround, all advisories and D-125/M1/M2 retained.

## Exact files changed

The manifest records the entire current Git inventory, including inherited work.
Only the following paths belong to this governance integration; previous document
bodies and all other candidate bytes remain preserved:

- `AGENTS.md`
- `docs/governance/MASTER_PROMPT.md`
- `ENGINEERING_GUIDE.md`
- `CODE_REVIEW.md`
- `.agents/skills/documentation-sync/SKILL.md`
- `.agents/skills/post-increment-gate/SKILL.md`
- `.agents/skills/quality-gate/SKILL.md`
- `.agents/skills/readiness-review/SKILL.md`
- `.agents/skills/session-end/SKILL.md`
- `.agents/skills/verified-increment/SKILL.md`
- `docs/templates/INCREMENT_TEMPLATE.md`
- `docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md`
- `docs/templates/READINESS_REVIEW_TEMPLATE.md`
- `docs/workflows/END_SESSION.md`
- `docs/workflows/RESUME_SESSION.md`
- `docs/workflows/START_SESSION.md`
- `.codex/hooks/lifecycle_closure.py`
- `.codex/hooks/post_increment_gate.py`
- `.codex/hooks/tests/test_post_increment_gate.py`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-08-d131-qa-worktree-integration.md`
- `docs/reviews/2026-10-08-d131-qa-worktree-integration-post-increment-review.md`

## Exact commands executed

Commands and output/exit receipts are retained under the external evidence root.
Hooks used system Python; repository checks used Python3.12 via process-local PATH.
No toolchain/global setting changed. Full Stop had complete cwd/event/active fields.

- `/usr/bin/python3 -B -m unittest discover -s .codex/hooks/tests -p test_*.py -v`
- `/usr/local/bin/python3 -B -m unittest discover -s scripts/tests -p test_*.py -v`
- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`
- `/usr/bin/python3 -B .codex/hooks/session_end_gate.py`
- `/usr/bin/python3 -B .codex/hooks/post_increment_gate.py status`
- `/usr/bin/python3 -B .codex/hooks/post_increment_gate.py stop`
