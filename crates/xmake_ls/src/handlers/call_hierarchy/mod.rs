mod build_call_hierarchy;

use build_call_hierarchy::{
    CallHierarchyItemData, build_call_hierarchy_item, build_incoming_hierarchy,
};
use emmylua_parser::{LuaAstNode, LuaTokenKind};
use lsp_types::{
    CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, CallHierarchyItem,
    CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams,
    ClientCapabilities, ServerCapabilities,
};
use rowan::TokenAtOffset;
use tokio_util::sync::CancellationToken;
use xmake_code_analysis::SemanticDeclLevel;

use crate::context::ServerContextSnapshot;

use super::RegisterCapabilities;

pub async fn on_prepare_call_hierarchy_handler(
    context: ServerContextSnapshot,
    params: CallHierarchyPrepareParams,
    _: CancellationToken,
) -> Option<Vec<CallHierarchyItem>> {
    let uri = params.text_document_position_params.text_document.uri;
    let analysis = context.analysis().read().await;
    let file_id = analysis.get_file_id(&uri)?;
    let position = params.text_document_position_params.position;
    let semantic_model = analysis.compilation.get_semantic_model(file_id)?;
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

    let semantic_decl =
        semantic_model.find_decl(token.clone().into(), SemanticDeclLevel::default())?;

    Some(vec![build_call_hierarchy_item(
        &semantic_model,
        semantic_decl,
    )?])
}

pub async fn on_incoming_calls_handler(
    context: ServerContextSnapshot,
    params: CallHierarchyIncomingCallsParams,
    _: CancellationToken,
) -> Option<Vec<CallHierarchyIncomingCall>> {
    let item = params.item;
    let data = item.data.as_ref()?;
    let data = serde_json::from_value::<CallHierarchyItemData>(data.clone()).ok()?;
    let analysis = context.analysis().read().await;
    let semantic_model = analysis.compilation.get_semantic_model(data.file_id)?;
    let semantic_decl_id = data.semantic_decl;

    build_incoming_hierarchy(&semantic_model, &analysis.compilation, semantic_decl_id)
}

pub async fn on_outgoing_calls_handler(
    _: ServerContextSnapshot,
    _: CallHierarchyOutgoingCallsParams,
    _: CancellationToken,
) -> Option<Vec<CallHierarchyOutgoingCall>> {
    None
}

pub struct CallHierarchyCapabilities;

impl RegisterCapabilities for CallHierarchyCapabilities {
    fn register_capabilities(server_capabilities: &mut ServerCapabilities, _: &ClientCapabilities) {
        server_capabilities.call_hierarchy_provider = Some(true.into());
    }
}
