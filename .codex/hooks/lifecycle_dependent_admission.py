"""Two exact owner-authorized dependent routes; no executor or waiver.

Same-user evidence integrity is not authentication. Missing custody stays unknown.
"""
from __future__ import annotations

import json
import hashlib
import stat
import struct
import os
import re
from pathlib import Path
from typing import Any

import lifecycle_closure as closure
import post_increment_gate as gate

PREDECESSOR = "vps-dual-label-operating-mode"
SUCCESSOR = "vps-dual-label-operating-mode-remediation"
BRANCH = "codex/docs/vps-dual-label-operating-mode"
MEMORY_PREDECESSOR = "persistent-conversations-bot-memory"
MEMORY_SUCCESSOR = "persistent-memory-codex-continuation"
MEMORY_BRANCH = "codex/persistent-bot-memory"
MEMORY_ROOT = "/Users/hdang/.codex/worktrees/persistent-bot-memory/ai-agent-assistant"
MEMORY_STATE = "9eb254fd81388a34121cfc6022529752ad17ab030ea0987da820e3fb65182726"
MEMORY_REPORT = "1124aadd7e5d1471cf3599288e5a091332548f63b89453c295a704fa60f94bd6"
MEMORY_CANDIDATE = "90b8408c4ed3e477ee211017dcbaba9aed46bfe3ec9a91925cbde5be05f7aeaa"
MEMORY_DOCS = {"ARCHITECTURE.md", "CHANGELOG.md", "DECISIONS.md", "HANDOFF.md",
               "NEXT_STEPS.md", "PLANS.md", "PROJECT_STATUS.md", "TESTING_GUIDE.md",
               "TROUBLESHOOTING_LOG.md"}
MEMORY_SOURCE = {"src-tauri/src/codex_connection.rs",
                 "src/infrastructure/tauri/agent-chat-client.ts",
                 "src/infrastructure/tauri/agent-chat-client.test.ts"}
MAINTENANCE = "memory-dependent-admission"
MAINTENANCE_FILES = {".codex/hooks/lifecycle_dependent_admission.py",
                     ".codex/hooks/tests/test_lifecycle_dependent_admission.py",
                     ".codex/hooks/post_increment_gate.py"}
STATE_SCHEMA = 5
STORE = Path(".codex/state/dependent-admission")
POINTER = STORE / "consumed.json"
PRESERVATION_COMMAND = "python3 -B .codex/hooks/post_increment_gate.py dependent-preservation"
ROOT_DOCS = {"CHANGELOG.md", "DECISIONS.md", "HANDOFF.md", "NEXT_STEPS.md", "PLANS.md",
             "PROJECT_STATUS.md", "docs/github/SELF_HOSTED_RUNNER.md"}
EVIDENCE_KEYS = {"comparison", "historical_manifest", "snapshot", "archival_provenance",
                 "unavailable_inventory", "surviving_preservation"}
UNAVAILABLE_HASHES = {
    "adoption-package": "b22baadaac8f9f20bdcacd73c474a0738671cccb6f24859979f5f9c06cc5dfc0",
    "correction-package": "9d492c20200cfd9b852e8e0bd704eea59effb73320d387f246181ed220a72dc3",
    "validation-package": "de56d7fc3ec5fcf555c5d0cbe530566cb932dcd74945285b9bffbb702228b179",
    "bounded-index": "a18d041df1006378b157f15a80ed00193494c150c3eac3aa6c95e6164493d060",
    "publication-index": "c576cf82bb95d5b3f9a9443e0ab359112e81ab7f86c14b71b270d85e507195c1",
    "sidebar-index": "adea2cd545f1344d66f24acaad3a1916b26edc3ccb59b8374fd0a465b9e80984",
    "bounded-ignored-state": None, "publication-ignored-state": None, "sidebar-ignored-state": None,
}
UNAVAILABLE = set(UNAVAILABLE_HASHES)
require = closure.require
keys = closure.keys
digest = closure.digest
encode = closure.encoded


def parse(data: bytes) -> Any:
    def unique(pairs):
        result = {}
        for key, value in pairs:
            require(key not in result, "dependent duplicate JSON key")
            result[key] = value
        return result
    return json.loads(data, object_pairs_hook=unique)


def read(root: Path, path: str) -> bytes:
    return closure.read_bytes(closure.relative(root, path))


def reference(root: Path, value: Any) -> bytes:
    keys(value, {"path", "sha256"})
    closure.sha(value["sha256"])
    data = read(root, value["path"])
    require(digest(data) == value["sha256"], "dependent evidence hash mismatch")
    return data


def branch(root: Path) -> str:
    return gate._run_git(root, "branch", "--show-current").decode().strip()


@closure.guarded
def shape(state: dict[str, Any]) -> None:
    require(state.get("schema_version") == STATE_SCHEMA, "dependent schema mismatch")
    require(state.get("increment_id") in {SUCCESSOR, MEMORY_SUCCESSOR}, "dependent task mismatch")
    require(not any(k in state for k in ("closure_lineage", "acceptance_lineage",
                "predecessor_disposition", "successor_disposition")), "dependent mixed lineage")
    keys(state["dependent_lineage"], {"admission"})
    closure.sha(state["dependent_lineage"]["admission"])


def exclusive(root: Path, name: Path, data: bytes) -> None:
    target = closure.relative(root, name.as_posix())
    require(not any(p.is_symlink() for p in [target, *target.parents]), "dependent unsafe store")
    target.parent.mkdir(parents=True, exist_ok=True)
    # Exclusive creation is intentionally fail-closed after interruption/replay.
    try:
        fd = os.open(target, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        with os.fdopen(fd, "wb") as handle:
            handle.write(data)
            handle.flush()
            os.fsync(handle.fileno())
    except FileExistsError as error:
        raise gate.GateError("dependent replay rejected") from error



def prospective_report(root: Path, name: str, successor: str = SUCCESSOR) -> None:
    """Admission may bind an absent report; finalization still requires its bytes."""
    gate.validate_relative_path(name)
    relative = Path(name)
    expected = rf"{gate.REPORT_DATE_PATTERN}-{re.escape(successor)}-post-increment-review\.md"
    require(relative.parent == gate.REPORTS_RELATIVE_PATH
            and re.fullmatch(expected, relative.name) is not None,
            "dependent prospective report identity mismatch")
    path = root / relative
    require(not any(p.is_symlink() for p in [path, *path.parents]),
            "dependent prospective report symlink")
    require(path.parent.is_dir() and path.parent.resolve().is_relative_to(root.resolve()),
            "dependent prospective report parent unsafe")
    require(not path.exists() or path.is_file(), "dependent prospective report not regular")


def snapshot_copies(root: Path, request: dict[str, Any]) -> None:
    snapshot = parse(reference(root, request["evidence"]["snapshot"]))
    keys(snapshot, {"files"})
    files = snapshot["files"]
    require(isinstance(files, list) and bool(files), "dependent snapshot evidence missing")
    hashes = set()
    paths = set()
    for item in files:
        reference(root, item)
        require(item["path"] not in paths, "dependent duplicate snapshot copy")
        paths.add(item["path"])
        hashes.add(item["sha256"])
    required = {request["predecessor"]["state"]["sha256"],
                request["predecessor"]["report"]["sha256"], request["index_sha256"]}
    require(required.issubset(hashes), "dependent essential snapshot copy missing")
    survival = parse(reference(root, request["evidence"]["surviving_preservation"]))
    keys(survival, {"result", "scope", "files"})
    require(survival["result"] == "PASS" and survival["scope"] == "surviving-evidence"
            and survival["files"] == files, "dependent surviving copy inventory mismatch")


def obligations(original: dict[str, Any], request: dict[str, Any]) -> None:
    # unresolved consumes a report body, so construct its exact manifest wrapper.
    body = gate.MANIFEST_START + json.dumps(original) + gate.MANIFEST_END
    expected = set(closure.unresolved(body))
    keys(request["obligations"], expected)
    for value in request["obligations"].values():
        keys(value, {"criterion", "required"})
        require(value["required"] is True and value["criterion"] in request["milestone"]["acceptance"],
                "dependent obligation omitted or waived")
    prior_criteria = original.get("milestone", {}).get("criteria", {})
    require(bool(prior_criteria) and set(prior_criteria).issubset(request["milestone"]["acceptance"]),
            "dependent acceptance criterion omitted")
    if request["increment_id"] == MEMORY_SUCCESSOR:
        require(request["acceptance_change"] is None, "memory acceptance changes forbidden")
        require(all(v["criterion"] == "native-qa" for v in request["obligations"].values()),
                "memory unresolved native obligations misattributed")
        return
    change = request["acceptance_change"]
    keys(change, {"criterion", "original_sha256", "replacement", "replacement_command", "owner_authorization"})
    require(change["criterion"] == "preservation" and "preservation" in prior_criteria,
            "dependent acceptance change out of scope")
    require(change["original_sha256"] == digest(encode(prior_criteria["preservation"])),
            "dependent original criterion mismatch")
    require(change["replacement"] == "surviving-evidence-and-explicit-unavailable-custody"
            and change["replacement_command"] == PRESERVATION_COMMAND,
            "dependent acceptance replacement out of scope")


def custody(data: bytes) -> None:
    inventory = parse(data)
    keys(inventory, {"records", "complete_historical_preservation", "absence_of_loss"})
    require(inventory["complete_historical_preservation"] is False and inventory["absence_of_loss"] is False,
            "dependent historical preservation overclaim")
    rows = inventory["records"]
    require(isinstance(rows, list) and len(rows) == len(UNAVAILABLE), "dependent unavailable inventory incomplete")
    seen = set()
    for row in rows:
        keys(row, {"id", "availability", "recorded_sha256"})
        require(row["id"] in UNAVAILABLE and row["id"] not in seen
                and row["availability"] == "unavailable", "dependent custody disposition mismatch")
        seen.add(row["id"])
        require(row["recorded_sha256"] == UNAVAILABLE_HASHES[row["id"]], "dependent custody reference mismatch")


def identity(root: Path, request: dict[str, Any]) -> None:
    expected_branch = MEMORY_BRANCH if request["increment_id"] == MEMORY_SUCCESSOR else BRANCH
    if request["increment_id"] == MEMORY_SUCCESSOR:
        require(str(root.resolve()) == MEMORY_ROOT, "memory checkout binding mismatch")
    require(request["root"] == str(root.resolve()) and request["branch"] == expected_branch
            and branch(root) == expected_branch, "dependent root/branch identity mismatch")
    require(request["head"] == gate.current_head_commit(root), "dependent HEAD mismatch")



# One completed, task-bound maintenance amendment; no new admission command.
INDEX_AMENDMENT = Path('.codex/state/memory-index-maintenance.json')
INDEX_MAINTENANCE = 'memory-index-preservation-maintenance'
INDEX_FILES = {'.codex/hooks/lifecycle_dependent_admission.py',
               '.codex/hooks/tests/test_lifecycle_dependent_admission.py'}
INDEX_CRITERIA = {'cache', 'binding', 'boundaries', 'verification'}


def index_semantics(data: bytes) -> bytes:
    """Strict SHA1 index-v2 subset; normalize only cached stat words."""
    require(type(data) is bytes and 32 <= len(data) <= 1024 * 1024, 'index bounds')
    require(hashlib.sha1(data[:-20]).digest() == data[-20:], 'index checksum')
    signature, version, count = struct.unpack_from('>4sII', data)
    require(signature == b'DIRC' and version == 2 and count <= 10000, 'unsupported index format')
    out = bytearray(data[:-20]); end = len(out); pos = 12; previous = None
    for _ in range(count):
        start = pos
        require(pos + 62 < end, 'index entry truncated')
        words = struct.unpack_from('>10I20sH', data, pos)
        flags = words[-1]
        require(not flags & 0x4000, 'unsupported index extended flags')
        require(words[6] in {0o100644, 0o100755, 0o120000, 0o160000}, 'unsupported index mode')
        zero = data.find(b'\0', pos + 62, end)
        require(zero != -1, 'index path truncated')
        name = data[pos + 62:zero]
        require(0 < len(name) <= 4096 and not name.startswith(b'/')
                and all(p not in {b'', b'.', b'..'} for p in name.split(b'/')), 'index path invalid')
        require(flags & 0xfff == min(len(name), 0xfff), 'index path length mismatch')
        key = (name, (flags >> 12) & 3)
        require(previous is None or previous < key, 'index duplicate or order')
        previous = key
        pos = start + ((zero + 1 - start + 7) // 8) * 8
        require(pos <= end and not any(data[zero:pos]), 'index padding invalid')
        for word in (0, 1, 2, 3, 4, 5, 7, 8, 9):
            out[start + word * 4:start + word * 4 + 4] = b'\0' * 4
    seen = False
    while pos < end:
        require(pos + 8 <= end, 'index extension truncated')
        sig, length = struct.unpack_from('>4sI', data, pos); pos += 8
        require(sig == b'TREE' and not seen and 0 < length <= end - pos, 'unsupported index extension')
        seen = True
        payload = data[pos:pos + length]
        tree_nodes = 0
        def tree(offset: int, depth: int) -> int:
            nonlocal tree_nodes
            tree_nodes += 1
            require(depth <= 128 and tree_nodes <= 10000, 'index TREE bounds')
            nul = payload.find(b'\0', offset)
            require(offset <= nul <= offset + 4096, 'index TREE path')
            name = payload[offset:nul]
            require((depth == 0 and name == b'') or
                    (depth > 0 and name not in {b'', b'.', b'..'} and b'/' not in name), 'index TREE path')
            nl = payload.find(b'\n', nul + 1)
            require(nul < nl <= nul + 32 and re.fullmatch(rb'(?:-1|0|[1-9][0-9]*) (?:0|[1-9][0-9]*)', payload[nul+1:nl]) is not None, 'index TREE fields')
            entries, children = map(int, payload[nul+1:nl].split(b' '))
            require(-1 <= entries <= count and children <= count, 'index TREE range')
            offset = nl + 1 + (20 if entries >= 0 else 0)
            require(offset <= len(payload), 'index TREE hash')
            for _ in range(children): offset = tree(offset, depth + 1)
            return offset
        require(tree(0, 0) == len(payload), 'index TREE trailing bytes')
        pos += length
    require(pos == end, 'index trailing bytes')
    return bytes(out)


def stable_index(root: Path) -> bytes:
    path = closure.git_index_path(root)
    lock = path.with_name(path.name + '.lock')
    require(not os.path.lexists(lock), 'index lock present')
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    try:
        before = os.fstat(fd)
        require(stat.S_ISREG(before.st_mode) and before.st_size <= 1024 * 1024, 'index object invalid')
        data = b''
        while len(data) <= before.st_size:
            chunk = os.read(fd, min(65536, before.st_size + 1 - len(data)))
            if not chunk: break
            data += chunk
        after = os.fstat(fd); current = path.lstat()
        binding = lambda s: (s.st_dev, s.st_ino, s.st_size, s.st_mtime_ns, s.st_ctime_ns, s.st_mode)
        require(binding(before) == binding(after) == binding(current)
                and len(data) == before.st_size and not os.path.lexists(lock), 'index read race')
        return data
    finally:
        os.close(fd)


# Exact same-admission cancellation maintenance; not a new admission or waiver.
CANCELLATION_STORE = Path('.codex/state/memory-cancellation-amendment')
CANCELLATION_POINTER = CANCELLATION_STORE / 'sealed.json'
CANCELLATION_ADMISSION = 'b9214051f001d43d447922211ae46da747ff73942e9013a8bb551ffe0cb862d5'
CANCELLATION_HEAD = 'ac04c6e53bc3349496eba44be93e933f05e1519c'
CANCELLATION_ROLE = 'src-tauri/src/agent_chat_tauri.rs'
CANCELLATION_BEFORE = '8883ebf8b4ee72d98c57ebea42090e1ece5341a64bea5534153aa6c8a92eb86b'
CANCELLATION_AFTER = 'd35474cd010c64215829858c7f3eb9d00b92595a6c21e8b4e1ca016658e38e56'
CANCELLATION_PROPOSAL = '8c49c91320f189f9961e7818fb511e63ee61139129496cc3b3f3be8907784c85'
CANCELLATION_EVIDENCE = {
    'acceptance': '28151b29f5d12841a7525b2dec535ee5811ba351a58400d4754f51dabf6efd3b',
    'artifact': '68a1f0353359d39e9052ac9c1af04bc5cb30d13ebc0f0d32cff0232b9b0867bd',
    'verification': '45fcd33cdcffb991060bd21bd3e87cbf9f7e9e8c7a4f6fc969aaed059240d60a',
    'source': 'e690074545c2ec9e2b356bfafb0d6b6eca8a94f9b1861fe64ab1fc9a59a3f1bb',
}
CANCELLATION_CHECKS = {
    'focused': ['-m', 'unittest', 'discover', '-s', '.codex/hooks/tests',
                '-p', 'test_lifecycle_dependent_admission.py'],
    'hooks': ['-m', 'unittest', 'discover', '-s', '.codex/hooks/tests', '-p', 'test_*.py'],
}


def cancellation_bytes(root: Path, name: str) -> bytes:
    """Bound only public amendment/source inputs; never consult private data."""
    path = closure.relative(root, name)
    require(not any(p.is_symlink() for p in [path, *path.parents]), 'cancellation symlink')
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    try:
        before = os.fstat(fd)
        require(stat.S_ISREG(before.st_mode) and before.st_nlink == 1
                and before.st_uid == os.getuid() and 0 < before.st_size <= closure.MAX_BYTES,
                'cancellation unsafe input')
        def identity(s):
            return (s.st_dev, s.st_ino, s.st_mode, s.st_uid, s.st_gid, s.st_nlink,
                    s.st_size, s.st_mtime_ns, s.st_ctime_ns)
        data = bytearray()
        while len(data) <= before.st_size:
            part = os.read(fd, min(65536, before.st_size + 1 - len(data)))
            if not part:
                break
            data.extend(part)
        require(len(data) == before.st_size and identity(before) == identity(os.fstat(fd))
                == identity(path.lstat()), 'cancellation read race')
        return bytes(data)
    finally:
        os.close(fd)


def cancellation_reference(root: Path, ref: Any) -> bytes:
    keys(ref, {'path', 'sha256'})
    closure.sha(ref['sha256'])
    require(ref['path'] == (CANCELLATION_STORE / (ref['sha256'] + '.blob')).as_posix(),
            'cancellation evidence namespace')
    data = cancellation_bytes(root, ref['path'])
    require(digest(data) == ref['sha256'], 'cancellation evidence drift')
    return data


def cancellation_amendment(root: Path, req: dict[str, Any], admission: str,
                           receipt: dict[str, Any] | None = None) -> dict[str, Any] | None:
    if receipt is None:
        if not os.path.lexists(root / CANCELLATION_POINTER):
            require(not os.path.lexists(root / CANCELLATION_STORE), 'cancellation partial publication')
            return None
        pointer = parse(cancellation_bytes(root, CANCELLATION_POINTER.as_posix()))
        keys(pointer, {'receipt'})
        receipt = pointer['receipt']
    value = parse(cancellation_reference(root, receipt))
    keys(value, {'schema_version', 'admission', 'root', 'branch', 'head', 'role',
                 'before_sha256', 'after_sha256', 'authorization', 'evidence', 'maintenance'})
    require(type(value['schema_version']) is int and value['schema_version'] == 1,
            'cancellation schema')
    require(req['increment_id'] == MEMORY_SUCCESSOR and str(root.resolve()) == MEMORY_ROOT
            and value['root'] == req['root'] == MEMORY_ROOT
            and value['branch'] == req['branch'] == branch(root) == MEMORY_BRANCH
            and value['head'] == req['head'] == gate.current_head_commit(root) == CANCELLATION_HEAD
            and value['admission'] == admission == CANCELLATION_ADMISSION,
            'cancellation identity')
    require(value['role'] == CANCELLATION_ROLE and value['before_sha256'] == CANCELLATION_BEFORE
            and value['after_sha256'] == CANCELLATION_AFTER
            and req['input_bindings'][CANCELLATION_ROLE]['sha256'] == CANCELLATION_BEFORE
            and CANCELLATION_ROLE not in req['allowed_paths'], 'cancellation source contract')
    auth = parse(cancellation_reference(root, value['authorization']))
    keys(auth, {'approved', 'scope', 'admission', 'proposal_sha256', 'historical_continuity_claimed'})
    require(auth['approved'] is True and auth['historical_continuity_claimed'] is False
            and auth == {'approved': True, 'scope': 'memory-cancellation-finalization',
                     'admission': admission, 'proposal_sha256': CANCELLATION_PROPOSAL,
                     'historical_continuity_claimed': False}, 'cancellation authority')
    keys(value['evidence'], set(CANCELLATION_EVIDENCE))
    for role, expected in CANCELLATION_EVIDENCE.items():
        require(digest(cancellation_reference(root, value['evidence'][role])) == expected,
                'cancellation acceptance evidence')
    maintenance = value['maintenance']
    keys(maintenance, {'before', 'after', 'checks', 'review'})
    keys(maintenance['before'], INDEX_FILES)
    keys(maintenance['after'], INDEX_FILES)
    keys(maintenance['checks'], set(CANCELLATION_CHECKS))
    for name in INDEX_FILES:
        cancellation_reference(root, maintenance['before'][name])
        current = cancellation_bytes(root, name)
        require(current == cancellation_reference(root, maintenance['after'][name]),
                'cancellation maintenance drift')
        require(stat.S_IMODE((root / name).stat().st_mode) == req['input_bindings'][name]['mode'],
                'cancellation maintenance mode')
    bindings = {name: ref['sha256'] for name, ref in maintenance['after'].items()}
    for label, args in CANCELLATION_CHECKS.items():
        check = parse(cancellation_reference(root, maintenance['checks'][label]))
        keys(check, {'schema_version', 'command', 'cwd', 'exit_status', 'timeout_seconds',
                     'elapsed_seconds', 'timed_out', 'complete_capture', 'files'})
        require(type(check['schema_version']) is int and check['schema_version'] == 1
                and check['command'] == ['/usr/bin/python3', '-B'] + args
                and check['cwd'] == MEMORY_ROOT and type(check['exit_status']) is int
                and check['exit_status'] == 0 and check['timed_out'] is False
                and check['complete_capture'] is True and check['files'] == bindings
                and type(check['timeout_seconds']) is int and 0 < check['timeout_seconds'] <= 600
                and type(check['elapsed_seconds']) in (int, float)
                and 0 <= check['elapsed_seconds'] <= check['timeout_seconds'],
                'cancellation validation incomplete')
    review = parse(cancellation_reference(root, maintenance['review']))
    keys(review, {'schema_version', 'scope', 'result', 'files', 'checks', 'historical_failure_preserved',
                  'original_admission_preserved', 'unrelated_routes_unchanged', 'product_execution_reused'})
    require(type(review['schema_version']) is int and review['schema_version'] == 1
            and review['scope'] == 'memory-cancellation-finalization' and review['result'] == 'PASS'
            and review['files'] == bindings and review['checks'] == maintenance['checks']
            and all(review[k] is True for k in ('historical_failure_preserved', 'original_admission_preserved',
                                              'unrelated_routes_unchanged', 'product_execution_reused')),
            'cancellation maintenance review')
    current = cancellation_bytes(root, CANCELLATION_ROLE)
    require(digest(current) == CANCELLATION_AFTER, 'cancellation current source drift')
    product = {'exists': True, 'mode': stat.S_IMODE((root / CANCELLATION_ROLE).stat().st_mode),
               'sha256': digest(current)}
    require(product['mode'] == req['input_bindings'][CANCELLATION_ROLE]['mode'], 'cancellation source mode')
    return {'product': product, 'maintenance': maintenance}


@closure.guarded
def publish_cancellation_amendment(root: Path, receipt: dict[str, Any]) -> None:
    """Publish the complete, verified additive proof once; never rewrite live state."""
    require(not os.path.lexists(root / CANCELLATION_POINTER), 'cancellation replay')
    state = gate.read_state(root)
    require(state is not None and state['status'] == 'active', 'cancellation requires active state')
    shape(state)
    admission = state['dependent_lineage']['admission']
    raw = read(root, (STORE / (admission + '.json')).as_posix())
    require(digest(raw) == admission, 'cancellation original admission drift')
    req = parse(raw)['request']
    amendment = cancellation_amendment(root, req, admission, receipt)
    require(index_amendment(root, req, admission, forward=amendment) is not None,
            'cancellation prior maintenance missing')
    exclusive(root, CANCELLATION_POINTER, encode({'receipt': receipt}))


def index_amendment(root: Path, req: dict[str, Any], admission: str, *,
                    forward: dict[str, Any] | None = None) -> dict[str, Any] | None:
    if not os.path.lexists(root / INDEX_AMENDMENT): return None
    pointer = parse(read(root, INDEX_AMENDMENT.as_posix()))
    keys(pointer, {'receipt'})
    value = parse(reference(root, pointer['receipt']))
    keys(value, {'schema_version', 'admission', 'root', 'branch', 'head',
                 'original_index_sha256', 'predecessor_state_sha256', 'candidate_sha256',
                 'authorization', 'before', 'maintenance'})
    require(type(value['schema_version']) is int and value['schema_version'] == 1, 'index amendment schema')
    require(req['increment_id'] == MEMORY_SUCCESSOR and str(root.resolve()) == MEMORY_ROOT
            and value['admission'] == admission and value['root'] == req['root']
            and value['branch'] == req['branch'] and value['head'] == req['head'], 'index amendment identity')
    require(value['original_index_sha256'] == req['index_sha256']
            and value['predecessor_state_sha256'] == MEMORY_STATE
            and value['candidate_sha256'] == MEMORY_CANDIDATE, 'index amendment original bindings')
    auth = parse(reference(root, value['authorization']))
    keys(auth, {'approved', 'scope', 'admission', 'original_index_sha256'})
    require(auth['approved'] is True and auth['scope'] == INDEX_MAINTENANCE
            and auth['admission'] == admission and auth['original_index_sha256'] == req['index_sha256'], 'index amendment authority')
    original = parse(reference(root, req['evidence']['maintenance']))
    keys(value['before'], INDEX_FILES)
    for name, ref in value['before'].items():
        require(reference(root, ref) == reference(root, original['files'][name])
                and digest(reference(root, ref)) == req['input_bindings'][name]['sha256'], 'index amendment before drift')
    maintenance = value['maintenance']
    keys(maintenance, {'state', 'report', 'manifest', 'files', 'stop'})
    state = parse(reference(root, maintenance['state'])); gate.validate_state(state)
    require(state['status'] == 'complete' and state['increment_id'] == INDEX_MAINTENANCE
            and state['quality_gate'] in {'PASS', 'PASS WITH ADVISORIES'}, 'index maintenance incomplete')
    stop = parse(reference(root, maintenance['stop']))
    keys(stop, {'exit_status', 'decision', 'state_sha256'})
    require(type(stop['exit_status']) is int and stop['exit_status'] == 0 and stop['decision'] == 'allow'
            and stop['state_sha256'] == maintenance['state']['sha256'], 'index maintenance Stop invalid')
    raw_report = reference(root, maintenance['report'])
    require(digest(raw_report) == state['report_sha256'], 'index maintenance report drift')
    report = gate._extract_manifest(raw_report.decode())
    require(report['increment_id'] == INDEX_MAINTENANCE and report['quality_gate'] == state['quality_gate'], 'index maintenance report identity')
    for group in ('verification', 'manual_verification'):
        require(bool(report[group]) and all(not row['required'] or row['status'] == 'Passed' for row in report[group]), 'index maintenance checks incomplete')
    require(not any(row['blocks_completion'] for row in report['findings']), 'index maintenance blocking finding')
    criteria = report['milestone']['criteria']
    require(set(criteria) == INDEX_CRITERIA and all(x['status'] == 'automatically_verified' for x in criteria.values()), 'index maintenance criteria incomplete')
    inventory = closure.normalize_manifest(parse(reference(root, maintenance['manifest'])))
    require(closure.fingerprint(inventory) == state['workspace_fingerprint'], 'index maintenance fingerprint drift')
    keys(maintenance['files'], INDEX_FILES)
    overlay = {}
    for name, ref in maintenance['files'].items():
        data = reference(root, ref)
        require(digest(data) == inventory[name]['sha256'], 'index maintenance historical implementation drift')
        if forward is not None:
            proof = forward['maintenance']
            require(cancellation_reference(root, proof['before'][name]) == data,
                    'cancellation previous maintenance drift')
            data = cancellation_reference(root, proof['after'][name])
        require(read(root, name) == data, 'index maintenance implementation drift')
        current = {'exists': True, 'mode': stat.S_IMODE((root / name).stat().st_mode), 'sha256': digest(data)}
        require(current['mode'] == inventory[name]['mode'] == req['input_bindings'][name]['mode'],
                'index maintenance mode drift')
        overlay[name] = current
    return overlay



def memory_evidence(root: Path, request: dict[str, Any], *, before_admission: bool, overlay: dict[str, Any] | None = None) -> None:
    """No acceptance replacement: this route binds one fully retained candidate."""
    keys(request["evidence"], {"candidate", "snapshot", "maintenance"})
    require(request["predecessor"]["state"]["sha256"] == MEMORY_STATE
            and request["predecessor"]["report"]["sha256"] == MEMORY_REPORT,
            "memory frozen predecessor mismatch")
    require(request["evidence"]["candidate"]["sha256"] == MEMORY_CANDIDATE,
            "memory frozen candidate mismatch")
    candidate = parse(reference(root, request["evidence"]["candidate"]))
    require(candidate["branch"] == MEMORY_BRANCH, "memory candidate branch mismatch")
    files = candidate["candidate"]
    require(isinstance(files, dict) and len(files) == 32, "memory candidate inventory mismatch")
    snapshot = parse(reference(root, request["evidence"]["snapshot"]))
    keys(snapshot, {"files", "state", "index"})
    keys(snapshot["files"], set(files))
    require(reference(root, snapshot["state"]) == reference(root, request["predecessor"]["state"]),
            "memory predecessor snapshot mismatch")
    require(digest(reference(root, snapshot["index"])) == request["index_sha256"],
            "memory index snapshot mismatch")
    for name, expected in files.items():
        gate.validate_relative_path(name)
        keys(expected, {"sha256", "mode"})
        keys(snapshot["files"][name], {"copy", "mode"})
        copy = snapshot["files"][name]
        original = reference(root, copy["copy"])
        require(digest(original) == expected["sha256"] and copy["mode"] == expected["mode"],
                "memory original snapshot drift")
        if not before_admission and name in MEMORY_DOCS:
            require(read(root, name).startswith(original), "memory historical document rewrite")
        if before_admission:
            current = read(root, name)
            require(oct((root / name).stat().st_mode & 0o7777) == expected["mode"],
                    "memory original mode drift")
            # Only additive preparation in current documents; no pre-admission code changes.
            require(current.startswith(original) if name in MEMORY_DOCS else current == original,
                    "memory candidate drift before admission")
    maintenance = parse(reference(root, request["evidence"]["maintenance"]))
    keys(maintenance, {"state", "report", "manifest", "files", "stop"})
    state = parse(reference(root, maintenance["state"]))
    gate.validate_state(state)
    require(state["increment_id"] == MAINTENANCE and state["status"] == "complete"
            and state["quality_gate"] in {"PASS", "PASS WITH ADVISORIES"},
            "memory unfinished maintenance cannot authorize admission")
    stop = parse(reference(root, maintenance["stop"]))
    keys(stop, {"exit_status", "decision", "state_sha256"})
    require(type(stop["exit_status"]) is int and stop["exit_status"] == 0
            and stop["decision"] == "allow"
            and stop["state_sha256"] == maintenance["state"]["sha256"],
            "memory maintenance full Stop unverified")
    raw_report = reference(root, maintenance["report"])
    require(digest(raw_report) == state["report_sha256"], "memory maintenance report drift")
    inventory = closure.normalize_manifest(parse(reference(root, maintenance["manifest"])))
    require(closure.fingerprint(inventory) == state["workspace_fingerprint"],
            "memory maintenance candidate drift")
    report = gate._extract_manifest(raw_report.decode())
    require(report["increment_id"] == MAINTENANCE
            and report["quality_gate"] == state["quality_gate"], "memory maintenance identity mismatch")
    for group in ("verification", "manual_verification"):
        require(bool(report[group]) and all(not row["required"] or row["status"] == "Passed"
                    for row in report[group]), "memory maintenance checks incomplete")
    require(all(not row["blocks_completion"] for row in report["findings"]),
            "memory maintenance blocking finding")
    criteria = report["milestone"]["criteria"]
    require(set(criteria) == {"binding", "obligations", "boundaries", "verification"}
            and all(row["status"] == "automatically_verified" for row in criteria.values()),
            "memory maintenance criteria incomplete")
    keys(maintenance["files"], MAINTENANCE_FILES)
    for name, ref in maintenance["files"].items():
        data = reference(root, ref)
        require(inventory[name]["sha256"] == digest(data) and (read(root, name) == data or (overlay is not None and name in overlay)),
                "memory verified validator drift")


@closure.guarded
def begin(root: Path, increment: str, path: str) -> None:
    require(increment in {SUCCESSOR, MEMORY_SUCCESSOR}, "dependent successor is task-bound")
    memory = increment == MEMORY_SUCCESSOR
    predecessor = MEMORY_PREDECESSOR if memory else PREDECESSOR
    require(not (root / POINTER).exists(), "dependent replay rejected")
    store = root / STORE
    require(not store.is_symlink(), "dependent unsafe store")
    require(not store.exists() or not any(store.iterdir()),
            "dependent replay rejected: interrupted admission evidence exists")
    state = gate.read_state(root)
    require(state is not None and state["status"] == "failed"
            and state["increment_id"] == predecessor and state["quality_gate"] == "FAIL",
            "dependent original terminal FAIL required")
    closure.reject_d133_context(root, state)
    require("successor_disposition" not in state and "dependent_lineage" not in state,
            "dependent historical route mismatch")
    closure.validate_lineage(root, state, scope=False)
    raw_request = read(root, path)
    request = parse(raw_request)
    keys(request, {"schema_version", "increment_id", "root", "branch", "head",
        "workspace_fingerprint", "index_sha256", "predecessor", "evidence", "input_bindings", "milestone",
        "allowed_paths", "plan", "report", "readiness_report", "readiness_increment",
        "obligations", "acceptance_change", "owner_authorization"})
    require(type(request["schema_version"]) is int and request["schema_version"] == 1
            and request["increment_id"] == increment, "dependent request identity mismatch")
    identity(root, request)
    require(request["index_sha256"] == digest(closure.read_bytes(closure.git_index_path(root))), "dependent index binding mismatch")
    require(gate._is_ancestor(root, state["head_commit"], request["head"]), "dependent historical HEAD lineage mismatch")
    require(request["workspace_fingerprint"] == gate.workspace_fingerprint(root), "dependent input drift")
    keys(request["owner_authorization"], {"admission", "execution"})
    for authorization in request["owner_authorization"].values(): closure.authorization(root, authorization)
    if not memory:
        closure.authorization(root, request["acceptance_change"]["owner_authorization"])
    closure.milestone_contract(request["milestone"])
    require(request["milestone"]["authorized_destructive_paths"] == [], "dependent destruction forbidden")
    keys(request["predecessor"], {"state", "report"})
    raw_state = reference(root, request["predecessor"]["state"])
    require(raw_state == closure.read_bytes(gate.state_path(root)) and parse(raw_state) == state,
            "dependent predecessor binding mismatch")
    raw_report = reference(root, request["predecessor"]["report"])
    require(request["predecessor"]["report"]["path"] == state["report_path"]
            and digest(raw_report) == state["report_sha256"], "dependent original report mismatch")
    original = gate._extract_manifest(raw_report.decode())
    require(original["increment_id"] == predecessor and original["quality_gate"] == "FAIL",
            "dependent original report must remain FAIL")
    require("closure_lineage" in state, "dependent predecessor needs schema-2 admission")
    prior_admission = closure.read_json(root / closure.DIRECTORY / ("admission-" + state["closure_lineage"]["admission"] + ".json"))
    require(prior_admission["request"]["schema_version"] == 2, "dependent predecessor needs schema-2 admission")
    prior_acceptance = prior_admission["request"]["milestone"]["acceptance"]
    require(set(request["milestone"]["acceptance"]) == set(prior_acceptance), "dependent acceptance criterion omitted or expanded")
    for name, description in prior_acceptance.items():
        if memory or name != "preservation":
            require(request["milestone"]["acceptance"][name] == description, "dependent acceptance text drift")
    if not memory:
        require(request["milestone"]["acceptance"]["preservation"] == "surviving-evidence-and-explicit-unavailable-custody", "dependent replacement checklist mismatch")
    obligations(original, request)
    if memory:
        memory_evidence(root, request, before_admission=True)
    else:
        keys(request["evidence"], EVIDENCE_KEYS)
        evidence = {label: reference(root, value) for label, value in request["evidence"].items()}
        historical = closure.normalize_manifest(parse(evidence["historical_manifest"]))
        require(closure.fingerprint(historical) == state["workspace_fingerprint"], "dependent historical inventory mismatch")
        comparison = parse(evidence["comparison"])
        require(comparison.get("pass") is False and comparison.get("exit_code") == 1,
                "dependent failed comparison missing")
        custody(evidence["unavailable_inventory"])
        survival = parse(evidence["surviving_preservation"])
        require(survival.get("result") == "PASS" and survival.get("scope") == "surviving-evidence",
                "dependent surviving preservation unverified")
        provenance = parse(evidence["archival_provenance"])
        require(provenance.get("owner_intent") == "intentional-archival"
                and provenance.get("complete_custody") is False, "dependent archival provenance unqualified")
        snapshot_copies(root, request)
    if memory:
        require(request["readiness_increment"] == MEMORY_SUCCESSOR + "-readiness",
                "memory readiness identity mismatch")
        require(re.fullmatch(r"docs/plans/" + gate.REPORT_DATE_PATTERN + "-"
                             + re.escape(MEMORY_SUCCESSOR) + r"\.md", request["plan"]) is not None,
                "memory continuation plan identity mismatch")
    prospective_report(root, request["report"], increment)
    gate.validate_relative_path(request["plan"])
    require(request["plan"].startswith("docs/plans/") and request["plan"].endswith(".md"), "dependent plan path")
    allowed = closure.path_list(request["allowed_paths"])
    allowed_base = MEMORY_DOCS | MEMORY_SOURCE if memory else ROOT_DOCS
    require(set(allowed) == allowed_base | {request["plan"], request["report"]}, "dependent documentation scope mismatch")
    protected = {state["report_path"], request["readiness_report"]}
    require(not protected.intersection(allowed) and protected.issubset(request["milestone"]["protected_paths"]),
            "dependent historical evidence not protected")
    baseline = closure.manifest(root)
    expected_inputs = {n: v for n, v in baseline.items() if n not in allowed}
    require(request["input_bindings"] == expected_inputs, "dependent implementation input bindings mismatch")
    ready, _, _ = gate.validate_report(root, request["readiness_report"], request["readiness_increment"])
    closure.reviews(ready, full=False)
    require(ready["next_increment_readiness"] in {"Ready", "Ready with advisories"}, "dependent readiness blocked")
    record = {"request": request, "raw_request": raw_request.decode(), "raw_state": raw_state.decode(),
              "raw_report": raw_report.decode(), "raw_decision": read(root, "DECISIONS.md").decode(),
              "baseline_manifest": baseline}
    data = encode(record)
    admission = digest(data)
    exclusive(root, STORE / (admission + ".json"), data)
    require(gate.workspace_fingerprint(root) == request["workspace_fingerprint"]
            and closure.read_bytes(gate.state_path(root)) == raw_state, "dependent admission race")
    exclusive(root, POINTER, encode({"admission": admission}))
    gate.write_state(root, {"schema_version": STATE_SCHEMA, "status": "active", "increment_id": increment,
        "baseline_fingerprint": request["workspace_fingerprint"], "dependent_lineage": {"admission": admission}})


@closure.guarded
def record(root: Path, state: dict[str, Any], *, scope: bool = True) -> dict[str, Any]:
    shape(state)
    admission = state["dependent_lineage"]["admission"]
    pointer = parse(read(root, POINTER.as_posix()))
    keys(pointer, {"admission"})
    require(pointer["admission"] == admission, "dependent consumed identity mismatch")
    data = read(root, (STORE / (admission + ".json")).as_posix())
    require(digest(data) == admission, "dependent sealed admission tampered")
    result = parse(data)
    keys(result, {"request", "raw_request", "raw_state", "raw_report", "raw_decision", "baseline_manifest"})
    req = result["request"]
    require(state["baseline_fingerprint"] == req["workspace_fingerprint"], "dependent baseline binding mismatch")
    require(parse(result["raw_request"].encode()) == req, "dependent raw request mismatch")
    memory = state["increment_id"] == MEMORY_SUCCESSOR
    amendment = cancellation_amendment(root, req, admission) if memory else None
    overlay = index_amendment(root, req, admission, forward=amendment) if memory else None
    require(amendment is None or overlay is not None, "cancellation prior maintenance missing")
    index = stable_index(root) if memory else closure.read_bytes(closure.git_index_path(root))
    if req["index_sha256"] != digest(index):
        require(memory and overlay is not None, "dependent index drift")
        snapshot = parse(reference(root, req["evidence"]["snapshot"]))
        original = reference(root, snapshot["index"])
        require(digest(original) == req["index_sha256"], "memory original index drift")
        require(index_semantics(original) == index_semantics(index), "dependent substantive index drift")
    require(req["increment_id"] == state["increment_id"], "dependent recorded task mismatch")
    predecessor = MEMORY_PREDECESSOR if memory else PREDECESSOR
    old = parse(result["raw_state"].encode())
    gate.validate_state(old)
    require(old["status"] == "failed" and old["quality_gate"] == "FAIL" and old["increment_id"] == predecessor,
            "dependent failure history mismatch")
    require(digest(result["raw_state"].encode()) == req["predecessor"]["state"]["sha256"], "dependent history hash mismatch")
    closure.validate_lineage(root, old, scope=False)
    require(digest(result["raw_report"].encode()) == old["report_sha256"], "dependent failed report binding mismatch")
    require(reference(root, req["predecessor"]["report"]) == result["raw_report"].encode(), "dependent failed report drift")
    require(req["root"] == str(root.resolve()) and req["branch"] == branch(root), "dependent checkout drift")
    require(gate._is_ancestor(root, req["head"], gate.current_head_commit(root)), "dependent HEAD lineage drift")
    if state["status"] == "active": require(req["head"] == gate.current_head_commit(root), "dependent active HEAD drift")
    authorizations = list(req["owner_authorization"].values())
    if not memory: authorizations.append(req["acceptance_change"]["owner_authorization"])
    for auth in authorizations:
        require(auth["approved"] is True and auth["decision_sha256"] == digest(result["raw_decision"].encode()),
                "dependent authority history mismatch")
    for ref in req["evidence"].values(): reference(root, ref)
    if memory:
        memory_evidence(root, req, before_admission=False, overlay=overlay)
    else:
        snapshot_copies(root, req)
    after = closure.manifest(root)
    qualified = dict(overlay or {})
    if amendment is not None: qualified[CANCELLATION_ROLE] = amendment["product"]
    for name, value in req["input_bindings"].items():
        require(after.get(name) == qualified.get(name, value), "dependent protected implementation drift")
    # Project only separately verified maintenance files back to their original
    # baseline for product-scope accounting; never change the sealed record.
    scoped_after = dict(after)
    for name in overlay or {}: scoped_after[name] = result["baseline_manifest"][name]
    if scope:
        delta = closure.validate_milestone_delta({"request": req, "baseline_manifest": result["baseline_manifest"]}, scoped_after)
        allowed = set(req["allowed_paths"])
        if amendment is not None: allowed.add(CANCELLATION_ROLE)
        require(delta.issubset(allowed), "dependent scope drift")
    if memory: require(stable_index(root) == index, "index changed during validation")
    if overlay is not None: result["verified_maintenance_overlay"] = overlay
    return result


def carry(source: dict[str, Any], target: dict[str, Any]) -> None:
    if "dependent_lineage" in source:
        target["schema_version"] = STATE_SCHEMA
        target["dependent_lineage"] = source["dependent_lineage"]


@closure.guarded
def milestone_report(root: Path, increment: str, report: dict[str, Any]) -> bool:
    state = gate.read_state(root)
    require(state is not None and increment == state["increment_id"]
            and increment in {SUCCESSOR, MEMORY_SUCCESSOR}, "dependent report identity mismatch")
    rec = record(root, state)
    req = rec["request"]
    original = gate._extract_manifest(rec["raw_report"])
    for kind, key in [("verification", "command"), ("manual_verification", "check")]:
        for prior in original[kind]:
            if not prior["required"]: continue
            name = prior[key]
            if increment == SUCCESSOR and kind == "verification" and name.endswith("cortexa-dual-label-final-preservation.py"):
                name = PRESERVATION_COMMAND
            rows = [row for row in report[kind] if row[key] == name]
            require(len(rows) == 1 and rows[0]["required"] is True, "dependent required check omitted")
    evidence = report.get("milestone")
    keys(evidence, {"criteria", "paths"})
    keys(evidence["criteria"], set(req["milestone"]["acceptance"]))
    blocked = False
    for value in evidence["criteria"].values():
        keys(value, {"status", "evidence"})
        require(value["status"] in {"automatically_verified", "live_verified", "implemented", "deferred", "blocked"}
                and isinstance(value["evidence"], str) and bool(value["evidence"].strip()), "dependent criterion evidence missing")
        blocked |= value["status"] not in {"automatically_verified", "live_verified"}
    scoped = closure.manifest(root)
    for name in rec.get("verified_maintenance_overlay", {}): scoped[name] = rec["baseline_manifest"][name]
    delta = closure.changed(rec["baseline_manifest"], scoped)
    keys(evidence["paths"], delta)
    for value in evidence["paths"].values():
        keys(value, {"criterion", "rationale", "within_objective", "preserves_existing"})
        require(value["criterion"] in evidence["criteria"] and value["within_objective"] is True
                and value["preserves_existing"] is True and isinstance(value["rationale"], str)
                and bool(value["rationale"].strip()), "dependent path attribution missing")
    return blocked
