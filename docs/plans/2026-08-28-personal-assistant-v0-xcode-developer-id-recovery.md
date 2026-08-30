# V0-3 prerequisite — Xcode-managed Developer ID recovery

Status: Blocked; the scoped identity check fully Passed and D-096 accepts the
prospective evidence standard, but the historical non-export criterion cannot
Pass as written and the signed build remains Not run
Owner: Henry Dang
Last updated: 2026-08-29
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

## Execution reconciliation

A separately approved execution later used this plan on clean synchronized
baseline `0931df66c389bdc13c705d1259706c4d3770761c`. Xcode created and listed
one Developer ID Application certificate record. Sanitized CLI checks found no
matching local certificate and no usable code-signing identity for the current
macOS user. On 2026-08-29, the owner categorically confirmed that Keychain
Access shows the certificate with a private key beneath it. Local pairing is
therefore observed. Later that day, after accepting the exact disclosed OS and
process-metadata boundaries, the owner approved one run of the sanitized
`keychain_identity_v1` wrapper. It returned
`passed_one_label_matched_valid_codesigning_identity`, establishing current
scoped CLI visibility of one matching valid identity. Non-exported owner
control and signing remain unconfirmed. The owner subsequently reported no
authorization prompt and no visible state change. This is still a blocked
recovery result, not proof that D-076 or TS-017's historical cause is resolved
and not authority to retry, replace, import, export, revoke, remove, or sign
with an identity.

The owner-supplied screenshots used during execution contained private Apple
account and certificate metadata. They are not repository artifacts, but their
use did not satisfy this plan's intended no-screenshot chat boundary. No more
identifier-bearing Apple, certificate, or Keychain screenshots may be used;
future evidence must remain categorical and owner-attested.

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

## Historical operation boundary superseded prospectively by D-096

The following numbered steps preserve the boundary approved before execution;
they are historical evidence, not current operational instructions. In
particular, step 3 and its `private key non-exported` result cannot be proved as
written. D-096 and the
[Developer ID present-use and local signing proof](2026-08-29-v0-developer-id-present-use-local-signing-proof.md)
are the only prospective evidence contract, and that plan remains operationally
Blocked.

The later owner-operated execution increment was permitted to perform only
these steps on the target Mac and only after fresh explicit approval:

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

## Historical verification for the documentation-only planning increment

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri package.json package-lock.json \
  src-tauri/Cargo.toml src-tauri/Cargo.lock .github .codex/hooks scripts
python3 .codex/hooks/session_end_gate.py
```

At planning closeout, Apple Developer, Xcode, Keychain, certificate, signing,
target-Mac identity, notarization, distribution, Cloudflare, provider, and
network checks were **Not run**. That historical statement does not describe
the later execution reconciled above.

## Rollback

Before a future execution, abandon the plan with no external cleanup. After a
future execution, a separate owner-approved incident or recovery plan must own
any certificate revocation, fake-item removal, or local identity removal. This
planning increment itself rolls back only by reverting the listed documentation
paths.

## Historical planning acceptance criteria

- [x] The plan distinguishes private use from App Store publication.
- [x] The plan retains D-072/D-075's Developer ID Application identity rather
      than weakening V0-3 to unsigned or ad-hoc behavior.
- [x] The plan defines a no-manual-CSR Xcode path with explicit non-export and
      sanitized-evidence boundaries.
- [x] Every external action remains separately owner-approval-bound.
- [x] At planning closeout, no V0-3, credential, Keychain, Apple, provider, or
      product capability was claimed or created. The later separately approved
      execution created only the certificate record described above; the later
      read-only scoped check established current identity visibility but did
      not establish non-exportability, custody, or a signed build.

## Evidence-standard discrepancy identified after the scoped check

Security review on 2026-08-29 determined that this plan's phrase “remains
owner-controlled and non-exported” overstates what ordinary Xcode/login-
Keychain state can prove retrospectively. Current pairing, identity visibility,
and even one valid signature cannot establish historical absence of export,
exclusive custody, or absence of a prior copy. The separate current-item
extractability attribute was not queried and remains `not_proven`.
The historical text is preserved as the then-selected gate, but it is not a
currently satisfiable evidence claim.

The owner first authorized drafting and then approved the bounded
[Developer ID present-use and local signing proof](2026-08-29-v0-developer-id-present-use-local-signing-proof.md).
D-096 now additively governs prospective evidence using owner-attested no-known-
private-key-export history, workflow private-key export not performed, future
present-session signing use, and fixed `not_proven` categories for technical
non-extractability, historical absence of export, and exclusive custody. It
does not rewrite this plan's historical criterion or supply the later owner
attestation. This decision authorizes no Keychain, Apple, build, private-key,
signing, prompt, cleanup, or V0-3 operation. The recovery gate is terminally
`failed` / `FAIL` / `Blocked`, and the historical screenshot/privacy failure
remains Failed.

Post-gate review also found that the consumed wrapper resolved the account home
through `getpwuid`/`opendirectoryd`. The wrapper emitted no account field and no
remote directory traffic is proven, but the possible full account-record,
configured local or remote directory-service, and OS cache/socket/log boundary
was not separately disclosed before execution. Its additive disposition remains
Pending; the consumed query must not be rerun. The same review found that the
then-current gate tooling had no terminal failed or abandoned state. D-097 and
the owner-authorized same-active-gate implementation now provide a validated
`failed` record with no completion marker. The immutable privacy failure remains
Failed, readiness remains Blocked, and no successor or operational authority is
created.

## Progress and final results

- 2026-08-29: The owner authorized the same-active-gate D-097 recovery. The
  repository hook now has an exact terminal `failed` state and `close-failed`
  path that cannot issue a completion marker or promote failure to completion.
  This resolves only the Stop-loop/tooling discrepancy; the report remains
  `FAIL`/Blocked and every Apple, Keychain, signing, and successor action remains
  prohibited.
- 2026-08-29: The owner approved the bounded documentation-only evidence-
  standard plan. D-096 accepts its closed prospective categories without
  rewriting historical evidence or authorizing the future signing milestone.
- 2026-08-29: The owner confirmed the target Mac remained personally
  controlled, accepted the exact OS trust and local process-metadata residual
  boundaries, acknowledged the identity-metadata scope, and approved one exact
  `keychain_identity_v1` run. It returned
  `passed_one_label_matched_valid_codesigning_identity` with no retry or raw
  identifier output. The owner then reported no authorization prompt and no
  visible state change. Non-exported owner control and signing remain pending
  or Not run, so recovery and V0-3 remain Blocked.
- 2026-08-29: The owner categorically confirmed, without another screenshot or
  identifier, that Keychain Access shows the Developer ID Application
  certificate with a private key beneath it. This passes local-pairing presence
  only. The later scoped check supersedes the prior zero result for current
  visibility, but non-exported owner control and signing remain unproven, so
  the execution and V0-3 remain Blocked.
- 2026-08-28: After exact action-time owner confirmation, Xcode created and
  listed one Developer ID Application certificate with no visible error status.
  Sanitized user-keychain checks then found zero matching local certificates
  and zero usable code-signing identities. Certificate creation is therefore
  observed, but owner-controlled private-key usability remains unconfirmed and
  the execution stops closed without import, export, revocation, alternate
  identity creation, or signing.
- 2026-08-28: The owner supplied private App Store Connect evidence confirming
  Account Holder authority. The screenshot contains identifiers and is not
  copied into the repository. This supersedes only the earlier interpretation
  of Xcode's abbreviated Admin label; all certificate, key, and signing gates
  remain pending.
- 2026-08-28: Sanitized target-Mac inspection confirmed the existing Xcode
  account is available but reports the team role as Admin rather than Account
  Holder. No certificate-management action was opened and no certificate,
  private key, signing, or additional external-state change occurred. The
  execution stops pending private Account Holder access or a separately
  approved superseding path.
- 2026-08-28: The owner approved the separately bounded execution increment.
  Full Xcode 26.6 was installed and verified on the target Mac. The active
  developer directory remains Apple Command Line Tools; this increment will
  not change that system-wide setting.
- 2026-08-28: Owner approved this documentation-only recovery-planning
  increment after confirming no valid Developer ID Application identity is
  installed on the target Mac.
- 2026-08-28: The plan was authored from D-072/D-075/D-076, TS-017, current
  source configuration, and official Apple Developer documentation.
- Final documentation-tier verification and post-increment review passed with
  the explicit advisory that D-076/TS-017 continue to block V0-3. The planning
  record is complete; every future Apple or target-Mac action remains separately
  owner-approval-bound.
