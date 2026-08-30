# V0 prerequisite — sanitized Keychain-scoped identity discrepancy check

Status: Exact scoped check and owner observations Passed; parent recovery
remains Blocked
Owner: Henry Dang
Last updated: 2026-08-29
Parent increment: `v0-xcode-developer-id-recovery-execution`
Baseline: `0931df66c389bdc13c705d1259706c4d3770761c`
Depends on: D-072, D-075, D-076, D-095, TS-017, and the blocked Xcode
Developer ID recovery execution

## Goal

Define one exact, target-Mac, read-only check that asks macOS Security.framework
whether the current user's default Keychain exposes exactly one valid
code-signing identity whose private display label begins with the fixed
`Developer ID Application: ` certificate-class prefix.

The check exists only to narrow the discrepancy between the owner's categorical
Keychain Access observation and the earlier zero-identity CLI result. It does
not prove Apple certificate provenance, successful signing, non-exportability,
absence of prior export, exclusive owner custody, application-identifier
binding, or future Keychain access.

## User-visible outcome

At documentation-amendment closeout, this plan produced only a reviewable plan
and the identity check was **Not run**. On 2026-08-29, the owner supplied the
complete residual-risk confirmation required below and approved one exact run.
That run emitted only `sanitizer_version=keychain_identity_v1` and
`identity_check=passed_one_label_matched_valid_codesigning_identity`.

The owner later approved a documentation-only correction of three
post-increment review findings. This revision now discloses that OS trust
evaluation may use cache, log, diagnostic, or other state; the account-home and
Keychain paths appear in the Python process and in child argv/`HOME` where
same-user process inspection or OS auditing may observe them; and the query
enumerates signing-identity metadata while requesting no private-key bytes,
password values, or provider credentials. Approval of this correction is not
acceptance of those residual boundaries and did not itself authorize
execution. The owner's later complete confirmation separately accepted those
boundaries and authorized the single run recorded above.

The approved execution could report only:

- `sanitizer_version=keychain_identity_v1`;
- one closed `identity_check` category from the allowlist below;
- `authorization_prompt=observed | not_observed`; and
- `state_changed=not_observed | suspected`.

The wrapper and workflow report may not emit or record a path, username,
account, team, certificate name, private display label, fingerprint, serial,
hash, raw stdout, raw stderr, screenshot, terminal transcript, or exception
text. This output guarantee does not claim that local process inspection,
Endpoint Security, system auditing, unified logging, or OS trust services cannot
observe or retain metadata outside the wrapper-owned result surface.

## Scope

- Resolve the current user's default Keychain path internally through the
  read-only no-`-s` form of `/usr/bin/security default-keychain -d user`.
- Pass that private path as one argv value to exactly one read-only
  `/usr/bin/security find-identity -v -p codesigning <keychain>` invocation.
- Capture all child-process output in bounded volatile memory before emitting
  anything.
- Parse every nonblank line against a closed grammar and reject drift,
  ambiguity, warnings, diagnostics, and malformed output.
- Count only valid identity rows whose private label begins exactly
  `Developer ID Application: `.
- Emit one fixed result category and terminate. There is no retry.

## Explicit non-goals

No Keychain query ran during the documentation-only amendment. The later check
does not open Keychain Access or Xcode, intentionally call Apple or another
network service, unlock a Keychain, enter a password, use Touch ID, change a
search list or default Keychain, inspect a certificate separately, find or read
a key, change trust, import, export, create, delete, revoke, renew, replace,
sign, notarize, build, or modify repository source, configuration,
dependencies, entitlements, profiles, capabilities, permissions, scripts,
hooks, or workflows. This draft does not claim that the operating system's
trust evaluation is network-free.

The following commands and options remain prohibited:

- `security list-keychains -s`;
- `security default-keychain -s`;
- `security unlock-keychain`;
- `security find-key`;
- `security dump-keychain`;
- `security find-certificate`;
- `security set-key-partition-list`;
- `codesign`, `xcodebuild`, `openssl`, `sudo`, or any retry or alternate tool.

## Existing behavior and constraints

- Xcode created and lists one Developer ID Application certificate record.
- The owner categorically confirmed that Keychain Access displays that
  certificate with a private key beneath it.
- Earlier sanitized default-search-list CLI checks reported zero matching
  certificates and zero usable code-signing identities.
- The exact one-time scoped check established current Security.framework code-
  signing-policy visibility of one label-matched valid identity. It did not
  expose or retain the Keychain path and did not establish provenance, non-
  exported owner control, or signing success.
- The installed `security find-identity` help exposes no local-only or
  network-disabled option. Its code-signing policy evaluation may delegate to
  OS trust services. The owner explicitly accepted that disclosed residual
  boundary for the consumed one-time run; it is not eliminated or contained.
- The recovery gate is now terminally `failed` / `FAIL` / `Blocked` without a
  completion marker; V0-3 remains Blocked.
- Earlier screenshots crossed the identifier-free evidence boundary. This plan
  prohibits screenshots and raw output.
- A later security review confirmed that this scoped pass cannot support a
  retrospective “never exported,” technical non-extractability, or exclusive-
  custody claim. The owner authorized and approved the documentation evidence
  milestone in the separate
  [present-use/local-signing plan](2026-08-29-v0-developer-id-present-use-local-signing-proof.md),
  and D-096 fixes prospective `technical_nonextractability`,
  `historical_absence_of_export`, and `exclusive_custody` to `not_proven`; it
  did not authorize the operation. This one-time query remains consumed and
  must not be repeated as part of that future plan.

## Files expected to change

This planning amendment may change documentation only:

- `CHANGELOG.md`
- `ARCHITECTURE.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/increments/v0-xcode-developer-id-recovery-execution.md`
- `docs/plans/2026-08-28-personal-assistant-v0-xcode-developer-id-recovery.md`
- `docs/plans/2026-08-29-v0-keychain-scoped-identity-discrepancy.md`
- `docs/reviews/2026-08-28-v0-xcode-developer-id-recovery-execution-post-increment-review.md`

No product, test, dependency, lockfile, configuration, entitlement,
capability, permission, workflow, hook, or script path may change.

## Trust-boundary map

| Element                | Exact boundary                                                                                                                                                                                                                                                                                   |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Actor                  | Project owner on the personally controlled target Mac                                                                                                                                                                                                                                            |
| Input                  | Current real/effective user IDs, the OS account record resolved through `getpwuid`/`opendirectoryd`, default-Keychain path plus local file metadata, and Security.framework identity metadata from that exact Keychain                                                                           |
| Private transient data | The directory-service passwd record may include username, home, GECOS, and shell fields even though the wrapper uses only `pw_dir`; account-home and Keychain paths then enter Python plus child argv/`HOME`, while identity labels and fingerprints enter bounded child pipes and Python memory |
| Output                 | Sanitizer version, one closed identity result, and two owner-observed manual categories                                                                                                                                                                                                          |
| Storage                | No intended wrapper-owned durable persistence or output redirection. The quoted here-document uses shell-managed transient input. Directory-service and OS trust cache, socket, diagnostic, unified-log, or other state effects are not disproven or contained                                   |
| Network                | No intentional request; `opendirectoryd` may consult configured local or remote directory systems, and OS trust-service certificate or revocation traffic is not disproven or contained                                                                                                          |
| Authority              | Signing-identity metadata observation only; no private-key bytes, password values, provider credentials, state mutation, signing, or later-increment authority                                                                                                                                   |

## Exact one-time check

The following block is the complete check that was approved and executed once
on 2026-08-29 after the owner accepted the disclosed OS trust and local process-
metadata boundaries. It must not be executed again under the consumed approval.
The recorded run used this block verbatim, with shell tracing disabled, no
additional input/output redirection beyond the quoted here-document supplying
the reviewed Python body, and no retry.

Post-execution review found that the approval did not separately disclose the
`pwd.getpwuid()` account-resolution boundary. On macOS that lookup is served
through `opendirectoryd`, which may use configured local or remote directory
systems and OS-owned cache/socket/log state, and may transiently return passwd-
record fields the wrapper does not use. No evidence establishes that remote
directory traffic occurred, and the wrapper emitted none of those fields, but
the historical residual-risk acceptance is incomplete. This is an additive
disclosure finding only: do not rerun the consumed wrapper to investigate it.

```bash
/usr/bin/python3 -I -B - <<'PY'
import os
import pwd
import re
import resource
import selectors
import subprocess
import sys
import time

SECURITY = "/usr/bin/security"
TIMEOUT_SECONDS = 10.0
MAX_CAPTURE_BYTES = 16_384
MAX_LINES = 64
MAX_LINE_BYTES = 1_024
MAX_PATH_BYTES = 4_096
TARGET_PREFIX = "Developer ID Application: "

ROW = re.compile(
    r'^\s*([1-9][0-9]*)\)\s+([0-9A-Fa-f]{40})\s+"([^"\x00-\x1f\x7f]{1,512})"\s*$'
)
SUMMARY = re.compile(r"^\s*([0-9]+)\s+valid identities found\s*$")
QUOTED_PATH = re.compile(r'^\s*"([^"\\\x00-\x1f\x7f]{1,4094})"\s*$')

ALLOWED = {
    "passed_one_label_matched_valid_codesigning_identity",
    "blocked_none",
    "blocked_multiple",
    "blocked_invalid_keychain_scope",
    "blocked_query_error",
    "blocked_timeout",
    "blocked_output_limit",
    "blocked_unrecognized_output",
    "blocked_wrapper_error",
}


class Closed(Exception):
    def __init__(self, outcome):
        super().__init__()
        self.outcome = outcome if outcome in ALLOWED else "blocked_wrapper_error"


def stop_child(process):
    if process.poll() is not None:
        return
    try:
        process.terminate()
        process.wait(timeout=0.5)
        return
    except (OSError, subprocess.TimeoutExpired):
        pass
    try:
        process.kill()
        process.wait(timeout=0.5)
    except (OSError, subprocess.TimeoutExpired) as error:
        del error
        raise Closed("blocked_wrapper_error")


def capture(argv, account_home):
    environment = {
        "HOME": account_home,
        "LANG": "C",
        "LC_ALL": "C",
        "PATH": "/usr/bin:/bin",
    }
    try:
        process = subprocess.Popen(
            argv,
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=environment,
            shell=False,
            close_fds=True,
        )
    except Exception as error:
        del error
        raise Closed("blocked_wrapper_error")

    selector = selectors.DefaultSelector()
    stdout = bytearray()
    stderr = bytearray()
    total = 0
    deadline = time.monotonic() + TIMEOUT_SECONDS

    try:
        selector.register(process.stdout, selectors.EVENT_READ, stdout)
        selector.register(process.stderr, selectors.EVENT_READ, stderr)
        while selector.get_map():
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise Closed("blocked_timeout")
            events = selector.select(remaining)
            if not events:
                raise Closed("blocked_timeout")
            for key, _ in events:
                chunk = os.read(key.fileobj.fileno(), 4_096)
                if not chunk:
                    selector.unregister(key.fileobj)
                    continue
                total += len(chunk)
                if total > MAX_CAPTURE_BYTES:
                    raise Closed("blocked_output_limit")
                key.data.extend(chunk)

        remaining = max(0.01, deadline - time.monotonic())
        try:
            status = process.wait(timeout=remaining)
        except subprocess.TimeoutExpired:
            raise Closed("blocked_timeout")
    except Closed:
        stop_child(process)
        raise
    except BaseException as error:
        stop_child(process)
        del error
        raise Closed("blocked_wrapper_error")
    finally:
        selector.close()
        for pipe in (process.stdout, process.stderr):
            try:
                pipe.close()
            except BaseException:
                pass

    if status != 0:
        raise Closed("blocked_query_error")
    if stderr:
        raise Closed("blocked_unrecognized_output")
    return bytes(stdout)


def decode_lines(payload):
    try:
        text = payload.decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        del error
        raise Closed("blocked_unrecognized_output")
    raw_lines = text.splitlines()
    if len(raw_lines) > MAX_LINES:
        raise Closed("blocked_output_limit")
    for line in raw_lines:
        if len(line.encode("utf-8")) > MAX_LINE_BYTES:
            raise Closed("blocked_output_limit")
    return text, raw_lines


def account_home():
    user_id = os.getuid()
    if user_id == 0 or user_id != os.geteuid():
        raise Closed("blocked_invalid_keychain_scope")
    try:
        home = pwd.getpwuid(user_id).pw_dir
    except (KeyError, TypeError) as error:
        del error
        raise Closed("blocked_invalid_keychain_scope")
    if (
        not home
        or not os.path.isabs(home)
        or os.path.normpath(home) != home
        or len(home.encode("utf-8")) > MAX_PATH_BYTES
        or any(ord(character) < 32 or ord(character) == 127 for character in home)
    ):
        raise Closed("blocked_invalid_keychain_scope")
    return home


def parse_default_keychain(payload, home, check_filesystem=True):
    _, raw_lines = decode_lines(payload)
    lines = [line for line in raw_lines if line.strip()]
    if len(lines) != 1:
        raise Closed("blocked_invalid_keychain_scope")
    match = QUOTED_PATH.fullmatch(lines[0])
    if match is None:
        raise Closed("blocked_invalid_keychain_scope")
    path = match.group(1)
    if (
        not path
        or not os.path.isabs(path)
        or os.path.normpath(path) != path
        or len(path.encode("utf-8")) > MAX_PATH_BYTES
        or any(ord(character) < 32 or ord(character) == 127 for character in path)
    ):
        raise Closed("blocked_invalid_keychain_scope")
    user_root = os.path.realpath(os.path.join(home, "Library", "Keychains"))
    canonical_path = os.path.realpath(path)
    try:
        if os.path.commonpath((user_root, canonical_path)) != user_root:
            raise Closed("blocked_invalid_keychain_scope")
    except ValueError as error:
        del error
        raise Closed("blocked_invalid_keychain_scope")
    if check_filesystem and (os.path.islink(path) or not os.path.isfile(path)):
        raise Closed("blocked_invalid_keychain_scope")
    return path


def classify_identities(payload):
    _, raw_lines = decode_lines(payload)
    lines = [line for line in raw_lines if line.strip()]
    if not lines:
        raise Closed("blocked_unrecognized_output")
    summary = SUMMARY.fullmatch(lines[-1])
    if summary is None:
        raise Closed("blocked_unrecognized_output")

    fingerprints = set()
    target_count = 0
    rows = lines[:-1]
    for expected_ordinal, line in enumerate(rows, start=1):
        match = ROW.fullmatch(line)
        if match is None or int(match.group(1)) != expected_ordinal:
            raise Closed("blocked_unrecognized_output")
        fingerprint = match.group(2).lower()
        if fingerprint in fingerprints:
            raise Closed("blocked_unrecognized_output")
        fingerprints.add(fingerprint)
        label = match.group(3)
        if label.startswith(TARGET_PREFIX):
            if not label[len(TARGET_PREFIX) :].strip():
                raise Closed("blocked_unrecognized_output")
            target_count += 1

    if int(summary.group(1)) != len(rows):
        raise Closed("blocked_unrecognized_output")
    if target_count == 0:
        return "blocked_none"
    if target_count == 1:
        return "passed_one_label_matched_valid_codesigning_identity"
    return "blocked_multiple"


def expect_closed(function, expected):
    try:
        function()
    except Closed as blocked:
        if blocked.outcome == expected:
            return
    raise Closed("blocked_wrapper_error")


def self_test():
    fake_home = "/Users/example"
    fake_keychain = "/Users/example/Library/Keychains/login.keychain-db"
    if (
        parse_default_keychain(
            f'"{fake_keychain}"\n'.encode("utf-8"),
            fake_home,
            check_filesystem=False,
        )
        != fake_keychain
    ):
        raise Closed("blocked_wrapper_error")
    expect_closed(
        lambda: parse_default_keychain(
            f"{fake_keychain}\n".encode("utf-8"),
            fake_home,
            check_filesystem=False,
        ),
        "blocked_invalid_keychain_scope",
    )
    expect_closed(
        lambda: parse_default_keychain(
            b'"/tmp/not-a-user-keychain"\n',
            fake_home,
            check_filesystem=False,
        ),
        "blocked_invalid_keychain_scope",
    )

    target_a = f'1) {"A" * 40} "{TARGET_PREFIX}Example"\n'
    target_b = f'2) {"B" * 40} "{TARGET_PREFIX}Second"\n'
    non_target = f'2) {"C" * 40} "Unrelated identity"\n'
    cases = (
        (
            (target_a + "1 valid identities found\n").encode("utf-8"),
            "passed_one_label_matched_valid_codesigning_identity",
        ),
        (b"0 valid identities found\n", "blocked_none"),
        (
            (target_a + target_b + "2 valid identities found\n").encode("utf-8"),
            "blocked_multiple",
        ),
        (
            (target_a + non_target + "2 valid identities found\n").encode("utf-8"),
            "passed_one_label_matched_valid_codesigning_identity",
        ),
    )
    for payload, expected in cases:
        if classify_identities(payload) != expected:
            raise Closed("blocked_wrapper_error")

    malformed = (
        target_a.encode("utf-8"),
        (target_a + "1 valid identities found\n1 valid identities found\n").encode(
            "utf-8"
        ),
        (target_a + "one valid identities found\n").encode("utf-8"),
        (target_a + "2 valid identities found\n").encode("utf-8"),
        f'2) {"A" * 40} "{TARGET_PREFIX}Example"\n1 valid identities found\n'.encode(
            "utf-8"
        ),
        f'1) {"Z" * 40} "{TARGET_PREFIX}Example"\n1 valid identities found\n'.encode(
            "utf-8"
        ),
        f'1) {"A" * 40} "{TARGET_PREFIX}"\n1 valid identities found\n'.encode(
            "utf-8"
        ),
        (
            target_a
            + f'2) {"A" * 40} "{TARGET_PREFIX}Second"\n'
            + "2 valid identities found\n"
        ).encode("utf-8"),
    )
    for payload in malformed:
        expect_closed(
            lambda payload=payload: classify_identities(payload),
            "blocked_unrecognized_output",
        )


def emit(outcome):
    if outcome not in ALLOWED:
        outcome = "blocked_wrapper_error"
    payload = (
        "sanitizer_version=keychain_identity_v1\n"
        f"identity_check={outcome}\n"
    )
    try:
        sys.stdout.write(payload)
        sys.stdout.flush()
    except BaseException:
        os._exit(3)
    return (
        0
        if outcome == "passed_one_label_matched_valid_codesigning_identity"
        else 2
    )


def run():
    try:
        resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
        self_test()
        home = account_home()
        default_output = capture(
            [SECURITY, "default-keychain", "-d", "user"], home
        )
        keychain = parse_default_keychain(default_output, home)
        identity_output = capture(
            [SECURITY, "find-identity", "-v", "-p", "codesigning", keychain],
            home,
        )
        outcome = classify_identities(identity_output)
        del identity_output
        del default_output
        del keychain
        del home
    except Closed as blocked:
        outcome = blocked.outcome
    except BaseException as error:
        del error
        outcome = "blocked_wrapper_error"
    return emit(outcome)


raise SystemExit(run())
PY
```

## Output and interpretation

The wrapper may emit only the sanitizer version and one wrapper-produced
category. `blocked_unexpected_prompt` is not synthesized by the wrapper; it is
a manual override reported only when the owner observes and cancels a system
prompt.

| Category                                              | Source      | Meaning                                                                                                                                                                            |
| ----------------------------------------------------- | ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `passed_one_label_matched_valid_codesigning_identity` | Wrapper     | The scoped query returned exactly one currently valid code-signing identity whose private label begins with the fixed Developer ID Application prefix. It is not provenance proof. |
| `blocked_none`                                        | Wrapper     | The exact scope returned no matching valid identity. The cause remains undetermined.                                                                                               |
| `blocked_multiple`                                    | Wrapper     | The exact scope returned more than one matching valid identity. No identity may be selected.                                                                                       |
| `blocked_invalid_keychain_scope`                      | Wrapper     | User identity, account home, default-Keychain output, canonical containment, symlink status, or regular-file status was invalid or ambiguous.                                      |
| `blocked_query_error`                                 | Wrapper     | One read-only command returned nonzero. Raw errors remain private and unreported.                                                                                                  |
| `blocked_timeout`                                     | Wrapper     | A command did not finish within ten seconds. Termination and kill waits are each bounded to 0.5 seconds; there is no retry.                                                        |
| `blocked_output_limit`                                | Wrapper     | Captured output exceeded 16 KiB, 64 lines, or 1 KiB per line.                                                                                                                      |
| `blocked_unrecognized_output`                         | Wrapper     | Output, stderr, UTF-8, ordinals, fingerprints, labels, or summary grammar did not match the closed parser.                                                                         |
| `blocked_wrapper_error`                               | Wrapper     | The wrapper or its built-in synthetic parser self-test could not preserve the exact closed procedure.                                                                              |
| `blocked_unexpected_prompt`                           | Manual only | The owner observed and cancelled a password, unlock, Touch ID, access-control, or authorization prompt without entering data.                                                      |

A wrapper pass plus owner-observed `authorization_prompt=not_observed` and
`state_changed=not_observed` establishes only current CLI visibility of one
label-matched valid certificate/private-key identity in the default user
Keychain. It does not satisfy the non-exported-owner-control or signed-build
manual gates, complete the active increment, resolve TS-017's historical root
cause, or authorize V0-3.

## Manual gates and stop conditions

Before the consumed one-time execution, the owner approved this exact plan,
confirmed the target Mac remained personally controlled, accepted possible OS-
managed certificate/revocation traffic and cache/log/state effects, accepted
the local process-metadata visibility of the account-home and Keychain paths,
and acknowledged that signing-identity metadata is enumerated even though
private-key bytes, password values, and provider credentials are not requested.
The following execution controls were applied and now grant no rerun:

The later-discovered `getpwuid`/`opendirectoryd` directory-service,
cache/socket/log, and unused-passwd-field boundary was not separately disclosed
or accepted before execution. It remains Manual verification pending in the
active review and grants no authority to repeat the query.

1. Do not open Xcode, Keychain Access, Apple Developer, a browser, or another
   terminal.
2. Run the wrapper once. Do not use shell tracing, piping, clipboard capture,
   screenshots, recording, or a transcript. The sole permitted input
   redirection is the quoted here-document already present in the exact wrapper;
   add no other input or output redirection.
3. If any password, Keychain unlock, Touch ID, access-control, or authorization
   prompt appears, enter nothing, cancel it, stop the wrapper, and report only
   `identity_check=blocked_unexpected_prompt` and
   `authorization_prompt=observed`.
4. Stop on every blocked category. Do not retry, change the target scope, run a
   raw command, inspect an item, or attempt remediation.
5. Record `state_changed=not_observed` only from the owner's observation. If a
   change is suspected, report `state_changed=suspected` and require a separate
   incident plan; do not attempt cleanup.

The wrapper result is recorded. The owner separately reported
`authorization_prompt=not_observed` and `state_changed=not_observed`; those
observations were not inferred from command output.

## Synthetic review cases

The embedded `self_test()` runs without a child process before the wrapper's
first `security` invocation. A mismatch becomes `blocked_wrapper_error` and
prevents the Keychain query. The documentation amendment initially validated
only embedded-Python syntax. The later exact run invoked `self_test()` and the
two bounded `security` commands once.

| Built-in parser case                                           | Exact expected category                               |
| -------------------------------------------------------------- | ----------------------------------------------------- |
| One well-formed target row and matching summary                | `passed_one_label_matched_valid_codesigning_identity` |
| No rows and zero summary                                       | `blocked_none`                                        |
| Two target rows and matching summary                           | `blocked_multiple`                                    |
| One target plus one well-formed non-target identity            | `passed_one_label_matched_valid_codesigning_identity` |
| Missing, duplicate, localized, or count-mismatched summary     | `blocked_unrecognized_output`                         |
| Malformed ordinal, non-hex fingerprint, or empty target suffix | `blocked_unrecognized_output`                         |
| Duplicate fingerprint                                          | `blocked_unrecognized_output`                         |
| One quoted in-scope synthetic Keychain path                    | accepted privately; no emitted path                   |
| Unquoted or out-of-scope synthetic Keychain path               | `blocked_invalid_keychain_scope`                      |

The subprocess controls are closed operational branches rather than synthetic
child-process tests: nonzero exit maps to `blocked_query_error`; any stderr or
grammar drift maps to `blocked_unrecognized_output`; output excess maps to
`blocked_output_limit`; timeout maps to `blocked_timeout`; and any failed
bounded cleanup maps to `blocked_wrapper_error`. There is no retry.

## Security and privacy considerations

- The wrapper accepts no caller-selected path, label, fingerprint, identity, or
  policy. It rejects root and requires equal real/effective user IDs, derives
  the account home from the OS account database, accepts one strictly quoted
  default-Keychain path, resolves canonical containment beneath that account's
  `Library/Keychains`, rejects a symlink or non-regular target, and never emits
  the path.
- Absolute binaries, argv arrays, `shell=False`, isolated Python mode,
  disabled bytecode, disabled core dumps, closed stdin, fixed C locale, output
  bounds, timeout, bounded terminate/kill/reap waits, closed pipes, strict
  parsing, a caller-unselectable built-in self-test, and zero retries reduce
  injection, persistence, prompt, cleanup, and parser-drift risk.
- The quoted here-document is the sole permitted shell input redirection and
  supplies only the reviewed Python body. No output redirection is permitted;
  shell-managed transient delivery details are not claimed storage-free.
- Captured metadata remains in ordinary short-lived Python memory. Python does
  not guarantee zeroization; the process terminates immediately, and no key
  material is requested or returned.
- The account-home and Keychain paths also appear in the Python process and in
  the `security` child argv and `HOME` environment. The wrapper never emits
  them, but same-user process inspection, Endpoint Security, or OS auditing may
  observe them during the bounded run.
- Resolving that account home through `pwd.getpwuid()` may transiently
  materialize additional passwd-record fields and invoke `opendirectoryd`, which
  can use configured local or remote directory systems plus OS-owned cache,
  socket, diagnostic, or log state. The wrapper used only `pw_dir` and emitted
  no record field; remote traffic is neither observed nor disproven. This
  boundary was discovered after execution and remains unaccepted.
- A label prefix is a bounded presentation discriminator, not independent
  certificate-profile or Apple-chain validation.
- The query intentionally enumerates signing-identity metadata. It does not
  request private-key bytes, password values, generic-password items, provider
  credentials, or key export.
- Code-signing policy evaluation may ask OS trust services to perform
  certificate or revocation work and may use cache, diagnostic, unified-log, or
  other OS-owned state. The CLI has no documented offline flag. The owner
  explicitly accepted these possible effects for the consumed one-time run;
  they were not eliminated, disproven, or contained.
- Every ambiguity denies progression. No raw diagnostic may be used to turn a
  blocked result into a pass.

## Verification commands for this documentation amendment

```bash
awk 'BEGIN { capture=0 } /^import os$/ { capture=1 } capture { if ($0 == "PY") exit; print }' \
  docs/plans/2026-08-29-v0-keychain-scoped-identity-discrepancy.md | \
  /usr/bin/python3 -c 'import ast, sys; ast.parse(sys.stdin.read())'
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri package.json package-lock.json \
  src-tauri/Cargo.toml src-tauri/Cargo.lock .github .codex/hooks scripts
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

The documentation-only amendment initially left application tests and
`npm audit` Not run. The later required stop-hook post-increment gate ran
`npm run verify` and `npm audit --audit-level=low`; both Passed, with 0 npm
vulnerabilities and one existing intentionally ignored opt-in Hermes probe.
The exact target-Mac scoped Keychain check later Passed once. The immutable
historical non-exported-owner-control row remains Manual verification pending,
D-096 fixes prospective technical non-extractability, historical absence of
export, and exclusive custody to `not_proven`, and signing remains Not run. No
further diagnostic execution is authorized.

## Risks

- Security CLI output can drift or localize; strict grammar blocks instead of
  inferring.
- The default user Keychain may not be the Keychain displayed in the owner's
  prior observation; a zero result remains a scoped result, not a root cause.
- A valid identity listing may still fail during signing or private-key access.
- Metadata briefly exists in process memory and private paths appear in child
  argv/`HOME`; the wrapper does not emit or persist them, but same-user process
  inspection or OS auditing may observe them.
- The path can change between metadata validation and the Security.framework
  open; the diagnostic assumes the personally controlled target Mac is not
  concurrently replacing Keychain paths and grants no authority beyond the
  observed result.
- An unexpected system prompt requires owner observation and immediate stop.
- Code-signing policy evaluation may cause OS-managed trust-service traffic or
  cache/log/state effects; this plan neither suppresses nor disproves them.

## Rollback or failure strategy

This planning amendment had no external rollback. The one approved query is
complete and created no intended wrapper-owned state; do not rerun it or save
additional output. OS trust cache/log/state effects are not controlled rollback
targets. If the owner reports a suspected state change, stop and require a
separately approved incident plan. Never unlock, repair, reset, import, export,
delete, revoke, or replace Keychain or certificate state as rollback.

## Decisions made

- Scope only the current user's OS-resolved default Keychain; do not enumerate
  the complete search list or accept a caller-supplied path.
- Use `find-identity`, not `find-certificate`, because the question is current
  certificate-plus-private-key identity visibility under code-signing policy.
- Treat label matching as a narrow visibility discriminator and preserve all
  stronger identity, custody, signing, and lifecycle gates.
- Keep the plan inside the then-active failed recovery increment. It did not
  start a successor gate or convert the prior `FAIL` to a pass; D-097 later
  closed that same increment as terminally failed.

## Discoveries

- Local `security` help confirms that no-keychain `find-identity` uses the
  default search list, while an explicit positional Keychain narrows the query.
- Local documentation defines an identity as a certificate plus private key;
  `-v -p codesigning` restricts output to identities currently valid for the
  code-signing policy.
- `find-certificate` would prove only certificate presence and unnecessarily
  broaden metadata exposure, so it is excluded.

## Progress

- 2026-08-29: The owner authorized drafting this documentation-only plan under
  the active recovery gate and required a stop for approval before execution.
- 2026-08-29: The exact command, parser grammar, closed outcomes, bounds,
  limitations, manual gates, rollback, and stop conditions were drafted. No
  Keychain or Apple command was run.
- 2026-08-29: Security review tightened account-home derivation, canonical path
  confinement, filesystem-type checks, child cleanup bounds, pipe closure,
  parser self-testing, and the label-only pass name. It also identified
  possible OS trust-service traffic as an unresolved execution boundary.
- 2026-08-29: Embedded-Python syntax, documentation formatting and links,
  repository health, secret scanning, protected-path diff, whitespace, and
  session-end validation Passed. The wrapper and Keychain query remained Not
  run, and the active gate remained active.
- 2026-08-29: The required stop-hook post-increment workflow reran full
  verification and npm audit successfully, then retained `FAIL`/Blocked because
  manual evidence and disclosure gates remain unresolved. No completion marker
  was finalized.
- 2026-08-29: The owner approved documentation-only correction of the three
  disclosure findings. The plan now states the OS cache/log/state boundary,
  local process-metadata visibility, and exact signing-identity metadata scope.
  This did not accept residual risk or authorize the wrapper.
- 2026-08-29: Post-correction static architecture/security review,
  embedded-Python syntax, report-manifest consistency, documentation,
  repository, secret-scan, protected-path, and whitespace checks Passed. The
  wrapper remained Not run.
- 2026-08-29: The owner confirmed that the target Mac remained personally
  controlled, accepted the disclosed OS trust traffic/cache/log/state and
  local process-metadata boundaries, acknowledged the exact metadata scope,
  and approved one execution of the revised plan. The exact wrapper ran once
  with no retry and returned
  `passed_one_label_matched_valid_codesigning_identity`; it emitted no path,
  account, label, fingerprint, raw output, or error. The owner then reported
  `authorization_prompt=not_observed` and `state_changed=not_observed`.

## Acceptance criteria

- [x] Exact read-only commands, private scope resolution, parser grammar,
      bounds, and closed outputs are documented.
- [x] Raw identifiers, paths, labels, hashes, errors, and command output cannot
      reach the declared result surface.
- [x] Zero, multiple, malformed, prompt, timeout, output-limit, command, and
      wrapper outcomes fail closed with no retry.
- [x] The plan does not claim non-exportability, custody, provenance, signing,
      V0-3 readiness, or TS-017 root-cause resolution.
- [x] The wrapper runs caller-unselectable synthetic parser cases before its
      first Security.framework subprocess.
- [x] Documentation-tier and embedded-Python syntax validation pass without
      executing the wrapper.
- [x] Wrapper-owned persistence is distinguished from unproven OS trust
      cache/log/state effects.
- [x] Local process-metadata observability of the private Keychain path is
      disclosed.
- [x] Identity-metadata enumeration is distinguished from private-key bytes,
      password values, and provider credentials.
- [x] The owner confirms the target Mac remains personally controlled.
- [x] The owner explicitly accepts or contains possible OS-managed certificate
      or revocation traffic and cache/log/state effects.
- [x] The owner explicitly accepts local process-metadata observability of the
      account-home and Keychain paths, or approves a redesign.
- [x] The owner acknowledges the signing-identity metadata query and its
      exclusion of private-key bytes, password values, and provider credentials.
- [x] The owner reviews and approves this exact completed plan.
- [x] The exact check is executed once under separate approval.
- [x] The owner reports `authorization_prompt=not_observed`.
- [x] The owner reports `state_changed=not_observed`.

## Final results

Planning result: **Passed** documentation-tier validation. The embedded Python
parsed successfully without execution; documentation formatting and links,
repository health, secret scanning, protected-path diff, whitespace, and
session-end checks Passed. The owner-approved disclosure correction also passed
static architecture/security re-review and final documentation-tier validation.
Operational result: **Passed for the exact scoped visibility question only**.
The wrapper ran once and returned
`passed_one_label_matched_valid_codesigning_identity`, establishing that the
current default user Keychain exposed exactly one currently valid code-signing
identity with the fixed Developer ID Application label prefix. It does not
prove provenance, non-exportability, owner custody, signing, or TS-017's
historical cause. Application tests, `npm audit`, and the remaining
documentation checks were previously rerun by the required stop-hook gate and
Passed; npm reported 0 vulnerabilities, and one existing opt-in Hermes
executable probe remained intentionally ignored. The owner reported no prompt
and no visible state change. The immutable historical non-exported-owner-control
row remains Manual verification pending; D-096 fixes prospective technical
non-extractability, historical absence of export, and exclusive custody to
`not_proven`; and signing remains Not run. The recovery increment is therefore
terminally `failed` / `FAIL` / `Blocked` without a completion marker.

## Documentation updates

- [x] `ARCHITECTURE.md`
- [x] `CHANGELOG.md`
- [x] `DECISIONS.md`
- [x] `HANDOFF.md`
- [x] `NEXT_STEPS.md`
- [x] `PLANS.md`
- [x] `PROJECT_STATUS.md`
- [x] `ROADMAP.md`
- [x] `SECURITY.md`
- [x] `TROUBLESHOOTING_LOG.md`
- [x] Active increment, parent recovery plan, and interim review
