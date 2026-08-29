# V0-3 prerequisite — Xcode-managed Developer ID recovery

Status: Blocked; documentation-only recovery plan is complete, but every Apple,
signing, Keychain, and target-Mac action needs separate owner approval
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: D-072, D-075, D-076, TS-017, and a separate owner-operated
execution increment

## Goal and user-visible outcome

Define the smallest Apple-supported recovery path to obtain one stable,
owner-controlled **Developer ID Application** identity for the private,
fake-only V0-3 signed-client proof. The path uses Xcode's documented Developer
ID certificate workflow rather than the failed manual Certificate Assistant CSR
flow. It does not publish Cortexa to the App Store, distribute it, notarize it,
or create a usable assistant.

If a later owner-operated execution succeeds, the only observable outcome is a
private yes/no determination that the target Mac has one usable identity for
the fixed `com.aiagentassistant.desktop` application. That is evidence for a
later V0-3 readiness review, not credential, gateway, provider, or release
authority.

## Current-state evidence

- `main` was clean and synchronized at
  `9a48b25ca0ccc7362eda0e0a496625ee3788a119` before this planning increment.
- V0-2 is published at `1513bd8`; its source is transport-free and has no
  signed identity, credential, Keychain consumer, network, Tauri, or WebView
  path.
- D-072 selects a stable owner-controlled signed macOS identity, D-075 selects
  Developer ID Application specifically for outside-App-Store use, and D-076
  defers execution after TS-017's unresolved Certificate Assistant failure.
- Apple documents that an Account Holder can create a Developer ID Application
  certificate through Xcode or the developer account. A Developer ID
  Application identity is for independently distributed Mac apps, not Mac App
  Store submission. See [Signing Mac Software with Developer
  ID](https://developer.apple.com/developer-id/) and [Developer ID
  certificates](https://developer.apple.com/help/account/certificates/create-developer-id-certificates/).
- The current app bundle identifier is `com.aiagentassistant.desktop`. Current
  configuration contains no Keychain-access-group entitlement or provisioning
  profile. Those are not authorized by this plan.

## Exact future operation boundary

The later owner-operated execution increment may perform only these steps, on
the target Mac and only after fresh explicit approval:

1. Confirm, privately, that the owner is the Account Holder for the existing
   Apple Developer Program membership. Do not record account, Team ID, email,
   certificate, key, or membership identifiers in chat or the repository.
2. Use Xcode's supported Developer ID Application certificate creation path.
   Do not retry the failed Certificate Assistant CSR path, use OpenSSL,
   generate a filesystem private key, import a key, use a cloud-managed
   certificate, or choose Apple Development, Developer ID Installer, or Mac
   App Store distribution identities.
3. Prove privately that the private key was generated and remains
   owner-controlled and non-exported on the target Mac. No export, backup,
   copy, terminal argument, screenshot, log, source, or repository evidence
   may contain signing material.
4. Sign one locally built app with the fixed bundle identifier and inspect its
   designated requirement privately. Do not add a custom requirement,
   entitlement, provisioning profile, hardened-runtime setting, Tauri setting,
   or build script under this path.
5. Report only closed sanitized categories: `identity available | unavailable`,
   `private key non-exported: confirmed | not confirmed`, `signed build:
passed | failed`, `unsigned/other copy: not tested | rejected | accepted`,
   and `state changed: observed | not observed`.

The later V0-3 increment owns fake Keychain-item setup and its exact
signed-versus-unauthorized-app proof. This recovery plan does not create
Keychain items or permit fake or real credential reads.

## Exact files for this documentation-only planning increment

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-08-28-personal-assistant-v0-xcode-developer-id-recovery.md`
- `docs/increments/personal-assistant-v0-xcode-developer-id-recovery-planning.md`
- `docs/reviews/2026-08-28-personal-assistant-v0-xcode-developer-id-recovery-planning-post-increment-review.md`

No Rust, TypeScript, Tauri configuration, entitlement, provisioning profile,
capability, CSP, dependency, lockfile, workflow, hook, script, test, Apple
account, certificate, Keychain item, Cloudflare resource, provider setting,
network request, or filesystem signing material may change.

## Security and privacy invariants

- App Store publication is out of scope. Developer ID is retained only as the
  selected stable identity for a private Mac application.
- An ad-hoc signature is not an acceptable substitute because it has no signing
  identity. A self-signed or app-specific-ACL alternative is not selected and
  would require a superseding decision and a new plan.
- The model, WebView, Tauri, logs, SQLite, shell environment, files, tests,
  CI, and documentation receive neither signing material nor credential bytes.
- Missing, ambiguous, expired, unsigned, wrong-identifier, prompt-dependent,
  or non-export proof outcomes deny and keep V0-3 Blocked.
- Revocation, renewal, update, reinstall, compromise, and removal are future
  owner-operated procedures. No plan may claim that removing an identity
  cryptographically erases APFS/SSD remnants.

## Threats, manual gates, and stop conditions

| Threat or uncertainty                                   | Required control                                                                                                                             |
| ------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| A personal-use app is mistaken for an App Store release | State that no App Store submission, notarization, or distribution is part of this path.                                                      |
| A different app gains access to a future Keychain item  | Preserve a stable signed-identity requirement; V0-3 must later prove an unsigned or unauthorized copy fails.                                 |
| Manual CSR failure is bypassed with an unsafe key       | Use only the documented Xcode Developer ID route; stop on key export, filesystem key generation, or an unapproved identity class.            |
| Signing material or private evidence leaks              | Owner-only execution; sanitized categorical evidence only; no identifiers, output, screenshots, transcripts, or artifacts enter chat or Git. |
| A config change silently broadens privilege             | Stop if an entitlement, provisioning profile, Tauri/configuration change, dependency, script, or source edit is needed.                      |
| Identity lifecycle is not stable                        | A later manual gate must cover renewal, revocation, update/reinstall, compromise response, and removal before V0-3 starts.                   |

## Verification for this planning increment

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri package.json package-lock.json \
  src-tauri/Cargo.toml src-tauri/Cargo.lock .github .codex/hooks scripts
python3 .codex/hooks/session_end_gate.py
```

Apple Developer, Xcode, Keychain, certificate, signing, target-Mac identity,
notarization, distribution, Cloudflare, provider, and network checks are **Not
run** for this documentation-only increment.

## Rollback

Before a future execution, abandon the plan with no external cleanup. After a
future execution, a separate owner-approved incident or recovery plan must own
any certificate revocation, fake-item removal, or local identity removal. This
planning increment itself rolls back only by reverting the listed documentation
paths.

## Acceptance criteria

- [x] The plan distinguishes private use from App Store publication.
- [x] The plan retains D-072/D-075's Developer ID Application identity rather
      than weakening V0-3 to unsigned or ad-hoc behavior.
- [x] The plan defines a no-manual-CSR Xcode path with explicit non-export and
      sanitized-evidence boundaries.
- [x] Every external action remains separately owner-approval-bound.
- [x] No current V0-3, credential, Keychain, Apple, provider, or product
      capability is claimed or created.

## Progress and final results

- 2026-08-28: Owner approved this documentation-only recovery-planning
  increment after confirming no valid Developer ID Application identity is
  installed on the target Mac.
- 2026-08-28: The plan was authored from D-072/D-075/D-076, TS-017, current
  source configuration, and official Apple Developer documentation.
- Final documentation-tier verification and post-increment review passed with
  the explicit advisory that D-076/TS-017 continue to block V0-3. The planning
  record is complete; every future Apple or target-Mac action remains separately
  owner-approval-bound.
