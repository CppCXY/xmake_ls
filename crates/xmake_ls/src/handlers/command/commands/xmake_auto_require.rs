use std::{collections::HashMap, time::Duration};

use emmylua_parser::{
    LuaAst, LuaAstNode, LuaBlock, LuaChunk, LuaClosureExpr, LuaStat, LuaSyntaxToken,
};
use lsp_types::{ApplyWorkspaceEditParams, Command, Position, TextEdit, WorkspaceEdit};
use rowan::{TextRange, TokenAtOffset};
use serde_json::Value;
use xmake_code_analysis::{FileId, XmakeFunction, get_xmake_function};

use crate::{context::ServerContextSnapshot, util::time_cancel_token};

use super::CommandSpec;

pub struct AutoRequireCommand;

impl CommandSpec for AutoRequireCommand {
    const COMMAND: &str = "xmake.auto.require";

    async fn handle(context: ServerContextSnapshot, args: Vec<Value>) -> Option<()> {
        let add_to: FileId = serde_json::from_value(args.get(0)?.clone()).ok()?;
        let need_require_file_id: FileId = serde_json::from_value(args.get(1)?.clone()).ok()?;
        let position: Position = serde_json::from_value(args.get(2)?.clone()).ok()?;
        // let member_name: String = serde_json::from_value(args.get(3)?.clone()).ok()?;

        let analysis = context.analysis().read().await;
        let semantic_model = analysis.compilation.get_semantic_model(add_to)?;
        let module_info = semantic_model
            .get_db()
            .get_module_index()
            .get_module(need_require_file_id)?;
        let full_module_path = module_info.full_module_name.clone();

        let require_str = format!("import(\"{}\")", full_module_path);
        let document = semantic_model.get_document();
        let position_offset =
            document.get_offset(position.line as usize, position.character as usize)?;
        let root = semantic_model.get_root();
        if position_offset > root.syntax().text_range().end() {
            return None;
        }

        let token = match root.syntax().token_at_offset(position_offset) {
            TokenAtOffset::Single(token) => token,
            TokenAtOffset::Between(left, _) => left,
            TokenAtOffset::None => {
                return None;
            }
        };

        let scope = find_first_function_scope(token.clone())?;

        let mut last_import_stat: Option<LuaStat> = None;
        for stat in scope.get_stats() {
            if stat.get_position() > position_offset {
                break;
            }

            if is_import_stat(stat.clone()).unwrap_or(false) {
                last_import_stat = Some(stat);
            }
        }

        let first_stat = scope.get_stats().next()?;
        let first_stat_start = first_stat.get_range().start();
        let line = if let Some(last_require_stat) = &last_import_stat {
            let last_import_stat_end = last_require_stat.get_range().end();
            document.get_line(last_import_stat_end)? + 1
        } else {
            document.get_line(first_stat_start)?
        };

        let line_range = if last_import_stat.is_some() {
            document.get_line_range(line - 1)?
        } else {
            document.get_line_range(line)?
        };

        let line_start_offset = line_range.start();
        let indent_text =
            document.get_text_slice(TextRange::new(line_start_offset, first_stat_start));

        let text_edit = TextEdit {
            range: lsp_types::Range {
                start: Position {
                    line: line as u32,
                    character: 0,
                },
                end: Position {
                    line: line as u32,
                    character: 0,
                },
            },
            new_text: format!("{}{}\n", indent_text, require_str),
        };

        let uri = document.get_uri();
        let mut changes = HashMap::new();
        changes.insert(uri.clone(), vec![text_edit.clone()]);

        let cancel_token = time_cancel_token(Duration::from_secs(5));
        let apply_edit_params = ApplyWorkspaceEditParams {
            label: None,
            edit: WorkspaceEdit {
                changes: Some(changes),
                document_changes: None,
                change_annotations: None,
            },
        };

        let context_clone = context.clone();
        tokio::spawn(async move {
            let res = context_clone
                .client()
                .apply_edit(apply_edit_params, cancel_token)
                .await;
            if let Some(res) = res {
                if !res.applied {
                    log::error!("Failed to apply edit: {:?}", res.failure_reason);
                }
            }
        });

        Some(())
    }
}

fn find_first_function_scope(token: LuaSyntaxToken) -> Option<LuaBlock> {
    let ast_node = LuaAst::cast(token.parent()?)?;
    for ancestor in ast_node.ancestors::<LuaBlock>() {
        if ancestor.get_parent::<LuaClosureExpr>().is_some() {
            return Some(ancestor);
        } else if ancestor.get_parent::<LuaChunk>().is_some() {
            return Some(ancestor);
        }
    }

    None
}

fn is_import_stat(stat: LuaStat) -> Option<bool> {
    match stat {
        LuaStat::CallExprStat(call_expr_stat) => {
            let call_expr = call_expr_stat.get_call_expr()?;
            let func = get_xmake_function(&call_expr)?;
            return Some(func == XmakeFunction::Import);
        }
        _ => {}
    }

    Some(false)
}

pub fn make_auto_import(
    title: &str,
    add_to: FileId,
    need_require_file_id: FileId,
    position: Position,
    member_name: Option<String>,
) -> Command {
    let args = vec![
        serde_json::to_value(add_to).unwrap(),
        serde_json::to_value(need_require_file_id).unwrap(),
        serde_json::to_value(position).unwrap(),
        serde_json::to_value(member_name.unwrap_or_default()).unwrap(),
    ];

    Command {
        title: title.to_string(),
        command: AutoRequireCommand::COMMAND.to_string(),
        arguments: Some(args),
    }
}
