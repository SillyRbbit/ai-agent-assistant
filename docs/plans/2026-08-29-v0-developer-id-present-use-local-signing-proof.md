# V0 prerequisite — Developer ID present-use and local signing proof

Status: Evidence-standard documentation reconciliation accepted under D-096;
operational execution Blocked
Owner: Henry Dang
Last updated: 2026-08-29
Parent increment: `v0-xcode-developer-id-recovery-execution`
Baseline: `0931df66c389bdc13c705d1259706c4d3770761c`
Depends on: D-072, D-075, D-076, D-095, D-096, TS-017, the terminal failed
recovery record, and the consumed `keychain_identity_v1` result

## Goal

Define the smallest truthful follow-up for the existing Xcode-created
Developer ID Application identity. A later, separately approved owner-operated
run may establish only both of these facts:

1. the current owner-operated macOS session can use exactly one matching
   Developer ID Application private key to sign one disposable local Cortexa
   application bundle; and
2. that bundle has the repository-fixed `com.aiagentassistant.desktop`
   identifier, a valid Developer ID Application signature, the hardened-runtime
   flag, no unreviewed entitlement, and the default designated requirement
   expected for that identifier and certificate class.

The plan also corrects the evidence standard before signing. Present pairing,
identity-list, and signature evidence cannot retrospectively prove that a
private key was never exported, copied, backed up, synchronized, or
compromised. Those selected observations also do not establish the current
item's technical extractability attribute or exclusive custody. Apple exposes
a separate current-item extractability attribute, but this plan deliberately
prohibits that additional query; even that attribute would not disprove a prior
copy. The strongest truthful owner-control evidence selected here is:

- the owner categorically attests that no private-key export, import, copy,
  backup, share, escrow, or cloud-synchronization action is known or was
  performed;
- the approved workflow contains no private-key export operation or standalone
  certificate-file export;
- the current owner-operated session successfully uses the key once; and
- technical non-extractability, historical absence of export, and exclusive
  custody remain explicitly `not_proven`.

If that evidence standard is not accepted or the owner cannot truthfully make
the bounded attestation, the operation stops. No export attempt, private-key
byte extraction, or broader Keychain inspection may be used as a substitute
proof.

## User-visible outcome

This documentation step created this plan, accepted D-096, and reconciled the
queue with the newly identified evidence limitation. It ran no Keychain, Apple,
build, or signing command.

The future `developer_id_local_signing_v1` sanitizer, if separately approved,
may emit only the following closed categories:

- `source_local=clean_head_matches_recorded_origin_ref | blocked`;
- `source_remote=matched_after_approved_ls_remote | not_checked | blocked`;
- `source_branch=matched_application_owned | blocked`;
- `source_commit=matched_application_owned | blocked`;
- `toolchains=matched_repository_pins | blocked`;
- `xcode_context=matched_reviewed_target | blocked`;
- `gate_state=expected_active_failed_gate | blocked`;
- `owner_attested_known_private_key_export=none_known | known | declined`;
- `approved_workflow_private_key_export=not_performed`;
- `technical_nonextractability=not_proven`;
- `historical_absence_of_export=not_proven`;
- `exclusive_custody=not_proven`;
- `git_remote_network=approved_anonymous_read_only | blocked`;
- `npm_install_network=approved_online | offline_enforced | blocked`;
- `npm_audit_network=approved_online | blocked`;
- `cargo_network=approved_online | offline_enforced | blocked`;
- `apple_trust_network=accepted_uncontrolled | not_accepted`;
- `disposable_clone=passed | failed`;
- `build_script_containment=passed | blocked`;
- `dependency_install=passed | failed`;
- `npm_audit=passed_zero_vulnerabilities | failed`;
- `repository_verify=passed | failed`;
- `bundle_shape=expected_single_executable | blocked_unexpected_content`;
- `signing_selection=single_trusted_team_bound_identity | blocked_unavailable_or_ambiguous`;
- `signing_process=completed | failed | timeout_unknown`;
- `bundle_identifier=matched | mismatched | unavailable`;
- `developer_id_requirement=matched | mismatched | unavailable`;
- `signature_integrity=passed | failed`;
- `hardened_runtime=present | absent | unconfirmed`;
- `entitlements=expected_none | unexpected | unconfirmed`;
- `timestamp=not_requested`;
- `notarization=not_run`;
- `gatekeeper=not_run`;
- `distribution=not_run`;
- `artifact_cleanup=passed | failed | not_run`; and
- `tracked_tree=unchanged | changed`.

The sanitizer has no trusted GUI-observation channel and therefore cannot emit
prompt or visible-state claims. After the process terminates, the owner may
separately report only:

- `manual_authorization_prompt=not_observed | observed_blocked | not_reported`;
  and
- `manual_visible_state_change=not_observed | suspected | not_reported`.

Because this plan performs no ACL query, persistent signing-key ACL change is
always `unconfirmed`; the owner-visible state category is not a substitute for
an ACL inspection.

An observed prompt stops the run without interaction. A child timeout or closed
signing error remains sanitizer evidence; it is not reclassified as proof that
a prompt appeared.

No user or account identifier, account-home or Keychain path, certificate label,
common name, fingerprint, serial, Team ID, designated-requirement text,
certificate chain, raw stdout/stderr, prompt contents, screenshot, transcript,
password, private-key bytes, or provider credential may enter chat, Git,
wrapper-owned output, or a redirected evidence file. Repository and temporary
paths may be visible to same-user process inspection and transient tool logs
inside the workflow-owned root; they must not cross the closed result surface,
and cleanup limitations remain disclosed.

The disposable signed `.app` necessarily embeds public certificate-chain, Team
ID, signer, and designated-requirement metadata. That metadata is authorized
only inside the ephemeral artifact, may be visible to same-user/system
inspection, may remain if cleanup fails, and may never be copied into textual
evidence, chat, Git, or logs.

Separate non-sensitive preflight evidence may record the application-owned
expected repository branch and commit, pinned public toolchain versions, and
the sanitizer's closed match/status categories, bounded parsed test counts, and
allowlisted warnings. The future operation may not publish raw preflight
command output. That repository-evidence contract may receive only values
parsed by the reviewed sanitizer and never includes an account-home, Keychain,
temporary-artifact, certificate, signer, team, dirty path, diagnostic, or
unbounded value.

## Scope

This plan has two dependency-ordered milestones:

1. **Evidence-standard reconciliation — documentation only.** Add one decision
   that additively supersedes only the prospective use of the historical
   phrases “non-exported owner-controlled private key” and
   “non-exportability” with the closed present-use and owner-attestation
   categories above. Preserve the original decision text, the historical
   screenshot/privacy failure as Failed, and the then-active recovery gate as
   `FAIL`; do not rewrite history or finalize a passing marker.
2. **One local signing proof — future operation.** After separate approval,
   create one workflow-owned no-hardlink disposable clone at the exact
   synchronized commit, perform every install/audit/verify/build write inside
   its temporary root, resolve and cryptographically bind one existing matching
   identity without caller-selected identity metadata, sign one derived `.app`
   once with secure timestamping explicitly disabled, verify internally, emit
   closed categories, and attempt descriptor-bound cleanup only of that root.

Milestone 1 is accepted under D-096 as an in-gate documentation amendment; it
did not begin a successor increment or second gate. Milestone 2 is Blocked
until the current terminal failed record no longer has Blocked successor
readiness through a separately approved truthful disposition, the Open
Directory boundary is resolved, the exact sanitizer is present in this plan and
statically reviewed, all other manual gates Pass, and the owner grants a fresh
one-attempt operational approval with the residual-risk acknowledgement below.

D-097 and its same-active-gate implementation now represent the immutable
failure as a valid terminal `failed` state without a completion marker. The
current report remains `FAIL` with successor readiness `Blocked`. D-097
deliberately rejects reclosure after HEAD changes, so any future post-commit
supersession requires a separately approved cumulative-evidence design; this
plan authorizes neither that hook change nor a second gate.

## Explicit non-goals

- No repeat of `keychain_identity_v1` as a standalone query.
- No proof or claim of “never exported,” technical non-extractability,
  historical exclusivity, hardware backing, exclusive custody, or absence of
  prior compromise.
- No private-key byte extraction, export test, backup test, migration test, or
  key extractability query.
- No certificate or key creation, replacement, renewal, import, standalone
  certificate-file or private-key export/persistence, presentation, revocation,
  removal, trust change, ACL change, Keychain repair, or search-list change.
  The future sanitizer may copy only bounded public leaf-certificate bytes into
  memory long enough to parse and validate class, trust, team, and fingerprint;
  public certificate-chain metadata exists only inside the disposable signature,
  and the wrapper may not otherwise persist, forward, print, or return those
  bytes or any derived identifier.
- No source, test, dependency, lockfile, Tauri configuration, entitlement,
  provisioning profile, capability, permission, CSP, workflow, hook, script,
  signing-identity configuration, or system-wide `xcode-select` change.
- No app launch, product credential-Keychain-item creation/read,
  unsigned-copy comparison, credential, Cloudflare, provider, model, gateway,
  transport, Tauri/WebView, persistence, filesystem feature, tool,
  real-content, or V0-3 implementation. The future identity lookup and signing
  operation may exercise the existing signing private key through
  Security.framework but may never extract its bytes.
- No secure timestamp, notarization, stapling, Gatekeeper assessment, DMG,
  installer, App Store action, distribution, release, or publication.
- No retry, fallback identity, alternate keychain, `sudo`, `openssl`, custom
  requirement, `--deep` signing shortcut, or caller-supplied identity, bundle,
  path, profile, runtime, or workflow selector.

## Existing behavior and constraints

- Clean synchronized source is recorded at baseline
  `0931df66c389bdc13c705d1259706c4d3770761c`; the current branch contains only
  the bounded recovery evidence plus D-097 repository gate/test and governance
  closeout work. No product source is changed.
- Xcode 26.6 created and lists one Developer ID Application certificate record.
- The owner observed the certificate and private key paired in Keychain Access.
- The consumed exact `keychain_identity_v1` run found one label-matched valid
  code-signing identity in the current user's default Keychain. That result
  proves present visibility only; it does not prove provenance, signing use,
  export history, non-extractability, or exclusive custody.
- The owner reported no prompt and no visible state change from that read-only
  query. Its approval is consumed and cannot authorize this plan's operation.
- The repository fixes product name `Cortexa`, application identifier
  `com.aiagentassistant.desktop`, app bundling, and macOS hardened runtime in
  `src-tauri/tauri.conf.json`. It contains no signing identity or entitlement
  configuration.
- Tauri's pinned CLI exposes `--bundles app` and `--no-sign`. The local
  `codesign(1)` documentation states that `--timestamp=none` explicitly
  disables timestamp-service use, `--keychain` narrows identity lookup, and
  ambiguous identity matching fails.
- An ignored bundle currently exists at the repository's ordinary target path.
  A future proof must not overwrite or treat that pre-existing artifact as
  workflow-owned.
- The recovery gate is terminally `failed` / `FAIL` / `Blocked` because the
  historical screenshot boundary failed and required manual/signing evidence
  was incomplete. This plan cannot convert that historical failure into Passed
  or begin a second gate.

## Current-state evidence and references

- Apple describes Developer ID Application as the certificate used to sign a
  Mac app distributed outside the Mac App Store and documents Xcode as one
  certificate-creation route: [Developer ID certificates](https://developer.apple.com/help/account/certificates/create-developer-id-certificates/).
- Apple documents that Keychain Access can export some certificates and keys.
  Therefore ordinary Xcode/login-Keychain presence is not itself evidence of
  non-extractability or no prior export: [Import and export keychain items](https://support.apple.com/guide/keychain-access/kyca35961/mac).
- Apple explains that code signing requirements establish code identity and
  that the system synthesizes a default designated requirement when code is
  signed without a custom one: [TN3127](https://developer.apple.com/documentation/technotes/tn3127-inside-code-signing-requirements)
  and [Applying Code Requirements](https://developer.apple.com/documentation/security/applying-code-requirements).
- Apple states that code-signing verification proves signature validity for
  the checked code but does not by itself establish suitability for every
  distribution or platform policy: [TN3161](https://developer.apple.com/documentation/technotes/tn3161-inside-code-signing-certificates).
- Tauri documents the `.app` bundle operation and its generated bundle shape:
  [macOS Application Bundle](https://v2.tauri.app/distribute/macos-application-bundle/).

## Files expected to change

This plan-drafting step may change documentation only:

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
- `docs/increments/v0-xcode-developer-id-recovery-execution.md`
- `docs/plans/2026-08-28-personal-assistant-v0-xcode-developer-id-recovery.md`
- `docs/plans/2026-08-29-v0-developer-id-present-use-local-signing-proof.md`
- `docs/plans/2026-08-29-v0-keychain-scoped-identity-discrepancy.md`
- `docs/reviews/2026-08-28-v0-xcode-developer-id-recovery-execution-post-increment-review.md`

The later documentation reconciliation may change the same existing memory,
decision, increment, plan, and review paths. The future operational run may
change no path in the authoritative checkout. Configured repository,
dependency, cache, log, frontend, Cargo, bundle, signing, and cleanup outputs
must be routed beneath one exact workflow-owned temporary root containing a
disposable no-hardlink clone and its signed `.app`. That routing alone does not
confine executable npm lifecycle scripts, Cargo build scripts, or Tauri/
frontend subprocesses. Operational readiness therefore requires a separately
reviewed containment or observation mechanism that fails closed on any outside-
root write, undeclared network action, or escaped child. OS-managed trust/cache/
log state remains a separately disclosed residual boundary.

## Affected components

| Component                   | Effect                                                                                                                                      |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| Repository governance       | Additively supersedes only prospective use of an unattainable historical-proof phrase without rewriting prior decisions or weakening D-072. |
| Target-Mac Keychain         | Future read/use of exactly one existing Developer ID Application identity; no item mutation is authorized.                                  |
| Tauri build output          | Future one-time no-identity-signed `.app` bundle wholly inside a workflow-owned disposable clone; an ad-hoc linker signature may exist.     |
| macOS Code Signing Services | Future one-time local signature plus read-only verification, with timestamp service explicitly disabled.                                    |
| Product runtime             | None; the signed artifact is not launched or distributed.                                                                                   |

## Interfaces and invariants

### Documentation evidence contract

- `owner_attested_known_private_key_export=none_known` means only that the owner
  knows of no private-key export/import/copy/backup/share/escrow/cloud-sync
  action and attests that none was performed by them. It is not a forensic or
  cryptographic conclusion.
- `approved_workflow_private_key_export=not_performed` describes only the
  reviewed workflow; it does not deny the public chain embedded in the signed
  bundle.
- `technical_nonextractability`, `historical_absence_of_export`, and
  `exclusive_custody` are fixed to `not_proven` on this path.
- A successful signature proves present private-key use by the current
  owner-operated session only.
- Historical evidence and failures remain additive and immutable.

### Future `developer_id_local_signing_v1` sanitizer contract

Before operational approval, this plan must contain and pass static review of
one exact, no-argument sanitizer named `developer_id_local_signing_v1`. The
sanitizer must:

- accept no stdin, environment-selected signer, caller path, label, hash,
  certificate, Team ID, bundle ID, requirement, entitlement, or option;
- use absolute tool paths, repository-pinned toolchain versions, a fully
  enumerated minimal environment, shell-free child processes, bounded captures,
  exact timeouts, strict UTF-8 parsing, a closed output allowlist, and zero
  retries;
- reject every unreviewed environment input, including Apple signing or
  notarization values and injection-capable `NODE_OPTIONS`, `NPM_CONFIG_*`,
  proxy, `RUSTFLAGS`, `CARGO_*`, `RUSTUP_*`, and `TAURI_*` values; the exact
  sanitizer must freeze the permitted locale, npm/Cargo cache and log roots,
  toolchain selectors, and network policy before operational readiness;
- remain operationally Blocked until a no-new-dependency containment or
  observation design addresses executable npm lifecycle scripts, Cargo build
  scripts, and Tauri/frontend subprocesses. Environment/cache routing and a
  process group alone do not prove they cannot write elsewhere or create an
  undeclared network/process effect; the exact design must fail closed on an
  outside-root write, undeclared connection, or escaped child and may not claim
  whole-device confinement it cannot enforce;
- start each npm/Tauri/Cargo process tree in a new process group or session;
  on timeout or failure, terminate then kill the whole group within fixed
  deadlines, reap every direct child it owns, confirm that the process group no
  longer exists, close every pipe, and run the separately reviewed bounded
  quiescence check before any cleanup. A process group does not prove that a
  descendant did not escape into a new session; if the fixed command graph or
  quiescence result cannot exclude that case, leave the root in place and emit
  `artifact_cleanup=failed`;
- resolve the current non-root user and that user's default Keychain internally,
  validate the Keychain path, and scope signing identity lookup to it;
- require exactly one valid identity whose private label begins with the fixed
  `Developer ID Application: ` class prefix, then validate Apple generic trust,
  the Developer ID Application certificate class, and a separately resolved
  application-owned expected-team binding before private-key use;
- capture and freeze the exact selected certificate fingerprint internally,
  pass only that fingerprint as the `codesign` selector, and confirm the signed
  leaf matches it. The fingerprint may appear in child argv and must therefore
  be included in the future same-user process-observability acknowledgement,
  but it may never enter wrapper output, chat, Git, or a redirected file;
- remain operationally Blocked until the no-caller-input expected-team binding
  source is separately selected and reviewed; a label prefix alone is not a
  trusted signer or team binding;
- copy only a bounded public leaf certificate into memory through the reviewed
  Security.framework interface needed for trust/class/team/fingerprint
  validation; only bounded in-memory DER/attribute parsing is permitted. Do not
  query private-key bytes, persist or separately export the public certificate,
  expose its fields, enumerate unrelated items, or retain the in-memory copy
  beyond the process;
- create one mode-0700 fixed-purpose temporary root that did not exist, retain
  a validated descriptor for the fixed parent, the root basename, an open root
  descriptor, and the original device/inode/owner/mode; place an ownership
  marker in it;
- capture every authoritative-checkout preflight byte inside the sanitizer and
  parse only closed branch/OID/cleanliness/ancestry/gate/toolchain categories;
  publish no dirty path or raw diagnostic. Use `GIT_OPTIONAL_LOCKS=0`, a
  workflow-owned temporary `HOME`/XDG/config/cache set,
  `GIT_CONFIG_NOSYSTEM=1`, fixed overrides disabling fsmonitor, untracked-cache,
  external attribute files, prompting, and credential helpers, and validate or
  neutralize checkout-local Git configuration and `.git/info/attributes` before
  trusting a working-tree result;
- create a fresh disposable local Git clone at the exact approved commit inside
  the root with `--no-hardlinks --no-checkout` under that same isolated Git
  environment and `core.hooksPath` fixed to an empty workflow-owned directory;
- inspect the approved tree before checkout and stop if it contains a tracked
  `.gitattributes` or `.gitmodules`; permit no smudge/clean/process filter, LFS,
  hook, submodule, alternates file, object borrowing, or host/user/system Git
  configuration. Check out only the exact detached commit, then require the
  expected commit and tree, no alternates, and a clean status before continuing;
- perform **all** dependency installation, audit, repository verification,
  frontend output, Cargo output, and Tauri bundling inside that clone or other
  root-owned cache/target directories; never run a writing build or
  verification command in the authoritative checkout and never overwrite its
  pre-existing ignored `dist/` or `src-tauri/target` outputs;
- with Git prompting disabled, capture one bounded read-only `ls-remote` result
  for the internally allowlisted canonical HTTPS `origin`, with fixed Git
  config, `GIT_TERMINAL_PROMPT=0`, false askpass programs, empty credential
  helpers, and SSH prohibited. Require its sole `refs/heads/main` commit to
  equal the approved commit before any dependency, build, or private-key use.
  This remote-freshness check requires separate network/process-metadata
  approval and must not update a ref; without it, report
  `source_remote=not_checked` and stop;
- invoke `npm ci --no-audit --no-fund` under the exact selected online or
  enforced-offline install policy, then `npm audit --audit-level=low --json` as
  the sole audit-network operation, followed by `npm run verify` and exactly
  `npm run tauri -- build --bundles app --no-sign` through the pinned toolchain
  in the disposable clone. Parse bounded audit JSON from unknown, require the
  expected schema, a successful exit, and exact zero values for total, info,
  low, moderate, high, and critical vulnerabilities before emitting
  `npm_audit=passed_zero_vulnerabilities`; never forward the JSON. Stop if the
  build emits or requires Developer ID or other identity signing,
  notarization, a DMG, or a different bundle. A linker-generated ad-hoc
  signature is allowed only as untrusted pre-sign state and may be replaced by
  the sole authorized `--force` operation;
- require the exact expected bundle shape and reject every nested `.app`,
  framework, plug-in, helper, XPC service, dylib, or other signable code;
- read the generated `Info.plist` internally and require the fixed bundle
  identifier before signing;
- invoke `/usr/bin/codesign` once, without `sudo`, custom entitlements, custom
  requirements, or `--deep`, using the frozen exact fingerprint, the internally
  resolved Keychain, `--force`, `--options runtime`, and `--timestamp=none`
  against only the internally derived disposable bundle;
- capture every signing and verification byte without forwarding it;
- verify the bundle strictly and recursively, check the fixed identifier and
  Developer ID Application certificate-class requirement, confirm the runtime
  flag, and reject any unreviewed entitlement;
- emit only the closed categories in this plan; and
- after every owned direct child is reaped, its process group no longer exists,
  and the reviewed quiescence check passes, compare the fixed parent's open
  descriptor and an `fstatat`-style non-following lookup of the retained root
  basename to the open root descriptor's device/inode/owner/mode. Then perform
  only reviewed `unlinkat`-style descriptor-relative cleanup of that exact
  object. If quiescence, name binding, identity, or containment is uncertain,
  leave the artifact in place, emit `artifact_cleanup=failed`, and require a
  separate cleanup plan; never broaden deletion.

No raw `codesign --display`, identity, or requirement output may be presented
to the owner. The explicit Developer ID requirement must be frozen and reviewed
inside the sanitizer before this plan can become operationally Ready.

## Implementation milestones

- [x] Record the evidence limitation and draft this documentation-only plan.
- [x] Add and accept D-096; reconcile D-075/D-076/D-095 and active recovery
      records without rewriting history.
- [ ] Add the exact fenced `developer_id_local_signing_v1` sanitizer to this
      plan and statically review its parser, subprocess, timeout, prompt,
      artifact, requirement, entitlement, cleanup, and output behavior.
- [ ] Run documentation-tier validation and independent architecture/security/
      readiness review of the final operational text.
- [ ] Obtain the owner's exact categorical attestation and residual-risk
      acknowledgement.
- [ ] After a separately approved truthful non-Blocked disposition and
      successor plan, obtain a fresh, explicit, one-attempt operational
      approval for that bounded successor.
- [ ] Execute once, record only closed categories, rerun required verification,
      and keep V0-3 Blocked pending its separate plan.

## Security and privacy considerations

| Threat or uncertainty                                  | Required control                                                                                                                                                                                  |
| ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Present use is overstated as historic custody          | Use the exact evidence taxonomy above; never say “never exported,” “non-exportable,” or “exclusive custody proved.”                                                                               |
| An export attempt leaks the private key                | Export, extractability probing, copying, backup, import, and private-key byte extraction are prohibited.                                                                                          |
| Wrong or ambiguous signer is used                      | Validate one Apple-trusted class/team identity, freeze its exact fingerprint, sign by that fingerprint, and confirm the signed leaf matches; accept no caller identity.                           |
| Signer metadata leaks                                  | Bounded internal public-leaf parsing only; acknowledge artifact and same-user argv visibility, and emit no raw output, screenshots, redirection, transcript, identifiers, or diagnostics.         |
| Tauri signs or notarizes implicitly                    | Stop on related environment variables and build with `--no-sign`; perform the sole fixed local signing operation explicitly.                                                                      |
| Timestamping creates intentional Apple traffic         | Use `--timestamp=none` and state that the artifact is not distribution-ready.                                                                                                                     |
| OS trust evaluation still uses network/cache/log state | Disclose that certificate/revocation traffic and OS cache, diagnostics, unified logs, and other state are not disproven or controlled.                                                            |
| Repository build output is overwritten                 | Clone the approved commit without hardlinks and run npm, verification, frontend, Cargo, and Tauri writes only inside the marker-owned temporary root.                                             |
| Executable build scripts escape configured roots       | Keep operation Blocked until a reviewed no-new-dependency control detects or contains outside-root writes, undeclared network effects, and escaped children; environment routing is insufficient. |
| Signing mutates the wrong artifact                     | Derive the disposable-clone path internally, allow only ad-hoc pre-sign state, and stop on pre-existing identity signing or unexpected content.                                                   |
| A prompt requests private-key authorization            | Preserve the parent recovery invariant: stop on any prompt without interaction. A separately approved decision would be required before any one-time **Allow** policy.                            |
| Cleanup races a live or escaped child or swapped path  | Kill the owned group, require bounded quiescence, validate parent/name/root descriptors, and use non-following descriptor-relative cleanup or leave the root untouched.                           |
| A local signature is mistaken for release evidence     | Keep timestamp, notarization, Gatekeeper, distribution, App Store, installer, update, and launch checks `not_run`.                                                                                |

The future owner acknowledgement must accept possible OS-managed certificate or
revocation traffic, one noninteractive Git remote-main query, npm
registry/audit/package traffic, Cargo registry traffic, temporary npm/Cargo
cache and log state, OS cache/log/diagnostic/state effects,
same-user process visibility of the account-home, Keychain, repository,
temporary artifact paths, and selected certificate fingerprint, one disposable
clone/build artifact, one signature mutation, the possibility that any prompt
stops the run, and exact cleanup or cleanup failure. It must acknowledge that
no private-key bytes are requested but signing exercises the private key through
Security.framework. The final sanitizer must either obtain separate approval
for each declared network/cache surface or enforce and verify an exact reviewed
package-tool offline mode. Git credential-helper, SSH-agent/key, browser, and
interactive authentication use are prohibited; stop if anonymous read-only
remote comparison is unavailable and require a separate repository-sync plan.

## Test plan

### Documentation milestone

- Verify all current-state records use the new categories consistently.
- Assert historical screenshot/privacy evidence remains Failed.
- Assert D-097 records valid terminal `failed` / `FAIL` / `Blocked` with no
  completion marker.
- Assert no product, dependency, configuration, workflow, or script path
  changed; the exact gate hook and focused test are the separately authorized
  D-097 governance diff.

### Static sanitizer review milestone

Table-drive the exact plan text or a review-only parser harness against:

- zero, one, and multiple valid Developer ID Application identities;
- malformed, oversized, non-UTF-8, diagnostic, warning, timeout, signal, and
  partial child output;
- hostile system/global Git config, hooks, credential helpers, askpass,
  proxies, object alternates, tracked `.gitattributes`, tracked `.gitmodules`,
  `.git/info/attributes`, fsmonitor, untracked cache, filters/LFS, submodules, a
  dirty or non-detached clone, wrong tree, stale local `origin/main`, remote
  authentication demand, and wrong remote `main`;
- an npm lifecycle, Cargo build, Tauri, or frontend child attempting an outside-
  root write, undeclared network connection, new session, or otherwise escaped
  descendant, including proof that the selected containment/observation control
  fails closed without broad cleanup;
- wrong user, root/effective-user mismatch, invalid/default-Keychain ambiguity,
  symlinked paths, and pre-existing temporary roots;
- unexpected Apple credential/signing/notarization, Node/npm, proxy,
  Rust/Cargo/rustup, and Tauri environment variables;
- wrong Apple trust/class/team binding, fingerprint/leaf mismatch, and identity
  mutation between enumeration and signing;
- wrong bundle identifier, malformed Info.plist, unexpected bundle shape,
  nested signable code, allowed ad-hoc linker signature, and rejected existing
  non-ad-hoc identity signature;
- successful bounded npm-audit JSON with all vulnerability counts at zero,
  nonzero counts at each severity, inconsistent totals, missing/extra or wrong-
  typed fields, oversized JSON, non-JSON output, and a nonzero audit exit, plus
  proof that repository `.npmrc` cannot re-enable implicit audit during the
  `npm ci --no-audit` install step;
- child completion, timeout, and closed signing failure without attributing a
  GUI cause, plus schema tests for each separate owner-observed prompt/state
  category;
- signing failure, verification failure, wrong Developer ID requirement,
  missing runtime flag, unexpected entitlement, process-group termination,
  an escaped-session descendant or otherwise unproven quiescence,
  parent/name/root descriptor mismatch, directory-swap/device/inode mismatch,
  and cleanup failure; and
- proof that every failure emits one allowlisted closed category and performs
  zero retry.

### Future target-Mac operation

- Freeze the exact expected branch, commit, local `origin/main`, ancestry, gate
  state, Node/npm/Rust/Cargo/Tauri/Xcode versions, and developer-tool context in
  the reviewed plan/sanitizer before approval. The sanitizer captures the
  authoritative-checkout preflight and emits only closed match/status
  categories; it never forwards dirty paths, host diagnostics, or raw output.
- Run the sanitizer exactly once; it owns the disposable clone, dependency
  installation, audit, complete verification, build, signing, validation, and
  cleanup attempt.
- After process termination, obtain the owner's separate closed prompt and
  visible-state observations without inferring them from child status.
- Recheck tracked Git state and protected paths after cleanup.
- Record Apple/Xcode/Keychain/signing/manual evidence only through the closed
  categories above.

## Verification commands

Documentation drafting and reconciliation use only:

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri package.json package-lock.json \
  src-tauri/Cargo.toml src-tauri/Cargo.lock .github .codex/hooks scripts
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

The future operation may run no raw public preflight command. Repository,
gate, toolchain, build, signing, validation, and cleanup subprocesses may run
only through the statically reviewed no-argument sanitizer, which captures all
bytes and emits only the closed categories above. The application-owned exact
expected branch, commit, local remote ref, toolchain versions, and gate identity
must be frozen in the accepted plan before operational approval. No preflight
or signing command is authorized by this plan draft.

## Manual gates

Before any future operation, all must be Passed:

- D-096 evidence-standard decision and documentation reconciliation — Passed;
- D-097 terminal-failed-gate disposition and reviewed gate-state design that
  preserves the historical privacy failure without a passing marker or second
  active gate — Passed;
- a separately approved truthful transition from the current Blocked failed
  readiness before a successor gate may begin;
- independent security and architecture review of the exact sanitizer;
- a frozen application-owned expected source branch, commit, local remote ref,
  pinned toolchain set, Xcode context, and governance gate identity;
- an approved no-caller-input source for the expected Apple Developer team
  binding and exact pre-sign trust/class/team/fingerprint validation;
- a separately approved, noninteractive read-only remote-main comparison that
  matches the approved commit without updating the authoritative checkout;
- a reviewed isolated-clone, fixed-environment, process-group, cache/network,
  executable-build-script containment/observation, implicit-audit suppression,
  bounded quiescence, and descriptor-bound cleanup design in the exact
  sanitizer;
- owner confirmation that the target Mac remains personally controlled and the
  owner remains the relevant Apple Developer Account Holder;
- owner attestation `owner_attested_known_private_key_export=none_known`, or
  stop;
- owner acceptance of the exact trust, network, cache/log/state,
  process-metadata, private-key-use, prompt, artifact, and cleanup boundaries;
- owner acknowledgement that any Keychain authorization, password, Touch ID,
  or other prompt stops the run without interaction; and
- a fresh explicit one-attempt operational approval naming
  `developer_id_local_signing_v1`.

## Stop conditions

Stop without retry if:

- the source is dirty, divergent, ambiguous, not the approved commit, or the
  terminal recovery record is replaced or an unauthorized second gate is
  begun;
- the terminal failed record is invalid or retains Blocked readiness, or any
  proposed transition would require rewriting Failed evidence, marking `FAIL`
  as passing, post-commit reclosure without cumulative evidence, or operating
  two active gates;
- the evidence decision is absent, the owner attestation is not confirmable, or
  any result would require a “never exported” or exclusive-custody claim;
- raw evidence, an identifier, a path, a fingerprint, a Team ID, a certificate
  name, a requirement string, or diagnostic output would cross the sanitizer's
  closed output boundary;
- an identity, keychain, bundle, path, requirement, entitlement, runtime option,
  or workflow can be caller-selected;
- no identity or multiple identities match, or a retry/fallback seems useful;
- remote `main` freshness is not checked, the remote is unexpected, an
  authentication prompt would be required, or the approved commit does not
  match remote `main`;
- Apple trust, Developer ID Application class, expected-team binding, or exact
  fingerprint selection cannot be established before private-key use, or the
  post-sign signed-leaf equality check fails;
- any Keychain authorization, password, Touch ID, Apple sign-in, Keychain
  unlock/repair, export/import, certificate creation, trust change, or other
  prompt appears;
- unexpected environment, network, device, filesystem, Keychain, account, or
  OS state is observed;
- the isolated target root already exists, is a symlink, lacks its ownership
  marker, escapes the fixed parent, or contains unexpected code;
- the authoritative checkout would receive any dependency, cache, log,
  frontend, Cargo, bundle, or cleanup write;
- executable dependency/build code can write outside the workflow-owned root,
  make an undeclared connection, or escape the owned process/quiescence model;
- bundle identifier, Developer ID requirement, runtime flag, entitlement,
  signature, or cleanup validation fails;
- any tracked file or protected path changes; or
- any source, configuration, dependency, entitlement, profile, script,
  notarization, distribution, provider, credential, or V0-3 change is needed.

## Risks

- The owner attestation is useful governance evidence but not cryptographic or
  forensic proof.
- Ordinary Developer ID private keys may be exportable. This plan intentionally
  makes no contrary claim.
- A local timestamp-free signature proves only present local key use and code
  identity; it is not a release, Gatekeeper, or distribution proof.
- Security.framework and Code Signing Services may create OS-managed network,
  cache, diagnostic, log, or trust state outside the wrapper's control.
- `npm ci`, npm audit, and Cargo dependency resolution may use package-registry
  traffic. Their caches/logs must be isolated under the temporary root, and
  their traffic requires explicit later approval unless an exact reviewed
  offline mode is selected.
- npm lifecycle scripts, Cargo build scripts, and Tauri/frontend subprocesses
  are executable child code. Environment and cache routing do not by themselves
  prevent writes or process/network effects elsewhere on the target Mac; an
  exact containment or observation design remains unselected and blocking.
- Same-user process inspection or system auditing may observe fixed and private
  paths plus the selected certificate fingerprint while child processes run.
- No application-owned expected-team binding source is selected yet. A class
  label or Apple certificate class alone is insufficient, so this remains an
  explicit operational blocker.
- A signing authorization prompt stops the workflow. Any future proposal to
  permit one-time **Allow** must explicitly supersede the parent prompt policy
  and account for OS-managed observation or state.
- Cleanup is ordinary filesystem deletion, not secure erasure of APFS/SSD
  remnants, snapshots, caches, or logs.

## Rollback or failure strategy

This plan draft rolls back by reverting only its documentation changes. It
creates no external state.

For a future operation, abandon before signing with no Keychain cleanup. After
the disposable clone or artifact is created, the reviewed sanitizer may attempt
cleanup only after its direct children are reaped, each owned process group no
longer exists, and the separately reviewed quiescence check passes. It must
revalidate the fixed parent, retained basename, and open root descriptors, use
non-following descriptor-relative cleanup, and leave the root in place rather
than risk deleting a substituted or possibly live path. It must never revoke,
remove, repair, export, import, or otherwise change the certificate or private
key. If cleanup fails or a persistent ACL/state change is suspected, stop and
require a separate incident plan. Deletion does not claim secure erasure.

## Decisions made

- The plan uses present-use evidence and a bounded owner attestation rather
  than an unattainable retrospective custody proof.
- Technical non-extractability, historical absence of export, and exclusive
  custody remain fixed as `not_proven`; no export or extractability test is
  selected.
- The proof is one local timestamp-free `.app` signature only. Notarization,
  Gatekeeper, launch, distribution, and V0-3 remain separate.
- The operation must use an exact no-argument sanitizer; no direct ad hoc shell
  signing sequence will be approved from this draft.
- The terminal failed recovery record is preserved. No successor gate begins.

These plan boundaries are accepted for documentation under D-096. That
decision additively governs future evidence while preserving the original
historical decision text; it grants no operational readiness or authority.

## Discoveries

- Apple documents export of some Keychain certificates and keys. The selected
  Xcode/login-Keychain path therefore does not inherently establish
  non-extractability.
- Present Keychain pairing and the successful scoped identity query prove
  current visibility, not prior export history or exclusive custody.
- The local `codesign` default may timestamp some signatures when `--timestamp`
  is omitted. `--timestamp=none` is required for the proposed local-only proof.
- A pre-existing ignored `.app` exists at the normal repository target path, so
  a future proof must use an isolated workflow-owned target root.
- `npm run tauri -- build` invokes the configured frontend build, and
  `npm run verify` writes ordinary ignored outputs. Setting only
  `CARGO_TARGET_DIR` would not isolate the proof; every writing command must run
  in a disposable no-hardlink clone.
- A local clone can inherit host Git configuration, credential helpers, hooks,
  filters, attributes, submodules, or object alternates. The exact workflow
  must isolate Git configuration, reject tracked checkout-active metadata, and
  verify a detached exact tree without borrowing objects.
- Raw `git status`, diff, gate, or toolchain/version output can disclose paths
  or host diagnostics on a failure path. The future sanitizer must capture and
  parse preflight internally and emit only closed match/status categories.
- A locally cached `origin/main` is not current-remote evidence. A later
  current-source claim requires its own approved, noninteractive, anonymous,
  bounded `ls-remote` comparison and must stop if that comparison is unavailable.
- npm-audit exit status alone is not the selected zero-vulnerability proof. The
  exact wrapper must parse bounded JSON from unknown and require every supported
  severity count and the total to be exactly zero.
- Repository `.npmrc` currently enables install-time audit. The future install
  must use `npm ci --no-audit` so only the separately classified explicit JSON
  audit can access the audit service.
- Public certificate, team, and requirement metadata is unavoidably embedded in
  a signed `.app`. The plan permits bounded in-memory public-leaf parsing and
  ephemeral artifact embedding only, while prohibiting textual evidence or
  persistence by the wrapper.
- D-097 now encodes the immutable result as exact terminal `failed` evidence
  without a completion marker. The current readiness is Blocked, and post-
  commit reclosure is deliberately unsupported; a separate cumulative-evidence
  supersession design is required before any successor or operational signing
  proof.
- npm/Cargo/Tauri environment routing controls configured outputs but does not
  confine executable lifecycle or build scripts. Operational readiness requires
  an exact fail-closed outside-root write/network/process control.
- Enumerating one label-matched identity and later signing by the label prefix
  does not bind the selected certificate. The future sanitizer must validate
  trust/class/team, freeze an exact fingerprint, sign by that token, and compare
  the signed leaf.
- npm/Tauri/Cargo form a process tree. A process group does not prove that a
  descendant could not escape into another session. Safe cleanup therefore
  requires bounded quiescence plus parent/name/root descriptor binding and
  non-following deletion, not a marker/path check or group exit alone.
- `--deep` is deprecated for signing but remains applicable to recursive
  verification. Nested code must instead stop this deliberately single-
  executable proof.

## Progress

- 2026-08-29: The owner approved the next bounded documentation plan. D-096 now
  accepts the closed present-use, owner-attestation, workflow-no-export, and
  `not_proven` evidence categories without rewriting D-095 or any historical
  result. No operation or second gate began.
- 2026-08-29: The owner authorized drafting the next bounded documentation-only
  plan. No operational execution, new gate, Keychain access, Apple action,
  private-key use, build, signing, or cleanup was authorized.
- 2026-08-29: Readiness and security review found that the prior
  “non-exported owner-controlled” phrase is not retrospectively provable on the
  selected ordinary Xcode/login-Keychain path. The plan was narrowed to
  truthful present-use and owner-attestation evidence.
- 2026-08-29: Independent static review found build-isolation, signer-binding,
  descendant-cleanup, extractability-wording, network/cache, prompt-policy, and
  private-key-wording defects. The draft now requires a host-config-isolated
  disposable clone, exact fingerprint/leaf binding, application-owned
  expected-team validation, bounded public-certificate parsing, process-group
  termination plus explicit quiescence and descriptor-bound cleanup, closed
  preflight capture, per-surface network policy, implicit-install-audit
  suppression, strict zero-count audit parsing, and stop-on-any-prompt behavior.
  D-097 later resolved the representation gap without completion authority.
  The resulting failed record's Blocked successor readiness, executable-build-
  script containment, expected-team source, exact quiescence check, and exact
  sanitizer remain operational blockers.

## Acceptance criteria

- [x] The draft distinguishes present key use from export history,
      non-extractability, and exclusive custody.
- [x] The draft defines one fixed-bundle, no-caller-input, timestamp-free local
      proof and keeps all release/product work out of scope.
- [x] The draft preserves the active failed gate and historical privacy failure.
- [x] The draft defines exact files, interfaces, evidence categories, risks,
      tests, manual gates, rollback, and stop conditions.
- [x] D-096 accepts the additive evidence standard and all current records are
      reconciled without rewriting historical evidence.
- [x] D-097 accepts and implements a terminal-failed-gate disposition without
      rewriting the historical failure or creating a false passing marker.
- [ ] The current failed record's Blocked readiness is truthfully superseded by
      an owner-approved cumulative-evidence mechanism before any successor gate.
- [ ] Executable dependency/build-script outside-root write, network, and child-
      escape behavior is contained or detected by a reviewed fail-closed design.
- [ ] The exact sanitizer is included and passes independent static review.
- [ ] The owner supplies the required attestation and risk acknowledgement.
- [ ] The owner separately approves one operational run.
- [ ] One run and its required target-Mac checks Pass with only closed evidence.

## Final results

Planning result: **Evidence-standard documentation reconciliation accepted;
operational execution Blocked**. No Keychain, Apple, build, signing, private-
key, cleanup, source, configuration, dependency, credential, provider, or
external operation ran. V0-3 remains Blocked.

## Documentation updates

- [x] Draft this plan and identify the current evidence-language discrepancy.
- [x] Accept and record D-096 governing future evidence.
- [x] Reconcile `HANDOFF.md`, `PROJECT_STATUS.md`, `NEXT_STEPS.md`, `PLANS.md`,
      `SECURITY.md`, `DECISIONS.md`, the parent plan/increment, and its review.
- [ ] Add the exact sanitizer and future closed operational results, if later
      approved and run.
