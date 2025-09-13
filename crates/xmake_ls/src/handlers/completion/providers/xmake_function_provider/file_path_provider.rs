use std::path::PathBuf;

use emmylua_parser::LuaStringToken;
use lsp_types::{CompletionItem, TextEdit};
use xmake_code_analysis::file_path_to_uri;

use crate::handlers::completion::{
    completion_builder::CompletionBuilder, providers::get_text_edit_range_in_string,
};

pub fn add_completion(builder: &mut CompletionBuilder, string_token: LuaStringToken) -> Option<()> {
    let text_edit_range = get_text_edit_range_in_string(builder, string_token.clone())?;
    let text = string_token.get_value();
    let document = builder.semantic_model.get_document();
    let file_path = document.get_file_path();
    let xmake_file_dir = file_path.parent()?;

    let prefix = if let Some(last_sep) = text.rfind(|c| c == '/' || c == '\\') {
        let (path, _) = text.split_at(last_sep + 1);
        path
    } else {
        ""
    };

    let suffix = prefix;

    let folder = if suffix.is_empty() {
        xmake_file_dir.to_path_buf()
    } else {
        xmake_file_dir.join(suffix)
    };

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
