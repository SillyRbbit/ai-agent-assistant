use std::process::ExitCode;

use ai_agent_assistant_lib::credentials::{
    probe_cloudflare_access_credentials, CloudflareAccessCredentialStatus,
};

fn main() -> ExitCode {
    match probe_cloudflare_access_credentials() {
        Ok(CloudflareAccessCredentialStatus::Available) => {
            println!("Cloudflare Access fake Keychain proof: available");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Cloudflare Access fake Keychain proof: {error}");
            ExitCode::FAILURE
        }
    }
}
