use emmylua_parser::{LuaAstNode, LuaCallExpr, LuaExpr};

use crate::{
    LuaDeclId, LuaTypeCache, XmakeFunction,
    compilation::analyzer::{common::bind_type, lua::LuaAnalyzer, unresolve},
};

pub fn analyze_xmake_function_call(
    analyzer: &mut LuaAnalyzer,
    call_expr: LuaCallExpr,
    xmake_function: XmakeFunction,
) {
    match xmake_function {
        XmakeFunction::Import => {
            analyze_import(analyzer, call_expr);
        }
        _ => {}
    }
}

fn analyze_import(analyzer: &mut LuaAnalyzer, call_expr: LuaCallExpr) -> Option<()> {
    let decl_id = LuaDeclId::new(analyzer.file_id, call_expr.get_position());
    let return_type = analyzer.infer_expr(&LuaExpr::CallExpr(call_expr.clone()));
    match return_type {
        Ok(return_type) => {
            bind_type(
                analyzer.db,
                decl_id.into(),
                LuaTypeCache::InferType(return_type),
            );
        }
        Err(reason) => {
            let unresolve = unresolve::UnResolveDecl {
                file_id: analyzer.file_id,
                decl_id,
                expr: LuaExpr::CallExpr(call_expr.clone()),
                ret_idx: 0,
            };
            analyzer.context.add_unresolve(unresolve.into(), reason);
        }
    }

    Some(())
}
