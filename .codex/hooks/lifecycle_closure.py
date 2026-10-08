"""Owner-directed closure of unfinished work; no failure-to-pass conversion.

These local receipts are workflow evidence, not authentication against the same
OS user. Historical integrity and current workspace readiness are separate.
"""
from __future__ import annotations

import hashlib
import json
import os
import re
import stat
import tempfile
from functools import wraps
from pathlib import Path
from typing import Any

import post_increment_gate as gate

DIRECTORY = Path('.codex/state/closures')
MAX_BYTES = 4 * 1024 * 1024
REQUIRED_COMMANDS = frozenset({
    'npm run verify', 'npm run docs:check', 'npm run repository:check',
    'npm run security:scan', 'git diff --check',
    'python3 -B .codex/hooks/session_end_gate.py',
})
REQUIRED_REVIEWS = frozenset({
    'Preservation and scope review',
    'Architecture, security, code-health and readiness review',
})


def require(condition: bool, message: str) -> None:
    if not condition:
        raise gate.GateError('closure: ' + message)


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def encoded(value: Any) -> bytes:
    return (json.dumps(value, sort_keys=True, indent=2) + '\n').encode()


def guarded(function):
    @wraps(function)
    def call(*args, **kwargs):
        try:
            return function(*args, **kwargs)
        except gate.GateError:
            raise
        except (OSError, ValueError, TypeError, KeyError, AttributeError, IndexError, RecursionError) as error:
            raise gate.GateError('closure: malformed or unavailable evidence') from error
    return call


def sha(value: Any) -> str:
    require(isinstance(value, str) and re.fullmatch('[0-9a-f]{64}', value) is not None,
            'invalid digest')
    return value


def keys(value: Any, expected: set[str]) -> None:
    require(isinstance(value, dict) and set(value) == expected, 'invalid record fields')


def read_bytes(path: Path) -> bytes:
    require(path.is_absolute(), 'absolute evidence path required')
    require(not any(p.is_symlink() for p in [path, *path.parents]), 'symlink evidence')
    try:
        require(path.is_file() and path.stat().st_size <= MAX_BYTES, 'unsafe or oversized evidence')
        return path.read_bytes()
    except OSError as error:
        raise gate.GateError('closure: unreadable evidence') from error


def read_json(path: Path) -> Any:
    try:
        return json.loads(read_bytes(path))
    except (ValueError, UnicodeError) as error:
        raise gate.GateError('closure: invalid JSON evidence') from error


def git_index_path(root: Path) -> Path:
    """Resolve the owning worktree index without assuming .git is a directory."""
    try:
        value = gate._run_git(root, 'rev-parse', '--git-path', 'index').decode('utf-8').strip()
    except UnicodeDecodeError as error:
        raise gate.GateError('closure: invalid Git index path') from error
    require(bool(value) and '\n' not in value and '\r' not in value, 'invalid Git index path')
    path = Path(value)
    path = path if path.is_absolute() else root / path
    # read_bytes preserves regular-file, symlink and size validation.
    read_bytes(path)
    return path


def reject_d133_context(root: Path, state: dict[str, Any] | None) -> None:
    # D-133 remains a separate immutable acceptance route, not generic closure.
    import lifecycle_acceptance as acceptance
    require(not (state and 'acceptance_lineage' in state) and
            not (root / acceptance.POINTER).exists(), 'D-133 requires its recorded acceptance route')


def relative(root: Path, name: str) -> Path:
    gate.validate_relative_path(name)
    return root / name


def path_list(value: Any, *, empty: bool = False) -> list[str]:
    require(isinstance(value, list) and (empty or bool(value)), 'missing path list')
    require(all(isinstance(p, str) for p in value), 'invalid path list')
    require(len(value) == len(set(value)), 'duplicate paths')
    for name in value:
        gate.validate_relative_path(name)
        require(name.rstrip('/') not in ('', '.') and Path(name).as_posix() == name.rstrip('/'),
                'noncanonical scope path')
        require(not name.startswith('.git/'), 'Git internals excluded from scope')
    return value


def manifest(root: Path) -> dict[str, Any]:
    names = gate._decode_git_paths(gate._run_git(
        root, 'ls-files', '--cached', '--others', '--exclude-standard', '-z'))
    result = {}
    for name in sorted(set(names)):
        p = relative(root, name)
        if not p.exists() and not p.is_symlink():
            result[name] = {'exists': False}
            continue
        require(not p.is_symlink() and p.is_file(), 'only regular workspace files supported')
        result[name] = {'exists': True, 'mode': stat.S_IMODE(p.stat().st_mode),
                        'sha256': gate._hash_file(p).hex()}
    return result


def normalize_manifest(value: Any) -> dict[str, Any]:
    require(isinstance(value, dict) and bool(value), 'missing historical inventory')
    result = {}
    for name, item in value.items():
        gate.validate_relative_path(name)
        require(isinstance(item, dict) and type(item.get('exists')) is bool,
                'invalid historical inventory entry')
        if not item['exists']:
            result[name] = {'exists': False}
        else:
            require(type(item.get('mode')) is int and 0 <= item['mode'] <= 0o7777,
                    'invalid historical mode')
            require(item.get('type', 'regular') == 'regular', 'unsupported historical type')
            result[name] = {'exists': True, 'mode': item['mode'], 'sha256': sha(item.get('sha256'))}
    return result


def fingerprint(inventory: dict[str, Any]) -> str:
    value = hashlib.sha256()
    for name, item in sorted(inventory.items()):
        if not item['exists']:
            continue
        path = name.encode()
        value.update(len(path).to_bytes(8, 'big'))
        value.update(path)
        value.update((item['mode'] & 0o111).to_bytes(2, 'big'))
        value.update(b'file')
        value.update(bytes.fromhex(item['sha256']))
    return value.hexdigest()


def changed(before: dict[str, Any], after: dict[str, Any]) -> set[str]:
    # Git drops a tracked deletion from ls-files when staged. Compare canonical
    # absence equally without changing either recorded inventory or its digest.
    absent = {'exists': False}
    return {n for n in set(before) | set(after)
            if before.get(n, absent) != after.get(n, absent)}


def authorization(root: Path, value: Any) -> None:
    keys(value, {'approved', 'source', 'decision_path', 'decision_sha256'})
    require(value['approved'] is True, 'recorded owner authorization required')
    require(isinstance(value['source'], str) and bool(value['source'].strip()),
            'owner instruction source required')
    require(value['decision_path'] == 'DECISIONS.md', 'durable decision required')
    require(digest(read_bytes(root / 'DECISIONS.md')) == sha(value['decision_sha256']),
            'owner decision binding drift')


def unresolved(report_text: str) -> list[str]:
    m = gate._extract_manifest(report_text)
    values = []
    for kind in ('verification', 'manual_verification'):
        for entry in m[kind]:
            if entry['required'] and entry['status'] != 'Passed':
                values.append(kind + ':' + digest(encoded(entry)))
    for finding in m['findings']:
        if finding['blocks_completion'] or finding['blocks_next_increment']:
            values.append('finding:' + digest(encoded(finding)))
    for criterion, entry in m.get('milestone', {}).get('criteria', {}).items():
        if entry['status'] not in ('automatically_verified', 'live_verified'):
            values.append('criterion:' + digest(encoded({'criterion': criterion, **entry})))
    return sorted(values)


def reviews(m: dict[str, Any], *, full: bool) -> None:
    commands = REQUIRED_COMMANDS if full else REQUIRED_COMMANDS - {'npm run verify'}
    for command in commands:
        entries = [e for e in m['verification'] if e['command'] == command]
        require(len(entries) == 1 and entries[0]['required'] is True
                and entries[0]['status'] == 'Passed', 'required verification missing or failed: ' + command)
    for check in REQUIRED_REVIEWS:
        entries = [e for e in m['manual_verification'] if e['check'] == check]
        require(len(entries) == 1 and entries[0]['required'] is True
                and entries[0]['status'] == 'Passed', 'required review missing or failed')


def atomic_record(root: Path, name: str, value: dict[str, Any]) -> None:
    """Only publish a complete receipt; interrupted temporary files confer no authority."""
    directory = root / DIRECTORY
    directory.mkdir(parents=True, exist_ok=True)
    require(directory.resolve() == directory and not directory.is_symlink(), 'unsafe receipt directory')
    path = directory / name
    data = encoded(value)
    if path.exists():
        require(read_bytes(path) == data, 'changed replay rejected')
        return
    temporary = None
    try:
        with tempfile.NamedTemporaryFile(dir=directory, prefix='.pending-', delete=False) as f:
            temporary = Path(f.name)
            f.write(data)
            f.flush()
            os.fsync(f.fileno())
        # Same-user concurrent mutation is not authentication; reject observed races.
        try:
            os.link(temporary, path)
        except FileExistsError:
            require(read_bytes(path) == data, 'concurrent changed receipt publication')
    finally:
        if temporary is not None and temporary.exists():
            temporary.unlink()


def receipt_path(root: Path, state_digest: str) -> Path:
    return root / DIRECTORY / (sha(state_digest) + '.json')


def current_receipt(root: Path) -> str | None:
    state_file = gate.state_path(root)
    if not state_file.exists():
        return None
    value = digest(read_bytes(state_file))
    return value if receipt_path(root, value).exists() else None


@guarded
def historical(root: Path, identity: str, seen: set[str] | None = None,
               cache: dict[str, Any] | None = None) -> dict[str, Any]:
    cache = {} if cache is None else cache
    if identity in cache:
        return cache[identity]
    seen = set() if seen is None else set(seen)
    require(identity not in seen and len(seen) < 32, 'cyclic or excessive history')
    seen.add(identity)
    record = read_json(receipt_path(root, identity))
    keys(record, {'payload', 'sha256'})
    p = record['payload']
    require(digest(encoded(p)) == sha(record['sha256']), 'closure receipt tampered')
    keys(p, {'request', 'raw_state', 'raw_report', 'historical_manifest',
             'result_manifest', 'workspace_fingerprint', 'report_sha256',
             'raw_verification_report', 'prior_closures'})
    require(digest(p['raw_state'].encode()) == identity, 'original state mismatch')
    state = json.loads(p['raw_state'])
    gate.validate_state(state)
    require(state['status'] == 'failed' and state['quality_gate'] == 'FAIL'
            and 'completion_marker' not in state, 'historical failure required')
    require(digest(p['raw_report'].encode()) == state['report_sha256'], 'original report mismatch')
    require(read_bytes(relative(root, state['report_path'])) == p['raw_report'].encode(),
            'original report missing or changed')
    require(fingerprint(normalize_manifest(p['historical_manifest'])) == state['workspace_fingerprint'],
            'historical workspace mismatch')
    require(fingerprint(normalize_manifest(p['result_manifest'])) == p['workspace_fingerprint'],
            'closure workspace evidence mismatch')
    request = p['request']
    require(request['failed_state_sha256'] == identity, 'wrong closure identity')
    require(request['owner_authorization']['approved'] is True, 'missing historical owner authorization')
    require(request['owner_authorization']['decision_sha256'] == p['result_manifest']['DECISIONS.md']['sha256'],
            'historical owner decision mismatch')
    require(set(unresolved(p['raw_report'])).issubset(request['disposition']['unresolved']),
            'unresolved failure omitted')
    require(digest(p['raw_verification_report'].encode()) == p['report_sha256'], 'verification report mismatch')
    require(read_bytes(relative(root, request['verification']['report_path']))
            == p['raw_verification_report'].encode(), 'closure verification report changed')
    vm = gate._extract_manifest(p['raw_verification_report'])
    require(vm['quality_gate'] in {'PASS', 'PASS WITH ADVISORIES'}, 'nonpassing closure evidence')
    reviews(vm, full=True)
    require(isinstance(p['prior_closures'], list) and len(p['prior_closures']) <= 32,
            'invalid prior history')
    require(p['prior_closures'] == state.get('closure_lineage', {}).get('closures', []),
            'prior closure history omitted')
    for old in p['prior_closures']:
        historical(root, old, seen, cache)
    cache[identity] = p
    return p


@guarded
def close(root: Path, request_path: str) -> None:
    reject_d133_context(root, gate.read_state(root))
    raw_request = read_bytes(relative(root, request_path))
    request = json.loads(raw_request)
    keys(request, {'schema_version', 'owner_authorization', 'failed_state_sha256',
                   'backup_directory', 'manifest_sha256', 'allowed_paths', 'disposition', 'verification'})
    require(type(request['schema_version']) is int and request['schema_version'] == 1, 'unsupported schema')
    authorization(root, request['owner_authorization'])
    identity = sha(request['failed_state_sha256'])
    existing = receipt_path(root, identity)
    if existing.exists():
        p = historical(root, identity)
        require(p['request'] == request, 'changed replay rejected')
        require(gate.workspace_fingerprint(root) == p['workspace_fingerprint'], 'workspace drift on replay')
        require(digest(read_bytes(gate.state_path(root))) == identity, 'live state changed on replay')
        return
    require(not gate.has_merge_conflicts(root), 'merge conflicts')
    require(not gate.suspicious_changed_paths(gate.changed_paths(root)), 'suspicious changed paths')
    raw_state = read_bytes(gate.state_path(root))
    require(digest(raw_state) == identity, 'live failure drift')
    state = gate.read_state(root)
    require(state is not None and state['status'] == 'failed', 'terminal failure required')
    require('successor_disposition' not in state, 'existing historical admission must not be replaced')
    require(state['head_commit'] == gate.current_head_commit(root), 'HEAD drift')
    backup = Path(request['backup_directory'])
    raw_manifest = read_bytes(backup / 'workspace-before.json')
    require(digest(raw_manifest) == sha(request['manifest_sha256']), 'backup inventory tampered')
    before = normalize_manifest(json.loads(raw_manifest))
    require(fingerprint(before) == state['workspace_fingerprint'], 'backup does not prove historical workspace')
    require(read_bytes(backup / 'before' / gate.STATE_RELATIVE_PATH) == raw_state, 'raw state backup mismatch')
    require(read_bytes(backup / 'before/.git/index') == read_bytes(git_index_path(root)), 'Git index drift')
    for name, item in before.items():
        p = backup / 'before' / name
        if item['exists']:
            require(digest(read_bytes(p)) == item['sha256'], 'incomplete or altered backup: ' + name)
        else:
            require(not p.exists(), 'historical absence mismatch')
    report = read_bytes(relative(root, state['report_path']))
    require(digest(report) == state['report_sha256'], 'historical report drift')
    require(read_bytes(backup / 'before' / state['report_path']) == report, 'report backup mismatch')
    allowed = path_list(request['allowed_paths'])
    require(state['report_path'] not in allowed and not any(p.startswith('.codex/state/') for p in allowed),
            'historical report and live state are protected')
    after = manifest(root)
    require(changed(before, after).issubset(allowed), 'unapproved workspace change or incomplete preservation')
    d = request['disposition']
    keys(d, {'retained', 'deferred', 'restored', 'unresolved'})
    for name in ('retained', 'deferred', 'unresolved'):
        gate._string_list(d[name], name)
    path_list(d['restored'], empty=True)
    require(set(d['restored']).issubset(allowed), 'restoration exceeds authorization')
    require(set(unresolved(report.decode())).issubset(d['unresolved']), 'unresolved acceptance criteria omitted')
    verification = request['verification']
    keys(verification, {'increment_id', 'report_path'})
    require(verification['increment_id'] != state['increment_id'], 'closure cannot pass the failed task')
    m, path, report_digest = gate.validate_report(root, verification['report_path'], verification['increment_id'])
    reviews(m, full=True)
    old = state.get('closure_lineage', {}).get('closures', [])
    for item in old:
        historical(root, item)
    payload = {'request': request, 'raw_state': raw_state.decode(), 'raw_report': report.decode(),
               'historical_manifest': before, 'result_manifest': after,
               'workspace_fingerprint': fingerprint(after), 'report_sha256': report_digest,
               'raw_verification_report': read_bytes(path).decode(), 'prior_closures': old}
    require(gate.workspace_fingerprint(root) == payload['workspace_fingerprint'], 'workspace changed during closure')
    require(read_bytes(gate.state_path(root)) == raw_state, 'state changed during closure')
    require(read_bytes(backup / 'before/.git/index') == read_bytes(git_index_path(root)), 'index changed during closure')
    require(state['head_commit'] == gate.current_head_commit(root), 'HEAD changed during closure')
    require(read_bytes(relative(root, request_path)) == raw_request, 'request changed during closure')
    atomic_record(root, identity + '.json', {'payload': payload, 'sha256': digest(encoded(payload))})


def milestone_contract(value: Any) -> None:
    """An immutable objective replaces path-count admission for new milestones."""
    keys(value, {'objective', 'exclusions', 'acceptance', 'protected_paths',
                 'authorized_destructive_paths'})
    require(isinstance(value['objective'], str) and bool(value['objective'].strip()),
            'milestone objective required')
    gate._string_list(value['exclusions'], 'milestone exclusions')
    acceptance = value['acceptance']
    require(isinstance(acceptance, dict) and bool(acceptance), 'acceptance checklist required')
    for key, description in acceptance.items():
        gate.validate_increment_id(key)
        require(isinstance(description, str) and bool(description.strip()), 'empty acceptance criterion')
    path_list(value['protected_paths'], empty=True)
    path_list(value['authorized_destructive_paths'], empty=True)


def protected_match(name: str, protected: list[str] | set[str]) -> bool:
    return any(name == p.rstrip('/') or name.startswith(p.rstrip('/') + '/') for p in protected)


def validate_milestone_delta(admission: dict[str, Any], after: dict[str, Any]) -> set[str]:
    contract = admission['request']['milestone']
    milestone_contract(contract)
    before = admission['baseline_manifest']
    delta = changed(before, after)
    protected = contract['protected_paths'] + [admission['request']['readiness_report']]
    for name in delta:
        require(not protected_match(name, protected) and not name.startswith('.codex/state/'),
                'protected path changed: ' + name)
        original, current = before.get(name, {}), after.get(name, {})
        destructive = original.get('exists') and (not current.get('exists') or
                      original.get('mode') != current.get('mode'))
        require(not destructive or name in contract['authorized_destructive_paths'],
                'destructive path change requires explicit authorization: ' + name)
    return delta


@guarded
def milestone_report(root: Path, increment_id: str, report: dict[str, Any]) -> bool:
    """Validate review attribution, not semantic truth or platform authorization.

    Every task delta must serve a frozen acceptance criterion. The human/code
    review checks the rationale; these same-user records cannot authenticate it.
    """
    state = gate.read_state(root)
    lineage = state.get('closure_lineage') if state else None
    admission = None
    if lineage and state['increment_id'] == increment_id:
        data = read_bytes(root / DIRECTORY / ('admission-' + lineage['admission'] + '.json'))
        require(digest(data) == lineage['admission'], 'admission evidence tampered')
        admission = json.loads(data)
    if admission is None or admission['request']['schema_version'] != 2:
        require('milestone' not in report, 'milestone evidence needs objective-based admission')
        return False
    evidence = report.get('milestone')
    keys(evidence, {'criteria', 'paths'})
    contract = admission['request']['milestone']
    milestone_contract(contract)
    criteria = evidence['criteria']
    require(isinstance(criteria, dict) and set(criteria) == set(contract['acceptance']),
            'acceptance checklist incomplete or expanded')
    blocked = False
    for value in criteria.values():
        keys(value, {'status', 'evidence'})
        require(value['status'] in ('automatically_verified', 'live_verified',
                                   'implemented', 'deferred', 'blocked'), 'invalid acceptance status')
        require(isinstance(value['evidence'], str) and bool(value['evidence'].strip()),
                'acceptance evidence required')
        blocked |= value['status'] not in ('automatically_verified', 'live_verified')
    delta = validate_milestone_delta(admission, manifest(root))
    paths = evidence['paths']
    require(isinstance(paths, dict) and set(paths) == delta, 'task path attribution incomplete')
    for value in paths.values():
        keys(value, {'criterion', 'rationale', 'within_objective', 'preserves_existing'})
        require(isinstance(value['criterion'], str) and value['criterion'] in criteria,
                'path lacks an accepted criterion')
        require(value['within_objective'] is True, 'unrelated scope expansion')
        require(value['preserves_existing'] is True, 'unrelated work attribution unresolved')
        require(isinstance(value['rationale'], str) and bool(value['rationale'].strip()),
                'path rationale required')
    return blocked


@guarded
def lineage_shape(value: Any) -> None:
    keys(value, {'closures', 'admission'})
    require(isinstance(value['closures'], list) and len(value['closures']) <= 32,
            'invalid closure history')
    require(len(set(value['closures'])) == len(value['closures']), 'duplicate closure history')
    for item in value['closures']:
        sha(item)
    sha(value['admission'])


@guarded
def validate_lineage(root: Path, state: dict[str, Any], *, scope: bool = True) -> None:
    lineage = state.get('closure_lineage')
    if lineage is None:
        return
    lineage_shape(lineage)
    history_cache: dict[str, Any] = {}
    for identity in lineage['closures']:
        historical(root, identity, cache=history_cache)
    data = read_bytes(root / DIRECTORY / ('admission-' + lineage['admission'] + '.json'))
    require(digest(data) == lineage['admission'], 'admission evidence tampered')
    admission = json.loads(data)
    require(admission['closures'] == lineage['closures'] and
            admission['request']['increment_id'] == state['increment_id'], 'admission identity mismatch')
    require(digest(admission['raw_readiness_report'].encode()) == admission['readiness_report_sha256']
            and read_bytes(relative(root, admission['request']['readiness_report']))
            == admission['raw_readiness_report'].encode(), 'readiness evidence changed')
    if scope:
        if admission['request']['schema_version'] == 2:
            validate_milestone_delta(admission, manifest(root))
        else:
            require(changed(admission['baseline_manifest'], manifest(root)).issubset(
                admission['request']['allowed_paths']), 'successor exceeds authorized scope')


@guarded
def begin(root: Path, increment_id: str, request_path: str | None) -> bool:
    state = gate.read_state(root)
    current = current_receipt(root)
    old = [] if state is None else state.get('closure_lineage', {}).get('closures', [])
    if current is None and not old and not (state and 'closure_lineage' in state) and request_path is None:
        return False
    reject_d133_context(root, state)
    require(state is None or state['status'] != 'active', 'another increment is active')
    require(request_path is not None, 'separate recorded owner admission required')
    require(current is not None or state is None or state['status'] == 'complete',
            'new terminal failure needs its own closure')
    closures = list(dict.fromkeys(old + ([current] if current else [])))
    if state is not None:
        validate_lineage(root, state, scope=False)
    history_cache: dict[str, Any] = {}
    records = [historical(root, item, cache=history_cache) for item in closures]
    require(all(json.loads(p['raw_state'])['increment_id'] != increment_id for p in records),
            'failed task cannot be relabeled or restarted')
    request = read_json(relative(root, request_path))
    require(isinstance(request, dict), 'invalid admission request')
    version = request.get('schema_version')
    require(type(version) is int and version in (1, 2), 'unsupported admission schema')
    keys(request, {'schema_version', 'owner_authorization', 'increment_id', 'workspace_fingerprint',
                   'allowed_paths' if version == 1 else 'milestone',
                   'dependencies', 'readiness_report', 'readiness_increment'})
    require(bool(closures) or version == 2, 'legacy admission requires closed failure history')
    authorization(root, request['owner_authorization'])
    require(request['increment_id'] == increment_id, 'wrong authorized successor')
    require(request['workspace_fingerprint'] == gate.workspace_fingerprint(root), 'admission workspace drift')
    allowed = path_list(request['allowed_paths']) if version == 1 else []
    if version == 2:
        milestone_contract(request['milestone'])
    protected = {json.loads(p['raw_state'])['report_path'] for p in records}
    protected |= {p['request']['verification']['report_path'] for p in records}
    protected.add(request['readiness_report'])
    if state is not None and state['status'] == 'complete':
        require(digest(read_bytes(relative(root, state['report_path']))) == state['report_sha256'],
                'completed predecessor report changed')
        protected.add(state['report_path'])
    require(not protected.intersection(allowed) and not any(p.startswith('.codex/state/') for p in allowed),
            'historical evidence cannot be successor scope')
    if version == 2:
        request['milestone']['protected_paths'] = sorted(set(
            request['milestone']['protected_paths']) | protected)
        require(not any(protected_match(p, request['milestone']['protected_paths'])
                        for p in request['milestone']['authorized_destructive_paths']),
                'protected evidence cannot authorize destruction')
    expected = {identity + '/' + issue for identity, p in zip(closures, records)
                for issue in p['request']['disposition']['unresolved']}
    dependencies = request['dependencies']
    require(isinstance(dependencies, dict) and set(dependencies) == expected, 'incomplete dependency assessment')
    for value in dependencies.values():
        keys(value, {'relationship', 'rationale'})
        require(value['relationship'] == 'independent' and isinstance(value['rationale'], str)
                and bool(value['rationale'].strip()), 'successor depends on unresolved failure')
    require(not gate.has_merge_conflicts(root), 'merge conflicts')
    require(not gate.suspicious_changed_paths(gate.changed_paths(root)), 'suspicious changed paths')
    m, ready_path, ready_digest = gate.validate_report(root, request['readiness_report'], request['readiness_increment'])
    reviews(m, full=False)
    require(m['next_increment_readiness'] in {'Ready', 'Ready with advisories'}, 'successor readiness blocked')
    p = {'request': request, 'closures': closures, 'baseline_manifest': manifest(root),
         'raw_readiness_report': read_bytes(ready_path).decode(), 'readiness_report_sha256': ready_digest}
    data = encoded(p)
    identity = digest(data)
    atomic_record(root, 'admission-' + identity + '.json', p)
    require(request['workspace_fingerprint'] == gate.workspace_fingerprint(root), 'workspace changed during admission')
    gate.write_state(root, {'baseline_fingerprint': request['workspace_fingerprint'],
                           'increment_id': increment_id, 'schema_version': gate.STATE_SCHEMA_VERSION,
                           'status': 'active', 'closure_lineage': {'closures': closures, 'admission': identity}})
    return True
