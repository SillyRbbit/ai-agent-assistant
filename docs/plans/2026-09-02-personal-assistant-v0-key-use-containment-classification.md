# Personal Assistant v0 in-process key-use containment classification

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Henry Dang
Last updated: 2026-09-02
Increment: `personal-assistant-v0-key-use-containment-classification`
Baseline: `2287c1bfe999d733495ee78728f4dc7a653f393f`
Predecessor: completed D-106 codeless signing-fixture classification

## Goal

Classify exactly one new containment-by-elimination candidate,
`in_process_security_framework_ephemeral_challenge_proof_v1`, against a frozen
current Apple public-source and pinned-Rust-source register. Determine whether
removing every build, helper, child process, bundle, filesystem artifact, and
`codesign` operation makes this exact in-memory key-use proof eligible only for
a later separately approved present-use implementation plan, or whether the
candidate remains ineligible or contract-unproved.

This is a static documentation review. It does not acquire an identity, call
Keychain or Security.framework, generate a challenge, use a private key, sign,
verify, build, or establish product-signing or V0-3 readiness.

## User-visible outcome

None. The output is one additive repository-governance decision with a closed
classification and source-minimized documentation evidence.

## Scope

1. Freeze exactly `in_process_security_framework_ephemeral_challenge_proof_v1`.
2. Review only the frozen sources and existing D-096 through D-106 decisions.
3. Separate a childless, fileless cryptographic liveness proof from product
   code signing and every build-bearing path.
4. Disposition every closed contract row exactly once.
5. Record one additive D-107 decision if that identifier remains available.
6. Preserve every historical Failed, Pending, Not-run, and negative result.
7. Reconcile only the fifteen documentation paths declared below.

## Explicit non-goals

- No product or test source, dependency, lockfile, configuration, capability,
  CSP, permission, entitlement, hook, workflow, helper, fixture, sanitizer,
  controller, build artifact, or generated content.
- No npm/Cargo/Tauri/compiler/linker build, package installation, lifecycle
  script, process launch, target-Mac probe, tracing, packet capture, filesystem
  effect test, or runtime observation.
- No Apple account, Xcode, Keychain, Security.framework, certificate,
  identity, private-key, signing, verification, trust, revocation, timestamp,
  Gatekeeper, notarization, distribution, release, or external-system action.
- No caller-, model-, WebView-, environment-, account-, path-, label-,
  identity-, algorithm-, attempt-, command-, policy-, retry-, or result-selected
  authority.
- No claim of a signed Cortexa app, stable client identity, hardened runtime,
  historical non-export, technical nonextractability, exclusive custody, or
  V0-3 readiness.
- No rewrite of D-096 through D-106 or the D-097 report, digests, failed gate,
  findings, readiness, or absent completion marker.
- No P3-3 through P3-5, P4, operational signing, V0-3, commit, push, merge,
  pull request, release, or publication.

## Existing behavior and constraints

- D-096 allows a later successful signature to prove only present-attempt key
  use. Historical absence of export, technical nonextractability, exclusive
  custody, and current operational signing remain unproved or Not run.
- D-100 permits only one closed, source-minimized, attempt-bound factual record.
- D-101 prohibits account/home/path resolution, ambient/default Keychain scope,
  search-list enumeration, fallback, and retry. It requires one exact opaque
  application credential-domain reference and separate platform-effect proof.
- D-102 contains no general waiver or `not_applicable` result. It remains fully
  binding for every executable-generating, artifact-generating, product-build,
  or otherwise build-bearing path.
- D-103 and D-105 are immutable negative classifications of their frozen
  candidate sets. D-106 does not admit the codeless fixture.
- The repository pins `security-framework` 3.7.0 and
  `security-framework-sys` 2.17.0 and forbids unsafe application Rust. It has no
  identity-selection, private-key-use, or signing boundary.

## Current-state evidence

- Clean local `main` and `origin/main` both resolved to
  `2287c1bfe999d733495ee78728f4dc7a653f393f` before branch creation.
- The predecessor ignored gate was `complete`, `valid: true`, and
  `PASS WITH ADVISORIES` for
  `personal-assistant-v0-codeless-signing-fixture-classification`.
- The owner approved the exact documentation-only candidate and source-review
  plan. Branch `codex/p3-in-process-key-use-containment-classification` was
  created from that clean baseline.
- The originally drafted 67-character increment identifier was rejected before
  gate state changed. The semantically equivalent 56-character identifier
  `personal-assistant-v0-key-use-containment-classification` was then accepted.
- No operational or target-derived evidence was collected.

## Files expected to change

Exactly:

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
13. `docs/increments/personal-assistant-v0-key-use-containment-classification.md`
14. this plan
15. `docs/reviews/2026-09-02-personal-assistant-v0-key-use-containment-classification-post-increment-review.md`

No other path may change.

## Affected components

| Component                       | Effect                                                       |
| ------------------------------- | ------------------------------------------------------------ |
| Repository governance           | Records one bounded static candidate classification.         |
| Future signing architecture     | Separates private-key liveness from product code signing.    |
| D-101 identity boundary         | Remains independently required and operationally unresolved. |
| D-102 build containment         | Remains unchanged for every build-bearing path.              |
| Product runtime, IPC, and UI    | None.                                                        |
| Target Mac and external systems | None; every operational check remains Not run.               |

## Interfaces and invariants

### Frozen candidate identity

```text
in_process_security_framework_ephemeral_challenge_proof_v1
```

The conceptual candidate is one future foreground, explicit-owner-action,
single-attempt trusted-Rust operation. It accepts no caller input. It may act
only on one separately proven, application-owned, attempt-bound opaque
`SecIdentity` reference. It uses a fixed versioned domain separator, 32 fresh
bytes from the platform CSPRNG, one fixed algorithm, one signature, and one
verification against the retained identity's paired public key.

It creates no build graph, helper, child, subprocess, bundle, artifact, file,
socket, timestamp request, IPC contract, persistent reference, or generic
signing interface. Challenge, signature, certificate, key, attribute, and raw
error values remain inside the adapter and are released promptly without a
claim of guaranteed memory zeroization.

### Proof-class separation

```text
present_key_use_challenge_v1 != product_bundle_signing_proof_v1
```

At most, a later separately approved successful implementation could establish
that one already-bound opaque identity reference signed one application-owned
ephemeral challenge during one approved attempt and that its paired public key
verified the signature. It cannot establish code signing, product identity,
trust, revocation status, hardened runtime, Gatekeeper, notarization,
distribution, custody, or V0-3 readiness.

### Closed factual evidence

The exact `evidence_privacy_v1` outcome allowlist is:

```text
all_contracts_documented | contract_unproven | boundary_failed
```

The exact check identifier is
`in_process_key_use_candidate_classification`. The compact record contains only
the three D-100 keys. It carries no target-derived value and grants no authority.

```text
{"protocol_version":"evidence_privacy_v1","check_id":"in_process_key_use_candidate_classification","outcome":"contract_unproven"}
```

The separate governance result is exactly one of:

```text
eligible_only_for_present_use_planning | not_eligible_or_unproven
```

A positive result requires every contract row to be `documented`. Any absent,
ambiguous, unsupported, deprecated-without-supported-replacement, or
source-conflicting contract produces `contract_unproven`. Prohibited evidence
or an unapproved effect produces `boundary_failed`. There is no score,
compensating control, fallback, retry, candidate substitution, or automatic
successor.

### Contract matrix

Review each row exactly once:

1. `candidate_identity_contract`
2. `pinned_dependency_provenance_contract`
3. `safe_wrapper_surface_contract`
4. `opaque_prebound_identity_contract`
5. `exact_signer_binding_contract`
6. `account_keychain_scope_contract`
7. `private_key_nonexport_contract`
8. `fresh_challenge_contract`
9. `fixed_algorithm_contract`
10. `single_use_sign_contract`
11. `paired_public_key_verification_contract`
12. `interaction_denial_contract`
13. `hard_deadline_cancellation_contract`
14. `late_result_rejection_contract`
15. `cleanup_quarantine_contract`
16. `evidence_minimization_contract`
17. `platform_effect_contract`
18. `d102_applicability_split_contract`
19. `claim_ceiling_history_contract`

### Recorded dispositions

`documented`:

- `candidate_identity_contract`
- `pinned_dependency_provenance_contract`
- `safe_wrapper_surface_contract`
- `fresh_challenge_contract`
- `single_use_sign_contract`
- `paired_public_key_verification_contract`
- `evidence_minimization_contract`
- `claim_ceiling_history_contract`

`contract_unproven`:

- `opaque_prebound_identity_contract`
- `exact_signer_binding_contract`
- `account_keychain_scope_contract`
- `private_key_nonexport_contract`
- `fixed_algorithm_contract`
- `interaction_denial_contract`
- `hard_deadline_cancellation_contract`
- `late_result_rejection_contract`
- `cleanup_quarantine_contract`
- `platform_effect_contract`
- `d102_applicability_split_contract`

The authoritative bounded reasons are recorded in D-107. Totals are
`documented=8`, `contract_unproven=11`, `not_run=0`, and
`boundary_failed=0`.

## Frozen source register

Repository sources are the baseline `src-tauri/Cargo.toml`,
`src-tauri/Cargo.lock`, and the exact checksum-resolved
`security-framework` 3.7.0 `Cargo.toml`, `identity.rs`, `item.rs`, `key.rs`,
`random.rs`, and `os/macos/certificate.rs` plus `security-framework-sys` 2.17.0
`certificate.rs` and `key.rs`. Public platform sources are limited to these
current Apple Developer pages:

- Code Signing Services
- Randomization Services and `SecRandomCopyBytes`
- Identities, `kSecClassIdentity`, and Parsing an Identity
- `SecItemCopyMatching`, `kSecAttrAccessGroup`, and `kSecMatchSearchList`
- `kSecUseAuthenticationUI`, `kSecUseAuthenticationUISkip`,
  `kSecUseAuthenticationContext`, and `LAContext.interactionNotAllowed`
- `SecKeyIsAlgorithmSupported`
- `SecKeyCreateSignature`
- `SecCertificateCopyKey`
- `SecKeyVerifySignature`
- `SecKeyCopyExternalRepresentation`
- TN3137: On Mac keychain APIs and implementations

No target-derived source, forum answer, private documentation, runtime probe,
or post-freeze source substitution is allowed.

## Implementation milestones

- [x] Verify clean synchronized baseline and valid predecessor gate.
- [x] Create the approved branch and begin the exact documentation gate.
- [x] Freeze and review every source and contract row.
- [x] Record exactly one additive D-107 closed disposition.
- [x] Reconcile the exact documentation allowlist.
- [x] Run documentation validation and required completion reviews.
- [x] Record truthful final gate status and stop.

## Security and privacy considerations

Threats include ambient identity enumeration, label/fingerprint substitution,
an unintended signing oracle, authentication UI, raw private-key export,
signature replay, native-error or certificate metadata leakage, blocking work
continuing after cancellation, OS-managed cache/log/IPC/trust/network effects,
and semantic elevation from data signing to product signing.

The review fails closed. `SecIdentity`, `SecKey`, certificate, signature,
challenge, attribute, `CFError`, OSStatus, account, path, label, serial,
fingerprint, and Team ID values cannot enter evidence, logs, screenshots,
attachments, tests, or ordinary CI. The source review must explicitly prohibit
`external_representation`, `kSecReturnData`, persistent references, debug
formatting of identity/key objects, fallback, and retry.

## Test plan

- Verify candidate, source, check, outcome, and contract identities are exact.
- Verify each contract row receives exactly one closed disposition and totals
  match the decision, plan, increment, memory, and review.
- Verify D-097 through D-106 and every Failed/Pending/Not-run record remain
  intact.
- Verify the claim ceiling and D-102 non-waiver appear consistently.
- Verify the complete diff equals the fifteen documentation paths and every
  protected implementation/configuration path is unchanged.
- Record all operational, Keychain, signing, build, and target-Mac tests as
  Not run.

## Verification commands

Run after the last classification edit:

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

`npm run verify`, `npm audit`, every build, and every operational check remain
Not run because this increment changes documentation only.

## Risks

- In-memory data signing could be misrepresented as application code signing.
- A supposedly pre-bound identity could conceal ambient/default Keychain scope.
- Lookup-time UI suppression might be incorrectly extended to private-key use.
- A worker deadline could reject a result without stopping late private-key use.
- Existing safe wrappers expose private-key export and debug surfaces that a
  future implementation must prohibit.
- Removing the child process could be misrepresented as satisfying or waiving
  D-102 for product builds.

## Rollback or failure strategy

Before publication, use `apply_patch` only to reverse uncommitted changes in
the exact documentation allowlist. Never reset, clean, discard, or rewrite
historical evidence. Published correction requires a separately approved
additive revert or superseding decision.

If the source register or candidate changes, stop without fallback. If a
required validation fails, keep the gate active, report the failure, and do not
finalize, commit, publish, or begin a successor.

## Stop conditions

Stop or close negatively if:

1. identity acquisition requires ambient/default Keychain scope, enumeration,
   account/home/path resolution, or a target-derived label;
2. lookup or signing may prompt and interaction cannot be denied by a current
   supported contract;
3. private-key use lacks a bounded hard deadline/cancellation guarantee;
4. raw errors, identifiers, certificate values, challenge bytes, or signatures
   must cross the trusted adapter;
5. private-key bytes, export, persistent references, unsafe application Rust,
   a new dependency, entitlement, helper, file, subprocess, or network request
   becomes necessary;
6. late work, ownership, cleanup, or quarantine is ambiguous;
7. source absence is replaced by inference, observation, or residual-risk
   acceptance;
8. the result is elevated to code signing, product identity, custody, or V0-3;
9. D-102 or D-097 through D-106 would be weakened or rewritten; or
10. any changed path exceeds the documentation allowlist.

## Decisions made

D-107 records `not_eligible_or_unproven`. Eight rows are `documented`; eleven
are `contract_unproven`. No candidate or successor is admitted.

## Discoveries

- The safe pinned Rust crate exposes CSPRNG, identity-to-private-key,
  signature, public-key, and verification operations, but also exposes a
  private-key external-representation method that must remain prohibited.
- Apple documents Keychain lookup as blocking. Lookup interaction-suppression
  controls do not by themselves establish prompt-free private-key use or hard
  cancellation of a signing call.
- The longer drafted increment identifier exceeded the gate's 64-character
  maximum. Its rejection changed no gate state; the shorter identifier changes
  no candidate or scope.
- The pinned crate's default features are disabled by the repository. Its safe
  item-query builder therefore cannot establish the data-protection-keychain
  branch as the scope for the existing Developer ID identity.
- The frozen source record directly supports eight narrow primitive,
  provenance, evidence, and claim-ceiling rows. Eleven complete-boundary rows
  remain unproved, so the conjunctive result closes negatively.

## Progress

- 2026-09-02: Owner approved the exact documentation-only candidate and plan.
- 2026-09-02: Clean baseline and valid predecessor verified; approved branch
  created and the shortened exact gate identifier begun.
- 2026-09-02: The frozen static review completed with eight `documented` and
  eleven `contract_unproven` rows; D-107 selected
  `not_eligible_or_unproven` without operational action.

## Acceptance criteria

- [x] Exactly the frozen candidate and source register are reviewed.
- [x] All nineteen contract rows receive one closed disposition.
- [x] Exactly one non-authorizing D-107 result is recorded.
- [x] D-096 through D-106 and all historical evidence remain intact.
- [x] Operational work and target-Mac checks remain Not run.
- [x] Exact documentation scope and required validation pass.
- [x] No operational successor is marked Ready automatically.

## Final results

The frozen source review records `not_eligible_or_unproven`: eight rows are
`documented`, eleven are `contract_unproven`, and none are `not_run` or
`boundary_failed`. The candidate is materially distinct but is not admitted.
No operational action ran and no successor is Ready. Documentation validation
and the completion workflow passed with `PASS WITH ADVISORIES`; next-increment
readiness is `Blocked`.

## Documentation updates

- [x] `ARCHITECTURE.md`
- [x] `CHANGELOG.md`
- [x] `DECISIONS.md`
- [x] `HANDOFF.md`
- [x] `NEXT_STEPS.md`
- [x] `PLANS.md`
- [x] `PROJECT_STATUS.md`
- [x] `ROADMAP.md`
- [x] `SECURITY.md`
- [x] `SECURITY_CHECKLIST.md`
- [x] `TESTING_GUIDE.md`
- [x] `TROUBLESHOOTING_LOG.md`
- [x] Active increment and post-increment review
