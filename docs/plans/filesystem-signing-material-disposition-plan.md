# Filesystem signing-material containment and disposition plan

- Status: Documentation-only; operational disposition unauthorized
- Date: 2026-08-02
- Authority: D-072, D-076, and the stopped TS-017 contact evidence

## Goal and custody assumptions

Select future abandonment and paired deletion of exactly one unuploaded CSR file
and its filesystem private-key file. No certificate exists; neither file was
used, copied, exported, or backed up; encryption and permissions are unknown;
the pair does not satisfy D-072.

## Future owner-operated procedure

This procedure requires a separate approval and must not run under this plan.

1. Privately resolve exactly the two intended targets without opening or reading
   content. Stop if either target is ambiguous or additional material appears.
2. Reconfirm no upload, use, certificate, copy, export, or backup occurred.
3. Acknowledge that deletion is irreversible and ordinary APFS/SSD deletion does
   not prove cryptographic erasure or removal from snapshots.
4. Delete the CSR and private-key files as one paired disposition. Do not import,
   move, copy, upload, preserve, or use either file.
5. Verify only absence and report the closed sanitized outcomes below.

## Evidence, controls, and stop conditions

Report only `target pair identified: yes | no | not determined`, `additional
material: yes | no | not determined`, each deletion as `completed | not
performed | unavailable`, `remaining copy: yes | no | not determined`,
`uploaded or used: no`, and `certificate created: no`.

Stop before deletion on target ambiguity, unexpected material, a request to
inspect content, cloud/snapshot uncertainty requiring investigation, or any
need to change permissions. Before deletion rollback is stopping; after deletion
there is no rollback, restoration, recovery, or regeneration authority.

Risks are wrong-target deletion, content exposure, unnoticed copies/snapshots,
and overstated erasure. Controls are owner-only exact targeting, no content
inspection, paired disposition, fail-closed ambiguity, sanitized evidence, and
no secure-erasure claim.

## Verification and non-goals

Confirm D-072 and D-076 remain unchanged and run `npm run docs:check`, `npm run
repository:check`, `npm run security:scan`, `git diff --check`, product/dependency
path absence, and `python3 .codex/hooks/session_end_gate.py`.

No inspection, command execution, upload, use, move, rename, copy, export,
backup, deletion, permission change, Keychain action, certificate creation,
signing, credential, Cloudflare, provider, deployment, traffic, code,
dependency, or runtime action. Blocked pending separate operational approval.
