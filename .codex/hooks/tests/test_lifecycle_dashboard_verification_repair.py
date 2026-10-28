"""Synthetic temporary repositories only; no actual admission or provider use."""
import copy
import json
import shutil
import sys
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
sys.path.insert(0, str(Path(__file__).resolve().parent))
import lifecycle_closure as c
import lifecycle_dashboard_verification as v
import lifecycle_dashboard_verification_repair as repair
import post_increment_gate as g
import test_lifecycle_dashboard_verification as fixture


class DashboardRepairTests(unittest.TestCase):
    def setUp(self):
        self.v = fixture.DashboardVerificationTests(methodName="runTest")
        self.addCleanup(self.v.doCleanups)
        self.v.setUp()
        self.f, self.root = self.v.f, self.v.root
        self.v.admit(); self.v.final(failed=True)
        g.close_failed_gate(self.root, v.SUCCESSOR, v.REPORT)
        self.raw = g.state_path(self.root).read_bytes()
        self.failed_report = (self.root / v.REPORT).read_bytes()
        self.pointer = (self.root / v.POINTER).read_bytes()
        state = g.read_state(self.root)
        before = c.manifest(self.root)
        backup = self.root / '.codex/state/repair-backup'
        for name in before:
            dest = backup / 'before' / name
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(self.root / name, dest)
        for name, source in [('.git/index', c.git_index_path(self.root)),
                             (g.STATE_RELATIVE_PATH.as_posix(), g.state_path(self.root))]:
            dest = backup / 'before' / name
            dest.parent.mkdir(parents=True, exist_ok=True); shutil.copy2(source, dest)
        self.f.write_json('.codex/state/repair-backup/workspace-before.json', before)
        for name, value in {'FAILED_STATE': c.digest(self.raw), 'FAILED_REPORT': state['report_sha256'],
                            'PREVIOUS_ADMISSION': state[v.LINEAGE]['admission'],
                            'INDEX_SHA256': c.digest(c.git_index_path(self.root).read_bytes())}.items():
            p = mock.patch.object(repair, name, value); p.start(); self.addCleanup(p.stop)
        self.f.write('DECISIONS.md', 'Separate synthetic repair/closure/admission authority.\n')
        self.f.write('.prettierignore', '# Exact synthetic historical exclusion\n' + v.REPORT + '\n')
        p = mock.patch.object(repair, 'IGNORE_SHA256', c.digest((self.root / '.prettierignore').read_bytes()))
        p.start(); self.addCleanup(p.stop)
        temp = self.f.review(repair.MAINTENANCE)
        self.f.write(repair.MAINTENANCE_REPORT, (self.root / temp).read_text())
        m = g._extract_manifest((self.root / repair.MAINTENANCE_REPORT).read_text())
        m['files_changed'] = list(g.changed_paths(self.root))
        self.f.write_review(repair.MAINTENANCE_REPORT, m)
        request = {'schema_version': 1, 'owner_authorization': self.f.auth(),
                   'failed_state_sha256': repair.FAILED_STATE, 'backup_directory': str(backup),
                   'manifest_sha256': c.digest((backup / 'workspace-before.json').read_bytes()),
                   'allowed_paths': ['DECISIONS.md', '.prettierignore', temp, repair.MAINTENANCE_REPORT],
                   'disposition': {'retained': ['All synthetic sources'], 'deferred': ['All unresolved'],
                                   'restored': [], 'unresolved': c.unresolved(self.failed_report.decode())},
                   'verification': {'increment_id': repair.MAINTENANCE, 'report_path': repair.MAINTENANCE_REPORT}}
        self.f.write_json('.codex/state/repair-close.json', request)
        c.close(self.root, '.codex/state/repair-close.json')
        self.f.write(repair.PLAN, '# Synthetic repair plan\n')
        temp = self.f.review(repair.READINESS_ID)
        self.f.write(repair.READINESS, (self.root / temp).read_text())
        m = g._extract_manifest((self.root / repair.READINESS).read_text())
        m['files_changed'] = list(g.changed_paths(self.root))
        self.f.write_review(repair.READINESS, m)
        old = repair.history(self.root)
        self.req = {'schema_version': 1, 'increment_id': repair.SUCCESSOR, 'root': str(self.root),
                    'branch': v.previous.BRANCH, 'head': g.current_head_commit(self.root),
                    'index_sha256': repair.INDEX_SHA256, 'workspace_fingerprint': g.workspace_fingerprint(self.root),
                    'failed_state_sha256': repair.FAILED_STATE, 'failed_report_sha256': repair.FAILED_REPORT,
                    'closure_id': repair.FAILED_STATE, 'maintenance_report': repair.MAINTENANCE_REPORT,
                    'maintenance_report_sha256': c.digest((self.root / repair.MAINTENANCE_REPORT).read_bytes()),
                    'readiness_report': repair.READINESS, 'readiness_report_sha256': c.digest((self.root / repair.READINESS).read_bytes()),
                    'readiness_increment': repair.READINESS_ID,
                    'owner_authorization': {'admission': self.f.auth(), 'execution': self.f.auth()},
                    'plan': repair.PLAN, 'report': repair.REPORT, 'milestone': self.f.milestone,
                    'allowed_paths': [repair.REPORT], 'protected_source_bindings': v.previous.PRODUCT,
                    'original_obligations': old['prior']['previous']['request']['obligations'],
                    'recovery_obligations': old['prior']['unresolved'], 'v_obligations': old['unresolved'],
                    'historical_bindings': old['bindings']}
        self.path = '.codex/state/repair-request.json'

    def admit(self):
        self.f.write_json(self.path, self.req)
        g.begin_gate(self.root, repair.SUCCESSOR, dashboard_verification_repair_request=self.path)

    def reject(self):
        with self.assertRaises(g.GateError): self.admit()
        self.assertEqual(g.state_path(self.root).read_bytes(), self.raw)
        self.assertFalse((self.root / repair.POINTER).exists())

    def final(self, failed=False):
        self.f.write(repair.REPORT, 'pending\n')
        checks = repair.required_checks(repair.history(self.root))
        m = {'schema_version': 1, 'increment_id': repair.SUCCESSOR,
             'quality_gate': 'FAIL' if failed else 'PASS WITH ADVISORIES', 'next_increment_readiness': 'Blocked',
             'files_changed': list(g.changed_paths(self.root)), 'commands_executed': sorted(checks['verification']),
             'verification': [{'command': n, 'required': True, 'status': 'Failed' if failed and n == 'npm run verify' else 'Passed'} for n in sorted(checks['verification'])],
             'manual_verification': [{'check': n, 'required': True, 'status': 'Passed'} for n in sorted(checks['manual_verification'])],
             'findings': [self.v.advisory],
             'milestone': {'criteria': {n: {'status': 'automatically_verified', 'evidence': 'Synthetic only'} for n in self.f.acceptance},
                           'paths': {repair.REPORT: {'criterion': 'verification', 'rationale': 'Synthetic final report', 'within_objective': True, 'preserves_existing': True}}}}
        current = c.manifest(self.root); current.pop(repair.REPORT)
        evidence = {'verification': {}, 'manual_verification': {}}
        for kind, key in [('verification', 'command'), ('manual_verification', 'check')]:
            for row in m[kind]:
                name = row[key]
                path = '.codex/state/dashboard-verification-repair-evidence/' + c.digest((kind + name).encode()) + '.json'
                proof = {'name': name, 'status': row['status'], 'input_bindings': current,
                         'environment': 'Synthetic temporary repository only', 'result': 'Synthetic evidence', 'scope_review': 'Synthetic bindings'}
                self.f.write_json(path, proof)
                evidence[kind][name] = {'path': path, 'sha256': c.digest((self.root / path).read_bytes()), 'reuse': False, 'review': 'Synthetic review'}
        self.write_final(m, evidence)
        return m, evidence

    def write_final(self, m, evidence):
        self.f.write_review(repair.REPORT, m)
        text = (self.root / repair.REPORT).read_text()
        self.f.write(repair.REPORT, text + '\n' + repair.EVIDENCE_START + json.dumps(evidence) + repair.EVIDENCE_END + '\n')

    def test_success_preserves_failure_and_stop(self):
        self.admit(); self.final()
        g.finalize_gate(self.root, repair.SUCCESSOR, repair.REPORT)
        self.assertTrue(g.validate_completed_state(self.root, g.read_state(self.root)))
        self.assertEqual((self.root / v.REPORT).read_bytes(), self.failed_report)
        self.assertEqual((self.root / v.POINTER).read_bytes(), self.pointer)
        self.assertEqual(repair.record(self.root, g.read_state(self.root))['raw_state'].encode(), self.raw)
        self.assertFalse(g.evaluate_stop_payload({'cwd': str(self.root), 'hook_event_name': 'Stop', 'stop_hook_active': False}).should_continue)

    def test_failure_immutable_no_false_completion_or_restart(self):
        self.admit(); self.final(True)
        g.close_failed_gate(self.root, repair.SUCCESSOR, repair.REPORT)
        self.assertTrue(g.validate_failed_state(self.root, g.read_state(self.root)))
        for op in [lambda: self.admit(), lambda: g.begin_gate(self.root, 'other'),
                   lambda: g.finalize_gate(self.root, repair.SUCCESSOR, repair.REPORT),
                   lambda: g.close_failed_gate(self.root, repair.SUCCESSOR, repair.REPORT)]:
            with self.assertRaises(g.GateError): op()
        state = g.read_state(self.root)
        state.pop('head_commit'); state.pop('next_increment_readiness')
        state.update(status='complete', quality_gate='PASS WITH ADVISORIES', completion_marker=g.COMPLETION_MARKER)
        self.assertFalse(g.validate_completed_state(self.root, state))

    def test_request_identity_and_criteria_omissions(self):
        baseline = copy.deepcopy(self.req)
        for key in ['head', 'root', 'branch', 'index_sha256', 'workspace_fingerprint', 'failed_state_sha256', 'failed_report_sha256', 'closure_id', 'plan', 'report', 'maintenance_report', 'maintenance_report_sha256', 'readiness_report_sha256']:
            self.req = copy.deepcopy(baseline); self.req[key] = '0'*64; self.reject()
        for key in ['original_obligations', 'recovery_obligations', 'v_obligations', 'historical_bindings', 'protected_source_bindings']:
            self.req = copy.deepcopy(baseline); self.req[key].pop(next(iter(self.req[key]))); self.reject()
        self.req = copy.deepcopy(baseline); self.req['milestone']['acceptance']['verification'] = 'waived'; self.reject()

    def test_scope_and_authority(self):
        baseline = copy.deepcopy(self.req)
        for path in [repair.PLAN, repair.READINESS, 'DECISIONS.md', '.prettierignore', 'src/extra.txt']:
            self.req = copy.deepcopy(baseline); self.req['allowed_paths'].append(path); self.reject()
        for lane in ['admission', 'execution']:
            self.req = copy.deepcopy(baseline); self.req['owner_authorization'][lane]['approved'] = False; self.reject()

    def test_historical_fail_missing_drifted_and_replaced(self):
        p = self.root / v.REPORT
        p.unlink(); self.reject()
        for raw in [self.failed_report + b'\ndrift\n', (self.root / repair.MAINTENANCE_REPORT).read_bytes()]:
            p.write_bytes(raw); self.reject()
        p.write_bytes(self.failed_report)
        self.f.write('.prettierignore', '# no historical exception\n'); self.reject()

    def test_preparation_product_history_and_index_frozen(self):
        self.admit()
        for name in [repair.PLAN, repair.READINESS, repair.MAINTENANCE_REPORT, v.REPORT, 'DECISIONS.md', '.prettierignore', next(iter(v.previous.PRODUCT))]:
            p = self.root / name; raw = p.read_bytes(); p.write_bytes(raw+b'\ndrift\n')
            with self.assertRaises(g.GateError): repair.record(self.root, g.read_state(self.root))
            p.write_bytes(raw)

    def test_inherited_checks_not_omitted_or_optional(self):
        self.admit(); m, evidence = self.final()
        for kind in ['verification', 'manual_verification']:
            for i in range(len(m[kind])):
                for omitted in [True, False]:
                    bad = copy.deepcopy(m)
                    if omitted: bad[kind].pop(i)
                    else: bad[kind][i]['required'] = False
                    self.write_final(bad, evidence)
                    with self.assertRaises(g.GateError): g.validate_report(self.root, repair.REPORT, repair.SUCCESSOR)

    def test_publication_advisory_and_terminal_evidence_drift(self):
        self.admit(); m, ev = self.final()
        bad = copy.deepcopy(m); bad['findings'] = []
        self.write_final(bad, ev)
        with self.assertRaises(g.GateError): g.validate_report(self.root, repair.REPORT, repair.SUCCESSOR)
        self.write_final(m, ev); g.finalize_gate(self.root, repair.SUCCESSOR, repair.REPORT)
        p = self.root / next(iter(ev['verification'].values()))['path']; p.write_bytes(p.read_bytes()+b' ')
        self.assertFalse(g.validate_completed_state(self.root, g.read_state(self.root)))

    def test_interrupted_effects_are_consumed(self):
        for effect in [1, 2, 3]:
            case = DashboardRepairTests(methodName='runTest')
            try:
                case.setUp(); original = repair.exclusive; calls = [0]
                def interrupted(root, name, data):
                    original(root, name, data); calls[0] += 1
                    if calls[0] == effect: raise OSError('Synthetic interrupted effect')
                with mock.patch.object(repair, 'exclusive', side_effect=interrupted), mock.patch.object(g, 'write_state', side_effect=OSError('Synthetic interrupted state')):
                    with self.assertRaises(g.GateError): case.admit()
                self.assertEqual(g.state_path(case.root).read_bytes(), case.raw)
                with self.assertRaises(g.GateError): case.admit()
            finally: case.doCleanups()

    def test_post_write_interruption_and_mixed_flags(self):
        original = g.write_state
        def interrupted(root, state):
            original(root, state); raise OSError('Synthetic uncertain write')
        with mock.patch.object(g, 'write_state', side_effect=interrupted):
            with self.assertRaises(g.GateError): self.admit()
        self.assertEqual(g.read_state(self.root)['status'], 'active')
        with self.assertRaises(g.GateError): self.admit()
        with self.assertRaises(g.GateError):
            g.begin_gate(self.root, repair.SUCCESSOR, admission=self.path, dashboard_verification_repair_request=self.path)

    def test_own_closure_required_and_wrong_passing_report_rejected(self):
        p = c.receipt_path(self.root, repair.FAILED_STATE); p.rename(p.with_suffix('.retained')); self.reject()

    def test_unrelated_execution_change_blocks(self):
        self.admit(); self.f.write('unrelated.md', 'Not authorized\n')
        with self.assertRaises(g.GateError): repair.record(self.root, g.read_state(self.root))

    def test_extra_formatting_exclusion_and_index_drift_block(self):
        p = self.root / '.prettierignore'; raw = p.read_bytes()
        p.write_bytes(raw+b'*.md\n'); self.reject(); p.write_bytes(raw)
        index = c.git_index_path(self.root); raw = index.read_bytes()
        index.write_bytes(raw+b'changed\n'); self.reject(); index.write_bytes(raw)

    def test_preparation_race_after_pointer_blocks_replacement(self):
        original = repair.exclusive
        def mutate(root, name, data):
            original(root, name, data)
            if name == repair.POINTER: self.f.write(repair.READINESS, 'Synthetic race\n')
        with mock.patch.object(repair, 'exclusive', side_effect=mutate):
            with self.assertRaises(g.GateError): self.admit()
        self.assertEqual(g.state_path(self.root).read_bytes(), self.raw)
        self.assertTrue((self.root / repair.POINTER).exists())
        with self.assertRaises(g.GateError): self.admit()

    def test_wrong_or_nonpassing_maintenance_cannot_close(self):
        p = c.receipt_path(self.root, repair.FAILED_STATE)
        receipt = json.loads(p.read_bytes())
        receipt['payload']['request']['verification']['report_path'] = v.MAINTENANCE_REPORT
        receipt['sha256'] = c.digest(c.encoded(receipt['payload']))
        p.write_bytes(c.encoded(receipt)); self.reject()


if __name__ == '__main__':
    unittest.main()
