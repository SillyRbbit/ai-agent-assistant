# On-demand artifact inventory

This repository maintenance utility observes explicitly supplied local artifact
paths. It does not run inside Cortexa, a hook, session completion or a scheduler.
It never uploads, downloads, restores, removes, launches or signals anything.
The [END_SESSION closeout procedure](END_SESSION.md#task-owned-qa-build-cleanup)
continues to govern all subsequent work. Implementation completion grants no
operation authority.

## Run only against explicit candidates

Use Python3.9 or later on macOS/POSIX with descriptor-relative no-follow support.
Supply one or more `--candidate` absolute canonical paths; no glob, home expansion,
relative path, implicit directory discovery or symlink ancestor is accepted.
Case/Unicode spellings are compared conservatively, and existing directory/file
identities detect physical aliases without following links. Similar scopes can
therefore be retained/deferred even on a case-sensitive volume.
Missing paths are deferred, never classified as previously removed. Overlapping
or duplicate candidates are deferred rather than counted twice. No actual QA
artifact scan was performed while implementing this utility.

Supported report roots are `/private/tmp`, `/private/var/folders` and the current
user’s `.codex/visualizations` (also `/tmp` on non-macOS POSIX). Other locations
are refused rather than assumed to be local.

Create a new owner-only report directory outside any checkout, candidate, evidence
input, cloud sync store or protected application/authentication store. Existing
permissions are never changed. Example using synthetic paths only:

```bash
mkdir -m 700 /private/tmp/cortexa-private-reports
python3 -B scripts/storage_inventory.py \
  --candidate /private/tmp/cortexa-synthetic-export \
  --report /private/tmp/cortexa-private-reports/inventory.json
```

The existing report parent must be owned by the current user with no group/other
permissions. Every candidate, recognized context candidate/receipt input and held
or required sealed path is an output exclusion, including aliases and unresolved
names. Valid exclusions survive unrelated schema errors in parsed context. Scope
checks use existing ancestor device/inode identities and conservative normalized
suffixes; inaccessible/unsafe scope refuses report creation. Final checks and
exclusive creation use the same output-parent descriptor, with a fresh parent
identity check before creation. New reports are exclusively created with initial mode0600 (umask may
further restrict it); existing files and links cannot be overwritten. Do not put
reports, context, inventories, receipt bodies or owner metadata in Git. Standard
output is a content-free success/error category. Exit0 means a report was written,
not that any candidate is eligible; exit2 means report/input-boundary failure.
A failed write can leave a private partial report: do not reuse, delete or overwrite
it automatically; record the failure and choose a new destination after inspection.

## Optional local context

`--context /absolute/private/context.json` adds schema1 assertions and exact
hash-bound local receipt references. All references are read as inert data, with
no execution, imports or commands taken from them. Unknown fields, duplicate keys,
malformed schema defers every candidate; unsupported links, inaccessible context
or ambiguous/unparseable JSON or uncertain scope-container shapes stop report creation with `context_scope_unavailable`;
the checker never silently discards recognized potentially controlling holds.
Invalid/noncanonical scope paths are never discarded to permit an output;
correct them in a separately reviewed invocation. Duplicate keys cannot hide a hold: when scope cannot be recovered
unambiguously, no destination is written. A non-string receipt role is an invalid
reference, including JSON lists/objects;
it produces deferred outcomes rather than an uncaught exception. Output must still
pass independent exclusion checks, so an unsafe requested destination exits2
without creating a report. Unreadable or unparseable data cannot establish the
absence of additional holds or any removal eligibility.

```json
{
  "schema_version": 1,
  "holds": ["/absolute/retained-resource"],
  "required_local_sealed_desktop_inputs": [
    {
      "path": "/absolute/required-evidence",
      "optional_large_manifest": true
    }
  ],
  "candidates": [
    {
      "path": "/absolute/explicit-artifact",
      "owner": "asserted owner identifier",
      "task": "asserted task identifier",
      "evidence": [
        {
          "role": "payload",
          "path": "/absolute/private/payload-VERIFIED.json",
          "sha256": "0000000000000000000000000000000000000000000000000000000000000000",
          "archive_filename": "Example.tar.gz",
          "archive_sha256": "0000000000000000000000000000000000000000000000000000000000000000",
          "archive_bytes": 123
        }
      ]
    }
  ]
}
```

Replace example paths and zeros with actual reviewed identities. A second receipt
uses role `recovery-records`; both roles are required even when a payload receipt
passes. Optional `expected_manifest_sha256` on a candidate compares a prior report's
inventory digest with the observed inventory. It is this utility's canonical JSON
digest, not an interchangeable historical MANIFEST hash. A mismatch remains deferred.

Exact holds apply to the candidate itself, its ancestors and any held descendant.
Required sealed inputs remain held regardless of optional flags. Context supports
no release field; a prior exact release is never generalized. Known source, Git,
checkouts, dependencies, data/auth stores, synced cloud stores and evidence are
conservatively retained. The name screening is defense in depth, not an exhaustive
sensitivity classifier: absent context or an unfamiliar name never proves safety.

Owner/task text is reduced to an assertion hash and `asserted_not_verified` status;
it is not echoed. Filesystem UID/GID is observed separately. Assertions, hashes,
local receipts and their claimed PASS fields are not owner authentication or proof
against malicious same-user editing.

## What is measured and what remains unknown

An inspected member records relative membership, kind, mode, size, device/inode,
link count, UID/GID, modification/change time, regular-file SHA256 and symlink
target as applicable. The report includes a canonical inventory digest and
nonadditive allocated-byte observations. Holds/protected trees stop expansion and
hashing. This is an intentionally incomplete inventory for retained trees, never
an exact deletion list. No outside symlink targets are followed. Hardlinks are
reported as unresolved dependencies without reading their content. No special
file is opened for content. macOS SF_DATALESS placeholders and dataless ancestors
are deferred before opening, preventing known placeholder hydration. Unknown
provider semantics are not a locality guarantee. Filesystem flags join the identity. Different-device mount boundaries stop expansion;
same-device mounts, registered worktrees and mount registries remain unchecked.

Every ancestor is opened without following links; descriptor identities are checked
before/after content reads, membership is compared before/after directory traversal,
and the final candidate ancestry is rechecked. Drift or inaccessible members mark
the observation incomplete; independent siblings/candidates can still finish.
The scan is not an atomic snapshot, filesystem lock or continuing quiescence proof.
Reads may change access times; inode/ctime stability is not raw-volume identity.
The filesystem can still change after observation, and same-user rename races are
not an OS sandbox. Future authorized operations require fresh checks.

Default run-wide limits:64 candidates,10,000 total entries,64MiB total regular-file
hashing, depth64,15seconds checked per candidate and16MiB serialized output.
Scope paths are limited to4096 per output and128 components per path.
Context and each receipt are limited to1MiB. A blocking filesystem operation is not
preempted by the cooperative deadline; no watchdog or signal is used. Large or
partial inventories remain deferred. Partial entries and exact reasons are retained;
a budget failure is never treated as complete membership or zero size.

Retained October21 `VERIFIED.json` fields are recognized only when the exact local
receipt SHA256, archive filename from independent download, archive SHA256/byte
count, PASS, stable source, positive restored count, matching content/modes/links,
zero executed files and distinct absolute download/restore locations agree.
The status is `recorded_pass_identity_matches`, not a fresh restore or full recovery
certification. Receipt bodies, download paths and restored content are not emitted.
The checker does not follow archive-index paths automatically or reopen archives.
Existing allowlists, source manifests, recovery maps and seal formats remain intact.

Private current OneDrive availability, complete source/archive mapping, receipt
provenance/limitations, fresh process/runner/task consumers, registered worktrees,
mounts, local hold releases and operation authority need agent/owner review. They
are explicitly unknown, even with matching receipts. Every candidate therefore
reports `retain` or `defer`, `removal_eligible: false` and fixed reasons. A later
milestone may propose richer verification only with separate owner authorization;
this implementation is complete without granting removal eligibility.

## Closeout and validation

Keep implementation completion separate from inventory completeness, recovery
verification and any archive/removal authorization. No free-space recovery is
claimed; allocated file values cannot be added as APFS physical reclaim estimates.
Normal closeout records per-path deferred/retained reasons and any observed
same-volume free space without attributing concurrent changes to this utility.

Run the focused filesystem tests and repository suite. They use only synthetic
fixtures, including test-owned setup/teardown; no historical QA artifact is touched.
Risk-based full coverage may reuse verified unchanged product stages with command,
result, input and environment provenance. Fresh executable-tool tests, relevant
formatting/repository/security/documentation/whitespace/session checks and scoped
review remain required. Never describe reused evidence as a new full verification
run. See the [milestone plan](../plans/2026-10-21-storage-inventory-checker.md).

## Boundary correction record

The first frozen plan/review/evidence remain historical snapshots. The separately
owner-approved [boundary correction](../plans/2026-10-21-storage-inventory-boundary-correction.md)
adds alias-aware scope checks, complete held/sealed output exclusions and role
type validation. It grants no operation authority and changes no product, hook,
dependency, scheduler or publication boundary. These checks remain point-in-time
observations; they do not lock the filesystem or defeat hostile same-user renames.
