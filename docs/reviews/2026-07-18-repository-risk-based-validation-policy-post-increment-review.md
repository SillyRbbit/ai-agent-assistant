# Repository-wide risk-based validation policy post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "git diff --check",
    "npm run docs:check (initial run: TESTING_GUIDE.md formatting failed)",
    "npx prettier --write TESTING_GUIDE.md",
    "npm run docs:check",
    "npm run repository:check",
    "git diff --exit-code -- src src-tauri package.json package-lock.json .github .codex .agents scripts",
    "git stash list --format=%gd %H %s",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 .codex/hooks/post_increment_gate.py begin --increment repository-risk-based-validation (sandboxed attempt: state write denied)",
    "python3 .codex/hooks/post_increment_gate.py begin --increment repository-risk-based-validation (approved elevated retry: passed)",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment repository-risk-based-validation --report docs/reviews/2026-07-18-repository-risk-based-validation-post-increment-review.md (draft manifest rejected unsupported finding category)",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment repository-risk-based-validation --report docs/reviews/2026-07-18-repository-risk-based-validation-post-increment-review.md (draft manifest rejected annotated verification command)",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment repository-risk-based-validation --report docs/reviews/2026-07-18-repository-risk-based-validation-post-increment-review.md (sandboxed attempt: state write denied)",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment repository-risk-based-validation --report docs/reviews/2026-07-18-repository-risk-based-validation-post-increment-review.md (approved elevated retry: passed)",
    "python3 .codex/hooks/post_increment_gate.py begin --increment repository-risk-based-validation (rejected: existing valid marker preserved)",
    "python3 .codex/hooks/post_increment_gate.py begin --increment repository-risk-based-validation-policy (passed)",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment repository-risk-based-validation-policy --report docs/reviews/2026-07-18-repository-risk-based-validation-post-increment-review.md (draft filename rejected)",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment repository-risk-based-validation-policy --report docs/reviews/2026-07-18-repository-risk-based-validation-policy-post-increment-review.md (sandboxed attempt: state write denied)",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment repository-risk-based-validation-policy --report docs/reviews/2026-07-18-repository-risk-based-validation-policy-post-increment-review.md (approved elevated retry: passed)",
    "for file in prompts/increments/*.md prompts/templates/increment-template.md prompts/templates/remediation-template.md prompts/workflows/remediation.md prompts/workflows/repository-health.md; do grep -Fq 'Risk-Based Validation Policy in AGENTS.md and ENGINEERING_GUIDE.md' \"$file\" || exit 1; done",
    "for file in prompts/increments/*.md; do grep -Fq 'complete required completion-gate verification for the selected tier once after the final relevant edit' \"$file\" || exit 1; done",
    "! rg -n 'Run complete required verification|run complete verification|complete repository verification before acceptance' prompts/increments prompts/templates prompts/workflows prompts/README.md",
    "complete architecture, security, code-health, technical-debt, roadmap-readiness, and diff review"
  ],
  "files_changed": [
    "AGENTS.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "ENGINEERING_GUIDE.md",
    "HANDOFF.md",
    "TESTING_GUIDE.md",
    "docs/reviews/2026-07-18-repository-risk-based-validation-policy-post-increment-review.md",
    "prompts/README.md",
    "prompts/increments/bug-fix.md",
    "prompts/increments/feature-implementation.md",
    "prompts/increments/refactor.md",
    "prompts/increments/remediation-by-severity.md",
    "prompts/increments/remediation-single-advisory.md",
    "prompts/increments/verified-increment.md",
    "prompts/templates/increment-template.md",
    "prompts/templates/remediation-template.md",
    "prompts/workflows/remediation.md",
    "prompts/workflows/repository-health.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Roadmap",
      "effort": "Small",
      "milestone": "Next separately approved documentation-governance reconciliation",
      "risk": "ROADMAP.md still describes ARB-022 as commit-pending and retains a pre-publication queue, which can mislead readers even though authoritative live project memory is current.",
      "severity": "Advisory",
      "summary": "ROADMAP.md retains stale ARB-022 publication wording outside this approved policy scope."
    }
  ],
  "increment_id": "repository-risk-based-validation-policy",
  "manual_verification": [],
  "next_increment_readiness": "Ready with advisories",
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
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --exit-code -- src src-tauri package.json package-lock.json .github .codex .agents scripts",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "for file in prompts/increments/*.md prompts/templates/increment-template.md prompts/templates/remediation-template.md prompts/workflows/remediation.md prompts/workflows/repository-health.md; do grep -Fq 'Risk-Based Validation Policy in AGENTS.md and ENGINEERING_GUIDE.md' \"$file\" || exit 1; done",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "for file in prompts/increments/*.md; do grep -Fq 'complete required completion-gate verification for the selected tier once after the final relevant edit' \"$file\" || exit 1; done",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "! rg -n 'Run complete required verification|run complete verification|complete repository verification before acceptance' prompts/increments prompts/templates prompts/workflows prompts/README.md",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-07-18
Increment: Repository-wide risk-based validation policy
Branch: `main`

## Executive summary

The repository now uses risk-based validation: focused checks during
implementation, one stable completion gate selected by change class, and the
complete suite for cross-cutting or explicitly stricter work. Documentation-
only changes no longer require unrelated frontend tests, Rust tests, or
application builds. D-056 records the durable policy, and the compact assistant
rules, engineering guide, testing matrix, all six reusable increment prompts,
both increment-authoring templates, prompt index, coordinating remediation and
repository-health workflows, changelog, and handoff agree.

The result is `PASS WITH ADVISORIES`. All required documentation and repository
checks pass. One pre-existing stale-roadmap advisory remains outside scope.

## Scope and boundaries

The policy scope is six core documentation and repository-governance files plus
11 reusable prompt, template, index, and coordinating-workflow files. This
consolidated mandatory report is the eighteenth path. No product source,
dependency, manifest, lockfile, GitHub workflow, hook, skill, script, generated
artifact, Tauri boundary, capability, permission, CSP, SQLite schema, runner
selector, or runner configuration changed.

`stash@{0}` at `b57fe0f7b361f59f57e4ef501e1e642c802cbd48`
remains intact and unapplied. Its useful policy concept was adopted manually;
its stale Meta Increment 8 active-state wording was not restored.

## Verification results

Passed:

- `git status --short --branch` confirmed synchronized `main` before edits and
  the exact documentation-only change set afterward.
- `git diff --check` passed.
- Final `npm run docs:check` passed Markdown formatting and internal-link/path
  validation.
- `npm run repository:check` reported `repository-health: PASS (all)`.
- Prompt coverage assertions confirmed every reusable increment prompt and both
  increment-authoring templates reference the policy, and all six increment
  prompts preserve one complete required completion gate after the final edit.
- The stale-wording scan found no remaining prompt instruction to run blanket
  complete verification without the policy and selected tier.
- The protected-path diff proved no product, dependency, workflow, hook, skill,
  or script change.
- `python3 .codex/hooks/session_end_gate.py` reported no conflicts or staged
  paths and only the declared documentation paths.

Failed checks: the first `npm run docs:check` found only repository formatting
in `TESTING_GUIDE.md`. `npx prettier --write TESTING_GUIDE.md` corrected it, and
the final affected check passed. The first finalization attempt rejected the
draft report's unsupported `Documentation` finding category before writing gate
state; changing that manifest-only category to supported `Roadmap` corrected
the schema. Neither issue was a product or final repository-check failure.

Checks not run: frontend tests, Rust tests, and application builds. They are not
applicable because no executable source, tested example, generated artifact,
dependency, workflow, hook, or configuration changed.

Manual verification pending: none.

## Architecture findings

None. The policy changes no product module, data flow, platform boundary,
dependency, packaging behavior, or current-versus-future architecture claim.
It explicitly requires complete verification for cross-boundary changes.

## Security findings

None blocking. The policy does not weaken security-sensitive verification:
security, IPC, storage, policy, approval, dependency, Tauri configuration, and
release changes still require `npm run verify` plus applicable manual evidence.
No credential, permission, networking, workflow, hook, or trust-boundary surface
changed.

## Code-health findings

None. `ENGINEERING_GUIDE.md` owns the detailed strategy, `AGENTS.md` retains a
compact mandatory subset, and `TESTING_GUIDE.md` provides aligned commands and
the change-to-test matrix. Reusable increment prompts reference those
authorities instead of duplicating an unconditional full-suite rule. D-056
prevents the earlier draft stash from becoming an accidental source of
authority.

## Technical debt

One pre-existing Advisory remains: `ROADMAP.md` still describes ARB-022 as
commit-pending and retains an older queue. Effort is Small, it blocks neither
completion nor the next task, and it should be corrected only in a separately
approved documentation-governance reconciliation.

## Roadmap findings

The validation policy does not reorder product or remediation work. PR #23 and
the separate macOS runner-routing proposal remain outside scope. Current queue
authority remains `NEXT_STEPS.md`; the stale `ROADMAP.md` text is advisory.

## Completion decision

`PASS WITH ADVISORIES`. Every required documentation, prompt-coverage, and
repository check passed on the final content, no manual check is pending, no
Critical or High finding exists, and the 18-path change set is
documentation-only.

## Next-increment readiness

`Ready with advisories`. Review and publish only this 18-path policy change
after separate project-owner approval. Do not apply the stash, modify PR #23,
route workflows to a different runner, or begin product work in the same step.

## Exact files changed

The machine manifest lists the six accepted core policy and project-memory
files, 11 reusable prompt-library files, and this mandatory report. No other
path changed.

## Exact commands executed

The machine manifest records the required verification and gate commands. The
initial formatting failure, report-schema corrections, final passing checks,
and sandbox-only gate-state write retries are recorded explicitly.
