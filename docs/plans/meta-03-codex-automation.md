# Meta Increment 3 - Codex automation and post-increment quality gates

Status: Verified complete; uncommitted and unpublished
Owner: Project maintainer
Last updated: 2026-07-15

## Goal

Extend the verified repository-local post-increment system with modular safe
validation, focused review skills, reusable prompts, and review templates that
reduce repeated prompting while preserving explicit human approval and all
existing completion controls.

## User-visible outcome

No application behavior changes. Maintainers and coding assistants gain
discoverable architecture, security, debt, readiness, quality, executive, and
release review workflows plus a deterministic session-end repository inventory.

## Verified baseline

- `main` and `origin/main` were clean and synchronized at `805efc1`.
- The `meta-02` completion marker was complete and valid.
- Codex CLI 0.144.2 reported the `hooks` feature stable and enabled.
- The current official Codex manual confirmed trusted project-local
  `.codex/hooks.json`, command-based Stop hooks, no Stop matcher, Git-root path
  resolution, and `/hooks` trust management.
- The installed binary and live repository workflow support the existing
  `decision: block` continuation contract.
- `npm run test:hooks` passed all 17 baseline tests.
- The project owner approved the exact file plan before the `meta-03` gate began.

## Exact files

Created:

```text
.agents/skills/architecture-review/SKILL.md
.agents/skills/readiness-review/SKILL.md
.agents/skills/technical-debt/SKILL.md
.agents/skills/quality-gate/SKILL.md
.agents/skills/executive-review/SKILL.md
.agents/skills/release-review/SKILL.md
.codex/hooks/common.py
.codex/hooks/session_end_gate.py
.codex/hooks/tests/test_common.py
.codex/hooks/tests/test_session_end_gate.py
docs/templates/READINESS_REVIEW_TEMPLATE.md
docs/templates/SECURITY_REVIEW_TEMPLATE.md
docs/reviews/.gitkeep
prompts/architecture-review.md
prompts/readiness-review.md
prompts/technical-debt-review.md
prompts/quality-gate.md
prompts/post-increment-gate.md
prompts/release-review.md
prompts/executive-review.md
docs/plans/meta-03-codex-automation.md
docs/increments/meta-03-codex-automation.md
docs/reviews/2026-07-15-meta-03-post-increment-review.md
```

Modified:

```text
.agents/skills/security-review/SKILL.md
.agents/skills/post-increment-gate/SKILL.md
.codex/hooks/post_increment_gate.py
.codex/hooks/tests/test_post_increment_gate.py
docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md
prompts/security-review.md
prompts/README.md
AGENTS.md
ARCHITECTURE.md
ASSISTANT_USAGE.md
CODE_REVIEW.md
CONTRIBUTING.md
ENGINEERING_GUIDE.md
SECURITY.md
TESTING_GUIDE.md
docs/workflows/END_SESSION.md
HANDOFF.md
PROJECT_STATUS.md
NEXT_STEPS.md
ROADMAP.md
CHANGELOG.md
DECISIONS.md
PLANS.md
docs/plans/README.md
docs/reviews/README.md
```

Renamed and updated:

```text
docs/plans/meta-03-verified-application-icon-rollout.md
docs/plans/meta-04-verified-application-icon-rollout.md
```

No other tracked path may change without renewed project-owner approval.

## Implementation

1. Keep the supported `.codex/hooks.json` Stop definition unchanged.
2. Extract shared bounded Git, path, JSON, conflict, and suspicious-path logic
   into `common.py` without changing marker or report semantics.
3. Add a read-only `session_end_gate.py` inventory for staged, unstaged,
   untracked, and conflicted paths with explicit exit codes.
4. Preserve and extend post-increment tests for missing reports, failed checks,
   pending manual gates, PASS, PASS WITH ADVISORIES, path containment, conflicts,
   deletion stability, stale content, and loop prevention.
5. Add concise repository-local skills, matching prompts, and review templates
   that reference authoritative policy instead of duplicating it.
6. Reconcile current governance, Git naming and PR policy, and the Meta 3/4
   queue without changing product scope.

## Security and privacy

- Hooks use Python's standard library only and make no network request.
- Hook input, report content, and paths remain untrusted and bounded.
- Scripts invoke only fixed Git inspection commands with argument arrays.
- The session-end inventory is read-only and cannot run report content,
  verification commands, commits, pushes, or product actions.
- No transcript, prompt, model output, personal content, environment value,
  database, credential, private key, or arbitrary command is read.
- Hooks and markers remain developer workflow controls, not security,
  authorization, approval, audit, or execution evidence.

## Risks

- Refactoring shared primitives could change report validation, deletion-stable
  fingerprints, Stop continuation, or explicit exit behavior.
- Overlapping skills could duplicate authority or produce inconsistent results.
- Renumbering could leave stale live links or competing Ready items.
- A template could imply that recorded prose proves a command ran.

Focused regressions, one unchanged hook definition, document precedence,
closed result vocabularies, link auditing, and the mandatory gate mitigate these
risks.

## Explicit non-goals

- Product source, UI, Rust, TypeScript, Tauri, SQLite, schema, migration,
  dependency, manifest, lockfile, capability, CSP, entitlement, or permission
  changes.
- Network access, transcript parsing, generic command execution, automatic test
  execution from hooks, automatic fixes, commits, pushes, merges, releases, or
  roadmap reordering.
- Application-icon generation or Meta Increment 4 implementation.
- Increment 4V or any product capability implementation.

## Verification

```bash
python3 -m json.tool .codex/hooks.json
python3 -m unittest discover -s .codex/hooks/tests -p 'test_*.py' -v
npm run test:hooks
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
npm run format:check
npm run verify
git diff --check
```

Also validate all eight changed or created skills with the official local skill
validator, audit Markdown links and referenced paths, confirm the exact approved
inventory, prove application source and protected configuration remain
unchanged, review the complete diff, and finalize a valid `meta-03` marker.

No application manual check is required because no product or native behavior
changes. Hook behavior is covered by deterministic tests and the unchanged live
Stop definition.

## Rollback

Before commit, restore the existing gate, skill, prompt, template, and governance
files, remove the new automation files, and rename the icon plan back to Meta
Increment 3. After commit, revert one bounded Meta Increment 3 commit. Ignored
gate state may be restarted; no product data, migration, credential, dependency,
identifier, or remote resource requires rollback.

## Acceptance criteria

- [x] Exact approved scope is preserved.
- [x] Installed Codex capabilities and hook format are documented from verified evidence.
- [x] All requested review skills, prompts, scripts, and templates exist without duplicating existing skills.
- [x] Existing Stop, report, marker, deletion, path, conflict, and loop behavior remains covered and passing.
- [x] Failed required checks, pending mandatory manual checks, and blocking Critical or High findings cannot complete the gate.
- [x] Hooks do not use network access, modify product source, commit, push, or begin another increment.
- [x] Git workflow documentation matches the project-owner naming and PR policy.
- [x] Meta Increment 4 is Ready but unimplemented.
- [x] Full verification, complete diff review, documentation sync, and mandatory gate pass.
