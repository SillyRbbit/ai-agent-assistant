# D-107 opaque prebound identity contract decision post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git switch -c codex/d107-opaque-prebound-identity-contract-decision",
    "python3 .codex/hooks/post_increment_gate.py begin --increment d107-opaque-prebound-identity-contract-decision",
    "npx prettier --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md ROADMAP.md SECURITY.md SECURITY_CHECKLIST.md TESTING_GUIDE.md TROUBLESHOOTING_LOG.md docs/increments/d107-opaque-prebound-identity-contract-decision.md docs/plans/2026-09-02-d107-opaque-prebound-identity-contract-decision.md",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "git diff --exit-code 51a80f60cf8d803b3c22945d0719a671382ee090 -- docs/plans/2026-09-02-personal-assistant-v0-key-use-containment-classification.md docs/increments/personal-assistant-v0-key-use-containment-classification.md docs/reviews/2026-09-02-personal-assistant-v0-key-use-containment-classification-post-increment-review.md",
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
    "docs/increments/d107-opaque-prebound-identity-contract-decision.md",
    "docs/plans/2026-09-02-d107-opaque-prebound-identity-contract-decision.md",
    "docs/reviews/2026-09-02-d107-opaque-prebound-identity-contract-decision-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "Large",
      "milestone": "Before any identity, Keychain, signing, P3, P4, V0-3, or operational successor work",
      "risk": "No repository-owned issuer establishes the required opaque signing-identity reference. The remaining signer, scope, export, algorithm, interaction, cancellation, late-result, cleanup, and platform-effect contracts are also unproved; treating object opacity or fixed labels as provenance could grant ambient authority.",
      "severity": "Advisory",
      "summary": "D-109 closes one governance question negatively while all ten D-107 blockers still prevent operational work."
    }
  ],
  "increment_id": "d107-opaque-prebound-identity-contract-decision",
  "manual_verification": [
    {
      "check": "D-109 covers only opaque prebound identity-reference issuance and selects one closed negative governance disposition",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "D-097 failure, D-107 factual 8/11 record, and D-108 additive 9/10 interpretation remain unchanged",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Complete changed-file inventory equals the exact fifteen-path documentation allowlist and contains no protected product or repository-automation path",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Privacy review found no target-derived or private credential, identity, account, certificate, key, signature, host, path, provider, or personal value",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Independent architecture, security, code-health, technical-debt, and readiness reviews completed without a completion-blocking finding",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Apple, Xcode, Keychain, certificate, private-key, signing, product build, target-Mac, provider, product, and state-changing external-system operations",
      "required": false,
      "status": "Not run"
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
      "command": "git diff --exit-code 51a80f60cf8d803b3c22945d0719a671382ee090 -- docs/plans/2026-09-02-personal-assistant-v0-key-use-containment-classification.md docs/increments/personal-assistant-v0-key-use-containment-classification.md docs/reviews/2026-09-02-personal-assistant-v0-key-use-containment-classification-post-increment-review.md",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --exit-code -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json",
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
    },
    {
      "command": "npm run verify",
      "required": false,
      "status": "Not run"
    },
    {
      "command": "npm audit --audit-level=low",
      "required": false,
      "status": "Not run"
    },
    {
      "command": "npm run tauri -- build --no-bundle",
      "required": false,
      "status": "Not run"
    }
  ]
}
-->

Date: 2026-09-02
Increment: `d107-opaque-prebound-identity-contract-decision`
Branch: `codex/d107-opaque-prebound-identity-contract-decision`
Baseline: `51a80f60cf8d803b3c22945d0719a671382ee090`

## Executive summary

The exact fifteen-path documentation-only increment is complete. D-109 selects
`reference_issuance_not_accepted` for only
`opaque_prebound_identity_contract`: current repository source contains no
application-owned issuer for a no-input, attempt-bound opaque signing-identity
reference without lookup, enumeration, selection, fallback, or ambient/default
authority. Opacity is not provenance. D-107 remains 8 documented / 11
unproved, D-108 remains prospectively 9/10, and no candidate or successor is
Ready. Quality gate: **PASS WITH ADVISORIES**.

## Scope and boundaries

The diff changes twelve current governance documents and adds the plan,
increment record, and this review. It creates no product module, runtime
interface, identity source, Tauri command/event, credential, signing operation,
or target-Mac effect. No product/test source, dependency, lockfile,
configuration, capability, CSP, permission, entitlement, IPC, workflow, hook,
skill, script, or toolchain path changed.

Documentation files were written and local formatting, validation, review, and
gate processes ran. No Apple, Xcode, Keychain, certificate, private-key,
signing, build, provider, product, target-Mac, or state-changing external
operation ran.

## Verification results

| Check                                        | Status  | Evidence                                                                                              |
| -------------------------------------------- | ------- | ----------------------------------------------------------------------------------------------------- |
| `npm run docs:check`                         | Passed  | Markdown formatting and repository link-health passed.                                                |
| `npm run repository:check`                   | Passed  | Complete repository-health policy passed.                                                             |
| `npm run security:scan`                      | Passed  | Secret scan passed without reporting a matched value.                                                 |
| `git diff --check`                           | Passed  | No whitespace errors.                                                                                 |
| Historical-evidence diff                     | Passed  | Protected D-107 plan, increment, and report have no diff.                                             |
| Protected-path diff                          | Passed  | No product, dependency, configuration, workflow, hook, skill, or script path changed.                 |
| Session inventory                            | Passed  | No conflicts; the change set matches the fifteen-path allowlist.                                      |
| Independent final reviews                    | Passed  | Architecture, security, code-health, debt, and readiness reviews have no completion-blocking finding. |
| `npm run verify`                             | Not run | Documentation-only scope changed no executable boundary.                                              |
| `npm audit --audit-level=low`                | Not run | No dependency or lockfile changed.                                                                    |
| `npm run tauri -- build --no-bundle`         | Not run | Product build is outside scope.                                                                       |
| Apple/Keychain/signing/target-Mac operations | Not run | Explicitly prohibited by this increment.                                                              |

No required automated or manual check remains Failed, Not run, or Pending.

## Architecture findings

No finding. The decision distinguishes planned opaque-reference issuance from
current behavior and creates no architecture edge or duplicated authority. D-101
remains the owner of account/home/path prohibition; D-108 remains limited to
D-102's build-child subject.

## Security findings

No completion-blocking finding. The review preserves the trust boundary:
neither model, WebView, caller, environment, fixed label, certificate filter,
nor default Keychain behavior gains identity-selection authority. No secrets,
credentials, logs, network, filesystem, capability, IPC, permission, or
dependency surface changed. The unresolved identity and platform boundaries are
recorded as blockers, not treated as mitigated.

## Code-health findings

No finding. The documentation uses the existing D-100/D-101/D-107/D-108
vocabulary, keeps governance dispositions separate from factual evidence, and
does not claim unimplemented code or tests.

## Technical debt

One existing deferred-security item is recorded.

| Category          | Severity | Risk                                                                                                                                                     | Effort | Milestone                                                                           | Blocks completion | Blocks next increment |
| ----------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ----------------------------------------------------------------------------------- | ----------------- | --------------------- |
| Security boundary | Advisory | No repository-owned identity-reference issuer and nine other contracts remain unproved; ambient authority could be misrepresented as trusted provenance. | Large  | Before any identity, Keychain, signing, P3, P4, V0-3, or operational successor work | No                | Yes                   |

## Roadmap findings

**Blocked.** D-109 resolves only the first remaining opaque-reference question
negatively. The remaining D-107 contracts and D-097's preserved terminal
record prevent any operational next increment. The smallest next action is a
separately owner-approved documentation-only plan for one remaining contract;
it must not claim or operate an identity source.

## Completion decision

**PASS WITH ADVISORIES.** Required documentation and protected-path checks
passed. The advisory is the preserved, next-blocking security posture; it does
not prevent this documentation increment from completing.

## Next-increment readiness

**Blocked.** No candidate or operational successor is Ready. A separate owner
approval is required even for another bounded documentation decision.

## Exact files changed

1. `ARCHITECTURE.md`
2. `CHANGELOG.md`
3. `DECISIONS.md`
4. `HANDOFF.md`
5. `NEXT_STEPS.md`
6. `PLANS.md`
7. `PROJECT_STATUS.md`
8. `ROADMAP.md`
9. `SECURITY.md`
10. `SECURITY_CHECKLIST.md`
11. `TESTING_GUIDE.md`
12. `TROUBLESHOOTING_LOG.md`
13. `docs/increments/d107-opaque-prebound-identity-contract-decision.md`
14. `docs/plans/2026-09-02-d107-opaque-prebound-identity-contract-decision.md`
15. `docs/reviews/2026-09-02-d107-opaque-prebound-identity-contract-decision-post-increment-review.md`

## Exact commands executed

All commands in the machine manifest were recorded from this increment.
Documentation validation, protected-history/path checks, session inventory,
and gate status passed. Product verification, audit, build, and all
Apple/Keychain/signing/target-Mac/provider/external operations were Not run by
scope.
