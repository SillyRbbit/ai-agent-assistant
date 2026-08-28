# V0-8 — real demo credential ingestion and lifecycle

Status: Blocked by D-076/TS-017 and absent external credential authority
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: verified V0-3, V0-5, V0-7, stable signed identity, and fresh external approval

## Goal and outcome

Create one short-lived synthetic-demo Cloudflare Access service token, bind one
Service Auth policy to that token and application, transfer the two values
directly into the fixed macOS Keychain labels for the approved signed app, and
prove private read/consume/rotation/removal behavior while every route remains
disabled. No provider key or product/data-plane/provider request is created;
only the explicitly approved owner control-plane operations occur.

## Exact repository files

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-08-28-personal-assistant-v0-real-demo-credential-ingestion.md`
- `docs/increments/personal-assistant-v0-real-demo-credential-ingestion.md`
  (new)
- `docs/reviews/2026-08-28-personal-assistant-v0-real-demo-credential-ingestion-post-increment-review.md`
  (new)

No source, test, manifest, lockfile, entitlement, profile, capability, CSP,
permission, workflow, frontend, or gateway file may change. Repository evidence
contains only sanitized outcomes and lifetimes—never an identity, label value,
token value, Keychain record, policy/application/account identifier,
screenshot, shell command containing a value, or private evidence location.

## Exact external scope and invariants

- Under fresh owner approval, create exactly one service token with lifetime at
  most 30 days and one Service Auth allow policy restricted to that token and
  the V0-5 application. The Worker stays disabled and has no reachable route.
- Treat the Cloudflare service-token resource `id` and `client_id` as distinct
  values from the same token record. The Service Auth policy selector
  `service_token.token_id` uses the resource `id`; the Access application JWT
  `common_name`, the Worker secret binding
  `PA_V0_EXPECTED_ACCESS_CLIENT_ID`, and the Keychain `client-id` value use the
  distinct `client_id`. Never compare them for equality. Retain separate
  sanitized fingerprints proving that the policy selector and expected-client
  binding came from the same reviewed token record, without retaining either
  value.
- The only approved transfer surfaces are the authenticated Cloudflare
  Dashboard, the target Mac's Keychain Access UI, and—only under the controls
  below—the system pasteboard. With the app stopped and screen
  sharing/recording disabled, the owner creates two login-keychain
  generic-password items under service
  `io.cortexa.demo.cloudflare-access` and accounts `client-id` and
  `client-secret`. The owner moves one value at a time from the authenticated
  Cloudflare Dashboard into Keychain Access. In that same Dashboard, the owner
  selects the reviewed token record's resource `id` for the Service Auth policy
  and moves its distinct `client_id`, one value at a time, into the target
  Worker's `PA_V0_EXPECTED_ACCESS_CLIENT_ID` secret field. If the macOS
  pasteboard is used for either dashboard-to-dashboard or dashboard-to-Keychain
  transfer, Universal Clipboard/history tooling must be disabled, the
  pasteboard must be overwritten immediately with a public canary after each
  paste, and the source and target value fields must be closed after
  confirmation. No value may remain in browser history or an ordinary form
  field. Neither value may pass through source, chat, Codex, shell
  arguments/environment/history, an unapproved clipboard, screenshot, WebView,
  SQLite, file, log, CI, or test.
- Only the approved signed application identity may read the items. Interactive
  prompts, unsigned/debug identity substitution, broad ACLs, export, sync, or
  an environment fallback are failures.
- Cloudflare Dashboard and Keychain Access own their respective temporary UI
  buffers; the system pasteboard is the only conditionally approved transfer
  buffer and is cleared immediately. The owner is responsible for closing and
  clearing every control-plane/Keychain UI surface. After a Keychain read,
  Rust's sealed credential owner owns its private buffer until one transport
  consumption/drop. No guaranteed zeroization is claimed for UI, pasteboard,
  Rust, compiler, OS memory, swap, or crash dumps.
- Rust secret values remain non-cloneable, non-serializable, non-debuggable,
  one-consumption owners. Best-effort memory overwrite is not described as
  guaranteed zeroization.
- Secret rotation and token replacement are separate procedures. Rotating only
  the secret preserves the token resource `id`, `client_id`, policy selector,
  and expected-client binding; replace only the Keychain `client-secret` item,
  record sanitized control-plane evidence that the prior secret version was
  invalidated, and keep traffic disabled. No live denial is claimed here;
  V0-9 separately proves network authentication denial. Replacing the token
  means, while the route is absent and traffic is false, creating one new token
  record, updating the policy selector to its new resource `id`, updating the
  expected-client binding and Keychain `client-id` to its new `client_id`,
  updating the Keychain secret, proving the two separate sanitized fingerprints
  refer to that new record, and then revoking/deleting the old record. Any
  ambiguity invokes rollback; it never permits both records for traffic.
- Private target-Mac evidence proves stable authorized access across an exact
  rebuild/restart, unauthorized and unsigned denial, one-time consumption,
  both secret-only rotation and replacement-token behavior while the route and
  traffic flag remain off, exact overwrite of affected fixed Keychain items (or
  deletion/recreation if overwrite cannot be proved), policy-selector and
  expected-client binding replacement for a new token, old-token revocation in
  the control plane, and item removal returning to closed `missing`.
- Record the rotation owner, maximum lifetime, emergency revocation trigger,
  route-disable order, Keychain cleanup owner, and incident-response contact.

## Verification and manual gates

```bash
cargo test --manifest-path src-tauri/Cargo.toml --test cloudflare_access_credential_boundary
cargo build --manifest-path src-tauri/Cargo.toml --example cloudflare_access_keychain_probe
npm run verify
npm audit --audit-level=low
npm run security:scan
npm run repository:check
npm run docs:check
git diff --check
python3 .codex/hooks/session_end_gate.py
```

The private signed-app/Keychain/Cloudflare control-plane checks are required.
Network authentication, JWKS, provider, prompt transmission, Tauri, and UI
checks are `Not run` and cannot be inferred from successful credential access.

## Threats, rollback, and stop conditions

Primary threats are secret exposure during transfer/evidence, an unstable code
requirement, prompt-based access, broad Keychain ACL, forgotten token, live
route, or false revocation claims. Rollback order is set
`PA_V0_TRAFFIC_ENABLED=false`, keep/remove the route, revoke every created
token, remove the Service Auth policy and `PA_V0_EXPECTED_ACCESS_CLIENT_ID`
binding, delete both Keychain items through Keychain Access, overwrite any
approved pasteboard buffer, verify private missing/revoked state, and retain
cleanup ownership if any step is ambiguous.

Stop if D-076 is not additively reopened, TS-017 remains unresolved, the
signed identity changes, any value enters an unapproved surface, a route is
reachable, a prompt/fallback is needed, the old token cannot be revoked, the
items cannot be removed, or source/config changes become necessary.

## Readiness

**Blocked.** This is the separately approval-bound handoff required by D-069
and D-070. No current authorization permits creating, transferring, or reading
real credentials.
