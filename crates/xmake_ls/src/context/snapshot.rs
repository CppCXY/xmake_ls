use lsp_types::ClientCapabilities;
use std::sync::Arc;
use tokio::sync::RwLock;

use xmake_code_analysis::XmakeAnalysis;

use super::{
    client::ClientProxy, file_diagnostic::FileDiagnostic, status_bar::StatusBar,
    workspace_manager::WorkspaceManager,
};

#[derive(Clone)]
pub struct ServerContextSnapshot {
    inner: Arc<ServerContextInner>,
}

impl ServerContextSnapshot {
    pub fn new(inner: Arc<ServerContextInner>) -> Self {
        Self { inner }
    }

    pub fn analysis(&self) -> &RwLock<XmakeAnalysis> {
        &self.inner.analysis
    }

    pub fn client(&self) -> &ClientProxy {
        &self.inner.client
    }

    pub fn file_diagnostic(&self) -> &FileDiagnostic {
        &self.inner.file_diagnostic
    }

    pub fn workspace_manager(&self) -> &RwLock<WorkspaceManager> {
        &self.inner.workspace_manager
    }

    pub fn status_bar(&self) -> &StatusBar {
        &self.inner.status_bar
    }

    pub fn client_capabilities(&self) -> &ClientCapabilities {
        &self.inner.client_capabilities
    }
}

pub struct ServerContextInner {
    pub analysis: Arc<RwLock<XmakeAnalysis>>,
    pub client: Arc<ClientProxy>,
    pub file_diagnostic: Arc<FileDiagnostic>,
    pub workspace_manager: Arc<RwLock<WorkspaceManager>>,
    pub status_bar: Arc<StatusBar>,
    pub client_capabilities: Arc<ClientCapabilities>,
}
