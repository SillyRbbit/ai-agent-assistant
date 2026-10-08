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

8. Diagnose baseline failures and handoff mismatches, preserve evidence and assess impact. Continue independent authorized work; stop for unresolved attribution, access or required-check failure.

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

## D-134 milestone operation

Follow AGENTS.md and ENGINEERING_GUIDE.md: objective, exclusions and acceptance
are the scope boundary. Path counts are informational; preserve protected paths,
attribution and unrelated work. Continue authorized routine work through review,
documentation and truthful finalization. Reuse valid unchanged-input verification
with provenance; distinguish implemented, automatically verified, live verified,
deferred and blocked criteria. Historical FAIL evidence and D-133's separate
acceptance route remain immutable. Request approval only at AGENTS.md boundaries.
