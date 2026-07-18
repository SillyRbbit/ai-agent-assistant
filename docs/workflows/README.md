# Working-session workflows

These runbooks keep development reproducible across assistant sessions.

| Workflow             | Use                                              |
| -------------------- | ------------------------------------------------ |
| `START_SESSION.md`   | Begin a new thread or task                       |
| `RESUME_SESSION.md`  | Continue from an existing handoff                |
| `END_SESSION.md`     | Close work and leave a complete handoff          |
| `TROUBLESHOOTING.md` | Diagnose setup, build, test, or runtime failures |

Equivalent reusable skills are under `.agents/skills/`. Categorized copy-paste
fallbacks and selection guidance are under `prompts/README.md`.

The repository state files at the root are authoritative. Workflow documents explain how to maintain them; they do not replace them.
