use emmylua_parser::LuaStringToken;
use lsp_types::{CompletionItem, CompletionTextEdit, TextEdit};

use crate::handlers::completion::{
    completion_builder::CompletionBuilder, providers::get_text_edit_range_in_string,
};

pub fn add_completion(builder: &mut CompletionBuilder, string_token: LuaStringToken) -> Option<()> {
    let text_edit_range = get_text_edit_range_in_string(builder, string_token.clone());

    add_targets(builder, text_edit_range);

    Some(())
}

fn add_targets(
    builder: &mut CompletionBuilder,
    text_edit_range: Option<lsp_types::Range>,
) -> Option<()> {
    let mut module_completions = vec![];
    let file_id = builder.semantic_model.get_file_id();
    let xmake_target_or_packages = builder
        .semantic_model
        .get_db()
        .get_xmake_index()
        .get_targets(file_id)?;
    for target_or_package in xmake_target_or_packages {
        if !target_or_package.kind.is_target() {
            continue;
        }
        let text_edit = text_edit_range.map(|text_edit_range| {
            CompletionTextEdit::Edit(TextEdit {
                range: text_edit_range.clone(),
                new_text: target_or_package.name.clone(),
            })
        });
        let completion_item = CompletionItem {
            label: target_or_package.name.clone(),
            kind: Some(lsp_types::CompletionItemKind::CONSTANT),
            text_edit,
            ..Default::default()
        };
        module_completions.push(completion_item);
    }

    for completion_item in module_completions {
        builder.add_completion_item(completion_item)?;
    }

    Some(())
}
