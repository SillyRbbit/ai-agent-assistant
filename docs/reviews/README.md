# Post-increment reviews

Store one consolidated review per implementation increment in this directory:

```text
YYYY-MM-DD-<increment>-post-increment-review.md
```

Start from `docs/templates/POST_INCREMENT_REVIEW_TEMPLATE.md`. Reports must contain the exact machine manifest and required sections validated by `.codex/hooks/post_increment_gate.py`.

The report is engineering evidence, not authorization. Do not include credentials, environment values, private keys, raw model content, personal data, raw tool arguments or results, database content, or unredacted errors.

The ignored `.codex/state/post_increment_gate.json` file tracks one active increment and receives a completion marker only after the report validates. It is local workflow state and must never be treated as a security boundary, approval, audit record, or substitute for actual command output.

Repository-local hooks require normal Codex trust review. Use `/hooks` to review, trust, or disable the exact hook definition. For an emergency session, start Codex with `--disable hooks`; record the bypass and rerun the complete gate before marking an increment complete. Do not use `--dangerously-bypass-hook-trust` as a routine workaround.
