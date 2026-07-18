# Code review guide

Review changes for correctness, security, maintainability, and evidence. Report concrete findings before general commentary.

Use `ENGINEERING_GUIDE.md` for the operating model, `ARCHITECTURE.md` for actual
module boundaries, `TESTING_GUIDE.md` for evidence, and
`SECURITY_CHECKLIST.md` for security-sensitive changes. A planned architecture
or requirement is not evidence that production behavior exists.

## Review order

1. User-visible correctness and regressions.
2. Security and trust-boundary violations.
3. Data loss, privacy, and logging risks.
4. Error handling, cancellation, and failure states.
5. Test coverage and verification gaps.
6. Portability and architecture boundaries.
7. Documentation drift.
8. Style and naming issues that affect maintainability.

## Required checks

### TypeScript and React

- Strict typing remains enabled.
- External or IPC data is narrowed from `unknown`.
- Components do not bypass typed infrastructure clients.
- Async state handles loading, success, failure, and cancellation.
- Tests cover meaningful state transitions and user interaction.

### Rust and Tauri

- Production code contains no `unwrap`, `expect`, `panic!`, `todo!`, or `unimplemented!`.
- Errors preserve useful context without exposing secrets.
- Tauri commands are narrow and explicitly registered.
- New capabilities are least-privilege and justified.
- Platform-specific behavior does not leak into portable domain logic.
- Mutations are not silently retried after an unknown result.

### Agent and tool behavior

- Model output is not used as authorization.
- Tool names and arguments are strictly validated.
- Policy is deterministic.
- Approval state is bound to exact normalized arguments.
- Audit records are complete but redacted.
- Untrusted content cannot override user or system authority.

### Persistence

- Migrations are versioned and tested.
- Foreign keys and transactions are used correctly.
- Sensitive credentials are not stored in SQLite.
- Schema changes have rollback or compatibility reasoning.

### GitHub and repository automation

- Workflow triggers, permissions, credentials, action digests, runner selection,
  timeouts, and commands are explicit and least-privilege.
- Pull-request workflows receive no secrets and never use
  `pull_request_target` for untrusted code.
- Pull-request jobs use ephemeral hosted runners. Any future persistent runner
  use requires a separately reviewed trust policy.
- Path filters and change classification include every current source,
  dependency, workflow, Tauri, IPC, storage, migration, permission, and
  security-sensitive path. Unknown non-documentation paths fail closed.
- No workflow commits, pushes, merges, publishes, deploys, signs, notarizes, or
  auto-merges.
- Dependabot changes remain review-only proposals.
- Repository-health checks test accepted and rejected cases and do not expose a
  matched secret value.
- CODEOWNERS, labels, milestones, and badges are not mistaken for remote policy
  enforcement.
- Conditional path-filtered workflows are required only when applicable; a
  skipped workflow is not mistaken for a universal branch-protection check.

## Verification evidence

A review should name the commands that ran and their outcomes. Missing platform checks must be identified explicitly.

Expected repository checks:

```bash
npm run format:check
npm run lint
npm run typecheck
npm run test
npm run build
npm run docs:check
npm run repository:check
npm run tauri -- build --no-bundle
```

Before completing an implementation increment, run
`python3 .codex/hooks/session_end_gate.py`, then `$quality-gate`, then
`$post-increment-gate`. The consolidated report must inventory the complete
change set, classify every required automated and manual check, record
architecture/security/code-health/debt/readiness findings, and finish with
exactly `PASS`, `PASS WITH ADVISORIES`, or `FAIL`.

Critical or High findings that block completion, failed required checks, merge conflicts, and pending required manual checks require `FAIL`. Do not automatically fix advisories or reorder the roadmap during the review.

The component reviews may be invoked independently through
`$architecture-review`, `$security-review`, `$code-review`,
`$technical-debt`, and `$readiness-review`. `$quality-gate` composes their
evidence; it does not replace the source authorities or prove that commands ran.

When repository skills are unavailable, use the matching fallback under
`prompts/reviews/`; `prompts/README.md` is the authoritative selection index.
Review prompt changes for metadata completeness, placeholder clarity, active
path validity, conceptual overlap, approval boundaries, and preservation of
useful migrated content.

For documentation-only changes, review factual consistency against source and
tests, internal links and paths, authority and supersession, protected source
scope, and whether current, mocked, planned, and prohibited behavior are clearly
distinguished.

## Finding format

Use this structure for each substantive issue:

```text
[Severity] Short title
Location: path:line
Why it matters: concrete failure or risk
Evidence: code path, reproduction, or missing invariant
Recommended fix: smallest safe correction
```

Severity guide:

- **Critical** — credential exposure, unauthorized execution, destructive action, or release-blocking compromise.
- **High** — likely security boundary bypass, data corruption, or major user-visible failure.
- **Medium** — real defect with bounded impact or missing failure handling.
- **Low** — maintainability or documentation issue that can cause future defects.
- **Advisory** — non-blocking improvement or explicitly accepted residual risk.

Do not report speculative findings without showing the affected path or invariant.
