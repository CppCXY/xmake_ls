use std::sync::Arc;

pub use super::checker::DiagnosticContext;
use super::{checker::check_file, lua_diagnostic_config::LuaDiagnosticConfig};
use crate::{Emmyrc, FileId, LuaCompilation};
use lsp_types::Diagnostic;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub struct LuaDiagnostic {
    enable: bool,
    config: Arc<LuaDiagnosticConfig>,
}

impl LuaDiagnostic {
    pub fn new() -> Self {
        Self {
            enable: true,
            config: Arc::new(LuaDiagnosticConfig::default()),
        }
    }

    pub fn update_config(&mut self, emmyrc: Arc<Emmyrc>) {
        self.enable = emmyrc.diagnostics.enable;
        self.config = LuaDiagnosticConfig::new(&emmyrc).into();
    }

    pub fn diagnose_file(
        &self,
        compilation: &LuaCompilation,
        file_id: FileId,
        cancel_token: CancellationToken,
    ) -> Option<Vec<Diagnostic>> {
        if !self.enable {
            return None;
        }

        if cancel_token.is_cancelled() {
            return None;
        }

        let db = compilation.get_db();
        if let Some(module_info) = db.get_module_index().get_workspace_id(file_id) {
            if !module_info.is_main() {
                return None;
            }
        }

        let mut semantic_model = compilation.get_semantic_model(file_id)?;
        let mut context = DiagnosticContext::new(file_id, db, self.config.clone());

        check_file(&mut context, &mut semantic_model);

        Some(context.get_diagnostics())
    }
}
