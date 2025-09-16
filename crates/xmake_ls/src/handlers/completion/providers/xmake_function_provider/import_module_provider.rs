use std::path::PathBuf;

use emmylua_parser::LuaStringToken;
use lsp_types::{CompletionItem, CompletionTextEdit, TextEdit};

use crate::handlers::completion::{
    completion_builder::CompletionBuilder,
    completion_data::CompletionData,
    providers::{
        get_text_edit_range_in_string,
        xmake_function_provider::file_path_provider::add_file_path_completion,
    },
};

pub fn add_completion(builder: &mut CompletionBuilder, string_token: LuaStringToken) -> Option<()> {
    let text_edit_range = get_text_edit_range_in_string(builder, string_token.clone());
    let prefix_content = string_token.get_value();
    let document = builder.semantic_model.get_document();
    let file_path = document.get_file_path();
    let xmake_file_dir = file_path.parent()?;

    add_modules(
        builder,
        &prefix_content,
        text_edit_range,
        &xmake_file_dir.to_path_buf(),
    );

    Some(())
}

fn add_modules(
    builder: &mut CompletionBuilder,
    module_path: &str,
    text_edit_range: Option<lsp_types::Range>,
    xmake_file_dir: &PathBuf,
) -> Option<()> {
    let prefix = if let Some(last_sep) = module_path.rfind(|c| c == '.') {
        let (path, _) = module_path.split_at(last_sep + 1);
        path
    } else {
        ""
    };

    let parts: Vec<&str> = if prefix.is_empty() {
        vec![]
    } else {
        prefix.split('.').filter(|s| !s.is_empty()).collect()
    };

    let dir_path = xmake_file_dir.join(parts.join("/"));
    if dir_path.exists() && dir_path.is_dir() {
        if let Ok(entries) = std::fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|e| e.to_str()) != Some("lua") {
                    continue;
                }
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    let trim_name = if let Some(name) = name.strip_suffix(".lua") {
                        name
                    } else {
                        name
                    };
                    add_file_path_completion(builder, &path, trim_name, prefix, text_edit_range);
                }
            }
        }
    }
    let db = builder.semantic_model.get_db();
    let mut module_completions = Vec::new();
    let prefix_module_path = parts.join(".");
    let module_info = db.get_module_index().find_import_node(&prefix_module_path)?;
    for (name, module_id) in &module_info.children {
        let child_module_node = db.get_module_index().get_module_node(module_id)?;
        let filter_text = format!("{}{}", prefix, name);
        let text_edit = text_edit_range.map(|text_edit_range| {
            CompletionTextEdit::Edit(TextEdit {
                range: text_edit_range.clone(),
                new_text: filter_text.clone(),
            })
        });
        if let Some(child_file_id) = child_module_node.file_ids.first() {
            let child_module_info = db.get_module_index().get_module(*child_file_id)?;
            let data = if let Some(property_id) = &child_module_info.semantic_id {
                CompletionData::from_property_owner_id(builder, property_id.clone(), None)
            } else {
                None
            };

            let uri = db.get_vfs().get_uri(child_file_id)?;
            let completion_item = CompletionItem {
                label: name.clone(),
                kind: Some(lsp_types::CompletionItemKind::FILE),
                filter_text: Some(filter_text.clone()),
                text_edit,
                detail: Some(uri.to_string()),
                data,
                ..Default::default()
            };
            module_completions.push(completion_item);
        } else {
            let completion_item = CompletionItem {
                label: name.clone(),
                kind: Some(lsp_types::CompletionItemKind::FOLDER),
                filter_text: Some(filter_text.clone()),
                text_edit,
                ..Default::default()
            };

            module_completions.push(completion_item);
        }
    }

    for completion_item in module_completions {
        builder.add_completion_item(completion_item)?;
    }

    Some(())
}
