# V0 — Xcode Developer ID recovery execution

Status: Blocked — scoped identity check fully Passed; the historical non-export
criterion cannot Pass as written, D-096 accepts a truthful prospective evidence
standard, and signed-build evidence remains Not run
Date started: 2026-08-28
Last evidence update: 2026-08-29
Owner: Henry Dang
Branch: `codex/v0-xcode-developer-id-recovery-execution`
Baseline: `0931df66c389bdc13c705d1259706c4d3770761c`
Plan: [`2026-08-28-personal-assistant-v0-xcode-developer-id-recovery.md`](../plans/2026-08-28-personal-assistant-v0-xcode-developer-id-recovery.md)

## Evidence-standard reconciliation accepted

On 2026-08-29, security/readiness review determined that the approved
objective's “non-exported owner-controlled” phrase cannot be proven
retrospectively from ordinary Xcode/login-Keychain state or a successful
signature. The historical objective remains recorded, but it is not eligible
to be marked Passed as written. The owner authorized and approved the
[Developer ID present-use and local signing proof](../plans/2026-08-29-v0-developer-id-present-use-local-signing-proof.md),
and D-096 accepts bounded owner attestation, workflow-private-key-no-export,
present-use, and fixed `not_proven` categories for historical absence of export,
non-extractability, and exclusive custody.

The evidence-standard documentation reconciliation is complete. The exact
sanitizer and every build, Keychain, private-key, signing, prompt, and cleanup
operation remain Blocked. At that checkpoint, the then-active gate stayed
`FAIL`/active, its historical screenshot/privacy failure stayed Failed, and no
second gate had begun. D-097 subsequently records that same gate as terminally
`failed` / `FAIL` / `Blocked` without a completion marker.

## Approved objective

Privately establish whether the target Mac can obtain one owner-controlled,
non-exported Developer ID Application identity through Xcode, then use it only
to sign one locally built app with the fixed `com.aiagentassistant.desktop`
bundle identifier. Record only the closed sanitized outcome categories in the
final review.

## Scope

- Privately confirm Apple Developer Account Holder status without recording any
  account, team, certificate, or key identifier.
- Use Xcode's Developer ID Application certificate path only.
- Privately verify that the resulting private key is non-exported and
  owner-controlled.
- Build and sign one local app without changing source, dependencies, Tauri
  configuration, entitlements, provisioning profiles, scripts, or the active
  developer-directory setting.

## Explicit non-goals

No App Store submission, distribution, notarization, certificate/key export,
manual CSR retry, OpenSSL or filesystem key generation, cloud-managed
certificate, Keychain credential or fake-item creation, raw key or credential
value reading, V0-3 implementation, unsigned-copy test, provider, network
product feature, credentials in source, logs, or repository, or
system-configuration change. Sanitized signing-certificate and identity
metadata counts were allowed only to establish the closed availability result.

## Current-state evidence

- Clean synchronized baseline: `main == origin/main == 0931df6`.
- Full Xcode 26.6 is installed on the target Mac; Command Line Tools remain
  the active developer directory.
- The valid recovery-planning completion marker remains `PASS WITH ADVISORIES`.

## Interfaces and invariants

- Only the owner operates any Apple-account authentication or confirmation.
- The WebView, Tauri, source tree, logs, and repository receive no secret,
  certificate bytes, private-key material, identifier, credential value, or
  raw Keychain output. Private screenshots containing account and certificate
  metadata entered chat despite the intended evidence boundary; they were not
  copied into the repository, are not repeated here, and no further such
  screenshot is permitted.
- An unavailable, ambiguous, prompt-dependent, expired, wrong-identity, or
  non-export outcome fails closed and leaves V0-3 Blocked.
- This increment may prove only identity/key/one-signed-build evidence;
  V0-3 owns the fake-Keychain unauthorized-copy proof.

## Milestones

- [x] Reconfirm clean source-current baseline and installed Xcode.
- [x] Inspect the available account role using sanitized target-Mac evidence.
- [x] Confirm Account Holder authority from owner-supplied private evidence.
- [x] Create one Xcode-managed Developer ID Application certificate record.
- [x] Confirm categorically that Keychain Access shows the certificate with a
      private key beneath it.
- [x] Confirm that the default user Keychain exposes one currently valid
      code-signing identity with the fixed Developer ID Application label
      prefix through the separately approved sanitized scoped check.
- [x] Preserve the historical non-exported owner-controlled criterion as not
      eligible to Pass and accept an additive evidence-standard reconciliation
      before any further operational work.
- [ ] Build and sign one local app, then report sanitized results.

## Verification and stop conditions

Run documentation, repository, security, diff, and session-end gates at
closeout. Use only sanitized categories for manual evidence. Stop immediately
if Xcode requests an entitlement, provisioning profile, source/configuration or
dependency change, key export/import, filesystem key generation, a different
certificate class, App Store action, notarization, or a real credential read.

### Closed result table

| Required result                                                                         | Status                      | Sanitized evidence                                                                                                                        |
| --------------------------------------------------------------------------------------- | --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| Clean synchronized source baseline                                                      | Passed                      | `main == origin/main == 0931df66c389bdc13c705d1259706c4d3770761c` before execution                                                        |
| Account Holder authority                                                                | Passed                      | Owner privately attested with account evidence; identifiers are omitted                                                                   |
| Xcode-managed certificate record                                                        | Passed                      | Xcode listed one Developer ID Application record without a visible error                                                                  |
| Keychain certificate/private-key pairing                                                | Passed                      | Owner categorically confirmed the certificate has a private key beneath it; no identifier or screenshot was supplied                      |
| Scoped label-matched valid signing-identity visibility                                  | Passed                      | One exact `keychain_identity_v1` run returned `passed_one_label_matched_valid_codesigning_identity`; earlier broader checks returned zero |
| Private key non-exported and owner-controlled                                           | Manual verification pending | Local pairing is observed, but export history cannot be proven retrospectively; this row is not eligible to Pass as written               |
| One fixed-bundle local app signed                                                       | Not run                     | Signing remained outside the completed scoped check; D-096 changes no operational authority                                               |
| Unsigned or other-copy rejection                                                        | Not run                     | Explicitly owned by V0-3, not this recovery increment                                                                                     |
| External state changed                                                                  | Passed                      | One certificate record was created; no other external change was authorized                                                               |
| Original Xcode stage had no permission prompt/device effect beyond certificate creation | Passed                      | This observation predates and is separate from the later scoped-wrapper observations                                                      |
| Identifier-free chat evidence                                                           | Failed                      | Private screenshots crossed the planned chat boundary; repository evidence remains sanitized                                              |
| Target Mac remains personally controlled                                                | Passed                      | Owner supplied the exact categorical confirmation                                                                                         |
| Possible OS trust traffic/cache/log/state boundary accepted                             | Passed                      | Owner accepted the disclosed residual boundary for one run                                                                                |
| Local account-home/Keychain process-metadata visibility accepted                        | Passed                      | Owner accepted the disclosed residual boundary for one run                                                                                |
| `getpwuid`/`opendirectoryd` account-record and directory-service boundary accepted      | Manual verification pending | Discovered after execution; no remote directory traffic is proven, no account field was emitted, and no rerun is authorized               |
| Identity-metadata enumeration scope acknowledged                                        | Passed                      | Owner acknowledged the query excludes private-key bytes, password values, and provider credentials                                        |
| Exact revised plan and one execution approved                                           | Passed                      | Owner approved one execution; that approval is consumed                                                                                   |
| Exact scoped-check authorization prompt observation                                     | Passed                      | Owner reported `authorization_prompt=not_observed`                                                                                        |
| Exact scoped-check visible state-change observation                                     | Passed                      | Owner reported `state_changed=not_observed`                                                                                               |

### Automated verification evidence

| Command                                       | Status              | Result                                                                                                                                                                                                                                             |
| --------------------------------------------- | ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `npm run verify`                              | Passed              | 53 hook, 80 repository, 313 frontend, 302 Rust library, and 248 Rust integration tests; 247 integration tests passed and one opt-in Hermes probe was ignored; lint, typecheck, frontend production build, and Tauri release no-bundle build passed |
| Focused post-increment hook tests             | Passed              | 44 exact state-machine, readiness, drift, legacy-state, reclosure, successor-admission, and report-contract tests passed                                                                                                                           |
| `npm audit --audit-level=low`                 | Failed, then Passed | Sandboxed attempt failed on registry DNS/log access; authorized network retry found 0 vulnerabilities                                                                                                                                              |
| `npm run docs:check`                          | Failed, then Passed | Initial closeout formatting affected this record and review; the 2026-08-29 evidence update affected only this record; repository Prettier corrected only the named files and final reruns passed                                                  |
| Read-only hook report structural validation   | Passed              | The hook validator accepted report structure and reached exactly the expected `post-increment report contains blocking evidence` result; no gate state was written                                                                                 |
| Read-only terminal-failure validation         | Passed              | `validate_failed_report` accepted the exact computed `FAIL` report without writing state                                                                                                                                                           |
| `npm run repository:check`                    | Passed              | Repository policy checks passed after final closeout synchronization                                                                                                                                                                               |
| `npm run security:scan`                       | Passed              | Secret scan passed after final closeout synchronization                                                                                                                                                                                            |
| `git diff --check`                            | Passed              | No whitespace error after final closeout synchronization                                                                                                                                                                                           |
| Protected product-path `git diff --exit-code` | Passed              | No product source, dependency, configuration, workflow, or script change; the hook and focused hook test are the exact authorized D-097 diff                                                                                                       |
| `python3 .codex/hooks/session_end_gate.py`    | Passed              | No conflicts or staged paths after final closeout synchronization                                                                                                                                                                                  |

The documentation, repository, security, diff, protected-path, and session-end
checks were rerun after the final evidence and review documents were written.
The post-increment completion decision remains **FAIL** because private-key
custody/non-exportability, signed-build evidence, the historical evidence-
privacy requirement, and the later-discovered Open Directory disclosure are
not all Passed. The completion marker remains absent. D-097 records the exact
terminal state as valid `failed` / `FAIL` / `Blocked`; it does not make the
increment complete or a successor Ready.

## Rollback

Before certificate creation, abandon with no external cleanup. After an
external identity change, do not revoke, remove, or modify it without a new
owner-approved incident or recovery increment.

## Discovery reconciliation

Xcode abbreviated the available developer-team role as Admin. The owner then
supplied private App Store Connect evidence that the same account is also the
Account Holder. The screenshot contains private identifiers and is not copied
into the repository. The Account Holder prerequisite is satisfied; certificate
creation, private-key proof, and signing remain pending and separately gated.

After exact owner confirmation at action time, Xcode created and listed one
Developer ID Application certificate with no visible error status. Sanitized
user-keychain checks found zero matching local certificates and zero usable
code-signing identities. On 2026-08-29, the owner categorically confirmed that
Keychain Access shows the certificate with a private key beneath it. The
pairing is present, but the Keychain/CLI discrepancy prevents proof that the
identity is usable for code signing or that the key satisfies the required
non-exported owner-control evidence. The increment stops closed:
no operator-initiated import, export, post-discrepancy Keychain mutation,
revocation, alternate certificate, cloud-managed path, or signing attempt was
performed. At that checkpoint, Keychain location, certificate validity, CLI
usability, and non-exported owner control remained unconfirmed.

Later on 2026-08-29, the owner accepted the exact residual OS and local-process
metadata boundaries, confirmed the target Mac remained personally controlled,
acknowledged the identity-metadata scope, and approved one run of the exact
`keychain_identity_v1` wrapper. The single run returned
`passed_one_label_matched_valid_codesigning_identity`, so current scoped CLI
visibility is now established. The earlier zero result's cause, certificate
provenance, export history, exclusive owner custody, actual signing, and future
Keychain access remain unproven. The owner later reported no authorization
prompt and no visible state change, and no retry or signing action occurred.

The Apple Account used for macOS sign-in differs from the Apple Developer
account used in Xcode. Apple documents account/team selection in Xcode and a
local matching private key as the signing requirements; the account difference
alone is therefore not recorded as the cause. A wrong Xcode account/team or a
private key unavailable to the current macOS user's Keychain could matter, but
neither cause was established here.

## Closeout and stop state

Quality gate: **FAIL**. Next-increment readiness: **Blocked**. Do not create a
second certificate, import or export material, revoke or remove the observed
record, change accounts or Keychains, repeat the exact scoped check, or attempt
signing under this approval. Current scoped identity visibility is established,
but any non-exportability, custody, signing, or further diagnostic action
requires a separately bounded owner decision.

Post-gate review originally confirmed that `.codex/hooks/post_increment_gate.py`
supported only `active` or a passing `complete` state. Because the historical
privacy failure is immutable, this increment cannot receive a truthful passing
marker. D-097 and the owner-authorized same-active-gate recovery below now add a
terminal `failed` state without rewriting the failure or beginning a second
gate. It grants no successor or operational signing authority.

## Documentation-only discrepancy-plan amendment

On 2026-08-29, the owner authorized amending this then-active recovery
increment only to draft the exact
[sanitized Keychain-scoped identity discrepancy plan](../plans/2026-08-29-v0-keychain-scoped-identity-discrepancy.md).
The draft defines an internally resolved default-user-Keychain scope, one
bounded read-only identity query, strict raw-output suppression, closed results,
zero retries, manual prompt handling, and fail-closed stop conditions. Static
security review found no documented offline mode for the proposed
Security.framework policy query, so possible OS-managed certificate or
revocation traffic was an explicit unresolved execution boundary before the
later owner acceptance.

That planning authority did not authorize execution. The owner later provided
the plan's complete risk acknowledgement and separately approved one exact run.
The wrapper ran once, returned
`passed_one_label_matched_valid_codesigning_identity`, and was not retried. It
did not authorize opening Keychain Access or Xcode, accessing Apple services,
interacting with a prompt, changing account/Keychain/certificate/key state, or
signing. Non-exported owner control remains Manual verification pending, the
signed build remains Not run, and the quality gate remains **FAIL**/Blocked.

The later evidence-standard review supersedes only the interpretation of that
pending label: the selected pairing, identity-list, and signature evidence does
not establish historical absence of export or exclusive custody, and the
separate current-item extractability attribute remains unqueried/`not_proven`.
Those claims must not be converted to Passed. D-096 now accepts the closed
prospective categories in the present-use/local-signing plan; no owner
attestation, private-key use, or signing execution occurred.

At documentation-amendment validation, the embedded Python parsed successfully
without execution. Documentation, repository-health, secret-scan, protected-
path, whitespace, and session-end checks Passed. The wrapper and every
Keychain/Apple/Xcode/signing operation were Not run at that checkpoint; the
active post-increment gate remained active.
The required architecture, security, code-health, technical-debt, and readiness
reviews retained `FAIL`/Blocked and, at that point, identified three unresolved
disclosure gaps in the proposed check: possible OS cache/log/state effects,
local process metadata visibility of the Keychain path, and overbroad
no-credential-access wording. They were recorded rather than automatically
fixed, and execution was not eligible for approval until a separate
owner-authorized correction or explicit residual-risk decision.

The owner then approved documentation-only correction of those findings. The
revised plan now distinguishes wrapper-owned persistence from unproven OS
cache/log/state effects, discloses local process-metadata visibility, and
narrows the identity-metadata boundary. Follow-up static review also corrected
the blanket no-redirection wording to permit only the exact quoted
here-document supplying the reviewed Python body. The corrected draft then
passed static architecture/security re-review and all required documentation-
tier checks without execution. The later exact owner confirmation accepted the
remaining disclosed boundaries and authorized only the single successful
scoped run recorded above. The owner then reported no authorization prompt and
no visible state change.

## Same-active-gate terminal-failed recovery amendment

### Authority and goal

On 2026-08-29, the owner authorized implementation of the smallest truthful
terminal-failure disposition inside this existing active increment and
explicitly prohibited a new increment. The goal is to preserve the exact
`FAIL`/Blocked report, end the Stop-hook loop with valid evidence, and create no
completion marker or successor authority.

### Exact files and interfaces

Implementation is limited to the repository gate, its focused tests, its
review template and skill, applicable governance/security/testing guidance,
D-097, this active record and plan, the existing post-increment report, and
current project-memory closeout files. Product source, dependencies, lockfiles,
workflows, Tauri configuration, Apple/Keychain/signing state, and external
systems remain out of scope.

The only new executable interface is:

```text
post_increment_gate.py close-failed --increment <id> --report <path>
```

It accepts no arbitrary command, content, external path, credential, network
input, or product authority. Report parsing remains bounded and closed.

### Invariants

- Report schema stays v1; state schema v2 adds exact `failed` evidence while
  legacy v1 active and complete state remains readable.
- `close-failed` accepts only a report whose declared and computed result is
  `FAIL`; `finalize` continues to accept only passing results.
- Failed state contains exact quality, readiness, report digest, workspace
  fingerprint, baseline and HEAD evidence, but no completion marker.
- Failed-to-complete, complete-to-failed, and same-ID restart transitions are
  rejected.
- Any next-blocking finding forces readiness `Blocked`.
- A valid failed state ends Stop. Ordinary unreclosed report/workspace drift,
  conflicts, suspicious paths, or malformed state restores the block.
- Reclosure is limited to the same increment, baseline, report path, and HEAD.
  Post-commit reclosure is rejected; identical committed content remains valid.
- In the checkout retaining the ignored state, a different gate may begin only
  from valid non-Blocked failed evidence after the workspace is clean and only
  with separate owner approval. This report is Blocked, so the same checkout
  denies a successor; repository policy and owner authority prohibit using a
  fresh clone to bypass that disposition.
- Local hashes detect ordinary drift; they are not same-user authentication or
  durable audit and do not survive a fresh clone.

### Threats and tests

Focused tests cover PASS-as-failure rejection; failed verification, pending
manual evidence and High blocking findings; exact failed keys; completion-
marker injection; failed-to-complete and complete-to-failed rejection; Stop
behavior; report/workspace drift; merge conflicts; suspicious paths; readiness
laundering; dirty or Blocked successor denial; clean non-Blocked fixture-only
successor admission, including Blocked-to-non-Blocked same-HEAD reclosure before
commit; same-HEAD reclosure; post-commit reclosure denial; legacy
state compatibility; closed finding categories; and missing or duplicate scope
sections. Full repository verification, audit, documentation, security, diff,
and session-end checks remain required after the final edit.

### Rollback and stop conditions

Before terminal closure, revert only this amendment's reviewed hook/document
diff if focused validation fails; do not delete or rewrite unrelated recovery
evidence. After closure, any edit invalidates the local failed binding until the
same report is revalidated and reclosed under the same HEAD. Stop on any product
path, dependency, workflow, credential, Apple, Keychain, signing, network,
publication, second-gate, false-PASS, or broader override requirement.

### Result

Implementation and final evidence are recorded in the existing post-increment
review. The only permissible terminal result remains `failed` / `FAIL` /
`Blocked`, valid true, with no completion marker.
