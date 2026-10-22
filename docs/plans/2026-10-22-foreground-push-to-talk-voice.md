# Foreground push-to-talk voice

Status: Implementation, full verification, qualified QA build and target native acceptance passed. Final report/session/quality/Stop validation controls formal completion.

## Objective and baseline

Add Mac-only en-US foreground on-device dictation to the existing Personal Assistant Bots composer, editable review before explicit Send, and optional explicit read-aloud with immediate Stop. Verified remote main f954b74c9122683133d5b028abdcae8f792e60ee, tree f7bd3dc2861cb630f1bdddb67a51bc7f0ad4b560. Memory C7 and Tasks acceptance remain preserved; no replay. Ledger89/96 is unchanged.

## Ownership and independence

Fresh codex/foreground-push-to-talk-voice worktree. Voice owns new frontend/application/client and Rust/Swift speech modules, composer-only integration, voice declarations/commands/lifecycle and helper packaging/privacy metadata. RAG owns Knowledge/retrieval and future context assembly. Tasks registration and migration11 remain unchanged. Parallel CM maintenance owns Desktop/memory gate modules/tests and additive documents; dashboard owns its separate publication tooling. No shared checkout writes or messages. Voice does not rely on their missing startup evidence or change their failed criteria. A separate worktree alone is not an admission claim: use supported general schema-2 readiness and begin.

## Contract

Explicit Enable voice requests native microphone/speech permissions. Hold to talk or accessible Start/Finish records at most30seconds. Release ends capture and permits at most10seconds finalization; Cancel/focus loss/navigation/close/quit discard unaccepted results and complete bounded cleanup. Present transcript separately; Use transcript appends only if the combined4096-character draft fits; never auto-Send or silently truncate. Explicit existing provider acknowledgement and Send remain unchanged. One voice operation application-wide; no concurrent voice/generation. Read answer accepts only a native-validated completed response, max4096characters/60seconds, one installed standard voice, no queue. Stop reading immediately cancels playback. All operations are window/conversation/session bound; late/replayed/unknown data reject. Owned helper stop/reap deadline5seconds; unresolved cleanup retains ownership.

## Architecture and privacy

A fixed bundled application-owned Swift helper uses Apple Speech/AVFoundation, forced requiresOnDeviceRecognition with supportsOnDeviceRecognition required. No server fallback, asset installation APIs, model downloads, arbitrary executable/path/endpoint or generic shell permission. Rust owns validation, session identity, finite pipe framing and lifecycle. Audio stays native and transient; application-owned queue max8MiB. No raw audio, private transcript or response in logs/evidence/SQLite; only user-accepted explicit Send uses existing persistence. No memory/RAG hints enter speech. Minimal child environment excludes provider credentials. Preserve unsafe_code=forbid, CSP and capability policy. OS-managed buffers/diagnostics are outside an app-level secure-erasure claim. Native permission attribution, installed recognition support and offline usability require actual QA; declarations are not proof.

## Acceptance

- V1: Explicit capture, native permission denial/revocation and unavailable assets/language/device handled safely.
- V2: Hold/release and accessible actions, review/edit/append/discard, Unicode/draft limits and no automatic Send.
- V3: Strict versioned privacy/identity/replay/order/size parsing; fixed helper and wrong-window/conversation rejection.
- V4: Capture/playback stop, task/readers reaped, late result rejection, ownership retention on failed cleanup and subsequent-use recovery.
- V5: Voice draft uses unchanged ordinary Send and deterministic completed-history/private-memory rules; cancelled dictation creates no turn.
- V6: Preserve the RAG seam through ordinary Send; actual future RAG parity remains conditional, not claimed against absent implementation.
- V7: Optional explicit native-validated completed-answer speech, installed standard voice, immediate Stop and no feedback recording.
- V8: No application audio retention/private diagnostics; on-device-only configuration and target offline usability established.
- V9: Focused Rust/Swift/frontend checks, full npm run verify, native QA, preservation/reviews/session/quality/actual Stop and truthful disposition.

## Resources and timing

One fresh eight-hour window with1800seconds reserved for closeout; authoritative external execution seal records start/deadline. Fresh8GiB allocation: target6GiB, dependencies1.5GiB, temporary256MiB, QA128MiB, source/before-copies64MiB, evidence64MiB including1MiB closeout. Independently measure disjoint domains before/through execution; keep8GiB available floor,64MiB headroom and other sessions' outstanding holds (conservatively11GiB plus129MiB). No old memory/runtime reservation is borrowed. Existing public dependencies copied into isolated locations without installation/network. Finite command timeouts and actual elapsed receipts; no unbounded retries. No paid/model dispatch; native Simulation and fixed synthetic phrases only.

## Validation and recovery

Focused deterministic Rust protocol/lifecycle and Swift reducer/helper tests; frontend mock interaction and strict typed-client tests; build/packaging privacy checks. Full npm run verify after final product edit, native QA in a dedicated app/home, then documentation/repository/security/whitespace/session/quality and post-increment workflow. Required unverified manual checks prevent PASS. No RAG-specific acceptance is invented. Preserve all failed receipts and attributable corrected versions. Verified baseline BEFORE.zip is external; restore selected files only under safe matching attribution, never reset/stash/clean. No commits/push/merge, HP/gate implementation, image utility, protected cleanup, database migration, provider change, acquisition or successor.

## Progress

- [x] Owner contract, integrated main, source/ownership and fresh resources inspected.
- [x] Verified recoverable baseline archive; isolated public dependencies prepared.
- [x] Readiness checks and ordinary admission.
- [x] Implementation, affected tests and full verification.
- [x] Native acceptance and preservation.
- [x] Required pre-finalization report/session/quality checks; actual finalization and configured Stop control completion and are retained in the external final handoff.

## Implementation review checkpoint

Native voice protocol and lifecycle are implemented. The first admission request rejected uppercase criterion labels; a preserved lowercase-only request passed unchanged semantics. Initial Swift checks rejected the macOS27 deprecated tap and then its refined error-parameter spelling; the corrected availability split passed warnings-as-errors, with a macOS14 deployment target and architecture pinned from Cargo. First frontend run found a real delayed-Stop race; immediate controller wake fixed it and all43 focused/frontend-composer tests passed. Eleven native voice/protocol/artifact/ownership tests passed. Swift deterministic decision tests passed. Strict TypeScript and frontend lint passed after attributable React-hook/type corrections, with every failed receipt preserved.

The native interlock reads only current Personal Assistant presentation, blocks conversation mutation/Send during voice ownership and releases only the matching lease. Provider context and storage remain unchanged. Permission sheets may retain a pending authorization operation across focus loss because no audio is captured; recording/playback still cancel on lost native focus. Native playback cancellation requires a stopped-synthesizer callback; it is not inferred from calling Stop. Reaped process and three reader/writer threads are required before release. Helper metadata is bound across the launch, including replacement with identical bytes; descriptors, content and before/after identity remain checked.

Full verification, native permissions/offline recognition, optional playback, preservation and truthful final report remain required. No voice request, microphone capture, paid/model request or native app launch has yet occurred. RAG is not implemented here.

## Necessary registration inventory synchronization

The first full verification stopped on the exact native-command inventory because voice-client.ts and the three owner-approved registrations were absent. Added only the exact voice import/invoke/registration entries in scripts/repository_health.py and focused negative cases in its existing test module. This is verification of the authorized voice registration, not lifecycle/admission gate maintenance or a generic boundary exception. Unknown clients, extra commands/payloads/imports, missing registrations, CSP/capability changes and all old checks still reject. Original files and failed verification are retained externally.

Added independent helper EOF/absolute-lifetime safeguards: parent-input loss exits within5seconds even if MainActor is stuck; otherwise an off-main130-second fail-safe caps the permission-request lifetime. Recording/playback have their earlier shorter owner deadlines. No new capture starts or background activation are added.

## 2026-10-22 — Foreground voice implementation and partial native acceptance

The owner-authorized foreground-push-to-talk-voice milestone is implemented in its separate worktree. Final `npm run verify` passed: 645 frontend tests, 549 Rust unit tests, integration/repository/hook coverage, strict TypeScript/Clippy/formatting and native release build. The separately identified QA bundle passed source/artifact/helper/privacy-metadata binding. Deterministic Swift arbitration and real no-audio helper rejection were checked. Preserve every earlier failed test, rejected admission, probe assertion and intermediate artifact.

The direct deny-network QA launch had a null runtime bundle identifier. Its exact executable and outer bundle paths matched, but computer association was unavailable. The cause remains unknown: a suspected dev-plist mechanism was not established by artifact inspection. That attempt was terminated and reaped with complete streams and absent owned group. Normal NSWorkspace opening then established the exact bundle, executable and complete bounded window census; computer control worked. This route does not inherit the earlier network-denial policy. Do not claim whole-host offline proof.

Native QA confirmed a fresh Simulation/Memory-Off namespace with zero prior turns, no automatic capture, a bounded unsuccessful permission attempt, released ownership and one subsequent completed Simulation turn. Explicit read-aloud reached the Reading state; immediate Stop in the same tool invocation returned idle with controls released and no error. Audible hardware output is not independently attested. No hosted request was sent; ledger89/96 remains unchanged.

Positive microphone permission/capture, review from actual dictation and offline on-device recognition remain unverified. Computer control explicitly rejects the OS UserNotificationCenter app for safety; no bypass was attempted. An owner-operated Enable/approve step is pending while independent closeout work continues. No acceptance marker is justified by these partial results. The current app is the isolated Cortexa Foreground Voice QA instance; finish owned cleanup and preserve its data before final custody.

Voice source remains fixed to on-device en-US, reviewed transcript before existing Send, no saved audio and no cloud fallback/download API. The 8 MiB check bounds each native buffer handoff; there is no retained application audio queue. OS-managed audio/recognition buffers are outside an application secure-erasure or complete-memory-bound claim. RAG-specific acceptance remains conditional on a separate integrated implementation; context/migrations/Tasks and accepted memory C7 remain unchanged.

Retain the complete authorized voice objective across continuation: resolve supported in-scope issues, validate affected behavior, complete actual native acceptance, preservation/reviews/documentation and truthful session/quality/Stop; do not stop at this checkpoint. The original eight-hour execution seal and 1800-second reserve remain controlling. Existing resources/builds/history stay retained; no Git publication or successor is authorized. See this living plan.

## 2026-10-22 — Voice window02 verified implementation and current native gap

This additive current record supersedes only the earlier partial-checkpoint counts and pending build statements; historical failures and snapshots remain unchanged. The recognition-classifier source snapshot09e84bcfea046b9a4ca60a6d92793388de08b5b00d73dcb393ad479bdb91cbaf passed full `npm run verify`:649 frontend tests,554 Rust unit tests, integration/hooks/repository checks, strict TypeScript, Clippy, formatting and release build. The separately bound Recognition07 QA bundle passed. The Vite chunk-size notice remains Advisory. No lockfile, dependency, capability, CSP, provider, storage/context or migration change was made.

Actual current05 native authorization and Recording were observed. Both a bounded synthetic-phrase attempt and the separate physical06 owner-ready opportunity ended in strict recognition_failed without a reviewable transcript. No missing NSError or acoustic cause was reconstructed. Current05 active Cancel preserved the draft and stored history and released controls. Unchanged current04 playback reached Reading, direct Stop and separate natural completion with cleanup; physical audibility was not attested. These scoped observations are reused only through source-impact review.

An exact current04 parent/UUID-bound OS diagnostic demonstrated a Swift callback executor assertion. A deterministic off-main callback fixture reproduced the mechanism; explicit Sendable boundaries before MainActor hops corrected it. New passing tests do not explain unrelated historical failures. The later four-file recognition correction adds only installed-SDK-supported closed failure labels; unknown errors and cancellation remain failures, with no raw NSError, identifiers, audio or arbitrary diagnostic retention.

Recognition07 initially rejected authorization protocol; later readiness actions returned unavailable. Current exact PID1176 and original window1574 remained verified, while independent foreground observations were false at several attempts. One earlier owner foreground transition was observed, but its continuity across subsequent actions is unknown. Computer-control title-bar/AX actions have not established usable foreground activation; this does not prove a speech-service or permission cause. A direct owner-operated Enable/Start/synthetic phrase/Finish handoff is pending. Native focus guards remain strict. Positive target dictation, actual review/edit/Use/Discard and offline recognition remain required and unverified; automatic tests cannot satisfy them. No acceptance marker or voice completion is claimed.

The owner-approved time-only extension has its own EXECUTION-SEAL-WINDOW02.json: start1792677919.17153,deadline1792688719.17153,1800-second closeout reserve. Preserve the expired original deadline and every consumed operation separately. The fresh8GiB envelope, domain caps, other-session holds,8GiB floor and64MiB headroom remain unchanged. This task has dispatched zero hosted/model requests; historical ledger89/96 is not reset. All review builds, verified source/document before-copies and failed receipts remain retained.

The full standing objective remains authorized: complete supported voice acceptance, routine attributable repair, preservation, current documentation/reviews/session/quality and truthful post-increment disposition. A diagnostic or checkpoint is not a stopping point. No native/RAG acceptance is inferred from mocks; future integrated RAG parity remains conditional. No publication, downloads/assets, settings/permission expansion, provider requests, HP, image utility, protected cleanup or successor is authorized. External evidence is in foreground-voice-20261022-01; relevant records include SOURCE-REVIEW-RECOGNITION-07.json,QA-ARTIFACT-RECOGNITION-07.json and NATIVE-READINESS-07-CURRENT-CHECKPOINT.json.

## 2026-10-22 — Foreground voice target acceptance and final review

This additive record supersedes the earlier pending native result without changing historical failures. In a fresh owner-08 QA home, the same verified Recognition07 build produced actual on-device transcript reviews. The agent directly edited a review, used it to append to an existing draft, observed zero automatic Sends, then explicitly sent the accepted text through local Simulation. Exactly one completed turn/context1/omitted0 was observed. A separate actual review was cleared to empty and Discard preserved the existing draft and stored turn. Native keyboard hold/release, safe no-speech handling, active Cancel and cancellation when the exact recording window was minimized were checked. Draft/history remained unchanged by discarded or cancelled capture.

The standard Window menu's exact makeKeyAndOrderFront item supplied a supported computer-operated key-window route; this does not explain earlier association/focus failures. Actual current-build Reading, direct Stop, and separate natural completion were observed with released controls and no terminal error. Hardware audibility is not independently attested. Preserve the earlier unavailable attempts, the uncorroborated owner Use report, the corrected external compiler invocation, the premature Recording assertion and the unusable artificial audio fixture independently; no old failure is promoted.

Installed SDK semantics bind supportsOnDeviceRecognition to operation without network and requiresOnDeviceRecognition to keeping request audio on-device when supported. The fixed helper requires both before the successful target recognition. This is live evidence for the enforced on-device request; no whole-host network-disconnection, packet-capture, OS-telemetry or secure-erasure claim is made. No cloud fallback, asset download or provider dispatch was added. Future RAG parity remains separately conditional; this work uses unchanged ordinary Send and preserves Knowledge/context/storage/migration11.

Full source-bound verification remains649 frontend tests/554 Rust unit tests plus integration, strict checks and release build; the qualified QA bundle passed. Only documentation/report work follows that product snapshot. Both QA instances terminated normally without force. Final custody was captured after their last SQLite observations; neither finalized database may be reopened by this closeout. All source, data, historical failures, six retained review/recovery bundles and backups remain; no deletion, archival or disk recovery is claimed. The eight-hour history and separate three-hour window, their consumption,8GiB envelope and all floors/headroom remain recorded externally. This task added zero hosted/model dispatches to the historical ledger89/96.

The consolidated voice review records actual acceptance separately from publication. Final current documentation/session/security/preservation and configured Stop must validate before a completion claim. The next proposed work is separately owner-authorized voice Git integration, not another product milestone. No staging, commit, push, merge or publication occurred here.
