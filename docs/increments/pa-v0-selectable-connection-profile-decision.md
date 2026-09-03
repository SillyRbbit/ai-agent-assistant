# Personal Assistant selectable connection-profile architecture decision

Status: Complete (`PASS WITH ADVISORIES`) — operational readiness remains Blocked
Owner: Project owner
Date: 2026-09-03
Baseline: `01dbb1fdce10c197033c1a88dbeeb53afb0a21cd`
Branch: `codex/personal-assistant-v0-selectable-connection-profile-architecture-decision`
Gate: `pa-v0-selectable-connection-profile-decision`

## Objective

Record one closed architecture direction for a future Rust-owned catalog of
local and cloud Personal Assistant connection profiles without making any
profile operational or changing the fixed V0 contracts.

## Accepted disposition

The owner accepted `closed_catalog_direction_selected`. D-119 reserves only a
distinct post-v0 `personal-assistant-selectable-connection-profile-v3` whose
catalog-schema V1 has exactly these ten candidates:

1. `local_no_auth`
2. `google_gemini_oauth`
3. `google_gemini_api_key`
4. `direct_openai_api_key`
5. `direct_openai_workload_identity`
6. `azure_openai_entra`
7. `azure_openai_api_key`
8. `anthropic_api_key`
9. `mistral_api_key`
10. `aws_bedrock_identity`

Every entry remains `candidate_blocked`. The blocked catalog exposes no
selection handle. Direct OpenAI and Azure OpenAI are separate provider
boundaries, and ChatGPT login or subscription access is not OpenAI API OAuth,
authorization, or billing.

## Scope

The increment may change exactly these sixteen documentation paths:

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

No source, test source, dependency, manifest, lockfile, workflow, hook,
capability, CSP, permission, entitlement, toolchain, credential, provider,
signing, or external-system path is in scope.

## Invariants and preservation

- Synthetic-v1 and reserved `real-content-v2` remain fixed, nonselectable, and
  unchanged under D-094. Catalog tests cannot prove either lane.
- Only a future separately admitted entry may receive a Rust-issued opaque
  process- and catalog-generation-bound handle. Rust resolves the complete
  immutable provider/auth/model/endpoint/credential/disclosure/lifecycle tuple.
- There is no default, retry, downgrade, environment inference, ambient
  credential discovery, provider fallback, or local/cloud fallback.
- D-060 keeps all provider credentials gateway-owned, while D-021 additionally
  controls the current OpenAI/gateway contract. Direct/native custody requires
  separately accepted reconciliation of every applicable decision. D-061 exact
  external-processing/ZDR evidence and D-062 Cortexa identity remain
  controlling.
- D-118 remains `no_eligible_client`; no dependency or transport is selected.
  V0-3, V0-7, the live synthetic-text milestone, and every operational
  successor remain `Blocked`.
- Historical D-096 through D-118 remain preserved. D-107 stays eight of eleven,
  D-108 stays additively nine of ten, all ten blockers remain unproved, and
  D-113 through D-117 remain Proposed/non-controlling.
- `NativeAgentRuntime` remains sole/default. The tool set stays empty, and no
  filesystem, persistence, background autonomy, action, or device effect is
  introduced.

## Evidence boundary and discoveries

Every applicable external-provider source in the approved plan was re-opened
on 2026-09-03 through unauthenticated public documentation only.
`local_no_auth` has no provider authentication or applicable vendor source and
remains repository-evidence-only. No provider API, account, sign-in,
credential, model, resource, or external write was accessed.

The named mechanisms remain documented, but the review does not establish
Cortexa eligibility, safe desktop custody, accepted retention, billing, an
approved client, or implementation readiness. Google has a date-ambiguous
September 2026 API-key transition. OpenAI WIF is project service-account
workload identity, not end-user OAuth. Anthropic documents API keys plus WIF,
App Attest, and CLI-specific OAuth; Mistral documents API keys and WIF; AWS
documents IAM/SigV4 identity and Bedrock API keys. Those additional methods are
deliberately excluded from this catalog version and are not described as
unsupported. Missing dates were not invented.

## Security boundary

Future OAuth requires a system browser, authorization code with PKCE S256,
unpredictable state, OIDC nonce when applicable, exact redirect and scope,
bounded one-use attempts, terminal cancellation, cleanup ownership, and late-
result rejection. Provider auth is separate from Cortexa identity and from the
separate external-processing disclosure/admission boundary. Authorization
success never starts model transport; prompt transmission requires a fresh
explicit foreground action and separate one-use model-processing admission.
Sensitive material no longer needed for cleanup is zeroized, while bounded
nonsecret terminal and quarantine ownership state remains until cleanup is
proved or a closed bounded disposition applies. Local discard or late-result
filtering does not prove browser, provider, OS, network, credential source, or
local-engine quiescence. Under D-060, the gateway owns credential-bearing OAuth
state, authorization results, provider tokens, and their cleanup. Trusted
desktop Rust may own such material only after separately accepted direct/native-
custody reconciliation; until then it retains bounded nonsecret coordination
state only and the profile remains blocked. Ambiguity fails closed.

## Progress

- [x] Confirmed the synchronized baseline, free D-119 slot, and unchanged prior
      complete record. Its marker was `valid: false` only because the approved
      untracked plan changed the workspace fingerprint; no historical finalizer
      was run or retargeted.
- [x] Corrected the plan-only gate ID after the original overlength ID was
      rejected before state changed.
- [x] Created the approved branch and began the corrected gate.
- [x] Revalidated every applicable external-provider entry in the official
      source register within the bounded public read-only evidence allowance;
      `local_no_auth` remained repository-evidence-only.
- [x] Recorded the owner-accepted closed disposition and D-119.
- [x] Reconciled the authorized project-memory, architecture, product,
      security, testing, plan, and increment records.
- [x] Complete exact-scope and preservation assertions.
- [x] Complete independent review, session, quality, report, and finalizer
      gates.
- [x] Verify a valid completion marker and stop for owner review.

The first sandboxed finalizer validated the report but could not write ignored
gate state. The identical authorized rerun completed the marker; after this
truthful report correction, the report was revalidated and rebound once.
Owner-authorized same-increment correction then replaced the stale active-gate
wording, narrowed source claims to applicable external providers, and made
OAuth credential-bearing ownership conditional on D-060. The corrected result
was independently re-reviewed; that review found one residual overbroad source
claim in the plan's acceptance checklist, which was narrowed under the same
authorization. The final result was re-reviewed, revalidated, and rebound
without running `begin` or starting a successor.

## Result

**PASS WITH ADVISORIES.** Every required documentation, repository, security,
complete verification, exact-scope, preservation, independent-review, session,
quality, report, and completion gate passed. The exact sixteen-file workspace
has a valid marker. All ten profiles and every operational successor remain
`Blocked`; no implementation or operational authority was created.
