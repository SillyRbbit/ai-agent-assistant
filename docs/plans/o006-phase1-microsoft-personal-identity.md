# O-006 Phase 1 Microsoft personal identity decision

Status: Verified complete with advisories; awaiting publication review
Owner: Project owner
Date: 2026-07-19
Gate ID: `o006-phase1-microsoft-personal-identity-decision`
Baseline: clean synchronized `main` at `c90c77f`

## Goal

Record the approved Phase 1 identity choice and its closed security boundaries
without implementing or configuring authentication.

## Decision

- Microsoft personal identity is the sole Phase 1 provider.
- Phase 1 is personal-account-only through the `/consumers` authority.
- The planned flow uses the system browser, Authorization Code Flow, PKCE S256,
  one-time `state`, and OIDC `nonce`.
- Separate public desktop client and gateway API resource registrations are
  required.
- Initial scopes are `openid`, `email`, and one delegated Cortexa gateway
  scope.
- `offline_access`, persistent sessions, profile and Microsoft Graph scopes,
  workforce tenants, and automatic email linking are excluded.
- The account key is provider ID plus normalized issuer plus subject.
- Google is deferred until demonstrated demand after Microsoft verification.
- Apple is deferred until Mac App Store planning or demonstrated demand.

## Explicit non-goals

- No source, test, dependency, lockfile, workflow, hook, skill, script, Tauri,
  capability, permission, CSP, IPC, SQLite, migration, or runtime change.
- No Microsoft registration, tenant configuration, client ID, resource ID,
  redirect registration, scope creation, credential, secret, or token.
- No OAuth/OIDC client, system-browser launch, loopback listener, PKCE, token
  validation, Keychain, session, account, linking, gateway, or networking code.
- No Google or Apple integration and no Phase 2 workforce identity.
- No Azure resource, DNS, TLS, `AgentProvider`, AI-provider, ZDR, disclosure UI,
  cloud, enterprise, ARB-002, licensing, signing, or notarization work.
- No rewrite of D-060, D-061, the published O-006/O-007 plan or increment, or
  their completion reports.
- No commit, push, merge, publication, or later increment.

## Exact files

Modified:

```text
AGENTS.md
ARCHITECTURE.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PRODUCT_REQUIREMENTS.md
PROJECT_STATUS.md
ROADMAP.md
SECURITY.md
SECURITY_CHECKLIST.md
docs/plans/README.md
docs/reviews/2026-07-16-advisory-remediation-backlog.md
```

Created:

```text
docs/plans/o006-phase1-microsoft-personal-identity.md
docs/increments/o006-phase1-microsoft-personal-identity-decision.md
docs/reviews/2026-07-19-o006-phase1-microsoft-personal-identity-decision-post-increment-review.md
```

## Risks

- Treating identity selection as implementation or deployment authorization.
- Inventing registration values before approved configuration evidence exists.
- Enabling persistent sessions through `offline_access` without a decision.
- Using email as an identity key or automatic linking signal.
- Weakening provider neutrality or implying Google or Apple support.
- Missing Apple's future Mac App Store review trigger.
- Falsely resolving O-006 or ARB-002 while AI-provider and D-061 gates remain.
- Recording tenant, client, subscription, credential, or secret values.

## Verification

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.toml Cargo.lock .github .codex .agents scripts
rg -n "Microsoft personal|consumers|PKCE|offline_access|account linking|O-006|D-062|ARB-002" AGENTS.md ARCHITECTURE.md PRODUCT_REQUIREMENTS.md SECURITY.md SECURITY_CHECKLIST.md DECISIONS.md HANDOFF.md PROJECT_STATUS.md NEXT_STEPS.md PLANS.md ROADMAP.md
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

Frontend tests, Rust tests, application builds, native authentication,
Microsoft registration, token, Keychain, gateway, and network checks are not
required for this documentation-only tier and must be recorded as not run.

## Acceptance criteria

- [x] D-062 records the exact approved provider, tenant, flow, scopes, identity
      key, linking prohibition, and provider deferrals.
- [x] Current absence and future-only implementation remain explicit.
- [x] `offline_access` remains excluded pending a separate decision.
- [x] O-006 remains open for exact evidence and AI-provider configuration.
- [x] D-061 and ARB-002 remain unchanged in severity and blocking effect.
- [x] D-060, D-061, prior plans, increments, and reports remain unchanged.
- [x] The exact 17-path documentation scope passes all required checks.
- [x] The final report is `PASS` or `PASS WITH ADVISORIES`, and the marker is
      complete and valid.

## Rollback

Before publication, restore only the exact 17 documentation paths. After
publication, revert only the bounded documentation commit. A later owner-policy
change must append a superseding decision instead of deleting D-062. No runtime,
credential, registration, cloud, network, dependency, or data rollback applies.
