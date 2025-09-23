use super::ClientConfig;
use crate::context::ServerContextSnapshot;
use crate::handlers::initialized::client_config::default_config::get_client_config_default;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
struct VscodeFilesConfig {
    exclude: Option<HashMap<String, bool>>,
    associations: Option<HashMap<String, String>>,
    encoding: Option<String>,
}

pub async fn get_client_config_vscode(
    context: &ServerContextSnapshot,
    config: &mut ClientConfig,
) -> Option<()> {
    get_client_config_default(context, config, None).await;
    Some(())
}
