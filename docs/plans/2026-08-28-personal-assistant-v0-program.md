# Personal Assistant v0 capability program

Status: Complete documentation-only program; no implementation authority
Owner: Henry Dang
Last updated: 2026-08-28
Baseline: `41ff7141007c8c0a684a5e2434ecf81dd4707418`
Decision authority: D-060 through D-093 plus D-094
Increment: `personal-assistant-v0-capability-planning`

## Goal

Define the smallest truthful, dependency-ordered path from the current
transport-free native core to one genuinely usable private Personal Assistant.
This program authorizes no source, dependency, credential, provisioning,
provider traffic, IPC, signing, external state, or personal-data transmission.

## First usable v0

The first usable v0 is Personal Assistant only: one foreground, explicitly
user-initiated text request at a time, bounded streamed plain text, one bounded
final answer, explicit terminal/idempotent cancellation, and closed redacted
errors. Trusted Rust owns:

- private run, gateway-request, presentation, and local support-correlation
  identities and exact validation/non-projection of gateway/provider identity;
- the sole `personal-assistant` agent, immutable instruction profile, selected
  provider/model profile, empty tool set, and every limit;
- request construction, one-request/single-flight state, deadlines,
  cancellation, transport cleanup quarantine, late-event rejection, and
  terminal outcome; and
- distinct Rust-only, one-use disclosure admissions required before the
  authentication-only socket and before synthetic model transport.

No caller selects a trusted agent, task, run, request, presentation, profile,
runtime, workflow, instruction, provider, model, endpoint, credential, data
class, tool, limit, retry, fallback, or outcome. An opaque presentation handle
is volatile correlation, not a bearer credential or authorization identity.

The v0 has no tools or function calls, files/attachments/clipboard, persistence,
memory, retrieval, scheduling, background autonomy, specialist delegation,
fallback, policy/approval dispatch, durable audit claim, filesystem/process
access, permission request, or device effect.

## Three milestones that must remain separate

| Milestone                              | Truthful capability                                                                                                                                                                          | Allowed data                                        | Status                |
| -------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- | --------------------- |
| 1. Live synthetic-text proof           | The signed target-Mac app sends one fixed application-owned fixture to OpenAI through the Access-protected Cloudflare Worker after explicit disclosure acknowledgment; bounded text returns. | Exact synthetic fixture only; no free text          | Blocked through V0-13 |
| 2. Private Personal Assistant          | One explicitly typed owner prompt follows a distinct real-content-v2 contract after real identity, provider/hosting, ZDR, logging/deletion, disclosure, operations, and target-Mac gates.    | Only a separately approved personal-text data class | Blocked at V0-14      |
| 3. Action-taking or production product | Tools, files, memory, durable audit, scheduling, background work, delegation, device effects, distribution, or production operations.                                                        | Not selected                                        | Blocked; no ExecPlan  |

Milestone 1 is a live transport proof, not a usable assistant. Milestone 2 is
the first genuinely usable capability, but its topology is not authorized.
Milestone 3 requires a new product/security/release program.

## Current-state evidence

- `NativeAgentRuntime` is the sole/default runtime and performs no I/O. Its
  current `InitialGatewayTurn` advertises `cortexa_desktop_mvp@1` with two fixed
  tool schemas; it cannot truthfully represent empty tools.
- `AgentRuntime::start` is the sole runtime start authority.
  `GatewayStreamValidator` already enforces closed protocol/version/identity,
  sequence, size/count, terminal cancellation, and late-event behavior; an
  empty allowed function-name set can reject every function call.
- There is no Personal Assistant host, direct Rust HTTPS client/provider
  adapter, live coordinator, Tauri contract, live conversation client, or live
  reducer. Transitive lockfile crates are not direct-client authority.
- Conversations is a deterministic frontend mock. Command Center, the mock,
  the sealed Research/Knowledge lifecycle, and Rust acceptance workflows are
  separate deterministic proofs and must stay separate.
- F-07 production/dev CSP separation, F-08 IPC narrowing, F-12 static
  UI/native enforcement, F-01 exact runtime identity, F-02 rejected-run
  quarantine, and F-15 documentation reconciliation remain mandatory.
- D-076/TS-017 blocks stable signed credential identity. No real Access token,
  Worker route, OpenAI secret, provider request, or external traffic exists.
- D-066/D-067 authorize OpenAI/Cloudflare only for the synthetic demo. D-068's
  service token is synthetic-only. D-061 and a new explicit provider/hosting
  and identity decision block real prompts.

## Closed limits

| Limit                                                         | Exact value                                |
| ------------------------------------------------------------- | ------------------------------------------ |
| Concurrent requests                                           | 1 process-wide                             |
| Future real user input                                        | 4,096 Unicode scalars / 16,384 UTF-8 bytes |
| Model turns / gateway requests / automatic retries / fallback | 1 / 1 / 0 / none                           |
| Tools / function calls / attachments                          | 0 / 0 / 0                                  |
| Normalized events                                             | 128                                        |
| One text delta                                                | 1,024 Unicode scalars / 4,096 UTF-8 bytes  |
| Complete assistant output                                     | 8,192 Unicode scalars / 32,768 UTF-8 bytes |
| Presentation/support correlation                              | 128 visible ASCII bytes                    |
| Connect / idle / provider / total deadline                    | 10 / 20 / 60 / 120 seconds                 |
| Rust journal / poll batch                                     | 128 / 16 entries                           |
| Work after terminal state                                     | 0                                          |

The existing 16,384-byte normalized-event and 65,536-byte request limits remain
outer ceilings. Every lower v0 limit is enforced first and cannot be silently
raised.

## Immutable profile and identity ownership

Synthetic version 1 is exactly:

- agent `personal-assistant`;
- instruction `personal-assistant-text-v0@1` with the literal text in V0-1;
- provider profile `openai-cloudflare-personal-assistant-v0@1`, OpenAI model
  `gpt-5.6-luna`, `store: false`, `background: false`, zero retry/fallback;
- data class `application-owned-synthetic@1` and the literal V0-1 fixture; and
- tool set `empty@1` with `tools: []` and zero function calls.

The gateway maps the instruction to the Responses `instructions` field and the
fixture to `input`; it never concatenates them. Rust issues local run/request
identity before the sole `AgentRuntime::start`, validates returned runtime
identity exactly, and quarantines construction or stream rejection. The
provider response ID is untrusted, validated, retained privately, and never
becomes local authority.

Real personal text may never widen synthetic version 1. V0-14 must decide and
freeze `personal-assistant-text-v0@2`, data class/disclosure version 2, and a
distinct request/command/parser/handle family only after its blockers resolve.

## Lifecycle, cancellation, and cleanup

The accepted graph is:

`Idle -> Starting -> Streaming -> Completed | Failed`; `Starting` or
`Streaming -> Cancelling -> Cancelled | Failed`; and `Starting` may also fail
or cancel terminally before `started`.

`Cancelling` accepts no content. Cancel closes local ingress first, aborts the
original desktop HTTP request/socket, and relies on downstream disconnect to
abort the one upstream provider request—never a second model/cancel request.
No `waitUntil`, detached work, retry, or fallback continues. Abort ambiguity or
cleanup failure retains ownership, rejects restart, and allows only bounded
trusted cleanup. Completed, failed, and cancelled runs reject every late event.
The guarantee is local terminal state, abort attempts at both hops, bounded
cleanup ownership, and late rejection—not proof that a remote provider stopped.

## Synthetic-v1 presentation boundary

V0-11 freezes exactly three commands:

- `start_personal_assistant_synthetic_v1` accepts only contract version 1 and
  exact disclosure acknowledgment; it accepts no text;
- `poll_personal_assistant_synthetic_v1` accepts a Rust handle and nullable
  cursor and returns at most 16 contiguous updates plus a snapshot; and
- `cancel_personal_assistant_synthetic_v1` accepts only the Rust handle and is
  idempotent.

Trusted Rust mints a private, one-use disclosure admission and transport cannot
write without it. TypeScript treats replies as `unknown`, validates a complete
immutable candidate before application retention/state commit, and rejects
unknown keys/types/literals, unsafe integers, Unicode/UTF-8 bounds, gaps,
regressions, illegal transitions, terminal conflicts, prefix rewrites, and
final mismatch.

Polling is one 250 ms `setTimeout` scheduled only after the previous promise
settles. Session/poll generations reject late promises. Route-away,
invisibility, cancel, terminal, and teardown stop polling; route-back performs
one snapshot-only recovery and never starts. WebView crash reattachment is not
claimed. No Tauri event/listener/emitter, raw invoke consumer, WebView fetch,
storage, capability, CSP, or permission expansion exists.

The panel displays the fixed prompt, unchecked acknowledgment, exact external-
processing disclosure, one chronological escaped-plain-text transcript,
partial output labelled Incomplete, coarse live status, alert failure, and
explicit recovery. It remains separately labelled `LIVE SYNTHETIC TEXT PROOF`
and does not replace the deterministic mock or its `DEMO MODE · SIMULATED
AGENT DATA` disclosure.

Before V0-9's earlier zero-body Access probe, the terminal displays the exact
`personal-assistant-access-auth-probe@1` disclosure and accepts only explicit
interactive `AUTHENTICATE`. Rust consumes a distinct one-use admission before
credentials or any socket. It cannot start model transport or substitute for
V0-11's `personal-assistant-synthetic-external-processing@1` acknowledgment.

## Dependency graph and independently approvable plans

This planning increment is complete. V0-1 is the sole Ready plan and every
later plan is Blocked; Ready is not implementation authority.

1. [V0-1 sealed empty-tool turn and minimal host](2026-08-28-personal-assistant-v0-empty-tool-turn.md) — **Ready** for separate owner approval; transport-free exact profile, Rust IDs, sole runtime start, process-wide lease.
2. [V0-2 volatile lifecycle and presentation journal](2026-08-28-personal-assistant-v0-session-host.md) — **Blocked** by V0-1.
3. [V0-3 fake signed-client/secret owner](2026-08-28-personal-assistant-v0-signed-client.md) — **Blocked** by D-076/TS-017.
4. [V0-4 local deny-only Access verifier](2026-08-28-personal-assistant-v0-gateway-admission.md) — **Blocked** by the V0-1 queue baseline and dependency-free design review; generated JWT/JWKS only.
5. [V0-5 Cloudflare no-traffic provisioning](2026-08-28-personal-assistant-v0-no-traffic-provisioning.md) — **Blocked** by V0-4 and external authority.
6. [V0-6 direct Rust HTTPS dependency decision](2026-08-28-personal-assistant-v0-https-dependency-decision.md) — **Blocked**; documentation decision only.
7. [V0-7 fixed-origin Rust transport](2026-08-28-personal-assistant-v0-synthetic-transport.md) — **Blocked** by V0-2/V0-3/V0-5/V0-6; fake I/O/credentials only.
8. [V0-8 real demo credential ingestion](2026-08-28-personal-assistant-v0-real-demo-credential-ingestion.md) — **Blocked** by V0-3/V0-5/V0-7 and exact credential authority.
9. [V0-9 Access authentication-only rehearsal](2026-08-28-personal-assistant-v0-access-auth-rehearsal.md) — **Blocked**; fixed zero-content probes, no provider.
10. [V0-10 OpenAI adapter with fake upstream](2026-08-28-personal-assistant-v0-synthetic-gateway.md) — **Blocked** by V0-9/current provider contract.
11. [V0-11 synthetic-v1 Tauri/presentation](2026-08-28-personal-assistant-v0-tauri-presentation.md) — **Blocked** by V0-2/V0-7/V0-10/F-12; must close before provider traffic.
12. [V0-12 no-traffic provider provisioning](2026-08-28-personal-assistant-v0-provider-no-traffic-provisioning.md) — **Blocked** by V0-10/V0-11 and external authority.
13. [V0-13 live synthetic Stage C rehearsal](2026-08-28-personal-assistant-v0-live-synthetic-rehearsal.md) — **Blocked** by V0-9..V0-12, a fresh credential window, and traffic approval.
14. [V0-14 private real-prompt admission decision](2026-08-28-personal-assistant-v0-real-prompt-activation.md) — **Blocked** by V0-13, D-061, real auth, and provider/hosting authority.

V0-3/V0-4/V0-6 are technically separable investigations, but repository queue
discipline permits only the approved next increment. External provisioning,
credentials, auth traffic, provider provisioning, provider traffic, and real
content each remain a separately approved boundary.

## Blocker disposition

| Boundary                                                            | Current disposition                              | Resolving increment or stop rule                                                                            |
| ------------------------------------------------------------------- | ------------------------------------------------ | ----------------------------------------------------------------------------------------------------------- |
| Immutable instructions, exact profile, empty tools, Rust identities | Designed, unimplemented                          | V0-1; stop on public/profile selector, second runtime start, dead-code allowance, or legacy byte drift      |
| Volatile journal/deadlines/cancel/late rejection                    | Designed, unimplemented                          | V0-2; V0-7 later owns active socket timers/cleanup                                                          |
| Signed client and secret owner                                      | Blocked                                          | V0-3; unsigned/prompt fallback is forbidden                                                                 |
| JWT/JWKS admission and exact live bindings                          | Designed, unimplemented                          | V0-4 source; V0-5 issuer/AUD/JWKS/origin; V0-8 expected Client ID; V0-9 real auth proof                     |
| Rust HTTPS                                                          | No direct client                                 | V0-6 decision then V0-7; no WebView fetch, subprocess, transitive/private API, unsafe TLS, or improvisation |
| Real credential handoff                                             | Blocked                                          | V0-8 only; direct owner transfer, rotation/revocation/removal required                                      |
| Provider mapping/limits                                             | Designed, unimplemented                          | V0-10 fake upstream, V0-12 disabled provisioning, V0-13 live synthetic                                      |
| Disclosure/Tauri/TypeScript/UI                                      | Designed, unimplemented                          | V0-7/V0-9 auth disclosure before first socket; V0-11 before provider transmission; F-12 review              |
| Provider ZDR/retention                                              | Synthetic evidence must be current; real blocked | V0-12/13 disclose synthetic controls; V0-14 requires exact D-061 evidence; `store: false` is not ZDR        |
| Real owner authentication and hosting/provider authority            | Unselected                                       | V0-14; D-068 cannot be extended and D-066/D-067 are synthetic-only                                          |
| Logs, kill, rollback, target Mac                                    | Unproved                                         | Runtime metadata <=7 days; synthetic admin audit separate; false traffic flag denies new admission only     |

V0-5 is the sole owner of the selected issuer/AUD/JWKS/origin; V0-8 is
the sole owner of the expected service-token Client ID binding; V0-12 is the
sole provider-secret owner. Every later gate compares the installed value to
its prior sanitized fingerprint and fails closed on drift. For the synthetic
lane, Access authentication-request and Worker/runtime metadata are D-061
gateway logging and must be content-free with retention at most seven days.
D-094 separately accepts the current 18-month mandatory Cloudflare admin-action
audit class only after field inspection proves no prompt/output/request body,
authorization value, credential secret, or provider content. That acceptance
does not carry into milestone 2.

The exact synthetic disclosure names current documented maximum retention:
OpenAI abuse-monitoring content up to 30 days, encrypted prompt-cache state up
to 24 hours, Cloudflare Free-plan Access authentication metadata for 24 hours,
and mandatory admin-action audit records for 18 months. It states
`store=false` is not ZDR. V0-12/V0-13 must reverify every value and require a
new disclosure version on drift.

## Local-model alternative

A verified fully local model could materially shorten the private-data path by
removing remote auth/gateway/provider retention, but there is no selected
engine/model, license/provenance, artifact/update/removal policy, target-Mac
hardware benchmark, dependency plan, cache/log boundary, or no-egress proof.
It is therefore not an evidenced shortcut and is not authorized.

The decision-impact ledger is exact:

- An additive local **milestone-2** lane would amend D-094 only to select the
  previously unselected real-content topology and would amend D-060/D-061 for
  local processing/data evidence. It would leave D-062 and D-064's remote
  identity/gateway direction and D-066/D-067/D-068's synthetic lane intact.
- A replacement that removes the remote synthetic proof would require a
  successor decision superseding only D-094's milestone-1 remote selection and
  D-066/D-067/D-068 for this v0. It would amend D-060/D-061/D-064 and would
  revisit D-062 and D-076 only if remote identity were also deferred. D-063 is
  historical and already superseded by D-066, so it is not superseded again.
- D-065's instruction hierarchy remains mandatory for any local lane.
  D-069..D-077 remain historical/deferred credential and signing evidence;
  they are not rewritten, though a replacement decision must explicitly retire
  any still-open remote-demo trigger while preserving the evidence.
- D-078's private-project scope remains. Its runtime-adapter placement and
  D-079's accepted multi-runtime architecture need amendment only if the local
  proposal adds a new runtime or moves transport authority into
  `NativeAgentRuntime`. D-080 would be revisited only if the selected engine is
  Hermes over its managed local WebSocket; D-081's Hermes ACP rejection remains
  unless separately superseded and cannot be bypassed by calling the engine
  local.
- D-082..D-093 remain intact: their application-owned native agent ownership,
  deterministic workflows, governance, volatile memory/document boundary,
  non-executing proposals, manual sealed dispatch, bounded specialists,
  frontend projection, and separate demonstration proofs gain no model,
  provider, transport, or execution authority from a local Personal Assistant.
- A direct `complete(prompt) -> String` surface also conflicts with D-032's
  streaming boundary. Any affected earlier decision requires a named successor
  decision; none of these impact statements is approval.

## Program manual gates and rollback

Milestone 1 requires exact signed-target-Mac, Keychain, no-traffic, JWT/TLS,
stream, cancellation at practical phases, deadlines, late rejection, bounds,
redaction, vendor-log inspection, disclosure, accessibility, kill, revocation,
and rollback evidence. The route is disabled and secrets/tokens revoked after
rehearsal unless a fresh bounded window is separately approved.

Milestone 2 additionally requires explicit real provider/hosting and owner-auth
decisions, exact approved ZDR and data-use/region/retention evidence, real-v2
disclosure/acknowledgment, input classification, incident response, log
retention/deletion, kill/revocation, target-Mac evidence, and separate
activation approval. Credentials, regulated/sensitive content, files,
clipboard, and automatic context remain denied.

Each increment rolls back only its exact files/state in its recorded order.
Stop on an unlisted file/action, caller-selected trusted value, raw content or
secret in evidence/logs, unbounded stream, automatic retry/fallback, detached
work, cleanup ambiguity, accepted late event, scope expansion, or external
action without exact approval.

## Documentation-tier validation

This planning increment runs only:

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

Target-Mac application, signing, Keychain, gateway/provider, traffic, Tauri/UI,
credential, and rollback checks are `Not run` for this documentation-only
increment.
