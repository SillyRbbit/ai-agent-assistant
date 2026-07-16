# Contributing

## Development workflow

1. Read `AGENTS.md`, `ENGINEERING_GUIDE.md`, and the current handoff files.
2. Confirm the repository and toolchain state.
3. Select only the first Ready increment from `NEXT_STEPS.md`, unless the
   project owner explicitly selects another bounded task.
4. Define exact files, acceptance criteria, risks, non-goals, verification, and
   rollback, then wait for approval.
5. Begin the mandatory repository gate before editing.
6. Implement the smallest coherent approved change.
7. Run targeted checks during development and the full relevant checks before
   completion.
8. Review the complete diff against `CODE_REVIEW.md`, `SECURITY.md`, and the
   applicable checklists.
9. Update project memory, the increment record, and the post-increment report.
10. Require a valid completion marker, then stop. Commit or publish only when
    explicitly directed.

## Setup

```bash
npm ci
npm run tauri -- dev
```

The preferred toolchain is:

```text
Node.js 26.3.0
npm 11.16.0
Rust 1.90.0
```

## Checks

```bash
npm run format:check
npm run lint
npm run typecheck
npm run test
npm run build
npm run tauri -- build --no-bundle
```

Use `npm run verify` for the complete sequence.

Testing details and the change-to-test matrix are in `TESTING_GUIDE.md`.

## Commit guidance

Prefer focused conventional commit messages:

```text
feat: add platform-neutral agent interfaces
fix: expose Homebrew rustup on macOS setup path
test: cover mock policy denial
docs: add session handoff workflow
chore: update locked toolchain metadata
```

Do not combine unrelated refactors, dependency upgrades, and features in one commit.

Use one branch per increment. Start from clean synchronized `main` unless an
approved reconstruction plan says otherwise. See `ENGINEERING_GUIDE.md` for
branch, publication, and rollback rules.

## Pull-request or change summary

Include:

- Goal.
- User-visible behavior.
- Security impact.
- Files and architectural layers changed.
- Tests and commands run.
- Known limitations.
- Follow-up task.

## Generated and local files

Do not commit:

- `node_modules/`
- `dist/`
- `coverage/`
- `src-tauri/target/`
- `.env` files
- local logs
- secrets or personal test data

Use synthetic fixtures only.

## Architecture and release references

- Current architecture: `ARCHITECTURE.md`
- Product requirements: `PRODUCT_REQUIREMENTS.md`
- Milestone roadmap: `ROADMAP.md`
- Security checklist: `SECURITY_CHECKLIST.md`
- Release checklist: `RELEASE_CHECKLIST.md`
