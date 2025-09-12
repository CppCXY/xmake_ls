use super::RegisterCapabilities;
use crate::context::ServerContextSnapshot;
use emmylua_parser::{LuaAstNode, LuaDocDescription};
use lsp_types::{
    ClientCapabilities, SelectionRange, SelectionRangeParams, SelectionRangeProviderCapability,
    ServerCapabilities,
};
use rowan::TokenAtOffset;
use tokio_util::sync::CancellationToken;

pub async fn on_document_selection_range_handle(
    context: ServerContextSnapshot,
    params: SelectionRangeParams,
    _: CancellationToken,
) -> Option<Vec<SelectionRange>> {
    let uri = params.text_document.uri;
    let position = params.positions;

    let analysis = context.analysis().read().await;
    let file_id = analysis.get_file_id(&uri)?;
    let semantic_model = analysis.compilation.get_semantic_model(file_id)?;
    let document = semantic_model.get_document();
    let root = semantic_model.get_root();
    let mut result = Vec::new();
    for pos in position {
        let offset = document.get_offset(pos.line as usize, pos.character as usize)?;
        let token = match root.syntax().token_at_offset(offset) {
            TokenAtOffset::Single(token) => token,
            TokenAtOffset::Between(_, right) => right,
            TokenAtOffset::None => {
                return None;
            }
        };

        let mut ranges = Vec::new();

        let description = token
            .parent()
            .and_then(|parent| LuaDocDescription::cast(parent));
        if let Some(_) = description {
            continue;
        } else {
            let range = token.text_range();
            ranges.push(range);
        }

        for ancestor in token.parent_ancestors() {
            let range = ancestor.text_range();
            ranges.push(range);
        }

        let mut parent: Option<Box<SelectionRange>> = None;
        for range in ranges.into_iter().rev() {
            let lsp_range = document.to_lsp_range(range)?;
            let selection_range = SelectionRange {
                range: lsp_range,
                parent,
            };
            parent = Some(Box::new(selection_range));
        }
        if let Some(selection_range) = parent {
            result.push(*selection_range);
        }
    }

    Some(result)
}

pub struct DocumentSelectionRangeCapabilities;

impl RegisterCapabilities for DocumentSelectionRangeCapabilities {
    fn register_capabilities(server_capabilities: &mut ServerCapabilities, _: &ClientCapabilities) {
        server_capabilities.selection_range_provider =
            Some(SelectionRangeProviderCapability::Simple(true));
    }
}
