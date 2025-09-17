use std::path::PathBuf;

use emmylua_parser::{
    LuaAstNode, LuaAstToken, LuaCallArgList, LuaCallExpr, LuaLiteralExpr, LuaStringToken,
    LuaSyntaxNode,
};
use lsp_types::DocumentLink;
use xmake_code_analysis::{
    DbIndex, Emmyrc, LuaDocument, XmakeFunction, file_path_to_uri, get_xmake_function,
};

pub fn build_links(
    db: &DbIndex,
    root: LuaSyntaxNode,
    document: &LuaDocument,
    emmyrc: &Emmyrc,
) -> Option<Vec<DocumentLink>> {
    let string_tokens = root
        .descendants_with_tokens()
        .filter_map(|it| it.into_token())
        .filter_map(|it| LuaStringToken::cast(it));

    let mut result = vec![];
    for token in string_tokens {
        try_build_file_link(db, token, document, &mut result, &emmyrc);
    }

    Some(result)
}

fn try_build_file_link(
    db: &DbIndex,
    token: LuaStringToken,
    document: &LuaDocument,
    result: &mut Vec<DocumentLink>,
    _: &Emmyrc,
) -> Option<()> {
    if is_require_path(token.clone()).unwrap_or(false) {
        try_build_module_link(db, token, document, result);
        return Some(());
    }

    if let Some(xmake_func) = get_xmake_call(token.clone()) {
        match xmake_func {
            XmakeFunction::Import => {
                try_build_import_module_link(db, token, document, result);
                return Some(());
            }
            XmakeFunction::Includes => {
                try_build_include_module_link(db, token, document, result);
                return Some(());
            }
            _ => {}
        }
    }

    try_build_relative_path(token, document, result)
}

fn try_build_module_link(
    db: &DbIndex,
    token: LuaStringToken,
    document: &LuaDocument,
    result: &mut Vec<DocumentLink>,
) -> Option<()> {
    let module_path = token.get_value();
    let module_index = db.get_module_index();
    let founded_module = module_index.find_module(&module_path)?;
    let file_id = founded_module.file_id;
    let vfs = db.get_vfs();
    let uri = vfs.get_uri(&file_id)?;
    let range = token.get_range();
    let lsp_range = document.to_lsp_range(range)?;
    let document_link = DocumentLink {
        target: Some(uri.clone()),
        range: lsp_range,
        tooltip: None,
        data: None,
    };

    result.push(document_link);

    Some(())
}

fn try_build_import_module_link(
    db: &DbIndex,
    token: LuaStringToken,
    document: &LuaDocument,
    result: &mut Vec<DocumentLink>,
) -> Option<()> {
    let module_path = token.get_value();
    let module_index = db.get_module_index();
    let source_file_id = document.get_file_id();
    let founded_module = module_index.find_import(db, &module_path, source_file_id)?;
    let file_id = founded_module.file_id;
    let vfs = db.get_vfs();
    let uri = vfs.get_uri(&file_id)?;
    let range = token.get_range();
    let lsp_range = document.to_lsp_range(range)?;
    let document_link = DocumentLink {
        target: Some(uri.clone()),
        range: lsp_range,
        tooltip: None,
        data: None,
    };

    result.push(document_link);
    Some(())
}

fn try_build_include_module_link(
    db: &DbIndex,
    token: LuaStringToken,
    document: &LuaDocument,
    result: &mut Vec<DocumentLink>,
) -> Option<()> {
    let module_path = token.get_value();
    let module_index = db.get_module_index();
    let source_file_id = document.get_file_id();
    let founded_module = module_index.find_include(db, &module_path, source_file_id)?;
    let file_id = founded_module.file_id;
    let vfs = db.get_vfs();
    let uri = vfs.get_uri(&file_id)?;
    let range = token.get_range();
    let lsp_range = document.to_lsp_range(range)?;
    let document_link = DocumentLink {
        target: Some(uri.clone()),
        range: lsp_range,
        tooltip: None,
        data: None,
    };

    result.push(document_link);
    Some(())
}

pub fn is_require_path(token: LuaStringToken) -> Option<bool> {
    let call_expr = token
        .get_parent::<LuaLiteralExpr>()?
        .get_parent::<LuaCallArgList>()?
        .get_parent::<LuaCallExpr>()?;

    Some(call_expr.is_require())
}

pub fn get_xmake_call(token: LuaStringToken) -> Option<XmakeFunction> {
    let call_expr = token
        .get_parent::<LuaLiteralExpr>()?
        .get_parent::<LuaCallArgList>()?
        .get_parent::<LuaCallExpr>()?;

    get_xmake_function(&call_expr)
}

fn try_build_relative_path(
    token: LuaStringToken,
    document: &LuaDocument,
    result: &mut Vec<DocumentLink>,
) -> Option<()> {
    let file_path = token.get_value();
    if file_path.find(|c| c == '\\' || c == '/').is_some() {
        let xmake_file_path = document.get_file_path();
        let xmake_file_dir = xmake_file_path.parent()?;
        let suffix_path = PathBuf::from(file_path);
        if suffix_path.exists() {
            if let Some(uri) = file_path_to_uri(&suffix_path) {
                let document_link = DocumentLink {
                    target: Some(uri),
                    range: document.to_lsp_range(token.get_range())?,
                    tooltip: None,
                    data: None,
                };

                result.push(document_link);
            }
            return Some(());
        }

        let full_path = xmake_file_dir.join(&suffix_path);
        if full_path.exists() {
            if let Some(uri) = file_path_to_uri(&full_path) {
                let document_link = DocumentLink {
                    target: Some(uri),
                    range: document.to_lsp_range(token.get_range())?,
                    tooltip: None,
                    data: None,
                };

                result.push(document_link);
            }
            return Some(());
        }
    }

    Some(())
}
