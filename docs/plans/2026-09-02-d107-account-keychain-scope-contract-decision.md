# D-107 account and Keychain scope contract decision

Status: Complete (`PASS WITH ADVISORIES`)
Owner: Project owner
Last updated: 2026-09-02
Baseline: `b5fe79a424ecc3450054cc80eb89003294942326`
Predecessor: D-110 (`d107-exact-signer-binding-contract-decision`)

## Goal

Perform one repository-only, documentation-only review of D-107's
`account_keychain_scope_contract`. Decide only whether accepted repository
records establish a future application-owned signing-identity scope without
account, home, path, default-Keychain, search-list, environment, profile,
runtime, caller, or fallback authority.

The decision must fail closed. It cannot inspect Keychain state or infer scope
from fixed labels, an opaque identity, access-group names, or a disabled feature.

## User-visible outcome

None. This is planning only. It does not access a Keychain, certificate,
private key, account, filesystem, signing operation, build, or external system.

## Scope

1. Re-read D-096 through D-110, D-101, and current credential/signing-adjacent
   repository source only.
2. Define a future account-and-Keychain-scope contract without adding a query,
   configuration input, account/path resolution, persistence record, or runtime
   interface.
3. Define exactly three closed governance dispositions:
   `scope_contract_documented`, `scope_contract_not_accepted`, and
   `boundary_failed`.
4. Preserve D-097; D-107's immutable 8/11 factual record; D-108's 9/10
   interpretation; and D-109/D-110's negative results.
5. Keep readiness Blocked regardless of outcome.

## Explicit non-goals

- No product, source, dependency, lockfile, configuration, capability, CSP,
  permission, entitlement, workflow, hook, script, test, or toolchain change.
- No Keychain/Security framework, account, directory, home, path, filesystem,
  certificate, private-key, signing, Apple/Xcode, build, target-Mac, provider,
  network, credential, product, or external-system operation.
- No explicit or implicit account lookup; no UID/eUID, session, environment,
  default/search-list, current-directory, or path derivation.
- No caller-selected identity, Keychain, access group, label, fingerprint,
  certificate, account, profile, runtime, task, run, workflow, or fallback.
- No claim that an access-group API, disabled data-protection Keychain feature,
  fixed credential label, or absent visible prompt proves scope or OS effects.
- No branch, gate, commit, push, merge, release, or publication.

## Existing behavior and constraints

- D-107 records this contract as `contract_unproven`: file-based queries use
  ambient search-list/default behavior, while access-group scope is unproved for
  the existing Developer ID identity and the data-protection-Keychain feature is
  disabled.
- D-101 prohibits account/home/path resolution and requires any future native
  reference to be adapter-private, attempt-bound, non-serializable, and
  independently reviewed.
- D-109 supplies no identity issuer; D-110 supplies no immutable expected
  signer binding. Neither can be repaired by ambient Keychain scope.

## Current-state evidence

- Clean synchronized `main`, `HEAD`, and `origin/main` resolved to
  `b5fe79a424ecc3450054cc80eb89003294942326` when drafted.
- D-110's gate is valid and complete with `PASS WITH ADVISORIES`.
- Repository records identify ambient file-based lookup and unproved
  access-group/feature scope; no current application-owned scope contract
  exists.

## Files expected to change

The exact documentation-only 15-path inventory is this plan, one increment record, one review,
`DECISIONS.md`, and applicable current-state memory documents
(`ARCHITECTURE.md`, `CHANGELOG.md`, `HANDOFF.md`, `NEXT_STEPS.md`,
`PLANS.md`, `PROJECT_STATUS.md`, `ROADMAP.md`, `SECURITY.md`,
`SECURITY_CHECKLIST.md`, `TESTING_GUIDE.md`, and
`TROUBLESHOOTING_LOG.md`). Any other path stops the increment.

## Interfaces and invariants

`AccountKeychainScopePolicyV1` is conceptual governance documentation, not an
implemented Rust/Tauri/WebView API. A future positive design requires
application-owned fixed scope provenance before an attempt, a private adapter
that accepts no scope selector, no explicit/ambient account or path derivation,
no default/search-list or fallback behavior, no serialization/exposure, and
separate proof for every remaining D-107 contract.

The three dispositions are not D-100 evidence or runtime values. Missing,
ambiguous, contradictory, disabled, or drifted scope facts select
`boundary_failed`; absence of a complete repository contract selects
`scope_contract_not_accepted`. Neither result admits a candidate.

## Implementation milestones

- [x] Obtain owner approval for a dedicated `codex/` branch and gate.
- [x] Reconcile repository-only facts against D-096 through D-110.
- [x] Record `scope_contract_not_accepted` with no account, Keychain, or signer data.
- [x] Run documentation validation and completion workflow.
- [x] Stop with readiness Blocked.

## Security and privacy considerations

The threat is ambient-scope laundering: treating a default/search list,
file-based Keychain query, access-group label, disabled feature, or account-path
derivation as application-owned identity scope. Risks include Open Directory
effects, account metadata exposure, private identity selection, fallback, and
OS-managed cache/log/IPC/network effects. This plan remains repository-only and
keeps all such effects unproved.

## Test plan

- Compare the decision with D-101's prohibited inputs and D-107's precise scope
  reason.
- Confirm current credential code, fixed labels, access-group terminology, and
  disabled feature paths are not treated as scope proof.
- Confirm D-097 through D-110 stay unchanged and no private/target-derived
  value enters the diff.
- Confirm missing facts fail closed and readiness remains Blocked.

## Verification commands

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

No product verification, audit, build, Keychain, certificate, signing,
Apple/Xcode, target-Mac, provider, or external command is authorized.

## Risks

Repository evidence may be insufficient; record a closed negative or failed
result rather than adding external research. Treating policy vocabulary as a
live scope interface, or broadening into account/Keychain operations, is a stop
condition.

## Rollback or failure strategy

Before publication, remove this uncommitted plan only with owner direction.
After a future approved gate begins, preserve all evidence and record a closed
negative or failed result. Never rewrite D-097 through D-110 or invoke an
operational fallback.

## Progress

- 2026-09-02: Drafted from clean synchronized `main`. No branch, gate,
  source change, external operation, or readiness promotion occurred.

## Acceptance criteria

- [ ] Owner approves the later documentation increment before branch/gate.
- [ ] Exactly one closed scope disposition is selected.
- [ ] Historical evidence and remaining blockers are preserved.
- [ ] No account, Keychain, signer, operational, or external data/action occurs.
- [ ] No operational successor becomes Ready.

## Final results

Completed as a documentation-only negative decision: the account/Keychain scope
contract is not accepted and no successor is Ready.
