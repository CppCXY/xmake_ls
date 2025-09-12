use xmake_code_analysis::uri_to_file_path;
use lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DidSaveTextDocumentParams,
};
use std::time::Duration;

use crate::context::ServerContextSnapshot;

pub async fn on_did_open_text_document(
    context: ServerContextSnapshot,
    params: DidOpenTextDocumentParams,
) -> Option<()> {
    let mut analysis = context.analysis().write().await;
    let uri = params.text_document.uri;
    let text = params.text_document.text;
    let old_file_id = analysis.get_file_id(&uri);
    // check is filter file
    if old_file_id.is_none() {
        let workspace_manager = context.workspace_manager().read().await;
        if !workspace_manager.is_workspace_file(&uri) {
            return None;
        }
    }

    let file_id = analysis.update_file_by_uri(&uri, Some(text));
    let emmyrc = analysis.get_emmyrc();
    let interval = emmyrc.diagnostics.diagnostic_interval.unwrap_or(500);
    if let Some(file_id) = file_id {
        context
            .file_diagnostic()
            .add_diagnostic_task(file_id, interval)
            .await;
    }

    let mut workspace = context.workspace_manager().write().await;
    workspace.current_open_files.insert(uri);
    drop(workspace);

    Some(())
}

pub async fn on_did_save_text_document(
    context: ServerContextSnapshot,
    _: DidSaveTextDocumentParams,
) -> Option<()> {
    let emmyrc = context.analysis().read().await.get_emmyrc();
    if !emmyrc.workspace.enable_reindex {
        return Some(());
    }

    let mut duration = emmyrc.workspace.reindex_duration;
    // if duration is less than 1000ms, set it to 1000ms
    if duration < 1000 {
        duration = 1000;
    }
    let workspace = context.workspace_manager().read().await;
    workspace
        .reindex_workspace(Duration::from_millis(duration))
        .await;
    Some(())
}

pub async fn on_did_change_text_document(
    context: ServerContextSnapshot,
    params: DidChangeTextDocumentParams,
) -> Option<()> {
    let mut analysis = context.analysis().write().await;
    let uri = params.text_document.uri;
    let text = params.content_changes.first()?.text.clone();
    let old_file_id = analysis.get_file_id(&uri);
    // check is filter file
    if old_file_id.is_none() {
        let workspace_manager = context.workspace_manager().read().await;
        if !workspace_manager.is_workspace_file(&uri) {
            return None;
        }
    }

    let file_id = analysis.update_file_by_uri(&uri, Some(text));
    let emmyrc = analysis.get_emmyrc();
    let interval = emmyrc.diagnostics.diagnostic_interval.unwrap_or(500);
    drop(analysis);

    if emmyrc.workspace.enable_reindex {
        let workspace = context.workspace_manager().read().await;
        workspace.extend_reindex_delay().await;
        drop(workspace);
    }
    if let Some(file_id) = file_id {
        context
            .file_diagnostic()
            .add_diagnostic_task(file_id, interval)
            .await;
    }

    Some(())
}

pub async fn on_did_close_document(
    context: ServerContextSnapshot,
    params: DidCloseTextDocumentParams,
) -> Option<()> {
    let uri = &params.text_document.uri;
    let mut workspace = context.workspace_manager().write().await;
    workspace
        .current_open_files
        .remove(&params.text_document.uri);
    drop(workspace);

    // 如果关闭后文件不存在, 则移除
    if let Some(file_path) = uri_to_file_path(uri) {
        if !file_path.exists() {
            let mut mut_analysis = context.analysis().write().await;
            mut_analysis.remove_file_by_uri(uri);
            drop(mut_analysis);

            context
                .file_diagnostic()
                .clear_file_diagnostics(uri.clone())
                .await;

            return Some(());
        }
    }

    let analysis = context.analysis().read().await;
    let file_id = analysis.get_file_id(uri)?;
    let module_info = analysis
        .compilation
        .get_db()
        .get_module_index()
        .get_module(file_id);
    if module_info.is_none() {
        drop(analysis);
        let mut mut_analysis = context.analysis().write().await;
        mut_analysis.remove_file_by_uri(uri);
        drop(mut_analysis);
        // 发送空诊断消息以清除客户端显示的诊断
        context
            .file_diagnostic()
            .clear_file_diagnostics(uri.clone())
            .await;
    }

    Some(())
}
