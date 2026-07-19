# O-006 Phase 1 Azure OpenAI provider decision review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "python3 .codex/hooks/post_increment_gate.py begin --increment o006-phase1-azure-openai-provider-decision",
    "official Microsoft documentation review for Azure OpenAI privacy, retention controls, managed identity, RBAC, and Responses",
    "npx prettier --write approved documentation scope",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "test \"$(git status --porcelain | wc -l | tr -d ' ')\" = \"17\"",
    "git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.toml Cargo.lock .github .codex .agents scripts",
    "git diff --exit-code -- preserved O-006/O-007 and D-062 plans, increment records, and review reports",
    "rg -n \"D-063|Azure OpenAI|managed identity|ContentLogging=false|synthetic|automatic fallback|ARB-002\" authoritative live documents",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment o006-phase1-azure-openai-provider-decision --report docs/reviews/2026-07-19-o006-phase1-azure-openai-provider-decision-post-increment-review.md",
    "python3 .codex/hooks/post_increment_gate.py status"
  ],
  "files_changed": [
    "AGENTS.md",
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PRODUCT_REQUIREMENTS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "docs/increments/o006-phase1-azure-openai-provider-decision.md",
    "docs/plans/README.md",
    "docs/plans/o006-phase1-azure-openai-provider.md",
    "docs/reviews/2026-07-16-advisory-remediation-backlog.md",
    "docs/reviews/2026-07-19-o006-phase1-azure-openai-provider-decision-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "Large",
      "milestone": "Live gateway architecture",
      "risk": "Exact identity, Azure deployment, D-061, disclosure, threat-model, implementation, and operational evidence remains incomplete.",
      "severity": "High",
      "summary": "ARB-002 remains unresolved after the Phase 1 AI-provider decision."
    }
  ],
  "increment_id": "o006-phase1-azure-openai-provider-decision",
  "manual_verification": [
    {
      "check": "The project owner approved the Azure OpenAI provider decision and exact documentation scope.",
      "required": true,
      "status": "Passed"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {"command": "npm run docs:check", "required": true, "status": "Passed"},
    {"command": "npm run repository:check", "required": true, "status": "Passed"},
    {"command": "npm run security:scan", "required": true, "status": "Passed"},
    {"command": "git diff --check", "required": true, "status": "Passed"},
    {"command": "test \"$(git status --porcelain | wc -l | tr -d ' ')\" = \"17\"", "required": true, "status": "Passed"},
    {"command": "git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.toml Cargo.lock .github .codex .agents scripts", "required": true, "status": "Passed"},
    {"command": "git diff --exit-code -- preserved O-006/O-007 and D-062 plans, increment records, and review reports", "required": true, "status": "Passed"},
    {"command": "rg -n \"D-063|Azure OpenAI|managed identity|ContentLogging=false|synthetic|automatic fallback|ARB-002\" authoritative live documents", "required": true, "status": "Passed"},
    {"command": "python3 .codex/hooks/session_end_gate.py", "required": true, "status": "Passed"}
  ]
}
-->

Date: 2026-07-19
Increment: O-006 Phase 1 Azure OpenAI provider decision
Branch: `main`

## Executive summary

D-063 selects Azure OpenAI in Microsoft Foundry as the sole Phase 1
synthetic-evaluation candidate while preserving all implementation and
live-traffic blocks. Result: `PASS WITH ADVISORIES`.

## Scope and boundaries

The exact scope is 17 documentation paths. No product source, test, dependency,
lockfile, workflow, hook, skill, Tauri, IPC, SQLite, permission, capability,
CSP, credential, cloud resource, provider adapter, or runtime behavior changed.

## Verification results

Passed: documentation formatting and links, repository policy, secret scan,
whitespace, exact scope, protected-path review, historical-evidence
preservation, provider-state consistency, complete diff, and session-end
inspection.

Failed and corrected: the first documentation check found only Prettier
formatting in `SECURITY_CHECKLIST.md`; final checks passed after formatting.

Not run: frontend tests, Rust tests, builds, native launch, Azure, identity,
managed identity, RBAC, provider, network, and ZDR operational checks because
no executable or infrastructure path changed. Manual verification pending:
none.

## Architecture findings

The provider choice remains behind the Cortexa gateway and does not enter the
desktop protocol. Direct OpenAI remains a future adapter, not a fork of the
desktop architecture.

## Security findings

Managed identity and least-privilege RBAC avoid a provider API key. D-061,
`ContentLogging=false`, stateless Responses evidence, disclosure, and exact
deployment review remain mandatory before real content. Automatic fallback is
prohibited.

## Code-health findings

No product code changed.

## Technical debt

No new implementation debt. ARB-002 remains the tracked High blocker.

## Roadmap findings

No product or remediation increment becomes Ready from this decision alone.

## Completion decision

`PASS WITH ADVISORIES`. Documentation acceptance criteria pass; ARB-002 remains
High because deployment, evidence, and implementation are intentionally absent.

## Next-increment readiness

`Blocked` pending exact identity and Azure resource evidence, D-061 evidence,
disclosure, threat model, and a separately approved implementation plan.

## Exact files changed

The machine manifest records exactly 17 documentation paths.

## Exact commands executed

The machine manifest records all inspection, formatting, verification,
session-end, finalization, and status commands and the corrected formatting
failure.
