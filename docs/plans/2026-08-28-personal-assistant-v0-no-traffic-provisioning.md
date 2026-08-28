# V0-5 — Cloudflare no-traffic provisioning

Status: Blocked; external resource authority is absent
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: verified V0-4, D-064 Stage B approval, and a fresh exact external-action approval

## Goal and outcome

Privately provision the minimum disabled Cloudflare boundary needed to inspect
the real team issuer, Access application AUD, JWKS endpoint, routing controls,
and rollback before any credential or traffic exists. This is an external
operations increment with documentation evidence only; it changes no product
source or dependency.

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
- `docs/security/phase4-gateway-threat-model.md`
- `docs/plans/2026-08-28-personal-assistant-v0-no-traffic-provisioning.md`
- `docs/increments/personal-assistant-v0-no-traffic-provisioning.md` (new)
- `docs/reviews/2026-08-28-personal-assistant-v0-no-traffic-provisioning-post-increment-review.md`
  (new)

No source, test, manifest, lockfile, workflow, capability, CSP, permission,
runner, signing, credential, or frontend file may change. Repository evidence
contains only redacted Passed/Failed/Not run facts; account, zone, application,
AUD, team-domain, route, and evidence-store identifiers remain private.

## Exact external scope

Under a fresh owner approval, create exactly one private demo Worker from the
verified V0-4 artifact and one Access self-hosted application with:

- `workers_dev = false`, no custom route/DNS mapping, and
  `PA_V0_TRAFFIC_ENABLED=false`;
- no Service Auth allow policy, service token, OpenAI key, provider binding,
  storage product, queue, analytics sink, Logpush, R2, or durable object;
- an application-token lifetime configured to at most 15 minutes; and
- an owner-selected fixed HTTPS application origin and path, reserved but not
  mapped or reachable; the hostname/path are trusted application configuration,
  not a secret, and V0-7 may pin only this exact value in signed Rust; and
- installation of the exact issuer, one AUD, and sole fixed HTTPS JWKS endpoint
  into the V0-4 bindings `PA_V0_ACCESS_ISSUER`, `PA_V0_ACCESS_AUD`, and
  `PA_V0_ACCESS_JWKS_URL`. `PA_V0_EXPECTED_ACCESS_CLIENT_ID` remains absent
  until V0-8, so admission still denies even if the traffic flag drifts.

The owner inspects Cloudflare control-plane configuration without sending an
HTTP request. A no-traffic gate cannot prove JWT acceptance, missing/wrong
token denial, revocation, or live JWKS rotation; those belong only to V0-9.

## Threats, manual gates, and evidence

- Confirm no default hostname or route is reachable and no preview/development
  URL is treated as approved evidence.
- Confirm the Worker is disabled before any authentication path and that the
  deployed source hash matches the reviewed artifact.
- Confirm Access runtime logs and Worker/runtime metadata are separately
  classified from mandatory account/admin control-plane audit logs. Record
  current fields, retention, deletion limits, and access ownership. Any runtime
  metadata retention above seven days, or any content/secret in either class,
  blocks Stage C. D-094 accepts the current content-free 18-month Cloudflare
  admin-action audit class for the synthetic lane only; it is never described
  as no logging or as gateway-runtime logging.
- Confirm least-privilege operator access, emergency disable order, resource
  ownership, and deletion authority before creation.
- Confirm control-plane inspection shows zero provider secrets, zero service
  tokens, zero allow policies, and zero traffic.

## Verification

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Manual Cloudflare checks are required and recorded only as sanitized outcomes.
Target-Mac, provider, credential, request, Tauri, and UI checks are `Not run`.

## Rollback

Keep the Worker disabled, confirm no route, delete the Access application and
Worker, and privately verify their absence. If deletion cannot be proved,
leave the program blocked and retain cleanup ownership; never continue to a
token or traffic step.

## Stop conditions

Stop on absent explicit external authority, a reachable hostname/route, an
automatic preview request, disagreement between the reserved origin and the
three installed verifier bindings, an unexpected resource/policy/secret/log
sink, a credential/private account identifier entering repository evidence,
runtime retention above seven days, control-plane logs containing content or
secret values, inability to disable/delete, or any request to create
credentials or provider state.

## Readiness

**Blocked.** V0-4 is unimplemented and no Stage B/external-action approval has
been granted. This plan itself grants no provisioning authority.
