# D-107 account and Keychain scope contract decision post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": ["git switch -c codex/d107-account-keychain-scope-contract-decision", "python3 .codex/hooks/post_increment_gate.py begin --increment d107-account-keychain-scope-contract-decision", "npm run docs:check", "npm run repository:check", "npm run security:scan", "git diff --check", "git diff --exit-code b5fe79a424ecc3450054cc80eb89003294942326 -- docs/plans/2026-09-02-d107-exact-signer-binding-contract-decision.md docs/increments/d107-exact-signer-binding-contract-decision.md docs/reviews/2026-09-02-d107-exact-signer-binding-contract-decision-post-increment-review.md", "git diff --exit-code -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json", "python3 .codex/hooks/session_end_gate.py", "python3 .codex/hooks/post_increment_gate.py status", "npm run verify", "npm audit --audit-level=low", "npm run tauri -- build --no-bundle"],
  "files_changed": ["ARCHITECTURE.md", "CHANGELOG.md", "DECISIONS.md", "HANDOFF.md", "NEXT_STEPS.md", "PLANS.md", "PROJECT_STATUS.md", "ROADMAP.md", "SECURITY.md", "SECURITY_CHECKLIST.md", "TESTING_GUIDE.md", "TROUBLESHOOTING_LOG.md", "docs/increments/d107-account-keychain-scope-contract-decision.md", "docs/plans/2026-09-02-d107-account-keychain-scope-contract-decision.md", "docs/reviews/2026-09-02-d107-account-keychain-scope-contract-decision-post-increment-review.md"],
  "findings": [{"blocks_completion": false, "blocks_next_increment": true, "category": "Security", "effort": "Large", "milestone": "Before any operational successor", "risk": "No application-owned account/Keychain scope and nine other contracts remain unproved.", "severity": "Advisory", "summary": "D-111 is a negative scope decision; operational work remains blocked."}],
  "increment_id": "d107-account-keychain-scope-contract-decision",
  "manual_verification": [{"check":"D-111 selects only the closed negative scope disposition and preserves D-097 through D-110","required":true,"status":"Passed"},{"check":"Exact fifteen-path documentation inventory contains no product or automation path","required":true,"status":"Passed"},{"check":"Keychain, account, certificate, signing, Apple/Xcode, target-Mac, provider, product, and external operations","required":false,"status":"Not run"}],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [{"command":"npm run docs:check","required":true,"status":"Passed"},{"command":"npm run repository:check","required":true,"status":"Passed"},{"command":"npm run security:scan","required":true,"status":"Passed"},{"command":"git diff --check","required":true,"status":"Passed"},{"command":"git diff --exit-code b5fe79a424ecc3450054cc80eb89003294942326 -- docs/plans/2026-09-02-d107-exact-signer-binding-contract-decision.md docs/increments/d107-exact-signer-binding-contract-decision.md docs/reviews/2026-09-02-d107-exact-signer-binding-contract-decision-post-increment-review.md","required":true,"status":"Passed"},{"command":"git diff --exit-code -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json","required":true,"status":"Passed"},{"command":"python3 .codex/hooks/session_end_gate.py","required":true,"status":"Passed"},{"command":"python3 .codex/hooks/post_increment_gate.py status","required":true,"status":"Passed"},{"command":"npm run verify","required":false,"status":"Not run"},{"command":"npm audit --audit-level=low","required":false,"status":"Not run"},{"command":"npm run tauri -- build --no-bundle","required":false,"status":"Not run"}]
}
-->

Date: 2026-09-02
Increment: `d107-account-keychain-scope-contract-decision`
Branch: `codex/d107-account-keychain-scope-contract-decision`
Baseline: `b5fe79a424ecc3450054cc80eb89003294942326`

## Executive summary

D-111 selects `scope_contract_not_accepted`: ambient file-based
default/search-list behavior and unproved access-group scope cannot establish
one application-owned identity scope. D-097 through D-110 remain unchanged; all
ten blockers remain and no successor is Ready. **PASS WITH ADVISORIES**.

## Scope and boundaries

The exact fifteen documentation paths changed. No source, dependency,
configuration, Keychain/account/directory, certificate/private-key/signing,
build, target-Mac, provider, product, or external operation occurred.

## Verification results

Required documentation, repository, secret, whitespace, protected-history/path,
and session checks Passed. Product verification, audit, builds, and all system/
external checks were Not run by scope. No required check failed or remained
pending.

## Architecture findings

No finding. D-111 creates no runtime edge, scope selector, or authority.

## Security findings

No completion-blocking finding. D-111 rejects ambient-scope laundering and
introduces no Keychain, account, credential, filesystem, network, permission,
IPC, or secret surface.

## Code-health findings

No finding. The documentation keeps governance disposition separate from
implemented behavior and D-100 evidence.

## Technical debt

| Category          | Severity | Risk                                                                                       | Effort | Milestone                        | Blocks completion | Blocks next increment |
| ----------------- | -------- | ------------------------------------------------------------------------------------------ | ------ | -------------------------------- | ----------------- | --------------------- |
| Security boundary | Advisory | Missing owned scope and nine other contracts could be misrepresented as trusted authority. | Large  | Before any operational successor | No                | Yes                   |

## Roadmap findings

**Blocked.** D-111 is a negative result; no operational successor is Ready.

## Completion decision

**PASS WITH ADVISORIES.**

## Next-increment readiness

**Blocked.** No operational successor is Ready.

## Exact files changed

The machine manifest is the complete fifteen-path inventory.

## Exact commands executed

The machine manifest records all commands and actual statuses.
