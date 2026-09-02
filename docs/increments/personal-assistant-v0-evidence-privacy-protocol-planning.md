# Personal Assistant v0 evidence-privacy protocol planning

Status: Complete — `PASS WITH ADVISORIES`
Owner: Henry Dang
Last updated: 2026-09-01
Decision: D-100

## Goal

Define a closed privacy-safe evidence protocol for separately approved future
work without collecting evidence or changing any product or external system.

## Scope

- Add the linked ExecPlan and D-100.
- Permit only plan-owned check IDs and closed categorical outcomes.
- Require source minimization and prohibit screenshots, recordings,
  transcripts, raw output, free text, target-derived sensitive identifiers,
  metadata, private paths, credentials, and content.
- Require bounded exact ASCII literals, one canonical record, non-authorizing
  predicate semantics, and private once-only plan/check/attempt binding for any
  future consumer.
- Define stop-without-retry handling for an evidence-boundary failure.
- Reconcile the exact current documentation state.

## Explicit non-goals

No Apple, Xcode, Keychain, certificate, private-key, signing, build, target-Mac
security/signing/product operation, credential, provider, network, product,
source, dependency, configuration, branch, commit, push, merge, release,
publication, or external-system work.

## Result

The closed documentation protocol passed architecture, security, code-health,
technical-debt, readiness, formatting/link, repository-health, secret-scan,
protected-path, whitespace, session-end, and post-increment review. It creates
no operational evidence or authority; no parser/sanitizer exists, and P2–P4
plus every operational successor remain Proposed/Blocked.
