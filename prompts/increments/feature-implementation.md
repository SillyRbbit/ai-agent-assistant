# Feature Implementation

- **Category:** Increment
- **Purpose:** Turn one requested product capability into a bounded, reviewable feature increment.
- **Use when:** A feature has explicit users, observable behavior, acceptance criteria, and protected boundaries but is not yet an approved Ready plan.
- **Do not use when:** The task is a defect, behavior-preserving refactor, advisory remediation, or unbounded product vision.
- **Required inputs:** `{{FEATURE_NAME}}`, `{{ACCEPTANCE_CRITERIA}}`, `{{NON_GOALS}}`, and `{{BRANCH_NAME}}`.
- **Expected outputs:** A readiness proposal followed, after approval, by one tested feature increment and verified closeout.
- **Related skills:** `$readiness-review`, `$verified-increment`, `$security-review`.
- **Related prompts:** [Verified increment](verified-increment.md), [Readiness review](../reviews/readiness-review.md), [Refactor](refactor.md).
- **Last reviewed:** 2026-07-18

## Prompt

```text
Use $verified-increment for feature {{FEATURE_NAME}}.

Acceptance criteria:
{{ACCEPTANCE_CRITERIA}}

Explicit non-goals:
{{NON_GOALS}}

Use branch {{BRANCH_NAME}} only after approval. First read AGENTS.md and the applicable engineering, product, architecture, security, testing, roadmap, and project-memory documents. Inspect existing source and tests before proposing an implementation.

Define one user-visible outcome, exact source/test/closeout files, data and trust-boundary impact, dependencies, risks, manual gates, verification commands, and rollback. Confirm the feature does not conflict with the current queue or an unresolved architecture decision. Wait for project-owner approval before creating the branch, beginning the gate, or editing.

After approval, implement only the accepted plan. Keep presentation, trusted-core, IPC, persistence, platform, and network ownership aligned with ARCHITECTURE.md. Add focused success, failure, boundary, and regression tests. Preserve all unrelated behavior and stop before any scope expansion.

Follow the Risk-Based Validation Policy in AGENTS.md and ENGINEERING_GUIDE.md: use focused checks during implementation, then run the complete required completion-gate verification for the selected tier once after the final relevant edit. Cross-cutting, security-sensitive, dependency, Tauri-configuration, and release work still requires npm run verify plus applicable manual checks. Run the post-increment gate. Do not commit, push, merge, release, or begin another increment without separate direction.
```
