# How to use a coding assistant with this repository

This guide explains how to use Codex or another repository-aware coding assistant without relying on prior chat history.

## 1. Open the correct project

Open the repository root, not `src/` or `src-tauri/` alone.

```bash
cd /path/to/ai-agent-assistant
pwd
```

The root should contain `AGENTS.md`, `HANDOFF.md`, `package.json`, and `src-tauri/`.

## 2. Confirm the local application works

For a fresh checkout:

```bash
node --version
npm --version
cargo --version
npm ci
npm run tauri -- dev
```

Use `Control-C` in the terminal to stop the development process.

## 3. Start a coding session

Ask the assistant to use the repository's start workflow:

```text
Use $session-start for this repository. Read the required project-memory files, inspect Git status and the toolchain, summarize the current state, and propose the smallest ready increment. Do not change files until you have identified the goal, affected files, and verification commands.
```

If skills are unavailable, paste `prompts/start-work.md` instead.

## 4. Resume an existing task

Use:

```text
Use $resume-session and continue from HANDOFF.md. Verify the repository instead of assuming the handoff is current. Work only on the exact next task unless you find a blocker.
```

Or paste `prompts/resume-work.md`.

## 5. Give a new task

A useful task prompt contains:

- One goal.
- Explicit non-goals.
- Acceptance criteria.
- Required verification commands.
- Any files or user-visible behavior that must remain unchanged.

Example:

```text
Use $verified-increment. Implement only Phase 2 Increment 2A from NEXT_STEPS.md. Preserve the current UI and IPC behavior. Do not add SQLite, networking, permissions, or new Tauri capabilities. Add focused Rust tests, run the documented checks, and update the project-memory files with actual results.
```

## 6. Use repository skills

Repository-scoped skills are under `.agents/skills/`.

| Skill                 | Use it for                                                   |
| --------------------- | ------------------------------------------------------------ |
| `$session-start`      | Orienting at the beginning of a new thread                   |
| `$resume-session`     | Continuing from `HANDOFF.md`                                 |
| `$verified-increment` | Implementing one small feature or refactor safely            |
| `$troubleshoot`       | Diagnosing a build, test, install, or runtime failure        |
| `$code-review`        | Reviewing a diff against project rules                       |
| `$security-review`    | Reviewing trust boundaries, permissions, tools, or data flow |
| `$documentation-sync` | Updating repository memory after changes                     |
| `$branding`           | Applying Cortexa identity to product and presentation assets |
| `$session-end`        | Closing a session with a complete handoff                    |

In Codex, type `$` or use the skills menu to invoke a skill explicitly. If newly added skills are not visible, restart or reload the assistant from the repository root.

For brand, logo, presentation, or architecture-diagram work, invoke `$branding`
and follow `docs/branding/BRAND_GUIDELINES.md`. The files under
`assets/branding/` are authoritative; do not redraw or recolor the logo or use
an external presentation as the source of current product facts.

## 7. Use prompt files when skills are not available

Reusable prompts live in `prompts/`. Open the appropriate file and paste its contents into the assistant.

Recommended sequence:

1. `prompts/start-work.md`
2. `prompts/implement-next-increment.md`
3. `prompts/review-change.md`
4. `prompts/end-of-session-handoff.md`

For a failure, switch to `prompts/troubleshooting.md` before making broad changes.

## 8. Review changes before accepting them

Ask for a diff-based review:

```text
Use $code-review. Review all uncommitted changes against AGENTS.md, SECURITY.md, CODE_REVIEW.md, and the increment acceptance criteria. Report concrete defects first, then verification gaps, then documentation drift. Do not rewrite code unless I ask.
```

Check locally:

```bash
git status --short --branch
git diff --stat
git diff
npm run check
```

Run the native build when Rust and platform prerequisites are available:

```bash
npm run tauri -- build --no-bundle
```

## 9. End every session cleanly

Use:

```text
Use $session-end. Run the appropriate final checks, update HANDOFF.md, PROJECT_STATUS.md, NEXT_STEPS.md, DECISIONS.md, CHANGELOG.md, TROUBLESHOOTING_LOG.md, and the increment record as needed. Include the exact next resume prompt and distinguish passed, failed, and not-run checks.
```

Or paste `prompts/end-of-session-handoff.md`.

Then stop active development processes and inspect Git status:

```bash
git status --short --branch
```

## 10. First-time Git setup

Run this only if the local folder is not already a Git repository:

```bash
git rev-parse --is-inside-work-tree
```

If that command reports that the folder is not a repository:

```bash
git init
git add .
git commit -m "chore: establish Cortexa scaffold"
```

Do not run `git init` inside an existing checkout or nested repository.

## 11. What the assistant must not do

Do not authorize the coding assistant to:

- Store API keys or secrets in source files.
- Add unrestricted shell or filesystem access to the product.
- Enable macOS permissions merely to make a demo work.
- Skip tests and report success.
- Delete or rewrite project history files without preserving the prior record.
- Combine unrelated implementation increments.
- Push, merge, release, or rewrite Git history unless you explicitly request that action.

## 12. Good session boundaries

Use one assistant thread per coherent unit of work. Continue in the same thread while fixing the same increment. Start a new thread after the handoff is complete and the next task is materially different.

The repository files, not chat memory, are the source of truth for resuming work.
