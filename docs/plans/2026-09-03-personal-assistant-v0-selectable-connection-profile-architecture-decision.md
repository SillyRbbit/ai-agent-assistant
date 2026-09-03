# Personal Assistant V0 selectable connection-profile architecture decision

Status: Complete (`PASS WITH ADVISORIES`) — documentation-only; no product authority
Readiness: Blocked for every operational successor
Owner: Project owner
Last updated: 2026-09-03
Planning baseline: `01dbb1fdce10c197033c1a88dbeeb53afb0a21cd`
Branch: `codex/personal-assistant-v0-selectable-connection-profile-architecture-decision`
Gate ID: `pa-v0-selectable-connection-profile-decision`
Decision: D-119; `closed_catalog_direction_selected`

## Goal

Make one closed documentation decision about a future Rust-owned catalog of
selectable Personal Assistant connection profiles. The catalog direction must
cover local/no-auth and the exact cloud provider/authentication families named
below without making any profile operational, selectable, credential-bearing,
or network-capable in the current product.

This planning work is sequenced from the Personal Assistant V0 program, but
D-094 already reserves both the sealed synthetic-v1 contract and the future
`real-content-v2` contract as nonselectable. The only conflict-free target is a
distinct, post-v0 `personal-assistant-selectable-connection-profile-v3`
contract. A positive D-119 may narrowly supersede D-094 only to reserve that
post-v0 selector direction; synthetic-v1 and `real-content-v2` remain unchanged,
unimplemented where applicable, and governed by D-094.

The decision must keep provider identity, authentication method, model policy,
endpoint policy, credential ownership, disclosure, limits, cancellation, and
late-result handling inseparable inside trusted Rust. A future user selection
may express intent only through a Rust-issued opaque option handle; it may not
supply or override any trusted provider, auth, model, endpoint, account,
profile, runtime, agent, task, workflow, or run identity.

## User-visible outcome

None. This is a documentation-only architecture decision. It creates no
provider picker, authentication flow, model connection, local model, network
transport, credential store, or runtime behavior.

If the owner later accepts the positive documentation disposition, the result
will establish only a closed architecture direction for a future versioned
profile catalog. Every catalog entry remains `candidate_blocked` until its own
complete evidence and separately approved implementation sequence exists.

## Readiness decision

`Ready` applies only to the bounded documentation increment described here.
The repository is not Ready to implement or expose any listed profile:

- D-118 remains `no_eligible_client`, so no approved direct Rust HTTPS client
  or cloud transport exists.
- V0-3, V0-7, and all operational successors remain Blocked.
- The current Personal Assistant synthetic profile remains fixed,
  application-owned, transport-free, and not caller-selectable.
- Direct desktop provider-credential custody would conflict with D-060 and,
  for the current OpenAI/gateway contract, D-021. Any such design needs
  separately accepted reconciliation of every applicable controlling decision
  before implementation.
- Local/no-auth still lacks an accepted engine, model, license, provenance,
  artifact, update/removal, no-egress, logging, benchmark, and dependency
  decision.
- D-107's ten unresolved operational signing/credential blockers and the
  Proposed/non-controlling status of D-113 through D-117 remain unchanged.

## Current-state evidence

- After a read-only `git fetch --prune origin`, `HEAD`, local `main`, and
  `origin/main` all resolved to
  `01dbb1fdce10c197033c1a88dbeeb53afb0a21cd`; ahead/behind was `0/0` and the
  worktree was clean.
- Before this increment began, the ignored post-increment gate remained
  `complete` for `personal-assistant-v0-pr108-publication-closeout` but reported
  `valid: false` solely because this new untracked plan changed the workspace
  fingerprint. No finalizer was run and the historical increment was not
  retargeted. The owner explicitly approved the plan-only identifier correction
  and beginning this separately admitted increment. During execution, branch
  `codex/personal-assistant-v0-selectable-connection-profile-architecture-decision`
  held the `pa-v0-selectable-connection-profile-decision` gate. That gate is
  now `complete` with a valid report- and workspace-bound marker. The earlier
  73-character proposed ID was rejected by the hook's 64-character limit before
  state changed.
- The planning environment used Node `26.3.0`, npm `11.16.0`, rustc `1.90.0`,
  and cargo `1.90.0` on macOS `26.6` (`25G72`, arm64), matching the repository
  pins and constraints.
- `RuntimeTurnProfile` is private and currently exposes only `Initial` and
  `PersonalAssistantV0Synthetic`. The synthetic constructor supplies the
  application-owned fixture; the caller cannot choose provider, model,
  instructions, tools, endpoint, or auth.
- The Personal Assistant host is volatile, issues its own run and request
  identities, validates returned runtime identity, and has no production
  response ingress.
- `NativeAgentRuntime` remains the sole/default runtime. Provider selection is
  not runtime selection.
- The Tauri surface exposes no Personal Assistant route, and the repository has
  no approved Personal Assistant TypeScript client or provider picker.
- `src-tauri/Cargo.toml` has no direct HTTP/TLS or provider client. D-118
  accepted `no_eligible_client` and keeps V0-7 Blocked.
- Baseline `npm run docs:check`, `npm run repository:check`,
  `npm run security:scan`, and `git diff --check` passed before this plan was
  added.

## Scope

### This planning task

Create or revise exactly this one file:

1. `docs/plans/2026-09-03-personal-assistant-v0-selectable-connection-profile-architecture-decision.md`

Do not create a branch or begin a gate during this planning task.

### Separately approved decision increment

If the owner approves this exact plan, the documentation-only decision
increment may change exactly these sixteen documentation paths:

1. `ARCHITECTURE.md`
2. `CHANGELOG.md`
3. `DECISIONS.md`
4. `HANDOFF.md`
5. `NEXT_STEPS.md`
6. `PLANS.md`
7. `PRODUCT_REQUIREMENTS.md`
8. `PROJECT_STATUS.md`
9. `ROADMAP.md`
10. `SECURITY.md`
11. `SECURITY_CHECKLIST.md`
12. `TESTING_GUIDE.md`
13. `docs/PROJECT_DIRECTION.md`
14. `docs/plans/2026-09-03-personal-assistant-v0-selectable-connection-profile-architecture-decision.md`
15. `docs/increments/pa-v0-selectable-connection-profile-decision.md`
16. `docs/reviews/2026-09-03-pa-v0-selectable-connection-profile-decision-post-increment-review.md`

`TROUBLESHOOTING_LOG.md` is excluded unless a separately approved amendment is
needed for a newly observed troubleshooting fact. Any other changed or new
path is scope drift and stops the increment.

## Explicit non-goals

Neither this plan nor the proposed documentation increment may:

- change Rust, TypeScript, test, build, workflow, hook, capability, CSP,
  entitlement, permission, dependency, manifest, lockfile, toolchain, runner,
  signing, or product configuration;
- add a provider picker, profile DTO, Tauri command/event, network client,
  provider SDK, local inference engine, model artifact, authentication flow,
  credential reader/writer, persistence, filesystem access, background task,
  tool, agent delegation, fallback, or device effect;
- access Keychain, credentials, tokens, API keys, certificates, private keys,
  accounts, provider consoles, models, gateways, cloud resources, Apple/Xcode,
  signing systems, product systems, or operational external systems;
- sign in, create a client registration, consent screen, workload mapping,
  service account, IAM role, subscription, billing relationship, resource,
  endpoint, key, token, model deployment, or local model installation;
- transmit a synthetic or personal prompt, run a provider request, or test
  authentication against a live service;
- accept consumer subscription credentials, browser cookies, CLI tokens,
  environment variables, shared credential files, ambient cloud identity, or
  arbitrary endpoint/model/account values as trusted product configuration;
- make any profile `selectable`, supersede D-118, unblock V0-3 or V0-7, accept
  D-113 through D-117, close a D-107 blocker, or authorize a successor;
- retrofit the current sealed V0 synthetic profile, reinterpret its tests as
  live-provider proof, or turn provider choice into runtime choice; or
- commit, push, merge, release, publish, deploy, or begin later work without
  the distinct authority required for that action.

The only external read allowed by the proposed documentation increment is
unauthenticated retrieval of current public official primary documentation to
revalidate the source register below. It must not sign in, call provider APIs,
or change external state.

## Terminology and trust-boundary separation

- **Cortexa user identity** answers who may use this owner-only application.
  It is not provider API authorization.
- **Provider authentication** authorizes a specific provider API resource or
  workload. It is not automatically Cortexa user identity.
- **Connection profile** is an application-owned, versioned, atomic binding of
  provider family, authentication class, topology, exact model policy, exact
  endpoint/resource policy, credential owner, disclosure, limits, and lifecycle
  policy. It is not a bag of caller-selected settings.
- **OAuth** means a provider-documented authorization flow for the relevant
  API. A consumer login screen, subscription, browser cookie, CLI session, or
  product-specific login is not automatically reusable OAuth authority.
- **Workload identity** authenticates an administrator-bound workload or
  service principal. It is not consumer OAuth and must not be presented as a
  user subscription entitlement.
- **Local/no-auth** means no external provider authentication and zero network
  authority for inference. It does not mean an unverified executable or model
  may be loaded.
- **Direct OpenAI** means `api.openai.com` / OpenAI API Platform semantics.
  **Azure OpenAI** means a Microsoft Azure resource with Microsoft endpoint,
  identity, RBAC, quota, billing, retention, and deployment semantics. They are
  separate providers and cannot share a profile, token, endpoint, billing
  claim, or fallback path.

ChatGPT login and ChatGPT Free, Plus, Pro, Business, Enterprise, Edu, or other
subscription access are explicitly prohibited as evidence of OpenAI API OAuth,
OpenAI API authorization, API-key possession, API billing, or Cortexa's right
to call the OpenAI API. OpenAI's Codex-specific "Sign in with ChatGPT" flow is
not a general third-party OAuth grant for Cortexa and must not be reused or
represented as one.

## Closed candidate catalog

The conceptual Rust-owned `ConnectionProfileKindV1` has exactly the following
members. This is a closed candidate register, not a runtime interface. Every
entry has current status `candidate_blocked`.

| Closed profile kind               | Provider boundary | Authentication class                            | Current closed reason                                                                                                      |
| --------------------------------- | ----------------- | ----------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `local_no_auth`                   | Local             | None                                            | No accepted engine/model/supply-chain/no-egress decision                                                                   |
| `google_gemini_oauth`             | Google Gemini     | OAuth authorization code with PKCE              | No accepted client registration, exact scopes/project/billing, token custody, transport, retention, or activation decision |
| `google_gemini_api_key`           | Google Gemini     | One exact Google-documented API-key class       | Exact key class transition, custody, restriction, billing, transport, and retention decisions are unresolved               |
| `direct_openai_api_key`           | Direct OpenAI     | OpenAI API Platform API key                     | D-021 and D-118 block direct desktop key custody and transport                                                             |
| `direct_openai_workload_identity` | Direct OpenAI     | Eligible OpenAI workload identity federation    | Eligibility, administrator mapping, token exchange, transport, and runtime deployment are unresolved                       |
| `azure_openai_entra`              | Azure OpenAI      | One exact future Microsoft Entra principal mode | Principal type, tenant, issuer, audience, resource, RBAC, deployment, transport, and custody are unresolved                |
| `azure_openai_api_key`            | Azure OpenAI      | Azure resource API key                          | Resource/deployment binding, custody, transport, retention, and D-060 reconciliation are unresolved                        |
| `anthropic_api_key`               | Anthropic         | Anthropic Console API key                       | Custody, workspace/billing, transport, retention, and activation decisions are unresolved                                  |
| `mistral_api_key`                 | Mistral           | Mistral API key                                 | Custody, workspace/billing, transport, retention, and activation decisions are unresolved                                  |
| `aws_bedrock_identity`            | AWS Bedrock       | One exact future nonambient AWS identity source | Identity source, account/role/region/model access, SigV4, custody, transport, retention, and activation are unresolved     |

The list intentionally does not admit:

- generic `oauth`, `api_key`, `provider`, `custom`, `openai_compatible`,
  `endpoint`, `model`, `runtime`, or `other` escape hatches;
- Anthropic workload identity or App Attest, Mistral workload identity, AWS
  Bedrock API keys, Google service-account/default credentials, or any other
  vendor-supported method not expressly listed above;
- an OpenAI consumer-login or ChatGPT-subscription profile;
- a combined OpenAI/Azure profile;
- arbitrary local endpoints, OpenAI-compatible servers, environment-driven
  providers, or plugin-supplied providers; or
- automatic or manual fallback between any entries.

Exclusion is deliberate scope control, not a claim that a vendor lacks the
method. Adding a method or profile requires a new versioned decision and cannot
silently widen `V1`.

## Official primary-source verification

The following public official sources were reviewed without authentication on
2026-09-03. They establish only that the named authentication mechanism is
documented by its provider; they do not prove Cortexa eligibility, safe desktop
custody, acceptable retention, subscription entitlement, an approved client,
or implementation readiness. Current source ambiguity fails closed and must be
rechecked when the decision increment begins and again before any later
implementation.

| Catalog option                  | Current official evidence                                                                                                                                                                                                                                                                                                                                                                                                                                          | Bounded conclusion                                                                                                                                                                                                                                                                                                           |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `local_no_auth`                 | No provider source applies because the entry forbids external provider authentication and inference traffic. Repository evidence, not vendor documentation, must later prove the exact engine/model has no egress or credential path.                                                                                                                                                                                                                              | `None` is the only allowed auth class. This does not approve a local engine, model, artifact, server, socket, plugin, or download.                                                                                                                                                                                           |
| Google Gemini OAuth             | [Gemini API OAuth quickstart](https://ai.google.dev/gemini-api/docs/oauth), [Google OAuth for installed apps](https://developers.google.com/identity/protocols/oauth2/native-app), [Google OAuth 2.0](https://developers.google.com/identity/protocols/oauth2), [OAuth policy](https://developers.google.com/identity/protocols/oauth2/policies), and [OAuth security practices](https://developers.google.com/identity/protocols/oauth2/resources/best-practices) | Google documents OAuth for the Gemini API and installed-app system-browser protections. The Gemini quickstart's ADC/gcloud setup, downloaded client-secret file, token-file persistence, and refresh behavior are examples, not Cortexa product authority. A future flow remains blocked pending an exact reviewed contract. |
| Google Gemini API key           | [Gemini API keys](https://ai.google.dev/gemini-api/docs/api-key)                                                                                                                                                                                                                                                                                                                                                                                                   | The date-ambiguous page proves only that Gemini API-key authentication exists. It does not establish an eligible or selectable key class for Cortexa; the exact current class and transition status must be revalidated or fail closed.                                                                                      |
| Direct OpenAI API key           | [OpenAI API authentication](https://developers.openai.com/api/reference/overview)                                                                                                                                                                                                                                                                                                                                                                                  | OpenAI documents bearer API keys and says keys must not be exposed in client-side code. No direct desktop custody exception is inferred.                                                                                                                                                                                     |
| Direct OpenAI workload identity | [OpenAI workload identity federation](https://developers.openai.com/api/docs/guides/workload-identity-federation) and [OpenAI workload identity token exchange](https://developers.openai.com/api/reference/workload-identity-federation)                                                                                                                                                                                                                          | OpenAI documents administrator-configured workload identity federation for eligible workloads/projects and short-lived project service-account tokens. It is not consumer OAuth, a ChatGPT login, or a subscription entitlement.                                                                                             |
| ChatGPT/OpenAI separation       | [OpenAI authentication guidance](https://learn.chatgpt.com/docs/auth) and [ChatGPT usage and API billing separation](https://learn.chatgpt.com/docs/enterprise/usage-limits)                                                                                                                                                                                                                                                                                       | Cortexa conservatively infers from OpenAI's documented Codex-specific login and API Platform/billing separation that a ChatGPT token is not reusable authority for Cortexa. No provider entitlement claim beyond those sources is made.                                                                                      |
| Azure OpenAI Entra or API key   | [Microsoft Foundry authentication and authorization](https://learn.microsoft.com/en-us/azure/foundry/concepts/authentication-authorization-foundry) and [Microsoft identity-platform authorization code flow with PKCE](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-auth-code-flow)                                                                                                                                                        | Microsoft documents Entra token and API-key paths. A later profile must freeze one principal type, tenant/issuer/audience/scope, Azure resource/deployment, RBAC, endpoint family, and token owner. Direct OpenAI credentials are invalid authority for this entry.                                                          |
| Anthropic API key               | [Anthropic API authentication](https://platform.claude.com/docs/en/manage-claude/authentication), [Anthropic CLI authentication](https://platform.claude.com/docs/en/cli-sdks-libraries/cli/authentication), and [Claude subscription/API separation](https://support.claude.com/en/articles/9876003-i-have-a-paid-claude-subscription-pro-max-team-or-enterprise-plans-why-do-i-have-to-pay-separately-to-use-the-claude-api-and-console)                         | Anthropic documents API keys, WIF, App Attest, and CLI-specific OAuth. The latter methods remain deliberately excluded; CLI OAuth is not a general third-party Cortexa grant. A Claude consumer subscription does not provide Anthropic Console/API entitlement.                                                             |
| Mistral API key                 | [Mistral API keys](https://docs.mistral.ai/admin/identity-access/api-keys), [Mistral workload identity](https://docs.mistral.ai/admin/identity-access/workload-identity), and [Mistral subscriptions](https://docs.mistral.ai/admin/billing-usage/subscriptions)                                                                                                                                                                                                   | Mistral documents API keys; workload identity is deliberately excluded from V1. Because current Mistral subscription documentation may span product surfaces, this plan makes no blanket consumer/API billing-separation claim.                                                                                              |
| AWS Bedrock identity            | [Amazon Bedrock IAM](https://docs.aws.amazon.com/bedrock/latest/userguide/security-iam.html), [AWS standardized credential providers](https://docs.aws.amazon.com/sdkref/latest/guide/standardized-credentials.html), [AWS SDK for Rust credential providers](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credproviders.html), and [Amazon Bedrock API keys](https://docs.aws.amazon.com/bedrock/latest/userguide/api-keys.html)                            | AWS recommends federated/temporary identities and documents broad default credential chains. Cortexa must later select one exact nonambient source; environment/shared-file/default-chain search is prohibited. Bedrock API keys are deliberately excluded from V1.                                                          |

### Decision-increment revalidation result

Every applicable external-provider entry in the register was re-opened from
current official pages on 2026-09-03 without authentication, provider API
calls, credentials, provisioning, or external writes. `local_no_auth` has no
provider authentication or applicable vendor source; it remains a repository-
evidence-only candidate whose engine, model, artifact, and no-egress properties
are unproved. The evidence remained consistent with
`closed_catalog_direction_selected`, subject to these fail-closed observations:

- Google's Gemini API-key page reported `Last updated 2026-09-02 UTC` but also
  described a transition occurring “September 2026” without an exact day. The
  candidate therefore remains blocked on the exact eligible key class. The
  Gemini OAuth page reported `Last updated 2026-08-24 UTC`; Google's installed-
  app page reported `Last updated 2026-08-07 UTC` and continues to require a
  system browser, registered redirect, and PKCE protections.
- OpenAI currently documents API keys and administrator-configured WIF that
  produces short-lived API Platform project service-account tokens. The pages
  exposed no visible update date. WIF remains workload identity, not end-user
  OAuth. ChatGPT/Codex authentication and workspace limits remain separate from
  general OpenAI API authorization and billing.
- Microsoft's Foundry authentication page reported `Last updated 2026-08-05`
  and the authorization-code/PKCE page reported `Last updated 2026-01-09`.
  They continue to document distinct Azure Entra and resource API-key paths;
  neither supplies authority for Direct OpenAI.
- Anthropic currently documents API keys plus WIF and App Attest, and its CLI
  docs describe a CLI-specific browser OAuth flow. Only `anthropic_api_key` is
  deliberately included. The Claude subscription/API separation page was
  dated 2026-03-16.
- Mistral continues to document API keys and separately documents workload
  identity. Its current plan may span Studio, API, and Vibe, so this decision
  makes no blanket consumer/API billing-separation claim.
- AWS continues to document IAM/SigV4 identity, broad SDK/default credential
  chains, and Bedrock bearer API keys. Only one later-frozen explicit
  nonambient IAM identity source is represented; Bedrock API keys remain a
  deliberate catalog exclusion.

Pages without a visible update date were not assigned an invented date. New or
omitted vendor methods did not widen the closed catalog; admission requires a
future versioned decision and fresh evidence.

No reviewed current official primary source established a general third-party
end-user OAuth grant for Direct OpenAI, Anthropic, or Mistral that Cortexa may
use. This is a bounded evidence result, not a claim of impossibility. Any future
new vendor method requires a new source review and a versioned catalog change.

## Decision conflicts and required reconciliation

The decision increment must preserve history and explicitly reconcile—not
silently overwrite—the following constraints:

1. **D-021 and D-060, gateway-owned production credentials.** D-060 keeps
   every production AI-provider credential out of the desktop; D-021 defines
   the current OpenAI/gateway credential contract. Provider OAuth/Entra access
   or refresh tokens, API keys, workload assertions and temporary credentials,
   and AWS credentials therefore remain gateway-side. Only a separately
   governed, short-lived Cortexa gateway access token may live in trusted Rust;
   that token is not provider authentication. Every direct/native provider-auth
   candidate remains blocked. Any future direct implementation requires a
   separately accepted reconciliation or successor for every applicable
   controlling decision—at minimum D-060, plus D-021 for the current OpenAI/
   gateway contract—and must name the exact credential owner, provisioning/
   removal path, exposure boundary, and rollback.
2. **D-032, no generic synchronous provider abstraction.** The catalog may not
   revive `complete(prompt) -> String`, a stringly provider interface, or an
   open provider plugin boundary.
3. **D-035, narrow fixed gateway request.** The current V1 request contract is
   unchanged. Any later selectable UI must use a separately approved versioned
   contract carrying only an opaque Rust-issued option handle and bounded user
   text; it may not carry provider/model/auth/endpoint/account fields.
4. **D-060, separated identity/hosting/provider authority.** The catalog
   preserves that separation and makes trusted Rust—not UI, model, runtime, or
   environment—the profile resolver. Google provider OAuth and Azure Entra are
   not automatically Cortexa user identity.
5. **D-061, exact external-processing evidence.** No cloud profile can become
   selectable without provider-approved Zero Data Retention for the exact
   provider, organization/account, project, endpoint, model, and region plus
   logging, deletion, training, abuse-monitoring, disclosure, and rollback
   evidence. If exact ZDR is unavailable, the profile remains blocked unless a
   separately accepted decision explicitly supersedes D-061. Admission binds
   the accepted evidence/disclosure version and freshness; expiry or drift
   closes availability.
6. **D-062, current Cortexa identity direction.** Microsoft personal-account
   identity for Cortexa is independent of every provider-auth method. This plan
   neither changes it nor grants a provider token authority over Cortexa.
7. **D-063 and D-064, historical/planned Azure production direction.** Their
   provider and four-stage identity/hosting/configuration evidence remain
   historical and independently governed; the catalog cannot reinterpret them
   as approval for an Azure profile. Any future departure requires explicit
   additive lineage reconciliation.
8. **D-066 through D-068, sealed OpenAI-through-Cloudflare synthetic
   direction.** These govern the fixed synthetic-v1 lane. The new decision does
   not remove Cloudflare, activate Direct OpenAI, or grant transport authority.
9. **D-079 through D-081, runtime ownership.** Provider/profile choice does not
   select or replace `NativeAgentRuntime`; Hermes remains Deferred/Blocked and
   OpenClaw remains evaluation-only.
10. **D-094, fixed V0 contracts and selector prohibition.** Synthetic-v1 and
    reserved `real-content-v2` remain fixed and nonselectable. A positive D-119
    may narrowly supersede only any broader inference that D-094 permanently
    prohibits a separately approved post-v0
    `personal-assistant-selectable-connection-profile-v3` contract. It does not
    change either V0 contract, and catalog tests may not be claimed as proof for
    those lanes or vice versa.
11. **D-096 through D-117.** The published failed signing record, successor
    lineage, all ten D-107 blockers, and Proposed/non-controlling decisions
    remain untouched. This plan grants no credential, Keychain, signing, or
    containment authority.
12. **D-118, no eligible HTTPS client.** Every network-backed candidate remains
    blocked. The profile decision cannot select, infer, vendor, or bypass a
    client, TLS stack, proxy policy, DNS policy, or transport.

If another increment claims D-119, if any cited decision has materially
changed, or if the exact current V1 boundaries differ when work begins, stop
and amend this plan before editing decision memory.

## Conceptual closed interfaces

The following names describe architecture invariants only. They are not source
design approval and must not be copied into product code under this increment.
Here `V1` names the first closed catalog schema nested within the separately
versioned post-v0 product contract
`personal-assistant-selectable-connection-profile-v3`; it does not rename or
widen synthetic-v1 or `real-content-v2`.

```text
ConnectionProfileKindV1 =
  local_no_auth
  | google_gemini_oauth
  | google_gemini_api_key
  | direct_openai_api_key
  | direct_openai_workload_identity
  | azure_openai_entra
  | azure_openai_api_key
  | anthropic_api_key
  | mistral_api_key
  | aws_bedrock_identity

ConnectionProfileAvailabilityV1 = candidate_blocked

ConnectionProfileDescriptorV1 = {
  display_label,        // fixed application text
  provider_label,       // fixed application text
  auth_label,           // fixed application text
  availability,         // currently only candidate_blocked
  disclosure_id,        // closed application-owned disclosure reference
  blocked_reason_code   // closed, redacted, application-owned enum
}
```

`ConnectionProfileAvailabilityV1` intentionally has no `selectable` state in
the current decision. A later separately approved version may add it only after
one exact entry meets every dependency. The UI may eventually display blocked
options for transparent planning, but a blocked descriptor has no selection
handle and cannot be submitted. The UI cannot unlock, synthesize, mutate, or
submit raw configuration.

Only a separately admitted future catalog version may issue an opaque,
process- and catalog-generation-bound selection handle, and only for a profile
that has become fully admitted. Trusted Rust must atomically resolve that
handle to one complete application-owned tuple containing:

- catalog version and generation;
- exact profile kind and provider boundary;
- exact authentication class and credential owner;
- exact topology and transport owner;
- exact endpoint/resource/region/project/account constraints;
- exact provider model or local model/artifact policy;
- immutable agent instructions and empty tool set for the V0 text slice;
- external-processing disclosure and retention policy identifier;
- exact accepted external-processing evidence version and freshness boundary;
- request, stream, token, byte, event, queue, time, and concurrency bounds;
- cancellation, cleanup/quarantine, and late-result policies; and
- availability/kill-switch state.

The run receives an immutable Rust-owned snapshot. Catalog refresh, UI route
change, credential change, provider drift, or kill-switch transition cannot
mutate an in-flight run. A closed revocation rule may terminate it; nothing may
silently rebind or fall back.

## Invariants

1. The catalog is a compile-time or equivalently closed application-owned Rust
   mapping. No environment, file, URL, plugin, model, WebView, caller, provider
   response, or remote registry can add or mutate an entry.
2. The catalog has exactly the ten profile kinds listed above. Unknown,
   duplicate, malformed, future-version, or omitted values fail closed.
3. Direct OpenAI and Azure OpenAI are distinct providers in types, display,
   credentials, endpoints, identity/audience, resources, billing, retention,
   errors, tests, and logs. Neither can fall back to the other.
4. Provider and authentication class are inseparable. The UI cannot combine a
   provider descriptor with a different auth method or submit raw enum names.
5. The current blocked catalog issues no selection handle. Only a separately
   admitted future version may issue opaque handles for fully admitted entries,
   bound to the current process, catalog version, and generation. Unknown,
   forged, stale, cross-session, cross-generation, replayed, or mismatched
   handles are rejected before credential or network work.
6. Any future user selection remains untrusted intent. Trusted Rust derives the
   exact profile tuple, run/request identities, instructions, model policy,
   empty tool set, limits, and lifecycle policy.
7. The initial state is `Unselected`. There is no implicit default, environment
   inference, most-recent profile, silent auto-connect, or fallback.
8. A future profile starts only from one explicit foreground owner action and
   supports at most one foreground request at a time.
9. No profile is currently selectable. Missing evidence, unavailable auth,
   credential ambiguity, source drift, unsupported platform, or dependency
   failure stays `candidate_blocked`; it never degrades to another profile.
10. ChatGPT login, subscription, cookies, sessions, and Codex authentication
    are never accepted as OpenAI API authorization or billing evidence.
11. Secrets, authorization codes, access/refresh tokens, assertions, signed AWS
    requests, API keys, account identifiers, endpoint details, and raw provider
    errors never enter WebView state, Tauri DTOs/events, logs, debug output,
    SQLite, project files, tests, reports, or ordinary CI.
12. No OAuth client secret is embedded in a public desktop client. Any future
    OAuth flow uses the system browser and Authorization Code with PKCE S256.
    The separately approved credential-owning authentication boundary
    independently generates the one-time CSPRNG PKCE verifier and `state`, plus
    a one-time CSPRNG `nonce` only when the selected protocol is OIDC. Under
    D-060, that owner is the gateway; trusted desktop Rust may own any
    credential-bearing OAuth state, callback, code, or provider token only
    after separately accepted direct/native-custody reconciliation. The owner
    binds the material to the exact attempt, profile and catalog generation,
    issuer/client/scopes/audience, redirect, and deadline. Exactly one bounded
    callback and one code may be accepted; mismatch, replay, duplicate, late,
    expired, foreign, or cancelled results fail closed. Embedded WebViews and
    password grants are prohibited. Attempt material remains only in bounded
    volatile memory of the approved owner. On every terminal outcome, that
    owner zeroizes sensitive verifier, code, and token material no longer
    needed for bounded cleanup while bounded nonsecret attempt identity,
    generation, terminal tombstone, and cleanup/quarantine ownership remain
    until cleanup/quiescence is proved or a closed bounded terminal disposition
    applies. Pure provider OAuth is never Cortexa identity.
13. No `offline_access`, refresh token, persistent token cache, or silent
    renewal exists in the first post-v0 selectable implementation unless a
    separate credential-lifecycle decision explicitly accepts it.
14. API-key profiles use gateway-owned custody under D-060 and the applicable
    provider-specific decisions. Direct or native custody requires separately
    accepted reconciliation of every controlling decision—at minimum D-060,
    plus D-021 for the current OpenAI/gateway contract—and a native Rust-owned
    secret-reference/lifecycle decision. Key bytes never become a profile
    field.
15. Workload identity profiles are enabled only for an exact eligible,
    administrator-bound workload configuration. They are not user OAuth and
    may not consume arbitrary assertion sources.
16. AWS identity uses one exact explicitly configured provider in a future
    decision. The broad default credential chain, environment variables,
    shared files, EC2/ECS metadata probing, and ambient profile search are
    prohibited.
17. Local/no-auth proves zero provider/network/credential authority. Model
    downloads, update checks, telemetry, remote embeddings, localhost servers,
    and implicit fallback are prohibited unless separately accepted.
18. Every cloud profile binds one exact, freshness-bounded external-processing
    evidence and disclosure version. Before any system-browser authorization,
    token exchange, or other first external auth boundary, trusted Rust requires
    a versioned one-use auth-boundary admission after the disclosure is visible.
    Before model content transmission, it requires a separate, noninterchangeable
    one-use model-processing admission. Both remain visible in Settings and
    require re-acknowledgment after a material provider, endpoint, retention,
    logging, or data-class change. Neither admission starts the other boundary.
19. Every terminal outcome is closed, redacted, monotonic, and idempotent.
    Success, failure, cancellation, timeout, auth denial, and policy rejection
    cannot transition into one another after terminalization.
20. No provider response, auth callback, refresh event, task completion, model
    output, runtime event, or clock value is trusted identity or authority.
21. Tools remain empty. No profile enables filesystem, memory, scheduling,
    background autonomy, specialist delegation, approval dispatch, tool use,
    persistence, or device effects.

## Credential ownership and lifecycle

This increment documents boundaries; it does not choose a credential-storage
implementation. Under D-060, every AI-provider credential remains gateway-
owned; D-021 additionally defines the current OpenAI/gateway contract. The
desktop may hold only the separately governed short-lived Cortexa gateway
access token in trusted Rust; Cortexa gateway-user OIDC is distinct from every
provider-authentication option below.

| Authentication class  | Current owner and future minimum contract                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| None                  | Trusted Rust must prove that the local path creates no provider credential, auth request, network connection, external server, or remote update/telemetry effect.                                                                                                                                                                                                                                                                                                                                                                                             |
| OAuth/Entra delegated | D-060 requires provider authorization results and tokens to remain gateway-owned. A future profile must fix a gateway-brokered topology or obtain separately accepted reconciliation of every applicable controlling decision for direct/native custody, including D-021 for the current OpenAI/gateway contract. System browser, attempt binding, PKCE/state/nonce, token destination, zeroization, tombstone/quarantine retention, revocation semantics, deadlines, cancellation, and redaction must be exact; no provider refresh token is admitted in V0. |
| API key               | D-060 requires the provider key to remain gateway-owned. A future direct/native owner could resolve only an opaque application-owned secret reference after every applicable controlling decision is reconciled; the current OpenAI/gateway contract also requires a D-021 successor. WebView/profile DTOs never contain the key. Provisioning, validation, rotation, removal, backup behavior, OS prompts/effects, and rollback require separate acceptance.                                                                                                 |
| Workload identity     | D-060 keeps assertions and resulting provider credentials gateway-owned. Any direct/native exception requires reconciliation of every applicable controlling decision, including D-021 for the current OpenAI/gateway contract. One later decision must bind the exact issuer/subject/audience/mapping and bounded assertion source; browser subscriptions, arbitrary token files, environment assertions, and caller-supplied issuers are prohibited.                                                                                                        |
| AWS identity          | D-060 keeps AWS credentials and signing authority gateway-owned. Any direct/native exception requires reconciliation of every applicable controlling decision. One later decision must bind a single nonambient source, account/role/region/resource policy, temporary-credential lifecycle, and private SigV4 owner; default-chain discovery and metadata probing remain disabled.                                                                                                                                                                           |

If safe credential ownership cannot be specified without persistent secrets,
OS prompts, signing, ambient discovery, or a new trust boundary, that profile
remains blocked and requires its own plan. Absence of a credential must produce
a closed `credential_unavailable`-class result, never a fallback or setup side
effect.

## Cancellation, cleanup, and late-result rejection

Authentication and model execution use separate application-owned attempts
with distinct opaque IDs, monotonic generations, state owners, limits, and
terminal results. Trusted desktop Rust owns only bounded nonsecret coordination
state under the current D-060 gateway topology; the gateway owns provider-
credential-bearing authentication and model state. Trusted desktop Rust may
become the credential-bearing owner only after separately accepted direct/
native-custody reconciliation. Authorization success never starts model
transport. Model transmission always requires a fresh explicit foreground
action and a distinct one-use model-processing admission; no later UX decision
may collapse those boundaries without first superseding this constraint
through a separately accepted security decision.

The auth attempt cannot create its first external effect until trusted desktop
Rust has validated the exact, current auth-boundary disclosure version,
consumed its one-use admission, and authorized the separately approved
credential-owning boundary. The model attempt separately validates and
consumes the exact model-processing disclosure/admission before prompt
transmission. These admissions are not credentials, are not durable, and are
never interchangeable. Under D-060, no provider authorization result or token
returns to the desktop.

For any future attempt:

1. Trusted desktop Rust creates a bounded nonsecret coordination record before
   the first browser, credential, transport, local-engine, or provider effect.
   The separately approved credential-owning boundary creates and owns its
   internal attempt before its first effect; under D-060 that boundary is the
   gateway.
2. For OAuth/OIDC, the approved credential-owning boundary independently
   creates the one-time CSPRNG PKCE verifier and `state`, plus `nonce` only for
   OIDC, and binds them to the attempt ID, profile kind, catalog generation,
   exact redirect, and deadline. It accepts at most one bounded callback and
   one authorization code; mismatch, replay, duplicate, foreign, late, expired,
   or cancelled input fails closed. Native ownership requires the separately
   accepted custody reconciliation described above.
3. Cancellation and deadline terminalization close desktop-visible result
   ingress first at one serialized linearization point. The approved remote or
   authentication boundary must independently close its ingress, revoke
   further work, and own bounded cleanup; neither boundary may accept a late
   result.
4. Cancellation sends no replacement request, selects no alternate auth/profile,
   and causes no retry, refresh, reconnect, browser relaunch, local fallback, or
   cloud fallback.
5. Logout or cancellation makes the approved credential-owning boundary
   zeroize sensitive verifier, authorization-code, and token material no longer
   needed for bounded cleanup. Under D-060, the desktop retains only bounded
   nonsecret attempt identity, generation, terminal tombstone, and cleanup/
   quarantine coordination state; the gateway retains cleanup ownership until
   cleanup/quiescence is proved or a closed bounded terminal disposition
   applies. Local discard and expiry are not remote revocation and do not prove
   the system-browser session ended. A provider revocation or end-session call
   requires a separately approved exact endpoint and observed successful
   evidence; absence or failure remains disclosed and closed, with no retry or
   fallback.
6. A local deadline does not claim that a remote provider, system browser,
   credential source, DNS/TLS operation, SDK, OS API, or local engine stopped.
   Cleanup/quiescence must be proved separately for the exact implementation.
7. Unknown, malformed, duplicate, foreign, prior-generation, post-cancel,
   post-timeout, post-failure, and post-success auth callbacks, token results,
   transport bytes, stream events, provider results, and local-engine results
   are rejected before state, UI, IPC, logs, evidence, billing-follow-up,
   persistence, or subsequent dispatch.
8. Cleanup uncertainty retains an application-owned quarantine lease and
   blocks restart for that profile. It never releases ownership optimistically.
9. Buffers, tombstones, event queues, cleanup time, and quarantines have fixed
   bounds. Exhaustion and ambiguity fail closed.
10. Terminal errors are closed application enums. Raw provider, SDK, HTTP, OAuth,
    IAM, operating-system, model, endpoint, account, key, and token details are
    adapter-private and redacted before crossing a boundary.

Existing deterministic cancellation and late-event tests are reusable design
references only. They do not prove provider auth, networking, local inference,
credential cleanup, remote cancellation, or this future catalog.

## Threat model

- **Provider/auth mix-and-match:** UI or model combines an allowed provider
  with a stronger, cheaper, or unrelated auth method.
- **OpenAI/Azure collapse:** an endpoint, token, deployment, retention, billing,
  or error from one boundary is accepted by the other.
- **Consumer-login laundering:** ChatGPT, Claude, Gemini, Mistral, AWS console,
  CLI, browser, or subscription state is treated as provider API authority.
- **Auth downgrade or fallback:** OAuth denial silently becomes API-key use;
  cloud failure becomes local use; one provider becomes another.
- **Ambient credential capture:** environment variables, shared files, login
  sessions, metadata services, CLI caches, or default SDK chains are consumed.
- **Caller-selected authority:** raw provider/model/endpoint/account/project/
  tenant/region/role/scope/issuer/redirect/credential identifiers cross IPC.
- **Stale-handle and ABA replay:** a prior catalog generation or completed run
  selects a newly configured profile.
- **Mid-run rebinding:** profile refresh, credential rotation, route changes, or
  policy drift mutates an active run.
- **Secret exfiltration:** credentials or raw/secret provider configuration
  reaches WebView, logs, errors, debug formatters, persistence, tests,
  screenshots, or CI.
- **OAuth interception/CSRF:** embedded user agents, missing PKCE/state/nonce,
  wildcard redirect, callback races, code replay, or overbroad scopes expose an
  authorization result.
- **Workload-identity confusion:** a caller supplies issuer/subject/audience or
  a consumer token is exchanged as a workload credential.
- **AWS chain expansion:** an SDK silently searches additional providers or
  metadata endpoints.
- **Local-path hidden egress:** engine/model install, update, telemetry,
  embeddings, license checks, or localhost delegation violates no-auth.
- **Disclosure drift:** a profile runs under retention, training, geography,
  billing, or logging terms different from its accepted disclosure.
- **Cancellation laundering:** UI cancellation is claimed as proof that browser,
  credential, provider, network, OS, or local-engine work stopped.
- **Late-result mutation:** a late callback/event reverses terminal state,
  exposes content, triggers billing or persistence, or starts a follow-up.
- **D-118 bypass:** a provider SDK, WebView fetch, shell, plugin, transitive
  crate, or local server is used as an undeclared transport.
- **Fixture transference:** deterministic synthetic tests are described as live
  auth/provider/local-model evidence.

## Closed decision dispositions

The documentation increment must select exactly one disposition:

| Disposition                         | Meaning                                                                                                                                                                                                 |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `closed_catalog_direction_selected` | The owner accepts the closed architecture direction and proposed D-119 wording. All ten profiles remain `candidate_blocked`; no runtime, credential, transport, or implementation authority is granted. |
| `catalog_not_accepted`              | The owner does not accept this profile/catalog direction. Current V1 architecture and all Blocked states remain unchanged.                                                                              |
| `evidence_boundary_failed`          | Required official or repository evidence is missing, ambiguous, contradictory, stale, or outside the authorized boundary. Nothing is accepted or inferred.                                              |

There is no partial, provisional, default-open, or "accept what is supported"
outcome. Source drift or disagreement selects `evidence_boundary_failed`.

## Proposed D-119 wording

If and only if the owner accepts `closed_catalog_direction_selected`, the
decision increment may add a D-119 record with this substantive result:

> Accept a future Rust-owned, versioned, closed connection-profile catalog
> direction with exactly the ten candidate profile kinds recorded in the
> approved plan. Direct OpenAI and Azure OpenAI are separate provider
> boundaries. ChatGPT login or subscription access is not OpenAI API OAuth,
> authorization, or billing. The blocked catalog exposes no selection handle.
> Only a separately approved future catalog version may issue a Rust-owned,
> opaque, process- and catalog-generation-bound handle for a fully admitted
> post-v0 `personal-assistant-selectable-connection-profile-v3` profile; user
> choice remains untrusted intent, and trusted Rust resolves one immutable
> atomic provider/auth/model/endpoint/credential/disclosure/lifecycle tuple.
> D-119 narrowly supersedes D-094 only to reserve that distinct post-v0 selector
> direction; synthetic-v1 and reserved `real-content-v2` remain unchanged and
> nonselectable under D-094. Atomic runtime pairing does not merge D-060's
> independently required Cortexa identity, hosting/transport, and provider
> approvals, and D-062 remains the sole planned Cortexa Phase-1 identity
> provider. No profile is currently selectable: every entry remains
> `candidate_blocked`, D-118 remains `no_eligible_client`, V0-3/V0-7 and
> operational successors remain Blocked, and all existing credential, signing,
> external-processing, transport, cancellation, cleanup, late-result,
> empty-tool, no-fallback, and explicit-user-action boundaries remain. This
> decision grants documentation direction only and does not authorize source,
> dependencies, credentials, authentication, provider access, local models,
> network, persistence, provisioning, signing, or external-system work.

The final record must also cite the exact accepted plan, evidence date,
baseline, conflicts, selected disposition, and owner acceptance. If exact
wording needs a material change, stop for owner approval rather than weakening
the result during reconciliation.

## Implementation milestones for the documentation decision

- [x] Reconfirm synchronized `main`, the unchanged prior complete record whose
      marker was invalid only because of this plan's workspace fingerprint,
      free D-119 slot, current source boundaries, and unchanged decision
      conflicts.
- [x] Revalidate every source in the official evidence table without signing
      in or accessing provider APIs; record publication/update dates when the
      source exposes them and treat ambiguity as closed failure.
- [x] Create the approved `codex/` branch and run the accepted corrected
      `begin` command
      only after separate owner approval.
- [x] Record the exact closed catalog, trust-boundary separation, threats,
      invariants, credential ownership, cancellation, cleanup, and late-result
      contracts in the authorized documentation scope.
- [x] Select exactly one closed disposition. Add D-119 only when the owner has
      accepted `closed_catalog_direction_selected` under the increment.
- [x] Reconcile current memory without rewriting historical evidence or
      changing any Blocked/Proposed status outside the accepted result.
- [x] Run documentation-tier, exact-scope, protected-path, preservation,
      independent architecture/security/readiness, session, quality, and
      post-increment checks.
- [x] Stop for owner review without committing, pushing, merging, or starting
      any implementation or successor.

## Test plan

### Documentation-decision tests

- Confirm the catalog has exactly ten unique profile kinds and no open/custom
  variant.
- Confirm the allowed provider/auth pairs match the table exactly.
- Confirm Direct OpenAI and Azure OpenAI are separate everywhere.
- Confirm ChatGPT login/subscription is explicitly rejected as OpenAI API
  OAuth, authorization, and billing evidence.
- Confirm every applicable external-provider authentication claim has a current
  official primary source and no source is a search result, blog, aggregator,
  forum, or third-party guide. Confirm separately that `local_no_auth` has no
  provider source and remains repository-evidence-only.
- Confirm omitted vendor auth mechanisms are described as excluded, not
  unsupported.
- Confirm all entries remain `candidate_blocked` and D-118 remains
  `no_eligible_client`.
- Confirm V0-3/V0-7, all operational successors, D-107's ten blockers, and
  D-113 through D-117 status remain unchanged.
- Confirm current fixed synthetic profile source/tests and all historical
  records are unmodified.
- Confirm exactly the sixteen authorized documentation paths change after the
  future increment, with no product or dependency path touched.

### Contracts required before any later source increment

These are future acceptance requirements, not tests run by this documentation
increment:

- exhaustive Rust catalog membership and provider/auth pairing tests;
- closed parsing/rejection of unknown, malformed, duplicate, missing, and
  future-version catalog data;
- opaque-handle forgery, expiry, replay, cross-session, cross-generation, and
  cross-profile rejection tests;
- proof that callers cannot submit provider/model/endpoint/account/auth/secret
  fields and that Rust derives the complete immutable tuple;
- no-default, no-fallback, no-auto-connect, explicit-user-action, one-run, and
  empty-tool tests;
- separate Direct OpenAI/Azure token, endpoint, model/deployment, billing,
  disclosure, error, and fallback-denial tests;
- ChatGPT/subscription/cookie/CLI-token rejection tests;
- OAuth PKCE/state/nonce/callback, minimal-scope, code-replay, timeout,
  cancellation, cleanup, and late-callback tests using hermetic fakes;
- API-key absent/invalid/rotated/revoked/removal and secret-redaction tests
  without real keys;
- workload issuer/subject/audience/mapping and arbitrary-assertion rejection
  tests without live token exchange;
- AWS explicit-provider tests proving environment/shared-file/metadata/default-
  chain discovery is absent;
- local no-network/no-provider/no-telemetry/no-update/fallback-denial tests;
- immutable in-flight snapshot and kill-switch/cancellation race tests;
- bounded stream/event/byte/queue/time tests and terminal monotonicity;
- late auth/provider/local-result rejection before state, UI, IPC, logs,
  persistence, evidence, follow-up, or retry;
- closed redacted error/log/DTO snapshot tests proving no secret or raw upstream
  detail crosses the native boundary; and
- TypeScript parsing from `unknown` with exact version, bounds, enum, and state-
  transition rejection if a later Tauri presentation increment is approved.

No live-provider test, account login, credential test, local model execution,
network call, target-Mac auth prompt, billing test, or retention claim belongs
to this documentation increment.

## Verification commands

Run baseline checks before editing and repeat them after the final documentation
edit:

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
```

For the separately approved decision increment, also run:

```bash
git status --short --branch
git diff --name-only <recorded-baseline> --
npm run verify
```

Run the repository-prescribed independent architecture, security, and readiness
reviews; session-end and quality review; and the exact post-increment finalizer
only after the last authorized edit. Record every command as Passed, Failed,
Pending, or Not run. A valid completion marker must match the final report and
workspace fingerprint.

## Manual and target-Mac gates

For this documentation decision:

- owner acceptance of exactly one closed disposition: Passed — the owner
  explicitly accepted `closed_catalog_direction_selected` in the approved
  implementation prompt;
- independent architecture review of catalog closure and V1 separation:
  Passed;
- independent security review of credential/cancellation/late-result wording:
  Passed after bounded wording corrections and accepted re-review;
- revalidation of current official primary sources: Passed within the bounded
  unauthenticated read-only evidence scope; and
- branch, begin, session, quality, report, and completion-marker gates: Passed.
  Commit, push, merge, and publication: Not run and unauthorized.

Target-Mac authentication, Keychain, browser callback, API-key, workload
identity, IAM, SigV4, local model, provider request, transport, cancellation,
cleanup, late-result, network, billing, logging, resize/theme/accessibility, and
device-effect checks are all Not run. Documentation cannot promote them to
Passed.

## Risks

- Official authentication, key-class, eligibility, subscription, billing, or
  SDK documentation may change between this plan and implementation.
- A catalog may appear to authorize provider work even when all profiles are
  blocked; status and non-authority wording must remain adjacent to the list.
- "OAuth" may be misunderstood as one portable login across providers.
- OpenAI and Azure OpenAI may be conflated by similar API shapes.
- Workload identity and AWS federation may be misrepresented as consumer login.
- Direct desktop API keys may weaken the existing gateway-held-secret boundary.
- A generic connector abstraction may recreate the deleted D-032 authority.
- Display descriptors or errors may leak provider/account configuration.
- An SDK may introduce hidden credential discovery, retry, proxy, telemetry,
  storage, transport, or background behavior.
- Local/no-auth may hide downloads, update checks, telemetry, subprocesses,
  localhost networking, or unreviewed model artifacts.
- Cancellation and late rejection may be overstated as remote or OS quiescence.
- Reconciliation may recursively describe publication state or rewrite dated
  historical evidence.

All of these risks remain closed by documentation-only scope, exact catalog
membership, `candidate_blocked` status, no fallback, and separate future
decisions.

## Rollback and failure strategy

- During this uncommitted planning task, rollback is deletion or owner-directed
  revision of this one new plan file; no runtime or external state exists to
  unwind.
- During the future documentation decision, rollback is a documentation-only
  revert of that increment before publication. Historical records remain
  additive and must not be rewritten.
- `catalog_not_accepted` leaves the current sealed synthetic profile and all
  blockers unchanged.
- `evidence_boundary_failed` records the exact evidence problem without
  inferring support or selecting a subset.
- After any future publication, correction requires an additive superseding
  decision and normal Git rollback; D-119 may not be silently edited into a
  broader catalog.
- No failure may trigger account setup, credential access, dependency work,
  provider calls, local model installation, fallback, or successor start.

## Stop conditions

Stop immediately and report rather than infer, repair, or widen scope if:

- `main` is dirty, divergent, unsynchronized, ambiguous, or no longer descends
  from the recorded planning baseline;
- existing user changes overlap an authorized documentation path;
- the prior complete record, report, or historical marker changed; its
  fingerprint invalidity has any cause other than this approved untracked plan;
  or another gate is active;
- D-119 is occupied, a named predecessor changes, or historical evidence would
  need rewriting;
- any official source is unavailable, stale, materially changed, ambiguous, or
  contradicts the catalog/authentication claim;
- a requested auth method requires reusing a consumer subscription, browser or
  CLI token, embedded client secret, ambient credential chain, or undisclosed
  external processing;
- Direct OpenAI and Azure OpenAI cannot remain separate end to end;
- a profile would need to become selectable, obtain a credential, access a
  provider, load a local model, add a dependency, or send network traffic to
  make the documentation decision;
- any entry loses `candidate_blocked`, D-118 changes, V0-3/V0-7 is described as
  Ready, or a D-107 blocker/Proposed decision is promoted;
- a custom/open provider, auth, endpoint, model, runtime, plugin, or fallback
  escape hatch is introduced;
- product, dependency, lockfile, workflow, permission, CSP, signing, credential,
  provider, or external-system scope changes;
- the exact file ceiling is exceeded, a required validation fails, an
  independent review returns Revise/Blocked, or a completion marker is invalid;
  or
- owner approval is missing for the plan, closed disposition, material wording
  change, publication, or successor.

## Acceptance criteria

- [x] The planning baseline and current implementation boundary are recorded.
- [x] The plan defines exactly ten closed candidate profile kinds.
- [x] Local/no-auth, Gemini OAuth/API key, Direct OpenAI API key/eligible
      workload identity, Azure OpenAI Entra/API key, Anthropic API key, Mistral
      API key, and AWS Bedrock identity are covered.
- [x] Direct OpenAI and Azure OpenAI are separate boundaries.
- [x] ChatGPT login/subscription reuse is explicitly prohibited.
- [x] Every applicable external-provider authentication option has current
      official primary-source evidence, with limitations and deliberately
      excluded methods stated; `local_no_auth` has no applicable provider
      source and remains repository-evidence-only.
- [x] Decision conflicts, threats, invariants, credential ownership,
      cancellation, cleanup, late-result rejection, tests, rollback, and stop
      conditions are explicit.
- [x] All profiles remain `candidate_blocked`; no operational authority is
      granted.
- [x] The future documentation increment has an exact sixteen-file ceiling and
      closed decision outcomes.
- [x] The owner approves this exact Ready ExecPlan.
- [x] The separately approved documentation decision is implemented, reviewed,
      validated, and receives a valid completion marker.

## Progress

- 2026-09-03: Inspected clean synchronized `main`, current gate state,
  repository-owned Personal Assistant/runtime/Tauri boundaries, decision
  lineage through D-118, V0 program plans, and security/testing guidance.
- 2026-09-03: Reviewed current unauthenticated public official authentication
  documentation for every applicable external provider/authentication family.
  `local_no_auth` was assessed only against repository-owned boundaries because
  no provider source applies. No account, credential, provider API, or external
  state was accessed.
- 2026-09-03: Drafted this one-file Ready ExecPlan. No branch was created and no
  gate was begun.
- 2026-09-03: The owner approved the exact decision increment. Created the
  approved `codex/` branch from baseline
  `01dbb1fdce10c197033c1a88dbeeb53afb0a21cd`. The original 73-character gate
  ID was rejected by the hook's 64-character bound before state changed; the
  owner approved the corrected plan-only ID and artifact paths.
- 2026-09-03: Baseline plan formatting, documentation, repository, security,
  and diff checks passed. Began gate
  `pa-v0-selectable-connection-profile-decision` successfully.
- 2026-09-03: Revalidated every applicable external-provider entry in the
  official primary-source register through unauthenticated read-only public
  pages. The repository-only `local_no_auth` entry remained unproved. Source
  drift remained compatible with the closed, deliberately non-exhaustive
  catalog because every entry remains blocked; new or omitted provider
  mechanisms received no implied authority.
- 2026-09-03: The owner accepted `closed_catalog_direction_selected`. Added
  D-119 and reconciled the exact documentation scope without changing product
  source or operational readiness.
- 2026-09-03: Initial independent review identified bounded credential-lineage,
  auth/model-action, terminal-state, and chronology wording issues. Corrected
  only the authorized documentation; architecture, security, and documentation-
  readiness re-reviews accepted the result.
- 2026-09-03: Documentation, repository, security, complete verification,
  exact-scope, protected-path, preservation, independent-review, session,
  quality, and report-validation gates passed. The first sandboxed finalizer
  could not write ignored gate state after validating the report; the identical
  authorized rerun completed it. The corrected report was revalidated and
  rebound once, leaving a complete valid marker for the exact sixteen-file
  workspace.
- 2026-09-03: Owner-authorized same-increment correction replaced one stale
  active-gate statement, scoped official-source claims to applicable external
  provider/authentication families, kept `local_no_auth` repository-evidence-
  only, and made OAuth credential-bearing state and cleanup ownership
  conditional on D-060 gateway custody unless a future direct/native-custody
  reconciliation is separately accepted. The first corrected architecture re-
  review found one remaining overbroad source claim in the acceptance checklist;
  the same-increment correction narrowed it consistently. The final result was
  re-reviewed, revalidated, and rebound without beginning a successor.

## Final results

Complete with **PASS WITH ADVISORIES**. The accepted result is
`closed_catalog_direction_selected`; all ten profiles remain
`candidate_blocked`, no profile has a selection handle, and no product
authority exists. Every required check passed and the completion marker is
valid. Operational/provider/credential/local-model/target-Mac checks remain
`Not run`; owner review and publication remain pending.
