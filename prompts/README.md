# Cortexa prompt library

This directory contains copy-and-paste instructions for repository-aware coding
assistants. Use a prompt when a repository skill is unavailable, when the task
needs explicit placeholders, or when a repeatable multi-step instruction is
more convenient than invoking a skill directly.

`AGENTS.md`, `ENGINEERING_GUIDE.md`, `SECURITY.md`, accepted decisions, and the
current approved plan remain authoritative. A prompt does not override those
files, grant approval, or prove that a command ran.

## Directory map

| Directory                    | Responsibility                                                          |
| ---------------------------- | ----------------------------------------------------------------------- |
| [`increments/`](increments/) | Authorize one bounded implementation, remediation, bug fix, or refactor |
| [`reviews/`](reviews/)       | Analyze evidence without modifying the reviewed work                    |
| [`workflows/`](workflows/)   | Coordinate a multi-step repository operating procedure                  |
| [`templates/`](templates/)   | Provide a skeleton for authoring a new prompt                           |

## Prompt selection guide

| Need                                                                 | Prompt                                                                                   |
| -------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| Implement an already Ready increment                                 | [`increments/verified-increment.md`](increments/verified-increment.md)                   |
| Bound a new product feature before implementation                    | [`increments/feature-implementation.md`](increments/feature-implementation.md)           |
| Remediate all current findings at one severity                       | [`increments/remediation-by-severity.md`](increments/remediation-by-severity.md)         |
| Remediate one advisory                                               | [`increments/remediation-single-advisory.md`](increments/remediation-single-advisory.md) |
| Reproduce and fix one defect                                         | [`increments/bug-fix.md`](increments/bug-fix.md)                                         |
| Refactor without changing behavior                                   | [`increments/refactor.md`](increments/refactor.md)                                       |
| Review architecture, security, code, debt, or readiness              | Choose the matching file under [`reviews/`](reviews/)                                    |
| Start or resume a repository session                                 | [`workflows/start-session.md`](workflows/start-session.md)                               |
| Close a verified session                                             | [`workflows/end-session.md`](workflows/end-session.md)                                   |
| Coordinate remediation, release, documentation, or repository health | Choose the matching file under [`workflows/`](workflows/)                                |
| Author a new prompt                                                  | Choose the matching skeleton under [`templates/`](templates/)                            |

## Prompts, skills, workflows, and templates

- **Prompts** are version-controlled copy-and-paste task instructions under this
  directory.
- **Skills** under [`.agents/skills/`](../.agents/skills/) are named operational
  procedures that Codex can discover and invoke directly.
- **Workflow prompts** under [`workflows/`](workflows/) coordinate several
  repository steps. Human-readable runbooks remain under
  [`docs/workflows/`](../docs/workflows/).
- **Prompt templates** under [`templates/`](templates/) are authoring skeletons.
  Repository artifact templates remain under
  [`docs/templates/`](../docs/templates/).

Do not convert every prompt into a skill. Add or change a skill only when Codex
needs a named operational capability rather than a copy-and-paste instruction.

## How to run a prompt

1. Open the prompt that matches the task.
2. Replace every `{{PLACEHOLDER}}` with a concrete repository value.
3. Paste the text inside its `Prompt` code block into the assistant.
4. Review the proposed scope and verification before approving edits.
5. Give separate approval for commit, push, merge, release, or publication.

Never leave an unresolved placeholder in an executed prompt. Paths are
repository-relative unless the prompt explicitly requests an external path.

## Placeholder conventions

- Use uppercase snake case inside double braces, such as `{{BRANCH_NAME}}`.
- Replace a path placeholder with one repository-relative path.
- Replace a list placeholder with an explicit bounded list, not `all` or
  `anything relevant`.
- Preserve the literal severity values `Critical`, `High`, `Medium`, `Low`, and
  `Advisory` where a prompt requires one of them.
- When an input does not apply, replace it with `None` and explain why rather
  than deleting the prompt's boundary.

## Standard prompt metadata

Every executable prompt and prompt template contains these Markdown fields:

- Title
- Category
- Purpose
- Use when
- Do not use when
- Required inputs
- Expected outputs
- Related skills
- Related prompts
- Last reviewed

The metadata is human-readable Markdown. No custom parser or dependency is
required.

## Adding or changing a prompt

1. Confirm an existing prompt cannot serve the same purpose with a placeholder
   or cross-reference.
2. Choose exactly one category and a lowercase kebab-case filename.
3. Start from the matching prompt template.
4. Keep project-wide rules in their authoritative files and link to them.
5. Keep task-specific scope controls in the prompt itself.
6. Add the prompt to this selection guide and update active references.
7. Search for overlap with prompts, skills, workflows, and document templates.
8. Run Markdown formatting, path/link checks, and complete repository
   verification before acceptance.

Prompt changes are versioned through normal Git review. Update `Last reviewed`
when behavior or authoritative references change, not for formatting-only edits.
Preserve useful history through moves or documented merges.

## Common workflows

### Ready implementation

1. Run [`workflows/start-session.md`](workflows/start-session.md).
2. Run [`increments/verified-increment.md`](increments/verified-increment.md).
3. Run the relevant review prompts and
   [`reviews/quality-gate.md`](reviews/quality-gate.md).
4. Run [`workflows/end-session.md`](workflows/end-session.md).

### Advisory remediation

1. Use [`workflows/remediation.md`](workflows/remediation.md) to select and
   bound the work.
2. Use either
   [`increments/remediation-single-advisory.md`](increments/remediation-single-advisory.md)
   or
   [`increments/remediation-by-severity.md`](increments/remediation-by-severity.md).
3. Close with [`workflows/end-session.md`](workflows/end-session.md).

### Release assessment

1. Coordinate evidence with [`workflows/release.md`](workflows/release.md).
2. Assess it with [`reviews/release-review.md`](reviews/release-review.md).
3. Obtain separate approval before any tag, signing, upload, or publication.
