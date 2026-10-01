//! Narrow native dialogs. IPC accepts IDs/content, never source/destination paths.
use crate::{
    knowledge::{self, Item, KnowledgeError as E, Result, SaveNote},
    storage::Storage,
};
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Selection {
    id: i64,
    version: u64,
    passage: usize,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Import {
    id: Option<i64>,
    expected_version: Option<u64>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Identity {
    id: i64,
    version: u64,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Query {
    text: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Match {
    source: crate::collaboration::Source,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DraftRequest {
    room_id: i64,
    run_id: String,
    stage_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Draft {
    title: String,
    content: String,
}
#[tauri::command]
pub(crate) fn list_knowledge(storage: tauri::State<'_, Storage>) -> Result<Vec<Item>> {
    storage.knowledge_items()
}
#[tauri::command]
pub(crate) fn save_knowledge(
    request: SaveNote,
    storage: tauri::State<'_, Storage>,
) -> Result<Item> {
    storage.knowledge_save(request)
}
#[tauri::command]
pub(crate) fn remove_knowledge(
    request: Identity,
    storage: tauri::State<'_, Storage>,
) -> Result<()> {
    storage.knowledge_remove(request.id, request.version)
}
#[tauri::command]
pub(crate) fn select_knowledge_source(
    request: Selection,
    storage: tauri::State<'_, Storage>,
) -> Result<crate::collaboration::Source> {
    let all = storage.knowledge_items()?;
    let i = all
        .iter()
        .find(|i| i.id == request.id)
        .ok_or(E::Unavailable)?;
    knowledge::source(i, request.version, request.passage)
}
#[tauri::command]
pub(crate) fn search_knowledge(
    request: Query,
    storage: tauri::State<'_, Storage>,
) -> Result<Vec<Match>> {
    search(request, &storage)
}
fn search(request: Query, storage: &Storage) -> Result<Vec<Match>> {
    if request.text.len() > 200 {
        return Err(E::Limit);
    }
    let terms: Vec<_> = request
        .text
        .split_whitespace()
        .map(str::to_lowercase)
        .collect();
    if terms.is_empty() {
        return Ok(vec![]);
    }
    let mut found = Vec::new();
    for i in storage.knowledge_items()? {
        let v = i.current()?;
        for p in knowledge::passages(&v.content) {
            let lower = p.text.to_lowercase();
            if terms.iter().all(|t| lower.contains(t)) {
                found.push(Match {
                    source: knowledge::source(&i, v.id, p.id)?,
                });
                if found.len() == 50 {
                    return Ok(found);
                }
            }
        }
    }
    Ok(found)
}
#[tauri::command]
pub(crate) fn knowledge_draft(
    request: DraftRequest,
    storage: tauri::State<'_, Storage>,
) -> Result<Draft> {
    draft(request, &storage)
}
fn draft(request: DraftRequest, storage: &Storage) -> Result<Draft> {
    let rooms = storage.collaboration_rooms().map_err(|_| E::Storage)?;
    let room = rooms
        .iter()
        .find(|r| r.id == request.room_id)
        .ok_or(E::Unavailable)?;
    let run = room
        .runs
        .iter()
        .find(|r| r.id == request.run_id)
        .ok_or(E::Unavailable)?;
    let stage = run
        .stages
        .iter()
        .find(|s| s.id == request.stage_id)
        .ok_or(E::Unavailable)?;
    let h = stage.handoff.as_ref().ok_or(E::Unavailable)?;
    let mut content=format!("# Draft — {}\n\nModel-generated draft; claims are not verified. Review before reuse.\n\n{}\n\n",room.title,h.summary);
    for f in &h.findings {
        content.push_str(&format!("- {f}\n"));
    }
    content.push_str("\n## Limitations\n\n");
    for l in &h.limitations {
        content.push_str(&format!("- {l}\n"));
    }
    content.push_str("\n## Source attribution\n\n");
    for label in &h.evidence {
        if let Some(s) = run.input.sources.iter().find(|s| &s.label == label) {
            content.push_str(&format!("### {}\n\n", s.label));
            if let Some(o) = &s.origin {
                content.push_str(&format!(
                    "{} — document {}, version {}, passage {}, lines {}–{}; SHA-256 {}\n\n",
                    o.title,
                    o.document_id,
                    o.version,
                    o.passage_id,
                    o.start_line,
                    o.end_line,
                    o.hash
                ));
            }
            content.push_str(&s.text);
            content.push_str("\n\n");
        }
    }
    knowledge::validate(&room.title, &content)?;
    Ok(Draft {
        title: format!("{} draft", room.title),
        content,
    })
}
#[tauri::command]
pub(crate) async fn import_knowledge(
    request: Import,
    storage: tauri::State<'_, Storage>,
) -> Result<Option<Item>> {
    #[cfg(target_os = "macos")]
    {
        let selected = tauri::async_runtime::spawn_blocking(|| {
            rfd::FileDialog::new()
                .add_filter("Markdown or UTF-8 text", &["md", "txt"])
                .pick_file()
        })
        .await
        .map_err(|_| E::Unavailable)?;
        let Some(path) = selected else {
            return Ok(None);
        };
        let content =
            crate::documents::read_owner_selected_snapshot(&path).map_err(|e| match e {
                crate::documents::ApprovedDocumentError::InvalidUtf8 => E::InvalidUtf8,
                crate::documents::ApprovedDocumentError::UnsupportedFormat => E::UnsupportedFile,
                crate::documents::ApprovedDocumentError::DocumentTooLarge => E::Limit,
                crate::documents::ApprovedDocumentError::PathIdentityChanged => E::ChangedFile,
                crate::documents::ApprovedDocumentError::SymlinkRejected
                | crate::documents::ApprovedDocumentError::HardLinkRejected => E::UnsafePath,
                _ => E::Unavailable,
            })?;
        let title = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or(E::InvalidContent)?;
        let format = path
            .extension()
            .and_then(|s| s.to_str())
            .ok_or(E::UnsupportedFile)?;
        storage
            .knowledge_write(
                request.id,
                request.expected_version,
                title,
                &content,
                format,
                knowledge::Kind::Imported,
            )
            .map(Some)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (request.id, request.expected_version, storage);
        Err(E::Unavailable)
    }
}
#[tauri::command]
pub(crate) async fn export_knowledge(
    request: Identity,
    storage: tauri::State<'_, Storage>,
) -> Result<bool> {
    let all = storage.knowledge_items()?;
    let i = all
        .iter()
        .find(|i| i.id == request.id)
        .ok_or(E::Unavailable)?;
    let v = i
        .versions
        .iter()
        .find(|v| v.id == request.version)
        .ok_or(E::Unavailable)?
        .clone();
    #[cfg(target_os = "macos")]
    {
        let selected = tauri::async_runtime::spawn_blocking(|| {
            rfd::FileDialog::new()
                .add_filter("Markdown", &["md"])
                .set_file_name("note.md")
                .save_file()
        })
        .await
        .map_err(|_| E::Unavailable)?;
        let Some(path) = selected else {
            return Ok(false);
        };
        export_new(&path, &v.content)?;
        Ok(true)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = v;
        Err(E::Unavailable)
    }
}
#[cfg(any(target_os = "macos", test))]
fn export_new(path: &std::path::Path, content: &str) -> Result<()> {
    use std::io::Write;
    if path.extension().and_then(|s| s.to_str()) != Some("md") {
        return Err(E::UnsupportedFile);
    }
    let parent = path
        .parent()
        .ok_or(E::UnsafePath)?
        .canonicalize()
        .map_err(|_| E::Unavailable)?;
    let name = path.file_name().ok_or(E::UnsafePath)?;
    let directory =
        crate::documents::open_confined_directory(&parent).map_err(|_| E::UnsafePath)?;
    let fd = rustix::fs::openat(
        &directory,
        name,
        rustix::fs::OFlags::WRONLY
            | rustix::fs::OFlags::CREATE
            | rustix::fs::OFlags::EXCL
            | rustix::fs::OFlags::NOFOLLOW,
        rustix::fs::Mode::RUSR | rustix::fs::Mode::WUSR,
    )
    .map_err(|e| {
        if e == rustix::io::Errno::EXIST {
            E::DestinationExists
        } else {
            E::Unavailable
        }
    })?;
    let mut file = std::fs::File::from(fd);
    file.write_all(content.as_bytes())
        .map_err(|_| E::Unavailable)?;
    file.sync_all().map_err(|_| E::Unavailable)?;
    Ok(())
}

#[tauri::command]
pub(crate) async fn export_knowledge_draft(request: Draft) -> Result<bool> {
    knowledge::validate(&request.title, &request.content)?;
    #[cfg(target_os = "macos")]
    {
        let selected = tauri::async_runtime::spawn_blocking(|| {
            rfd::FileDialog::new()
                .add_filter("Markdown", &["md"])
                .set_file_name("draft.md")
                .save_file()
        })
        .await
        .map_err(|_| E::Unavailable)?;
        let Some(path) = selected else {
            return Ok(false);
        };
        export_new(&path, &request.content)?;
        Ok(true)
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err(E::Unavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn knowledge_search_and_reviewed_draft_are_bounded_and_attributed(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        use crate::{
            agent::definition::AgentId,
            agent_preferences::AgentProfile,
            collaboration::{Objective, Run, Status, Workflow},
            storage::DatabaseConfig,
        };
        let storage = Storage::initialize(&DatabaseConfig::in_memory())?.into_storage();
        let item = storage.knowledge_save(SaveNote {
            id: None,
            expected_version: None,
            title: "Runbook".into(),
            content: "# Incident\nRestart proposal only. [[Other]]".into(),
            draft: false,
        })?;
        storage.knowledge_save(SaveNote {
            id: None,
            expected_version: None,
            title: "Unrelated".into(),
            content: "Office inventory".into(),
            draft: false,
        })?;
        assert!(search(
            Query {
                text: "missing".into()
            },
            &storage
        )?
        .is_empty());
        let results = search(
            Query {
                text: "INCIDENT proposal".into(),
            },
            &storage,
        )?;
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].source.origin.as_ref().map(|o| o.document_id),
            Some(item.id)
        );
        let profiles = AgentId::ALL
            .into_iter()
            .map(AgentProfile::defaults)
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let mut room = storage.create_collaboration_room("Ops")?;
        let source = results[0].source.clone();
        let mut run = Run::new(
            "room-1/run-1".into(),
            Objective {
                workflow: Workflow::Operations,
                objective: "Assess".into(),
                sources: vec![source.clone()],
            },
            &profiles,
        )?;
        run.status = Status::Running;
        run.stages[0].status = Status::Running;
        run.accept(0,&serde_json::json!({"version":1,"stage":0,"agentId":"personal-assistant","status":"complete","summary":"Proposed diagnosis","findings":[],"evidence":[source.label],"limitations":["Offline only"]}).to_string())?;
        let stage_id = run.stages[0].id.clone();
        room.runs.push(run);
        storage.save_collaboration_room(&room)?;
        let result = draft(
            DraftRequest {
                room_id: room.id,
                run_id: "room-1/run-1".into(),
                stage_id,
            },
            &storage,
        )?;
        assert!(result.content.contains("Model-generated draft"));
        assert!(result.content.contains(&source.text));
        assert!(result.content.contains("SHA-256"));
        assert_eq!(storage.knowledge_items()?.len(), 2); // Review itself never saves.
        let saved = storage.knowledge_save(SaveNote {
            id: None,
            expected_version: None,
            title: result.title,
            content: result.content,
            draft: true,
        })?;
        assert_eq!(saved.kind, knowledge::Kind::GeneratedDraft);
        assert_eq!(
            search(
                Query {
                    text: "Proposed diagnosis".into()
                },
                &storage
            )?
            .len(),
            1
        );
        Ok(())
    }
    #[test]
    fn knowledge_ipc_rejects_paths_and_unknown_authority() {
        assert!(serde_json::from_str::<Import>(
            r#"{"id":null,"expectedVersion":null,"path":"/arbitrary"}"#
        )
        .is_err());
        assert!(serde_json::from_str::<Identity>(
            r#"{"id":1,"version":1,"destination":"/arbitrary"}"#
        )
        .is_err());
        assert!(serde_json::from_str::<SaveNote>(r#"{"id":null,"expectedVersion":null,"title":"Note","content":"hello","draft":false,"provider":"other"}"#).is_err());
    }
    #[test]
    fn knowledge_export_is_exact_portable_and_never_overwrites(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let d = tempfile::tempdir()?;
        let path = d.path().join("note.md");
        let text = "---\ntags: [ops]\n---\n# Draft\n[[Source]]\nSource attribution: K1_V1_P1\n";
        export_new(&path, text)?;
        assert_eq!(std::fs::read_to_string(&path)?, text);
        assert_eq!(export_new(&path, "replacement"), Err(E::DestinationExists));
        assert_eq!(std::fs::read_to_string(&path)?, text);
        assert_eq!(
            export_new(&d.path().join("note.html"), text),
            Err(E::UnsupportedFile)
        );
        #[cfg(unix)]
        {
            let link = d.path().join("linked.md");
            std::os::unix::fs::symlink(&path, &link)?;
            assert!(export_new(&link, "replacement").is_err());
            assert_eq!(std::fs::read_to_string(&path)?, text);
        }
        Ok(())
    }
    #[test]
    #[cfg(target_os = "macos")]
    fn knowledge_selected_reader_preserves_markdown_and_rejects_unsafe_input(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let d = tempfile::tempdir()?;
        let p = d.path().join("vault.md");
        let text = "---\r\ntags: [test]\r\n---\r\n[[Link]] <img src='https://invalid.test/x'>\r\n";
        std::fs::write(&p, text)?;
        assert_eq!(crate::documents::read_owner_selected_snapshot(&p)?, text);
        let bad = d.path().join("bad.md");
        std::fs::write(&bad, [0xff, 0xfe])?;
        assert!(crate::documents::read_owner_selected_snapshot(&bad).is_err());
        let unsupported = d.path().join("bad.html");
        std::fs::write(&unsupported, "safe")?;
        assert!(crate::documents::read_owner_selected_snapshot(&unsupported).is_err());
        let link = d.path().join("link.md");
        std::os::unix::fs::symlink(&p, &link)?;
        assert!(crate::documents::read_owner_selected_snapshot(&link).is_err());
        Ok(())
    }
}
