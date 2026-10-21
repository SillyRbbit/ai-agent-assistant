"""Snapshot-bound publication of the completed memory milestone.

Historical acceptance is immutable. Current publication is separately checked;
these same-user workflow receipts are integrity evidence, not authentication.
No product execution, network operation or historical validator replay occurs.
"""
from __future__ import annotations

import hashlib
import json
import lzma
import os
from pathlib import Path
import re
import stat
import subprocess
from typing import Any

import lifecycle_closure as closure
import lifecycle_dependent_admission as dependent
import post_increment_gate as gate

ROOT = Path(dependent.MEMORY_ROOT)
BRANCH = dependent.MEMORY_BRANCH
HEAD = dependent.CANCELLATION_HEAD
EXTERNAL = Path('/Users/hdang/.codex/visualizations/2026/09/21/01a0c184-189a-7b01-ae8f-07cfb1b83d96/cortexa-memory-rejection-sites-20261017-01/c7-consolidated-20261021-01/git-publication-02')
ACCEPTED_SHA = '62f53f5edcd49a3c96ba28f31040832a1b52fa8d25e9d385214dd69a27ed8c10'
STATE_SHA = '9a32165e58f41c68e59b3a242a2dccbb5aaec753144c6382f4512cee8c701e23'
DIRECTORY = Path('.codex/state/memory-publication')
CODE = {'.codex/hooks/post_increment_gate.py',
        '.codex/hooks/lifecycle_memory_publication.py',
        '.codex/hooks/tests/test_lifecycle_memory_publication.py'}
DOCS = dependent.MEMORY_DOCS | {'docs/plans/2026-10-21-persistent-memory-publication.md'}
CHECKS = {'focused', 'hooks', 'docs', 'repository', 'security', 'whitespace', 'session'}
NODE = '/opt/homebrew/Cellar/node/26.3.0/bin/node'
NPM = '/opt/homebrew/lib/node_modules/npm/bin/npm-cli.js'
CHECK_ARGV = {
    'focused': ['/usr/bin/python3', '-B', '-m', 'unittest', 'discover', '-s',
                '.codex/hooks/tests', '-p', 'test_lifecycle_memory_publication.py', '-v'],
    'hooks': ['/usr/bin/python3', '-B', '-m', 'unittest', 'discover', '-s',
              '.codex/hooks/tests', '-p', 'test_*.py', '-v'],
    **{k: [NODE, NPM, 'run', v] for k, v in
       [('docs', 'docs:check'), ('repository', 'repository:check'), ('security', 'security:scan')]},
    'whitespace': ['git', 'diff', '--check', 'HEAD'],
    'session': ['/usr/bin/python3', '-B', '.codex/hooks/session_end_gate.py'],
}
MAX_STEPS = 4
MAX_BYTES = 4 * 1024 * 1024
require = closure.require


def read(path: Path) -> bytes:
    require(path.is_absolute() and not any(p.is_symlink() for p in (path, *path.parents)),
            'publication unsafe path')
    before = path.lstat()
    require(stat.S_ISREG(before.st_mode) and before.st_nlink == 1
            and before.st_uid == os.getuid() and not before.st_mode & 0o022
            and before.st_size <= MAX_BYTES,
            'publication unsafe file')
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    def identity(s):
        return (s.st_dev, s.st_ino, s.st_mode, s.st_uid, s.st_gid, s.st_nlink,
                s.st_size, s.st_mtime_ns, s.st_ctime_ns)
    try:
        require(identity(before) == identity(os.fstat(fd)), 'publication replacement')
        data = bytearray()
        while len(data) <= before.st_size:
            block = os.read(fd, min(65536, before.st_size + 1 - len(data)))
            if not block:
                break
            data.extend(block)
        require(len(data) == before.st_size and identity(before) == identity(os.fstat(fd))
                == identity(path.lstat()), 'publication read race')
        return bytes(data)
    finally:
        os.close(fd)


def unpack(raw: bytes) -> Any:
    decoder = lzma.LZMADecompressor()
    try:
        data = decoder.decompress(raw, max_length=MAX_BYTES + 1)
    except lzma.LZMAError as error:
        raise gate.GateError('publication malformed compression') from error
    require(len(data) <= MAX_BYTES and decoder.eof and not decoder.unused_data,
            'publication compressed bounds')
    return dependent.parse(data)


def reference(value: Any) -> dict[str, Any]:
    closure.keys(value, {'path', 'sha256'})
    path = Path(value['path'])
    require(path.parent == EXTERNAL and path.name not in {'', '.', '..'},
            'publication evidence namespace')
    data = read(path)
    require(closure.digest(data) == closure.sha(value['sha256']), 'publication evidence drift')
    return unpack(data) if path.suffix == '.xz' else dependent.parse(data)


def accepted(root: Path, state: dict[str, Any]) -> dict[str, Any]:
    require(root.resolve() == ROOT and dependent.branch(root) == BRANCH,
            'publication checkout identity')
    raw = read(EXTERNAL / 'ACCEPTED-SNAPSHOT.json.xz')
    require(closure.digest(raw) == ACCEPTED_SHA, 'publication acceptance snapshot drift')
    value = unpack(raw)
    require(value['root'] == str(ROOT) and value['branch'] == BRANCH and value['head'] == HEAD
            and value['state'] == state and state['status'] == 'complete'
            and state['increment_id'] == dependent.MEMORY_SUCCESSOR
            and value['state_sha256'] == STATE_SHA
            and closure.digest(read(gate.state_path(root))) == STATE_SHA,
            'publication historical state identity')
    require(closure.fingerprint(value['manifest']) == state['workspace_fingerprint']
            and value['observed_status']['valid'] is True
            and value['observed_status']['status'] == 'complete',
            'publication historical acceptance invalid')
    require(closure.digest(read(root / state['report_path'])) == state['report_sha256'],
            'publication accepted report drift')
    for name, digest in value['protected_records'].items():
        gate.validate_relative_path(name)
        require(name.startswith('.codex/state/') and closure.digest(read(root / name)) == digest,
                'publication protected historical record drift')
    return value


def snapshot(root: Path) -> tuple[dict[str, Any], dict[str, tuple[str, str]]]:
    names = gate._decode_git_paths(gate._run_git(root, 'ls-files', '--cached', '--others',
                                                '--exclude-standard', '-z'))
    manifest = {}
    blobs = {}
    for name in sorted(set(names)):
        path = closure.relative(root, name)
        require(path.exists(), 'publication missing workspace file')
        data = read(path)
        mode = stat.S_IMODE(path.stat().st_mode)
        manifest[name] = {'exists': True, 'mode': mode, 'sha256': closure.digest(data)}
        oid = hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest()
        blobs[name] = ('100755' if mode & 0o111 else '100644', oid)
    return manifest, blobs


def tree_entries(root: Path, revision: str) -> dict[str, tuple[str, str]]:
    result = {}
    for row in gate._run_git(root, 'ls-tree', '-r', '-z', revision).split(b'\0'):
        if not row:
            continue
        fields, name = row.split(b'\t', 1)
        mode, kind, oid = fields.decode().split()
        require(kind == 'blob' and mode in {'100644', '100755'}, 'publication tree object type')
        path = name.decode(); gate.validate_relative_path(path)
        require(path not in result, 'publication duplicate tree path')
        result[path] = (mode, oid)
    return result


def index_entries(root: Path, raw: bytes) -> dict[str, tuple[str, str]]:
    dependent.index_semantics(raw)  # Retain strict v2 format/extension rejection.
    result = {}
    for row in gate._run_git(root, 'ls-files', '--stage', '-z').split(b'\0'):
        if row:
            fields, name = row.split(b'\t', 1)
            mode, oid, stage = fields.decode().split()
            require(stage == '0' and mode in {'100644', '100755'}, 'publication index stage/type')
            path = name.decode(); gate.validate_relative_path(path)
            require(path not in result, 'publication duplicate staged path')
            result[path] = (mode, oid)
    for row in gate._run_git(root, 'ls-files', '-v', '-z').split(b'\0'):
        require(not row or row.startswith(b'H '), 'publication index visibility flags')
    return result


def commit_parents(root: Path, commit: str) -> list[str]:
    return gate._run_git(root, 'show', '-s', '--format=%P', commit).decode().strip().split()


def verify_tree_contents(root: Path, commit: str, manifest: dict[str, Any]) -> None:
    entries = tree_entries(root, commit)
    require(set(entries) == set(manifest), 'publication prior commit tree paths')
    names = list(entries)
    oids = [entries[n][1] for n in names]
    require(len(oids) <= 10000, 'publication object count')
    input_bytes = ('\n'.join(oids) + '\n').encode()
    def batch(option):
        try:
            result = subprocess.run(['git', 'cat-file', option], cwd=root,
                                    input=input_bytes, stdout=subprocess.PIPE,
                                    stderr=subprocess.PIPE, timeout=15, check=False)
        except subprocess.TimeoutExpired as error:
            raise gate.GateError('publication object capture timeout') from error
        require(result.returncode == 0 and not result.stderr, 'publication object capture failed')
        return result
    headers = batch('--batch-check').stdout.splitlines()
    require(len(headers) == len(oids), 'publication object census incomplete')
    total = 0
    for oid, header in zip(oids, headers):
        fields = header.split()
        require(len(fields) == 3 and fields[:2] == [oid.encode(), b'blob']
                and re.fullmatch(b'[0-9]{1,8}', fields[2]) is not None,
                'publication object census identity')
        size = int(fields[2]); total += size + len(header) + 2
        require(size <= MAX_BYTES and total <= 16 * 1024 * 1024, 'publication object census bounds')
    result = batch('--batch')
    require(result.returncode == 0 and not result.stderr
            and len(result.stdout) <= 16 * 1024 * 1024, 'publication bounded object capture')
    position = 0
    for name, oid in zip(names, oids):
        end = result.stdout.find(b'\n', position)
        require(end >= position, 'publication object header')
        fields = result.stdout[position:end].split()
        require(len(fields) == 3 and fields[:2] == [oid.encode(), b'blob']
                and re.fullmatch(b'[0-9]{1,8}', fields[2]) is not None,
                'publication object identity')
        size = int(fields[2]); position = end + 1
        data = result.stdout[position:position + size]; position += size
        require(size <= MAX_BYTES and len(data) == size
                and result.stdout[position:position + 1] == b'\n', 'publication object bounds')
        position += 1
        item = manifest[name]
        require(item['exists'] and closure.digest(data) == item['sha256']
                and (int(entries[name][0], 8) & 0o111) == (item['mode'] & 0o111),
                'publication prior commit content')
    require(position == len(result.stdout), 'publication object trailing data')


def records(root: Path) -> list[dict[str, Any]]:
    directory = root / DIRECTORY
    if not os.path.lexists(directory):
        return []
    require(not directory.is_symlink() and directory.is_dir(), 'publication unsafe store')
    files = sorted(directory.iterdir())
    require(0 < len(files) <= MAX_STEPS, 'publication record count')
    result = []
    for number, path in enumerate(files, 1):
        require(re.fullmatch(rf'{number:04d}-[0-9a-f]{{64}}\.xz', path.name) is not None,
                'publication incomplete or ambiguous records')
        raw = read(path)
        require(closure.digest(raw) == path.name[5:-3], 'publication record hash')
        value = unpack(raw)
        require(value['prior'] == (closure.digest(closure.encoded(result[-1])) if result else None),
                'publication record lineage')
        result.append(value)
    return result


def check_record(root: Path, value: dict[str, Any], base: dict[str, Any],
                 previous: dict[str, Any] | None) -> None:
    closure.keys(value, {'schema', 'owner_approved', 'accepted_sha256', 'prior', 'parents',
                         'upstream', 'manifest', 'checks', 'review'})
    require(type(value['schema']) is int and value['schema'] == 1
            and value['owner_approved'] is True and value['accepted_sha256'] == ACCEPTED_SHA,
            'publication authorization identity')
    before = previous['manifest'] if previous else base['manifest']
    allowed = CODE | DOCS
    parents = value['parents']
    require(isinstance(parents, list) and len(parents) in {1, 2}
            and all(isinstance(p, str) and re.fullmatch('[0-9a-f]{40}', p) for p in parents)
            and len(set(parents)) == len(parents), 'publication parent shape')
    if previous is None:
        require(parents == [HEAD] and value['upstream'] is None and value['prior'] is None,
                'publication original parent')
    else:
        require(value['prior'] == closure.digest(closure.encoded(previous))
                and commit_parents(root, parents[0]) == previous['parents'],
                'publication prior commit ancestry')
        verify_tree_contents(root, parents[0], previous['manifest'])
        require(len(parents) == 2 and value['upstream'] == parents[1]
                and gate._is_ancestor(root, HEAD, parents[1]), 'publication integration ancestry')
        upstream_names = gate._decode_git_paths(gate._run_git(root, 'diff', '--name-only', '-z',
                                                            HEAD, parents[1]))
        require(not any(n.startswith(('src/', 'src-tauri/src/', 'src-tauri/tests/'))
                        for n in upstream_names), 'publication upstream product-source change')
        allowed |= set(upstream_names)
    changed = closure.changed(before, value['manifest'])
    require(closure.normalize_manifest(value['manifest']) == value['manifest'],
            'publication manifest shape')
    require(changed.issubset(allowed), 'publication unrelated scope')
    require(all(v['exists'] for v in value['manifest'].values()), 'publication deletion unsupported')
    for name, item in base['manifest'].items():
        if name.startswith(('src/', 'src-tauri/src/', 'src-tauri/tests/')) or name == base['state']['report_path']:
            require(value['manifest'].get(name) == item, 'publication accepted product/evidence drift')
    fingerprint = closure.fingerprint(value['manifest'])
    closure.keys(value['checks'], CHECKS)
    for label, ref in value['checks'].items():
        check = reference(ref)
        require(check['label'] == label and check['cwd'] == str(ROOT)
                and check['input_fingerprint'] == fingerprint and check['exit_status'] == 0
                and check['complete_capture'] is True and check['timed_out'] is False
                and check['input_preserved'] is True
                and type(check['timeout_seconds']) is int and 0 < check['timeout_seconds'] <= 600
                and type(check['elapsed_seconds']) in {int, float}
                and 0 <= check['elapsed_seconds'] <= check['timeout_seconds']
                and check['argv'] == CHECK_ARGV[label],
                'publication required verification failed or stale')
    review = reference(value['review'])
    require(review == {'scope': 'memory-publication', 'result': 'PASS WITH ADVISORIES',
                      'input_fingerprint': fingerprint, 'historical_preserved': True,
                      'product_unchanged': True}, 'publication review invalid')


def present(root: Path) -> bool:
    return os.path.lexists(root / DIRECTORY)


@closure.guarded
def validate(root: Path, state: dict[str, Any]) -> dict[str, Any]:
    base = accepted(root, state)
    values = records(root)
    require(bool(values), 'publication missing receipt')
    for i, value in enumerate(values):
        check_record(root, value, base, values[i - 1] if i else None)
    value = values[-1]
    raw_index = dependent.stable_index(root)
    manifest, blobs = snapshot(root)
    require(manifest == value['manifest'], 'publication current workspace drift')
    current = gate.current_head_commit(root)
    staged = index_entries(root, raw_index)
    if current == value['parents'][0]:
        original = bytes.fromhex(base['index_hex'])
        require((len(values) == 1 and dependent.index_semantics(raw_index)
                 == dependent.index_semantics(original)) or staged == blobs,
                'publication partial or unrelated staging')
        phase = 'staged' if staged == blobs else 'prepared'
    else:
        require(commit_parents(root, current) == value['parents']
                and staged == blobs and tree_entries(root, current) == blobs,
                'publication unsupported commit/tree/ancestry')
        phase = 'committed'
    require(dependent.stable_index(root) == raw_index and gate.current_head_commit(root) == current,
            'publication index or HEAD race')
    return {'historical_acceptance_valid': True, 'current_publication_valid': True,
            'phase': phase, 'head': current, 'steps': len(values),
            'historical_C7_not_reexecuted': True}


@closure.guarded
def seal(root: Path, request_path: str) -> None:
    state = gate.read_state(root)
    require(state is not None, 'publication state missing')
    base = accepted(root, state)
    previous = records(root)
    require(len(previous) < MAX_STEPS, 'publication steps exhausted')
    value = reference({'path': request_path,
                       'sha256': closure.digest(read(Path(request_path)))})
    require(value not in previous, 'publication replay')
    if previous:
        for i, old in enumerate(previous):
            check_record(root, old, base, previous[i - 1] if i else None)
    check_record(root, value, base, previous[-1] if previous else None)
    raw_index = dependent.stable_index(root)
    manifest, blobs = snapshot(root)
    require(manifest == value['manifest'] and gate.current_head_commit(root) == value['parents'][0],
            'publication seal current binding')
    original = bytes.fromhex(base['index_hex'])
    require((not previous and dependent.index_semantics(raw_index)
             == dependent.index_semantics(original)) or index_entries(root, raw_index) == blobs,
            'publication seal partial staging')
    require(dependent.stable_index(root) == raw_index, 'publication seal index race')
    raw = lzma.compress(closure.encoded(value))
    name = DIRECTORY / f'{len(previous) + 1:04d}-{closure.digest(raw)}.xz'
    dependent.exclusive(root, name, raw)
    validate(root, state)
