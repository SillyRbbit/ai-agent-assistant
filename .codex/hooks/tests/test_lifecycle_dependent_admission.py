from __future__ import annotations

import copy
import json
import subprocess
import sys
import tempfile
import unittest
from unittest import mock
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import post_increment_gate as gate
import lifecycle_closure as closure
import lifecycle_dependent_admission as dependent


class DependentAdmissionTests(unittest.TestCase):
    """Synthetic schema-2 predecessor; every rejection reaches its own boundary."""

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name).resolve()
        for args in [("init", "-q"), ("config", "user.email", "fixture@example.invalid"),
                     ("config", "user.name", "Synthetic fixture"), ("branch", "-m", dependent.BRANCH)]:
            self.git(*args)
        (self.root / ".gitignore").write_text(".codex/state/\n__pycache__/\n")
        for name in sorted(dependent.ROOT_DOCS | {"src/protected.txt"}):
            p = self.root / name; p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text("# Synthetic original " + name + "\n")
        self.git("add", "."); self.git("commit", "-qm", "Synthetic baseline")
        self.old_plan = "docs/plans/2026-10-11-vps-dual-label-operating-mode.md"
        p = self.root / self.old_plan; p.parent.mkdir(parents=True, exist_ok=True); p.write_text("# Synthetic plan\n")
        ready_id = dependent.PREDECESSOR + "-readiness"
        ready = self.review(ready_id)
        acceptance = {"records": "Truthful documentation", "preservation": "Compare every checkout", "validation": "Required checks"}
        req = {"schema_version": 2, "increment_id": dependent.PREDECESSOR,
               "owner_authorization": self.auth(), "workspace_fingerprint": gate.workspace_fingerprint(self.root),
               "dependencies": {}, "readiness_report": ready, "readiness_increment": ready_id,
               "milestone": self.contract(acceptance, [ready, "src"])}
        admission = self.put(closure.encoded(req))["path"]
        gate.begin_gate(self.root, dependent.PREDECESSOR, admission=admission)
        self.old_report = self.review(dependent.PREDECESSOR, failed=True, milestone=req["milestone"])
        gate.close_failed_gate(self.root, dependent.PREDECESSOR, self.old_report)
        self.raw = gate.state_path(self.root).read_bytes()
        self.original = gate._extract_manifest((self.root / self.old_report).read_text())
        self.historical = self.put(closure.encoded(closure.manifest(self.root)))
        self.old_state_ref = self.put(self.raw)
        copied_index = self.put(closure.read_bytes(closure.git_index_path(self.root)))
        files = [self.old_state_ref, copied_index, self.ref(self.old_report)]
        self.evidence = {
            "historical_manifest": self.historical,
            "comparison": self.put(closure.encoded({"pass": False, "exit_code": 1})),
            "snapshot": self.put(closure.encoded({"files": files})),
            "archival_provenance": self.put(closure.encoded({"owner_intent": "intentional-archival", "complete_custody": False})),
            "unavailable_inventory": self.put(closure.encoded({"records": [{"id": n, "availability": "unavailable", "recorded_sha256": h} for n, h in sorted(dependent.UNAVAILABLE_HASHES.items())], "complete_historical_preservation": False, "absence_of_loss": False})),
            "surviving_preservation": self.put(closure.encoded({"result": "PASS", "scope": "surviving-evidence", "files": files})),
        }
        self.plan = "docs/plans/2026-10-11-" + dependent.SUCCESSOR + ".md"
        (self.root / self.plan).write_text("# Synthetic dependent plan\n")
        ready_id = dependent.SUCCESSOR + "-readiness"
        self.ready = self.review(ready_id)
        self.final = self.report_path(dependent.SUCCESSOR)
        allowed = sorted(dependent.ROOT_DOCS | {self.plan, self.final})
        self.request = {"schema_version": 1, "increment_id": dependent.SUCCESSOR, "root": str(self.root),
            "branch": dependent.BRANCH, "head": gate.current_head_commit(self.root),
            "workspace_fingerprint": gate.workspace_fingerprint(self.root),
            "index_sha256": closure.digest(closure.read_bytes(closure.git_index_path(self.root))),
            "predecessor": {"state": self.old_state_ref, "report": self.ref(self.old_report)},
            "evidence": self.evidence, "input_bindings": {n: v for n, v in closure.manifest(self.root).items() if n not in allowed},
            "milestone": self.contract({**acceptance, "preservation": "surviving-evidence-and-explicit-unavailable-custody"}, [self.old_report, self.ready, "src"]), "allowed_paths": allowed,
            "plan": self.plan, "report": self.final, "readiness_report": self.ready, "readiness_increment": ready_id,
            "obligations": {n: {"criterion": "preservation", "required": True} for n in closure.unresolved((self.root / self.old_report).read_text())},
            "acceptance_change": {"criterion": "preservation", "original_sha256": closure.digest(closure.encoded(self.original["milestone"]["criteria"]["preservation"])), "replacement": "surviving-evidence-and-explicit-unavailable-custody", "replacement_command": dependent.PRESERVATION_COMMAND, "owner_authorization": self.auth()},
            "owner_authorization": {"admission": self.auth(), "execution": self.auth()},
        }
        self.request_path = ".codex/state/dependent-request.json"
        self.save_request()
        self.assertEqual(gate.read_state(self.root)["status"], "failed")
        self.assertTrue(self.request["obligations"])

    def tearDown(self): self.temp.cleanup()

    def git(self, *args):
        subprocess.run(["git", *args], cwd=self.root, check=True, capture_output=True)

    def auth(self):
        return {"approved": True, "source": "Explicit synthetic owner instruction", "decision_path": "DECISIONS.md", "decision_sha256": closure.digest((self.root / "DECISIONS.md").read_bytes())}

    def contract(self, acceptance, protected):
        return {"objective": "Bounded synthetic documentation", "exclusions": ["Execution and publication"], "acceptance": acceptance, "protected_paths": protected, "authorized_destructive_paths": []}

    def put(self, data):
        name = ".codex/state/blobs/" + closure.digest(data)
        p = self.root / name; p.parent.mkdir(parents=True, exist_ok=True); p.write_bytes(data)
        return {"path": name, "sha256": closure.digest(data)}

    def ref(self, name): return {"path": name, "sha256": closure.digest((self.root / name).read_bytes())}
    def report_path(self, increment): return "docs/reviews/2026-10-11-" + increment + "-post-increment-review.md"
    def save_request(self): (self.root / self.request_path).write_bytes(closure.encoded(self.request))

    def review(self, increment, *, failed=False, milestone=None):
        name = self.report_path(increment)
        p = self.root / name; p.parent.mkdir(parents=True, exist_ok=True); p.write_text("placeholder\n")
        commands = sorted(closure.REQUIRED_COMMANDS - {"npm run verify"})
        if milestone:
            commands.append(dependent.PRESERVATION_COMMAND if increment == dependent.SUCCESSOR else "python3 -B /private/tmp/cortexa-dual-label-final-preservation.py")
        m = {"schema_version": 1, "increment_id": increment, "quality_gate": "FAIL" if failed else "PASS", "next_increment_readiness": "Blocked" if failed else "Ready", "files_changed": list(gate.changed_paths(self.root)), "commands_executed": commands,
             "verification": [{"command": c, "required": True, "status": "Failed" if failed and "preservation" in c else "Passed"} for c in commands],
             "manual_verification": [{"check": c, "required": True, "status": "Failed" if failed and "Preservation" in c else "Passed"} for c in sorted(closure.REQUIRED_REVIEWS)], "findings": []}
        if milestone:
            state = gate.read_state(self.root)
            if increment == dependent.SUCCESSOR:
                before = dependent.record(self.root, state)["baseline_manifest"]
            else:
                before = closure.read_json(self.root / closure.DIRECTORY / ("admission-" + state["closure_lineage"]["admission"] + ".json"))["baseline_manifest"]
            m["milestone"] = {"criteria": {c: {"status": "blocked" if failed else "automatically_verified", "evidence": "Synthetic failed/passing evidence; never a real execution claim"} for c in milestone["acceptance"]}, "paths": {n: {"criterion": "records", "rationale": "Synthetic bounded documentation", "within_objective": True, "preserves_existing": True} for n in closure.changed(before, closure.manifest(self.root))}}
        p.write_text("# Synthetic review\n\n" + gate.MANIFEST_START + json.dumps(m, indent=2) + gate.MANIFEST_END + "\n\n" + "\n".join(s + "\n\nSynthetic evidence only.\n" for s in gate.REQUIRED_REPORT_SECTIONS))
        return name

    def admit(self):
        self.save_request()
        gate.begin_gate(self.root, dependent.SUCCESSOR, dependent_request=self.request_path)

    def reject(self, pattern):
        self.save_request()
        with self.assertRaisesRegex(gate.GateError, pattern):
            gate.begin_gate(self.root, dependent.SUCCESSOR, dependent_request=self.request_path)
        self.assertEqual(gate.state_path(self.root).read_bytes(), self.raw)
        self.assertFalse((self.root / dependent.POINTER).exists())

    def test_completion_preserves_original_and_full_stop(self):
        self.admit()
        final = self.review(dependent.SUCCESSOR, milestone=self.request["milestone"])
        gate.finalize_gate(self.root, dependent.SUCCESSOR, final)
        state = gate.read_state(self.root)
        self.assertTrue(gate.validate_completed_state(self.root, state))
        rec = dependent.record(self.root, state)
        self.assertEqual(rec["raw_state"].encode(), self.raw)
        self.assertEqual(json.loads(rec["raw_state"])["quality_gate"], "FAIL")
        self.assertNotIn("completion_marker", json.loads(rec["raw_state"]))
        self.assertFalse(gate.evaluate_stop_payload({"cwd": str(self.root), "hook_event_name": "Stop", "stop_hook_active": False}).should_continue)

    def test_terminal_failure_is_immutable_and_requires_new_authority(self):
        self.admit(); final = self.review(dependent.SUCCESSOR, failed=True, milestone=self.request["milestone"])
        gate.close_failed_gate(self.root, dependent.SUCCESSOR, final)
        state = gate.read_state(self.root)
        self.assertTrue(gate.validate_failed_state(self.root, state))
        self.assertNotIn("completion_marker", state)
        with self.assertRaisesRegex(gate.GateError, "terminal failure is immutable"):
            gate.close_failed_gate(self.root, dependent.SUCCESSOR, final)
        with self.assertRaisesRegex(gate.GateError, "cannot be replayed"):
            gate.begin_gate(self.root, "another-task")
        with self.assertRaisesRegex(gate.GateError, "terminally failed"):
            gate.finalize_gate(self.root, dependent.SUCCESSOR, final)

    def test_separate_admission_execution_and_acceptance_approval(self):
        for lane in ["admission", "execution", "acceptance-change"]:
            old = copy.deepcopy(self.request)
            auth = self.request["acceptance_change"]["owner_authorization"] if lane == "acceptance-change" else self.request["owner_authorization"][lane]
            auth["approved"] = False; self.reject("owner authorization required"); self.request = old

    def test_root_branch_head_and_fingerprint_bound(self):
        for key, value, pattern in [("root", "/private/tmp/other", "identity mismatch"), ("branch", "main", "identity mismatch"), ("head", "0"*40, "HEAD mismatch"), ("workspace_fingerprint", "0"*64, "input drift")]:
            old = self.request[key]; self.request[key] = value; self.reject(pattern); self.request[key] = old

    def test_all_unmet_obligations_and_criteria_required(self):
        old = copy.deepcopy(self.request)
        self.request["obligations"].pop(next(iter(self.request["obligations"]))); self.reject("invalid record fields"); self.request = copy.deepcopy(old)
        self.request["milestone"]["acceptance"].pop("validation"); self.reject("criterion omitted or expanded"); self.request = copy.deepcopy(old)
        self.request["obligations"][next(iter(self.request["obligations"]))]["required"] = False; self.reject("omitted or waived")

    def test_acceptance_change_exact_and_original_bound(self):
        for key, value, pattern in [("criterion", "validation", "out of scope"), ("original_sha256", "0"*64, "original criterion mismatch"), ("replacement_command", "npm run verify", "replacement out of scope")]:
            old = self.request["acceptance_change"][key]; self.request["acceptance_change"][key] = value; self.reject(pattern); self.request["acceptance_change"][key] = old

    def test_missing_or_tampered_reference_rejected(self):
        self.request["evidence"]["comparison"]["sha256"] = "0"*64; self.reject("evidence hash mismatch")

    def test_historical_manifest_cannot_be_replaced(self):
        self.request["evidence"]["historical_manifest"] = self.put(closure.encoded(closure.manifest(self.root)))
        self.reject("historical inventory mismatch")

    def test_missing_custody_and_overclaims_rejected(self):
        inventory = json.loads((self.root / self.evidence["unavailable_inventory"]["path"]).read_bytes())
        old = copy.deepcopy(inventory)
        inventory["records"].pop(); self.request["evidence"]["unavailable_inventory"] = self.put(closure.encoded(inventory)); self.reject("inventory incomplete")
        inventory = old; inventory["complete_historical_preservation"] = True
        self.request["evidence"]["unavailable_inventory"] = self.put(closure.encoded(inventory)); self.reject("overclaim")

    def test_snapshot_original_bytes_required(self):
        snapshot = json.loads((self.root / self.evidence["snapshot"]["path"]).read_bytes())
        p = self.root / snapshot["files"][1]["path"]; p.write_bytes(b"changed index copy")
        self.reject("evidence hash mismatch")

    def test_arbitrary_scope_and_unbound_implementation_rejected(self):
        self.request["allowed_paths"].append("src/protected.txt"); self.reject("scope mismatch")
        self.request["allowed_paths"].remove("src/protected.txt")
        self.request["input_bindings"].pop("src/protected.txt"); self.reject("input bindings mismatch")

    def test_decision_and_predecessor_state_binding(self):
        self.request["owner_authorization"]["admission"]["decision_sha256"] = "0"*64; self.reject("decision binding drift")
        self.request["owner_authorization"]["admission"] = self.auth()
        self.request["predecessor"]["state"] = self.put(closure.encoded({"status": "failed"})); self.reject("predecessor binding mismatch")

    def test_replay_rejected_without_new_state(self):
        self.admit(); raw = gate.state_path(self.root).read_bytes()
        with self.assertRaisesRegex(gate.GateError, "replay rejected"): self.admit()
        self.assertEqual(gate.state_path(self.root).read_bytes(), raw)

    def test_post_admission_scope_and_evidence_tamper(self):
        self.admit()
        (self.root / "src/protected.txt").write_text("scope drift")
        with self.assertRaisesRegex(gate.GateError, "protected implementation drift"):
            dependent.record(self.root, gate.read_state(self.root))

    def test_sealed_receipt_and_pointer_tamper(self):
        self.admit(); state = gate.read_state(self.root)
        receipt = self.root / dependent.STORE / (state["dependent_lineage"]["admission"] + ".json")
        raw = receipt.read_bytes(); receipt.write_bytes(raw + b" ")
        with self.assertRaisesRegex(gate.GateError, "admission tampered"): dependent.record(self.root, state)
        receipt.write_bytes(raw)
        (self.root / dependent.POINTER).write_bytes(closure.encoded({"admission": "0"*64}))
        with self.assertRaisesRegex(gate.GateError, "consumed identity mismatch"): dependent.record(self.root, state)

    def test_omitted_inherited_check_and_false_completion_rejected(self):
        self.admit(); final = self.review(dependent.SUCCESSOR, milestone=self.request["milestone"])
        p = self.root / final; text = p.read_text(); m = gate._extract_manifest(text)
        m["verification"] = [r for r in m["verification"] if r["command"] != dependent.PRESERVATION_COMMAND]
        start = text.index(gate.MANIFEST_START) + len(gate.MANIFEST_START); end = text.index(gate.MANIFEST_END)
        p.write_text(text[:start] + json.dumps(m) + text[end:])
        with self.assertRaisesRegex(gate.GateError, "required check omitted"): gate.finalize_gate(self.root, dependent.SUCCESSOR, final)

    def test_d133_and_mixed_lineage_rejected(self):
        import lifecycle_acceptance as acceptance
        p = self.root / acceptance.POINTER; p.parent.mkdir(parents=True, exist_ok=True); p.write_text("{}")
        self.reject("D-133 requires")
        p.unlink(); self.admit(); state = gate.read_state(self.root)
        state["closure_lineage"] = {"closures": [], "admission": "0"*64}
        with self.assertRaisesRegex(gate.GateError, "mixed lineage"): gate.validate_state(state)

    def test_duplicate_keys_and_symlink_evidence_rejected(self):
        p = self.root / self.request_path; p.write_text('{"schema_version":1,"schema_version":1}')
        with self.assertRaisesRegex(gate.GateError, "duplicate JSON key"):
            gate.begin_gate(self.root, dependent.SUCCESSOR, dependent_request=self.request_path)
        self.save_request()
        ref = self.request["evidence"]["comparison"]; source = self.root / ref["path"]
        data = source.read_bytes(); source.unlink(); target = self.root / ".codex/state/outside-copy"; target.write_bytes(data); source.symlink_to(target)
        self.reject("symlink evidence")



    def test_prospective_report_absent_then_strict_finalization(self):
        self.assertFalse((self.root / self.final).exists())
        self.admit()
        with self.assertRaises(gate.GateError):
            gate.finalize_gate(self.root, dependent.SUCCESSOR, self.final)
        self.assertEqual(gate.read_state(self.root)["status"], "active")

    def test_prospective_report_wrong_location_and_name(self):
        for value in ["docs/elsewhere/report.md", "docs/reviews/2026-10-11-other-post-increment-review.md"]:
            self.request["report"] = value
            self.reject("prospective report identity mismatch")

    def test_prospective_report_symlink_and_directory_rejected(self):
        p = self.root / self.final
        p.symlink_to(self.root / self.old_report)
        with self.assertRaisesRegex(gate.GateError, "prospective report symlink"):
            dependent.prospective_report(self.root, self.final)
        p.unlink(); p.mkdir()
        with self.assertRaisesRegex(gate.GateError, "prospective report not regular"):
            dependent.prospective_report(self.root, self.final)

    def test_prospective_report_parent_symlink_rejected(self):
        parent = self.root / "docs/reviews"
        original = self.root / "docs/retained-reviews"
        parent.rename(original); parent.symlink_to(original)
        with self.assertRaisesRegex(gate.GateError, "prospective report symlink"):
            dependent.prospective_report(self.root, self.final)

    def test_required_snapshot_anchors_cannot_be_omitted(self):
        files = json.loads((self.root / self.evidence["snapshot"]["path"]).read_bytes())["files"]
        files.pop(1)
        self.request["evidence"]["snapshot"] = self.put(closure.encoded({"files": files}))
        self.request["evidence"]["surviving_preservation"] = self.put(closure.encoded({"result": "PASS", "scope": "surviving-evidence", "files": files}))
        self.reject("essential snapshot copy missing")

    def test_interrupted_record_publication_rejects_replay(self):
        original = dependent.exclusive
        def interrupted(root, name, data):
            if name == dependent.POINTER:
                raise OSError("Synthetic interruption before consumed receipt")
            return original(root, name, data)
        with mock.patch.object(dependent, "exclusive", side_effect=interrupted):
            with self.assertRaises(gate.GateError): self.admit()
        self.assertEqual(gate.state_path(self.root).read_bytes(), self.raw)
        self.assertFalse((self.root / dependent.POINTER).exists())
        self.reject("replay rejected")
        self.request["owner_authorization"]["admission"]["source"] = "Changed synthetic request after interruption"
        self.reject("interrupted admission evidence exists")

    def test_interrupted_state_write_cannot_reopen_original(self):
        with mock.patch.object(gate, "write_state", side_effect=OSError("Synthetic interrupted state write")):
            with self.assertRaises(gate.GateError): self.admit()
        self.assertEqual(gate.state_path(self.root).read_bytes(), self.raw)
        self.assertTrue((self.root / dependent.POINTER).exists())
        with self.assertRaisesRegex(gate.GateError, "replay rejected"): self.admit()
        self.assertEqual(gate.state_path(self.root).read_bytes(), self.raw)

    def test_evidence_drift_after_admission_blocks_completion(self):
        self.admit()
        (self.root / self.evidence["comparison"]["path"]).write_text("tampered")
        with self.assertRaisesRegex(gate.GateError, "evidence hash mismatch"):
            dependent.record(self.root, gate.read_state(self.root))

    def test_index_drift_after_admission_blocks_completion(self):
        self.admit(); self.git("add", self.plan)
        with self.assertRaisesRegex(gate.GateError, "index drift"):
            dependent.record(self.root, gate.read_state(self.root))


if __name__ == "__main__": unittest.main()
