pub mod agent;
mod agent_chat;
mod agent_chat_tauri;
mod agent_preferences;
mod app_info;
pub mod approvals;
pub mod audit;
pub mod credentials;
pub mod documents;
mod error;
pub mod memory;
pub mod menu_bar;
mod personal_assistant_direct;
mod personal_assistant_direct_tauri;
mod personal_assistant_v0;
pub mod policy;
mod research_knowledge_demo_lifecycle;
mod research_knowledge_demo_lifecycle_tauri;
mod research_knowledge_demo_projection;
mod startup;
pub mod storage;
pub mod tools;

pub use app_info::{current_app_info, AppInfo};
pub use error::AppError;
pub use personal_assistant_v0::{
    PersonalAssistantV0CompletedSnapshot, PersonalAssistantV0CompletedUpdate,
    PersonalAssistantV0Error, PersonalAssistantV0FailedSnapshot, PersonalAssistantV0FailedUpdate,
    PersonalAssistantV0Failure, PersonalAssistantV0FailureCode, PersonalAssistantV0Host,
    PersonalAssistantV0PresentationHandle, PersonalAssistantV0SequenceUpdate,
    PersonalAssistantV0Snapshot, PersonalAssistantV0Start, PersonalAssistantV0TextDeltaUpdate,
    PersonalAssistantV0TextSnapshot, PersonalAssistantV0Update, PersonalAssistantV0UpdateBatch,
};
pub use research_knowledge_demo_lifecycle::{
    ResearchKnowledgeDemoHost, ResearchKnowledgeDemoLifecycleEntry,
    ResearchKnowledgeDemoLifecycleError, ResearchKnowledgeDemoLifecycleEventKind,
    ResearchKnowledgeDemoLifecycleSnapshot, ResearchKnowledgeDemoLifecycleState,
    ResearchKnowledgeDemoLifecycleTransition,
};
pub use research_knowledge_demo_projection::{
    current_research_knowledge_demo_projection, ResearchKnowledgeDemoProjection,
    ResearchKnowledgeDemoProjectionError,
};
pub use tools::types::{PermissionKind, RiskClass};

pub fn run() -> Result<(), AppError> {
    let app = tauri::Builder::default()
        .manage(personal_assistant_direct_tauri::DirectState::default())
        .manage(agent_chat_tauri::AgentChatState::default())
        .manage(
            research_knowledge_demo_lifecycle_tauri::ResearchKnowledgeDemoLifecycleTauriState::new(
            ),
        )
        .setup(|app| {
            startup::initialize_tauri_app(app)?;
            menu_bar::initialize_tauri_app(app)?;
            Ok(())
        })
        .on_window_event(menu_bar::handle_window_event)
        .invoke_handler(tauri::generate_handler![
            app_info::get_app_info,
            research_knowledge_demo_projection::get_research_knowledge_demo_projection,
            research_knowledge_demo_lifecycle_tauri::get_research_knowledge_demo_lifecycle_snapshot,
            research_knowledge_demo_lifecycle_tauri::start_research_knowledge_demo_lifecycle,
            research_knowledge_demo_lifecycle_tauri::advance_research_knowledge_demo_lifecycle,
            research_knowledge_demo_lifecycle_tauri::cancel_research_knowledge_demo_lifecycle,
            personal_assistant_direct_tauri::start_personal_assistant_direct,
            personal_assistant_direct_tauri::poll_personal_assistant_direct,
            personal_assistant_direct_tauri::cancel_personal_assistant_direct,
            agent_chat_tauri::list_agent_preferences,
            agent_chat_tauri::save_agent_preferences,
            agent_chat_tauri::clear_agent_note,
            agent_chat_tauri::restore_agent_defaults,
            agent_chat_tauri::list_agent_connections,
            agent_chat_tauri::start_agent_conversation,
            agent_chat_tauri::send_agent_message,
            agent_chat_tauri::poll_agent_conversation,
            agent_chat_tauri::cancel_agent_conversation
        ])
        .build(tauri::generate_context!())
        .map_err(AppError::from)?;

    app.run(|app_handle, event| menu_bar::handle_run_event(app_handle, &event));
    Ok(())
}
