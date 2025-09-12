mod build_inline_values;

use build_inline_values::build_inline_values;
use lsp_types::{ClientCapabilities, InlineValue, InlineValueParams, OneOf, ServerCapabilities};
use tokio_util::sync::CancellationToken;

use crate::context::ServerContextSnapshot;

use super::RegisterCapabilities;

pub async fn on_inline_values_handler(
    context: ServerContextSnapshot,
    params: InlineValueParams,
    _: CancellationToken,
) -> Option<Vec<InlineValue>> {
    let uri = params.text_document.uri;
    let stop_location = params.context.stopped_location;
    let stop_position = stop_location.start;
    let analysis = context.analysis().read().await;
    let file_id = analysis.get_file_id(&uri)?;
    let mut semantic_model = analysis.compilation.get_semantic_model(file_id)?;
    if !semantic_model.get_emmyrc().inline_values.enable {
        return None;
    }

    build_inline_values(&mut semantic_model, stop_position)
}

pub struct InlineValuesCapabilities;

impl RegisterCapabilities for InlineValuesCapabilities {
    fn register_capabilities(server_capabilities: &mut ServerCapabilities, _: &ClientCapabilities) {
        server_capabilities.inline_value_provider = Some(OneOf::Left(true));
    }
}
