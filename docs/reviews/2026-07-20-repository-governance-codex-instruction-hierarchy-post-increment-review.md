# Repository governance - Codex instruction hierarchy post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "./node_modules/.bin/prettier --write AGENTS.md docs/governance/MASTER_PROMPT.md ROADMAP.md",
    "git diff --check",
    "test \"$(find prompts -type f -name '*.md' | wc -l | tr -d ' ')\" = \"24\"",
    "test \"$(rg -l 'Follow the root AGENTS.md and docs/governance/MASTER_PROMPT.md' prompts | wc -l | tr -d ' ')\" = \"23\"",
    "test \"$(wc -l < AGENTS.md | tr -d ' ')\" -le 160",
    "test -z \"$(git diff --name-only --diff-filter=D -- prompts)\"",
    "git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.toml Cargo.lock .github .codex .agents scripts",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 .codex/hooks/post_increment_gate.py status"
  ],
  "files_changed": [
    "AGENTS.md",
    "ASSISTANT_USAGE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "ENGINEERING_GUIDE.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "docs/governance/MASTER_PROMPT.md",
    "docs/increments/repository-governance-codex-instruction-hierarchy.md",
    "docs/plans/README.md",
    "docs/plans/repository-governance-codex-instruction-hierarchy.md",
    "docs/reviews/2026-07-20-repository-governance-codex-instruction-hierarchy-post-increment-review.md",
    "prompts/README.md",
    "prompts/increments/bug-fix.md",
    "prompts/increments/feature-implementation.md",
    "prompts/increments/refactor.md",
    "prompts/increments/remediation-by-severity.md",
    "prompts/increments/remediation-single-advisory.md",
    "prompts/increments/verified-increment.md",
    "prompts/reviews/architecture-review.md",
    "prompts/reviews/code-review.md",
    "prompts/reviews/executive-review.md",
    "prompts/reviews/quality-gate.md",
    "prompts/reviews/readiness-review.md",
    "prompts/reviews/release-review.md",
    "prompts/reviews/security-review.md",
    "prompts/reviews/technical-debt.md",
    "prompts/templates/increment-template.md",
    "prompts/templates/remediation-template.md",
    "prompts/templates/review-template.md",
    "prompts/workflows/documentation.md",
    "prompts/workflows/end-session.md",
    "prompts/workflows/release.md",
    "prompts/workflows/remediation.md",
    "prompts/workflows/repository-health.md",
    "prompts/workflows/start-session.md"
  ],
  "findings": [],
  "increment_id": "repository-governance-codex-instruction-hierarchy",
  "manual_verification": [
    {
      "check": "No manual verification applies to this documentation-only repository-governance increment",
      "required": false,
      "status": "Passed"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
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
      "command": "test \"$(find prompts -type f -name '*.md' | wc -l | tr -d ' ')\" = \"24\"",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "test \"$(rg -l 'Follow the root AGENTS.md and docs/governance/MASTER_PROMPT.md' prompts | wc -l | tr -d ' ')\" = \"23\"",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "test \"$(wc -l < AGENTS.md | tr -d ' ')\" -le 160",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "test -z \"$(git diff --name-only --diff-filter=D -- prompts)\"",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.toml Cargo.lock .github .codex .agents scripts",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/post_increment_gate.py status",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-07-20
Increment: Repository governance - Codex instruction hierarchy
Branch: `main` (clean baseline `36ce9ab`; no publication requested)

## Executive summary

Created a durable instruction hierarchy: concise root `AGENTS.md`, detailed
`docs/governance/MASTER_PROMPT.md`, aligned prompt/template entry points, and
D-065. The approved product roadmap and all runtime boundaries remain unchanged.
The result is **PASS WITH ADVISORIES** because no later product increment is
Ready; ARB-002 remains separately blocked by owner decisions and evidence.

## Scope and boundaries

The exact thirty-nine-path inventory stayed within the approved documentation
and repository-governance scope. No application source, test, dependency,
lockfile, workflow, hook, skill, configuration, Tauri, IPC, SQLite, deployment,
credential, or runtime behavior changed. No prompt was deleted, renamed, or
relocated. Multi-agent execution and model-switching automation remain out of
scope and unimplemented.

## Verification results

All required documentation-tier checks passed after the final relevant edits:
Markdown formatting and local links, repository policy and path validation,
secret scanning, whitespace validation, prompt-tree count, shared-preamble
count, concise-root line limit, no prompt deletion, protected-path absence, and
the repository session-end inventory. The initial `npm run docs:check` reported
formatting-only issues in `AGENTS.md`, `docs/governance/MASTER_PROMPT.md`, and
`ROADMAP.md`; the repository Prettier binary corrected only those paths, and
the final documentation check passed. No manual check applies.

## Architecture findings

No architecture finding. The hierarchy explicitly retains the local-first
trust boundary and does not represent future identity, gateway, provider,
execution, persistence, or enterprise capabilities as implemented.

## Security findings

No security finding. The review found no changes to CSP, capabilities,
permissions, credentials, network behavior, logging, hooks, SQLite, or
operating-system access. The new text preserves the prohibition on secret
storage and live external processing without separately approved work.

## Code-health findings

No code-health finding. No code changed. Shared global instruction boilerplate
is centralized in the master prompt while all existing prompt-specific scope,
validation, and stop conditions remain in place.

## Technical debt

None introduced. The deliberate advisory model preference is documentation
only; it does not detect, select, or switch models.

## Roadmap findings

`NEXT_STEPS.md` remains blocked for product work: D-064 Stage B, Stage C, Stage
D, and ARB-002 runtime work require their own approvals and evidence. D-065 did
not reorder, authorize, or replace the approved Phase 4 roadmap.

## Completion decision

**PASS WITH ADVISORIES**. Every required check passed; the advisory is the
pre-existing absence of a Ready later product increment, not a defect in this
documentation-only change.

## Next-increment readiness

**Blocked.** Obtain project-owner direction before selecting a separately
bounded future task. Do not infer authorization for ARB-002 Stage B, Stage C,
Stage D, runtime implementation, or another remediation from this closeout.

## Exact files changed

The machine manifest lists all thirty-nine changed paths and is validated
against the complete Git change set.

## Exact commands executed

The machine manifest records each required verification command verbatim. The
initial `npm run docs:check` exited nonzero for the three formatting-only files
listed above; `./node_modules/.bin/prettier --write AGENTS.md
docs/governance/MASTER_PROMPT.md ROADMAP.md` corrected them. The final required
commands all passed with exit status zero.
