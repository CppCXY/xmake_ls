use lsp_types::SetTraceParams;

use crate::context::ServerContextSnapshot;

pub async fn on_set_trace(_: ServerContextSnapshot, _: SetTraceParams) -> Option<()> {
    Some(())
}
