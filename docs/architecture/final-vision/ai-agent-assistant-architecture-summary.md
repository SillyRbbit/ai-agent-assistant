# Cortexa final-vision architecture

Status: Executive and technical architecture vision; not implementation evidence

Evidence date: 2026-07-29
Current-state reconciliation: 2026-08-11 under D-079 and D-082; diagram files
retain their original evidence date and remain target-architecture artifacts

## Deliverables

| Artifact                                        | Purpose                                                            |
| ----------------------------------------------- | ------------------------------------------------------------------ |
| `ai-agent-assistant-executive-architecture.svg` | Editable 16:9 executive visual story designed for a 30-second read |
| `ai-agent-assistant-executive-architecture.png` | High-resolution executive presentation export                      |
| `ai-agent-assistant-technical-architecture.svg` | Detailed, editable 16:9 technical architecture                     |
| `ai-agent-assistant-technical-architecture.png` | High-resolution technical presentation export                      |

The compatibility filenames retain the repository's historical
`ai-agent-assistant` identifier under D-026. Human-facing diagram text uses the
product name `Cortexa`.

## Overall design

Cortexa is designed as a local-first executive assistant. Human intent enters
through a native client and narrow typed IPC. Trusted Rust—not the WebView,
model, gateway, external content, or a prompt—owns validation, risk
classification, deterministic policy, exact approval, restricted execution,
cancellation, and redacted audit.

The completed vision separates five architectural concerns:

1. **User experience:** a native conversational workspace presents requests,
   context provenance, tasks, memory, tool activity, notifications, and approval
   decisions. macOS remains the initial platform. Windows and Linux are planned
   portability targets; iOS/mobile and a web companion remain optional.
2. **Trusted local control plane:** an executive-assistant runtime coordinates
   bounded runs and routes typed proposals through `AgentProvider`,
   `ToolRegistry`, `PolicyEngine`, `ApprovalManager`, a restricted executor,
   `AuditLogger`, `MemoryStore`, and `PlatformAdapter` boundaries.
3. **Platform and integration plane:** reviewed adapters provide
   least-privilege access to calendar, reminders, contacts, files,
   notifications, browser/URL opening, local tasks, and separately approved
   business integrations.
4. **AI and data plane:** approved cloud models sit behind an authenticated
   product gateway. Local encrypted storage holds user-controlled product data.
   Optional local models, retrieval services, semantic indexing, or a vector
   store require separate architecture and security decisions.
5. **Security and governance:** user consent, local policy, exact approval,
   secrets management, encryption, data minimization, retention, audit,
   least-privilege permissions, external-processing disclosure, and incident
   rollback apply across every layer.

## Security and approval model

- The model is an untrusted planner and cannot directly execute a device action.
- The WebView is an untrusted presentation surface and cannot select arbitrary
  commands, providers, models, schemas, credentials, permissions, or execution
  targets.
- Tool identity, schema, risk, and permission requirements come from the local
  trusted registry.
- Deterministic policy decides whether to allow, deny, or require approval.
- Consequential actions require an exact preview and one-time approval bound to
  the canonical proposal.
- Future execution must revalidate the exact target and current permission
  immediately before the effect.
- Audit evidence records normalized lifecycle and outcomes without treating an
  approval or receipt as execution authority.
- Provider credentials stay at the server-side gateway. Desktop session
  credentials remain short-lived, audience-bound, and outside the WebView and
  SQLite.
- No model-to-device, gateway-to-device, or generic WebView-to-device execution
  path is part of the architecture.

## What exists today

- Tauri 2 macOS application shell, window and menu lifecycle, React workspace,
  navigation, Settings diagnostics, Permission Center, Tasks surface, and
  Activity surface.
- Deterministic in-memory mock conversation, streaming, context provenance,
  approval, tool result, Stop, Retry, failure, and final-answer behavior.
- One narrow custom Tauri command, `get_app_info`, and one closed native menu
  event.
- SQLite/SQLCipher-capable storage foundation with private connections,
  immutable migrations, and bootstrap metadata only.
- Transport-free bounded gateway request and event validation.
- Strict local tool schemas for `get_current_datetime@1` and
  `create_local_task@1`.
- Deterministic policy, exact-subject approval, cancellation, native approval
  source, and a bounded volatile approval-audit receipt.
- A disconnected fake-only macOS Keychain status probe with no real credential
  or runtime consumer.

These components do not form a live end-to-end assistant. The React mock loop
and transport-free Rust security contracts are not connected.

## Planned architecture

- A live trusted Rust run coordinator and `AgentProvider`.
- Microsoft personal identity through the system browser with OAuth/OIDC and
  PKCE, subject to the unresolved 15-minute production token requirement and
  callback evidence.
- A Cortexa-operated authenticated product gateway, with Azure Container Apps
  retained as the planned production hosting target.
- Separately approved cloud language-model processing with provider-specific
  ZDR, retention, region, logging, disclosure, and security evidence.
- A restricted dispatcher/executor and reviewed macOS tools.
- Durable local audit, encrypted product repositories, conversation/task
  persistence, user-controlled memory, Permission Center workflows, and
  independently revocable integrations.
- Signed, notarized, installable, upgradeable, diagnosable, and recoverable
  macOS distribution.

The Cloudflare Access and Worker design remains a separate owner-only,
fake-data synthetic-demo boundary. It is not the production authentication or
gateway architecture.

## Optional or conceptual components

The repository does not yet define implementation contracts for these items,
so the diagrams label them optional or conceptual:

- accepted native nine-role catalog: Personal Assistant; Research; Knowledge &
  Document; Coding; QA & Validation; Security & Risk; Cloud Infrastructure;
  Systems Operations; and Workflow Automation, behind a bounded
  application-owned orchestrator with staged activation;
- a web companion;
- mobile clients beyond the documented iOS portability direction;
- local language models;
- retrieval and knowledge services;
- a semantic index or vector database;
- additional clouds, business applications, enterprise identity providers, or
  AI providers.

Any multi-agent design must use typed messages, shared bounded workflow state,
local policy-controlled tools, human approval, cancellation, audit, and
failure containment. It must not grant recursive delegation, hidden background
autonomy, direct device execution, or authority derived from another agent.

## Architectural assumptions

1. **Completed vision, not roadmap authorization:** the diagrams compose the
   accepted target architecture with explicitly requested future concepts. They
   do not make any product increment Ready.
2. **Specialized agents:** D-079 implements the native runtime foundation and
   D-082 accepts a separate native orchestrator with nine named
   application-owned roles. Only Personal Assistant and Research Agent are
   initially selected for a future deterministic flow; the remaining catalog
   roles are staged. No agent definition, registry, activation, task,
   delegation, provider, or UI is implemented yet, so diagrams remain target
   architecture rather than current capability.
3. **Web and mobile:** Windows, Linux, and iOS portability are documented.
   A web companion and broader mobile support are conceptual and do not change
   the requirement that Cortexa is not a browser-only product.
4. **Models and retrieval:** the approved direction is gateway-mediated cloud
   processing. Local models, retrieval services, and vector storage are shown
   only as separately reviewable options.
5. **Cloud portability:** Azure Container Apps is the planned production
   hosting target. Future AWS or Google Cloud portability does not imply
   deployment, active-active multicloud, automatic failover, or provider
   fallback.
6. **Business integrations:** calendar, contacts, files, notifications, and
   local tasks derive from product requirements. Email, SaaS, and other
   consequential external actions require independent OAuth, data, policy,
   approval, audit, and rollback designs.

## Decision anchors

- D-001 and D-004: Tauri/React/Rust architecture and the untrusted-model
  boundary.
- D-010 through D-024 and D-029 through D-042: storage, UI, gateway protocol,
  tool schema, policy, approval, cancellation, and audit ownership.
- D-053: one turn-owned typed approval-audit record.
- D-060 through D-064: identity, hosting, provider, retention, disclosure, and
  staged production gateway boundaries.
- D-066 through D-073: synthetic-demo provider, Cloudflare, Keychain, signed
  identity, and bounded secret-memory constraints.
- D-079 and D-082: implemented native runtime foundation plus accepted bounded
  native multi-agent application-service direction.

## Evidence sources

- `README.md`
- `ARCHITECTURE.md`
- `PRODUCT_REQUIREMENTS.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `DECISIONS.md`
- `PROJECT_STATUS.md`
- `NEXT_STEPS.md`
- `HANDOFF.md`
- `docs/product/PRODUCT_BRIEF.md`
- `docs/product/ARCHITECTURE_BASELINE.md`
- `docs/security/phase4-gateway-configuration-spec.md`
- `docs/security/phase4-gateway-threat-model.md`
- `docs/branding/BRAND_GUIDELINES.md`
- `docs/branding/PRESENTATION_GUIDELINES.md`

No future architecture shown by these artifacts has been implemented,
provisioned, deployed, connected to external traffic, or authorized by the
diagrams.
