use emmylua_parser::{
    LuaDoStat, LuaForRangeStat, LuaForStat, LuaIfStat, LuaRepeatStat, LuaWhileStat,
};
use lsp_types::{FoldingRange, FoldingRangeKind};

use super::builder::FoldingRangeBuilder;

pub fn build_for_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    for_stat: LuaForStat,
) -> Option<()> {
    let folding_lsp_range = builder.get_block_collapsed_range(for_stat.get_block()?)?;

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

pub fn build_for_range_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    for_range_stat: LuaForRangeStat,
) -> Option<()> {
    let folding_lsp_range = builder.get_block_collapsed_range(for_range_stat.get_block()?)?;

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

pub fn build_while_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    while_stat: LuaWhileStat,
) -> Option<()> {
    let folding_lsp_range = builder.get_block_collapsed_range(while_stat.get_block()?)?;

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

pub fn build_repeat_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    repeat_stat: LuaRepeatStat,
) -> Option<()> {
    let folding_lsp_range = builder.get_block_collapsed_range(repeat_stat.get_block()?)?;

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

pub fn build_do_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    do_stat: LuaDoStat,
) -> Option<()> {
    let folding_lsp_range = builder.get_block_collapsed_range(do_stat.get_block()?)?;

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

pub fn build_if_stat_fold_range(
    builder: &mut FoldingRangeBuilder,
    if_stat: LuaIfStat,
) -> Option<()> {
    let mut collapsed_range_text = Vec::new();
    if let Some(block) = if_stat.get_block() {
        if let Some(range) = builder.get_block_collapsed_range(block) {
            collapsed_range_text.push((range, " .. ".to_string()));
        }
    }

    for else_if in if_stat.get_else_if_clause_list() {
        if let Some(block) = else_if.get_block() {
            if let Some(range) = builder.get_block_collapsed_range(block) {
                collapsed_range_text.push((range, " .. ".to_string()));
            }
        }
    }

    if let Some(else_clause) = if_stat.get_else_clause() {
        if let Some(block) = else_clause.get_block() {
            if let Some(range) = builder.get_block_collapsed_range(block) {
                collapsed_range_text.push((range, " .. ".to_string()));
            }
        }
    }

    for (range, collapsed_text) in collapsed_range_text {
        let folding_range = FoldingRange {
            start_line: range.start.line,
            start_character: Some(range.start.character),
            end_line: range.end.line,
            end_character: Some(range.end.character),
            kind: Some(FoldingRangeKind::Region),
            collapsed_text: Some(collapsed_text),
        };

        builder.push(folding_range);
    }

    Some(())
}
