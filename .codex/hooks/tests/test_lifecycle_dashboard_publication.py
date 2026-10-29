from __future__ import annotations

import copy
import json
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import lifecycle_closure as closure
import lifecycle_dashboard_publication as dashboard
import post_increment_gate as gate


class DashboardPublicationTests(unittest.TestCase):
    """All state effects are synthetic; no real closure or admission is executed."""

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name).resolve()
        for args in [("init", "-q"), ("config", "user.email", "fixture@example.invalid"),
                     ("config", "user.name", "Synthetic fixture"), ("branch", "-m", dashboard.BRANCH)]:
            self.git(*args)
        self.write(".gitignore", ".codex/state/\n__pycache__/\n")
        self.write("DECISIONS.md", "Synthetic closure and separate admission/execution approval.\n")
        self.write("src/protected.txt", "Unrelated protected work\n")
        self.git("add", "."); self.git("commit", "-qm", "Synthetic baseline")
        self.acceptance = {"source": "Exact source", "disclosure": "Safe docs", "preservation": "Preserve", "verification": "Verify all"}
        self.milestone = {"objective": "Synthetic bounded dashboard", "exclusions": ["Live data and publication"],
            "acceptance": self.acceptance, "protected_paths": ["DECISIONS.md", "src/protected.txt"], "authorized_destructive_paths": []}
        ready = self.review(dashboard.PREDECESSOR + "-readiness")
        old = {"schema_version": 2, "increment_id": dashboard.PREDECESSOR, "owner_authorization": self.auth(),
               "workspace_fingerprint": gate.workspace_fingerprint(self.root), "dependencies": {},
               "readiness_report": ready, "readiness_increment": dashboard.PREDECESSOR + "-readiness",
               "milestone": self.milestone}
        self.write_json(".codex/state/old-request.json", old)
        gate.begin_gate(self.root, dashboard.PREDECESSOR, admission=".codex/state/old-request.json")
        admitted = gate.read_state(self.root)
        self.milestone = closure.read_json(self.root / closure.DIRECTORY /
            ("admission-" + admitted["closure_lineage"]["admission"] + ".json"))["request"]["milestone"]
        self.old_report = self.review(dashboard.PREDECESSOR, failed=True, milestone=True)
        gate.close_failed_gate(self.root, dashboard.PREDECESSOR, self.old_report)
        self.raw = gate.state_path(self.root).read_bytes()
        original = (self.root / self.old_report).read_bytes()
        self.obligations = set(closure.unresolved(original.decode()))
        self.assertEqual(len(self.obligations), 9)
        self.original_inventory = closure.manifest(self.root)
        backup = self.root / ".codex/state/backup"
        for name in self.original_inventory:
            target = backup / "before" / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(self.root / name, target)
        for name, source in [(".git/index", closure.git_index_path(self.root)),
                             (gate.STATE_RELATIVE_PATH.as_posix(), gate.state_path(self.root))]:
            target = backup / "before" / name
            target.parent.mkdir(parents=True, exist_ok=True); shutil.copy2(source, target)
        self.write_json(".codex/state/backup/workspace-before.json", self.original_inventory)
        maintenance = self.review(dashboard.MAINTENANCE)
        self.closure_request = {"schema_version": 1, "owner_authorization": self.auth(),
            "failed_state_sha256": closure.digest(self.raw), "backup_directory": str(backup),
            "manifest_sha256": closure.digest((backup / "workspace-before.json").read_bytes()),
            "allowed_paths": [maintenance], "disposition": {"retained": ["Synthetic failed history"],
                "deferred": ["All unfinished work"], "restored": [], "unresolved": sorted(self.obligations)},
            "verification": {"increment_id": dashboard.MAINTENANCE, "report_path": maintenance}}
        self.write_json(".codex/state/closure-request.json", self.closure_request)
        closure.close(self.root, ".codex/state/closure-request.json")
        self.product = {"src/synthetic-dashboard.txt": closure.digest(b"Synthetic accepted dashboard\n")}
        # Only test-local pins change. There is no runtime override or test flag.
        for name, value in {"HEAD": gate.current_head_commit(self.root),
                            "ROOT_SHA256": closure.digest(str(self.root).encode()),
                            "FAILED_STATE": closure.digest(self.raw), "FAILED_REPORT": closure.digest(original),
                            "OBLIGATIONS": self.obligations, "PRODUCT": self.product}.items():
            patcher = mock.patch.object(dashboard, name, value); patcher.start(); self.addCleanup(patcher.stop)
        self.write(dashboard.PLAN, "# Synthetic recovery plan\n")
        self.review(dashboard.READINESS_ID)
        self.request = {"schema_version": 1, "increment_id": dashboard.SUCCESSOR, "root": str(self.root),
            "branch": dashboard.BRANCH, "head": gate.current_head_commit(self.root),
            "workspace_fingerprint": gate.workspace_fingerprint(self.root),
            "index_sha256": closure.digest(closure.read_bytes(closure.git_index_path(self.root))),
            "closure_id": dashboard.FAILED_STATE, "owner_authorization": {"admission": self.auth(), "execution": self.auth()},
            "milestone": self.milestone, "allowed_paths": sorted(set(self.product) | {dashboard.REPORT}),
            "plan": dashboard.PLAN, "report": dashboard.REPORT, "readiness_report": dashboard.READINESS,
            "readiness_increment": dashboard.READINESS_ID,
            "obligations": {n: {"criterion": "verification", "required": True} for n in sorted(self.obligations)}}
        self.request_path = ".codex/state/dashboard-request.json"
        self.save()

    def git(self, *args):
        return subprocess.run(["git", *args], cwd=self.root, check=True, capture_output=True)

    def write(self, name, text):
        path = self.root / name; path.parent.mkdir(parents=True, exist_ok=True); path.write_text(text)
        path.chmod(0o644)  # Declared synthetic baseline mode, independent of caller umask.

    def test_written_fixture_files_have_declared_mode(self):
        name = '.codex/state/explicit-mode-fixture.txt'
        self.write(name, 'Synthetic mode regression\n')
        self.assertEqual((self.root / name).stat().st_mode & 0o777, 0o644)

    def write_json(self, name, value): self.write(name, closure.encoded(value).decode())

    def auth(self):
        return {"approved": True, "source": "Explicit synthetic owner authority", "decision_path": "DECISIONS.md",
                "decision_sha256": closure.digest((self.root / "DECISIONS.md").read_bytes())}

    def review(self, increment, *, failed=False, milestone=False):
        name = "docs/reviews/2026-10-20-" + increment + "-post-increment-review.md"
        self.write(name, "pending\n")
        commands = sorted(closure.REQUIRED_COMMANDS)
        checks = sorted(closure.REQUIRED_REVIEWS) + ["Mock browser integration"]
        if increment == dashboard.SUCCESSOR:
            checks += ["Recovery obligation " + n for n in sorted(self.obligations)]
        m = {"schema_version": 1, "increment_id": increment, "quality_gate": "FAIL" if failed else "PASS",
             "next_increment_readiness": "Blocked" if failed else "Ready", "files_changed": list(gate.changed_paths(self.root)),
             "commands_executed": commands, "verification": [{"command": c, "required": True,
                 "status": "Failed" if failed and c in {"npm run docs:check", "npm run repository:check"} else "Passed"} for c in commands],
             "manual_verification": [{"check": c, "required": True, "status": "Failed" if failed
                 and c in {"Architecture, security, code-health and readiness review", "Mock browser integration"} else "Passed"} for c in checks],
             "findings": [{"category": "Roadmap", "severity": "High", "summary": "Synthetic failure " + str(i),
                 "risk": "Synthetic risk", "effort": "Bounded review", "milestone": "Recovery",
                 "blocks_completion": True, "blocks_next_increment": True} for i in range(2)] if failed else []}
        if milestone:
            state = gate.read_state(self.root)
            if increment == dashboard.SUCCESSOR:
                before = dashboard.record(self.root, state)["baseline_manifest"]
            else:
                before = closure.read_json(self.root / closure.DIRECTORY /
                    ("admission-" + state["closure_lineage"]["admission"] + ".json"))["baseline_manifest"]
            m["milestone"] = {"criteria": {c: {"status": "blocked" if failed and c != "preservation" else "automatically_verified",
                "evidence": "Synthetic evidence, not real acceptance"} for c in self.acceptance},
                "paths": {n: {"criterion": "verification", "rationale": "Synthetic bounded fixture",
                    "within_objective": True, "preserves_existing": True}
                    for n in closure.changed(before, closure.manifest(self.root))}}
        self.write_review(name, m)
        return name

    def write_review(self, name, manifest):
        self.write(name, "# Synthetic report\n\n" + gate.MANIFEST_START + json.dumps(manifest, indent=2) + gate.MANIFEST_END
                   + "\n\n" + "\n".join(s + "\n\nSynthetic only.\n" for s in gate.REQUIRED_REPORT_SECTIONS))

    def save(self): self.write_json(self.request_path, self.request)
    def admit(self):
        self.save(); gate.begin_gate(self.root, dashboard.SUCCESSOR, dashboard_publication_request=self.request_path)

    def reject(self, pattern):
        self.save()
        with self.assertRaisesRegex(gate.GateError, pattern):
            gate.begin_gate(self.root, dashboard.SUCCESSOR, dashboard_publication_request=self.request_path)
        self.assertEqual(gate.state_path(self.root).read_bytes(), self.raw)
        self.assertFalse((self.root / dashboard.POINTER).exists())

    def final(self, *, failed=False):
        self.write(next(iter(self.product)), "Synthetic accepted dashboard\n")
        return self.review(dashboard.SUCCESSOR, failed=failed, milestone=True)

    def test_success_preserves_failure_and_full_stop(self):
        self.admit(); report = self.final()
        gate.finalize_gate(self.root, dashboard.SUCCESSOR, report)
        state = gate.read_state(self.root)
        self.assertTrue(gate.validate_completed_state(self.root, state))
        rec = dashboard.record(self.root, state)
        self.assertEqual(rec["raw_state"].encode(), self.raw)
        self.assertNotIn("completion_marker", json.loads(rec["raw_state"]))
        self.assertFalse(gate.evaluate_stop_payload({"cwd": str(self.root), "hook_event_name": "Stop", "stop_hook_active": False}).should_continue)

    def test_failed_successor_retains_obligations_no_marker_no_retry(self):
        self.admit(); report = self.final(failed=True)
        gate.close_failed_gate(self.root, dashboard.SUCCESSOR, report)
        state = gate.read_state(self.root)
        self.assertTrue(gate.validate_failed_state(self.root, state))
        self.assertNotIn("completion_marker", state)
        with self.assertRaisesRegex(gate.GateError, "terminal failure is immutable"):
            gate.close_failed_gate(self.root, dashboard.SUCCESSOR, report)
        with self.assertRaisesRegex(gate.GateError, "cannot be replayed"):
            gate.begin_gate(self.root, "another-task")
        with self.assertRaisesRegex(gate.GateError, "terminally failed"):
            gate.finalize_gate(self.root, dashboard.SUCCESSOR, report)

    def test_all_identity_bindings_and_exact_task(self):
        for key, value, pattern in [("root", "/private/tmp/other", "checkout binding"), ("branch", "main", "checkout binding"),
            ("head", "0"*40, "HEAD drift"), ("index_sha256", "0"*64, "index drift"),
            ("workspace_fingerprint", "0"*64, "input drift"), ("closure_id", "0"*64, "request identity"),
            ("schema_version", True, "request identity"), ("increment_id", "other", "request identity")]:
            with self.subTest(key=key):
                old = self.request[key]; self.request[key] = value; self.reject(pattern); self.request[key] = old
        with self.assertRaisesRegex(gate.GateError, "task-bound"):
            gate.begin_gate(self.root, "other", dashboard_publication_request=self.request_path)

    def test_separate_approvals_and_original_criteria_exclusions(self):
        for lane in ["admission", "execution"]:
            self.request["owner_authorization"][lane]["approved"] = False; self.reject("authorization required")
            self.request["owner_authorization"][lane] = self.auth()
        original = copy.deepcopy(self.request)
        for key, value in [("acceptance", {"new": "new"}), ("exclusions", []), ("authorized_destructive_paths", ["src/protected.txt"])]:
            self.request["milestone"][key] = value; self.reject("acceptance/exclusion|exclusions must be a non-empty list")
            self.request = copy.deepcopy(original)

    def test_all_nine_obligations_required(self):
        name = next(iter(self.request["obligations"]))
        value = self.request["obligations"].pop(name); self.reject("invalid record fields")
        self.request["obligations"][name] = value
        value["required"] = False; self.reject("omitted or waived")

    def test_missing_or_mutated_closure_and_maintenance_proof(self):
        p = closure.receipt_path(self.root, dashboard.FAILED_STATE); raw = p.read_bytes(); p.unlink()
        self.reject("unsafe or oversized evidence")
        p.write_bytes(raw)
        maintenance = self.closure_request["verification"]["report_path"]
        self.write(maintenance, "changed")
        self.reject("verification report changed")

    def test_stale_or_failed_readiness_rejected(self):
        old = self.request["readiness_report"]; self.request["readiness_report"] = self.old_report
        self.reject("fresh plan/report/readiness"); self.request["readiness_report"] = old
        m = gate._extract_manifest((self.root / dashboard.READINESS).read_text())
        m["verification"][0]["status"] = "Failed"; m["quality_gate"] = "FAIL"
        self.write_review(dashboard.READINESS, m)
        self.request["workspace_fingerprint"] = gate.workspace_fingerprint(self.root)
        self.reject("blocking evidence")

    def test_unknown_field_duplicate_and_oversized_json(self):
        self.request["override"] = True; self.reject("invalid record fields"); self.request.pop("override")
        self.save(); p = self.root / self.request_path
        p.write_text('{"schema_version":1,"schema_version":1}')
        with self.assertRaisesRegex(gate.GateError, "duplicate JSON key"):
            gate.begin_gate(self.root, dashboard.SUCCESSOR, dashboard_publication_request=self.request_path)
        p.write_bytes(b"x" * (closure.MAX_BYTES + 1))
        with self.assertRaisesRegex(gate.GateError, "oversized"):
            gate.begin_gate(self.root, dashboard.SUCCESSOR, dashboard_publication_request=self.request_path)

    def test_unsafe_request_and_store_symlinks(self):
        p = self.root / self.request_path; p.unlink(); p.symlink_to(self.root / "DECISIONS.md")
        with self.assertRaisesRegex(gate.GateError, "symlink"):
            gate.begin_gate(self.root, dashboard.SUCCESSOR, dashboard_publication_request=self.request_path)
        p.unlink(); self.save()
        (self.root / dashboard.STORE).symlink_to(self.root / ".codex/state")
        self.reject("interrupted admission")

    def test_scope_expansion_and_deletion_rejected(self):
        self.request["allowed_paths"].append("src/protected.txt"); self.reject("scope not attributed")
        self.request["allowed_paths"].remove("src/protected.txt"); self.admit()
        self.write("unrelated.txt", "out of scope")
        with self.assertRaisesRegex(gate.GateError, "unauthorized scope"):
            dashboard.record(self.root, gate.read_state(self.root))

    def test_protected_and_original_evidence_drift(self):
        self.admit(); self.write("src/protected.txt", "drift")
        with self.assertRaisesRegex(gate.GateError, "protected path"):
            dashboard.record(self.root, gate.read_state(self.root))
        self.write("src/protected.txt", "Unrelated protected work\n"); self.write(self.old_report, "drift")
        with self.assertRaisesRegex(gate.GateError, "original report missing or changed"):
            dashboard.record(self.root, gate.read_state(self.root))

    def test_receipt_pointer_and_decision_drift(self):
        self.admit(); state = gate.read_state(self.root)
        p = self.root / dashboard.STORE / (state["dashboard_publication_lineage"]["admission"] + ".json")
        raw = p.read_bytes(); p.write_bytes(raw + b" ")
        with self.assertRaisesRegex(gate.GateError, "sealed receipt drift"): dashboard.record(self.root, state)
        p.write_bytes(raw)
        self.write("DECISIONS.md", "drift")
        with self.assertRaisesRegex(gate.GateError, "decision binding drift"): dashboard.record(self.root, state)

    def test_missing_product_and_omitted_evidence_cannot_complete(self):
        self.admit(); report = self.review(dashboard.SUCCESSOR, milestone=True)
        with self.assertRaises(gate.GateError): gate.finalize_gate(self.root, dashboard.SUCCESSOR, report)
        report = self.final(); m = gate._extract_manifest((self.root / report).read_text())
        old = copy.deepcopy(m)
        for kind, key, name in [("verification", "command", "npm run verify"),
            ("manual_verification", "check", "Recovery obligation " + sorted(self.obligations)[0])]:
            m[kind] = [r for r in m[kind] if r[key] != name]; self.write_review(report, m)
            with self.assertRaisesRegex(gate.GateError, "required evidence omitted"):
                gate.finalize_gate(self.root, dashboard.SUCCESSOR, report)
            m = copy.deepcopy(old)

    def test_replay_and_mixed_lineage(self):
        self.admit(); raw = gate.state_path(self.root).read_bytes()
        with self.assertRaisesRegex(gate.GateError, "replay"): self.admit()
        self.assertEqual(gate.state_path(self.root).read_bytes(), raw)
        state = gate.read_state(self.root); state["dependent_lineage"] = {"admission": "0"*64}
        with self.assertRaisesRegex(gate.GateError, "mixed lineage"): gate.validate_state(state)

    def test_state_schema_requires_integer_not_float(self):
        self.admit(); state = gate.read_state(self.root); state["schema_version"] = 6.0
        with self.assertRaisesRegex(gate.GateError, "state identity mismatch"): gate.validate_state(state)

    def test_alternate_report_path_cannot_replace_admitted_report(self):
        self.admit(); final = self.final()
        alternate = final.replace('2026-10-20', '2026-10-21')
        m = gate._extract_manifest((self.root/final).read_text())
        self.write(alternate, 'placeholder')
        m['files_changed'] = list(gate.changed_paths(self.root))
        self.write_review(alternate, m)
        with self.assertRaisesRegex(gate.GateError, 'report path does not match'):
            gate.validate_report(self.root, alternate, dashboard.SUCCESSOR)

    def test_interruption_after_receipt_blocks_replay_without_state_change(self):
        original = dashboard.exclusive
        def interrupt(root, name, data):
            original(root, name, data)
            raise OSError("Synthetic interruption")
        with mock.patch.object(dashboard, "exclusive", side_effect=interrupt):
            with self.assertRaises(gate.GateError): self.admit()
        self.assertEqual(gate.state_path(self.root).read_bytes(), self.raw)
        self.reject("interrupted admission")

    def test_interruption_at_state_write_retains_consumed_pointer(self):
        with mock.patch.object(gate, "write_state", side_effect=OSError("Synthetic interruption")):
            with self.assertRaises(gate.GateError): self.admit()
        self.assertTrue((self.root / dashboard.POINTER).exists())
        self.assertEqual(gate.state_path(self.root).read_bytes(), self.raw)
        with self.assertRaisesRegex(gate.GateError, "interrupted admission"): self.admit()

    def test_interruption_after_pointer_retains_old_state_and_consumption(self):
        original = dashboard.exclusive
        def interrupt(root, name, data):
            original(root, name, data)
            if name == dashboard.POINTER: raise OSError("Synthetic pointer interruption")
        with mock.patch.object(dashboard, "exclusive", side_effect=interrupt):
            with self.assertRaises(gate.GateError): self.admit()
        self.assertTrue((self.root / dashboard.POINTER).exists())
        self.assertEqual(gate.state_path(self.root).read_bytes(), self.raw)
        with self.assertRaisesRegex(gate.GateError, "interrupted admission"): self.admit()

    def test_interruption_after_state_write_cannot_start_replacement(self):
        original = gate.write_state
        def interrupt(root, state):
            original(root, state); raise OSError("Synthetic response interruption")
        with mock.patch.object(gate, "write_state", side_effect=interrupt):
            with self.assertRaises(gate.GateError): self.admit()
        self.assertEqual(gate.read_state(self.root)["status"], "active")
        with self.assertRaisesRegex(gate.GateError, "interrupted admission"): self.admit()
        with self.assertRaisesRegex(gate.GateError, "cannot be replayed"):
            gate.begin_gate(self.root, dashboard.SUCCESSOR)

    def test_false_pass_with_blocked_criterion_or_failed_obligation(self):
        self.admit(); report = self.final()
        m = gate._extract_manifest((self.root / report).read_text()); old = copy.deepcopy(m)
        m["milestone"]["criteria"]["source"]["status"] = "blocked"
        self.write_review(report, m)
        with self.assertRaisesRegex(gate.GateError, "does not match report evidence"):
            gate.finalize_gate(self.root, dashboard.SUCCESSOR, report)
        m = old
        next(r for r in m["manual_verification"] if r["check"].startswith("Recovery obligation"))["status"] = "Failed"
        self.write_review(report, m)
        with self.assertRaisesRegex(gate.GateError, "does not match report evidence"):
            gate.finalize_gate(self.root, dashboard.SUCCESSOR, report)

    def test_changed_product_hash_and_deleted_existing_file_rejected(self):
        self.admit(); report = self.final()
        self.write(next(iter(self.product)), "unreviewed replacement")
        with self.assertRaisesRegex(gate.GateError, "accepted product binding"):
            gate.finalize_gate(self.root, dashboard.SUCCESSOR, report)
        (self.root / "src/protected.txt").unlink()
        with self.assertRaisesRegex(gate.GateError, "protected path"):
            dashboard.record(self.root, gate.read_state(self.root))

    def test_pointer_index_and_branch_drift_rejected(self):
        self.admit(); state = gate.read_state(self.root)
        p = self.root / dashboard.POINTER; raw = p.read_bytes()
        self.write_json(dashboard.POINTER, {"admission": "0"*64})
        with self.assertRaisesRegex(gate.GateError, "consumed identity"): dashboard.record(self.root, state)
        p.write_bytes(raw)
        self.git("branch", "-m", "wrong-branch")
        with self.assertRaisesRegex(gate.GateError, "checkout binding"): dashboard.record(self.root, state)
        self.git("branch", "-m", dashboard.BRANCH)
        self.git("add", "docs")
        with self.assertRaisesRegex(gate.GateError, "index drift"): dashboard.record(self.root, state)

    def test_missing_named_review_cannot_admit(self):
        m = gate._extract_manifest((self.root / dashboard.READINESS).read_text())
        m["manual_verification"] = [r for r in m["manual_verification"] if r["check"] != "Preservation and scope review"]
        self.write_review(dashboard.READINESS, m)
        self.request["workspace_fingerprint"] = gate.workspace_fingerprint(self.root)
        self.reject("required review missing")

    def test_race_after_receipt_stops_before_pointer(self):
        original = dashboard.exclusive
        def drift(root, name, data):
            original(root, name, data)
            self.write("DECISIONS.md", "raced")
        with mock.patch.object(dashboard, "exclusive", side_effect=drift):
            with self.assertRaisesRegex(gate.GateError, "decision binding drift"): self.admit()
        self.assertEqual(gate.state_path(self.root).read_bytes(), self.raw)
        self.assertFalse((self.root / dashboard.POINTER).exists())

    def test_routes_mutually_exclusive_and_ordinary_dependency_rejected(self):
        with self.assertRaisesRegex(gate.GateError, "choose one"):
            gate.begin_gate(self.root, dashboard.SUCCESSOR, admission=self.request_path,
                            dashboard_publication_request=self.request_path)
        ordinary = {"schema_version": 2, "increment_id": dashboard.SUCCESSOR, "owner_authorization": self.auth(),
            "workspace_fingerprint": gate.workspace_fingerprint(self.root), "milestone": self.milestone,
            "dependencies": {dashboard.FAILED_STATE + "/" + n: {"relationship": "dependent", "rationale": "Related"}
                             for n in self.obligations}, "readiness_report": dashboard.READINESS,
            "readiness_increment": dashboard.READINESS_ID}
        self.write_json(".codex/state/ordinary.json", ordinary)
        with self.assertRaisesRegex(gate.GateError, "depends on unresolved failure"):
            gate.begin_gate(self.root, dashboard.SUCCESSOR, admission=".codex/state/ordinary.json")


if __name__ == "__main__": unittest.main()
