# D-107 exact signer binding contract decision post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git switch -c codex/d107-exact-signer-binding-contract-decision",
    "python3 .codex/hooks/post_increment_gate.py begin --increment d107-exact-signer-binding-contract-decision",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "git diff --exit-code cc4f8b41638434977d4a038b8aa5d027f4964fb0 -- docs/plans/2026-09-02-d107-opaque-prebound-identity-contract-decision.md docs/increments/d107-opaque-prebound-identity-contract-decision.md docs/reviews/2026-09-02-d107-opaque-prebound-identity-contract-decision-post-increment-review.md",
    "git diff --exit-code -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 .codex/hooks/post_increment_gate.py status",
    "npm run verify",
    "npm audit --audit-level=low",
    "npm run tauri -- build --no-bundle"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/increments/d107-exact-signer-binding-contract-decision.md",
    "docs/plans/2026-09-02-d107-exact-signer-binding-contract-decision.md",
    "docs/reviews/2026-09-02-d107-exact-signer-binding-contract-decision-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "Large",
      "milestone": "Before identity, certificate, Keychain, signing, P3, P4, V0-3, or any operational successor work",
      "risk": "No immutable expected signer binding exists, and the remaining identity, scope, export, algorithm, interaction, cancellation, cleanup, and platform contracts remain unproved. Treating correspondence or metadata as expected identity could permit ambient substitution.",
      "severity": "Advisory",
      "summary": "D-110 closes the exact signer-binding question negatively while all ten D-107 blockers still prevent operational work."
    }
  ],
  "increment_id": "d107-exact-signer-binding-contract-decision",
  "manual_verification": [
    {"check":"D-110 covers only exact signer binding and selects one closed negative governance disposition","required":true,"status":"Passed"},
    {"check":"D-097, D-107 factual 8/11, D-108 additive 9/10, and D-109 remain unchanged","required":true,"status":"Passed"},
    {"check":"The complete changed-file inventory equals the exact fifteen-path documentation allowlist and contains no protected product or automation path","required":true,"status":"Passed"},
    {"check":"No target-derived or private certificate, identity, account, key, signature, fingerprint, host, path, provider, or personal value entered the diff","required":true,"status":"Passed"},
    {"check":"Apple, Xcode, Keychain, certificate, private-key, signing, product build, target-Mac, provider, product, and state-changing external-system operations","required":false,"status":"Not run"}
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {"command":"npm run docs:check","required":true,"status":"Passed"},
    {"command":"npm run repository:check","required":true,"status":"Passed"},
    {"command":"npm run security:scan","required":true,"status":"Passed"},
    {"command":"git diff --check","required":true,"status":"Passed"},
    {"command":"git diff --exit-code cc4f8b41638434977d4a038b8aa5d027f4964fb0 -- docs/plans/2026-09-02-d107-opaque-prebound-identity-contract-decision.md docs/increments/d107-opaque-prebound-identity-contract-decision.md docs/reviews/2026-09-02-d107-opaque-prebound-identity-contract-decision-post-increment-review.md","required":true,"status":"Passed"},
    {"command":"git diff --exit-code -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json","required":true,"status":"Passed"},
    {"command":"python3 .codex/hooks/session_end_gate.py","required":true,"status":"Passed"},
    {"command":"python3 .codex/hooks/post_increment_gate.py status","required":true,"status":"Passed"},
    {"command":"npm run verify","required":false,"status":"Not run"},
    {"command":"npm audit --audit-level=low","required":false,"status":"Not run"},
    {"command":"npm run tauri -- build --no-bundle","required":false,"status":"Not run"}
  ]
}
-->

Date: 2026-09-02
Increment: `d107-exact-signer-binding-contract-decision`
Branch: `codex/d107-exact-signer-binding-contract-decision`
Baseline: `cc4f8b41638434977d4a038b8aa5d027f4964fb0`

## Executive summary

The exact fifteen-path documentation-only increment is complete. D-110 selects
`signer_binding_not_accepted`: current repository source establishes at most
certificate/private-key correspondence, not an immutable expected Developer ID
Application signer/certificate/public-key binding. D-097, D-107 8/11, D-108
9/10, and D-109 remain unchanged. No candidate or successor is Ready. Quality
gate: **PASS WITH ADVISORIES**.

## Scope and boundaries

The change set contains only the approved governance documentation, increment
record, plan, and review. It adds no runtime interface, signer source,
certificate data, Keychain access, signing operation, dependency, configuration,
IPC, capability, workflow, hook, script, product build, or external action.

## Verification results

| Check                                                                            | Status  | Evidence                                                      |
| -------------------------------------------------------------------------------- | ------- | ------------------------------------------------------------- |
| Documentation, repository, and secret checks                                     | Passed  | `docs:check`, `repository:check`, and `security:scan` passed. |
| Whitespace, protected history, and protected paths                               | Passed  | No whitespace issue or protected source/automation diff.      |
| Session inventory                                                                | Passed  | No conflicts; exact fifteen-path inventory.                   |
| Architecture, security, code-health, debt, and readiness reviews                 | Passed  | No completion-blocking finding.                               |
| Product verification, audit, build, Apple/Keychain/signing/target-Mac operations | Not run | Explicitly outside the documentation-only scope.              |

No required check remains Failed, Not run, or Pending.

## Architecture findings

No finding. D-110 creates no architecture edge or authority and keeps signer
binding distinct from D-109 identity issuance and D-101 scope control.

## Security findings

No completion-blocking finding. The review rejects labels, fingerprints,
filters, pairing, default state, and caller input as expected-signer proof; no
secret or privilege surface changed.

## Code-health findings

No finding. Documentation uses closed governance terms and does not imply an
implemented interface or test.

## Technical debt

| Category          | Severity | Risk                                                                                                                                    | Effort | Milestone                        | Blocks completion | Blocks next increment |
| ----------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------- | ------ | -------------------------------- | ----------------- | --------------------- |
| Security boundary | Advisory | Immutable expected signer binding and nine other contracts remain unproved; substitution could be misrepresented as trusted provenance. | Large  | Before any operational successor | No                | Yes                   |

## Roadmap findings

**Blocked.** D-110 is a bounded negative result. No operational successor is
Ready; a separate owner-approved documentation plan is required for any other
remaining contract.

## Completion decision

**PASS WITH ADVISORIES.** The advisory preserves next-blocking security facts
without preventing completion of this documentation increment.

## Next-increment readiness

**Blocked.** No candidate or operational successor is Ready.

## Exact files changed

The fifteen paths in the machine manifest are the complete change set.

## Exact commands executed

The machine manifest records every command and its actual status. Product,
audit, build, and all system/external operations were Not run by scope.
