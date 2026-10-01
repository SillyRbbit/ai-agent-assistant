//! Closed text-only Codex app-server adapter. No caller-supplied executable,
//! filesystem roots, RPC methods, tools, credentials or config reach the child.
use crate::{
    agent_models::ModelInfo,
    agent_preferences::ReasoningEffort,
    personal_assistant_direct::{DirectError, ProviderEvent},
};
use serde_json::{json, Value};
use std::{
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc, Arc, Mutex, Weak,
    },
    thread::JoinHandle,
    time::Duration,
};

const MAX_FRAME: usize = 262_144;
const MAX_FRAMES: usize = 4096;
const FEATURES: &[&str] = &[
    "shell_tool",
    "unified_exec",
    "apps",
    "plugins",
    "hooks",
    "multi_agent",
    "browser_use",
    "computer_use",
    "image_generation",
    "in_app_browser",
    "workspace_dependencies",
    "tool_suggest",
    "memories",
    "code_mode_host",
    "view_image",
    "shell_snapshot",
    "daemon_auto_start",
    "in_app_local_automation",
    "unbounded_connection_retries",
    "remote_plugin",
    "skill_search",
];
static NEXT: AtomicU64 = AtomicU64::new(0);
type OwnedChild = Arc<Mutex<Child>>;
static CHILDREN: Mutex<Vec<Weak<Mutex<Child>>>> = Mutex::new(Vec::new());
static SHUTTING_DOWN: AtomicBool = AtomicBool::new(false);

fn reap(child: &OwnedChild) -> bool {
    let Ok(mut child) = child.lock() else {
        return false;
    };
    let _ = child.kill();
    child.wait().is_ok()
}
fn reap_children(children: &[Weak<Mutex<Child>>]) -> bool {
    let mut complete = true;
    for child in children.iter().filter_map(Weak::upgrade) {
        complete &= reap(&child);
    }
    complete
}
/// Called before Tauri exits, independent of async runtime scheduling.
/// Only children registered by this adapter are affected.
pub(crate) fn shutdown() -> bool {
    SHUTTING_DOWN.store(true, Ordering::SeqCst);
    let Ok(children) = CHILDREN.lock() else {
        return false;
    };
    reap_children(&children)
}

// Intentionally no Debug: even owner configuration stays out of error output.
pub(crate) struct Setup {
    executable: PathBuf,
    home: PathBuf,
}
impl Setup {
    pub(crate) fn from_environment() -> Result<Self, DirectError> {
        if !cfg!(debug_assertions) || std::env::var("CORTEXA_CODEX_DEMO").as_deref() != Ok("1") {
            return Err(DirectError::Disabled);
        }
        let executable = std::env::var_os("CORTEXA_CODEX_EXECUTABLE")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from("/Applications/ChatGPT.app/Contents/Resources/codex-cli/bin/codex")
            });
        let home = std::env::var_os("CORTEXA_CODEX_HOME")
            .map(PathBuf::from)
            .ok_or(DirectError::CodexSetup)?;
        if !executable.is_absolute()
            || !executable.is_file()
            || !home.is_absolute()
            || !home.is_dir()
            || home.join("config.toml").exists()
            || home.join("AGENTS.md").exists()
        {
            return Err(DirectError::CodexSetup);
        }
        Ok(Self { executable, home })
    }
}

struct Session {
    child: OwnedChild,
    tx: Option<mpsc::SyncSender<Vec<u8>>>,
    rx: mpsc::Receiver<Result<Value, DirectError>>,
    readers: Vec<JoinHandle<()>>,
    directory: PathBuf,
    frames: usize,
}
impl Drop for Session {
    fn drop(&mut self) {
        self.tx.take();
        let _ = reap(&self.child);
        for reader in self.readers.drain(..) {
            let _ = reader.join();
        }
        // Only this exclusively created, ephemeral runtime directory is removed.
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}
fn command(setup: &Setup, directory: &std::path::Path) -> Command {
    let mut command = Command::new(&setup.executable);
    command
        .env_clear()
        .env("HOME", directory)
        .env("CODEX_HOME", &setup.home)
        .env("PATH", "/usr/bin:/bin")
        .current_dir(directory)
        .args([
            "app-server",
            "--stdio",
            "--strict-config",
            "-c",
            "web_search=\"disabled\"",
        ]);
    for feature in FEATURES {
        command.args(["-c", &format!("features.{feature}=false")]);
    }
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    command
}
impl Session {
    fn spawn(setup: &Setup) -> Result<Self, DirectError> {
        let directory = std::env::temp_dir().join(format!(
            "cortexa-codex-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::SeqCst)
        ));
        std::fs::create_dir(&directory).map_err(|_| DirectError::CodexSetup)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700))
                .map_err(|_| DirectError::CodexSetup)?;
        }
        let mut children = CHILDREN.lock().map_err(|_| DirectError::Internal)?;
        children.retain(|child| child.strong_count() > 0);
        if SHUTTING_DOWN.load(Ordering::SeqCst) || children.len() >= 8 {
            let _ = std::fs::remove_dir(&directory);
            return Err(DirectError::Busy);
        }
        let child = command(setup, &directory).spawn().map_err(|_| {
            let _ = std::fs::remove_dir(&directory);
            DirectError::CodexSetup
        })?;
        let (tx, input) = mpsc::sync_channel::<Vec<u8>>(4);
        let (output, rx) = mpsc::sync_channel(64);
        let child = Arc::new(Mutex::new(child));
        children.push(Arc::downgrade(&child));
        drop(children);
        let mut session = Self {
            child,
            tx: Some(tx),
            rx,
            readers: Vec::new(),
            directory,
            frames: 0,
        };
        let (mut stdin, stdout) = {
            let mut child = session.child.lock().map_err(|_| DirectError::Internal)?;
            (
                child.stdin.take().ok_or(DirectError::Internal)?,
                child.stdout.take().ok_or(DirectError::Internal)?,
            )
        };
        session.readers.push(std::thread::spawn(move || {
            while let Ok(bytes) = input.recv() {
                if stdin
                    .write_all(&bytes)
                    .and_then(|()| stdin.flush())
                    .is_err()
                {
                    break;
                }
            }
        }));
        session.readers.push(std::thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                let mut bytes = Vec::new();
                let read = std::io::Read::take(&mut reader, (MAX_FRAME + 1) as u64)
                    .read_until(b'\n', &mut bytes);
                let value = match read {
                    Ok(0) => break,
                    Ok(n) if n <= MAX_FRAME && bytes.last() == Some(&b'\n') => {
                        serde_json::from_slice(&bytes).map_err(|_| DirectError::Protocol)
                    }
                    _ => Err(DirectError::Limit),
                };
                let failed = value.is_err();
                if output.try_send(value).is_err() || failed {
                    break;
                }
            }
        }));
        Ok(session)
    }
    fn send(&self, value: Value) -> Result<(), DirectError> {
        let mut bytes = serde_json::to_vec(&value).map_err(|_| DirectError::Protocol)?;
        if bytes.len() > 65_536 {
            return Err(DirectError::Limit);
        }
        bytes.push(b'\n');
        self.tx
            .as_ref()
            .ok_or(DirectError::Internal)?
            .try_send(bytes)
            .map_err(|_| DirectError::Protocol)
    }
    async fn next(&mut self) -> Result<Value, DirectError> {
        loop {
            match self.rx.try_recv() {
                Ok(value) => {
                    self.frames += 1;
                    if self.frames > MAX_FRAMES {
                        return Err(DirectError::Limit);
                    }
                    let value = value?;
                    if !value.is_object()
                        || (value.get("method").is_some() && value.get("id").is_some())
                    {
                        // Never approve a runtime tool, permission, auth or dynamic-tool request.
                        return Err(DirectError::CodexIsolation);
                    }
                    return Ok(value);
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    return Err(DirectError::ProviderUnavailable)
                }
                Err(mpsc::TryRecvError::Empty) => {
                    tokio::time::sleep(Duration::from_millis(5)).await
                }
            }
        }
    }
    async fn request(
        &mut self,
        id: u64,
        method: &str,
        params: Value,
    ) -> Result<Value, DirectError> {
        self.send(json!({"id":id,"method":method,"params":params}))?;
        loop {
            let value = self.next().await?;
            if value.get("id").is_some() {
                if value["id"] != id {
                    return Err(DirectError::Protocol);
                }
                if value.get("error").is_some() {
                    return Err(DirectError::CodexRuntime);
                }
                return value.get("result").cloned().ok_or(DirectError::Protocol);
            }
            reject_tool(&value)?;
        }
    }
    async fn initialize(&mut self) -> Result<(), DirectError> {
        let value = self.request(1,"initialize",json!({"clientInfo":{"name":"cortexa","version":"0.1.0"},"capabilities":{"experimentalApi":true}})).await?;
        if !value["userAgent"]
            .as_str()
            .is_some_and(|v| v.starts_with("cortexa/0.159.0 "))
        {
            return Err(DirectError::CodexSetup);
        }
        self.send(json!({"method":"initialized"}))
    }
    async fn authenticate(&mut self) -> Result<(), DirectError> {
        let value = self
            .request(2, "account/read", json!({"refreshToken":false}))
            .await?;
        if !matches!(
            value["account"]["type"].as_str(),
            Some("chatgpt" | "apiKey")
        ) {
            return Err(DirectError::CodexAuthentication);
        }
        Ok(())
    }
}
fn reject_tool(value: &Value) -> Result<(), DirectError> {
    if matches!(
        value["method"].as_str(),
        Some("item/started" | "item/completed")
    ) && !matches!(
        value["params"]["item"]["type"].as_str(),
        Some("userMessage" | "agentMessage" | "reasoning")
    ) {
        return Err(DirectError::CodexIsolation);
    }
    Ok(())
}
fn thread_parameters(model: &str) -> Value {
    json!({"model":model,"modelProvider":"openai","approvalPolicy":"never","sandbox":"read-only",
        "ephemeral":true,"environments":[],"dynamicTools":[],"runtimeWorkspaceRoots":[],"selectedCapabilityRoots":[],
        "baseInstructions":crate::agent_chat::COMMUNICATION_RULES,
        "allowProviderModelFallback":false})
}
fn thread_id(value: &Value, model: &str) -> Result<String, DirectError> {
    if value["model"] != model
        || value["modelProvider"] != "openai"
        || value["approvalPolicy"] != "never"
        || value["sandbox"]["type"] != "readOnly"
        || value["sandbox"]["networkAccess"] != false
        || value["runtimeWorkspaceRoots"] != json!([])
        || value["instructionSources"] != json!([])
    {
        return Err(DirectError::CodexIsolation);
    }
    identifier(&value["thread"]["id"])
}
fn identifier(value: &Value) -> Result<String, DirectError> {
    value
        .as_str()
        .filter(|s| !s.is_empty() && s.len() <= 256 && !s.chars().any(char::is_control))
        .map(str::to_owned)
        .ok_or(DirectError::Protocol)
}
fn models(value: &Value) -> Result<Vec<ModelInfo>, DirectError> {
    if !value["nextCursor"].is_null() {
        return Err(DirectError::Catalog);
    }
    let entries = value["data"]
        .as_array()
        .filter(|a| a.len() <= 128)
        .ok_or(DirectError::Catalog)?;
    let mut models = Vec::new();
    for entry in entries {
        if entry["hidden"] == true {
            continue;
        }
        let id = identifier(&entry["model"])?;
        if models.iter().any(|m: &ModelInfo| m.id == id) {
            return Err(DirectError::Catalog);
        }
        let mut model = ModelInfo::unknown(id);
        let efforts = entry["supportedReasoningEfforts"]
            .as_array()
            .ok_or(DirectError::Catalog)?;
        for effort in efforts {
            if let Ok(effort) =
                serde_json::from_value::<ReasoningEffort>(effort["reasoningEffort"].clone())
            {
                if !model.efforts.contains(&effort) {
                    model.efforts.push(effort);
                }
            }
        }
        model.provenance =
            Some("Installed Codex model/list; access requires owner authentication".into());
        models.push(model);
    }
    if models.is_empty() {
        return Err(DirectError::Catalog);
    }
    Ok(models)
}
pub(crate) async fn discover() -> Result<Vec<ModelInfo>, DirectError> {
    let setup = Setup::from_environment()?;
    let mut session = Session::spawn(&setup)?;
    session.initialize().await?;
    session.authenticate().await?;
    models(
        &session
            .request(3, "model/list", json!({"limit":128,"includeHidden":false}))
            .await?,
    )
}
pub(crate) async fn run(
    setup: Setup,
    model: String,
    effort: ReasoningEffort,
    input: String,
    emit: impl FnMut(ProviderEvent) -> Result<(), DirectError>,
) -> Result<(), DirectError> {
    run_with_rules(
        setup,
        model,
        effort,
        input,
        crate::agent_chat::COMMUNICATION_RULES,
        emit,
    )
    .await
}
pub(crate) async fn run_collaboration(
    setup: Setup,
    model: String,
    effort: ReasoningEffort,
    input: String,
    emit: impl FnMut(ProviderEvent) -> Result<(), DirectError>,
) -> Result<(), DirectError> {
    run_with_rules(
        setup,
        model,
        effort,
        input,
        crate::collaboration::RULES,
        emit,
    )
    .await
}
async fn run_with_rules(
    setup: Setup,
    model: String,
    effort: ReasoningEffort,
    input: String,
    rules: &str,
    mut emit: impl FnMut(ProviderEvent) -> Result<(), DirectError>,
) -> Result<(), DirectError> {
    let mut session = Session::spawn(&setup)?;
    session.initialize().await?;
    session.authenticate().await?;
    let catalog = models(
        &session
            .request(3, "model/list", json!({"limit":128,"includeHidden":false}))
            .await?,
    )?;
    crate::agent_models::validate_selection(
        catalog
            .iter()
            .find(|m| m.id == model)
            .ok_or(DirectError::ModelUnavailable)?,
        effort,
    )?;
    let mut parameters = thread_parameters(&model);
    parameters["baseInstructions"] = json!(rules);
    let thread = thread_id(
        &session.request(4, "thread/start", parameters).await?,
        &model,
    )?;
    let mut params =
        json!({"threadId":thread,"environments":[],"input":[{"type":"text","text":input}]});
    if effort != ReasoningEffort::Default {
        params["effort"] = json!(effort);
    }
    let response = session.request(5, "turn/start", params).await?;
    let turn = identifier(&response["turn"]["id"])?;
    emit(ProviderEvent::Started("resp_cortexa_codex".into()))?;
    let mut output = String::new();
    loop {
        let value = session.next().await?;
        reject_tool(&value)?;
        let method = value["method"].as_str().ok_or(DirectError::Protocol)?;
        if matches!(
            method,
            "item/agentMessage/delta" | "turn/completed" | "error"
        ) {
            if value["params"]["threadId"] != thread {
                return Err(DirectError::Protocol);
            }
            if method == "error" {
                return Err(DirectError::CodexRuntime);
            }
            if method == "turn/completed" {
                if value["params"]["turn"]["id"] != turn
                    || value["params"]["turn"]["status"] != "completed"
                {
                    return Err(DirectError::CodexRuntime);
                }
                if output.is_empty() {
                    return Err(DirectError::Incomplete);
                }
                // Kill/reap and join before exposing completion or releasing ownership.
                drop(session);
                return emit(ProviderEvent::Completed);
            }
            if value["params"]["turnId"] != turn {
                return Err(DirectError::Protocol);
            }
            let delta = value["params"]["delta"]
                .as_str()
                .ok_or(DirectError::Protocol)?;
            output.push_str(delta);
            if output.chars().count() > 8192 {
                return Err(DirectError::Limit);
            }
            if !delta.is_empty() {
                emit(ProviderEvent::Delta(delta.into()))?;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn isolation_parameters_and_clean_environment_are_closed() {
        let setup = Setup {
            executable: "/synthetic/codex".into(),
            home: "/synthetic/auth".into(),
        };
        let command = command(&setup, std::path::Path::new("/synthetic/empty"));
        let keys: Vec<_> = command
            .get_envs()
            .filter_map(|(k, v)| v.map(|_| k.to_string_lossy().into_owned()))
            .collect();
        assert_eq!(keys, ["CODEX_HOME", "HOME", "PATH"]);
        let p = thread_parameters("model");
        for key in [
            "environments",
            "dynamicTools",
            "runtimeWorkspaceRoots",
            "selectedCapabilityRoots",
        ] {
            assert_eq!(p[key], json!([]));
        }
        assert_eq!(p["approvalPolicy"], "never");
        assert_eq!(p["allowProviderModelFallback"], false);
    }
    #[test]
    fn runtime_drift_and_tools_fail_closed_without_payloads() {
        for kind in [
            "commandExecution",
            "fileChange",
            "mcpToolCall",
            "webSearch",
            "dynamicToolCall",
        ] {
            assert_eq!(
                reject_tool(
                    &json!({"method":"item/started","params":{"item":{"type":kind,"secret":"sentinel"}}})
                ),
                Err(DirectError::CodexIsolation)
            );
        }
        assert_eq!(
            thread_id(
                &json!({"model":"model","sandbox":{"type":"dangerFullAccess"},"secret":"sentinel"}),
                "model"
            ),
            Err(DirectError::CodexIsolation)
        );
        for error in [
            DirectError::CodexSetup,
            DirectError::CodexRuntime,
            DirectError::CodexAuthentication,
            DirectError::CodexIsolation,
        ] {
            assert!(!format!("{error:?} {error}").contains("sentinel"));
        }
    }
    #[test]
    fn discovered_models_only_expose_recognized_supported_efforts() -> Result<(), DirectError> {
        let result = models(
            &json!({"nextCursor":null,"data":[{"model":"gpt-runtime","hidden":false,"supportedReasoningEfforts":[{"reasoningEffort":"low"},{"reasoningEffort":"high"},{"reasoningEffort":"future"}]}]}),
        )?;
        assert_eq!(
            result[0].efforts,
            vec![
                ReasoningEffort::Default,
                ReasoningEffort::Low,
                ReasoningEffort::High
            ]
        );
        assert_eq!(
            crate::agent_models::validate_selection(&result[0], ReasoningEffort::Max),
            Err(DirectError::Unsupported)
        );
        assert!(models(&json!({"data":[],"nextCursor":"more"})).is_err());
        Ok(())
    }
    #[cfg(unix)]
    fn fixture(mode: &str) -> Result<(tempfile::TempDir, Setup), Box<dyn std::error::Error>> {
        use std::os::unix::fs::PermissionsExt;
        let directory = tempfile::tempdir()?;
        let executable = directory.path().join("fixture");
        let python = if cfg!(target_os = "macos") {
            "/Library/Frameworks/Python.framework/Versions/3.12/bin/python3"
        } else {
            "/usr/bin/python3"
        };
        let script = r#"
import sys,json,time,os
mode=MODE
assert set(os.environ).issubset({'HOME','CODEX_HOME','PATH','LC_CTYPE','__CF_USER_TEXT_ENCODING'})
def out(v): print(json.dumps(v),flush=True)
for line in sys.stdin:
 v=json.loads(line);m=v['method'];i=v.get('id')
 if m=='initialized': continue
 if m=='initialize': result={'userAgent':'cortexa/0.159.0 fixture'}
 elif m=='account/read': result={'account':{'type':'chatgpt'}}
 elif m=='model/list': result={'nextCursor':None,'data':[{'model':'fixture-model','hidden':False,'supportedReasoningEfforts':[{'reasoningEffort':'high'}]}]}
 elif m=='thread/start':
  p=v['params'];assert all(p[k]==[] for k in ['environments','runtimeWorkspaceRoots','selectedCapabilityRoots','dynamicTools']);assert p['approvalPolicy']=='never'
  result={'model':'fixture-model','modelProvider':'openai','approvalPolicy':'never','sandbox':{'type':'readOnly','networkAccess':False},'runtimeWorkspaceRoots':[],'instructionSources':[],'thread':{'id':'thread-fixture'}}
 elif m=='turn/start':
  assert v['params']['effort']=='high';assert v['params']['environments']==[]
  result={'turn':{'id':'turn-fixture'}}
 else: raise RuntimeError('closed fixture protocol mismatch')
 out({'id':i,'result':result})
 if m=='turn/start':
  if mode=='pending': time.sleep(30)
  elif mode=='tool': out({'method':'item/started','params':{'item':{'type':'commandExecution','raw':'secret-sentinel'}}})
  elif mode=='error': out({'method':'error','params':{'threadId':'thread-fixture','error':{'message':'secret-sentinel'}}})
  else:
   out({'method':'item/agentMessage/delta','params':{'threadId':'thread-fixture','turnId':'turn-fixture','delta':'Synthetic answer.'}})
   out({'method':'turn/completed','params':{'threadId':'thread-fixture','turn':{'id':'turn-fixture','status':'completed'}}})
"#;
        std::fs::write(
            &executable,
            format!(
                "#!{python}\n{}",
                script.replace("MODE", &serde_json::to_string(mode)?)
            ),
        )?;
        std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o700))?;
        let setup = Setup {
            executable,
            home: directory.path().into(),
        };
        Ok((directory, setup))
    }
    #[cfg(unix)]
    #[test]
    fn offline_app_server_streams_completes_and_rejects_tools_and_raw_errors(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        for (mode, expected) in [
            ("success", None),
            ("tool", Some(DirectError::CodexIsolation)),
            ("error", Some(DirectError::CodexRuntime)),
        ] {
            let (_directory, setup) = fixture(mode)?;
            let mut text = String::new();
            let mut complete = false;
            let result = runtime.block_on(async {
                tokio::time::timeout(Duration::from_secs(5), async {
                    run(
                        setup,
                        "fixture-model".into(),
                        ReasoningEffort::High,
                        "synthetic".into(),
                        |event| {
                            match event {
                                ProviderEvent::Delta(delta) => text.push_str(&delta),
                                ProviderEvent::Completed => complete = true,
                                ProviderEvent::Started(_) => {}
                            }
                            Ok(())
                        },
                    )
                    .await
                })
                .await
            });
            assert_eq!(result?, expected.map_or(Ok(()), Err));
            assert_eq!(complete, expected.is_none());
            assert!(!text.contains("secret-sentinel"));
            if complete {
                assert_eq!(text, "Synthetic answer.");
            }
        }
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn cancelled_session_reaps_child_and_removes_only_owned_directory(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_directory, setup) = fixture("pending")?;
        let mut session = Session::spawn(&setup)?;
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        runtime.block_on(session.initialize())?;
        let path = session.directory.clone();
        let pid = session.child.lock().map_err(|_| "child lock")?.id();
        drop(session);
        assert!(!path.exists());
        let alive = Command::new("/bin/kill")
            .args(["-0", &pid.to_string()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        assert!(!alive.success());
        assert!(setup.home.exists());
        Ok(())
    }
    #[cfg(unix)]
    #[test]
    fn exit_cleanup_reaps_registered_child_without_waiting_for_future_drop(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (_directory, setup) = fixture("pending")?;
        let session = Session::spawn(&setup)?;
        assert!(reap_children(&[Arc::downgrade(&session.child)]));
        assert!(session
            .child
            .lock()
            .map_err(|_| "child lock")?
            .try_wait()?
            .is_some());
        // The session remains alive here; cleanup does not depend on polling it.
        assert!(session.directory.exists());
        drop(session);
        Ok(())
    }
}
