use lsp_types::{DidChangeWatchedFilesParams, FileChangeType, Uri};
use xmake_code_analysis::{read_file_with_encoding, uri_to_file_path};

use crate::context::ServerContextSnapshot;

pub async fn on_did_change_watched_files(
    context: ServerContextSnapshot,
    params: DidChangeWatchedFilesParams,
) -> Option<()> {
    let workspace = context.workspace_manager().read().await;
    let mut analysis = context.analysis().write().await;
    let emmyrc = analysis.get_emmyrc();
    let encoding = &emmyrc.workspace.encoding;
    let interval = emmyrc.diagnostics.diagnostic_interval.unwrap_or(500);
    let mut watched_lua_files: Vec<(Uri, Option<String>)> = Vec::new();
    // let
    for file_event in params.changes.into_iter() {
        let file_type = get_file_type(&file_event.uri);
        match file_type {
            Some(WatchedFileType::XMakeLua) => {
                if file_event.typ == FileChangeType::DELETED {
                    analysis.remove_file_by_uri(&file_event.uri);
                    // 发送空诊断消息以清除客户端显示的诊断
                    context
                        .file_diagnostic()
                        .clear_file_diagnostics(file_event.uri)
                        .await;
                    continue;
                }

                if !workspace.current_open_files.contains(&file_event.uri) {
                    if !workspace.is_workspace_file(&file_event.uri) {
                        continue;
                    }

                    collect_lua_files(
                        &mut watched_lua_files,
                        file_event.uri,
                        file_event.typ,
                        encoding,
                    );
                }
            }
            None => {}
        }
    }

    let file_ids = analysis.update_files_by_uri(watched_lua_files);
    context
        .file_diagnostic()
        .add_files_diagnostic_task(file_ids, interval)
        .await;

    Some(())
}

fn collect_lua_files(
    watched_lua_files: &mut Vec<(Uri, Option<String>)>,
    uri: Uri,
    file_change_event: FileChangeType,
    encoding: &str,
) {
    match file_change_event {
        FileChangeType::CREATED | FileChangeType::CHANGED => {
            let path = uri_to_file_path(&uri).unwrap();
            if let Some(text) = read_file_with_encoding(&path, encoding) {
                watched_lua_files.push((uri, Some(text)));
            }
        }
        FileChangeType::DELETED => {
            watched_lua_files.push((uri, None));
        }
        _ => {}
    }
}

enum WatchedFileType {
    XMakeLua,
}

fn get_file_type(uri: &Uri) -> Option<WatchedFileType> {
    let path = uri_to_file_path(uri)?;
    let file_name = path.file_name()?.to_str()?;
    match file_name {
        _ => Some(WatchedFileType::XMakeLua),
    }
}
