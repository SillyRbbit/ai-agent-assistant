//! Shared dispatch for native-prepared chat and collaboration requests.
//!
//! Callers retain approval, request construction, host validation and generation
//! ownership. This module only dispatches the closed adapter and scopes transport
//! diagnostics; it does not choose settings, acquire credentials, retry or finish
//! the owning attempt. Requests deliberately have no Debug implementation.
use std::time::Duration;

#[cfg(test)]
use std::sync::Arc;

use crate::agent_preferences::ReasoningEffort;
use crate::personal_assistant_direct::{self as provider, ApiKey, DirectError, ProviderEvent};
use crate::{anthropic, codex_connection, local_models};

pub(crate) enum AdapterRequest {
    Codex {
        setup: codex_connection::Setup,
        model: String,
        effort: ReasoningEffort,
        input: String,
    },
    CollaborationCodex {
        setup: codex_connection::Setup,
        model: String,
        effort: ReasoningEffort,
        input: String,
    },
    Simulation,
    Anthropic {
        key: anthropic::AnthropicKey,
        body: Vec<u8>,
    },
    Local {
        endpoint: String,
        key: Option<local_models::LocalKey>,
        body: Vec<u8>,
    },
    Openai {
        key: ApiKey,
        body: Vec<u8>,
    },
    #[cfg(test)]
    Failure(DirectError),
    #[cfg(test)]
    Events(Vec<ProviderEvent>),
    #[cfg(test)]
    Stepped(tokio::sync::mpsc::Receiver<(ProviderEvent, tokio::sync::oneshot::Sender<()>)>),
    #[cfg(test)]
    Pending(Arc<std::sync::atomic::AtomicBool>),
}

pub(crate) async fn run_adapter(
    adapter: AdapterRequest,
    mut emit: impl FnMut(ProviderEvent) -> Result<(), DirectError>,
) -> Result<(), DirectError> {
    match adapter {
        AdapterRequest::CollaborationCodex {
            setup,
            model,
            effort,
            input,
        } => codex_connection::run_collaboration(setup, model, effort, input, emit).await,
        AdapterRequest::Codex {
            setup,
            model,
            effort,
            input,
        } => codex_connection::run(setup, model, effort, input, emit).await,
        #[cfg(test)]
        AdapterRequest::Failure(error) => Err(error),
        #[cfg(test)]
        AdapterRequest::Events(events) => {
            for event in events {
                emit(event)?;
            }
            Ok(())
        }
        #[cfg(test)]
        AdapterRequest::Stepped(mut events) => {
            while let Some((event, accepted)) = events.recv().await {
                emit(event)?;
                let _ = accepted.send(());
            }
            Ok(())
        }
        #[cfg(test)]
        AdapterRequest::Pending(dropped) => {
            struct Dropped(Arc<std::sync::atomic::AtomicBool>);
            impl Drop for Dropped {
                fn drop(&mut self) {
                    self.0.store(true, std::sync::atomic::Ordering::SeqCst);
                }
            }
            let _guard = Dropped(dropped);
            std::future::pending().await
        }
        AdapterRequest::Anthropic { key, body } => anthropic::run(key, body, emit).await,
        AdapterRequest::Local {
            endpoint,
            key,
            body,
        } => local_models::run(&endpoint, key, body, emit).await,
        AdapterRequest::Openai { key, body } => provider::run_configured(key, body, emit).await,
        AdapterRequest::Simulation => {
            emit(ProviderEvent::Started("resp_cortexa_simulation".into()))?;
            for part in [
                "Simulation only — no hosted request was made. ",
                "This native conversation is bound to your selected agent and saved settings. ",
                "Switch to an available connection for generated text advice; no tools or actions run.",
            ] {
                tokio::time::sleep(Duration::from_millis(180)).await;
                emit(ProviderEvent::Delta(part.into()))?;
            }
            emit(ProviderEvent::Completed)
        }
    }
}

pub(crate) async fn run_adapter_traced(
    adapter: AdapterRequest,
    observer: crate::diagnostics::Observer,
    mut emit: impl FnMut(ProviderEvent) -> Result<(), DirectError>,
) -> Result<(), DirectError> {
    observer.dispatch();
    let _transport = crate::diagnostics::TransportScope(observer.clone());
    crate::diagnostics::CURRENT
        .scope(
            observer.clone(),
            run_adapter(adapter, |event| {
                let text = matches!(&event, ProviderEvent::Delta(text) if !text.is_empty());
                emit(event)?;
                observer.first(text);
                Ok(())
            }),
        )
        .await
}
