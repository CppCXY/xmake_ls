use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};

use xmake_code_analysis::{
    FileId, LuaCompilation, LuaModuleIndex, LuaType, SemanticModel, WorkspaceId, file_path_to_uri,
    read_file_with_encoding, uri_to_file_path,
};
use emmylua_parser::{LuaAstNode, LuaCallExpr};
use lsp_types::{
    ApplyWorkspaceEditParams, FileRename, MessageActionItem, MessageType, RenameFilesParams,
    ShowMessageRequestParams, TextEdit, Uri, WorkspaceEdit,
};
use tokio_util::sync::CancellationToken;
use walkdir::WalkDir;

use crate::{context::ServerContextSnapshot, handlers::ClientConfig};

pub async fn on_did_rename_files_handler(
    context: ServerContextSnapshot,
    params: RenameFilesParams,
) -> Option<()> {
    let mut all_renames: Vec<RenameInfo> = vec![];

    let analysis = context.analysis().read().await;

    let module_index = analysis.compilation.get_db().get_module_index();
    for file_rename in params.files {
        let FileRename { old_uri, new_uri } = file_rename;

        let old_uri = Uri::from_str(&old_uri).ok()?;
        let new_uri = Uri::from_str(&new_uri).ok()?;

        let old_path = uri_to_file_path(&old_uri)?;
        let new_path = uri_to_file_path(&new_uri)?;

        // 提取重命名信息
        let rename_info = collect_rename_info(&old_uri, &new_uri, &module_index);
        if let Some(rename_info) = rename_info {
            all_renames.push(rename_info.clone());
        } else {
            // 有可能是目录重命名, 需要收集目录下所有 lua 文件
            if let Some(collected_renames) =
                collect_directory_lua_files(&old_path, &new_path, &module_index)
            {
                all_renames.extend(collected_renames);
            }
        }
    }

    // 如果有重命名的文件, 弹窗询问用户是否要修改require路径
    if !all_renames.is_empty() {
        drop(analysis);
        // 更新
        let mut analysis = context.analysis().write().await;
        let encoding = &analysis.get_emmyrc().workspace.encoding;
        for rename in all_renames.iter() {
            analysis.remove_file_by_uri(&rename.old_uri);
            if let Some(new_path) = uri_to_file_path(&rename.new_uri) {
                if let Some(text) = read_file_with_encoding(&new_path, encoding) {
                    analysis.update_file_by_uri(&rename.new_uri, Some(text));
                }
            }
        }
        drop(analysis);

        let analysis = context.analysis().read().await;
        if let Some(changes) = try_modify_require_path(&analysis.compilation, &all_renames) {
            drop(analysis);
            if changes.is_empty() {
                return Some(());
            }

            let client = context.client();

            let show_message_params = ShowMessageRequestParams {
                typ: MessageType::INFO,
                message: t!("Do you want to modify the require path?").to_string(),
                actions: Some(vec![MessageActionItem {
                    title: t!("Modify").to_string(),
                    properties: HashMap::new(),
                }]),
            };

            // 发送弹窗请求
            let cancel_token = CancellationToken::new();
            if let Some(selected_action) = client
                .show_message_request(show_message_params, cancel_token)
                .await
            {
                let cancel_token = CancellationToken::new();
                if selected_action.title == t!("Modify") {
                    client
                        .apply_edit(
                            ApplyWorkspaceEditParams {
                                edit: WorkspaceEdit {
                                    changes: Some(changes),
                                    document_changes: None,
                                    change_annotations: None,
                                },
                                label: None,
                            },
                            cancel_token,
                        )
                        .await?;
                }
            }
        }
    }

    Some(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct RenameInfo {
    old_uri: Uri,
    new_uri: Uri,
    old_module_path: String,
    new_module_path: String,
    workspace_id: WorkspaceId,
}

fn collect_rename_info(
    old_uri: &Uri,
    new_uri: &Uri,
    module_index: &LuaModuleIndex,
) -> Option<RenameInfo> {
    let (mut old_module_path, workspace_id) =
        module_index.extract_module_path(uri_to_file_path(&old_uri)?.to_str()?)?;
    old_module_path = old_module_path.replace(['\\', '/'], ".");

    let (mut new_module_path, _) =
        module_index.extract_module_path(uri_to_file_path(&new_uri)?.to_str()?)?;
    new_module_path = new_module_path.replace(['\\', '/'], ".");

    Some(RenameInfo {
        old_uri: old_uri.clone(),
        new_uri: new_uri.clone(),
        old_module_path,
        new_module_path,
        workspace_id,
    })
}

/// 收集目录重命名后所有的Lua文件
fn collect_directory_lua_files(
    old_path: &PathBuf,
    new_path: &PathBuf,
    module_index: &LuaModuleIndex,
) -> Option<Vec<RenameInfo>> {
    // 检查新路径是否是目录（旧路径已经不存在了）
    if !new_path.is_dir() {
        return None;
    }

    let mut renames = vec![];

    // 遍历新目录下的所有Lua文件
    for entry in WalkDir::new(new_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let new_file_path = entry.path();

        // 计算在新目录中的相对路径
        if let Ok(relative_path) = new_file_path.strip_prefix(new_path) {
            // 根据目录重命名推算出对应的旧文件路径
            let old_file_path = old_path.join(relative_path);

            // 转换为URI
            if let (Some(old_file_uri), Some(new_file_uri)) = (
                file_path_to_uri(&old_file_path),
                file_path_to_uri(&new_file_path.to_path_buf()),
            ) {
                let rename_info = collect_rename_info(&old_file_uri, &new_file_uri, module_index);
                if let Some(rename_info) = rename_info {
                    renames.push(rename_info);
                }
            }
        }
    }

    if renames.is_empty() {
        None
    } else {
        Some(renames)
    }
}

#[allow(unused)]
/// 检查文件路径是否是Lua文件
fn is_lua_file(file_path: &Path, client_config: &ClientConfig) -> bool {
    let file_name = file_path.to_string_lossy();

    if file_name.ends_with(".lua") {
        return true;
    }

    // 检查客户端配置的扩展名
    for extension in &client_config.extensions {
        if file_name.ends_with(extension) {
            return true;
        }
    }

    false
}

fn try_modify_require_path(
    compilation: &LuaCompilation,
    renames: &Vec<RenameInfo>,
) -> Option<HashMap<Uri, Vec<TextEdit>>> {
    let mut changes: HashMap<Uri, Vec<TextEdit>> = HashMap::new();
    for file_id in compilation.get_db().get_vfs().get_all_file_ids() {
        if compilation.get_db().get_module_index().is_std(&file_id) {
            continue;
        }

        if let Some(semantic_model) = compilation.get_semantic_model(file_id) {
            for call_expr in semantic_model.get_root().descendants::<LuaCallExpr>() {
                if call_expr.is_require() {
                    try_convert(&semantic_model, call_expr, renames, &mut changes, file_id);
                }
            }
        }
    }
    Some(changes)
}

fn try_convert(
    semantic_model: &SemanticModel,
    call_expr: LuaCallExpr,
    renames: &Vec<RenameInfo>,
    changes: &mut HashMap<Uri, Vec<TextEdit>>,
    current_file_id: FileId, // 当前文件id
) -> Option<()> {
    // if let Some(_) = call_expr.get_parent::<LuaIndexExpr>() {
    //     return None;
    // }

    let args_list = call_expr.get_args_list()?;
    let arg_expr = args_list.get_args().next()?;
    let ty = semantic_model
        .infer_expr(arg_expr.clone())
        .unwrap_or(LuaType::Any);
    let name = if let LuaType::StringConst(s) = ty {
        s
    } else {
        return None;
    };
    let emmyrc = semantic_model.get_emmyrc();
    let separator = &emmyrc.completion.auto_require_separator;
    let strict_require_path = emmyrc.strict.require_path;
    // 转换为标准导入语法
    let normalized_path = name.replace(separator, ".");

    for rename in renames {
        let is_matched = if strict_require_path {
            rename.old_module_path == normalized_path
        } else {
            rename.old_module_path.ends_with(&normalized_path)
        };

        if is_matched {
            let range = arg_expr.syntax().text_range();
            let lsp_range = semantic_model.get_document().to_lsp_range(range)?;

            let current_uri = semantic_model
                .get_db()
                .get_vfs()
                .get_uri(&current_file_id)?;

            let full_module_path = match separator.as_str() {
                "." | "" => rename.new_module_path.clone(),
                _ => rename.new_module_path.replace(".", &separator),
            };

            changes.entry(current_uri).or_insert(vec![]).push(TextEdit {
                range: lsp_range,
                new_text: format!("'{}'", full_module_path),
            });

            return Some(());
        }
    }

    Some(())
}
