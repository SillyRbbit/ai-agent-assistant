# Working-session workflows

These runbooks keep development reproducible across assistant sessions.

Start with [Using a coding assistant](ASSISTANT_USAGE.md) for repository setup,
session workflow, and review guidance. Repository-relative command examples
assume the checkout root. See the [documentation index](../README.md) for other
categories.

| Workflow             | Use                                              |
| -------------------- | ------------------------------------------------ |
| `START_SESSION.md`   | Begin a new thread or task                       |
| `RESUME_SESSION.md`  | Continue from an existing handoff                |
| `END_SESSION.md`     | Close work and leave a complete handoff          |
| `TROUBLESHOOTING.md` | Diagnose setup, build, test, or runtime failures |

Equivalent reusable skills are under `.agents/skills/`. Categorized copy-paste
fallbacks and selection guidance are under `prompts/README.md`.

The repository state files at the root are authoritative. Workflow documents explain how to maintain them; they do not replace them.
