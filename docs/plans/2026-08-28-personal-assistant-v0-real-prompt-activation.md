# V0-14 — private real-prompt admission decision and follow-on planning gate

Status: Blocked; real-content identity, provider/hosting authority, and D-061 evidence are absent
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: accepted V0-13 evidence and fresh owner decisions for every real-content boundary

## Goal and outcome

Decide whether Cortexa may move from the live fixed synthetic proof to one
foreground private Personal Assistant request containing owner-entered text.
If and only if every gate passes, freeze a distinct real-content-v2
architecture and record the exact follow-on planning work that a later,
separately approved documentation increment must derive. This increment is
decision-only and cannot create those ExecPlans, activate, transmit, or
implement real prompts.

## Exact files

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PRODUCT_REQUIREMENTS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/PROJECT_DIRECTION.md`
- `docs/security/phase4-gateway-threat-model.md`
- `docs/plans/2026-08-28-personal-assistant-v0-real-prompt-activation.md`
- `docs/increments/personal-assistant-v0-real-prompt-activation.md` (new)
- `docs/reviews/2026-08-28-personal-assistant-v0-real-prompt-activation-post-increment-review.md`
  (new)

No source/test, dependency/lockfile, credential, Keychain/signing state,
Cloudflare/OpenAI resource, route, provider request, Tauri/frontend, capability,
CSP, permission, workflow, release, or external system may change.

## Blocking decision matrix

Every item requires exact current primary evidence and an explicit accepted
outcome; one unresolved item keeps milestone 2 Blocked:

1. **Identity/authentication.** D-068's service token is synthetic-only and
   cannot carry personal prompts. Decide and evidence the real owner
   authentication path. If D-062 Microsoft personal identity remains selected,
   resolve its registration, issuer/audience/scope, PKCE/nonce/state/callback,
   maximum token lifetime, gateway validation, revocation, and target-Mac
   account UX. Do not silently extend the signed demo token.
2. **Provider and hosting authority.** D-066/D-067 select OpenAI/Cloudflare only
   for the synthetic demo. Accept an additive real-content decision or select a
   different architecture. D-094 does not itself extend those decisions.
3. **External processing.** Satisfy D-061 with provider-approved **Zero Data
   Retention** for the exact organization, project, endpoint, model,
   region/residency, feature eligibility, and evidence date. Separately record
   abuse-monitoring, prompt-cache, training/data-use, and any Modified Abuse
   Monitoring state, but MAM cannot substitute for ZDR and `store: false` alone
   is insufficient.
4. **Logging/deletion.** Classify Access, Worker, OpenAI, and control-plane
   metadata; name the exact sinks, access owners, retention/deletion mechanisms,
   support process, incident response, and prove prompt/output exclusion.
   Runtime gateway metadata remains capped at seven days. Cloudflare's
   synthetic-lane control-plane audit classification/acceptance does not
   automatically authorize the real-content lane; the owner must explicitly
   accept current content-free admin-action fields/retention or the conflict
   blocks.
5. **Operations.** Freeze one-user concurrency, hard budget/rate limits,
   deadlines, original-request abort, downstream-disconnect upstream abort,
   cleanup quarantine, late-event rejection, disable/kill order, credential
   rotation/revocation, rollback, and target-Mac validation.
6. **Disclosure/data class.** Approve exact real-content copy and version,
   persistent Settings visibility, explicit acknowledgment before first
   transmission and after material change, supported/prohibited data classes,
   and recovery copy. No sensitive data test is permitted.

## Required real-content-v2 boundary if admitted

The decision must freeze a new `personal-assistant-text-v0@2` instruction and
data-class profile; version 1 remains synthetic forever. Rust accepts exactly
one nonempty owner text value bounded to 4,096 Unicode scalars and 16,384 UTF-8
bytes, after Tauri deserialization but before application retention/state
commit. It issues run/request/presentation/support correlation, selects agent,
instruction, provider/model, empty tools, limits, one request, zero retry, and
no fallback. The gateway validates the returned-runtime/profile identity
exactly and quarantines any rejected run.

Use distinct commands and DTOs:

- `start_personal_assistant_real_v2` with only `contractVersion: 2`, bounded
  `text`, and the separately versioned real-content disclosure acknowledgment;
- `poll_personal_assistant_real_v2`; and
- `cancel_personal_assistant_real_v2`.

Synthetic-v1 and real-v2 handles, commands, parsers, data classes, disclosure
versions, gateway admission profiles, and static F-12 entries reject each
other. Shared private primitive validators may be reused; no wire DTO is
widened. Chronology/accessibility remains V0-11 exact: one user entry, one
assistant entry updated in place, escaped plain text, partial output labelled
Incomplete, coarse live status, alert failure, explicit recovery, and terminal
focus. One request is never retried/resubmitted automatically.

## Derived plans and exact stop rule

Only after the topology decision is accepted may a later, separately owner-
approved documentation-planning increment add exact plans for: real owner
authentication; sealed Rust real-text profile; gateway real-content adapter;
real-v2 Tauri/UI contract; no-traffic real provisioning; and target-Mac
activation/rollback. Each future plan must have its own exact files, threats,
tests, manual gate, rollback, and stop conditions. V0-14 creates none of them
and marks no source or operations increment Ready.

It is intentionally unsafe to predeclare exact production source files before
identity, provider/hosting, and data-control choices are made. If the accepted
topology differs from OpenAI-through-Cloudflare, stop and derive a new program
rather than editing the synthetic lane into a generic transport.

## Local-model alternative

A local model could materially shorten the private-data path only if a verified
engine/model runs fully on the target Mac with no egress, acceptable license
and model-origin evidence, bounded memory/latency/output, packaging/update and
removal controls, and no new tool/device authority. No such engine, model,
artifact, dependency, hardware benchmark, no-egress proof, or maintenance plan
exists, so it is not currently the shorter evidenced path and is not selected.

- An **additive local milestone-2 lane** would amend D-094 only to select its
  currently unselected real-content topology, plus D-060/D-061 for local
  processing/data policy. It leaves D-062/D-064's remote direction and
  D-066/D-067/D-068's synthetic proof intact.
- A **replacement that removes milestone 1** would require a successor decision
  superseding D-094's synthetic remote selection and D-066/D-067/D-068 for
  this v0, amending D-060/D-061/D-064, and revisiting D-062/D-076 only if remote
  identity is deferred. D-063 is historical and already superseded by D-066.
- D-065's instruction hierarchy remains mandatory. D-069..D-077 remain
  historical/deferred credential and signing evidence; a replacement must name
  and retire any still-open remote trigger without rewriting that evidence.
- D-078's private-project scope remains. D-078's adapter placement and D-079
  need amendment only if a new runtime is added or transport authority moves
  into Native. D-080 is revisited only for managed local Hermes WebSocket;
  D-081's Hermes ACP rejection remains unless separately superseded.
- D-082..D-093 remain intact and gain no provider, runtime, workflow,
  governance, memory, proposal, dispatch, specialist, frontend, or execution
  authority from a local Personal Assistant.
- A direct `complete(prompt) -> String` engine surface conflicts with D-032's
  streaming boundary. None of these decision impacts or alternatives is
  authorized here.

## Verification, rollback, and readiness

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

All real authentication, ZDR, credential, network, prompt, Tauri, UI,
target-Mac, and external checks are `Not run`. Rollback reverts the exact
documentation decision/derived plans. Stop on stale evidence, ambiguous
authority, a synthetic-contract widening, service-token reuse, unsupported data
class, logging/deletion conflict, absent kill/revocation owner, source/external
action, or pressure to infer a Ready implementation.

**Blocked.** Milestone 2 has no authorized authentication or provider/hosting
topology and lacks D-061 evidence. Milestone 3—tools, actions, persistence,
background autonomy, delegation, production distribution—is outside this
program and requires new product/security decisions.
