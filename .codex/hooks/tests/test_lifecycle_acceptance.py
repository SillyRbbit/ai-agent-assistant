from __future__ import annotations

import copy
import hashlib
import json
import sys
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import lifecycle_acceptance as acceptance
import test_post_increment_gate as legacy
gate = legacy.gate


class EvidenceAcceptanceTests(unittest.TestCase):
    def setUp(self):
        self.fixture = legacy.PostIncrementGateTests(methodName="runTest")
        self.fixture.setUp()
        self.root = self.fixture.root
        for name in acceptance.MAINTENANCE_FILES:
            p = self.root / name; p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text("# Original " + name + "\n")
        self.failed_report = self.fixture._write_report(manual_status="Manual verification pending", quality_gate="FAIL", readiness="Blocked", findings=[self.fixture._finding()])
        gate.close_failed_gate(self.root, "04g", self.failed_report)
        self.raw = gate.state_path(self.root).read_bytes()
        self.before = acceptance.inventory(self.root)
        self.history = [{"state": self.put(self.raw), "report": self.put((self.root/self.failed_report).read_bytes()), "manifest": self.put(acceptance.encode(self.before))}]
        for name, row in self.before.items():
            if row["kind"] == "file": self.put((self.root/name).read_bytes())
        self.plan = "docs/plans/2026-10-03-evidence-bound-acceptance-maintenance.md"
        self.report = "docs/reviews/2026-10-03-evidence-bound-acceptance-maintenance-post-increment-review.md"
        self.scope = sorted(set(acceptance.MAINTENANCE_FILES) | {self.plan,self.report})
        for name in acceptance.MAINTENANCE_FILES:
            p=self.root/name;p.write_text("# Maintenance addition\n"+p.read_text())
        p=self.root/self.plan;p.parent.mkdir(parents=True,exist_ok=True);p.write_text("# Plan\n")
        self.successor={"increment_id":"sidebar-acceptance", "plan":"docs/plans/2026-10-03-sidebar-acceptance.md", "report":"docs/reviews/2026-10-03-sidebar-acceptance-post-increment-review.md"}
        self.successor["allowed_paths"]=sorted(acceptance.document_scope(**{"increment":self.successor["increment_id"],"plan":self.successor["plan"],"report":self.successor["report"]}))
        self.write_report(self.report,acceptance.MAINTENANCE)
        evidence=self.put(b"Synthetic reviewed native and automated receipt; no application launch")
        bundle=self.root/"src-tauri/target/test/QA.app"
        exe=bundle/"Contents/MacOS/ai-agent-assistant";exe.parent.mkdir(parents=True);exe.write_bytes(b"synthetic executable")
        with (self.root/".git/info/exclude").open("a") as f:f.write("\nsrc-tauri/target/\n")
        artifact_receipt=self.put(acceptance.encode({"bundle":str(bundle),"executable":str(exe),"executable_sha256":acceptance.digest(exe.read_bytes()),"files":{"Contents/MacOS/ai-agent-assistant":acceptance.digest(exe.read_bytes())},"source_files":{n:v["sha256"] for n,v in self.before.items()}}))
        transfer_receipt=self.put(acceptance.encode({"destination":str(self.root),"head":gate.current_head_commit(self.root),"all_source_files":{n:v["sha256"] for n,v in self.before.items()}}))
        binding={"source_manifest":{n:v for n,v in self.before.items() if n not in self.scope},"artifact_sha256":acceptance.digest(exe.read_bytes()),"artifact_receipt":artifact_receipt,"transfer_receipt":transfer_receipt,"evidence":[evidence,artifact_receipt,transfer_receipt]}
        index=self.root/gate._run_git(self.root,"rev-parse","--git-path","index").decode().strip()
        self.request={"schema_version":1,"increment_id":acceptance.MAINTENANCE,"root":str(self.root),"head":gate.current_head_commit(self.root),"index_sha256":acceptance.digest(index.read_bytes()),"history":self.history,"current_failure":acceptance.digest(self.raw),"before_manifest":self.history[0]["manifest"],"allowed_paths":self.scope,"plan":self.plan,"report":self.report,"owner_authorization":self.auth("maintenance-only"),"evidence":{"native":evidence,"artifact":artifact_receipt,"transfer":transfer_receipt},"artifact_binding":self.put(acceptance.encode(binding)),"successor":self.successor}
        self.request_path=".codex/state/maintenance-request.json"
        self.write_request()
        self.write_report(self.report,acceptance.MAINTENANCE)

    def tearDown(self): self.fixture.tearDown()

    def put(self,data):
        identity=acceptance.digest(data);acceptance.atomic(self.root,acceptance.STORE+"/blobs/"+identity,data);return identity

    def auth(self,purpose):
        return {"approved":True,"purpose":purpose,"source":"Synthetic owner instruction", "decision_sha256":acceptance.digest((self.root/"DECISIONS.md").read_bytes())}

    def write_request(self):
        (self.root/self.request_path).write_bytes(acceptance.encode(self.request))

    def write_report(self,path,increment,failed=False):
        p=self.root/path;p.parent.mkdir(parents=True,exist_ok=True);p.write_text("placeholder")
        commands=sorted(acceptance.REQUIRED_COMMANDS)
        manifest={"schema_version":1,"increment_id":increment,"quality_gate":"FAIL" if failed else "PASS WITH ADVISORIES","next_increment_readiness":"Blocked" if failed else "Ready with advisories","files_changed":list(gate.changed_paths(self.root)),"commands_executed":commands,"verification":[{"command":c,"required":True,"status":"Passed"} for c in commands],"manual_verification":[{"check":c,"required":True,"status":"Failed" if failed else "Passed"} for c in sorted(acceptance.REQUIRED_REVIEWS)],"findings":[]}
        body="# Review\n\n"+gate.MANIFEST_START+json.dumps(manifest,indent=2)+gate.MANIFEST_END+"\n"
        for section in gate.REQUIRED_REPORT_SECTIONS:body+=section+"\n\nSynthetic reviewed evidence.\n\n"
        p.write_text(body)

    def seal(self):return acceptance.seal(self.root,self.request_path)

    def admission(self):
        identity=self.seal();rec=acceptance.maintenance(self.root)
        req={"schema_version":1,"increment_id":self.successor["increment_id"],"maintenance":identity,"workspace_fingerprint":gate.workspace_fingerprint(self.root),"owner_authorization":self.auth("acceptance-only"),"resolutions":{issue:{"status":"resolved","evidence":["native"],"rationale":"Synthetic criterion-specific reviewer mapping","reviewed_by_owner":True} for issue in rec["issues"]}}
        name=".codex/state/admission-request.json";(self.root/name).write_bytes(acceptance.encode(req));return name,req

    def reject_seal(self):
        self.write_request()
        with self.assertRaises(gate.GateError):self.seal()
        self.assertEqual(gate.state_path(self.root).read_bytes(),self.raw)
        self.assertFalse((self.root/acceptance.POINTER).exists())

    def test_seal_keeps_failure_raw_and_allows_truthful_stop(self):
        identity=self.seal();self.assertEqual(self.seal(),identity)
        self.assertEqual(gate.state_path(self.root).read_bytes(),self.raw)
        status=gate.redacted_status(self.root)
        self.assertEqual(status["quality_gate"],"FAIL");self.assertTrue(status["valid"])
        self.assertFalse(status["acceptance_maintenance"]["original_workspace_matches"])
        self.assertFalse(status["acceptance_maintenance"]["acceptance_started"])
        self.assertFalse(gate.evaluate_stop_payload(self.fixture._stop_payload()).should_continue)
        with self.assertRaises(gate.GateError):gate.begin_gate(self.root,"other")
        with self.assertRaises(gate.GateError):gate.finalize_gate(self.root,"04g",self.failed_report)

    def test_missing_authorization_and_wrong_purpose_rejected(self):
        for field,value in [("approved",False),("purpose","acceptance-only"),("decision_sha256","0"*64)]:
            original=copy.deepcopy(self.request);self.request["owner_authorization"][field]=value;self.reject_seal();self.request=original

    def test_unknown_duplicate_and_malformed_fields(self):
        self.request["force"]=True;self.reject_seal();del self.request["force"]
        (self.root/self.request_path).write_text('{"schema_version":1,"schema_version":1}')
        with self.assertRaises(gate.GateError):self.seal()
        self.request["history"]=[None];self.reject_seal()

    def test_head_index_state_report_and_blob_tamper(self):
        for key in ["head","index_sha256","current_failure"]:
            old=self.request[key];self.request[key]="0"*len(old);self.reject_seal();self.request[key]=old
        blob=self.root/acceptance.STORE/"blobs"/self.history[0]["report"];blob.write_bytes(b"changed");self.reject_seal()

    def test_missing_artifact_binding_and_candidate_drift(self):
        old=self.request["artifact_binding"];self.request["artifact_binding"]="0"*64;self.reject_seal();self.request["artifact_binding"]=old
        (self.root/"change.txt").write_text("product drift");self.reject_seal()

    def test_scope_duplicate_extra_and_historical_body_rejected(self):
        old=self.request["allowed_paths"][:];self.request["allowed_paths"].append("src/main.rs");self.reject_seal();self.request["allowed_paths"]=old
        (self.root/"HANDOFF.md").write_text("history replaced");self.reject_seal()

    def test_report_required_verification_not_weakened(self):
        p=self.root/self.report;p.write_text(p.read_text().replace('"Passed"','"Not run"',1));self.reject_seal()

    def test_safe_paths_and_symlinks(self):
        for name in ["../outside","/absolute",".codex/state/../x","a//b"]:
            with self.assertRaises(gate.GateError):acceptance.read(self.root,name)
        p=self.root/acceptance.STORE/"blobs"/self.request["evidence"]["native"];p.unlink();p.symlink_to(self.root/"README.md");self.reject_seal()

    def test_interrupted_seal_does_not_touch_failure_and_replay_succeeds(self):
        actual=acceptance.atomic
        def interrupt(root,name,data):
            if name==acceptance.POINTER:raise OSError("synthetic interrupted publication")
            return actual(root,name,data)
        with mock.patch.object(acceptance,"atomic",side_effect=interrupt):
            with self.assertRaises(gate.GateError):self.seal()
        self.assertEqual(gate.state_path(self.root).read_bytes(),self.raw);self.seal()

    def test_changed_seal_replay_rejected(self):
        self.seal();self.request["owner_authorization"]["source"]="changed approval";self.write_request()
        with self.assertRaises(gate.GateError):self.seal()
        self.assertEqual(gate.state_path(self.root).read_bytes(),self.raw)

    def test_sealed_drift_fails_status_and_stop(self):
        self.seal();(self.root/"change.txt").write_text("drift")
        self.assertFalse(gate.redacted_status(self.root)["valid"])
        self.assertTrue(gate.evaluate_stop_payload(self.fixture._stop_payload()).should_continue)

    def test_complete_resolution_and_separate_approval_required(self):
        name,req=self.admission();req["resolutions"].pop(next(iter(req["resolutions"])));(self.root/name).write_bytes(acceptance.encode(req))
        with self.assertRaises(gate.GateError):gate.begin_gate(self.root,self.successor["increment_id"],name)
        self.assertEqual(gate.state_path(self.root).read_bytes(),self.raw)

    def test_deferred_and_unknown_evidence_rejected(self):
        name,req=self.admission();issue=next(iter(req["resolutions"]))
        for key,value in [("status","deferred"),("reviewed_by_owner",False),("evidence",["missing"])]:
            modified=copy.deepcopy(req);modified["resolutions"][issue][key]=value;(self.root/name).write_bytes(acceptance.encode(modified))
            with self.assertRaises(gate.GateError):gate.begin_gate(self.root,self.successor["increment_id"],name)

    def test_wrong_successor_and_approval_rejected(self):
        name,req=self.admission()
        with self.assertRaises(gate.GateError):gate.begin_gate(self.root,"other",name)
        req["owner_authorization"]["purpose"]="maintenance-only";(self.root/name).write_bytes(acceptance.encode(req))
        with self.assertRaises(gate.GateError):gate.begin_gate(self.root,self.successor["increment_id"],name)

    def test_interrupted_admission_and_identical_active_replay(self):
        name,req=self.admission()
        with mock.patch.object(gate,"write_state",side_effect=OSError("interrupted")):
            with self.assertRaises(gate.GateError):gate.begin_gate(self.root,self.successor["increment_id"],name)
        self.assertEqual(gate.state_path(self.root).read_bytes(),self.raw)
        gate.begin_gate(self.root,self.successor["increment_id"],name);state=gate.state_path(self.root).read_bytes()
        gate.begin_gate(self.root,self.successor["increment_id"],name);self.assertEqual(state,gate.state_path(self.root).read_bytes())

    def test_successor_normal_completion_retains_immutable_history(self):
        name,_=self.admission();inc=self.successor["increment_id"];gate.begin_gate(self.root,inc,name)
        (self.root/self.successor["plan"]).write_text("# Acceptance plan\n")
        for doc in acceptance.ROOT_DOCS:
            p=self.root/doc;p.write_text("# Acceptance addition\n"+p.read_text())
        self.write_report(self.successor["report"],inc)
        gate.finalize_gate(self.root,inc,self.successor["report"])
        state=gate.read_state(self.root);self.assertEqual(state["schema_version"],4);self.assertEqual(state["status"],"complete")
        self.assertFalse(gate.redacted_status(self.root)["acceptance_maintenance"]["original_workspace_matches"])
        self.assertTrue(gate.validate_completed_state(self.root,state));self.assertEqual(acceptance.blob(self.root,self.history[0]["state"]),self.raw)
        self.assertFalse(gate.evaluate_stop_payload(self.fixture._stop_payload()).should_continue)

    def test_successor_product_drift_and_document_rewrite_block_completion(self):
        name,_=self.admission();inc=self.successor["increment_id"];gate.begin_gate(self.root,inc,name)
        p=self.root/"change.txt";old=p.read_bytes();p.write_bytes(b"drift")
        with self.assertRaises(gate.GateError):acceptance.validate_lineage(self.root,gate.read_state(self.root))
        p.write_bytes(old);(self.root/"HANDOFF.md").write_text("replacement")
        with self.assertRaises(gate.GateError):acceptance.validate_lineage(self.root,gate.read_state(self.root))

    def test_successor_failure_cannot_readmit_or_promote(self):
        name,_=self.admission();inc=self.successor["increment_id"];gate.begin_gate(self.root,inc,name)
        self.write_report(self.successor["report"],inc,failed=True);gate.close_failed_gate(self.root,inc,self.successor["report"])
        state=gate.read_state(self.root);self.assertEqual(state["status"],"failed");self.assertIn("acceptance_lineage",state);self.assertNotIn("completion_marker",state)
        self.assertTrue(gate.validate_failed_state(self.root,state))
        with self.assertRaises(gate.GateError):gate.begin_gate(self.root,inc,name)
        with self.assertRaises(gate.GateError):gate.finalize_gate(self.root,inc,self.successor["report"])

    def test_file_mode_and_type_drift_rejected(self):
        p=self.root/"change.txt";p.chmod(0o755);self.reject_seal()
        p.chmod(0o644);p.unlink();p.symlink_to("README.md");self.reject_seal()

    def test_atomic_collision_never_overwrites(self):
        name=acceptance.STORE+"/record.json";acceptance.atomic(self.root,name,b"original")
        with self.assertRaises(gate.GateError):acceptance.atomic(self.root,name,b"changed")
        self.assertEqual((self.root/name).read_bytes(),b"original")

    def test_missing_history_after_seal_blocks_stop(self):
        self.seal();(self.root/acceptance.STORE/"blobs"/self.history[0]["state"]).unlink()
        self.assertFalse(gate.redacted_status(self.root)["valid"])
        self.assertTrue(gate.evaluate_stop_payload(self.fixture._stop_payload()).should_continue)

    def test_artifact_bytes_and_source_binding_are_required(self):
        exe=self.root/"src-tauri/target/test/QA.app/Contents/MacOS/ai-agent-assistant"
        exe.write_bytes(b"replaced");self.reject_seal()

    def test_sealed_history_cannot_be_reclosed(self):
        self.seal()
        with self.assertRaises(gate.GateError):gate.close_failed_gate(self.root,"04g",self.failed_report)
        self.assertEqual(gate.state_path(self.root).read_bytes(),self.raw)

    def test_artifact_drift_after_seal_blocks_status_and_stop(self):
        self.seal()
        (self.root/"src-tauri/target/test/QA.app/Contents/MacOS/ai-agent-assistant").write_bytes(b"drift")
        self.assertFalse(gate.redacted_status(self.root)["valid"])
        self.assertTrue(gate.evaluate_stop_payload(self.fixture._stop_payload()).should_continue)

    def test_wrong_artifact_digest_and_missing_source_rejected(self):
        b=acceptance.parse(acceptance.blob(self.root,self.request["artifact_binding"]))
        b["artifact_sha256"]="0"*64
        self.request["artifact_binding"]=self.put(acceptance.encode(b));self.reject_seal()

    def test_supplement_cannot_invent_missing_asset_or_override_source(self):
        binding=acceptance.parse(acceptance.blob(self.root,self.request["artifact_binding"]))
        product=binding["source_manifest"].copy()
        product["assets/unbound.png"]={"kind":"file","mode":0,"sha256":"b"*64}
        binding["source_manifest"]=product
        with self.assertRaises(gate.GateError):acceptance.artifact(self.root,binding,self.request["evidence"],product)

    def test_transfer_receipt_drift_is_rejected(self):
        binding=acceptance.parse(acceptance.blob(self.root,self.request["artifact_binding"]))
        (self.root/acceptance.STORE/"blobs"/binding["transfer_receipt"]).write_bytes(b"drift")
        self.reject_seal()

    def test_duplicate_scope_and_mixed_lineage_rejected(self):
        self.request["allowed_paths"][-1]=self.request["allowed_paths"][0];self.reject_seal()
        with self.assertRaises(gate.GateError):
            acceptance.shape({"acceptance_lineage":{"maintenance":"a"*64,"admission":"b"*64},"predecessor_disposition":{}})

    def test_active_admission_cannot_replay_after_document_work(self):
        name,_=self.admission();inc=self.successor["increment_id"];gate.begin_gate(self.root,inc,name)
        p=self.root/"HANDOFF.md";p.write_text("New additive note\n"+p.read_text())
        with self.assertRaises(gate.GateError):gate.begin_gate(self.root,inc,name)

    def test_artifact_inventory_cannot_escape_bundle_or_follow_symlink(self):
        b=acceptance.parse(acceptance.blob(self.root,self.request["artifact_binding"]))
        receipt=acceptance.parse(acceptance.blob(self.root,b["artifact_receipt"]))
        receipt["files"]["../outside"]="a"*64
        identity=self.put(acceptance.encode(receipt));self.request["evidence"]["artifact"]=identity
        b["evidence"].remove(b["artifact_receipt"]);b["evidence"].append(identity);b["artifact_receipt"]=identity
        self.request["artifact_binding"]=self.put(acceptance.encode(b));self.reject_seal()
