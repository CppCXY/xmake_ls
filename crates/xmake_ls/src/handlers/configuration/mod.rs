use lsp_types::{ClientCapabilities, DidChangeConfigurationParams, ServerCapabilities};

use crate::{context::ServerContextSnapshot, handlers::initialized::get_client_config};

use super::RegisterCapabilities;

pub async fn on_did_change_configuration(
    context: ServerContextSnapshot,
    params: DidChangeConfigurationParams,
) -> Option<()> {
    let pretty_json = serde_json::to_string_pretty(&params).ok()?;
    log::info!("on_did_change_configuration: {}", pretty_json);

    let workspace_manager = context.workspace_manager().read().await;
    if !workspace_manager.is_workspace_initialized() {
        return Some(());
    }

    let client_id = workspace_manager.client_config.client_id;
    if client_id.is_vscode() {
        return Some(());
    }

    drop(workspace_manager);

    let supports_config_request = context
        .client_capabilities()
        .workspace
        .as_ref()?
        .configuration
        .unwrap_or_default();

    log::info!("change config client_id: {:?}", client_id);
    let new_client_config = get_client_config(&context, client_id, supports_config_request).await;
    let mut workspace_manager = context.workspace_manager().write().await;
    workspace_manager.client_config = new_client_config;

    log::info!("reloading workspace folders");
    workspace_manager.reload_workspace().await;
    Some(())
}

pub struct ConfigurationCapabilities;

impl RegisterCapabilities for ConfigurationCapabilities {
    fn register_capabilities(_: &mut ServerCapabilities, _: &ClientCapabilities) {}
}
