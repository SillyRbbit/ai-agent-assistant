# O-006 Phase 1 Azure OpenAI provider decision

Status: Verified complete with advisories; publication pending
Date: 2026-07-19
Owner: Project owner
Baseline: clean synchronized `main` at `30ae547`
Gate ID: `o006-phase1-azure-openai-provider-decision`

## Goal

Record Azure OpenAI in Microsoft Foundry as the sole Phase 1 provider candidate
for synthetic evaluation without authorizing implementation or external
processing.

## Decision boundary

D-063 selects one planned Standard/Regional Central US Azure OpenAI deployment,
managed-identity authentication, least-privilege RBAC, and a bounded foreground
Responses profile. `gpt-5.1` version `2025-11-13` is a revalidation-bound
synthetic-evaluation candidate, not a production commitment.

Direct OpenAI and other providers remain future separately approved adapters.
No automatic fallback is permitted. D-061 evidence remains mandatory for the
exact deployed configuration before real user content.

## Current boundary

No gateway, cloud resource, network client, provider adapter, `AgentProvider`,
managed identity, RBAC assignment, credential, disclosure UI, or external
processing exists. Synthetic-only describes the maximum data class for a later
approved transport test, not a current network capability.

## Verification status

Passed: documentation formatting and links, repository policy, secret scan,
whitespace, exact 17-path scope, protected-path review, historical decision and
report preservation, provider-state consistency, complete diff review,
session-end inspection, and mandatory post-increment gate.

Failed and corrected: the first documentation check found only Prettier
formatting in `SECURITY_CHECKLIST.md`. The formatter corrected it; all final
checks passed.

Not run: frontend tests, Rust tests, application builds, native launch, Azure,
identity, managed identity, RBAC, provider, network, and operational ZDR checks.
They are outside this documentation-only tier. Manual verification pending:
none; the project owner approved the decision and file plan.

Result: `PASS WITH ADVISORIES`. ARB-002 remains the pre-existing High blocker
for any implementation or live external processing.

## Rollback

Restore only the exact documentation scope before publication. After
publication, supersede D-063 additively if the provider decision changes.
