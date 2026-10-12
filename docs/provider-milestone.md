## Current provider guide — 2026-10-12

This maintained guide describes merged `main` at
`26447708934ef7f8118329d1b62977d7f14d45b6`, including provider publication
`e0d0547` and compatibility/stream fixes `bbe7546`. The September milestone
checkpoint below is retained history, not current setup or availability.
See the [capability matrix](../ARCHITECTURE.md#current-and-future-capability-matrix)
and [testing evidence](../TESTING_GUIDE.md) for acceptance limits.

### Connections, authentication and model selection

| Connection    | Native setup and behavior                                                                                                                    | Models / reasoning evidence                                                                                                                                     |
| ------------- | -------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Simulation    | No account/network; deterministic native advice                                                                                              | `simulation`, Default effort                                                                                                                                    |
| OpenAI API    | Debug build; `CORTEXA_OPENAI_DEMO=1`, private native-session `OPENAI_API_KEY`; API billing/access separate from ChatGPT                      | Closed saved models `gpt-5.6-luna`, `gpt-5.6-terra`, `gpt-5.6-sol`; declared effort selection, not a live entitlement guarantee                                 |
| Anthropic API | Debug build; `CORTEXA_ANTHROPIC_DEMO=1`, private native-session `ANTHROPIC_API_KEY`; explicit model discovery/Send                           | Native documented/discovered metadata; supported effort validated before Send; None unsupported                                                                 |
| LM Studio     | Owner-started loopback server and canonical endpoint, e.g. `http://127.0.0.1:1234/v1`; no install/load/start operation                       | Explicit refresh; exact discovered ID; Default effort only; local inference unproved                                                                            |
| Ollama        | Owner-started loopback server, e.g. `http://127.0.0.1:11434/v1`; cloud-labelled models excluded; runtime cloud disabling remains owner setup | Explicit refresh; exact discovered ID; Default effort only; loopback does not prove no egress                                                                   |
| Codex         | Debug build; `CORTEXA_CODEX_DEMO=1`, dedicated owner-authenticated `CORTEXA_CODEX_HOME`; optional absolute `CORTEXA_CODEX_EXECUTABLE`        | Runtime model/effort discovery; saved effort can display support unverified until Refresh; historical `0.159.0` hint is not a current-version support guarantee |

Codex defaults to the installed executable at
`/Applications/ChatGPT.app/Contents/Resources/codex-cli/bin/codex`. Authenticate the
owner's dedicated store with the installed CLI's supported login procedure;
Cortexa does not log in for the owner or accept credentials via the UI. The store
must have no `config.toml` or `AGENTS.md`. Do not reuse the coding assistant's
personal configuration or copy auth files into the repository. The retained A/B
QA used an owner-approved file store and the actual installed binary; that
artifact-specific success does not prove compatibility with every installed
version or managed-policy environment.

Optional local tokens are native environment inputs, never WebView or SQLite
fields. Enable local authentication only when needed and bind the token to the
same normalized endpoint using `CORTEXA_LM_STUDIO_TOKEN` with
`CORTEXA_LM_STUDIO_TOKEN_ENDPOINT`, or `CORTEXA_OLLAMA_TOKEN` with
`CORTEXA_OLLAMA_TOKEN_ENDPOINT`. Endpoint validation allows loopback only,
normalizes `localhost`, and rejects embedded credentials, query/fragment,
redirect/proxy substitution and noncanonical hosts. A loopback runtime can still
have its own network behavior; selecting it is not proof of local inference.
Private-note use with unknown locality needs explicit owner acknowledgement.

All setup names above are checked against native source. This audit did not
launch, log in, install a runtime, refresh models or send prompts. Supply secrets
privately to the native process; never put actual values in commands shared in
chat, source, screenshots, logs, instructions or notes. A standard launch starts
Simulation without acquiring provider credentials.

### Using Bots

Select a bot, connection, exact model and supported effort; save its preferences,
then start a fresh native conversation. Saving does not make a request. Start
captures profile/revision; a later settings change invalidates the old context.
Explicit disclosure/acknowledgement precedes Send. Refresh queries model/account
metadata and is a separate network/runtime operation, not a generated reply.
Discovery proves neither entitlement nor successful generation.

Completed turns supply bounded follow-up context (four user turns, 16,384 history
characters; message limit 4,096 characters). Individual history is volatile on
main. Private notes are owner-written and bot-isolated; Memory Off retains the
saved note but excludes it. Collaboration excludes notes regardless of profile.
No automatic learning, cross-bot note sharing, tool execution or Knowledge RAG is
performed. Identity/personality influences text, not authority.

### Streaming and failure limits

The native host limits each run to 60 seconds and 8,192 visible characters, with
a 128-update journal. Highly fragmented replies can hit `resource_limit` before
the visible-text cap. Partial output can be retained without claiming completion.
Stop is terminal locally; late events are rejected and ownership cleanup gates
another Send. Transport failure, malformed streams, account/model errors,
incomplete output and cleanup failures have fixed sanitized errors. Recovery is
an explicit new owner action; Cortexa does not automatically resubmit or fall
back to Simulation/another provider. Codex internal retries and usage budgets
remain runtime-controlled, and Stop cannot promise immediate remote termination.

OpenAI/Codex native success, streaming, follow-up, Stop/recovery and research
workflow evidence is retained for the tested builds in the
[live-provider review](reviews/2026-10-06-live-provider-e2e-post-increment-review.md).
Anthropic/local discovery and live operation, other collaboration routes and
later candidate runtime changes are not established by that report. External QA
temporary artifacts were unavailable in this audit; no new live verification is
claimed. Runtime configuration denials are not complete OS containment proof.

## Historical provider milestone checkpoints — preserved

---

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
