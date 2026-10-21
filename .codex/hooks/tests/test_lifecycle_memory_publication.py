from __future__ import annotations

import copy
import hashlib
import json
import lzma
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import lifecycle_closure as closure
import lifecycle_memory_publication as publication
import post_increment_gate as gate


class MemoryPublicationTests(unittest.TestCase):
    """Real temporary Git histories; no live memory state or product execution."""

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name).resolve() / 'repo'
        self.root.mkdir()
        self.external = Path(self.temp.name).resolve() / 'evidence'
        self.external.mkdir()
        for args in [('init', '-q'), ('config', 'user.email', 'fixture@example.invalid'),
                     ('config', 'user.name', 'Synthetic fixture'),
                     ('config', 'commit.gpgsign', 'false'), ('branch', '-m', publication.BRANCH)]:
            self.git(*args)
        for name, body in {'.gitignore': '.codex/state/\n', 'src/public.txt': 'Accepted product\n',
                           'HANDOFF.md': '# Accepted history\n',
                           '.codex/hooks/post_increment_gate.py': '# Before maintenance\n',
                           'docs/reviews/accepted.md': '# Original completed report\n'}.items():
            self.write(name, body)
        self.git('add', '.'); self.git('commit', '-qm', 'Synthetic accepted base')
        self.head = gate.current_head_commit(self.root)
        manifest = closure.manifest(self.root)
        self.state = {'schema_version': 5, 'status': 'complete',
                      'increment_id': 'persistent-memory-codex-continuation',
                      'quality_gate': 'PASS WITH ADVISORIES', 'completion_marker': gate.COMPLETION_MARKER,
                      'report_path': 'docs/reviews/accepted.md',
                      'report_sha256': closure.digest((self.root / 'docs/reviews/accepted.md').read_bytes()),
                      'baseline_fingerprint': '0' * 64,
                      'workspace_fingerprint': closure.fingerprint(manifest),
                      'dependent_lineage': {'admission': '1' * 64}}
        self.write('.codex/state/post_increment_gate.json', closure.encoded(self.state))
        self.write('.codex/state/original-admission.json', b'{"synthetic":true}\n')
        state_sha = closure.digest(gate.state_path(self.root).read_bytes())
        self.accepted = {'root': str(self.root), 'branch': publication.BRANCH, 'head': self.head,
                         'state': self.state, 'state_sha256': state_sha, 'manifest': manifest,
                         'protected_records': {'.codex/state/original-admission.json':
                             closure.digest((self.root / '.codex/state/original-admission.json').read_bytes())},
                         'index_hex': closure.read_bytes(closure.git_index_path(self.root)).hex(),
                         'observed_status': {'status': 'complete', 'valid': True}}
        raw = lzma.compress(closure.encoded(self.accepted))
        (self.external / 'ACCEPTED-SNAPSHOT.json.xz').write_bytes(raw)
        for name, value in [('ROOT', self.root), ('HEAD', self.head), ('EXTERNAL', self.external),
                            ('STATE_SHA', state_sha), ('ACCEPTED_SHA', closure.digest(raw))]:
            patch = mock.patch.object(publication, name, value); patch.start(); self.addCleanup(patch.stop)
        self.write('.codex/hooks/post_increment_gate.py', '# Reviewed maintenance\n')
        self.write('.codex/hooks/lifecycle_memory_publication.py', '# Synthetic new reader\n')
        self.write('.codex/hooks/tests/test_lifecycle_memory_publication.py', '# Synthetic tests\n')
        self.write('HANDOFF.md', '# Accepted history\n\nPublication maintenance.\n')
        self.original_state = gate.state_path(self.root).read_bytes()

    def git(self, *args):
        env = dict(os.environ, GIT_OPTIONAL_LOCKS='0')
        return subprocess.check_output(['git', *args], cwd=self.root, env=env,
                                       stderr=subprocess.PIPE, timeout=15)

    def write(self, name, body):
        path = self.root / name; path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(body.encode() if isinstance(body, str) else body)

    def ref(self, name, value):
        raw = closure.encoded(value)
        path = self.external / (name + '-' + closure.digest(raw) + '.json')
        if path.exists():
            self.assertEqual(path.read_bytes(), raw)
        else:
            path.write_bytes(raw)
        return {'path': str(path), 'sha256': closure.digest(raw)}

    def request(self, previous=None, upstream=None):
        manifest, _ = publication.snapshot(self.root)
        fingerprint = closure.fingerprint(manifest)
        checks = {}
        for label in publication.CHECKS:
            checks[label] = self.ref(label + '.json', {
                'label': label, 'cwd': str(self.root), 'input_fingerprint': fingerprint,
                'exit_status': 0, 'complete_capture': True, 'timed_out': False,
                'input_preserved': True,
                'timeout_seconds': 120, 'elapsed_seconds': 0.01,
                'argv': publication.CHECK_ARGV[label]})
        review = self.ref('review.json', {'scope': 'memory-publication',
                     'result': 'PASS WITH ADVISORIES', 'input_fingerprint': fingerprint,
                     'historical_preserved': True, 'product_unchanged': True})
        return {'schema': 1, 'owner_approved': True,
                'accepted_sha256': publication.ACCEPTED_SHA,
                'prior': closure.digest(closure.encoded(previous)) if previous else None,
                'parents': [gate.current_head_commit(self.root)] + ([upstream] if upstream else []),
                'upstream': upstream, 'manifest': manifest, 'checks': checks, 'review': review}

    def seal(self, request=None):
        value = request if request is not None else self.request()
        path = self.external / 'request.json'; path.write_bytes(closure.encoded(value))
        publication.seal(self.root, str(path))
        return value

    def validate(self):
        return publication.validate(self.root, gate.read_state(self.root))

    def commit(self):
        self.git('add', '.'); self.git('commit', '-qm', 'Synthetic scoped publication')

    def test_prepared_staged_committed_and_full_stop(self):
        self.seal()
        self.assertEqual(self.validate()['phase'], 'prepared')
        self.git('add', '.')
        self.assertEqual(self.validate()['phase'], 'staged')
        self.git('commit', '-qm', 'Synthetic publication')
        self.assertEqual(self.validate()['phase'], 'committed')
        self.assertTrue(gate.validate_completed_state(self.root, self.state))
        decision = gate.evaluate_stop_payload({'hook_event_name': 'Stop',
                    'stop_hook_active': False, 'cwd': str(self.root)})
        self.assertFalse(decision.should_continue)
        self.assertEqual(gate.state_path(self.root).read_bytes(), self.original_state)

    def test_partial_staging_rejected(self):
        self.seal(); self.git('add', 'HANDOFF.md')
        with self.assertRaisesRegex(gate.GateError, 'partial'): self.validate()

    def test_bounded_compressed_request(self):
        path = self.external / 'request.json.xz'
        path.write_bytes(lzma.compress(closure.encoded(self.request())))
        publication.seal(self.root, str(path))
        self.assertEqual(self.validate()['phase'], 'prepared')

    def test_unrelated_workspace_change_rejected(self):
        self.seal(); self.write('unrelated.txt', 'Unexpected\n')
        with self.assertRaisesRegex(gate.GateError, 'workspace drift'): self.validate()

    def test_out_of_scope_candidate_rejected(self):
        self.write('unrelated.txt', 'Unexpected\n')
        with self.assertRaisesRegex(gate.GateError, 'unrelated scope'): self.seal()

    def test_product_changes_not_qualified(self):
        self.write('src/public.txt', 'Changed product\n')
        with self.assertRaises(gate.GateError): self.seal()

    def test_old_report_cannot_be_rewritten(self):
        self.write('docs/reviews/accepted.md', 'Changed old result\n')
        with self.assertRaisesRegex(gate.GateError, 'accepted report drift'): self.seal()

    def test_historical_admission_preserved(self):
        self.seal(); self.write('.codex/state/original-admission.json', '{}\n')
        with self.assertRaisesRegex(gate.GateError, 'historical record drift'): self.validate()

    def test_completion_state_preserved(self):
        self.seal(); self.write('.codex/state/post_increment_gate.json', '{}\n')
        with self.assertRaises(gate.GateError): self.validate()

    def test_failed_stale_incomplete_timeout_and_wrong_command_checks(self):
        baseline = self.request()
        for field, bad in [('exit_status', 1), ('input_fingerprint', '0' * 64),
                           ('complete_capture', False), ('timed_out', True),
                           ('input_preserved', False),
                           ('timeout_seconds', 0), ('elapsed_seconds', 121),
                           ('argv', ['true'])]:
            value = copy.deepcopy(baseline)
            check = json.loads(Path(value['checks']['focused']['path']).read_bytes())
            check[field] = bad
            value['checks']['focused'] = self.ref('bad-check.json', check)
            with self.subTest(field=field):
                with self.assertRaisesRegex(gate.GateError, 'verification failed or stale'):
                    self.seal(value)

    def test_missing_check_rejected(self):
        value = self.request(); value['checks'].pop('hooks')
        with self.assertRaises(gate.GateError): self.seal(value)

    def test_replay_rejected_without_second_record(self):
        value = self.seal()
        with self.assertRaisesRegex(gate.GateError, 'replay'): self.seal(value)
        self.assertEqual(len(list((self.root / publication.DIRECTORY).iterdir())), 1)

    def test_extra_descendant_is_not_accepted(self):
        self.seal(); self.commit(); self.git('commit', '--allow-empty', '-qm', 'Unreviewed child')
        with self.assertRaisesRegex(gate.GateError, 'unsupported commit'): self.validate()

    def test_different_commit_tree_is_rejected(self):
        self.seal(); self.write('HANDOFF.md', 'Different candidate\n'); self.commit()
        with self.assertRaises(gate.GateError): self.validate()

    def test_mode_change_rejected(self):
        self.seal(); (self.root / 'src/public.txt').chmod(0o755)
        with self.assertRaisesRegex(gate.GateError, 'workspace drift'): self.validate()

    def test_symlink_and_multiple_links_rejected(self):
        p = self.external / 'link'; p.symlink_to(self.external / 'ACCEPTED-SNAPSHOT.json.xz')
        with self.assertRaisesRegex(gate.GateError, 'unsafe path'): publication.read(p)
        p.unlink(); os.link(self.external / 'ACCEPTED-SNAPSHOT.json.xz', p)
        with self.assertRaisesRegex(gate.GateError, 'unsafe file'): publication.read(p)

    def test_read_race_rejected(self):
        p = self.external / 'race'; p.write_bytes(b'original')
        real = os.read
        def replacing(fd, size):
            data = real(fd, size)
            p.write_bytes(b'replaced')
            return data
        with mock.patch.object(publication.os, 'read', side_effect=replacing):
            with self.assertRaisesRegex(gate.GateError, 'read race'): publication.read(p)

    def test_group_writable_helper_rejected(self):
        p = self.external / 'writable'; p.write_bytes(b'public synthetic helper')
        p.chmod(0o664)
        with self.assertRaisesRegex(gate.GateError, 'unsafe file'): publication.read(p)

    def test_index_checksum_and_lock_rejected(self):
        self.seal(); p = closure.git_index_path(self.root); old = p.read_bytes()
        p.write_bytes(old[:-1] + bytes([old[-1] ^ 1]))
        with self.assertRaises(gate.GateError): self.validate()
        p.write_bytes(old); lock = Path(str(p) + '.lock'); lock.write_bytes(b'')
        with self.assertRaisesRegex(gate.GateError, 'index lock'): self.validate()

    def test_skip_worktree_not_an_inactivity_or_preservation_exception(self):
        self.seal(); self.git('update-index', '--skip-worktree', 'src/public.txt')
        with self.assertRaises(gate.GateError): self.validate()

    def test_unknown_or_partial_record_rejected(self):
        self.seal(); (self.root / publication.DIRECTORY / 'partial.tmp').write_bytes(b'')
        with self.assertRaisesRegex(gate.GateError, 'incomplete or ambiguous'): self.validate()

    def test_compressed_trailing_data_and_bomb_rejected(self):
        with self.assertRaises(gate.GateError): publication.unpack(lzma.compress(b'{}') + b'junk')
        with mock.patch.object(publication, 'MAX_BYTES', 10):
            with self.assertRaises(gate.GateError): publication.unpack(lzma.compress(b' ' * 11))

    def test_public_tree_census_bounds_and_timeout(self):
        entries = publication.tree_entries(self.root, self.head)
        headers = b''.join((oid + ' blob ' + str(publication.MAX_BYTES + 1) + '\n').encode()
                           for _, oid in entries.values())
        response = subprocess.CompletedProcess(['git'], 0, stdout=headers, stderr=b'')
        with mock.patch.object(publication, 'tree_entries', return_value=entries):
            with mock.patch.object(publication.subprocess, 'run', return_value=response):
                with self.assertRaisesRegex(gate.GateError, 'census bounds'):
                    publication.verify_tree_contents(self.root, self.head, self.accepted['manifest'])
            with mock.patch.object(publication.subprocess, 'run',
                                   side_effect=subprocess.TimeoutExpired(['git'], 15)):
                with self.assertRaisesRegex(gate.GateError, 'capture timeout'):
                    publication.verify_tree_contents(self.root, self.head, self.accepted['manifest'])

    def test_wrong_root_or_branch_rejected(self):
        self.seal(); self.git('branch', '-m', 'wrong')
        with self.assertRaisesRegex(gate.GateError, 'checkout identity'): self.validate()

    def test_index_race_rejected(self):
        self.seal(); real = publication.dependent.stable_index
        original = real(self.root)
        with mock.patch.object(publication.dependent, 'stable_index', side_effect=[original, original + b'x']):
            with self.assertRaisesRegex(gate.GateError, 'race'): self.validate()

    def test_recorder_failure_does_not_publish_success(self):
        with mock.patch.object(publication.dependent, 'exclusive', side_effect=OSError('synthetic')):
            with self.assertRaises(gate.GateError): self.seal()
        self.assertFalse(publication.present(self.root))

    def test_failed_review_rejected(self):
        value = self.request(); review = json.loads(Path(value['review']['path']).read_bytes())
        review['result'] = 'FAIL'; value['review'] = self.ref('bad-review.json', review)
        with self.assertRaisesRegex(gate.GateError, 'review invalid'): self.seal(value)

    def test_supported_exact_upstream_integration(self):
        first = self.seal(); self.commit(); publication_head = gate.current_head_commit(self.root)
        self.git('checkout', '-qb', 'synthetic-upstream', self.head)
        self.write('docs/upstream.md', '# Verified upstream documentation\n')
        self.git('add', '.'); self.git('commit', '-qm', 'Synthetic upstream')
        upstream = gate.current_head_commit(self.root)
        self.git('checkout', '-q', publication.BRANCH)
        self.git('merge', '--no-commit', '--no-ff', upstream)
        value = self.request(previous=first, upstream=upstream)
        # New check records must retain the immutable earlier records.
        old = publication.records(self.root)[0]
        for ref in old['checks'].values():
            name = Path(ref['path']).name
            self.assertTrue((self.external / name).exists())
        self.assertEqual(value['parents'], [publication_head, upstream])
        self.seal(value); self.git('commit', '-qm', 'Synthetic reviewed integration')
        self.assertEqual(self.validate()['phase'], 'committed')


if __name__ == '__main__':
    unittest.main()
