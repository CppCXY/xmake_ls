use lsp_types::ShowMessageParams;
use xmake_code_analysis::{Emmyrc, EmmyrcLuaVersion};

use crate::context::ServerContextSnapshot;

pub async fn xmake_initialize(emmyrc: &mut Emmyrc, context: &ServerContextSnapshot) {
    emmyrc.runtime.extensions = vec!["xmake.lua".to_string()];
    emmyrc.runtime.require_like_function = vec!["import".to_string()];
    emmyrc.runtime.version = EmmyrcLuaVersion::Lua54;
    emmyrc.runtime.require_pattern = vec!["?.xmake.lua".to_string()];

    if context.xmake().check_available().await {
        log::info!("xmake is available");
    } else {
        log::warn!("xmake is not available");
        let message = ShowMessageParams {
            typ: lsp_types::MessageType::ERROR,
            message: "xmake is not available. Please ensure xmake is installed and accessible in your system PATH.".to_string(),
        };

        context.client().show_message(message);
    }
}
