mod highlight_tokens;

use emmylua_parser::{LuaAstNode, LuaTokenKind};
use highlight_tokens::highlight_tokens;
use lsp_types::{
    ClientCapabilities, DocumentHighlight, DocumentHighlightParams, OneOf, ServerCapabilities,
};
use rowan::TokenAtOffset;
use tokio_util::sync::CancellationToken;

use crate::context::ServerContextSnapshot;

use super::RegisterCapabilities;

pub async fn on_document_highlight_handler(
    context: ServerContextSnapshot,
    params: DocumentHighlightParams,
    _: CancellationToken,
) -> Option<Vec<DocumentHighlight>> {
    let uri = params.text_document_position_params.text_document.uri;
    let analysis = context.analysis().read().await;
    let file_id = analysis.get_file_id(&uri)?;
    let position = params.text_document_position_params.position;
    let mut semantic_model = analysis.compilation.get_semantic_model(file_id)?;
    let root = semantic_model.get_root();
    let position_offset = {
        let document = semantic_model.get_document();
        document.get_offset(position.line as usize, position.character as usize)?
    };

    if position_offset > root.syntax().text_range().end() {
        return None;
    }

    let token = match root.syntax().token_at_offset(position_offset) {
        TokenAtOffset::Single(token) => token,
        TokenAtOffset::Between(left, right) => {
            if left.kind() == LuaTokenKind::TkName.into() {
                left
            } else {
                right
            }
        }
        TokenAtOffset::None => {
            return None;
        }
    };

    highlight_tokens(&mut semantic_model, token)
}

pub struct DocumentHighlightCapabilities;

impl RegisterCapabilities for DocumentHighlightCapabilities {
    fn register_capabilities(server_capabilities: &mut ServerCapabilities, _: &ClientCapabilities) {
        server_capabilities.document_highlight_provider = Some(OneOf::Left(true));
    }
}
