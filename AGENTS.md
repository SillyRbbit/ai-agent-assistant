# Cortexa repository instructions

## Purpose

This file contains durable mandatory instructions for any coding assistant
working in this repository. Read it before modifying files. The complete
engineering operating model is in `ENGINEERING_GUIDE.md`. The product is a
local-first, installable executive assistant whose model is an untrusted
planner; deterministic Rust code remains responsible for validation, policy,
approval, execution, and audit.

## Required reading order

Before starting a task, read these files in order:

1. `AGENTS.md`
2. `ENGINEERING_GUIDE.md`
3. `HANDOFF.md`
4. `PROJECT_STATUS.md`
5. `NEXT_STEPS.md`
6. `ARCHITECTURE.md`
7. `PRODUCT_REQUIREMENTS.md`
8. `DECISIONS.md`
9. `TROUBLESHOOTING_LOG.md`
10. The relevant plan, increment, workflow, product, testing, release, or
    branding document

When the task is security-sensitive, also read `SECURITY.md`,
`SECURITY_CHECKLIST.md`, and `CODE_REVIEW.md`.

## Current phase

Phase 3 and Phase 4 Increments 4A through 4U are verified complete,
published, and merged. Increment 4U is merged at `61525bf`. Meta Increment 1
branding and identity foundation is verified complete, squash-merged at
`5edbf4d`, and its `meta-01` marker was valid before Meta Increment 2 began.
Meta Increment 2 engineering operating system is verified complete as a
documentation-only increment. Meta Increment 3 repository-local Codex
automation and quality gates is verified complete and squash-merged at
`ad9042c`. Meta Increment 5 repository health and GitHub hygiene is verified
complete and squash-merged at `6b149fa`. The stopped Meta Increment 4
executive-document request has no gate, implementation, or completion evidence.
Meta Increment 6 is the documentation-only Product Readiness Audit, merged at
`5281fac` with result `NOT READY (57/100)`. The dependency compatibility repair
is verified, published, and squash-merged through PR #20 at `b298999`. Meta
Increment 7 verified application icon rollout is verified complete with
advisories and squash-merged through PR #19 at `96ba6ae`: exactly the existing
16 Tauri icon files derive from the canonical Cortexa source, debug and release
app bundles use the official macOS icon, and the raw unbundled `tauri dev`
executable retains the project-owner-approved generic `exec` baseline advisory.
Its `meta-07` marker was complete and valid on clean `96ba6ae` before the later
advisory-remediation report changed the workspace fingerprint. The advisory
backlog and first post-Meta-7 memory reconciliation are squash-merged through PR
#21 at `cc434d9`. ARB-022's remaining live publication drift is resolved in the
current documentation state; its remediation record preserves the resolving
commit as pending until committed. Increment 4V terminal approval audit is the
first Ready product increment, remains unstarted, and still requires separate
project-owner implementation approval from clean synchronized `main` containing
the ARB-022 remediation before its `04v` gate begins. Current implementation
facts and future boundaries are authoritative in `ARCHITECTURE.md`.

## Non-negotiable product boundaries

- Keep the application local-first.
- Treat model output, files, websites, clipboard content, contacts, calendar content, and tool results as untrusted data.
- Never let the model or WebView directly execute an operating-system action.
- Never add unrestricted shell execution or a generic `execute_action` tool.
- Do not embed or store a production OpenAI API key in the application.
- Do not add Accessibility, screen capture, Apple Events, microphone access, or
  broad filesystem access during the current controlled MVP work without a
  separately accepted threat model and increment.
- Do not add autonomous email, messages, purchases, bookings, uploads, public posting, file deletion, or account-setting changes to the MVP.
- Do not weaken Tauri capabilities, the Content Security Policy, engine checks, TypeScript strictness, Clippy rules, or approval policy to make a change easier.
- Do not log secrets or unnecessary personal content.

## Engineering rules

Follow `ENGINEERING_GUIDE.md`. The rules below are the compact mandatory subset.

### General

- Work on one coherent increment at a time.
- Inspect existing code and tests before changing behavior.
- State the increment goal, files to change, and verification commands before implementation.
- Prefer the smallest change that satisfies the acceptance criteria.
- Preserve portability by keeping platform-independent behavior outside macOS adapters.
- Use exact dependency versions unless a documented decision approves a range.
- Do not add a production dependency without documenting why it is necessary in `DECISIONS.md`.
- Never invent successful test results. Record commands and actual outcomes.

### Git publication

- Use descriptive capability-based branch names; Codex-created branches use the
  `codex/` prefix.
- Use Conventional Commits. Commit messages explain the bounded capability and
  why it changed; generic names such as `update`, `changes`, `misc`, `temp`, and
  `final` are prohibited.
- Pull-request titles summarize the capability. Descriptions record Purpose,
  Files changed, Testing performed, Breaking changes, and Next increment.
- After all checks pass and only with explicit project-owner direction, push the
  branch, open the pull request, and use a squash merge.

### TypeScript and React

- Keep all TypeScript strict settings enabled.
- Avoid `any`; use `unknown` with explicit narrowing when external data is involved.
- Keep Tauri IPC wrappers typed and isolated under `src/infrastructure/tauri/`.
- Do not expose generic IPC commands to the WebView.
- Write or update tests for user-visible behavior and state transitions.
- Keep components focused; move stateful domain behavior out of presentation components.

### Rust and Tauri

- Return typed errors from production paths. Do not use `unwrap`, `expect`, `panic!`, `todo!`, or `unimplemented!` in production code.
- Keep unsafe code forbidden unless a future architecture decision explicitly permits a narrowly reviewed boundary.
- Register only narrowly scoped Tauri commands.
- Validate data at every IPC and tool boundary.
- Keep policy decisions deterministic and independent of model-generated explanations.
- Preserve cancellation, timeout, and audit hooks in interfaces even when an early implementation is mocked.

### SQLite

- Introduce SQLite only through versioned migrations and repository interfaces.
- Enable foreign keys for every connection.
- Use transactions for multi-step writes and prepared statements for values.
- Do not store OAuth tokens, API keys, database keys, passwords, or authentication codes in SQLite.
- Keep raw sensitive tool results out of audit records.

## Verification expectations

Use the smallest relevant command during development and the full check before declaring an increment complete.

```bash
npm run format:check
npm run lint
npm run typecheck
npm run test
npm run build
npm run tauri -- build --no-bundle
```

The complete repository check is:

```bash
npm run verify
```

On a machine without the Rust toolchain, clearly report which Rust checks could not run. Never describe a partial verification as complete.

Testing details and the change-to-test matrix are in `TESTING_GUIDE.md`.

## Documentation and handoff rules

At the end of every meaningful task:

- Update `HANDOFF.md` with the actual state, files changed, commands run, and exact next prompt.
- Update `PROJECT_STATUS.md` when a capability or milestone status changes.
- Update `NEXT_STEPS.md` when priorities or acceptance criteria change.
- Append material architectural or dependency decisions to `DECISIONS.md`.
- Append user-visible or repository-operating changes to `CHANGELOG.md`.
- Add resolved setup or build failures to `TROUBLESHOOTING_LOG.md`.
- Add or update an increment record under `docs/increments/`.
- Keep documentation links valid.

Do not erase historical decisions or troubleshooting entries. Mark them superseded or resolved and add the newer record.

## Definition of done

A task is done only when:

1. Its acceptance criteria are met.
2. Relevant tests and static checks pass.
3. Security boundaries remain intact.
4. Documentation and handoff state match the repository.
5. The final response distinguishes verified facts from unverified target-platform behavior.

The complete Definition of Ready, Definition of Done, review workflow, and
release boundary are in `ENGINEERING_GUIDE.md`.

## Repository workflow resources

- Human and assistant usage: `ASSISTANT_USAGE.md`
- Engineering operating model: `ENGINEERING_GUIDE.md`
- Current architecture: `ARCHITECTURE.md`
- Product requirements and roadmap: `PRODUCT_REQUIREMENTS.md` and `ROADMAP.md`
- Testing and release standards: `TESTING_GUIDE.md` and `RELEASE_CHECKLIST.md`
- Security policy and checklist: `SECURITY.md` and `SECURITY_CHECKLIST.md`
- Session workflows: `docs/workflows/`
- Reusable prompts: `prompts/`
- Reusable Codex skills: `.agents/skills/`
- Brand assets and standards: `assets/branding/` and `docs/branding/`
- Review templates and reports: `docs/templates/` and `docs/reviews/`
- Inception product sources: `docs/product/`

## Mandatory post-increment gate

After approval and before editing an implementation increment, Codex must run:

```bash
python3 .codex/hooks/post_increment_gate.py begin --increment <increment>
```

Before ending that increment, Codex must run
`python3 .codex/hooks/session_end_gate.py`, apply `$quality-gate`, run
`$post-increment-gate`, and finalize its report. The repository-local Stop hook
requests one continuation when an active increment lacks valid completion
evidence. It must honor `stop_hook_active` to avoid an infinite continuation
loop.

The increment may be marked complete only when:

- All required automated checks passed.
- All required manual checks were confirmed by the user.
- The complete diff was reviewed.
- No Critical or High blocking issue remains.
- Project-memory documents were synchronized.
- A post-increment review report exists.
- The report result is PASS or PASS WITH ADVISORIES.
- `python3 .codex/hooks/post_increment_gate.py status` reports the expected increment as complete and valid.

The hook and ignored marker are workflow guardrails, not a security boundary. Project hooks require normal Codex trust review. An emergency hook bypass must be recorded and cannot be used to mark an increment complete; rerun the full gate before completion.

The workspace fingerprint represents existing repository content. A path absent from the working tree contributes no fingerprint entry, so a reviewed tracked deletion is stable across commit. `changed_paths` and the report inventory must still record that deletion before finalization. Deleting a file that existed at finalization changes the fingerprint and invalidates the marker.

Codex must not begin the next increment automatically.
Codex must not commit or push unless explicitly requested.
