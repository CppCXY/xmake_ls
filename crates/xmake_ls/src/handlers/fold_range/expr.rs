use emmylua_parser::{
    LuaAstNode, LuaAstToken, LuaClosureExpr, LuaLiteralExpr, LuaLiteralToken, LuaTableExpr,
};
use lsp_types::{FoldingRange, FoldingRangeKind};

use super::builder::FoldingRangeBuilder;

pub fn build_table_expr_fold_range(
    builder: &mut FoldingRangeBuilder,
    table_expr: LuaTableExpr,
) -> Option<()> {
    let document = builder.get_document();
    let expr_range = table_expr.get_range();
    let lsp_range = document.to_lsp_range(expr_range)?;
    let folding_lsp_range = builder.get_folding_lsp_range(
        lsp_range.start.line as usize,
        lsp_range.end.line as usize,
        lsp_range.start.character as usize,
        lsp_range.end.character as usize,
    )?;

    let folding_range = FoldingRange {
        start_line: folding_lsp_range.start.line,
        start_character: Some(folding_lsp_range.start.character),
        end_line: folding_lsp_range.end.line,
        end_character: Some(folding_lsp_range.end.character),
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: Some(" .. ".to_string()),
    };

    builder.push(folding_range);
    Some(())
}

pub fn build_string_fold_range(
    builder: &mut FoldingRangeBuilder,
    literal: LuaLiteralExpr,
) -> Option<()> {
    let token = literal.get_literal()?;
    let string_token = match token {
        LuaLiteralToken::String(s) => s,
        _ => return None,
    };

    let range = string_token.get_range();
    let document = builder.get_document();
    let lsp_range = document.to_lsp_range(range)?;
    if lsp_range.start.line == lsp_range.end.line {
        return None;
    }

    let folding_range = FoldingRange {
        start_line: lsp_range.start.line,
        start_character: Some(lsp_range.start.character),
        end_line: lsp_range.end.line,
        end_character: Some(lsp_range.end.character),
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: Some("'..'".to_string()),
    };

    builder.push(folding_range);
    Some(())
}

pub fn build_closure_expr_fold_range(
    builder: &mut FoldingRangeBuilder,
    closure: LuaClosureExpr,
) -> Option<()> {
    let block = closure.get_block()?;
    let lsp_range = builder.get_block_collapsed_range(block)?;

    let folding_range = FoldingRange {
        start_line: lsp_range.start.line,
        start_character: Some(lsp_range.start.character),
        end_line: lsp_range.end.line,
        end_character: Some(lsp_range.end.character),
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: Some(" .. ".to_string()),
    };

    builder.push(folding_range);
    Some(())
}
