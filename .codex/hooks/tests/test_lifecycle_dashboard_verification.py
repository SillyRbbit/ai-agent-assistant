"""Temporary Git repositories only; no real admission, closure or provider access."""
from __future__ import annotations

import copy
import json
import shutil
import sys
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
sys.path.insert(0, str(Path(__file__).resolve().parent))
import lifecycle_closure as closure
import lifecycle_dashboard_publication as previous
import lifecycle_dashboard_verification as verification
import post_increment_gate as gate
import test_lifecycle_dashboard_publication as predecessor_fixture


class DashboardVerificationTests(unittest.TestCase):
    def setUp(self):
        f = predecessor_fixture.DashboardPublicationTests(methodName="runTest")
        self.addCleanup(f.doCleanups)
        f.setUp()
        self.f, self.root = f, f.root
        f.admit()
        report = f.final(failed=True)
        failed = gate._extract_manifest((self.root / report).read_text())
        self.advisory = {"category": "Technical debt", "severity": "Advisory", "summary": "Synthetic post-commit blocker",
            "risk": "Fixed HEAD/index", "effort": "Separate owner review", "milestone": "Publication",
            "blocks_completion": False, "blocks_next_increment": True}
        failed["findings"].append(self.advisory)
        f.write_review(report, failed)
        gate.close_failed_gate(self.root, previous.SUCCESSOR, report)
        self.raw = gate.state_path(self.root).read_bytes()
        state = gate.read_state(self.root)
        self.old_pointer = (self.root / previous.POINTER).read_bytes()
        before = closure.manifest(self.root)
        backup = self.root / ".codex/state/second-backup"
        for name in before:
            dest = backup / "before" / name
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(self.root / name, dest)
        for name, source in [(".git/index", closure.git_index_path(self.root)),
                             (gate.STATE_RELATIVE_PATH.as_posix(), gate.state_path(self.root))]:
            dest = backup / "before" / name
            dest.parent.mkdir(parents=True, exist_ok=True); shutil.copy2(source, dest)
        f.write_json(".codex/state/second-backup/workspace-before.json", before)
        for name, value in {"FAILED_STATE": closure.digest(self.raw), "FAILED_REPORT": state["report_sha256"],
                "PREVIOUS_ADMISSION": state["dashboard_publication_lineage"]["admission"],
                "INDEX_SHA256": closure.digest(closure.read_bytes(closure.git_index_path(self.root)))}.items():
            patcher = mock.patch.object(verification, name, value)
            patcher.start(); self.addCleanup(patcher.stop)
        # A new current decision must not be passed to the old schema-6 validator.
        f.write("DECISIONS.md", "Synthetic M then separate C/V approvals, distinct from historical authority.\n")
        old_maintenance = f.review(verification.CONTINUATION)
        original_m = gate._extract_manifest((self.root / old_maintenance).read_text())
        original_m.update(increment_id=verification.MAINTENANCE, quality_gate="FAIL",
                          next_increment_readiness="Blocked")
        original_m["verification"][0]["status"] = "Failed"
        f.write_review(verification.ORIGINAL_MAINTENANCE_REPORT, original_m)
        self.original_m = (self.root / verification.ORIGINAL_MAINTENANCE_REPORT).read_bytes()
        patcher = mock.patch.object(verification, "ORIGINAL_MAINTENANCE_SHA256", closure.digest(self.original_m))
        patcher.start(); self.addCleanup(patcher.stop)
        f.write(verification.MAINTENANCE_REPORT, (self.root / old_maintenance).read_text())
        m = gate._extract_manifest((self.root / verification.MAINTENANCE_REPORT).read_text())
        m["files_changed"] = list(gate.changed_paths(self.root))
        f.write_review(verification.MAINTENANCE_REPORT, m)
        request = {"schema_version": 1, "owner_authorization": f.auth(),
            "failed_state_sha256": verification.FAILED_STATE, "backup_directory": str(backup),
            "manifest_sha256": closure.digest((backup / "workspace-before.json").read_bytes()),
            "allowed_paths": ["DECISIONS.md", old_maintenance, verification.ORIGINAL_MAINTENANCE_REPORT,
                              verification.MAINTENANCE_REPORT],
            "disposition": {"retained": ["All synthetic sources"], "deferred": ["All unresolved items"],
                "restored": [], "unresolved": closure.unresolved((self.root / report).read_text())},
            "verification": {"increment_id": verification.CONTINUATION, "report_path": verification.MAINTENANCE_REPORT}}
        f.write_json(".codex/state/second-close.json", request)
        closure.close(self.root, ".codex/state/second-close.json")
        f.write(verification.PLAN, "# Synthetic fixed verification plan\n")
        ready = f.review(verification.READINESS_ID)
        f.write(verification.READINESS, (self.root / ready).read_text())
        m = gate._extract_manifest((self.root / verification.READINESS).read_text())
        m["files_changed"] = list(gate.changed_paths(self.root))
        f.write_review(verification.READINESS, m)
        old = verification.history(self.root)
        self.request = {"schema_version": 1, "increment_id": verification.SUCCESSOR, "root": str(self.root),
            "branch": previous.BRANCH, "head": gate.current_head_commit(self.root),
            "index_sha256": verification.INDEX_SHA256, "workspace_fingerprint": gate.workspace_fingerprint(self.root),
            "failed_state_sha256": verification.FAILED_STATE, "failed_report_sha256": verification.FAILED_REPORT,
            "closure_id": verification.FAILED_STATE, "maintenance_report": verification.MAINTENANCE_REPORT,
            "maintenance_report_sha256": closure.digest((self.root / verification.MAINTENANCE_REPORT).read_bytes()),
            "readiness_report": verification.READINESS,
            "readiness_report_sha256": closure.digest((self.root / verification.READINESS).read_bytes()),
            "readiness_increment": verification.READINESS_ID, "owner_authorization": {"admission": f.auth(), "execution": f.auth()},
            "plan": verification.PLAN, "report": verification.REPORT, "milestone": f.milestone,
            "allowed_paths": [verification.REPORT], "protected_source_bindings": previous.PRODUCT,
            "original_obligations": old["previous"]["request"]["obligations"],
            "recovery_obligations": old["unresolved"], "historical_bindings": old["bindings"]}
        self.path = ".codex/state/verification-request.json"
        self.save()

    def save(self): self.f.write_json(self.path, self.request)

    def admit(self):
        self.save()
        gate.begin_gate(self.root, verification.SUCCESSOR, dashboard_verification_request=self.path)

    def reject(self, pattern="verification|closure"):
        self.save()
        with self.assertRaisesRegex(gate.GateError, pattern):
            gate.begin_gate(self.root, verification.SUCCESSOR, dashboard_verification_request=self.path)
        self.assertEqual(gate.state_path(self.root).read_bytes(), self.raw)
        self.assertFalse((self.root / verification.POINTER).exists())

    def final(self, *, failed=False):
        self.f.write(verification.REPORT, "pending\n")
        old = verification.history(self.root)
        checks = verification.required_checks(old)
        m = {"schema_version": 1, "increment_id": verification.SUCCESSOR,
            "quality_gate": "FAIL" if failed else "PASS WITH ADVISORIES", "next_increment_readiness": "Blocked",
            "files_changed": list(gate.changed_paths(self.root)), "commands_executed": sorted(checks["verification"]),
            "verification": [{"command": n, "required": True, "status": "Failed" if failed and n == "npm run verify" else "Passed"}
                             for n in sorted(checks["verification"])],
            "manual_verification": [{"check": n, "required": True, "status": "Passed"} for n in sorted(checks["manual_verification"])],
            "findings": [self.advisory],
            "milestone": {"criteria": {n: {"status": "automatically_verified", "evidence": "Synthetic only"}
                                       for n in self.f.acceptance},
                "paths": {verification.REPORT: {"criterion": "verification", "rationale": "Synthetic final report",
                    "within_objective": True, "preserves_existing": True}}}}
        current = closure.manifest(self.root); current.pop(verification.REPORT)
        evidence = {"verification": {}, "manual_verification": {}}
        for kind, key in [("verification", "command"), ("manual_verification", "check")]:
            for row in m[kind]:
                name = row[key]
                path = ".codex/state/dashboard-verification-evidence/" + closure.digest((kind + name).encode()) + ".json"
                proof = {"name": name, "status": row["status"], "input_bindings": current,
                    "environment": "Synthetic temporary repository; no real command success claimed",
                    "result": "Synthetic classification only", "scope_review": "Synthetic complete bindings"}
                self.f.write_json(path, proof)
                evidence[kind][name] = {"path": path, "sha256": closure.digest((self.root / path).read_bytes()),
                    "reuse": False, "review": "Synthetic named review"}
        self.write_final(m, evidence)
        return m, evidence

    def write_final(self, m, evidence):
        self.f.write_review(verification.REPORT, m)
        text = (self.root / verification.REPORT).read_text()
        self.f.write(verification.REPORT, text + "\n" + verification.EVIDENCE_START + json.dumps(evidence) + verification.EVIDENCE_END + "\n")

    def test_success_preserves_history_and_publication_blocker(self):
        self.admit(); self.final()
        gate.finalize_gate(self.root, verification.SUCCESSOR, verification.REPORT)
        state = gate.read_state(self.root)
        self.assertTrue(gate.validate_completed_state(self.root, state))
        self.assertEqual(state["quality_gate"], "PASS WITH ADVISORIES")
        self.assertEqual(verification.record(self.root, state)["raw_state"].encode(), self.raw)
        self.assertEqual((self.root / previous.POINTER).read_bytes(), self.old_pointer)
        self.assertEqual((self.root / verification.ORIGINAL_MAINTENANCE_REPORT).read_bytes(), self.original_m)
        self.assertEqual(verification.history(self.root)["bindings"]["maintenance_failure"], closure.digest(self.original_m))
        self.assertFalse(gate.evaluate_stop_payload({"cwd": str(self.root), "hook_event_name": "Stop", "stop_hook_active": False}).should_continue)

    def test_failed_terminal_and_no_restart(self):
        self.admit(); self.final(failed=True)
        gate.close_failed_gate(self.root, verification.SUCCESSOR, verification.REPORT)
        state = gate.read_state(self.root)
        self.assertTrue(gate.validate_failed_state(self.root, state))
        self.assertNotIn("completion_marker", state)
        with self.assertRaises(gate.GateError): gate.finalize_gate(self.root, verification.SUCCESSOR, verification.REPORT)
        with self.assertRaises(gate.GateError): gate.close_failed_gate(self.root, verification.SUCCESSOR, verification.REPORT)
        with self.assertRaises(gate.GateError): gate.begin_gate(self.root, "other")
        with self.assertRaises(gate.GateError): self.admit()

    def test_identity_drift_and_unknown_fields(self):
        baseline = copy.deepcopy(self.request)
        for key in ["root", "branch", "head", "index_sha256", "workspace_fingerprint", "failed_state_sha256",
                    "failed_report_sha256", "closure_id", "readiness_report_sha256", "maintenance_report_sha256",
                    "plan", "report", "increment_id", "readiness_increment"]:
            with self.subTest(key=key):
                self.request = copy.deepcopy(baseline); self.request[key] = "0" * 64; self.reject()
        self.request = copy.deepcopy(baseline); self.request["extra"] = True; self.reject()

    def test_obligations_criteria_scope_and_separate_approval(self):
        baseline = copy.deepcopy(self.request)
        for key in ["original_obligations", "recovery_obligations", "protected_source_bindings", "historical_bindings"]:
            self.request = copy.deepcopy(baseline); self.request[key].pop(next(iter(self.request[key]))); self.reject()
        self.request = copy.deepcopy(baseline); self.request["milestone"]["acceptance"]["verification"] = "waived"; self.reject()
        for path in [verification.PLAN, verification.READINESS, "DECISIONS.md", "src/extra.txt", ".codex/hooks/post_increment_gate.py"]:
            self.request = copy.deepcopy(baseline); self.request["allowed_paths"].append(path); self.reject()
        for lane in ["admission", "execution"]:
            self.request = copy.deepcopy(baseline); self.request["owner_authorization"][lane]["approved"] = False; self.reject()

    def test_own_closure_required(self):
        path = closure.receipt_path(self.root, verification.FAILED_STATE)
        path.rename(path.with_suffix(".retained"))
        self.reject()

    def test_historical_receipt_and_consumed_drift(self):
        path = self.root / previous.POINTER
        raw = path.read_bytes()
        self.f.write_json(previous.POINTER.as_posix(), {"admission": "0"*64}); self.reject()
        path.write_bytes(raw)
        path = self.root / previous.STORE / (verification.PREVIOUS_ADMISSION + ".json")
        path.write_bytes(path.read_bytes() + b" "); self.reject()

    def test_strict_json_size_duplicates_nonfinite_and_symlink(self):
        for raw in [b'{"x":1,"x":2}', b'{"x":NaN}', b" "*(closure.MAX_BYTES+1)]:
            with self.assertRaises((gate.GateError, ValueError)): verification.parse(raw)
        path = self.root / self.path; path.unlink(); path.symlink_to(self.root / "DECISIONS.md")
        with self.assertRaises(gate.GateError):
            gate.begin_gate(self.root, verification.SUCCESSOR, dashboard_verification_request=self.path)

    def test_interruption_each_effect_consumes_no_retry(self):
        # Each test instance is one synthetic attempt; iterate effects in isolated fixtures.
        for effect in [1, 2, 3]:
            with self.subTest(effect=effect):
                case = DashboardVerificationTests(methodName="runTest")
                try:
                    case.setUp()
                    original = verification.exclusive
                    calls = [0]
                    def interrupt(root, name, data):
                        original(root, name, data); calls[0] += 1
                        if calls[0] == effect: raise OSError("synthetic interrupted effect")
                    with mock.patch.object(verification, "exclusive", side_effect=interrupt), \
                         mock.patch.object(gate, "write_state", side_effect=OSError("synthetic interrupted state")):
                        with self.assertRaises(gate.GateError): case.admit()
                    self.assertEqual(gate.state_path(case.root).read_bytes(), case.raw)
                    with self.assertRaises(gate.GateError): case.admit()
                finally: case.doCleanups()

    def test_race_between_effects(self):
        original = verification.exclusive
        def mutate(root, name, data):
            original(root, name, data)
            self.f.write("DECISIONS.md", "changed during synthetic admission")
        with mock.patch.object(verification, "exclusive", side_effect=mutate):
            with self.assertRaises(gate.GateError): self.admit()
        self.assertEqual(gate.state_path(self.root).read_bytes(), self.raw)
        with self.assertRaises(gate.GateError): self.admit()

    def test_complete_inherited_required_checks_cannot_be_omitted_or_optional(self):
        self.admit(); m, evidence = self.final()
        baseline = copy.deepcopy(m)
        for kind in ["verification", "manual_verification"]:
            for i in range(len(baseline[kind])):
                for omit in [True, False]:
                    with self.subTest(kind=kind, index=i, omit=omit):
                        m = copy.deepcopy(baseline)
                        if omit: m[kind].pop(i)
                        else: m[kind][i]["required"] = False
                        self.write_final(m, evidence)
                        with self.assertRaises(gate.GateError): gate.validate_report(self.root, verification.REPORT, verification.SUCCESSOR)

    def test_preparation_and_product_frozen_during_execution(self):
        self.admit()
        for path in [verification.PLAN, verification.READINESS, verification.ORIGINAL_MAINTENANCE_REPORT,
                     verification.MAINTENANCE_REPORT, "DECISIONS.md", next(iter(previous.PRODUCT))]:
            p = self.root / path; raw = p.read_bytes(); p.write_bytes(raw+b"\ndrift\n")
            with self.assertRaises(gate.GateError): verification.record(self.root, gate.read_state(self.root))
            p.write_bytes(raw)

    def test_provenance_must_bind_inputs_outcome_and_review(self):
        self.admit(); m, evidence = self.final()
        entry = evidence["verification"]["npm run verify"]
        p = self.root / entry["path"]; proof = json.loads(p.read_text())
        for key, value in [("status", "Failed"), ("environment", ""), ("input_bindings", {}),
                           ("scope_review", ""), ("result", "")]:
            bad = copy.deepcopy(proof); bad[key] = value
            self.f.write_json(entry["path"], bad); entry["sha256"] = closure.digest(p.read_bytes())
            self.write_final(m, evidence)
            with self.assertRaises(gate.GateError): gate.validate_report(self.root, verification.REPORT, verification.SUCCESSOR)

    def test_post_commit_advisory_cannot_disappear_or_be_resolved(self):
        self.admit(); m, evidence = self.final()
        for change in ["missing", "resolved", "ready"]:
            bad = copy.deepcopy(m)
            if change == "missing": bad["findings"] = []
            elif change == "resolved": bad["findings"][0]["blocks_next_increment"] = False
            else: bad["next_increment_readiness"] = "Ready"
            self.write_final(bad, evidence)
            with self.assertRaises(gate.GateError): gate.validate_report(self.root, verification.REPORT, verification.SUCCESSOR)

    def test_terminal_forward_evidence_drift_and_false_completion(self):
        self.admit(); _, ev = self.final()
        gate.finalize_gate(self.root, verification.SUCCESSOR, verification.REPORT)
        state = gate.read_state(self.root)
        self.assertTrue(gate.validate_completed_state(self.root, state))
        path = self.root / next(iter(ev["verification"].values()))["path"]
        path.write_bytes(path.read_bytes()+b" ")
        self.assertFalse(gate.validate_completed_state(self.root, state))

    def test_mixed_lineage_and_admission_flags(self):
        with self.assertRaises(gate.GateError):
            gate.begin_gate(self.root, verification.SUCCESSOR, admission=self.path, dashboard_verification_request=self.path)
        self.admit(); state = gate.read_state(self.root)
        state["dashboard_publication_lineage"] = {"admission": "0"*64}
        with self.assertRaises(gate.GateError): gate.validate_state(state)

    def test_state_write_then_interruption_does_not_allow_replacement(self):
        original = gate.write_state
        def interrupted(root, state):
            original(root, state)
            raise OSError("synthetic uncertainty after state write")
        with mock.patch.object(gate, "write_state", side_effect=interrupted):
            with self.assertRaises(gate.GateError): self.admit()
        self.assertEqual(gate.read_state(self.root)["status"], "active")
        with self.assertRaises(gate.GateError): self.admit()
        self.assertTrue(gate.evaluate_stop_payload({"cwd": str(self.root), "hook_event_name": "Stop", "stop_hook_active": False}).should_continue)

    def test_failed_evidence_cannot_be_forged_into_complete_state(self):
        self.admit(); self.final(failed=True)
        gate.close_failed_gate(self.root, verification.SUCCESSOR, verification.REPORT)
        state = gate.read_state(self.root)
        state.pop("head_commit"); state.pop("next_increment_readiness")
        state.update(status="complete", quality_gate="PASS WITH ADVISORIES", completion_marker=gate.COMPLETION_MARKER)
        gate.write_state(self.root, state)
        self.assertFalse(gate.validate_completed_state(self.root, state))

    def test_unapproved_document_fails_closed(self):
        self.admit()
        self.f.write("new-unrelated.md", "synthetic unapproved file")
        with self.assertRaises(gate.GateError): verification.record(self.root, gate.read_state(self.root))

    def test_readiness_race_after_pointer_preserves_interruption(self):
        original = verification.exclusive
        def mutate(root, name, data):
            original(root, name, data)
            if name == verification.POINTER:
                self.f.write(verification.READINESS, "synthetic readiness race")
        with mock.patch.object(verification, "exclusive", side_effect=mutate):
            with self.assertRaises(gate.GateError): self.admit()
        self.assertEqual(gate.state_path(self.root).read_bytes(), self.raw)
        self.assertTrue((self.root / verification.POINTER).exists())
        with self.assertRaises(gate.GateError): self.admit()

    def test_preserved_m_failure_missing_drifted_or_replaced_blocks_admission(self):
        p = self.root / verification.ORIGINAL_MAINTENANCE_REPORT
        p.unlink()
        self.reject()
        for data in [self.original_m + b"\ndrift\n",
                     (self.root / verification.MAINTENANCE_REPORT).read_bytes()]:
            p.write_bytes(data)
            self.reject()
        p.write_bytes(self.original_m)

    def test_only_fixed_continuation_can_bind_closure_and_admission(self):
        baseline = copy.deepcopy(self.request)
        for path in [verification.ORIGINAL_MAINTENANCE_REPORT,
                     "docs/reviews/2026-10-21-dashboard-verification-binding-preparation-review.md"]:
            self.request = copy.deepcopy(baseline)
            self.request["maintenance_report"] = path
            self.reject()
        self.request = baseline
        p = closure.receipt_path(self.root, verification.FAILED_STATE)
        receipt = json.loads(p.read_text())
        receipt["payload"]["request"]["verification"]["increment_id"] = verification.MAINTENANCE
        receipt["sha256"] = closure.digest(closure.encoded(receipt["payload"]))
        p.write_bytes(closure.encoded(receipt))
        self.reject()

    def test_missing_or_changed_continuation_cannot_be_admitted(self):
        p = self.root / verification.MAINTENANCE_REPORT
        raw = p.read_bytes()
        p.unlink()
        self.reject()
        p.write_bytes(raw + b"\nchanged\n")
        self.reject()

    def test_nonpassing_continuation_cannot_supply_own_closure(self):
        # Remove only this synthetic receipt to exercise the pre-closure check.
        closure.receipt_path(self.root, verification.FAILED_STATE).unlink()
        p = self.root / verification.MAINTENANCE_REPORT
        m = gate._extract_manifest(p.read_text())
        m.update(quality_gate="FAIL", next_increment_readiness="Blocked")
        for status in ["Failed", "Not run"]:
            for row in m["verification"]:
                if row["command"] == "npm run verify": row["status"] = status
            self.f.write_review(verification.MAINTENANCE_REPORT, m)
            with self.assertRaises(gate.GateError):
                closure.close(self.root, ".codex/state/second-close.json")
            self.assertFalse(closure.receipt_path(self.root, verification.FAILED_STATE).exists())


if __name__ == "__main__":
    unittest.main()
