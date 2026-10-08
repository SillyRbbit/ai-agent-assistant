---
name: documentation-sync
description: Synchronize Cortexa handoff, status, decisions, next steps, changelog, troubleshooting, plans, and increment docs with actual code and test evidence.
---

# Documentation sync

1. Read `AGENTS.md` and inspect Git status, diff, recent commits, and actual test output.
2. Update only facts supported by repository evidence.
3. Keep `HANDOFF.md` actionable and include an exact next prompt.
4. Update capability status and priorities when implementation state changed.
5. Append decisions and troubleshooting history; do not erase old records.
6. Update the changelog and active increment or plan.
7. Verify internal links and Markdown formatting.
8. Report documentation-only changes separately from runtime changes.

Do not claim target-platform verification that did not run.

## D-134 milestone operation

Follow AGENTS.md and ENGINEERING_GUIDE.md: objective, exclusions and acceptance
are the scope boundary. Path counts are informational; preserve protected paths,
attribution and unrelated work. Continue authorized routine work through review,
documentation and truthful finalization. Reuse valid unchanged-input verification
with provenance; distinguish implemented, automatically verified, live verified,
deferred and blocked criteria. Historical FAIL evidence and D-133's separate
acceptance route remain immutable. Request approval only at AGENTS.md boundaries.
