//! Fixed Docker execution; no caller-selected shell, image, mount or environment.
use super::{hash, ActionError, Check, Result, IMAGE, MAX_OUTPUT};
use std::{
    fs,
    io::Read,
    path::Path,
    process::{Command, Stdio},
    sync::mpsc,
    time::{Duration, Instant},
};

// Both output streams are bounded while the child is alive. Never collect an
// unbounded Command::output from repository-controlled execution.
pub(crate) fn host_command(
    mut command: Command,
    timeout: Duration,
    limit: usize,
) -> Result<(Option<i32>, Vec<u8>)> {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().map_err(|_| ActionError::Isolation)?;
    let (tx, rx) = mpsc::sync_channel::<Vec<u8>>(4);
    let stdout = child.stdout.take().ok_or(ActionError::Isolation)?;
    let stderr = child.stderr.take().ok_or(ActionError::Isolation)?;
    for mut reader in [Box::new(stdout) as Box<dyn Read + Send>, Box::new(stderr)] {
        let sender = tx.clone();
        std::thread::spawn(move || {
            let mut buf = [0_u8; 1024];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        if sender.send(buf[..n].to_vec()).is_err() {
                            break;
                        }
                    }
                }
            }
        });
    }
    drop(tx);
    let started = Instant::now();
    let mut output = Vec::new();
    let mut status = None;
    let mut disconnected = false;
    loop {
        match rx.recv_timeout(Duration::from_millis(10)) {
            Ok(bytes) => {
                if output.len() + bytes.len() > limit {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(ActionError::Limit);
                }
                output.extend(bytes);
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => disconnected = true,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
        if status.is_none() {
            status = child.try_wait().map_err(|_| ActionError::Isolation)?;
        }
        if disconnected && status.is_some() {
            break;
        }
        if started.elapsed() > timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Err(ActionError::Timeout);
        }
    }
    Ok((status.and_then(|s| s.code()), output))
}
fn docker() -> Command {
    let mut c = Command::new("/usr/local/bin/docker");
    // No host secrets, auth helpers, proxy settings or caller Docker context.
    c.env_clear().env("PATH", "/usr/local/bin:/usr/bin:/bin");
    c.args([
        "--config",
        "/var/empty",
        "--host",
        "unix:///var/run/docker.sock",
    ]);
    c
}
fn valid_id(id: &str) -> Result<()> {
    if id.len() != 64 || !id.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err(ActionError::Isolation);
    }
    Ok(())
}
fn absent(id: &str) -> Result<bool> {
    valid_id(id)?;
    let mut c = docker();
    c.args(["inspect", "--format", "{{.Id}}", id]);
    let (exit, bytes) = host_command(c, Duration::from_secs(3), 1024)?;
    if exit == Some(0) && String::from_utf8_lossy(&bytes).trim() == id {
        return Ok(false);
    }
    let message = String::from_utf8_lossy(&bytes);
    if exit == Some(1)
        && message.trim().to_ascii_lowercase() == format!("error: no such object: {id}")
    {
        return Ok(true);
    }
    #[cfg(test)]
    eprintln!(
        "action_diagnostic: absence_unresolved exit={exit:?} bytes={}",
        bytes.len()
    );
    Err(ActionError::Isolation)
}
fn remove(id: &str) -> Result<()> {
    valid_id(id)?;
    if !absent(id)? {
        let mut c = docker();
        c.args(["rm", "--force", id]);
        let (exit, _) = host_command(c, Duration::from_secs(3), 1024)?;
        if exit != Some(0) {
            return Err(ActionError::Isolation);
        }
    }
    if !absent(id)? {
        return Err(ActionError::Isolation);
    }
    Ok(())
}
struct ContainerGuard {
    cid: std::path::PathBuf,
    removed: bool,
}
impl ContainerGuard {
    fn id(&self) -> Result<String> {
        let v = fs::read_to_string(&self.cid).map_err(|_| ActionError::Isolation)?;
        let v = v.trim();
        valid_id(v)?;
        Ok(v.into())
    }
    fn finish(&mut self) -> Result<String> {
        let id = self.id()?;
        remove(&id)?;
        self.removed = true;
        Ok(id)
    }
}
impl Drop for ContainerGuard {
    fn drop(&mut self) {
        if !self.removed {
            if let Ok(id) = self.id() {
                let _ = remove(&id);
            }
        }
    }
}
// Called after the owning future is joined, before ingress is released. Drop is
// best-effort only; this independent check determines cancellation acceptance.
pub(crate) fn verify_cleanup(area: &Path) -> Result<()> {
    for i in 0..super::MAX_ATTEMPTS {
        let p = area.join(format!("attempt-{i}")).with_extension("cid");
        if p.with_extension("creating").exists() && !p.exists() {
            return Err(ActionError::Isolation);
        }
        if p.exists() {
            let id = fs::read_to_string(p).map_err(|_| ActionError::Isolation)?;
            if !absent(id.trim())? {
                return Err(ActionError::Isolation);
            }
        }
    }
    Ok(())
}
pub(crate) fn readiness() -> Result<()> {
    let mut c = docker();
    c.args([
        "image",
        "inspect",
        IMAGE,
        "--format",
        "{{.Os}}/{{.Architecture}}",
    ]);
    let (exit, bytes) = host_command(c, Duration::from_secs(3), 4096)?;
    if exit != Some(0) || bytes != b"linux/arm64\n" {
        #[cfg(test)]
        eprintln!("action_diagnostic: image_readiness_failed exit={exit:?}");
        return Err(ActionError::Isolation);
    }
    Ok(())
}
pub(crate) async fn validate(
    area: &Path,
    file: &str,
    test_file: &str,
    candidate: &str,
    test: &str,
) -> Result<Check> {
    readiness()?;
    super::workspace::verify_snapshot(area, file, test_file, candidate, test)?;
    let cid = area.with_extension("cid");
    if cid.exists() {
        return Err(ActionError::Isolation);
    }
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(cid.with_extension("creating"))
        .map_err(|_| ActionError::Isolation)?
        .sync_all()
        .map_err(|_| ActionError::Isolation)?;
    let mut guard = ContainerGuard {
        cid: cid.clone(),
        removed: false,
    };
    let mount = format!(
        "type=bind,source={},target=/workspace,readonly",
        area.display()
    );
    if area.to_string_lossy().contains([',', '\n', '\r']) {
        return Err(ActionError::Scope);
    }
    // Fixed code is trusted. Only the validated root-level test filename is data.
    // Test code can execute only inside this container; it cannot select host commands.
    let script="import sys,unittest; sys.path.insert(0,'/workspace'); suite=unittest.defaultTestLoader.discover('/workspace',pattern=sys.argv[1]); count=suite.countTestCases(); result=unittest.TextTestRunner(verbosity=1).run(suite); sys.exit(0 if count>0 and result.wasSuccessful() else 1)";
    let mut c = docker();
    c.args(["create", "--pull", "never", "--cidfile"])
        .arg(&cid)
        .args([
            "--network",
            "none",
            "--cap-drop",
            "ALL",
            "--security-opt",
            "no-new-privileges",
            "--read-only",
            "--user",
            "65534:65534",
            "--pids-limit",
            "32",
            "--memory",
            "128m",
            "--memory-swap",
            "128m",
            "--ulimit",
            "nofile=64:64",
            "--ulimit",
            "core=0:0",
            "--cpus",
            "1",
            "--tmpfs",
            "/tmp:rw,noexec,nosuid,size=16m",
            "--log-driver",
            "none",
            "--mount",
            &mount,
            "--workdir",
            "/workspace",
            "--entrypoint",
            "/usr/bin/timeout",
            IMAGE,
            "--signal=KILL",
            "15s",
            "/usr/local/bin/python3",
            "-I",
            "-B",
            "-c",
            script,
            test_file,
        ]);
    let (created, _) = host_command(c, Duration::from_secs(5), 2048)?;
    if created != Some(0) {
        #[cfg(test)]
        eprintln!("action_diagnostic: container_create_failed exit={created:?}");
        return Err(ActionError::Isolation);
    }
    let id = guard.id().inspect_err(|_| {
        #[cfg(test)]
        eprintln!("action_diagnostic: cid_unavailable");
    })?;
    #[cfg(test)]
    eprintln!("action_diagnostic: owned_container={id}");
    let mut c = docker();
    c.args(["start", "--attach", &id]);
    let (tx, rx) = mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let _ = tx.send(host_command(c, Duration::from_secs(18), MAX_OUTPUT));
    });
    let started = Instant::now();
    let result = loop {
        match rx.try_recv() {
            Ok(r) => break r,
            Err(mpsc::TryRecvError::Disconnected) => break Err(ActionError::Isolation),
            Err(mpsc::TryRecvError::Empty) => {}
        }
        if started.elapsed() > Duration::from_secs(20) {
            break Err(ActionError::Timeout);
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    };
    let id = guard.finish().inspect_err(|_| {
        #[cfg(test)]
        eprintln!("action_diagnostic: cleanup_failed");
    })?;
    let (exit, bytes) = result?;
    super::workspace::verify_snapshot(area, file, test_file, candidate, test)?;
    let output = String::from_utf8_lossy(&bytes)
        .chars()
        .map(|c| {
            if c.is_control() && c != '\n' && c != '\t' {
                '�'
            } else {
                c
            }
        })
        .collect();
    Ok(Check {
        command: format!("python3 -I -B <fixed unittest runner> {test_file} (candidate {file})"),
        exit,
        output,
        passed: exit == Some(0),
        image: IMAGE.into(),
        candidate_hash: hash(candidate.as_bytes()),
        test_hash: hash(test.as_bytes()),
        container_id: id,
        process_absent: true,
    })
}
