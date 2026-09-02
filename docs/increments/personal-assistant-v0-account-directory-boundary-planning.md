# Personal Assistant v0 account-directory boundary planning

Status: Complete — `PASS WITH ADVISORIES`
Owner: Henry Dang
Last updated: 2026-09-01
Decision: D-101

## Goal

Define the application-resolution prohibition and every independent proof gate
required before a future account-directory containment claim, without running
an operational check.

## Scope

- Add the linked ExecPlan and D-101.
- Require an operation-specific application-owned opaque wrapper over an
  adapter-private platform reference.
- Prohibit account/home/path inputs, environment selection, resolution APIs,
  subprocess/path fallback, and inferred owner acceptance.
- Require independent input, exact scope-provenance, directory/account-record,
  cache, log, socket, trust-service, process-metadata, and network predicates.
- Separate application-source guarantees from unproven OS-internal effects and
  keep explicit resolution/input/ambient scope non-waivable under D-101.
- Preserve the historical Pending `getpwuid`/`opendirectoryd` finding and
  prohibit rerun.
- Reconcile the exact current documentation state.

## Explicit non-goals

No account-directory query, Apple, Xcode, Keychain, Security.framework,
certificate, private-key, signing, build, target-Mac security/signing/product,
credential, provider, network, product, source, dependency, configuration,
branch, commit, push, merge, release, publication, or external-system work.

## Result

`PASS WITH ADVISORIES`. The exact documentation policy, proof gates, closed
review table, and historical-preservation rules passed the required reviews and
checks. This does not close operational P2, prove that an acceptable platform
contract exists, or establish that OS-internal effects are absent. Every
operational predicate is Not run; P3/P4 and every operational successor remain
Proposed/Blocked.
