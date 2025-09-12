use std::{
    path::{Path, PathBuf},
    vec,
};

use emmylua_parser::{
    LuaAstNode, LuaAstToken, LuaCallArgList, LuaCallExpr, LuaExpr, LuaStringToken,
};
use lsp_types::{CompletionItem, TextEdit};
use xmake_code_analysis::file_path_to_uri;

use crate::handlers::completion::completion_builder::CompletionBuilder;

use super::get_text_edit_range_in_string;

pub fn add_completion(builder: &mut CompletionBuilder) -> Option<()> {
    if builder.is_cancelled() {
        return None;
    }

    let string_token = LuaStringToken::cast(builder.trigger_token.clone())?;
    let maybe_file_path = string_token.get_value();

    if maybe_file_path.find(|c| c == '/' || c == '\\').is_none()
        && check_is_special_function(&string_token).is_none()
    {
        return None;
    }

    let prefix = if let Some(last_sep) = maybe_file_path.rfind(|c| c == '/' || c == '\\') {
        let (path, _) = maybe_file_path.split_at(last_sep + 1);
        path
    } else {
        ""
    };

    let document = builder.semantic_model.get_document();
    let file_path = document.get_file_path();
    let file_dir = file_path.parent()?;
    let mut resources = vec![file_dir.to_string_lossy().to_string()];
    resources.extend(builder.semantic_model.get_emmyrc().resource.paths.clone());

    let suffix = prefix;
    let text_edit_range = get_text_edit_range_in_string(builder, string_token)?;

    for resource in resources {
        let path = Path::new(&resource);
        let folder = path.join(suffix);
        if folder.exists() && folder.is_dir() {
            if let Ok(entries) = std::fs::read_dir(folder) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        add_file_path_completion(builder, &path, name, prefix, text_edit_range);
                    }
                }
            }
        }
    }

    builder.stop_here();

    Some(())
}

fn check_is_special_function(string_token: &LuaStringToken) -> Option<()> {
    let call_expr = string_token
        .get_parent::<LuaCallArgList>()?
        .get_parent::<LuaCallExpr>()?;
    let prefix_expr = call_expr.get_prefix_expr()?;
    let LuaExpr::NameExpr(name_expr) = prefix_expr else {
        return None;
    };

    let name = name_expr.get_name_text()?;
    match name.as_str() {
        "add_files" | "includes" => {}
        _ => return None,
    }

    Some(())
}

fn add_file_path_completion(
    builder: &mut CompletionBuilder,
    path: &PathBuf,
    name: &str,
    prefix: &str,
    text_edit_range: lsp_types::Range,
) -> Option<()> {
    let kind: lsp_types::CompletionItemKind = if path.is_dir() {
        lsp_types::CompletionItemKind::FOLDER
    } else {
        lsp_types::CompletionItemKind::FILE
    };

    let detail = match file_path_to_uri(path) {
        Some(uri) => Some(uri.to_string()),
        None => None,
    };

    let filter_text = format!("{}{}", prefix, name);
    let text_edit = TextEdit {
        range: text_edit_range.clone(),
        new_text: filter_text.clone(),
    };
    let completion_item = CompletionItem {
        label: name.to_string(),
        kind: Some(kind),
        filter_text: Some(filter_text.clone()),
        text_edit: Some(lsp_types::CompletionTextEdit::Edit(text_edit)),
        detail,
        ..Default::default()
    };

    builder.add_completion_item(completion_item)?;

    Some(())
}
