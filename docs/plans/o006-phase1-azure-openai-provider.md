# O-006 Phase 1 Azure OpenAI provider decision

Status: Active documentation-only decision increment
Owner: Project owner
Date: 2026-07-19
Baseline: clean synchronized `main` at `30ae547`
Gate ID: `o006-phase1-azure-openai-provider-decision`

## Goal

Record Azure OpenAI in Microsoft Foundry as the sole Phase 1 AI-provider
candidate for synthetic evaluation while preserving the provider-neutral
gateway boundary and every live-traffic block.

## Approved direction

- Planned service: Azure OpenAI in Microsoft Foundry.
- Planned deployment: one Standard/Regional deployment in Central US.
- Synthetic-evaluation model candidate: `gpt-5.1`, version `2025-11-13`, subject
  to exact deployment-time revalidation.
- Gateway authentication: Azure managed identity and least-privilege RBAC; no
  provider API key on the desktop or in ordinary configuration.
- API profile: foreground Responses streaming, `store: false`, `background:
false`, strict custom functions, and no parallel function calls.
- Direct OpenAI and other providers remain separately approved future adapters;
  automatic provider fallback is prohibited.

## Exact scope

Modified: `AGENTS.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `DECISIONS.md`,
`HANDOFF.md`, `NEXT_STEPS.md`, `PLANS.md`, `PRODUCT_REQUIREMENTS.md`,
`PROJECT_STATUS.md`, `ROADMAP.md`, `SECURITY.md`, `SECURITY_CHECKLIST.md`,
`docs/plans/README.md`, and
`docs/reviews/2026-07-16-advisory-remediation-backlog.md`.

Created: this plan,
`docs/increments/o006-phase1-azure-openai-provider-decision.md`, and
`docs/reviews/2026-07-19-o006-phase1-azure-openai-provider-decision-post-increment-review.md`.

## Non-goals

- No application source, tests, dependency, lockfile, workflow, hook, skill,
  Tauri, IPC, CSP, capability, permission, SQLite, or runtime change.
- No Azure resource, deployment, DNS, TLS, identity integration, managed
  identity, RBAC assignment, credential, Keychain, network client,
  `AgentProvider`, disclosure UI, or provider call.
- No real user content, production approval, ZDR claim, automatic fallback,
  direct OpenAI integration, or ARB-002 implementation.
- No hosted tools, files, retrieval, Agents, Assistants, Batch, web search, MCP,
  code execution, or stored response behavior.

## Risks

- Treating a candidate model or published regional availability as deployment
  evidence.
- Treating `store: false` as ZDR or overlooking provider feature persistence.
- Treating Azure alignment as permission to create resources or enable traffic.
- Coupling the desktop protocol to Azure or silently falling back to another
  provider with different retention terms.

## Official evidence reviewed

- [Microsoft Foundry data privacy and security](https://learn.microsoft.com/en-us/azure/foundry/responsible-ai/openai/data-privacy)
  documents resource-level abuse-monitoring controls and the
  `ContentLogging=false` verification signal.
- [Azure OpenAI managed-identity security building blocks](https://learn.microsoft.com/en-us/azure/developer/ai/get-started-securing-your-ai-app)
  document Azure Container Apps, managed identity, Azure RBAC, and Responses
  integration without a provider API key in the application.

These public documents support the architecture decision but do not prove an
exact future Cortexa resource, deployment, contract, or ZDR approval.

## Verification

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.toml Cargo.lock .github .codex .agents scripts
rg -n "D-063|Azure OpenAI|managed identity|ContentLogging=false|synthetic|automatic fallback|ARB-002" AGENTS.md ARCHITECTURE.md PRODUCT_REQUIREMENTS.md SECURITY.md SECURITY_CHECKLIST.md DECISIONS.md HANDOFF.md PROJECT_STATUS.md NEXT_STEPS.md PLANS.md ROADMAP.md
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

Application tests, builds, Azure, provider, networking, identity, managed
identity, RBAC, ZDR, and native checks are not run because this increment changes
documentation only.

## Rollback

Before publication, restore only the exact 17 documentation paths. After
publication, revert the bounded documentation commit; any later change to D-063
must be an additive superseding decision.

## Acceptance criteria

- D-063 records the provider, region, authentication, API profile, exclusions,
  and provider-neutral future boundary.
- Exact D-061 evidence remains mandatory before real user content.
- Current absence and ARB-002's High unresolved status remain explicit.
- The exact 17-path documentation scope passes all required checks and has a
  valid completion marker.
