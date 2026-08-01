# macOS Developer ID Application identity-creation plan

- Status: Documentation-only plan; certificate creation remains unauthorized
- Date: 2026-07-31
- Decision authority: D-064, D-068, D-070, D-072, D-073, D-074, and D-075

## Goal

Define the future owner-operated creation and private target-Mac evidence of
the single Developer ID Application certificate selected by D-075 for D-072's
stable signed macOS application identity proof. This plan does not authorize
Apple access, a certificate signing request (CSR), certificate, key, download,
installation, signing, notarization, Keychain action, or product behavior.

## Exact future certificate model

The later operational increment may create exactly one Developer ID Application
certificate, owned by the individual Apple Developer Program Account Holder, to
sign a future fake-only macOS proof application. It must not create a Developer
ID Installer certificate, Apple Development certificate, Apple Distribution
certificate, Mac App Store certificate, provisioning profile, App ID,
entitlement, installer, or notarization request.

The later increment must bind the signer to one approved future proof application
identifier and one target Mac. That identifier, Apple team information,
certificate subject, hash, serial number, Keychain path, and private evidence
location stay outside the repository, chat, terminal transcript, logs, and
ordinary notes.

## Future owner gates and procedure

The following procedure is future-facing and must not run under this plan.

1. Confirm D-075 remains appropriate: Cortexa is still personally owned,
   Developer ID Application still matches the non-App-Store proof scope, and no
   organization-ownership or seller-identity trigger requires a new decision.
2. Obtain separate project-owner approval for one operational increment that
   names only the Developer ID Application certificate and explicitly excludes
   every other signing asset, entitlement, profile, distribution, notarization,
   Keychain credential, Cloudflare, provider, traffic, deployment, and runtime
   action.
3. The owner alone privately accesses Apple Developer with the owner-controlled
   Account Holder account. No credential, account detail, browser artifact,
   certificate material, or signing identifier may be copied into repository
   material or chat.
4. The owner creates the CSR and its private key only on the approved target
   Mac. The private key must remain non-exported, owner-controlled, and outside
   source control, screenshots, shell arguments, logs, archives, cloud sync,
   shared storage, and backup/export workflows. Stop if the creation path cannot
   preserve those limits.
5. The owner creates and downloads only the approved certificate type, installs
   it only into the approved target-Mac signing store, and stops. Do not sign an
   artifact, create a profile or entitlement, request notarization, create an
   App ID, or access a credential.
6. Privately record sanitized outcomes only: `not started`, `created`,
   `unavailable`, `pending`, `declined`, `compromised`, or `not determined`,
   plus `private key non-exported: confirmed` and `other signing asset created:
not observed`. Do not record identifiers, files, screenshots, or account
   information.
7. A later separately approved fake-only proof increment may define the exact
   artifact, signing command, verification command, and cleanup. Creation alone
   does not authorize any of them.

## Private target-Mac evidence for a later proof

A later proof increment must collect privately and report only sanitized results
for all of the following:

- the intended target-Mac proof artifact is signed by the owner-controlled
  Developer ID Application identity;
- signer continuity survives the approved rebuild/restart scenario;
- an unsigned or non-authorized copy is rejected for the Keychain boundary;
- no certificate subject, hash, serial, team value, requirement, private key,
  fake value, or native error is added to repository evidence; and
- the future proof uses D-073's three-file fake-only boundary and contains no
  Cloudflare, provider, traffic, deployment, IPC, WebView, or runtime wiring.

Certificate creation is not target-Mac proof evidence. Notarization, Gatekeeper
distribution behavior, installer behavior, release signing, and public
distribution are excluded and require their own decisions and plans.

## Lifecycle, incident response, and rollback

- Before future certificate creation: abandon the proposed action; no external
  or local signing state should exist.
- After future creation: do not export, duplicate, share, or silently delete
  the private key or certificate. Do not claim a reversible rollback.
- If compromise, loss of owner control, unexpected additional asset creation,
  or ownership change is suspected: stop use immediately, preserve no sensitive
  evidence in the repository, and require a separate owner-approved incident,
  revocation, and continuity plan before any remediation or replacement.
- Renewal, expiration, revocation, replacement, target-Mac migration, and any
  change from individual to organization ownership each require a separate
  approved lifecycle or migration increment.

## Risks and security controls

| Risk                             | Required control                                                                                                                                                       |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Private-key compromise or export | Owner-only, target-Mac-only generation; non-export policy; no repository, chat, terminal, screenshot, cloud-sync, shared-storage, or backup/export path.               |
| Over-broad certificate use       | One Developer ID Application certificate, one future fake-only proof identifier, and no installer, App Store, entitlement, profile, notarization, or distribution use. |
| False identity assurance         | Require a later signed-artifact and unauthorized-copy private proof; certificate presence is insufficient.                                                             |
| Lifecycle failure                | Separate renewal, revocation, compromise, migration, and ownership-change plans; no assumed rollback.                                                                  |
| Boundary conflation              | Membership and certificate planning authorize neither Keychain credentials nor Cloudflare, provider, traffic, deployment, runtime, or product code.                    |

## Verification for this documentation increment

- Confirm D-075 chooses only Developer ID Application and distinguishes it from
  installer, App Store, and development certificate types.
- Confirm the plan requires a separate owner approval before all future Apple,
  CSR, certificate, key, download, installation, or signing action.
- Confirm private-key, target-Mac evidence, lifecycle, compromise response,
  stop conditions, and all excluded boundaries are explicit.
- Run `npm run docs:check`, `npm run repository:check`, `npm run security:scan`,
  `git diff --check`, and `python3 .codex/hooks/session_end_gate.py`.

## Non-goals and readiness

No Apple access, certificate, CSR, key, profile, App ID, entitlement, download,
signing, notarization, Keychain action, credential, Cloudflare change, provider
request, deployment, traffic, code, dependency, or runtime behavior.

Blocked. This plan selects no signing asset and does not make a signing,
fake-only proof, credential, Cloudflare, or product increment Ready. D-064's
15-minute production requirement and D-068's 30-day owner-only demo exception
remain unchanged.
