# End a working session

## Goal

Leave the repository in a state that another assistant session can resume without access to the current conversation.

Use `.agents/skills/session-end/SKILL.md` when repository skills are available.
Otherwise use `prompts/workflows/end-session.md`, which incorporates the
post-increment quality and marker closeout.

## Procedure

1. Stop only task-owned development processes when no longer needed; preserve other tasks.
2. Inspect all changes:

   ```bash
   git status --short --branch
   git diff --stat
   git diff
   ```

3. Run relevant final checks under the Risk-Based Validation Policy; reuse valid unchanged-input stages with provenance. The full cross-cutting suite includes:

   ```bash
   npm run format:check
   npm run lint
   npm run typecheck
   npm run test
   npm run build
   npm run tauri -- build --no-bundle
   ```

4. Record each command as passed, failed, not run, or manual verification pending. Include why any required command was not run.
5. Run the deterministic repository inventory:

   ```bash
   python3 .codex/hooks/session_end_gate.py
   ```

   Resolve every conflict and investigate any unexpected staged, unstaged, or
   untracked path.

6. Run `$quality-gate` to compose architecture, security, code-health,
   technical-debt, and roadmap-readiness review. Do not silently fix advisory
   findings or reorder the roadmap.
7. Review changes against `CODE_REVIEW.md` and `SECURITY.md`.
8. Update:
   - `HANDOFF.md`
   - `PROJECT_STATUS.md`
   - `NEXT_STEPS.md`
   - `DECISIONS.md` when a durable decision changed
   - `CHANGELOG.md`
   - `TROUBLESHOOTING_LOG.md` for a resolved or active failure
   - The applicable file under `docs/increments/`
   - Any active plan under `docs/plans/`
9. Put an exact copy-paste resume prompt in `HANDOFF.md`.
10. Confirm no secrets, build output, personal data, or local logs were added.
11. Run `$post-increment-gate`, create the consolidated report, and finalize the deterministic completion marker only when the result is `PASS` or `PASS WITH ADVISORIES`.
12. Confirm `python3 .codex/hooks/post_increment_gate.py status` reports the active increment as complete and valid.
13. Record the QA-build archival/cleanup disposition using the procedure below,
    including exact retained paths and reasons. Implementation completion remains
    separate; a completion marker never authorizes cloud transfer or removal.
14. Record the separate worktree archival disposition below. Ordinary QA-build
    cleanup always protects worktrees; completion is not archival authorization.
15. Show the final Git status.

## Task-owned QA-build cleanup

The optional [on-demand inventory checker](STORAGE_INVENTORY.md) observes only
explicit paths and local receipt identities. It produces private retain/defer
reports, grants no removal eligibility and is never run automatically by closeout.
All prerequisites below still require fresh agent/owner review.

This is a conditional completion/closeout step, not a deletion command or blanket
cleanup authorization. It does not authorize a commit, push, PR, merge or release.
The archive stage is agent-guided work in the owning task, not a completion-event
executor, background schedule or standing permission. Session completion, idleness
and chat archival trigger no transfer or removal. Existing hooks validate completion
evidence only; they do not execute this procedure.

Use existing authorization only when it explicitly covers the concrete operation.
Bind archive creation/upload to the exact screened inventory and private destination;
bind independent download/isolated restore to the named archive identities; bind
local removal separately to the exact unchanged members and passing recovery proof.
One owner instruction may cover all stages explicitly. Otherwise obtain the missing
authority for the reviewable inventory before that stage. This documentation grants
none of those operations, process signals, credentials or permission changes.
Never delete cloud objects or convert them to online-only under this procedure.

### Eligibility: every condition must be established

- Required implementation, automated checks and native QA are complete and passed.
- Required owner review is complete, with its approval retained.
- Publication and merge are complete where required and separately authorized;
  applicable post-merge verification passed. Record evidence or a justified
  not-applicable disposition for conditional requirements; absence of publication
  authorization is not a waiver of required publication.
- No active session, process, pending acceptance or rollback requirement depends
  on the artifacts. Check across tasks sharing the paths, not just this session.
  An idle session or lack of a visible process alone does not establish this.

Unknown, failed or pending conditions mean **retain**. Keep the review build while
owner review or mandatory acceptance is pending. Never mark incomplete QA passed,
rewrite historical failure, or waive rollback needs to make artifacts eligible.
This step does not change existing implementation gates or grant successor authority.
Explicit local-retention instructions override optional flags, age, apparent
redundancy and archive coverage. Release only exact owner-named resources through
a separate recorded release; an earlier task-specific release is not general
authority. Reconcile dated dispositions with fresh owning-task evidence without
rewriting history or reintroducing already completed cleanup as pending work.

### Before removal

1. Record each exact absolute artifact path, resolved location, type, logical and
   allocated size, filesystem, owning task and ownership evidence. Identify shared
   consumers, complete directory membership and the proposed retained/removed
   disposition. Bind a versioned manifest and its SHA256 to each selected member:
   original/resolved path, archive member mapping, kind, bytes, mode, content hash
   for regular files, symlink target, device/inode, relevant timestamps and hardlink
   group/link count. Record necessary directory metadata and permitted recovery
   differences. Inventories are exact members, not globs or recursive parent grants.
   Explicit exclusions and local holds always take precedence.
   A temporary location or filename prefix alone is not ownership proof. Do not
   follow links into other trees; stop on unexpected links, shared content, changed
   identity, unresolved attribution or evidence drift.
2. Confirm source and necessary build configuration are durably preserved outside
   temporary directories, including uncommitted source where needed. Retain the
   exact revision/patch, dependency/lockfile and toolchain identities, sanitized
   build configuration and rebuild/recovery instructions. Never copy credentials
   or owner data into a cleanup evidence package. Rebuildability does not replace
   required acceptance or rollback artifact custody.
3. Preserve acceptance results, sanitized logs, manifests, hashes, approvals and
   recovery instructions outside temporary directories and outside the deletion
   inventory. Verify the durable copies against their recorded identities before
   removing originals. Keep the evidence-to-source/build binding, original failures
   and any missing-evidence qualifications; do not manufacture missing receipts.
4. Confirm any signals target only task-owned processes using captured process
   identity and current executable/start identity; recheck before signaling and
   verify exit. Never use broad process-name matching, stale PIDs or unrelated
   process termination. If another session still needs the process/artifact,
   retain it. Record no signaling when none is needed.
5. Complete the private OneDrive archive and recovery checks below before removal.
   Then refresh eligibility, authorization, full member identities and complete
   membership, symlink/hardlink dependencies, mounts, registered worktrees and current
   task/process/runner consumers across sharing tasks. A mounted or actively needed
   item is retained; no unmount or signal is implied. A process snapshot alone is
   not proof of nonuse. Retain the current inventory, dependency evidence and
   same-filesystem baseline measurements. Stop affected members on new/unlisted
   contents, drift, unexplained dependencies, inaccessible checks or platform denial.
   Continue independent authorized safe candidates without widening their scope.

### Private OneDrive archival and recovery proof

Apply these checks within the existing before-removal procedure. Preserve the
selected archive bytes and recovery evidence before approving any original's
local retirement; rebuildability alone is not a replacement for historical QA.

1. **Screen and identify.** Use only the owner-approved private OneDrive Personal
   destination under `Cortexa-QA-Archive`, with a unique dated task folder. Record
   exact task ownership, inventory/manifest version and hash, selected originals,
   source revision or preserved dirty patch, lockfile/toolchain/configuration and
   acceptance/provenance bindings. Confirm no credentials, authentication stores,
   owner application databases or unapproved personal content enter the upload.
   Keep sensitive inventories/locators in approved private task evidence outside
   Git and outside all deletion/scratch inventories. Do not expose tokens, passwords
   or authenticated download URLs. Existing encrypted recovery needs separately
   retained key custody; this procedure neither acquires keys nor changes encryption.
2. **Bind every archive.** Record its exact filename, byte count, SHA256, source
   manifest hash, original-to-member mapping and declared recovery limitations.
   Split parts must have complete, nonoverlapping coverage or explained verified
   redundancy; no selected member may lack a covering archive. Confirm source
   stability through archive preparation and preserve the original manifest. For
   raw disk images, file-level selection does not preserve unused sectors, event
   history or original volume identity. Do not remove an original image on that
   basis without a separate exact release accepting the limitations. Record required
   modes, links, directory metadata, extended attributes and signing/provenance
   needs; retain originals when required recovery properties cannot be verified.
3. **Verify private availability.** After an authorized upload, confirm the completed
   object in the exact owner account/folder with supported authenticated cloud
   evidence. Record object locator/version where available, expected size and the
   observed private sharing state with time/method. Local sync presence, a request
   log, filename, placeholder or completed upload alone is insufficient proof of
   remote custody or recovery. Missing access or unknown privacy means retain;
   never change account, sharing or permissions to bypass the check.
4. **Independently download and restore.** Retrieve the actual cloud object through
   an authenticated web download or supported cloud read into separately identified
   task scratch. Do not substitute the upload/staging copy or a sync-cache read.
   Require a completed download with exact byte count and SHA256. Inspect archive
   members before extraction; reject absolute paths, traversal, duplicate/ambiguous
   members, escaping links and unsupported entries. Extract only into a new isolated
   directory, never over live source, Git, worktrees, targets or application data.
   Compare every selected member against the frozen manifest for kind, bytes/hash,
   mode, symlink target and each required metadata property. Record hardlink topology
   and any approved equal-file restoration limits. Execute no restored program,
   helper, hook, acceptance command or migration. Preserve added quarantine; no
   security bypass or renewed executable compatibility/acceptance is implied.
5. **Preserve recovery records too.** Keep source/configuration bindings, archive
   index, manifests, coverage, approvals, original failures, sanitized acceptance
   records, recovery instructions and limitations outside deletion scope. Verify
   their durable copies. Preserve them in a separately identified recovery-record
   archive with its own independent download/restore PASS receipt alongside every
   payload receipt; an unchanged covering records archive may be reused under the
   same identity/coverage rules. Check that all prerequisites needed for recovery are
   accessible. Keep final verification receipts and seals external to the archives
   they verify, with durable approved custody; do not create a self-verifying cycle.
6. **Record proof and reuse safely.** Each PASS receipt binds archive filename,
   bytes/hash, manifest hash, cloud observation, completed independent-download
   identity, isolated restore location, compared member count/properties, result,
   time/method, limitations and zero executed files. Bind receipt hashes in the
   archive index. Reuse unchanged passing evidence only after confirming exact
   identity, member coverage, record integrity and all qualifications; refresh
   current private availability and local dependencies. If unchanged remote bytes
   cannot be established from bound evidence/object version, obtain fresh authorized
   verification or retain. A changed object, missing recovery proof or unexplained
   discrepancy stops its affected candidates. Never
   repeat a completed upload/download/restore merely because the task resumed.

Keep headroom for archive staging, independent downloads and isolated restoration;
choose bounded batches and record a task-specific free-space floor before any
later execution. Insufficient capacity means defer, not premature original deletion.
Any scratch retirement also needs explicit scope and passing preserved receipts;
never infer that a temporary path is disposable. Cloud archives and their synced
paths remain excluded from local removal to prevent cloud deletion through sync.
The completed October 21 cleanup's helpers/allowlists and exact releases are frozen
historical evidence, not reusable execution authority for new candidates.

### Removal boundaries and reporting

Eligible items may include task-owned disposable QA app bundles, isolated build
output directories and temporary fixtures after all dependencies are resolved.
Remove only the individually identified disposable items. A directory is eligible
only when its complete contents and ownership have been checked; an allowlisted
parent must not authorize removal of unexpected descendants or shared content.
Use only verified exact members, then remove an explicitly authorized directory
only when empty. Preserve unlisted members and target/evidence/reconciliation
parents. Blanket directory deletion and `git clean` are prohibited. Never run `cargo clean`
against an active or shared target directory.

Never remove source checkouts, linked worktrees, Git metadata, owner application
data, credentials, historical failure records, required evidence or recovery
backups. When protected and disposable content are mixed, retain the containing
path unless an exact safe disposable subset is established. A backup directory
containing a review app remains protected; do not infer it is disposable by name.

After authorized removal, verify the exact removed paths are absent and retained
source/configuration/evidence/recovery identities still match. Record every success,
partial failure and retained item. Measure allocated artifact bytes before/after
and available space on the same filesystem; report actual observed space recovery
separately from logical sizes or estimates. Account for concurrent disk activity,
APFS clones/snapshots or delayed reclamation; if attribution is uncertain, report
that uncertainty instead of claiming an estimated total as recovered space.

The durable cleanup receipt and handoff must identify:

- Eligibility evidence, cleanup authorization, owner/task and exact inventory.
- A per-path receipt for every removal, skip, failure or uncertain result: action,
  result/reason, owning task, authorization reference, inventory hash and expected
  identity, covering archive and verification-receipt identities, time/method and
  post-action observation. Directory receipts also bind complete checked membership.
- Exact removed/retained paths, failures, preservation results and actual same-volume
  before/after available space. Keep logical/allocated artifact sizes separate from
  observed recovery; do not add APFS allocation estimates or promise attribution.
- Preserved source/configuration/evidence locations and hash verification results.
- Exact rebuild commands, required nonsecret environment/toolchain/dependency
  setup and recovery instructions; rebuilding does not recreate historical QA.
- If cleanup is deferred: the exact pending condition, artifact dependencies,
  review-build location and next decision/check. Record zero removal and zero
  space recovered when no cleanup ran; no speculative cleanup schedule.

On a partial or unknown result, append the receipt and inspect current per-path
state before retrying; an absent original alone is not proof of successful removal
or authorization. Keep prior failures immutable. Verify exclusions and exact cloud
objects remain preserved. Complete independent authorized safe members only.

Use the handoff to distinguish **retained** (exact hold/dependency), **archive
pending**, **recovery verified**, **removal authorized**, **removed** and
**deferred/partial** (exact reason and next decision/check). Recovery verified does
not mean removal authorized. Implementation can remain complete while archival or
cleanup is deferred. When no removal ran, report zero removed and no space recovery
claim; do not schedule speculative cleanup. No executor, scheduler or completion
trigger is implemented by documenting these dispositions.

Do not change frozen acceptance reports or completion fingerprints to record
later cleanup. Use an additive durable cleanup receipt/handoff through the existing
authorized documentation route. Required evidence must remain accessible after
cleanup; a receipt referencing a deleted-only file is not preserved evidence.

## Task-owned worktree archival

This separate optional closeout procedure never follows automatically from QA-build
cleanup, a completion marker, PR merge, session idleness or chat archival. Worktrees
remain protected by ordinary QA-build cleanup. This procedure documents prerequisites;
it does not itself authorize archival, removal, publication or process signals.

### Eligibility and explicit authorization

Establish every condition before proposing an exact worktree for archival:

- All required implementation, automated checks and native/manual acceptance passed;
  required owner review is complete and its approval is retained.
- Required publication/merge was separately authorized and completed, and applicable
  post-merge verification passed. Record evidence or a justified not-applicable
  disposition for conditional requirements; missing authority never waives them.
- Exact task ownership is established, including all contents and shared consumers.
  No active session/task/process, pending acceptance or rollback requirement depends
  on the checkout, its ignored files or artifacts. An idle session alone is insufficient.
- Source, necessary build configuration and required evidence are durably preserved
  and verified as below. Unknown, failed or pending conditions mean **retain**.
- The owner explicitly authorizes archival of the identified worktree and its precise
  disposition after reviewing the inventory, preservation and recovery record.
  Task completion or authorization to delete QA builds does not provide this authority.

**Persistent-memory worktrees and artifacts remain retained pending C7 and all
remaining closeout requirements.** Keep the review build and recovery evidence;
never mark incomplete QA passed or relabel historical failure to permit archival.

### Before archival

1. Record the exact absolute/resolved checkout path, owning task, Git common-directory
   relationship, HEAD, branch or detached state, local changes, unpushed commits,
   untracked and ignored inventory, logical/allocated sizes and filesystem. Use
   task/session attachment and Git records together; names or location are not proof.
   Check linked paths and all consumers without following links into other trees.
   Stop on shared content, ambiguous ownership, unexpected links or identity drift.
2. Prefer the supported managed recoverable archive. For Codex-managed worktrees,
   inspect `list_artifacts` and use only the exact attached worktree `identityKey`
   with `archive_worktree` after explicit authorization. The manager snapshots local
   changes, unpushed commits and non-ignored untracked files before removing the
   checkout. Necessary ignored files are **not** included automatically. Primary,
   pinned or shared worktrees, initialized submodules and embedded Git repositories
   are unsupported: retain them. A non-managed or unsupported checkout needs a
   separately reviewed procedure; never fall back to forced removal, blanket deletion,
   `git worktree remove`/`prune` or `git clean` under this procedure.
3. Independently preserve exact source revision/history and any local/untracked
   source, necessary nonsecret build configuration, lockfile/toolchain identities,
   acceptance results, sanitized logs, approvals, manifests, hashes, historical
   failures and recovery instructions outside this checkout and temporary directories.
   Inventory necessary ignored files separately, including checkout-local gate state,
   admission/closure receipts and required QA/recovery artifacts. Verify durable
   copies against recorded hashes before action. Do not put credentials or owner
   application data into Git snapshots or evidence packages; stop if such content
   would be captured or protected custody cannot be established, without reading
   secret values. Rebuildability does not replace historical evidence or backups.
4. Retain a durable pre-archival inventory/receipt outside the checkout with source
   and evidence bindings, manager recovery expectations, required ignored-file
   copies, sizes, baseline disk measurements and exact recovery steps. Confirm
   preserved source/configuration can reconstruct the task and retained evidence is
   readable. Missing or inaccessible evidence is a blocker, not preservation proof.
5. Recheck all prerequisites, current identities, dependencies and explicit archival
   authorization immediately before the supported operation. No process termination
   is implied; resolve process dependencies under separate applicable authorization.
   Do not archive owner data, credentials, required evidence or recovery backups
   whose verified durable custody would be lost, and never manually delete shared
   Git metadata or source outside the specifically authorized managed checkout.

### Preservation verification, recovery and reporting

After an authorized managed operation, verify the exact checkout disposition and
archived attachment, record the recovery snapshot identity/locator, and verify that
preserved source/local/untracked identities, necessary ignored-file copies and
required evidence remain accessible and match the inventory. A successful tool
response alone is not preservation proof. Record partial or uncertain outcomes;
inspect current attachment state before retrying, and stop without a deletion fallback.

For Codex-managed recovery, record how to select the archived `identityKey` from
`list_artifacts` and use `restore_worktree` under separate authorization. Restoration
recreates the original path with detached HEAD; previously uncommitted changes are
in the snapshot commit, not restored as staged/unstaged changes. Record original
branch/index disposition and instructions to recover preserved necessary ignored
files and rebuild from the bound source/configuration. Do not claim restoration
was tested unless it actually was; restoration/testing is not authorized by archival.

The durable receipt and additive handoff must report authorization/eligibility,
exact archived or retained paths and reasons, snapshot locator, preserved source,
evidence and ignored-file locations/hashes, verification results, failures and
recovery/rebuild instructions. Measure allocated bytes and available space on the
same filesystem before/after; report observed recovery separately from logical
size or estimates, qualifying APFS snapshots/clones and concurrent activity.
Do not rewrite frozen reports or completion fingerprints to record later archival;
use the existing authorized additive documentation route. If deferred, record the
exact pending condition and next decision/check, with zero archival/removal and
zero space recovered when no action ran. Keep required evidence accessible;
references to files available only inside a removed checkout do not preserve them.

## Gate or access blockers

Preserve the rejection and evidence, identify the exact restriction, and use an
existing supported authorized route. Do not disable hooks or bypass platform
approval rejection to complete a milestone. Missing access is a blocker; finish
independent work before handing back.

## Required handoff content

- Phase and increment.
- Last completed task.
- Branch and working-tree state.
- What works.
- What is partial.
- What is broken or unknown.
- Next recommended task.
- Exact resume prompt.
- Files changed.
- Commands run and results.
- Consolidated report path and quality-gate result.
- Warnings and open questions.

## Exit condition

A new session can begin by reading the repository alone and can reproduce the last known result. A completed implementation increment also has a valid passing post-increment report and completion marker.

## D-134 milestone operation

Follow AGENTS.md and ENGINEERING_GUIDE.md: objective, exclusions and acceptance
are the scope boundary. Path counts are informational; preserve protected paths,
attribution and unrelated work. Continue authorized routine work through review,
documentation and truthful finalization. Reuse valid unchanged-input verification
with provenance; distinguish implemented, automatically verified, live verified,
deferred and blocked criteria. Historical FAIL evidence and D-133's separate
acceptance route remain immutable. Request approval only at AGENTS.md boundaries.
