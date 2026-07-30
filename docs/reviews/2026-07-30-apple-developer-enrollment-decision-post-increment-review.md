# Apple Developer enrollment decision post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "python3 .codex/hooks/post_increment_gate.py status",
    "python3 .codex/hooks/session_end_gate.py",
    "npx prettier --write DECISIONS.md HANDOFF.md PROJECT_STATUS.md NEXT_STEPS.md PLANS.md CHANGELOG.md",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock"
  ],
  "files_changed": [
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "docs/reviews/2026-07-30-apple-developer-enrollment-decision-post-increment-review.md"
  ],
  "findings": [],
  "increment_id": "apple-developer-enrollment-decision",
  "manual_verification": [
    {
      "check": "Review D-074 for ownership, seller-name visibility, future team access, certificate control, cost, migration risk, owner-only fake-demo scope, official-source links, and an explicit no-enrollment recommendation",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Confirm no Apple account, enrollment, purchase, agreement, signing asset, Keychain, credential, Cloudflare, provider, traffic, deployment, or runtime action was performed",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Confirm D-064 production 15-minute and D-068 demo-only 30-day token requirements remain unchanged",
      "required": true,
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
      "command": "git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

- Date: 2026-07-30
- Increment: apple-developer-enrollment-decision
- Branch: main

## Executive summary

D-074 records a documentation-only Apple Developer enrollment recommendation.
Cortexa should defer enrollment now. If a future separately approved owner-only
fake-demo proof needs membership while ownership remains personal, individual
membership is conditionally preferred. Organization enrollment must be
re-evaluated before company-owned contracts, seller identity, shared certificate
control, or team continuity becomes necessary.

The decision compares ownership, seller-name visibility, future team access,
certificate control, cost, migration risk, and the owner-only fake-demo scope
against current official Apple guidance. No enrollment, purchase, agreement,
account access, signing asset, Keychain item, credential, Cloudflare, provider,
traffic, deployment, source, dependency, or runtime behavior changed. Quality
result: `PASS WITH ADVISORIES` because all later implementation remains Blocked.

## Scope and boundaries

The approved scope was one documentation-only decision record and its current
state synchronization. It excludes all external account and operational actions.
D-074 changes no trust boundary, entitlement, permission, platform adapter,
secret, identity, credential, network, cloud, gateway, provider, or execution
path.

The complete seven-path change set contains only the decision, project-memory
records, changelog, and this review. No product source, manifest, lockfile,
Tauri configuration, signing, Keychain, credential, or infrastructure file
changed.

## Verification results

- `python3 .codex/hooks/session_end_gate.py`: Passed; no conflicts or
  unexpected paths.
- `npm run docs:check`: Passed.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- Product-source, manifest, lockfile, and Tauri unchanged assertion: Passed.
- D-074 comparison, official-source links, conditional recommendation,
  non-authority, and D-064/D-068 preservation review: Passed.

No required automated or manual check is failed, not run, or pending.
Application tests, Rust tests, builds, native launch, Apple account review, and
all provider checks are not applicable because this increment changes only
documentation.

## Architecture findings

No finding. The decision respects the existing D-072 signed-identity boundary,
does not promote a free personal development team into a stable production
control, and makes future company-ownership review an explicit prerequisite
rather than assuming an automatic migration.

## Security findings

No finding. The decision creates no account access or mutation, signing asset,
certificate, private key, Keychain item, credential, permission, provider
request, network path, or runtime authority. It preserves fail-closed future
approval and target-Mac evidence requirements.

## Code-health findings

No finding. No production or test code changed. The decision is additive,
internally consistent with D-070 through D-073, and links to current official
Apple guidance without storing account-specific material.

## Technical debt

None introduced. The unproven signing identity and unperformed private account
review remain explicit evidence prerequisites, not debt created by the decision.

## Roadmap findings

`Blocked`. No product or remediation increment is Ready. D-074 recommends no
enrollment now. A later owner decision may authorize an enrollment increment,
but individual versus organization must be re-evaluated against ownership at
that time; enrollment still cannot authorize signing, Keychain, credential, or
fake-only implementation work.

## Completion decision

`PASS WITH ADVISORIES`

## Next-increment readiness

`Blocked`. The exact next task is an owner-directed private read-only account
review under the published evidence plan, or a separately approved enrollment
decision when the owner actually wishes to make that contractual purchase. Do
not begin a gate, edit code, or access an Apple account unless separately
directed.

## Exact files changed

```text
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/reviews/2026-07-30-apple-developer-enrollment-decision-post-increment-review.md
```

## Exact commands executed

- `python3 .codex/hooks/post_increment_gate.py status`: expected active
  increment confirmed.
- `python3 .codex/hooks/session_end_gate.py`: Passed; no conflicts.
- `npx prettier --write DECISIONS.md HANDOFF.md PROJECT_STATUS.md NEXT_STEPS.md PLANS.md CHANGELOG.md`:
  Passed; mechanical Markdown formatting only.
- `npm run docs:check`: Passed.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- `git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock`:
  Passed; product source, dependency, and Tauri configuration paths are
  unchanged.
