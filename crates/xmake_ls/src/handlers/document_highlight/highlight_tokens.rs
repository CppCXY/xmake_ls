use emmylua_parser::{LuaAstNode, LuaSyntaxKind, LuaSyntaxNode, LuaSyntaxToken, LuaTokenKind};
use lsp_types::{DocumentHighlight, DocumentHighlightKind};
use rowan::NodeOrToken;
use xmake_code_analysis::{
    LuaDeclId, LuaDocument, LuaSemanticDeclId, SemanticDeclLevel, SemanticModel,
};

pub fn highlight_tokens(
    semantic_model: &SemanticModel,
    token: LuaSyntaxToken,
) -> Option<Vec<DocumentHighlight>> {
    let mut result = Vec::new();
    match token.kind().into() {
        LuaTokenKind::TkName => {
            let semantic_decl =
                semantic_model.find_decl(token.clone().into(), SemanticDeclLevel::NoTrace);
            match semantic_decl {
                Some(LuaSemanticDeclId::LuaDecl(decl_id)) => {
                    highlight_decl_references(&semantic_model, decl_id, token, &mut result);
                }
                _ => {
                    highlight_name(semantic_model, token, &mut result);
                }
            }
        }
        token_kind if is_keyword(token_kind) => {
            highlight_keywords(semantic_model, token, &mut result);
        }

        _ => {}
    }

    Some(result)
}

fn highlight_decl_references(
    semantic_model: &SemanticModel,
    decl_id: LuaDeclId,
    token: LuaSyntaxToken,
    result: &mut Vec<DocumentHighlight>,
) -> Option<()> {
    let decl = semantic_model
        .get_db()
        .get_decl_index()
        .get_decl(&decl_id)?;
    let document = semantic_model.get_document();
    if decl.is_local() {
        let decl_refs = semantic_model
            .get_db()
            .get_reference_index()
            .get_decl_references(&decl_id.file_id, &decl_id)?;

        for decl_ref in &decl_refs.cells {
            let range: lsp_types::Range = document.to_lsp_range(decl_ref.range.clone())?;
            let kind = if decl_ref.is_write {
                Some(DocumentHighlightKind::WRITE)
            } else {
                Some(DocumentHighlightKind::READ)
            };
            result.push(DocumentHighlight { range, kind });
        }

        let range = document.to_lsp_range(decl.get_range())?;
        result.push(DocumentHighlight { range, kind: None });

        return Some(());
    } else {
        highlight_name(semantic_model, token, result);
    }

    Some(())
}

fn highlight_name(
    semantic_model: &SemanticModel,
    token: LuaSyntaxToken,
    result: &mut Vec<DocumentHighlight>,
) -> Option<()> {
    let root = semantic_model.get_root();
    let token_name = token.text();
    let document = semantic_model.get_document();
    for node_or_token in root.syntax().descendants_with_tokens() {
        if let NodeOrToken::Token(token) = node_or_token {
            if token.kind() == LuaTokenKind::TkName.into() && token.text() == token_name {
                let range = document.to_lsp_range(token.text_range())?;
                result.push(DocumentHighlight {
                    range,
                    kind: Some(DocumentHighlightKind::TEXT),
                });
            }
        }
    }

    Some(())
}

fn is_keyword(kind: LuaTokenKind) -> bool {
    match kind {
        LuaTokenKind::TkAnd
        | LuaTokenKind::TkBreak
        | LuaTokenKind::TkDo
        | LuaTokenKind::TkElse
        | LuaTokenKind::TkElseIf
        | LuaTokenKind::TkEnd
        | LuaTokenKind::TkFor
        | LuaTokenKind::TkFunction
        | LuaTokenKind::TkGoto
        | LuaTokenKind::TkIf
        | LuaTokenKind::TkIn
        | LuaTokenKind::TkLocal
        | LuaTokenKind::TkRepeat
        | LuaTokenKind::TkReturn
        | LuaTokenKind::TkThen
        | LuaTokenKind::TkUntil
        | LuaTokenKind::TkWhile => true,
        _ => false,
    }
}

fn highlight_keywords(
    semantic_model: &SemanticModel,
    token: LuaSyntaxToken,
    result: &mut Vec<DocumentHighlight>,
) -> Option<()> {
    let document = semantic_model.get_document();
    let parent_node = token.parent()?;
    match parent_node.kind().into() {
        LuaSyntaxKind::LocalFuncStat | LuaSyntaxKind::FuncStat => {
            highlight_node_keywords(&document, parent_node.clone(), result);
            let closure_node = parent_node
                .children()
                .find(|node| node.kind() == LuaSyntaxKind::ClosureExpr.into())?;
            highlight_node_keywords(&document, closure_node, result);
        }
        _ => {
            highlight_node_keywords(&document, parent_node, result);
        }
    }

    Some(())
}

fn highlight_node_keywords(
    document: &LuaDocument,
    node: LuaSyntaxNode,
    result: &mut Vec<DocumentHighlight>,
) -> Option<()> {
    for node_or_token in node.children_with_tokens() {
        if let NodeOrToken::Token(token) = node_or_token {
            if is_keyword(token.kind().into()) {
                let range = document.to_lsp_range(token.text_range())?;
                result.push(DocumentHighlight {
                    range,
                    kind: Some(DocumentHighlightKind::TEXT),
                });
            }
        }
    }

    Some(())
}
