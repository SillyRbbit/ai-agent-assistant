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


class MemoryAdmissionTests(unittest.TestCase):
    """Synthetic histories only; never import or mutate real memory state."""

    git = DependentAdmissionTests.git
    auth = DependentAdmissionTests.auth
    contract = DependentAdmissionTests.contract
    put = DependentAdmissionTests.put
    ref = DependentAdmissionTests.ref
    report_path = DependentAdmissionTests.report_path
    save_request = DependentAdmissionTests.save_request
    def reject(self, pattern):
        self.save_request()
        with self.assertRaisesRegex(gate.GateError, pattern):
            gate.begin_gate(self.root, dependent.MEMORY_SUCCESSOR, dependent_request=self.request_path)
        self.assertEqual(gate.state_path(self.root).read_bytes(), self.raw)
        self.assertFalse((self.root / dependent.POINTER).exists())

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name).resolve()
        for args in [("init", "-q"), ("config", "user.email", "fixture@example.invalid"),
                     ("config", "user.name", "Synthetic fixture"), ("branch", "-m", dependent.MEMORY_BRANCH)]:
            self.git(*args)
        (self.root / ".gitignore").write_text(".codex/state/\n")
        names = sorted(dependent.MEMORY_DOCS | dependent.MEMORY_SOURCE)
        names += ["src/frozen-" + str(i) + ".txt" for i in range(31-len(names))]
        for name in names + sorted(dependent.MAINTENANCE_FILES):
            target = self.root / name; target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text("Synthetic retained input.\n")
        self.git("add", "."); self.git("commit", "-qm", "Synthetic baseline")
        self.acceptance = {"native-qa": "Mandatory live native evidence", "preservation": "Retain every original byte"}
        ready_id = dependent.MEMORY_PREDECESSOR + "-readiness"
        ready = self.review(ready_id)
        req = {"schema_version": 2, "increment_id": dependent.MEMORY_PREDECESSOR,
               "owner_authorization": self.auth(), "workspace_fingerprint": gate.workspace_fingerprint(self.root),
               "dependencies": {}, "readiness_report": ready, "readiness_increment": ready_id,
               "milestone": self.contract(self.acceptance, [ready])}
        gate.begin_gate(self.root, dependent.MEMORY_PREDECESSOR, admission=self.put(closure.encoded(req))["path"])
        self.old_report = self.review(dependent.MEMORY_PREDECESSOR, failed=True)
        gate.close_failed_gate(self.root, dependent.MEMORY_PREDECESSOR, self.old_report)
        self.raw = gate.state_path(self.root).read_bytes()
        old_ref = self.put(self.raw)
        index_ref = self.put(closure.read_bytes(closure.git_index_path(self.root)))
        names.append(self.old_report)
        candidate = {"branch": dependent.MEMORY_BRANCH, "candidate": {n: {"sha256": self.ref(n)["sha256"], "mode": oct((self.root/n).stat().st_mode & 0o7777)} for n in names}}
        self.candidate = self.put(closure.encoded(candidate))
        snapshot = {"files": {n: {"copy": self.put((self.root/n).read_bytes()), "mode": v["mode"]} for n,v in candidate["candidate"].items()}, "state": old_ref, "index": index_ref}
        self.maintenance = self.maintenance_evidence()
        self.plan = "docs/plans/2026-10-11-" + dependent.MEMORY_SUCCESSOR + ".md"
        target = self.root/self.plan; target.parent.mkdir(parents=True, exist_ok=True); target.write_text("# Synthetic continuation\n")
        ready_id = dependent.MEMORY_SUCCESSOR + "-readiness"
        self.ready = self.review(ready_id)
        self.final = self.report_path(dependent.MEMORY_SUCCESSOR)
        allowed = sorted(dependent.MEMORY_DOCS | dependent.MEMORY_SOURCE | {self.plan, self.final})
        self.request = {"schema_version": 1, "increment_id": dependent.MEMORY_SUCCESSOR,
            "root": str(self.root), "branch": dependent.MEMORY_BRANCH, "head": gate.current_head_commit(self.root),
            "workspace_fingerprint": gate.workspace_fingerprint(self.root), "index_sha256": index_ref["sha256"],
            "predecessor": {"state": old_ref, "report": self.ref(self.old_report)},
            "evidence": {"candidate": self.candidate, "snapshot": self.put(closure.encoded(snapshot)), "maintenance": self.put(closure.encoded(self.maintenance))},
            "input_bindings": {n:v for n,v in closure.manifest(self.root).items() if n not in allowed},
            "milestone": self.contract(self.acceptance, [self.old_report, self.ready]),
            "allowed_paths": allowed, "plan": self.plan, "report": self.final,
            "readiness_report": self.ready, "readiness_increment": ready_id,
            "obligations": {n: {"criterion": "native-qa", "required": True} for n in closure.unresolved((self.root/self.old_report).read_text())},
            "acceptance_change": None, "owner_authorization": {"admission": self.auth(), "execution": self.auth()}}
        self.request_path = ".codex/state/memory-request.json"
        for name,value in {"MEMORY_ROOT": str(self.root), "MEMORY_STATE": old_ref["sha256"], "MEMORY_REPORT": self.ref(self.old_report)["sha256"], "MEMORY_CANDIDATE": self.candidate["sha256"]}.items():
            patch = mock.patch.object(dependent,name,value);patch.start();self.addCleanup(patch.stop)
        self.save_request()

    def review(self, increment, *, failed=False):
        name = self.report_path(increment);p=self.root/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text("pending\n")
        commands = sorted(closure.REQUIRED_COMMANDS - {"npm run verify"})
        state=gate.read_state(self.root)
        m={"schema_version":1,"increment_id":increment,"quality_gate":"FAIL" if failed else "PASS","next_increment_readiness":"Blocked" if failed else "Ready","files_changed":list(gate.changed_paths(self.root)),"commands_executed":commands,"verification":[{"command":c,"required":True,"status":"Passed"}for c in commands],"manual_verification":[{"check":c,"required":True,"status":"Failed" if failed else "Passed"}for c in sorted(closure.REQUIRED_REVIEWS)],"findings":[]}
        if state and state["increment_id"] == increment:
            before = dependent.record(self.root,state)["baseline_manifest"] if "dependent_lineage" in state else closure.read_json(self.root/closure.DIRECTORY/("admission-"+state["closure_lineage"]["admission"]+".json"))["baseline_manifest"]
            m["milestone"]={"criteria":{c:{"status":"blocked" if failed else "automatically_verified","evidence":"Synthetic validation only"}for c in self.acceptance},"paths":{n:{"criterion":"native-qa","rationale":"Synthetic continuation","within_objective":True,"preserves_existing":True}for n in closure.changed(before,closure.manifest(self.root))}}
        p.write_text("# Synthetic review\n\n"+gate.MANIFEST_START+json.dumps(m,indent=2)+gate.MANIFEST_END+"\n\n"+"\n".join(h+"\n\nSynthetic only.\n"for h in gate.REQUIRED_REPORT_SECTIONS));return name

    def maintenance_evidence(self):
        report={"increment_id":dependent.MAINTENANCE,"quality_gate":"PASS","verification":[{"required":True,"status":"Passed"}],"manual_verification":[{"required":True,"status":"Passed"}],"findings":[],"milestone":{"criteria":{k:{"status":"automatically_verified"}for k in ("binding","obligations","boundaries","verification")}}}
        raw=(gate.MANIFEST_START+json.dumps(report)+gate.MANIFEST_END).encode()
        manifest=closure.manifest(self.root)
        state={"schema_version":gate.STATE_SCHEMA_VERSION,"increment_id":dependent.MAINTENANCE,"status":"complete","quality_gate":"PASS","completion_marker":gate.COMPLETION_MARKER,"baseline_fingerprint":"0"*64,"workspace_fingerprint":closure.fingerprint(manifest),"report_path":self.report_path(dependent.MAINTENANCE),"report_sha256":closure.digest(raw)}
        return {"state":self.put(closure.encoded(state)),"report":self.put(raw),"manifest":self.put(closure.encoded(manifest)),"files":{n:self.put((self.root/n).read_bytes())for n in dependent.MAINTENANCE_FILES},"stop":self.put(closure.encoded({"exit_status":0,"decision":"allow","state_sha256":closure.digest(closure.encoded(state))}))}

    def admit(self):
        self.save_request();gate.begin_gate(self.root,dependent.MEMORY_SUCCESSOR,dependent_request=self.request_path)

    def test_memory_completion_preserves_failure_and_full_stop(self):
        self.admit();final=self.review(dependent.MEMORY_SUCCESSOR)
        gate.finalize_gate(self.root,dependent.MEMORY_SUCCESSOR,final)
        state=gate.read_state(self.root)
        self.assertTrue(gate.validate_completed_state(self.root,state))
        self.assertEqual(dependent.record(self.root,state)["raw_state"].encode(),self.raw)
        self.assertFalse(gate.evaluate_stop_payload({"cwd":str(self.root),"hook_event_name":"Stop","stop_hook_active":False}).should_continue)

    def test_memory_cannot_complete_unmet_required_native_check(self):
        self.admit();final=self.review(dependent.MEMORY_SUCCESSOR,failed=True)
        with self.assertRaises(gate.GateError):gate.finalize_gate(self.root,dependent.MEMORY_SUCCESSOR,final)
        gate.close_failed_gate(self.root,dependent.MEMORY_SUCCESSOR,final)
        self.assertTrue(gate.validate_failed_state(self.root,gate.read_state(self.root)))
        with self.assertRaises(gate.GateError):self.admit()

    def test_memory_rejects_acceptance_substitution_and_missing_obligations(self):
        baseline=copy.deepcopy(self.request)
        self.request["acceptance_change"]={};self.reject("acceptance changes forbidden");self.request=copy.deepcopy(baseline)
        self.request["milestone"]["acceptance"]["preservation"]="Skip";self.reject("acceptance text drift");self.request=copy.deepcopy(baseline)
        self.request["obligations"].pop(next(iter(self.request["obligations"])));self.reject("invalid record fields")

    def test_memory_separate_approvals_required(self):
        for lane in ("admission","execution"):
            self.request["owner_authorization"][lane]["approved"]=False;self.reject("authorization required")
            self.request["owner_authorization"][lane]["approved"]=True

    def test_memory_rejects_unfinished_maintenance(self):
        value=json.loads(dependent.reference(self.root,self.maintenance["state"]))
        value["status"]="failed";value.pop("completion_marker");value.update(quality_gate="FAIL",next_increment_readiness="Blocked",head_commit=gate.current_head_commit(self.root))
        self.maintenance["state"]=self.put(closure.encoded(value));self.request["evidence"]["maintenance"]=self.put(closure.encoded(self.maintenance));self.reject("unfinished maintenance")

    def test_memory_rejects_unverified_validator(self):
        name=next(iter(dependent.MAINTENANCE_FILES));(self.root/name).write_text("changed\n")
        self.request["workspace_fingerprint"]=gate.workspace_fingerprint(self.root);self.reject("verified validator drift")

    def test_memory_rejects_frozen_hash_and_candidate_tampering(self):
        self.request["evidence"]["candidate"]["sha256"]="0"*64;self.reject("frozen candidate mismatch")

    def test_memory_rejects_pre_admission_product_drift(self):
        name="src-tauri/src/codex_connection.rs";(self.root/name).write_text("changed\n")
        self.request["workspace_fingerprint"]=gate.workspace_fingerprint(self.root);self.reject("candidate drift before admission")

    def test_memory_rejects_scope_expansion(self):
        self.request["allowed_paths"].append("src-tauri/Cargo.toml");self.reject("scope mismatch")

    def test_memory_post_admission_protected_drift_blocks_stop(self):
        self.admit();(self.root/"src/frozen-0.txt").write_text("changed\n")
        with self.assertRaisesRegex(gate.GateError,"protected implementation drift"):dependent.record(self.root,gate.read_state(self.root))

    def test_memory_required_manual_check_cannot_be_omitted(self):
        self.admit();name=self.review(dependent.MEMORY_SUCCESSOR);p=self.root/name
        m=gate._extract_manifest(p.read_text());m["manual_verification"].pop()
        p.write_text("# Synthetic\n"+gate.MANIFEST_START+json.dumps(m)+gate.MANIFEST_END+"\n"+"\n".join(h+"\nSynthetic"for h in gate.REQUIRED_REPORT_SECTIONS))
        with self.assertRaisesRegex(gate.GateError,"required check omitted"):gate.finalize_gate(self.root,dependent.MEMORY_SUCCESSOR,name)


    def test_memory_requires_bound_actual_stop(self):
        self.maintenance["stop"]=self.put(closure.encoded({"exit_status":2,"decision":"block","state_sha256":"0"*64}))
        self.request["evidence"]["maintenance"]=self.put(closure.encoded(self.maintenance));self.reject("full Stop unverified")

    def test_memory_identity_and_replay_are_closed(self):
        original=copy.deepcopy(self.request)
        for key,value,pattern in [("root","/private/tmp/other","identity mismatch"),("branch","main","identity mismatch"),("head","0"*40,"HEAD mismatch"),("workspace_fingerprint","0"*64,"input drift")]:
            self.request[key]=value;self.reject(pattern);self.request=copy.deepcopy(original)
        self.admit()
        with self.assertRaisesRegex(gate.GateError,"replay"):self.admit()

    def test_memory_snapshot_omission_rejected(self):
        snap=json.loads(dependent.reference(self.root,self.request["evidence"]["snapshot"]))
        snap["files"].pop(next(iter(snap["files"])))
        self.request["evidence"]["snapshot"]=self.put(closure.encoded(snap));self.reject("invalid record fields")

    def test_memory_historical_documents_remain_additive(self):
        self.admit();(self.root/"HANDOFF.md").write_text("Replacement\n")
        with self.assertRaisesRegex(gate.GateError,"historical document rewrite"):
            dependent.record(self.root,gate.read_state(self.root))

    def test_memory_preserved_criteria_cannot_be_waived(self):
        original=copy.deepcopy(self.request)
        key=next(iter(self.request["obligations"]))
        self.request["obligations"][key]["required"]=False;self.reject("omitted or waived")
        self.request=copy.deepcopy(original);self.request["obligations"][key]["criterion"]="preservation";self.reject("misattributed")


    def test_memory_existing_plan_and_other_readiness_cannot_be_reused(self):
        original=copy.deepcopy(self.request)
        self.request["plan"]="docs/plans/original-failed-plan.md";self.reject("plan identity mismatch")
        self.request=original;self.request["readiness_increment"]="other-readiness";self.reject("readiness identity mismatch")

class IndexFormatTests(unittest.TestCase):
    @staticmethod
    def index(*, word=None, flag=0, path=b'a', extension=b''):
        import struct, hashlib
        words = [1, 2, 3, 4, 5, 6, 0o100644, 7, 8, 9]
        if word is not None: words[word] += 1
        entry = struct.pack('>10I20sH', *words, b'a'*20, flag | len(path)) + path + b'\0'
        entry += b'\0' * ((-len(entry)) % 8)
        data = struct.pack('>4sII', b'DIRC', 2, 1) + entry + extension
        return data + hashlib.sha1(data).digest()

    @staticmethod
    def seal(data):
        import hashlib
        return bytes(data[:-20]) + hashlib.sha1(data[:-20]).digest()

    def test_each_cached_stat_field_is_nonsemantic(self):
        base = dependent.index_semantics(self.index())
        for word in (0,1,2,3,4,5,7,8,9):
            with self.subTest(word=word): self.assertEqual(base, dependent.index_semantics(self.index(word=word)))

    def test_paths_object_modes_stages_and_flags_protected(self):
        base = dependent.index_semantics(self.index())
        for raw in [self.index(path=b'b'), self.index(flag=0x1000), self.index(flag=0x8000)]:
            self.assertNotEqual(base, dependent.index_semantics(raw))
        for offset in (12+24,12+40):
            raw=bytearray(self.index());raw[offset+3]^=1
            with self.subTest(offset=offset):
                try: self.assertNotEqual(base,dependent.index_semantics(self.seal(raw)))
                except gate.GateError: pass

    def test_malformed_formats_rejected(self):
        import struct
        base=self.index()
        cases=[b'',base[:-1],base+b'0',self.index(flag=0x4000),self.index(path=b'../bad')]
        for start,value in [(0,b'NOPE'),(4,struct.pack('>I',3)),(8,struct.pack('>I',2)),(12+60,b'\x00\x02'),(len(base)-21,b'\x01')]:
            raw=bytearray(base);raw[start:start+len(value)]=value;cases.append(self.seal(raw))
        cases.append(b'x'*(1024*1024+1))
        for raw in cases:
            with self.subTest(size=len(raw)):
                with self.assertRaises(gate.GateError):dependent.index_semantics(raw)

    def test_extensions_strict_and_byte_protected(self):
        import struct
        payload=b'\0' + b'1 0\n' + b'h'*20
        ext=b'TREE'+struct.pack('>I',len(payload))+payload
        a=dependent.index_semantics(self.index(extension=ext))
        b=dependent.index_semantics(self.index(extension=ext[:-1]+b'z'))
        self.assertNotEqual(a,b)
        for bad in [ext+ext,b'link'+ext[4:],b'sdir'+ext[4:],b'UNKN'+ext[4:],ext[:-1],b'TREE'+struct.pack('>I',3)+b'bad']:
            with self.subTest(length=len(bad)):
                with self.assertRaises(gate.GateError):dependent.index_semantics(self.index(extension=bad))

    def test_duplicate_order_and_invalid_tree_rejected(self):
        import struct,hashlib
        entry=self.index()[12:-20]
        for data in [struct.pack('>4sII',b'DIRC',2,2)+entry+entry,
                     self.index(path=b'b')[0:8]+struct.pack('>I',2)+self.index(path=b'b')[12:-20]+entry]:
            with self.assertRaises(gate.GateError):dependent.index_semantics(data+hashlib.sha1(data).digest())
        for payload in [b'\0' + b'2 0\n'+b'h'*20,b'\0' + b'1 1\n'+b'h'*20,b'root\0' + b'1 0\n'+b'h'*20]:
            with self.assertRaises(gate.GateError):dependent.index_semantics(self.index(extension=b'TREE'+struct.pack('>I',len(payload))+payload))


class MemoryIndexAmendmentTests(MemoryAdmissionTests):
    def install_amendment(self):
        self.admit()
        admission=gate.read_state(self.root)['dependent_lineage']['admission']
        before={n:copy.deepcopy(self.maintenance['files'][n]) for n in dependent.INDEX_FILES}
        for n in dependent.INDEX_FILES:(self.root/n).write_text('Synthetic independently verified amendment.\n')
        manifest=closure.manifest(self.root)
        report={'increment_id':dependent.INDEX_MAINTENANCE,'quality_gate':'PASS',
          'verification':[{'required':True,'status':'Passed'}],
          'manual_verification':[{'required':True,'status':'Passed'}],'findings':[],
          'milestone':{'criteria':{k:{'status':'automatically_verified'} for k in dependent.INDEX_CRITERIA}}}
        raw=(gate.MANIFEST_START+json.dumps(report)+gate.MANIFEST_END).encode()
        state={'schema_version':gate.STATE_SCHEMA_VERSION,'increment_id':dependent.INDEX_MAINTENANCE,
          'status':'complete','quality_gate':'PASS','completion_marker':gate.COMPLETION_MARKER,
          'baseline_fingerprint':'0'*64,'workspace_fingerprint':closure.fingerprint(manifest),
          'report_path':self.report_path(dependent.INDEX_MAINTENANCE),'report_sha256':closure.digest(raw)}
        maintenance={'state':self.put(closure.encoded(state)),'report':self.put(raw),
          'manifest':self.put(closure.encoded(manifest)),
          'files':{n:self.put((self.root/n).read_bytes()) for n in dependent.INDEX_FILES},
          'stop':self.put(closure.encoded({'exit_status':0,'decision':'allow','state_sha256':closure.digest(closure.encoded(state))}))}
        self.amendment={'schema_version':1,'admission':admission,'root':str(self.root),
          'branch':self.request['branch'],'head':self.request['head'],
          'original_index_sha256':self.request['index_sha256'],
          'predecessor_state_sha256':dependent.MEMORY_STATE,'candidate_sha256':dependent.MEMORY_CANDIDATE,
          'authorization':self.put(closure.encoded({'approved':True,'scope':dependent.INDEX_MAINTENANCE,
            'admission':admission,'original_index_sha256':self.request['index_sha256']})),
          'before':before,'maintenance':maintenance}
        self.seal_amendment()

    def seal_amendment(self):
        (self.root/dependent.INDEX_AMENDMENT).write_bytes(closure.encoded({'receipt':self.put(closure.encoded(self.amendment))}))

    def cache_refresh(self):
        p=closure.git_index_path(self.root);data=bytearray(p.read_bytes());data[12:16]=b'\0'*4
        p.write_bytes(IndexFormatTests.seal(data));return p

    def record(self):return dependent.record(self.root,gate.read_state(self.root))

    def test_verified_amendment_accepts_cache_and_preserves_history(self):
        self.install_amendment();old=gate.state_path(self.root).read_bytes();p=self.cache_refresh();current=p.read_bytes()
        result=self.record()
        self.assertEqual(result['request'],self.request)
        self.assertEqual(result['raw_state'].encode(),self.raw)
        self.assertEqual(gate.state_path(self.root).read_bytes(),old)
        self.assertEqual(p.read_bytes(),current)
        self.assertEqual(set(result['verified_maintenance_overlay']),dependent.INDEX_FILES)

    def test_unamended_memory_still_rejects_raw_drift(self):
        self.admit();self.cache_refresh()
        with self.assertRaisesRegex(gate.GateError,'index drift'):self.record()

    def test_staged_changes_rejected_even_with_amendment(self):
        self.install_amendment();(self.root/'src/frozen-0.txt').write_text('Changed staged content\n');self.git('add','src/frozen-0.txt')
        with self.assertRaisesRegex(gate.GateError,'index drift'):self.record()

    def test_amendment_does_not_exempt_other_source(self):
        self.install_amendment();self.cache_refresh();(self.root/'src/frozen-0.txt').write_text('Unrelated changed content\n')
        with self.assertRaisesRegex(gate.GateError,'protected implementation drift'):self.record()

    def test_wrong_amendment_identity_and_authority_rejected(self):
        self.install_amendment();original=copy.deepcopy(self.amendment)
        for key in ['admission','root','branch','head','original_index_sha256','predecessor_state_sha256','candidate_sha256']:
            self.amendment=copy.deepcopy(original);self.amendment[key]='wrong';self.seal_amendment()
            with self.subTest(key=key):
                with self.assertRaises(gate.GateError):self.record()
        self.amendment=original
        auth=json.loads(dependent.reference(self.root,self.amendment['authorization']));auth['approved']=False
        self.amendment['authorization']=self.put(closure.encoded(auth));self.seal_amendment()
        with self.assertRaisesRegex(gate.GateError,'authority'):self.record()

    def test_unfinished_or_invalid_stop_rejected(self):
        self.install_amendment();original=copy.deepcopy(self.amendment)
        s=json.loads(dependent.reference(self.root,self.amendment['maintenance']['state']))
        s.pop('completion_marker');s['status']='active'
        # Structurally valid active state must still fail the completion prerequisite.
        s={k:s[k] for k in ['schema_version','increment_id','status','baseline_fingerprint']}
        self.amendment['maintenance']['state']=self.put(closure.encoded(s));self.seal_amendment()
        with self.assertRaisesRegex(gate.GateError,'incomplete'):self.record()
        self.amendment=original;self.amendment['maintenance']['stop']=self.put(closure.encoded({'exit_status':1,'decision':'block','state_sha256':'0'*64}));self.seal_amendment()
        with self.assertRaisesRegex(gate.GateError,'Stop invalid'):self.record()

    def test_missing_tampered_or_changed_implementation_rejected(self):
        self.install_amendment();original=(self.root/dependent.INDEX_AMENDMENT).read_bytes()
        pointer=json.loads(original);pointer['receipt']['sha256']='0'*64
        (self.root/dependent.INDEX_AMENDMENT).write_bytes(closure.encoded(pointer))
        with self.assertRaisesRegex(gate.GateError,'hash mismatch'):self.record()
        (self.root/dependent.INDEX_AMENDMENT).write_bytes(original)
        (self.root/next(iter(dependent.INDEX_FILES))).write_text('Unverified implementation\n')
        with self.assertRaisesRegex(gate.GateError,'implementation drift'):self.record()

    def test_original_binding_and_extra_overlay_rejected(self):
        self.install_amendment();original=copy.deepcopy(self.amendment)
        name=next(iter(dependent.INDEX_FILES));self.amendment['before'][name]=self.put(b'wrong');self.seal_amendment()
        with self.assertRaisesRegex(gate.GateError,'before drift'):self.record()
        self.amendment=original;self.amendment['maintenance']['files']['src/frozen-0.txt']=self.ref('src/frozen-0.txt');self.seal_amendment()
        with self.assertRaises(gate.GateError):self.record()

    def test_index_lock_and_symlink_rejected(self):
        self.install_amendment();p=closure.git_index_path(self.root);lock=p.with_name('index.lock');lock.write_bytes(b'')
        with self.assertRaisesRegex(gate.GateError,'index lock'):self.record()
        lock.unlink();copy_path=p.with_name('fixture-index');copy_path.write_bytes(p.read_bytes());p.unlink();p.symlink_to(copy_path)
        with self.assertRaises(gate.GateError):self.record()

    def test_read_replacement_race_rejected(self):
        self.install_amendment();p=closure.git_index_path(self.root)
        import os
        original=os.read;changed=False
        def replacing(fd,size):
            nonlocal changed
            data=original(fd,size)
            if not changed:
                changed=True;q=p.with_name('replacement');q.write_bytes(p.read_bytes());os.replace(q,p)
            return data
        with mock.patch.object(dependent.closure,'git_index_path',return_value=p), mock.patch.object(dependent.os,'read',side_effect=replacing):
            with self.assertRaisesRegex(gate.GateError,'index read race'):dependent.stable_index(self.root)


    def test_amended_completion_keeps_original_failure_and_acceptance(self):
        self.install_amendment(); self.cache_refresh()
        name=self.review(dependent.MEMORY_SUCCESSOR); p=self.root/name
        text=p.read_text(); report=gate._extract_manifest(text)
        for key in dependent.INDEX_FILES: report['milestone']['paths'].pop(key)
        start=text.index(gate.MANIFEST_START)+len(gate.MANIFEST_START); end=text.index(gate.MANIFEST_END)
        p.write_text(text[:start]+json.dumps(report)+text[end:])
        gate.finalize_gate(self.root,dependent.MEMORY_SUCCESSOR,name)
        self.assertTrue(gate.validate_completed_state(self.root,gate.read_state(self.root)))
        self.assertEqual(self.record()['raw_state'].encode(),self.raw)

    def test_amended_unverified_C7_still_blocks_finalization(self):
        self.install_amendment(); self.cache_refresh()
        name=self.review(dependent.MEMORY_SUCCESSOR,failed=True);p=self.root/name
        text=p.read_text();report=gate._extract_manifest(text)
        for key in dependent.INDEX_FILES:report['milestone']['paths'].pop(key)
        start=text.index(gate.MANIFEST_START)+len(gate.MANIFEST_START);end=text.index(gate.MANIFEST_END)
        p.write_text(text[:start]+json.dumps(report)+text[end:])
        with self.assertRaises(gate.GateError):gate.finalize_gate(self.root,dependent.MEMORY_SUCCESSOR,name)
        self.assertEqual(gate.read_state(self.root)['status'],'active')


class CancellationAmendmentTests(MemoryIndexAmendmentTests):
    """Synthetic exact bindings, never the real admission or acceptance data."""

    def patch_constant(self, name, value):
        patch = mock.patch.object(dependent, name, value)
        patch.start()
        self.addCleanup(patch.stop)

    def cput(self, data):
        sha = closure.digest(data)
        name = dependent.CANCELLATION_STORE / (sha + '.blob')
        path = self.root / name
        if path.exists():
            self.assertEqual(path.read_bytes(), data)
        else:
            dependent.exclusive(self.root, name, data)
        return {'path': name.as_posix(), 'sha256': sha}

    def cjson(self, value):
        return self.cput(closure.encoded(value))

    def prepare_cancellation(self, *, publish=True):
        role = dependent.CANCELLATION_ROLE
        path = self.root / role
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(b'Synthetic original cancellation host.\n')
        self.patch_constant('CANCELLATION_BEFORE', closure.digest(path.read_bytes()))
        self.review(dependent.MEMORY_SUCCESSOR + '-readiness')
        self.request['input_bindings'] = {n: v for n, v in closure.manifest(self.root).items()
                                          if n not in self.request['allowed_paths']}
        self.request['workspace_fingerprint'] = gate.workspace_fingerprint(self.root)
        self.install_amendment()
        admission = gate.read_state(self.root)['dependent_lineage']['admission']
        self.patch_constant('CANCELLATION_ADMISSION', admission)
        self.patch_constant('CANCELLATION_HEAD', self.request['head'])
        before = {n: self.cput((self.root / n).read_bytes()) for n in dependent.INDEX_FILES}
        for n in dependent.INDEX_FILES:
            (self.root / n).write_text('Synthetic cancellation maintenance.\n')
        path.write_bytes(b'Synthetic accepted cancellation host.\n')
        self.patch_constant('CANCELLATION_AFTER', closure.digest(path.read_bytes()))
        evidence = {role: self.cjson({'synthetic': role}) for role in dependent.CANCELLATION_EVIDENCE}
        self.patch_constant('CANCELLATION_EVIDENCE', {k: v['sha256'] for k, v in evidence.items()})
        after = {n: self.cput((self.root / n).read_bytes()) for n in dependent.INDEX_FILES}
        bindings = {n: v['sha256'] for n, v in after.items()}
        checks = {label: self.cjson({
            'schema_version': 1, 'command': ['/usr/bin/python3', '-B'] + args,
            'cwd': str(self.root), 'exit_status': 0, 'timeout_seconds': 600,
            'elapsed_seconds': 1.0, 'timed_out': False, 'complete_capture': True, 'files': bindings,
        }) for label, args in dependent.CANCELLATION_CHECKS.items()}
        review = self.cjson({'schema_version': 1, 'scope': 'memory-cancellation-finalization',
            'result': 'PASS', 'files': bindings, 'checks': checks, 'historical_failure_preserved': True,
            'original_admission_preserved': True, 'unrelated_routes_unchanged': True,
            'product_execution_reused': True})
        self.cancel = {'schema_version': 1, 'admission': admission, 'root': str(self.root),
            'branch': self.request['branch'], 'head': self.request['head'], 'role': role,
            'before_sha256': dependent.CANCELLATION_BEFORE, 'after_sha256': dependent.CANCELLATION_AFTER,
            'authorization': self.cjson({'approved': True, 'scope': 'memory-cancellation-finalization',
                'admission': admission, 'proposal_sha256': dependent.CANCELLATION_PROPOSAL,
                'historical_continuity_claimed': False}),
            'evidence': evidence, 'maintenance': {'before': before, 'after': after, 'checks': checks, 'review': review}}
        self.cancel_ref = self.cjson(self.cancel)
        if publish:
            dependent.publish_cancellation_amendment(self.root, self.cancel_ref)

    def reseal_cancel(self):
        self.cancel_ref = self.cjson(self.cancel)
        (self.root / dependent.CANCELLATION_POINTER).write_bytes(closure.encoded({'receipt': self.cancel_ref}))

    def test_cancellation_accepts_only_exact_role_and_preserves_originals(self):
        self.prepare_cancellation()
        old_state = gate.state_path(self.root).read_bytes()
        old_index = closure.git_index_path(self.root).read_bytes()
        rec = self.record()
        self.assertEqual(rec['request'], self.request)
        self.assertNotIn(dependent.CANCELLATION_ROLE, rec['request']['allowed_paths'])
        self.assertEqual(rec['raw_state'].encode(), self.raw)
        self.assertEqual(gate.state_path(self.root).read_bytes(), old_state)
        self.assertEqual(closure.git_index_path(self.root).read_bytes(), old_index)
        self.assertEqual(set(rec['verified_maintenance_overlay']), dependent.INDEX_FILES)

    def test_cancellation_completion_requires_original_criteria_and_full_stop(self):
        self.prepare_cancellation()
        name = self.review(dependent.MEMORY_SUCCESSOR)
        path = self.root / name
        text = path.read_text()
        report = gate._extract_manifest(text)
        for n in dependent.INDEX_FILES:
            report['milestone']['paths'].pop(n)
        self.assertIn(dependent.CANCELLATION_ROLE, report['milestone']['paths'])
        start = text.index(gate.MANIFEST_START) + len(gate.MANIFEST_START)
        end = text.index(gate.MANIFEST_END)
        path.write_text(text[:start] + json.dumps(report) + text[end:])
        gate.finalize_gate(self.root, dependent.MEMORY_SUCCESSOR, name)
        self.assertTrue(gate.validate_completed_state(self.root, gate.read_state(self.root)))
        self.assertFalse(gate.evaluate_stop_payload({'cwd': str(self.root), 'hook_event_name': 'Stop',
                                                    'stop_hook_active': False}).should_continue)
        self.assertEqual(self.record()['raw_state'].encode(), self.raw)

    def test_cancellation_rejects_identity_source_and_extra_role_changes(self):
        self.prepare_cancellation()
        original = copy.deepcopy(self.cancel)
        for key in ('admission', 'root', 'branch', 'head', 'role', 'before_sha256', 'after_sha256'):
            with self.subTest(key=key):
                self.cancel = copy.deepcopy(original)
                self.cancel[key] = 'wrong'
                self.reseal_cancel()
                with self.assertRaises(gate.GateError): self.record()
        self.cancel = original
        self.cancel['extra_role'] = 'src/frozen-0.txt'
        self.reseal_cancel()
        with self.assertRaises(gate.GateError): self.record()

    def test_cancellation_rejects_missing_or_ambiguous_authority(self):
        self.prepare_cancellation()
        auth = json.loads(dependent.cancellation_reference(self.root, self.cancel['authorization']))
        for key, value in [('approved', False), ('approved', 1), ('proposal_sha256', '0'*64),
                           ('historical_continuity_claimed', True)]:
            row = dict(auth)
            row[key] = value
            self.cancel['authorization'] = self.cjson(row)
            self.reseal_cancel()
            with self.assertRaises(gate.GateError): self.record()

    def test_cancellation_rejects_stale_or_failed_checks_and_review(self):
        self.prepare_cancellation()
        original = copy.deepcopy(self.cancel)
        for label in dependent.CANCELLATION_CHECKS:
            for key, value in [('exit_status', 1), ('exit_status', False), ('complete_capture', False),
                               ('timed_out', True), ('files', {}), ('command', []), ('elapsed_seconds', -1)]:
                self.cancel = copy.deepcopy(original)
                row = json.loads(dependent.cancellation_reference(self.root, self.cancel['maintenance']['checks'][label]))
                row[key] = value
                self.cancel['maintenance']['checks'][label] = self.cjson(row)
                self.reseal_cancel()
                with self.assertRaises(gate.GateError): self.record()
        self.cancel = original
        row = json.loads(dependent.cancellation_reference(self.root, self.cancel['maintenance']['review']))
        row['result'] = 'FAIL'
        self.cancel['maintenance']['review'] = self.cjson(row)
        self.reseal_cancel()
        with self.assertRaises(gate.GateError): self.record()

    def test_cancellation_preserves_strict_other_input_and_current_source_checks(self):
        self.prepare_cancellation()
        for name in [dependent.CANCELLATION_ROLE, 'src/frozen-0.txt', *sorted(dependent.INDEX_FILES)]:
            path = self.root / name
            before = path.read_bytes()
            path.write_bytes(before + b'Drift\n')
            with self.subTest(name=name):
                with self.assertRaises(gate.GateError): self.record()
            path.write_bytes(before)
        self.record()

    def test_cancellation_requires_exact_old_and_new_maintenance_bytes(self):
        self.prepare_cancellation()
        original = copy.deepcopy(self.cancel)
        for group in ('before', 'after'):
            for name in dependent.INDEX_FILES:
                self.cancel = copy.deepcopy(original)
                self.cancel['maintenance'][group][name] = self.cput(b'Other implementation\n')
                self.reseal_cancel()
                with self.assertRaises(gate.GateError): self.record()

    def test_cancellation_missing_tampered_and_symlink_evidence_fail_closed(self):
        self.prepare_cancellation()
        path = self.root / self.cancel['evidence']['acceptance']['path']
        before = path.read_bytes()
        path.write_bytes(b'tampered')
        with self.assertRaises(gate.GateError): self.record()
        path.unlink()
        with self.assertRaises(gate.GateError): self.record()
        target = self.root / 'synthetic-copy'
        target.write_bytes(before)
        path.symlink_to(target)
        with self.assertRaises(gate.GateError): self.record()

    def test_cancellation_partial_publication_replay_and_unsupported_state(self):
        self.prepare_cancellation(publish=False)
        with self.assertRaisesRegex(gate.GateError, 'partial publication'): self.record()
        dependent.publish_cancellation_amendment(self.root, self.cancel_ref)
        with self.assertRaisesRegex(gate.GateError, 'replay'):
            dependent.publish_cancellation_amendment(self.root, self.cancel_ref)
        self.record()
        with mock.patch.object(dependent.gate, 'read_state', return_value={'status': 'failed'}):
            (self.root / dependent.CANCELLATION_POINTER).unlink()
            with self.assertRaisesRegex(gate.GateError, 'active state'):
                dependent.publish_cancellation_amendment(self.root, self.cancel_ref)

    def test_cancellation_descriptor_replacement_with_same_bytes_rejected(self):
        self.prepare_cancellation()
        path = self.root / dependent.CANCELLATION_ROLE
        original = dependent.os.read
        replaced = False
        def replacing(fd, size):
            nonlocal replaced
            data = original(fd, size)
            if not replaced:
                replaced = True
                other = path.with_name('synthetic-replacement')
                other.write_bytes(path.read_bytes())
                dependent.os.replace(other, path)
            return data
        with mock.patch.object(dependent.os, 'read', side_effect=replacing):
            with self.assertRaisesRegex(gate.GateError, 'read race'):
                dependent.cancellation_bytes(self.root, dependent.CANCELLATION_ROLE)

    def test_cancellation_extra_scope_and_incomplete_native_acceptance_rejected(self):
        self.prepare_cancellation()
        extra = self.root / 'src/new-unapproved.txt'
        extra.write_text('Not authorized\n')
        with self.assertRaisesRegex(gate.GateError, 'scope drift'): self.record()
        extra.unlink()
        name = self.review(dependent.MEMORY_SUCCESSOR, failed=True)
        path = self.root / name
        text = path.read_text()
        report = gate._extract_manifest(text)
        for n in dependent.INDEX_FILES: report['milestone']['paths'].pop(n)
        start = text.index(gate.MANIFEST_START) + len(gate.MANIFEST_START)
        end = text.index(gate.MANIFEST_END)
        path.write_text(text[:start] + json.dumps(report) + text[end:])
        with self.assertRaises(gate.GateError): gate.finalize_gate(self.root, dependent.MEMORY_SUCCESSOR, name)
