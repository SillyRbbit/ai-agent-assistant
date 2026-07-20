# Phase 4 gateway closed configuration specification

Status: Approved design boundary; not implemented or provisioned
Decision authority: D-060, D-061, D-062, D-063, and D-064
Owner: Henry Dang, Founder & Principal Engineer, Cortexa AI
Last updated: 2026-07-19

## Purpose

This specification defines the only approved Phase 1 identity, gateway, Azure,
AI-provider, data, logging, disclosure, and evidence configuration. It is an
input to later separately approved provisioning and implementation increments.
It does not authorize registration, deployment, credentials, networking, or
external processing.

The current application has no identity client, loopback listener, token
exchange, token validator, gateway transport, cloud resource, `AgentProvider`,
AI-provider adapter, provider credential, disclosure UI, or external traffic.

## Configuration closure rules

1. Every value is server- or build-owned. The WebView, user content, model,
   provider response, and arbitrary local configuration cannot select or
   override it.
2. Angle-bracket values identify evidence that cannot exist before no-traffic
   provisioning. They are not wildcard patterns or runtime defaults.
3. A future component must refuse startup or traffic if a required value is
   missing, remains a placeholder, has multiple ambiguous values, or differs
   from approved evidence.
4. Registration IDs, subscription IDs, resource IDs, endpoint names, and
   evidence artifacts are operational configuration. Store them only in an
   approved restricted system; this repository keeps sanitized references.
5. No secret, authorization code, access token, refresh token, managed-identity
   token, certificate private key, or provider credential belongs in source,
   the WebView, SQLite, logs, tests, ordinary CI, or these documents.
6. No environment fallback, alternate origin, automatic provider fallback, or
   caller-supplied endpoint is permitted.

## Evidence stages

### Stage A - design

ARB-002A approves this specification and the companion threat model. No remote
object exists and no network test occurs.

### Stage B - no-traffic provisioning

A separate approved increment may create the intended registrations and Azure
resources with traffic disabled. It records sanitized manifests, owners,
resource relationships, role assignments, network state, and rollback evidence.
It does not authenticate a user or call the gateway or model provider.

### Stage C - synthetic-only transport

A separate approved increment may implement and verify the closed transport
using a project-owner-approved synthetic corpus. Disclosure and acknowledgement
must precede the first external transmission, including synthetic transmission.
Real user content remains prohibited.

### Stage D - real-content activation

A separate activation decision may permit explicitly submitted non-sensitive
text only after every D-061 and provider-specific evidence requirement passes.
No test or deployment automatically advances to this stage.

## Identity registration boundary

### Closed values

| Field                          | Approved value                                                                                  |
| ------------------------------ | ----------------------------------------------------------------------------------------------- |
| Provider ID                    | `microsoft-personal`                                                                            |
| Account type                   | Personal Microsoft accounts only                                                                |
| Authorization authority        | `https://login.microsoftonline.com/consumers`                                                   |
| OIDC discovery source          | `https://login.microsoftonline.com/consumers/v2.0/.well-known/openid-configuration`             |
| OAuth flow                     | Authorization Code Flow with system browser and PKCE S256                                       |
| Desktop registration           | One public native-client registration; no client secret or certificate                          |
| Gateway API registration       | One separate API-resource registration                                                          |
| API Application ID URI         | `api://<gateway-api-client-id>`                                                                 |
| Requested access-token version | `2`                                                                                             |
| Delegated scope name           | `gateway.access`                                                                                |
| Requested delegated scope      | `api://<gateway-api-client-id>/gateway.access`                                                  |
| OIDC scopes                    | `openid email`                                                                                  |
| Excluded scopes                | `profile`, `offline_access`, Microsoft Graph, directory, group, mail, calendar, files, contacts |
| Redirect host                  | `127.0.0.1` only; not `localhost`, wildcard, custom URI scheme, or non-loopback interface       |
| Redirect port                  | Operating-system-assigned ephemeral port                                                        |
| Redirect path                  | `/oauth/callback`                                                                               |
| Access-token maximum lifetime  | 15 minutes                                                                                      |
| External account key           | Provider ID + normalized issuer + subject                                                       |
| Automatic email linking        | Prohibited                                                                                      |
| Persistent session             | Prohibited until a separate decision                                                            |

The registered loopback representation must be supported by the Microsoft app
registration manifest and must match the runtime callback under Microsoft's
native-app port rules. Stage B captures the sanitized manifest. Stage C proves
that the exact `127.0.0.1` callback, ephemeral port, and fixed path work; a need
to use another host, path, scheme, or fixed port stops the increment and
requires a superseding decision.

Microsoft's current redirect guidance requires an HTTP `127.0.0.1` URI to be
added through the application manifest and describes special ephemeral-port
matching explicitly for `localhost`. The project therefore treats both the
manifest representation and ephemeral-port behavior for the approved
IP-literal callback as unproven until Stage B and Stage C evidence passes.
There is no silent fallback to `localhost`, a fixed port, a custom scheme, or
an embedded browser.

### Desktop client responsibilities

- Generate a cryptographically random PKCE verifier/challenge, `state`, and
  OIDC `nonce` for each attempt.
- Open only the configured Microsoft authorization endpoint in the system
  browser.
- Bind the callback listener only to `127.0.0.1` on one ephemeral port before
  browser launch, accept one matching callback, and close it on success,
  cancellation, mismatch, or timeout.
- Reject missing, repeated, malformed, mismatched, expired, or unsolicited
  callbacks without displaying raw provider errors or codes.
- Exchange the code only from trusted Rust against the discovery-derived token
  endpoint and include the exact redirect URI and PKCE verifier.
- Validate the ID-token signature, issuer, audience, expiration, `nonce`, and
  personal-account boundary before deriving local account identity.
- Retain the gateway access token only in Rust process memory and erase the
  reference on sign-out, cancellation, expiry, terminal authentication error,
  or application exit.
- Never send an identity token, authorization code, PKCE verifier, token, or
  provider error through the WebView or store it in SQLite.

Microsoft currently documents a default access-token lifetime varying from
approximately 60 to 90 minutes. That does not satisfy D-060 and D-062's maximum
15-minute Cortexa gateway-token boundary. Stage B must prove an approved
Microsoft personal-account configuration that can enforce the maximum, or a
later additive architecture decision must define a different short-lived
Cortexa gateway-session exchange. Until one path is approved and evidenced,
Stage C authentication is blocked. This specification does not authorize a
Cortexa token issuer or relax the 15-minute maximum.

### Gateway API responsibilities

- Use discovery metadata and signing keys from the one approved personal
  identity authority. Key rollover must be supported without allowing a
  caller-selected metadata URL or issuer.
- Require a supported asymmetric signature algorithm; reject unsigned tokens
  and algorithm/key confusion.
- Validate one literal approved issuer and consumer-account tenant boundary,
  one literal audience, expiration/not-before, maximum token lifetime, and the
  exact `gateway.access` delegated scope.
- Treat the approved audience as
  `api://<gateway-api-client-id>`. Stage C must confirm the token's actual `aud`
  claim. Any Microsoft-issued difference fails closed and requires an additive
  decision before traffic.
- Reject work, school, guest, arbitrary Entra tenant, multi-audience,
  application-only, missing-scope, wrong-scope, expired, future, malformed, or
  replayed credentials.
- Authorize the principal for the exact request after authentication. A valid
  token alone does not authorize a provider, model, tool set, or local action.
- Use bounded clock skew and cache discovery/signing material only under a
  separately reviewed expiry and refresh policy. Failure to refresh fails
  closed for new requests.

### Registration evidence

Stage B must record, without secrets:

- tenant/account-type selection and registration owners;
- desktop and gateway application identifiers in the restricted evidence
  store;
- public-client status and absence of desktop credentials;
- exact redirect manifest entry;
- gateway Application ID URI and `gateway.access` scope definition;
- requested access-token version `2` and the mechanism intended to satisfy the
  maximum 15-minute gateway-token lifetime;
- requested API permissions and proof that excluded scopes are absent;
- access-token version and approved issuer/audience expectations;
- consent model and publisher/branding state;
- creation time, reviewer, evidence hash, and deletion/rollback procedure.

Stage C must add sanitized token-claim evidence from a synthetic account proving
the observed issuer, tenant, audience, scope, token lifetime, and negative
tenant paths. Tokens and authorization codes must never be retained as evidence.

## Gateway deployment boundary

### Closed values

| Field                                 | Approved value                                                            |
| ------------------------------------- | ------------------------------------------------------------------------- |
| Operator                              | Cortexa AI                                                                |
| Hosting platform                      | Microsoft Azure Container Apps                                            |
| Primary region                        | Central US                                                                |
| Production origin                     | `https://api.cortexaai.io`                                                |
| Desktop gateway origins               | Exactly the approved origin for the active environment; no user override  |
| Gateway ingress                       | External HTTPS only; insecure HTTP disabled                               |
| Gateway authentication                | Microsoft personal access token validated by the gateway                  |
| Provider egress                       | Azure OpenAI private endpoint only                                        |
| Azure OpenAI public network           | Disabled before Stage C                                                   |
| Gateway managed identity              | One dedicated non-shared user-assigned identity                           |
| Provider authorization                | `Cognitive Services OpenAI User` at the exact Azure OpenAI resource scope |
| Provider API-key fallback             | Prohibited                                                                |
| Secondary cloud or automatic failover | None                                                                      |

The Container Apps default hostname is not a configured desktop origin. If the
platform keeps it reachable, it must enforce the same authentication,
authorization, limits, and logging controls and cannot become an alternate
unauthenticated route. The reserved production origin remains inactive until
DNS, certificate, TLS, ingress, authentication, authorization, logging,
monitoring, and security evidence pass.

### Network controls

- Place the Container Apps environment in an approved virtual network that can
  resolve and reach the Azure OpenAI private endpoint.
- Disable Azure OpenAI public network access before Stage C.
- Restrict gateway egress to the approved Azure OpenAI private endpoint,
  Microsoft identity discovery/signing-key endpoints, and separately reviewed
  platform-operational dependencies. Any new destination requires review.
- Do not permit arbitrary URL fetch, redirect following to unapproved origins,
  proxy configuration, caller-supplied endpoint, hosted web search, MCP, or
  provider tools.
- Configure the custom DNS record and certificate only in Stage B under the
  named DNS and certificate owner. HTTPS validation cannot be disabled.
- Do not add certificate private keys to the repository, application bundle,
  desktop, SQLite, ordinary CI, or gateway image.
- Apply bounded request size, connection, rate, concurrency, model-turn,
  function-call, retry, idle, provider, and total deadlines at the gateway.

### Managed identity and RBAC

- The gateway receives one dedicated user-assigned managed identity. It is not
  shared with unrelated applications or environments.
- Assign only `Cognitive Services OpenAI User` at the exact Azure OpenAI
  resource, not subscription or resource-group scope.
- Do not grant Owner, Contributor, User Access Administrator, account-key read,
  deployment-management, or unrelated data-plane roles to the runtime identity.
- Deployment automation and runtime identity are separate principals.
- Stage B records the principal ID, assignment scope, role definition, owners,
  and negative proof that API-key fallback and broader role assignments are
  absent. Identifiers remain in the restricted evidence store.
- Stage C proves successful managed-identity inference and denial after the
  role is removed or the request targets an unapproved resource.

## Azure OpenAI provider boundary

### Closed values

| Field                              | Approved value                                                                                            |
| ---------------------------------- | --------------------------------------------------------------------------------------------------------- |
| Provider                           | Azure OpenAI in Microsoft Foundry                                                                         |
| Deployment type                    | One Standard/Regional deployment                                                                          |
| Region                             | Central US                                                                                                |
| Model                              | Deployment-time approved candidate; D-063 currently names `gpt-5.1` version `2025-11-13` for revalidation |
| API shape                          | Foreground Responses streaming                                                                            |
| Response storage                   | `store: false` forced by gateway                                                                          |
| Background mode                    | `background: false` forced by gateway                                                                     |
| Tool mode                          | Strict custom functions only                                                                              |
| Parallel function calls            | Disabled                                                                                                  |
| Hosted tools and stateful features | Prohibited                                                                                                |
| Automatic provider/model fallback  | Prohibited                                                                                                |

The gateway owns the provider endpoint, deployment, model/version, parameters,
tool-set version, and authorization. Desktop input cannot add arbitrary tools,
hosted features, provider parameters, files, retrieval, Agents, Assistants,
Batch, stored completions, web search, MCP, code execution, or response
retrieval.

### Provider-specific privacy evidence

D-061 is not satisfied by this specification, `store: false`, intended RBAC,
or a portal screenshot alone. Before Stage D, the restricted evidence set must
identify the exact intended production subscription, Azure OpenAI resource,
endpoint, deployment, model, version, region, agreement, and responsible
owners, and must include:

- Microsoft/provider approval for the required Zero Data Retention or modified
  abuse-monitoring status for that exact configuration;
- resource evidence that `ContentLogging=false` is effective;
- documented proof that the selected Responses request and excluded features
  create no application-state retention;
- provider terms and effective date covering data use, human review, abuse
  monitoring, region, and deletion;
- successful synthetic verification of forced `store: false`,
  `background: false`, strict functions, disabled parallel calls, and excluded
  features;
- access review, managed-identity/RBAC evidence, network-isolation evidence,
  logging-field review, deletion procedure, incident-response procedure, and
  approval signatures from the privacy and security owner.

Evidence for another subscription, resource, deployment, model, version,
region, provider, or commercial arrangement does not transfer. If a required
provider assurance is unavailable, Stage D remains blocked; severity is not
lowered and the product must not claim ZDR.

## Data and disclosure boundary

### Data classes

- Stages A and B transmit no content.
- Stage C permits only owner-approved synthetic text created for testing.
- Stage D initially permits only text the user explicitly submits and that is
  classified as non-sensitive.
- Credentials and secrets, attachments, regulated data, financial or
  healthcare data, sensitive personal data, clipboard content, local files,
  memory, tool results, and automatically collected context remain prohibited.

Classification must occur before transmission. An uncertain classification
fails closed. Prompt or model instructions cannot change the classification.

### Disclosure and acknowledgement

- A versioned external-processing disclosure must identify Cortexa's gateway,
  Microsoft Azure OpenAI processing, the active data-class restriction,
  prohibited data, applicable retention posture, and how to cancel.
- The disclosure must appear before the first external transmission, including
  Stage C synthetic traffic, and remain accessible in Settings.
- The user must explicitly acknowledge the current disclosure version. Absence,
  dismissal, version mismatch, or failed persistence denies transmission.
- A material provider, model-retention, region, data-classification, or
  disclosure change invalidates the prior acknowledgement and requires a new
  one.
- Exact user-facing copy, accessibility review, and acknowledgement storage are
  Stage C design evidence. This document does not implement or approve a UI or
  persistence mechanism.

## Logging, errors, and audit

- Gateway logs may contain only authentication outcome, opaque principal and
  correlation IDs, contract/tool/model versions, timing, status, rate-limit
  metadata, and aggregate usage.
- Prompts, model output, function arguments, tool results, attachments,
  authorization headers, tokens, provider response bodies, stack traces, and
  raw errors are prohibited in logs, traces, metrics, support artifacts, and
  crash reports.
- Operational metadata retention is at most seven days and requires automatic
  deletion plus access and deletion evidence before Stage C.
- Provider and gateway failures cross to trusted Rust only as closed redacted
  codes, retryability, bounded retry delay, and opaque correlation IDs.
- Gateway operational logs and future local trusted audit remain separate.
  Neither grants approval or execution authority.

## Provisioning and activation evidence register

Each evidence item must include an ID, stage, environment, exact object scope,
owner, reviewer, collection time, authoritative source, sanitized hash or
reference, pass/fail result, expiration/revalidation trigger, and rollback
status. Required groups are:

1. Identity registration and consent configuration.
2. Token claim and negative validation tests.
3. Container Apps resource, image, ingress, environment, and ownership.
4. DNS, certificate, and TLS validation.
5. Managed identity and least-privilege RBAC.
6. Virtual network, private endpoint, DNS resolution, and egress controls.
7. Azure OpenAI resource, deployment, region, model, and API configuration.
8. ZDR, abuse-monitoring, `ContentLogging=false`, and stateless Responses.
9. Disclosure, acknowledgement, data classification, logging, and deletion.
10. Threat-model security tests, incident response, shutdown, and rollback.

An expired, missing, mismatched, or failed item blocks the dependent stage.

## Normative external references

These references inform the design but do not prove Cortexa operational state:

- [Microsoft identity platform authorization code flow and PKCE](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-auth-code-flow)
- [Microsoft identity platform redirect URI restrictions](https://learn.microsoft.com/en-us/entra/identity-platform/reply-url)
- [Microsoft identity platform access-token validation](https://learn.microsoft.com/en-us/entra/identity-platform/access-tokens)
- [Azure Container Apps ingress](https://learn.microsoft.com/en-us/azure/container-apps/ingress-overview)
- [Azure Container Apps security](https://learn.microsoft.com/en-us/azure/container-apps/security)
- [Azure OpenAI keyless connections](https://learn.microsoft.com/en-us/azure/developer/ai/keyless-connections)
- [Azure OpenAI data privacy](https://learn.microsoft.com/en-us/azure/foundry/responsible-ai/openai/data-privacy)
- [Azure OpenAI limited access and modified abuse monitoring](https://learn.microsoft.com/en-us/Azure/foundry/responsible-ai/openai/limited-access)

## Change control

Any change to identity provider, authority, registration type, audience, scope,
redirect host/path, token lifetime, gateway origin, hosting platform, region,
managed-identity type, RBAC scope, provider, endpoint exposure, model feature,
data class, retention, disclosure behavior, or evidence requirement needs a
reviewed additive decision before implementation. Runtime convenience cannot
weaken this specification.
