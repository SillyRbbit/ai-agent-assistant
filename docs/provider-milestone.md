# Provider connections milestone

## Current checkpoint — implemented and automatically verified; owner QA pending

- Worktree: `/Users/hdang/.codex/worktrees/provider-milestone/ai-agent-assistant`
- Branch: `codex/provider-milestone`
- Last commit / baseline: `f85d36954c77287901095afde3b000d8952d544d`
- Gate: ordinary `provider-milestone` admission passed on 2026-09-30.
- Existing work: this checkout began clean. Every other checkout, dirty file,
  terminal record and prunable registration is preserved. No inherited changes
  are copied or rewritten. Preservation inventory is outside the repository at
  `/private/tmp/cortexa-provider-milestone-evidence/preservation.json`.
- New changes: native Codex adapter; preferences/chat ownership wiring; native static
  errors; typed client and Agents controls; focused protocol/UI tests; this handoff
  and plan. Other providers and dependency files remain unchanged.
- Actual focused results: typecheck passed; frontend client/Agents 30 tests passed.
  Initial native pure checks passed 3/3. Two new subprocess tests exposed a test
  expectation missing the macOS-added `__CF_USER_TEXT_ENCODING` key; corrected,
  rerun pending. No production environment inheritance was added.
- Installed Codex 0.159.0 key-free loopback fixture: one synthetic request,
  empty tools, streamed text and explicit completed status. This is not live proof.
- Runtime limitation: built-in OpenAI retry overrides are rejected by Codex.
  Cortexa never resubmits a turn or falls back; Codex internal retries and token
  budgets remain runtime-controlled and must be disclosed before owner Send.

## Completion checklist

- [x] Inspect current main and existing implementation; isolate from dirty Desktop checkout.
- [x] Preserve all six existing connection options and application orchestration.
- [x] Direct OpenAI selection, runtime-specific model/effort validation and secure owner setup.
- [x] Headless Codex authentication through the supported runtime, model/effort discovery,
      bounded text execution, streamed output and cancellation with cleanup.
- [x] Per-agent persistent settings and notes survive restart and remain agent-isolated.
- [x] Conversation settings/context captured at Start; stale revisions rejected.
- [x] Explicit disclosure/acknowledgement; sanitized actionable failures; no retry/fallback.
- [x] Automated protocol, credential-redaction, persistence, concurrency/cancellation and UI checks.
- [x] Full applicable verification and native build; documentation/security checks.
- [x] Completion review/report prepared with live/manual limitations separately recorded.
      Validate generated completion status and full Stop receipt before resuming.
- [ ] Owner manual QA (pending; not automated verification).

## Evidence and blockers

Existing code already implements native SQLite preferences, OpenAI/Anthropic/local
streaming and cancellation. Codex is persisted only as an unavailable choice and
is blocked in UI and Rust. Installed bundled Codex CLI is 0.159.0; public protocol
schema is being inspected without reading credentials or making model requests.
The prior native workaround is process-local Python 3.12.1, Xcode/SDK 27.0 and
`profile.release.build-override.strip="none"`. No global setting will change.

Live OpenAI/Codex generation is not verified by historical screenshots or fixtures.
No current live request allowance or credentials have been supplied for this
milestone. Complete all independent implementation/automated checks before handoff.
D-127/D-128 and historical live-success/isolation advisories remain until replaced
by actual evidence. D-125/M1/M2 remain parked.

## Scope and exact next action

Modify only the existing agent preferences/chat/client/UI and their tests, a narrow
native Codex adapter (and registration only if necessary), security/current-state
records, this handoff, the dated plan and consolidated review. No graph changes,
provider expansion, dependency migration, governance changes or publication.
Next: finish subprocess regressions, review adapter isolation/cleanup and run full
automated verification. Keep this same increment open
through recoverable in-scope failures. Save checkpoints here after meaningful work.

## Owner setup and manual QA (no live request performed by Codex)

Build/run only this candidate, not the dirty Desktop checkout. The prepared artifact
path will be recorded after its actual build. Keep keys out of commands pasted into
chat, screenshots, source, SQLite notes/instructions and logs.

- Direct OpenAI: privately launch the debug executable with
  `CORTEXA_OPENAI_DEMO=1` and your session `OPENAI_API_KEY`. API billing/access is
  separate from ChatGPT. Choose one of the unchanged three models and supported
  effort. No API call occurs merely from profile selection or Save.
- Headless Codex: use installed Codex **0.159.0**. Create a dedicated private home
  yourself, use the CLI's supported `login` with `CODEX_HOME` set to that directory,
  and do not add config.toml, AGENTS.md, plugins or tools. Do not copy credentials
  into Cortexa. Privately launch with `CORTEXA_CODEX_DEMO=1` and
  `CORTEXA_CODEX_HOME` pointing to that dedicated directory. The default executable
  is the bundled ChatGPT CLI; `CORTEXA_CODEX_EXECUTABLE` may specify an absolute
  owner-selected installed binary of the tested version. Refresh models explicitly.
  Refresh reads runtime account/model metadata, never sends a conversation prompt.
- Codex discovery proves neither subscription/model entitlement nor live generation.
  Internal retries/token budgets and account usage/retention belong to Codex.
  Cortexa never resubmits a turn or falls back. A 60-second deadline and 8,192-character
  visible-output limit apply. Stop cannot promise immediate remote termination.
- Select each agent independently; save model/effort, instructions and Memory mode.
  Close/reopen to verify persistence. Memory Off retains the note but excludes it.
- Start a fresh conversation, verify the displayed captured revision/model/effort,
  instructions/note selection and disclosure. Only after a new explicit request
  allowance and acknowledgement, send one short harmless message.
- Observe incremental text, explicit completed answer and no active native generation.
  Separately approve another request to test Stop; verify stopped/error status and
  released ownership. Do not automatically retry failure. Report only sanitized codes.
- Confirm simulation, Anthropic and local choices remain present. Do not Refresh
  or Send to those services without their own owner authorization.

## Validation checkpoint

Typecheck and focused frontend/client: Passed (30 tests). Native adapter: Passed
(5 tests). Strict all-target/all-feature Clippy: Passed. The first fixture attempt
failed because macOS inserts a documented OS locale key; its test allowlist was
corrected without inheriting any host environment. Remaining: full verification,
relevant native bundle, preservation, final review/schema/gates. Live requests: 0.

### Recovery checkpoint

The full check caught two strict frontend lint issues (corrected) and legacy-table
`max` effort storage (corrected by the existing sidecar path; no schema/history
change). Focused persistence now passes 10/10, including all-nine-agent restart
with OpenAI max and independent Codex model/high effort. Preservation passes for
seven predecessor worktrees, all raw states and 36 prunable registry entries.
The initial comparator incorrectly expected an extra registry block even though
the snapshot already contained this worktree; exact registry equality now passes.
Full verification is running. Runtime daemon auto-start and local automation are
explicitly disabled as additional child-lifecycle controls; final fixture recheck
and strict lint remain required. No live requests or auth inspection occurred.

### Final lifecycle checkpoint

App exit now synchronously kills/reaps adapter-owned Codex children before allowing
Tauri to exit, independently of async-task drop scheduling. New child starts are
blocked once shutdown begins. Failed cleanup cancels exit rather than claiming
success. The native exit-cleanup regression passed; adapter tests now pass 6/6.
The final full offline verify and refreshed unsigned debug bundle are running.
No source, dependencies, records or artifacts in predecessor worktrees changed.

## Final automated result and artifact

Final full offline verification passed after the lifecycle correction: 74 hook,
85 repository, 432 frontend and 374 native unit tests, all applicable integration
suites, strict lint, typecheck, formatting, frontend and native release builds.
The existing operator-only real-Hermes probe is the sole ignored integration test;
no real Hermes was installed or launched. The final unsigned debug bundle passed.
This is local macOS evidence; no new Linux CI or authenticated live result is claimed.
All 24 changed paths are listed in the plan/report; no commit was made.

Artifact: `/Users/hdang/.codex/worktrees/provider-milestone/ai-agent-assistant/src-tauri/target/acceptance/debug/bundle/macos/Cortexa.app`

Executable: `/Users/hdang/.codex/worktrees/provider-milestone/ai-agent-assistant/src-tauri/target/acceptance/debug/bundle/macos/Cortexa.app/Contents/MacOS/ai-agent-assistant`

Executable SHA-256: `88bacf8f7f8ba59d507acf3cf28c3a6c9425f65f736ad972d775063a718c1abf`.
Identity: `com.aiagentassistant.desktop`, version `0.1.0`, arm64 Mach-O.
No application launch was performed. Open this exact candidate for owner QA.

Owner-only Codex setup example (run yourself; never paste credentials here):

```sh
mkdir -m 700 "$HOME/.cortexa-codex"
CODEX_HOME="$HOME/.cortexa-codex" "/Applications/ChatGPT.app/Contents/Resources/codex-cli/bin/codex" login
CORTEXA_CODEX_DEMO=1 CORTEXA_CODEX_HOME="$HOME/.cortexa-codex" \
  "/Users/hdang/.codex/worktrees/provider-milestone/ai-agent-assistant/src-tauri/target/acceptance/debug/bundle/macos/Cortexa.app/Contents/MacOS/ai-agent-assistant"
```

Use an existing dedicated directory only if it is yours and has no custom config,
AGENTS.md, tools or plugins; do not overwrite or copy credential files. OpenAI setup
uses its separate private session key and `CORTEXA_OPENAI_DEMO=1`. API credentials
never authenticate Codex. Starting or saving a conversation sends no prompt.

Next action: owner QA using the checklist above, after explicit live allowance and
acknowledgement. Live generation is blocked pending owner credentials/access and
approval; implementation is complete, not fully live-verified. D-127/D-128 and the
native build workaround remain. D-125/M1/M2 are parked. Final completion/Stop receipts
are external; validate the checkout-local marker before resuming or publication.

## Acceptance checkpoint

All required implementation and available automated checks passed. Final report:
[provider milestone review](reviews/2026-09-30-provider-milestone-post-increment-review.md),
`PASS WITH ADVISORIES`; owner QA is `Ready with advisories`. Documentation, repository,
security, whitespace, exact 24-path inventory and preservation passed. Ordinary
finalization and full Stop receipts are generated after report freeze in
`/private/tmp/cortexa-provider-milestone-evidence`; validate the local gate before
resuming. No required live check is claimed passed. No commits/publication.
