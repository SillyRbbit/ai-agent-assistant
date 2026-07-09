# Contributing

## Development workflow

1. Read `AGENTS.md` and the current handoff files.
2. Confirm the repository and toolchain state.
3. Select one ready increment from `NEXT_STEPS.md`.
4. Define acceptance criteria and verification commands.
5. Implement the smallest coherent change.
6. Run targeted checks during development.
7. Run the full relevant checks before completion.
8. Review the diff against `CODE_REVIEW.md` and `SECURITY.md`.
9. Update project-memory and increment documents.
10. Commit only reviewed files.

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
