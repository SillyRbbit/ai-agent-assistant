# Review changes

```text
Review the current changes without rewriting them unless I explicitly ask for fixes.

Read AGENTS.md, SECURITY.md, CODE_REVIEW.md, the active increment or plan, and the relevant source and tests. Inspect Git status and the full diff.

Report findings in severity order. For each real finding, provide the file and line, concrete impact, evidence or reproduction path, and the smallest recommended fix. Focus on correctness, authorization boundaries, permissions, personal-data handling, error and cancellation behavior, test gaps, portability, and documentation drift. Avoid speculative style comments.

Then list verification commands that were run, commands that still need to run, and whether the increment acceptance criteria are satisfied.
```
