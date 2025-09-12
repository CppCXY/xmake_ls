use emmylua_parser::{LuaAstNode, LuaComment, LuaTokenKind};
use lsp_types::{FoldingRange, FoldingRangeKind};
use rowan::NodeOrToken;

use super::builder::FoldingRangeBuilder;

pub fn build_comment_fold_range(
    builder: &mut FoldingRangeBuilder,
    comment: LuaComment,
) -> Option<()> {
    let range = comment.get_range();
    let document = builder.get_document();
    let lsp_range = document.to_lsp_range(range)?;
    if lsp_range.start.line != lsp_range.end.line {
        let folding_range = FoldingRange {
            start_line: lsp_range.start.line,
            start_character: Some(lsp_range.start.character),
            end_line: lsp_range.end.line,
            end_character: Some(lsp_range.end.character),
            kind: Some(FoldingRangeKind::Comment),
            collapsed_text: None,
        };

        builder.push(folding_range);
    }

    for child in comment.syntax().children_with_tokens() {
        if let NodeOrToken::Token(token) = child {
            if token.kind() == LuaTokenKind::TkDocRegion.into() {
                builder.begin_region(token.text_range());
            } else if token.kind() == LuaTokenKind::TkDocEndRegion.into() {
                builder.finish_region(token.text_range());
            }
        }
    }

    Some(())
}
