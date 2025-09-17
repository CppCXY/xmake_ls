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
    };

    if prefix.is_empty() {
        // add @builtin
        let filter_text = "@builtin".to_string();
        let text_edit = text_edit_range.map(|text_edit_range| {
            CompletionTextEdit::Edit(TextEdit {
                range: text_edit_range.clone(),
                new_text: filter_text.clone(),
            })
        });
        let completion_item = CompletionItem {
            label: filter_text.to_string(),
            kind: Some(lsp_types::CompletionItemKind::FOLDER),
            filter_text: Some(filter_text.clone()),
            text_edit,
            ..Default::default()
        };

        builder.add_completion_item(completion_item)?;
    } else if prefix.starts_with("@builtin/") {
        // add @builtin sub folder
        add_builtin_include_completion(builder, prefix, text_edit_range);
    }

    Some(())
}

fn add_builtin_include_completion(
    builder: &mut CompletionBuilder,
    prefix: &str,
    text_edit_range: Option<lsp_types::Range>,
) -> Option<()> {
    let builtin_path = if let Some(builtin_path) = prefix.strip_prefix("@builtin/") {
        builtin_path.strip_suffix("/").unwrap_or("")
    } else {
        ""
    };

    let db = builder.semantic_model.get_db();
    let mut module_completions = vec![];
    let module_info = db.get_module_index().find_include_node(&builtin_path)?;
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
