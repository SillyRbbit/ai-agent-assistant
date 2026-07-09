#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::ExitCode;

fn main() -> ExitCode {
    match ai_agent_assistant_lib::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("AI Agent Assistant failed to start: {error}");
            ExitCode::FAILURE
        }
    }
}
