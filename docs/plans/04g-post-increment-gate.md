# Repository Workflow Increment 4G - Post-increment gate

Status: Complete
Owner: Project maintainer
Last updated: 2026-07-14

## Goal

Add one repository-local Codex Stop hook and consolidated post-increment workflow that prevents an active implementation increment from ending without current verification evidence, engineering review, synchronized project memory, a passing report, and a deterministic completion marker.

## User-visible outcome

- A trusted Codex project hook requests `Run $post-increment-gate for the active increment before ending the session.` when completion evidence is absent or stale.
- `$post-increment-gate` produces one review covering verification, architecture, security, code health, technical debt, roadmap readiness, documentation, exact files, and exact commands.
- Application behavior and compatibility identifiers remain unchanged.

## Scope

- Add a repository-local Stop hook configuration.
- Add one Python standard-library validator with explicit begin, finalize, status, and Stop-hook modes.
- Add focused deterministic tests and include them in `npm run verify`.
- Add a structured report template, generated-report directory, and 4G bootstrap report.
- Integrate the workflow with repository instructions, review/security policy, end-session workflow, verified-increment skill, and project memory.

## Explicit non-goals

- No application, frontend, Rust, Tauri, IPC, capability, CSP, permission, persistence, database, network, provider, gateway, approval, audit, execution, package-name, crate-name, executable, bundle-ID, or storage change.
- No external dependency, transcript parser, arbitrary command runner, network request, automatic commit, automatic push, advisory auto-fix, or roadmap reorder.
- No claim that a user-disabled or untrusted hook is an unbypassable security control.

## Existing behavior and constraints

- `AGENTS.md` mandates `$post-increment-gate`, but Increment 4F closed under D-027 because no skill or hook existed.
- Codex CLI 0.144.2 reports `hooks` stable and enabled by default.
- Project-local hooks load from `.codex/hooks.json` only after normal trust review.
- Stop input includes `stop_hook_active`; a blocking Stop response requires `decision: block` and a non-empty continuation `reason`.
- The repository had no `.codex/` tree at baseline.

## Files expected to change

Created:

```text
.agents/skills/post-increment-gate/SKILL.md
.codex/hooks.json
.codex/hooks/post_increment_gate.py
.codex/hooks/tests/test_post_increment_gate.py
docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md
docs/reviews/README.md
docs/reviews/2026-07-14-04g-post-increment-review.md
docs/increments/04g-post-increment-gate.md
docs/plans/04g-post-increment-gate.md
```

Modified:

```text
.gitignore
package.json
.agents/skills/verified-increment/SKILL.md
AGENTS.md
CODE_REVIEW.md
SECURITY.md
docs/workflows/END_SESSION.md
prompts/end-of-session-handoff.md
HANDOFF.md
PROJECT_STATUS.md
NEXT_STEPS.md
CHANGELOG.md
DECISIONS.md
PLANS.md
docs/plans/README.md
```

Runtime-generated and ignored:

```text
.codex/state/post_increment_gate.json
```

## Implementation steps

- [x] Confirm clean merged `main`, current Codex hook support, absent `.codex` layout, Python availability, and formatting baseline.
- [x] Create the approved branch and bootstrap one active ignored 4G state.
- [x] Implement strict report, path, state, conflict, suspicious-file, fingerprint, and Stop-hook validation.
- [x] Add repository hook configuration, report template, review directory, and focused tests.
- [x] Integrate hook tests into `npm run verify` and wire the skills and durable review/security/session rules.
- [x] Run focused and complete verification plus code/security/scope review.
- [x] Complete normal Codex hook trust and live Stop behavior confirmation.
- [x] Synchronize project memory and create the passing 4G report.
- [x] Update passing manual evidence and finalize the completion marker.

## Security and privacy considerations

- Hook input and report metadata are untrusted and size-bounded.
- Paths must remain relative to the resolved Git root; traversal, `.git`, symlink escapes, non-UTF-8 paths, and unsupported file types fail closed.
- The validator executes only fixed Git inspection commands with argument arrays and never evaluates report content as code.
- The script does not read transcripts, model messages, raw tool data, environment values, databases, or network resources.
- State is ignored, atomically written with mode 0600, content-fingerprinted, and explicitly non-authorizing.
- Merge conflicts, stale report hashes, changed workspace content, suspicious changed paths, failed required checks, pending required manual checks, and blocking Critical/High findings prevent completion.
- `stop_hook_active` allows the continuation turn to stop without recursively re-triggering another continuation request; it does not create completion evidence.

## Risks

- A hook that is not trusted or is explicitly disabled cannot enforce the workflow. Documentation must require normal trust and disclose emergency bypass behavior.
- Markdown prose cannot prove a command ran. The skill must execute each command and the structured manifest can only check consistency with recorded evidence.
- A stale local marker could approve later content. The marker is bound to report hash and a deterministic workspace-content fingerprint.
- A fresh clone has no ignored active state. `$verified-increment` and `AGENTS.md` must run `begin` before editing; a dirty repository without state also triggers continuation.
- Overbroad file detection could block legitimate work. The deterministic blocklist is limited to changed paths with clear generated, credential, certificate, environment, database, build, or log patterns.

## Verification commands

```bash
python3 -m json.tool .codex/hooks.json
python3 -m unittest discover -s .codex/hooks/tests -p 'test_*.py' -v
npm run test:hooks
npm run format:check
npm run lint
npm run typecheck
npm run test
npm run build
npm run tauri -- build --no-bundle
npm run verify
python3 .codex/hooks/post_increment_gate.py status
git diff --check
git status --short --branch
```

Manual verification:

- Start or reload Codex from the repository.
- Open `/hooks`, review the exact project hook, and trust it normally.
- Confirm an active increment without a report requests the exact continuation prompt.

Focused automated tests confirm that a valid completed marker allows Stop and that `stop_hook_active: true` does not loop.

## Rollback or failure strategy

Disable the hook for an emergency session with `/hooks` or `codex --disable hooks`, record the bypass, and do not mark the increment complete. To roll back the increment, remove the new hook, skill, tests, report assets, ignored state rule, npm test integration, and workflow documentation. No application or stored-data migration is required.

## Exit criteria

- Hook configuration parses and is discovered through normal project trust review.
- Every focused required case passes.
- Existing repository verification passes with hook tests included.
- No application source file changes.
- Code, security, scope, secret, generated-output, and documentation reviews pass.
- The 4G report has `PASS` or `PASS WITH ADVISORIES` and the marker is complete and valid.

## Verification result

Automated verification and review pass: 15 focused hook tests, 124 frontend tests, 92 Rust library tests, ten Rust integration tests, formatting, ESLint, Clippy with warnings denied, typecheck, Vite build, Tauri release no-bundle build, exact scope, no-application-source, secret, generated-output, diff, code, and security review. The project owner passed normal `/hooks` trust and live Stop confirmation. The report is `PASS WITH ADVISORIES`, and the completion marker is valid.
