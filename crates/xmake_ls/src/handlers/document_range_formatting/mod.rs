mod external_range_format;

use lsp_types::{
    ClientCapabilities, DocumentRangeFormattingParams, OneOf, Position, Range, ServerCapabilities,
    TextEdit,
};
use tokio_util::sync::CancellationToken;

use crate::{
    context::ServerContextSnapshot,
    handlers::{
        document_formatting::FormattingOptions,
        document_range_formatting::external_range_format::external_tool_range_format,
    },
};

use super::RegisterCapabilities;

pub async fn on_range_formatting_handler(
    context: ServerContextSnapshot,
    params: DocumentRangeFormattingParams,
    _: CancellationToken,
) -> Option<Vec<TextEdit>> {
    let uri = params.text_document.uri;
    let request_range = params.range;
    let analysis = context.analysis().read().await;
    let workspace_manager = context.workspace_manager().read().await;
    let client_id = workspace_manager.client_config.client_id;
    let file_id = analysis.get_file_id(&uri)?;
    let syntax_tree = analysis
        .compilation
        .get_db()
        .get_vfs()
        .get_syntax_tree(&file_id)?;

    if syntax_tree.has_syntax_errors() {
        return None;
    }
    let emmyrc = analysis.get_emmyrc();
    let document = analysis
        .compilation
        .get_db()
        .get_vfs()
        .get_document(&file_id)?;
    let file_path = document.get_file_path();
    let normalized_path = file_path.to_string_lossy().to_string().replace("\\", "/");
    let formatting_options = FormattingOptions {
        indent_size: params.options.tab_size,
        use_tabs: !params.options.insert_spaces,
        insert_final_newline: params.options.insert_final_newline.unwrap_or(true),
        non_standard_symbol: !emmyrc.runtime.nonstandard_symbol.is_empty(),
    };
    let formatted_result = if let Some(external_tool) = &emmyrc.format.external_tool_range_format {
        external_tool_range_format(
            external_tool,
            &document,
            &request_range,
            &normalized_path,
            formatting_options,
        )
        .await?
    } else {
        return None;
    };

    let mut formatted_text = formatted_result.text;
    if client_id.is_intellij() || client_id.is_other() {
        formatted_text = formatted_text.replace("\r\n", "\n");
    }

    let text_edit = TextEdit {
        range: Range {
            start: Position {
                line: formatted_result.start_line as u32,
                character: formatted_result.start_col as u32,
            },
            end: Position {
                line: formatted_result.end_line as u32 + 1,
                character: 0,
            },
        },
        new_text: formatted_text,
    };

    Some(vec![text_edit])
}

#[allow(unused)]
pub struct DocumentRangeFormattingCapabilities;

impl RegisterCapabilities for DocumentRangeFormattingCapabilities {
    fn register_capabilities(server_capabilities: &mut ServerCapabilities, _: &ClientCapabilities) {
        server_capabilities.document_range_formatting_provider = Some(OneOf::Left(true));
    }
}
