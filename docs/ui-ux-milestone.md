# Cortexa UI/UX milestone

The candidate is ready for owner review once the external complete/valid and full Stop receipts
are verified. [Plan/checklist](plans/2026-10-05-ui-ux-redesign.md) and
[review](reviews/2026-10-05-ui-ux-redesign-post-increment-review.md).

Worktree: `/Users/hdang/.codex/worktrees/ui-ux-redesign/ai-agent-assistant`.
Branch: `codex/ui-ux-redesign`; baseline `d2090d66f5d6212bf7ca030f0502b0b88c215d94`.
No inherited changes. 31 candidate paths; no commits or publication.

## Design and evidence

The owner video informed charcoal surfaces, restrained lavender accents, approved colorful
identity cards, a central work area and contextual panels. The geographic map/fake metrics
were not copied. Conversations/Knowledge emphasize reading and editing. All existing routes,
profile customizations, stored-data/execution boundaries, logo and mascot bytes are preserved.

Evidence: `/private/tmp/cortexa-ui-ux-redesign-evidence-6jl4jhzb`.
Comparable before screenshots: `before/`; final browser screenshots:
`browser-webkit-presentation/`. Automated: 40 frontend files/586 tests, followed by 10 affected
Knowledge tests; 156 final layout cases, 22 inherited affected Bots cases, source/privacy,
search/version/discard, normal/reduced-motion and sole-dark-theme checks. Counts overlap and
are not summed. No provider execution was inferred from synthetic fixtures.

Direct native: wide six-screen review, Conductor inspector, mascot spin returning idle,
unsaved note discard without Save; compact six-screen review and independent scrolling.
`native-controls/direct-observations.json` binds the final corrected native selects/menu/Escape
observations; its artifact.json binds exact source/dist/executable. Earlier bundles and failures
remain as historical debugging evidence. All test processes were gracefully stopped.

## Preview

From the exact worktree, first verify that port 4192 is free:

```bash
cd /Users/hdang/.codex/worktrees/ui-ux-redesign/ai-agent-assistant
npm run dev -- --host 127.0.0.1 --port 4192 --strictPort
```

Open `http://127.0.0.1:4192/`. Browser mode uses mocks; it does not establish native persistence,
provider or OS behavior. The explicit synthetic full-App layout fixture is
`http://127.0.0.1:4192/scripts/browser/knowledge.html?design`.

For native owner review, first reverify
`/private/tmp/cortexa-ui-ux-redesign-evidence-6jl4jhzb/native-controls/artifact.json`.
Use its isolated identifier and existing key-free wrapper, preserving the synthetic data:

```bash
sh /private/tmp/cortexa-connected-knowledge-evidence/offline.sh \
  '/Users/hdang/.codex/worktrees/ui-ux-redesign/ai-agent-assistant/src-tauri/target/debug/bundle/macos/Cortexa UI UX Review QA.app/Contents/MacOS/ai-agent-assistant'
```

This unsigned QA bundle is separate from the owner application. Do not use ordinary
`npm run tauri dev` against personal data for this review. Do not Send or start workflows.

## Owner manual checklist

- Approve spacing, typography, colorful identities and information density against the video.
- Compare six screens at comfortable wide/compact sizes, with both navigation states.
- Check reading, source disclosure, focus, inspector placement, scrolling and dropdown menus.
- Review synthetic note/search/version workflows only after explicitly approving any data changes.

Owner aesthetics, real personal-data workflow comfort, remote CI, Windows native execution,
release signing and live-provider success/cancellation are unverified. Existing advisories,
D-127/D-128 and the process-local workaround remain. Live QA stays parked 3/10; D-125/M1/M2 parked.

## Recovery

Keep the worktree, branch and all evidence intact. Exact tracked changes are in candidate.patch;
new files and all candidate payloads are in candidate-new-files.tar and candidate-files.tar,
with completed-candidate.json hashes and recovery-verification.json. baseline.tar preserves
starting files. RECOVERY.md gives verification/reconstruction steps. These local temporary
archives should be copied to durable storage before any separately approved cleanup.

Before integration, request approval to archive only this isolated candidate; never reset or
clean other worktrees. After a future integration, reverse only this milestone's reviewed diff
on a new branch from then-current main, preserving later work and rerunning affected checks.
No restoration, deletion, branch removal or automatic rollback was performed.

Next action: owner review of this candidate, not another implementation milestone.
