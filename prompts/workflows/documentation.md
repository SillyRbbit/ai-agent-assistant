# Documentation Synchronization

- **Category:** Workflow
- **Purpose:** Reconcile current documentation with actual Git, source, test, and publication evidence.
- **Use when:** Project memory or active guidance is stale after verified work or publication.
- **Do not use when:** Runtime behavior needs modification or evidence is unavailable.
- **Required inputs:** `{{DOCUMENTATION_SCOPE}}` and the commits, diffs, checks, or reports that prove the current state.
- **Expected outputs:** Minimal evidence-backed documentation changes, preserved history, verified links, and an exact next prompt.
- **Related skills:** `$documentation-sync`, `$code-review`.
- **Related prompts:** [End session](end-session.md), [Repository health](repository-health.md), [Review template](../templates/review-template.md).
- **Last reviewed:** 2026-07-17

## Prompt

```text
Use $documentation-sync for {{DOCUMENTATION_SCOPE}}.

Read AGENTS.md and inspect Git status, recent commits or diff, actual verification output, and every affected current-state document. Update only facts supported by repository evidence.

Preserve historical decisions, troubleshooting records, completed plans, dated reports, and prior handoff evidence. Mark information superseded or add a current publication record rather than rewriting history.

Update HANDOFF.md, PROJECT_STATUS.md, NEXT_STEPS.md, CHANGELOG.md, PLANS.md, and applicable decisions, troubleshooting, plans, increments, workflows, and indexes only where evidence requires it. Include an exact next prompt and distinguish Passed, Failed, Not run, and Manual verification pending.

Verify Markdown formatting, internal links, referenced paths, and protected source scope. Do not modify runtime source, invent completion evidence, commit, push, merge, or begin another increment.
```
