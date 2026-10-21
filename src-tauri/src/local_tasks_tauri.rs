//! Four fixed owner-operated task commands. No paths, tools or model authority.
use crate::{
    local_tasks::{LocalTask, Result, SaveTask, SetCompletion, TaskIdentity},
    storage::Storage,
};
#[tauri::command]
pub(crate) fn list_local_tasks(storage: tauri::State<'_, Storage>) -> Result<Vec<LocalTask>> {
    storage.list_local_tasks()
}
#[tauri::command]
pub(crate) fn save_local_task(
    request: SaveTask,
    storage: tauri::State<'_, Storage>,
) -> Result<LocalTask> {
    storage.save_local_task(request)
}
#[tauri::command]
pub(crate) fn set_local_task_completion(
    request: SetCompletion,
    storage: tauri::State<'_, Storage>,
) -> Result<LocalTask> {
    storage.set_local_task_completion(request)
}
#[tauri::command]
pub(crate) fn remove_local_task(
    request: TaskIdentity,
    storage: tauri::State<'_, Storage>,
) -> Result<()> {
    storage.remove_local_task(request)
}
