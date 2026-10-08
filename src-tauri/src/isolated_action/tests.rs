use super::workspace::Workspace;
use super::*;
use std::{fs, path::Path, process::Command};
type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;
fn git(root: &Path, args: &[&str]) -> TestResult {
    let status = Command::new("/usr/bin/git")
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .args([
            "-c",
            "user.name=QA Fixture",
            "-c",
            "user.email=qa@example.invalid",
            "-c",
            "core.hooksPath=/dev/null",
            "-c",
            "commit.gpgsign=false",
        ])
        .current_dir(root)
        .args(args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()?;
    assert!(status.success());
    Ok(())
}
fn fixture(root: &Path) -> TestResult {
    fs::create_dir(root)?;
    git(root, &["init", "-q"])?;
    fs::write(root.join("baseline.txt"), "Retain this original file.\n")?;
    fs::write(root.join("test_solution.py"),"import unittest\nfrom solution import add\nclass Test(unittest.TestCase):\n def test_add(self): self.assertEqual(add(2,3),5)\n")?;
    git(root, &["add", "baseline.txt", "test_solution.py"])?;
    git(root, &["commit", "-qm", "Synthetic baseline"])
}
fn passed(w: &Workspace) -> std::result::Result<Evidence, Box<dyn std::error::Error>> {
    let mut e = w.evidence()?;
    let content = "def add(a,b): return a+b\n";
    let h = hash(content.as_bytes());
    w.stage(content, 0)?;
    e.attempts.push(ActionAttempt {
        candidate: content.into(),
        candidate_hash: h.clone(),
        diff: diff(&e.file, &e.original, content),
        qa_summary: "Synthetic regression, not live QA".into(),
        qa_handoff: Some(crate::collaboration::Handoff {
            version: 1,
            stage: 1,
            agent_id: "qa-validation".into(),
            status: "complete".into(),
            summary: "Synthetic fixture".into(),
            findings: vec![],
            evidence: vec![],
            limitations: vec![],
        }),
        check: Check {
            command: "synthetic test double".into(),
            exit: Some(0),
            output: "".into(),
            passed: true,
            image: IMAGE.into(),
            candidate_hash: h,
            test_hash: hash(w.context()?.as_bytes()),
            container_id: "a".repeat(64),
            process_absent: true,
        },
    });
    e.review_hash = Some(e.review_hash()?);
    Ok(e)
}
#[test]
fn edit_contract_rejects_scope_injection_unknown_and_stale_fields() -> TestResult {
    let before = "x=1\n";
    let valid = serde_json::json!({"version":1,"file":"solution.py","baseHash":hash(before.as_bytes()),"content":"x=2\n"});
    assert!(Edit::parse(&valid.to_string(), "solution.py", before).is_ok());
    for name in [
        "../x.py",
        "/tmp/x.py",
        "a/b.py",
        ".x.py",
        "x;id.py",
        "x..py",
    ] {
        assert!(file_name(name).is_err());
    }
    for key in ["command", "environment", "approved"] {
        let mut v = valid.clone();
        v[key] = serde_json::json!("anything");
        assert!(Edit::parse(&v.to_string(), "solution.py", before).is_err());
    }
    assert!(Edit::parse(&valid.to_string(), "solution.py", "drift").is_err());
    assert!(text(b"secret\x1b[2J").is_err());
    assert!(text(&vec![b'a'; 8193]).is_err());
    Ok(())
}
#[test]
fn exact_add_preserves_baseline_and_rejects_replay() -> TestResult {
    let t = tempfile::tempdir()?;
    let root = t.path().canonicalize()?.join("repo");
    fixture(&root)?;
    let w = Workspace::prepare(
        root.clone(),
        "solution.py".into(),
        "test_solution.py".into(),
        t.path().join("action"),
    )?;
    let e = passed(&w)?;
    let review = e.review_hash()?;
    assert!(w.apply(&e, w.test_grant(&"b".repeat(64))).is_err());
    assert!(!root.join("solution.py").exists());
    w.apply(&e, w.test_grant(&review))?;
    assert_eq!(
        fs::read_to_string(root.join("solution.py"))?,
        e.attempts[0].candidate
    );
    assert_eq!(
        fs::read_to_string(root.join("baseline.txt"))?,
        "Retain this original file.\n"
    );
    assert!(w.area.join("baseline.json").exists());
    assert!(w.area.join("apply-complete.json").is_file());
    assert!(w.apply(&e, w.test_grant(&review)).is_err());
    Ok(())
}
#[test]
fn target_index_branch_file_and_untracked_drift_are_rejected() -> TestResult {
    for mode in ["file", "index", "branch", "extra", "test", "backup"] {
        let t = tempfile::tempdir()?;
        let root = t.path().canonicalize()?.join("repo");
        fixture(&root)?;
        let w = Workspace::prepare(
            root.clone(),
            "solution.py".into(),
            "test_solution.py".into(),
            t.path().join("action"),
        )?;
        let e = passed(&w)?;
        match mode {
            "file" => fs::write(root.join("solution.py"), "owner edit\n")?,
            "index" => {
                fs::write(root.join("baseline.txt"), "staged owner edit\n")?;
                git(&root, &["add", "baseline.txt"])?;
                fs::write(root.join("baseline.txt"), "Retain this original file.\n")?;
            }
            "branch" => git(&root, &["checkout", "-qb", "changed"])?,
            "extra" => fs::write(root.join("untracked.txt"), "retain")?,
            "test" => fs::write(root.join("test_solution.py"), "changed")?,
            "backup" => fs::write(w.area.join("original-tests.py"), "changed")?,
            _ => {}
        }
        assert!(
            w.apply(&e, w.test_grant(&e.review_hash()?)).is_err(),
            "{mode}"
        );
        assert!(!w.area.join("apply-complete.json").exists());
    }
    Ok(())
}
#[test]
fn symbolic_hard_links_and_path_swap_fail_closed() -> TestResult {
    for mode in ["symbolic", "hard", "directory"] {
        let t = tempfile::tempdir()?;
        let root = t.path().canonicalize()?.join("repo");
        fixture(&root)?;
        let w = Workspace::prepare(
            root.clone(),
            "solution.py".into(),
            "test_solution.py".into(),
            t.path().join("action"),
        )?;
        if mode == "directory" {
            fs::rename(&root, t.path().join("old"))?;
            fs::create_dir(&root)?;
        } else {
            fs::write(t.path().join("owner.py"), "Preserve independent bytes")?;
            if mode == "symbolic" {
                std::os::unix::fs::symlink(t.path().join("owner.py"), root.join("solution.py"))?;
            } else {
                fs::hard_link(t.path().join("owner.py"), root.join("solution.py"))?;
            }
        }
        assert!(w.recheck().is_err());
    }
    Ok(())
}
#[test]
fn nonpassing_or_tampered_check_never_creates_review_hash() -> TestResult {
    let t = tempfile::tempdir()?;
    let root = t.path().canonicalize()?.join("repo");
    fixture(&root)?;
    let w = Workspace::prepare(
        root,
        "solution.py".into(),
        "test_solution.py".into(),
        t.path().join("action"),
    )?;
    let good = passed(&w)?;
    for mode in ["exit", "passed", "cleanup", "bytes", "image"] {
        let mut e = good.clone();
        match mode {
            "exit" => e.attempts[0].check.exit = Some(1),
            "passed" => e.attempts[0].check.passed = false,
            "cleanup" => e.attempts[0].check.process_absent = false,
            "bytes" => e.attempts[0].candidate.push('x'),
            "image" => e.attempts[0].check.image = "other".into(),
            _ => {}
        }
        assert!(e.review_hash().is_err());
    }
    Ok(())
}
#[test]
fn absent_existing_file_is_added_without_overwriting_existing_test() -> TestResult {
    let t = tempfile::tempdir()?;
    let root = t.path().canonicalize()?.join("repo");
    fixture(&root)?;
    let tests = fs::read(root.join("test_solution.py"))?;
    let w = Workspace::prepare(
        root.clone(),
        "new.py".into(),
        "test_solution.py".into(),
        t.path().join("action"),
    )?;
    let e = passed(&w)?;
    w.apply(&e, w.test_grant(&e.review_hash()?))?;
    assert_eq!(fs::read(root.join("test_solution.py"))?, tests);
    assert_eq!(
        fs::read_to_string(root.join("new.py"))?,
        e.attempts[0].candidate
    );
    Ok(())
}

#[test]
#[ignore = "requires the explicitly approved local Docker engine and pinned image"]
fn docker_executor_boundaries_and_cancellation() -> TestResult {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(async {
      for (label,tests,expected) in [
        ("pass","import unittest\nfrom solution import add\nclass T(unittest.TestCase):\n def test_add(self): self.assertEqual(add(2,3),5)\n",Some(true)),
        ("fail","import unittest\nclass T(unittest.TestCase):\n def test_fail(self): self.fail('synthetic failure')\n",Some(false)),
        ("isolation","import unittest,os,socket,pathlib\nclass T(unittest.TestCase):\n def test_boundary(self):\n  self.assertEqual(os.getuid(),65534)\n  self.assertFalse(pathlib.Path('/Users').exists())\n  self.assertFalse(pathlib.Path('/var/run/docker.sock').exists())\n  self.assertFalse(pathlib.Path('/workspace/.git').exists())\n  self.assertNotIn('OPENAI_API_KEY',os.environ)\n  with self.assertRaises(OSError): pathlib.Path('/workspace/solution.py').write_text('bad')\n  with self.assertRaises(OSError): pathlib.Path('/root/private').write_text('bad')\n  s=socket.socket(); s.settimeout(1); self.assertNotEqual(s.connect_ex(('192.0.2.1',443)),0); s.close()\n",Some(true)),
        ("output","import unittest\nprint('x'*20000)\n",None),
        ("memory","import unittest\nx=bytearray(256*1024*1024)\n",Some(false)),
        ("timeout","import time,os,signal\nos.kill(1,signal.SIGSTOP)\ntime.sleep(120)\n",Some(false)),
      ] {
        let t=tempfile::tempdir()?;let root=t.path().canonicalize()?.join("repo");fixture(&root)?;
        fs::write(root.join("test_solution.py"),tests)?;git(&root,&["add","test_solution.py"])?;git(&root,&["commit","-qm","Synthetic validation case"])?;
        let w=Workspace::prepare(root,"solution.py".into(),"test_solution.py".into(),t.path().join("action"))?;
        let code="def add(a,b): return a+b\n";let stage=w.stage(code,0)?;
        let result=sandbox::validate(&stage,&w.file,&w.test_file,code,tests).await;
        match expected {Some(pass)=>{let c=result?;assert_eq!(c.passed,pass,"{label}");assert!(c.process_absent);},None=>assert!(matches!(result,Err(ActionError::Limit)),"{label}")}
        sandbox::verify_cleanup(&w.area)?;w.recheck()?;
      }
      // Cancel with a live child inside the container; no host process is targeted.
      let t=tempfile::tempdir()?;let root=t.path().canonicalize()?.join("repo");fixture(&root)?;
      let tests="import subprocess,time\nsubprocess.Popen(['/usr/local/bin/python3','-c','import time; time.sleep(120)'])\ntime.sleep(120)\n";
      fs::write(root.join("test_solution.py"),tests)?;git(&root,&["add","test_solution.py"])?;git(&root,&["commit","-qm","Synthetic cancellation case"])?;
      let w=Workspace::prepare(root,"solution.py".into(),"test_solution.py".into(),t.path().join("action"))?;
      let stage=w.stage("x=1\n",0)?;let cid=stage.with_extension("cid");
      let task=tokio::spawn(async move {sandbox::validate(&stage,"solution.py","test_solution.py","x=1\n",tests).await});
      let start=std::time::Instant::now();
      while !cid.exists() && start.elapsed()<std::time::Duration::from_secs(8) {tokio::time::sleep(std::time::Duration::from_millis(50)).await;}
      assert!(cid.exists());tokio::time::sleep(std::time::Duration::from_millis(700)).await;task.abort();assert!(task.await.is_err());
      sandbox::verify_cleanup(&w.area)?;w.recheck()?;
      Ok(())
    })
}

#[test]
fn staged_bytes_and_crash_intent_block_application_replay() -> TestResult {
    for mode in ["candidate", "tests", "intent"] {
        let t = tempfile::tempdir()?;
        let root = t.path().canonicalize()?.join("repo");
        fixture(&root)?;
        let w = Workspace::prepare(
            root.clone(),
            "solution.py".into(),
            "test_solution.py".into(),
            t.path().join("action"),
        )?;
        let e = passed(&w)?;
        if mode == "intent" {
            fs::write(
                w.area.join("apply-started.json"),
                "retained interrupted intent",
            )?;
        } else {
            use std::os::unix::fs::PermissionsExt;
            let p = w.area.join("attempt-0").join(if mode == "candidate" {
                "solution.py"
            } else {
                "test_solution.py"
            });
            fs::set_permissions(&p, fs::Permissions::from_mode(0o644))?;
            fs::write(&p, "changed staging bytes")?;
            fs::set_permissions(&p, fs::Permissions::from_mode(0o444))?;
        }
        assert!(w.apply(&e, w.test_grant(&e.review_hash()?)).is_err());
        assert!(!root.join("solution.py").exists());
    }
    Ok(())
}

#[test]
#[ignore = "requires the approved Docker image; adapters are synthetic, never live requests"]
fn bounded_correction_uses_actual_checks_and_preserves_failed_attempts() -> TestResult {
    use crate::{
        agent::definition::AgentId,
        agent_adapter::AdapterRequest,
        agent_preferences::{AgentConnection, AgentProfile},
        collaboration::{Objective, Run, Status, Workflow},
        personal_assistant_direct::ProviderEvent,
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(async {
  for correct in [true,false] {
   let t=tempfile::tempdir()?;let root=t.path().canonicalize()?.join("repo");fixture(&root)?;
   let w=Workspace::prepare(root,"solution.py".into(),"test_solution.py".into(),t.path().join("action"))?;
   let mut profiles=AgentId::ALL.into_iter().map(AgentProfile::defaults).collect::<std::result::Result<Vec<_>,_>>()?;
   for p in &mut profiles {p.connection=AgentConnection::OpenaiApi;p.model="gpt-5.6-luna".into();}
   let mut run=Run::new("synthetic-action".into(),Objective{workflow:Workflow::CodingAction,objective:"Add two integers".into(),sources:vec![]},&profiles)?;
   run.action=Some(w.evidence()?);run.status=Status::Running;
   let mut calls=0;let mut snapshots=Vec::new();
   let result=crate::collaboration_action::execute(&mut run,&w,|profile,prompt| {
     calls+=1;
     let v:serde_json::Value=serde_json::from_str(prompt).map_err(|_|crate::agent_chat::ChatError::Internal)?;
     let reply=if profile.agent_id=="coding" {
        let code=if calls==1 {"def add(a,b): return a*b\n"}else if correct {"def add(a,b): return a+b\n"}else{"def add(a,b): return 0\n"};
        serde_json::json!({"version":1,"file":"solution.py","baseHash":v["baseHash"],"content":code})
     } else {serde_json::json!({"version":1,"stage":1,"agentId":"qa-validation","status":"complete","summary":"Synthetic QA assertion cannot override execution","findings":[],"evidence":[],"limitations":["Synthetic adapter evidence only"]})};
     Ok(AdapterRequest::Events(vec![ProviderEvent::Started("synthetic".into()),ProviderEvent::Delta(reply.to_string()),ProviderEvent::Completed]))
   },|snapshot|{snapshots.push(snapshot.clone());Ok(())}).await;
   assert_eq!(calls,4);assert_eq!(result.is_ok(),correct);
   let e=run.action.as_ref().ok_or("missing action")?;assert_eq!(e.requests,4);assert_eq!(e.attempts.len(),2);assert!(!e.attempts[0].check.passed);assert!(e.attempts[0].qa_handoff.is_some());
   assert_eq!(e.attempts[1].check.passed,correct);assert_eq!(e.review_hash.is_some(),correct);w.recheck()?;sandbox::verify_cleanup(&w.area)?;
   assert!(snapshots.windows(2).all(|pair|pair[0].sequence<pair[1].sequence));
  }Ok(())
 })
}

#[test]
fn existing_target_is_rejected_and_atomic_add_never_clobbers_a_creator() -> TestResult {
    let t = tempfile::tempdir()?;
    let root = t.path().canonicalize()?.join("repo");
    fixture(&root)?;
    fs::write(root.join("solution.py"), "owner bytes\n")?;
    git(&root, &["add", "solution.py"])?;
    git(&root, &["commit", "-qm", "Existing owner file"])?;
    assert!(Workspace::prepare(
        root.clone(),
        "solution.py".into(),
        "test_solution.py".into(),
        t.path().join("action")
    )
    .is_err());
    // The owner file exists only at the effect point in the real race. Exercise
    // the exact production primitive independently of an earlier fingerprint.
    fs::write(root.join(".stage"), "candidate bytes\n")?;
    let directory = fs::File::open(&root)?;
    assert_eq!(
        workspace::add_new_file(&directory, ".stage", "solution.py"),
        Err(ActionError::Recovery)
    );
    assert_eq!(
        fs::read_to_string(root.join("solution.py"))?,
        "owner bytes\n"
    );
    assert_eq!(
        fs::read_to_string(root.join(".stage"))?,
        "candidate bytes\n"
    );
    workspace::add_new_file(&directory, ".stage", "new.py")?;
    assert_eq!(
        fs::read_to_string(root.join("new.py"))?,
        "candidate bytes\n"
    );
    Ok(())
}
#[test]
fn incomplete_container_creation_never_claims_absence() -> TestResult {
    let t = tempfile::tempdir()?;
    fs::write(t.path().join("attempt-0.creating"), "creating")?;
    assert_eq!(
        sandbox::verify_cleanup(t.path()),
        Err(ActionError::Isolation)
    );
    Ok(())
}
