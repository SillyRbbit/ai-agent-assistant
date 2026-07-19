# Remediation plan - High-severity advisory disposition

Status: Verified complete with advisories in the current workspace; publication
pending
Date: 2026-07-19
Owner: Project maintainer
Gate ID: `remediation-high-severity-advisory-disposition`
Baseline: clean synchronized `main` at `6ce9fce`

## Goal

Classify every canonical High-severity advisory against current source and test
evidence, record secure owner direction, and identify exact future blocking
triggers without implementing missing product capabilities or representing
deferred findings as resolved.

## Current evidence

- ARB-001 is published and resolved through Increment 4V at `6e6f91d`.
- No HTTP client, provider SDK, gateway origin, authorization header, production
  credential, Keychain adapter, or live model traffic exists.
- No dispatcher, executor, operating-system action path, runtime coordinator,
  or production end-to-end workflow exists.
- Approval audit remains typed, bounded, and process-local. SQLite persists
  bootstrap metadata only; no durable product repository exists.
- No license grant, signed or notarized release, Intel support evidence, SSO,
  enrollment, fleet policy, or enterprise control plane exists or is claimed.

## Approved dispositions

| Finding | Disposition                   | Blocking trigger                                                     |
| ------- | ----------------------------- | -------------------------------------------------------------------- |
| ARB-001 | `RESOLVED`                    | Revisit only if terminal approval or audit ownership changes         |
| ARB-002 | `DECISION REQUIRED`           | Before any live gateway or provider traffic                          |
| ARB-003 | `BLOCKED - FUTURE CAPABILITY` | Before dispatch or any operating-system side effect                  |
| ARB-004 | `BLOCKED - FUTURE CAPABILITY` | Before a production end-to-end or controlled-pilot workflow          |
| ARB-005 | `BLOCKED - FUTURE CAPABILITY` | Before persisting approval evidence or user/product content          |
| ARB-006 | `DEFERRED - NON-BLOCKING`     | Before public distribution, open-source publication, or contribution |
| ARB-007 | `DEFERRED - NON-BLOCKING`     | Before release-candidate or trusted public distribution work         |
| ARB-008 | `BLOCKED - FUTURE CAPABILITY` | Before enterprise onboarding or managed-device deployment            |
| ARB-044 | `SUPERSEDED`                  | Revisit only if treated independently from its canonical split       |

No `REMEDIATE NOW` or `ACCEPTED TEMPORARY RISK` disposition applies. High
severity is preserved for every unresolved High finding.

## Decision boundaries

- O-006 remains open for the gateway operator, hosting platform, identity
  provider, token issuer, fixed gateway origin, credential owner, operational
  owner, and incident-response owner.
- O-007 remains open for provider retention, data classification, disclosure,
  consent, log fields, log retention, deletion, privacy, and security ownership.
- O-008 records the unresolved repository and distribution license. Until it
  closes, the temporary posture is proprietary and all rights reserved.
- O-003 remains open beyond the provisional macOS 14+ Apple Silicon tested
  baseline. Intel and older macOS support remain unclaimed.
- O-009 records unresolved signing, notarization, credential ownership, and
  release authority. These block trusted public distribution, not unsigned
  local development.

No vendor, identity provider, token issuer, legal license, credential owner,
release authority, or signing owner is selected by this plan.

## Exact documentation scope

Modified:

```text
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
ROADMAP.md
docs/reviews/2026-07-16-advisory-remediation-backlog.md
```

Created:

```text
docs/plans/remediation-high-severity-advisory-disposition.md
docs/increments/remediation-high-severity-advisory-disposition.md
docs/reviews/2026-07-19-remediation-high-severity-advisory-disposition-post-increment-review.md
```

## Non-goals

- No application source, tests, dependencies, manifests, lockfiles, workflows,
  hooks, skills, Tauri configuration, capabilities, permissions, CSP, IPC,
  SQLite schema, migration, identifier, or product behavior change.
- No live model networking, gateway deployment, provider integration,
  credentials, Keychain adapter, or external user-content processing.
- No executor, complete Workflow, durable product data, enterprise controls,
  SSO, enrollment, fleet policy, signing, notarization, or release automation.
- No open-source license or other legal grant.
- No Intel or older-macOS compatibility claim.
- No rewrite of dated source evidence or historical completion results. The
  dated backlog receives only an explicitly dated additive disposition update.
- No commit, push, merge, or later increment.

## Risks and controls

- **False resolution:** every unresolved High finding retains its severity and
  an exact trigger; only ARB-001 is resolved by implementation evidence.
- **Hidden risk acceptance:** no finding uses `ACCEPTED TEMPORARY RISK`; secure
  defaults are provisional boundaries, not permission to activate capability.
- **Premature architecture selection:** open decisions list required owners and
  acceptance criteria without selecting vendors or authorities.
- **Scope drift:** a protected-path diff must prove no product, dependency,
  workflow, hook, or skill path changed.
- **Historical drift:** dated report baselines and original verification remain
  intact; current-state updates are additive and explicitly dated.

## Verification

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.toml Cargo.lock src-tauri/Cargo.toml src-tauri/Cargo.lock .github .codex .agents
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

Frontend tests, Rust tests, application builds, native launch, and product
manual verification are not required because the approved change is
documentation-only and changes no executable or product path.

## Rollback

Before publication, restore the nine modified documentation files and remove
the three new records. After publication, revert only the bounded documentation
commit. No product, dependency, database, migration, credential, configuration,
permission, or release rollback applies.

## Exit criteria

- All nine canonical High records have one permitted disposition.
- O-006 through O-009 and O-003 retain exact unresolved owners and triggers.
- No severity is reduced and no unresolved finding is called resolved.
- No `REMEDIATE NOW` finding is omitted.
- The exact 12-path scope and documentation-tier checks pass.
- Architecture, security, code-health, technical-debt, and readiness review find
  no blocking issue introduced by the disposition.
- The consolidated report is `PASS` or `PASS WITH ADVISORIES`, and the marker
  is complete and valid.
