# Start a working session

## Goal

Orient to the actual repository state before making changes.

Use `.agents/skills/session-start/SKILL.md` when repository skills are
available. Otherwise use `prompts/workflows/start-session.md` with
`{{SESSION_MODE}}` set to `start`.

## Procedure

1. Open the repository root.
2. Read, in order:
   - `AGENTS.md`
   - `HANDOFF.md`
   - `PROJECT_STATUS.md`
   - `NEXT_STEPS.md`
   - `DECISIONS.md`
   - `TROUBLESHOOTING_LOG.md`
   - `SECURITY.md` for security-sensitive work
3. Inspect the working tree:

   ```bash
   git status --short --branch
   git diff --stat
   ```

4. Confirm the toolchain:

   ```bash
   node --version
   npm --version
   cargo --version
   rustc --version
   ```

5. Confirm dependencies or install them:

   ```bash
   npm ci
   ```

6. Run the smallest health check relevant to the intended work. For a general session:

   ```bash
   npm run typecheck
   npm run test:unit
   ```

7. State:
   - Current phase and increment.
   - Confirmed repository status.
   - One session goal.
   - Explicit non-goals.
   - Files likely to change.
   - Verification commands.

8. Stop and troubleshoot before implementation if the baseline does not compile or the handoff conflicts with the repository.

## Expected assistant opening

```text
Current state: ...
Goal for this session: ...
Non-goals: ...
Files expected to change: ...
Verification: ...
Risks or blockers: ...
```

## Exit condition

The session is ready for implementation only after the baseline, scope, and verification path are known.
