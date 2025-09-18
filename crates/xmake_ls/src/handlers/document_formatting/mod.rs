mod external_format;
mod format_diff;

use emmylua_parser::LuaAst;
use lsp_types::{
    ClientCapabilities, DocumentFormattingParams, OneOf, ServerCapabilities, TextEdit,
};
use tokio_util::sync::CancellationToken;

use crate::{
    context::ServerContextSnapshot, handlers::document_formatting::format_diff::format_diff,
};
pub use external_format::{FormattingRange, external_tool_format};
use xmake_formatter::reformat_node;

use super::RegisterCapabilities;

pub async fn on_formatting_handler(
    context: ServerContextSnapshot,
    params: DocumentFormattingParams,
    _: CancellationToken,
) -> Option<Vec<TextEdit>> {
    let uri = params.text_document.uri;
    let analysis = context.analysis().read().await;
    let workspace_manager = context.workspace_manager().read().await;
    let client_id = workspace_manager.client_config.client_id;
    let emmyrc = analysis.get_emmyrc();

    let file_id = analysis.get_file_id(&uri)?;
    let syntax_tree = analysis
        .compilation
        .get_db()
        .get_vfs()
        .get_syntax_tree(&file_id)?;

    if syntax_tree.has_syntax_errors() {
        return None;
    }

    let document = analysis
        .compilation
        .get_db()
        .get_vfs()
        .get_document(&file_id)?;
    let text = document.get_text();
    let file_path = document.get_file_path();
    let normalized_path = file_path.to_string_lossy().to_string().replace("\\", "/");
    let formatting_options = FormattingOptions {
        indent_size: params.options.tab_size,
        use_tabs: !params.options.insert_spaces,
        insert_final_newline: params.options.insert_final_newline.unwrap_or(true),
        non_standard_symbol: !emmyrc.runtime.nonstandard_symbol.is_empty(),
    };

    let mut formatted_text = if let Some(external_config) = &emmyrc.format.external_tool {
        external_tool_format(
            &external_config,
            text,
            &normalized_path,
            None,
            formatting_options,
        )
        .await?
    } else {
        let chunk = syntax_tree.get_chunk_node();
        let ast_node = LuaAst::LuaChunk(chunk);
        reformat_node(&ast_node)
    };

    if client_id.is_intellij() || client_id.is_other() {
        formatted_text = formatted_text.replace("\r\n", "\n");
    }

    let replace_all_limit = 50;
    let text_edits = if emmyrc.format.use_diff {
        // Use line-based diff algorithm if the diff is not too large
        format_diff(text, &formatted_text, &document, replace_all_limit)
    } else {
        let document_range = document.get_document_lsp_range();
        vec![TextEdit {
            range: document_range,
            new_text: formatted_text.to_string(),
        }]
    };

    Some(text_edits)
}

pub struct FormattingOptions {
    pub indent_size: u32,
    pub use_tabs: bool,
    pub insert_final_newline: bool,
    pub non_standard_symbol: bool,
}

pub struct DocumentFormattingCapabilities;

impl RegisterCapabilities for DocumentFormattingCapabilities {
    fn register_capabilities(server_capabilities: &mut ServerCapabilities, _: &ClientCapabilities) {
        server_capabilities.document_formatting_provider = Some(OneOf::Left(true));
    }
}
