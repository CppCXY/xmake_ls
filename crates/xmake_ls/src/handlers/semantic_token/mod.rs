mod build_semantic_tokens;
mod semantic_token_builder;

use crate::context::{ClientId, ServerContextSnapshot};
use build_semantic_tokens::build_semantic_tokens;
use lsp_types::{
    ClientCapabilities, SemanticTokens, SemanticTokensFullOptions, SemanticTokensLegend,
    SemanticTokensOptions, SemanticTokensParams, SemanticTokensResult,
    SemanticTokensServerCapabilities, ServerCapabilities,
};
pub use semantic_token_builder::{SEMANTIC_TOKEN_MODIFIERS, SEMANTIC_TOKEN_TYPES};
use tokio_util::sync::CancellationToken;
use xmake_code_analysis::{FileId, XmakeAnalysis};

use super::RegisterCapabilities;

pub async fn on_semantic_token_handler(
    context: ServerContextSnapshot,
    params: SemanticTokensParams,
    _: CancellationToken,
) -> Option<SemanticTokensResult> {
    let uri = params.text_document.uri;
    let analysis = context.analysis().read().await;
    let file_id = analysis.get_file_id(&uri)?;

    let workspace_manager = context.workspace_manager().read().await;
    let client_id = workspace_manager.client_config.client_id;
    let _ = workspace_manager;

    semantic_token(
        &analysis,
        file_id,
        &context.client_capabilities(),
        client_id,
    )
}

pub fn semantic_token(
    analysis: &XmakeAnalysis,
    file_id: FileId,
    client_capabilities: &ClientCapabilities,
    client_id: ClientId,
) -> Option<SemanticTokensResult> {
    let semantic_model = analysis.compilation.get_semantic_model(file_id)?;
    let emmyrc = semantic_model.get_emmyrc();
    if !emmyrc.semantic_tokens.enable {
        return None;
    }

    let result = build_semantic_tokens(
        &semantic_model,
        supports_multiline_tokens(client_capabilities),
        client_id,
        emmyrc,
    )?;

    Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: result,
    }))
}

pub struct SemanticTokenCapabilities;

impl RegisterCapabilities for SemanticTokenCapabilities {
    fn register_capabilities(
        server_capabilities: &mut ServerCapabilities,
        _client_capabilities: &ClientCapabilities,
    ) {
        server_capabilities.semantic_tokens_provider = Some(
            SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                legend: SemanticTokensLegend {
                    token_modifiers: SEMANTIC_TOKEN_MODIFIERS.iter().cloned().collect(),
                    token_types: SEMANTIC_TOKEN_TYPES.iter().cloned().collect(),
                },
                full: Some(SemanticTokensFullOptions::Bool(true)),
                ..Default::default()
            }),
        );
    }
}

fn supports_multiline_tokens(client_capability: &ClientCapabilities) -> bool {
    if let Some(text_document) = &client_capability.text_document {
        if let Some(support) = &text_document.semantic_tokens {
            if let Some(support) = &support.multiline_token_support {
                return *support;
            }
        }
    }

    false
}
