# Personal Assistant V0 HTTPS dependency decision

Status: Complete (`PASS WITH ADVISORIES`) — D-118 accepted; no client selected
Owner: Project owner
Date: 2026-09-03
Baseline: `6c3615c507b58a771f696ebb7bfdb5bd13d1d1b7`
Branch: `codex/personal-assistant-v0-https-dependency-decision`

## Objective

Evaluate exactly five frozen variants across three direct Rust HTTPS client
families against the approved conjunctive V0-6 evidence matrix, record one
closed documentation disposition, and create no executable transport or new
authority.

## Scope and invariants

- D-118 records exactly `no_eligible_client`; no dependency or transport is
  selected.
- Each frozen variant has at least one primary-source-documented failed
  mandatory criterion. Missing runtime evidence is not promoted to proof.
- A returned timeout, dropped future, or late-result filter does not prove
  abort, cleanup, complete ownership accounting, or quiescence for started
  blocking DNS work.
- The result is scoped to the frozen candidates and current hard cancellation
  contract. It is not a universal Rust HTTPS impossibility claim.
- V0-3 remains paused and `Blocked`. V0-7 remains `Blocked`, including its
  separate fake-only versus hermetic actual-client TLS/socket-test discrepancy.
- D-107 remains historical 8/11, D-108 remains additively 9/10, proposed D-113
  through D-117 remain non-controlling, and all ten D-107 blockers remain.

## Exact files

Exactly the fifteen documentation paths enumerated in the approved ExecPlan
may change. No source, test source, dependency, manifest, lockfile,
configuration, workflow, hook, capability, permission, signing state,
toolchain, generated resolver output, or external state is in scope.

## Evidence boundary

The owner-authorized evidence phase retrieved only unauthenticated frozen
public primary-source, registry, license, and advisory material and resolved
hypothetical graphs in separately isolated disposable Cargo roots. No candidate
was built, loaded, or executed; no build script ran; no resolver artifact was
copied into the repository; and the validated scratch roots were removed.

Five candidate variants were evaluated:

1. reqwest 0.13.4 with platform verification;
2. reqwest 0.13.4 with static WebPKI roots;
3. reqwest 0.13.4 with native TLS;
4. explicit Hyper 1.11.1/Tokio 1.53.1/Rustls 0.23.43/WebPKI; and
5. ureq 3.4.0 with Rustls/WebPKI.

All five fail the DNS/connection ownership and hard cleanup criteria. Reqwest
also fails exact pre-retention response header/read-buffer bounding. The frozen
RustSec comparison reproduced the baseline's 20 accepted entries and found no
candidate-added advisory; this was not a `cargo-audit` run.

## Prohibited work

No source or dependency edit, candidate build or runtime, credential,
certificate, Keychain, private-key, signing, Apple/Xcode, provider, gateway,
product-system, target-Mac transport, external traffic, commit, push, merge,
publication, or successor start. D-118 grants none of that authority.

## Progress

- [x] Established the exact synchronized baseline and began the approved gate.
- [x] Acquired the approved frozen evidence and isolated hypothetical graphs.
- [x] Completed the five-variant matrix and independent interim review.
- [x] Stopped for and received explicit owner acceptance of
      `no_eligible_client` and the exact D-118 wording.
- [x] Reconciled exactly the fifteen authorized documentation paths.
- [x] Ran documentation, repository, security, complete verification,
      exact-scope, independent-review, session, and completion gates.
- [x] Recorded a valid completion marker and stopped for owner review.

## Result

The exact fifteen-path documentation increment passes every required check
with advisories. D-118 is accepted and controls only the V0-6 negative
dependency decision. No client, dependency, or transport was selected, and
operational and next-increment readiness remains `Blocked`.
