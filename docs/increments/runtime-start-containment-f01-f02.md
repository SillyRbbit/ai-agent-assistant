# Runtime-start containment F-01/F-02

Status: Complete
Owner: Project owner
Last updated: 2026-08-26

## Goal

Make every existing runtime-start path validate exact application-owned
returned identity and retain cleanup ownership for rejected nonterminal runs.

## Scope

Generalize the existing D-091 exact-identity, duplicate-live, quarantine, and
cleanup-retry pattern across legacy root, child/continuation, and synthesis
starts. Add adversarial regression coverage across every sealed workflow
family.

## Explicit non-goals

No IPC, UI, CSP, capability, provider, model, network, credential, tool,
approval dispatch, persistence, filesystem, platform, dependency, concurrency,
workflow, or public runtime-interface expansion.

## Invariants

- The application-created request is the sole source of trusted runtime
  identity.
- Mismatched, stale, or duplicate identities never become active contexts.
- Rejected nonterminal runs remain quarantined until cleanup succeeds.
- Quarantine blocks all new and fallback starts.
- No rejected run is dropped nonterminal and no rejected late event is
  accepted.
- Existing sealed workflow projections remain deterministic.

## Verification

Focused legacy orchestration/workflow contracts, complete `npm run verify`,
security scan, diff check, independent architecture/security/code/debt review,
and the deterministic post-increment gate.

## Target-Mac checks

Target-Mac Rust lint and all-target tests are required. Rendered UI and device
effect checks are not applicable because no UI, IPC, platform, permission, or
effect changes.

## Security review and rollback

Review identity provenance, duplicate rejection, quarantine ownership, cleanup
retry, fallback blocking, error closure, event rejection, secrets/logging, and
the absence of new authority. Roll back only this bounded increment by normal
revert if it cannot preserve every established workflow contract.

## Final results

The universal start boundary and adversarial contracts are implemented. Focused
contracts, strict Clippy, 490 executed all-target Rust tests, complete repository
verification, independent review, documentation checks, and deterministic
finalization pass with one intentional ignored Hermes probe. The result is
`PASS WITH ADVISORIES` for volatile future external-runtime ownership and
blocked successor readiness.
