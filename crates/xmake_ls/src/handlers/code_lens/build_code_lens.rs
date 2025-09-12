use xmake_code_analysis::{LuaDeclId, LuaMemberId, SemanticModel};
use emmylua_parser::{LuaAst, LuaAstNode, LuaAstToken, LuaFuncStat, LuaLocalFuncStat, LuaVarExpr};
use lsp_types::CodeLens;

use super::CodeLensData;

pub fn build_code_lens(semantic_model: &SemanticModel) -> Option<Vec<CodeLens>> {
    let mut result = Vec::new();
    let root = semantic_model.get_root().clone();
    for node in root.descendants::<LuaAst>() {
        match node {
            LuaAst::LuaFuncStat(func_stat) => {
                add_func_stat_code_lens(semantic_model, &mut result, func_stat)?;
            }
            LuaAst::LuaLocalFuncStat(local_func_stat) => {
                add_local_func_stat_code_lens(semantic_model, &mut result, local_func_stat)?;
            }
            _ => {}
        }
    }

    Some(result)
}

fn add_func_stat_code_lens(
    semantic_model: &SemanticModel,
    result: &mut Vec<CodeLens>,
    func_stat: LuaFuncStat,
) -> Option<()> {
    let file_id = semantic_model.get_file_id();
    let func_name = func_stat.get_func_name()?;
    let document = semantic_model.get_document();
    match func_name {
        LuaVarExpr::IndexExpr(index_expr) => {
            let member_id = LuaMemberId::new(index_expr.get_syntax_id(), file_id);
            let data = CodeLensData::Member(member_id);
            let index_name_token = index_expr.get_index_name_token()?;
            let range = document.to_lsp_range(index_name_token.text_range())?;
            result.push(CodeLens {
                range,
                command: None,
                data: Some(serde_json::to_value(data).unwrap()),
            });
        }
        LuaVarExpr::NameExpr(name_expr) => {
            let name_token = name_expr.get_name_token()?;
            let decl_id = LuaDeclId::new(file_id, name_token.get_position());
            let data = CodeLensData::DeclId(decl_id);
            let range = document.to_lsp_range(name_token.get_range())?;
            result.push(CodeLens {
                range,
                command: None,
                data: Some(serde_json::to_value(data).unwrap()),
            });
        }
    }

    Some(())
}

fn add_local_func_stat_code_lens(
    semantic_model: &SemanticModel,
    result: &mut Vec<CodeLens>,
    local_func_stat: LuaLocalFuncStat,
) -> Option<()> {
    let file_id = semantic_model.get_file_id();
    let func_name = local_func_stat.get_local_name()?;
    let document = semantic_model.get_document();
    let range = document.to_lsp_range(func_name.get_range())?;
    let name_token = func_name.get_name_token()?;
    let decl_id = LuaDeclId::new(file_id, name_token.get_position());
    let data = CodeLensData::DeclId(decl_id);
    result.push(CodeLens {
        range,
        command: None,
        data: Some(serde_json::to_value(data).unwrap()),
    });
    Some(())
}
