use emmylua_parser::{LuaAstNode, LuaBlock, LuaChunk, LuaExpr, LuaStat};

use crate::{
    InferFailReason, LuaDeclId, LuaSemanticDeclId, LuaSignatureId,
    compilation::analyzer::unresolve::UnResolveModule, db_index::LuaType,
};

use super::{LuaAnalyzer, LuaReturnPoint, func_body::analyze_func_body_returns};

pub fn analyze_chunk_return(analyzer: &mut LuaAnalyzer, chunk: LuaChunk) -> Option<()> {
    let block = chunk.get_block()?;
    if !check_exist_return(block.clone()) {
        analyze_xmake_export(analyzer);
        return Some(());
    }

    let return_exprs = analyze_func_body_returns(block.clone());

    for point in return_exprs {
        match point {
            LuaReturnPoint::Expr(expr) => {
                let expr_type = match analyzer.infer_expr(&expr) {
                    Ok(expr_type) => expr_type,
                    Err(InferFailReason::None) => LuaType::Unknown,
                    Err(reason) => {
                        let unresolve = UnResolveModule {
                            file_id: analyzer.file_id,
                            expr,
                        };
                        analyzer.context.add_unresolve(unresolve.into(), reason);
                        return None;
                    }
                };

                let semantic_id = get_semantic_id(analyzer, expr.clone());

                let module_info = analyzer
                    .db
                    .get_module_index_mut()
                    .get_module_mut(analyzer.file_id)?;
                match expr_type {
                    LuaType::Variadic(multi) => {
                        let ty = multi.get_type(0)?;
                        module_info.export_type = Some(ty.clone());
                    }
                    _ => {
                        module_info.export_type = Some(expr_type);
                    }
                }
                module_info.semantic_id = semantic_id;
                break;
            }
            // Other cases are stupid code
            _ => {}
        }
    }

    Some(())
}

fn check_exist_return(block: LuaBlock) -> bool {
    block.get_stats().any(|stat| {
        if let LuaStat::ReturnStat(_) = stat {
            return true;
        }
        false
    })
}

fn get_semantic_id(analyzer: &LuaAnalyzer, expr: LuaExpr) -> Option<LuaSemanticDeclId> {
    match expr {
        LuaExpr::NameExpr(name_expr) => {
            let name = name_expr.get_name_text()?;
            let tree = analyzer
                .db
                .get_decl_index()
                .get_decl_tree(&analyzer.file_id)?;
            let decl = tree.find_local_decl(&name, name_expr.get_position())?;

            Some(LuaSemanticDeclId::LuaDecl(decl.get_id()))
        }
        LuaExpr::ClosureExpr(closure) => Some(LuaSemanticDeclId::Signature(
            LuaSignatureId::from_closure(analyzer.file_id, &closure),
        )),
        // `return {}`
        LuaExpr::TableExpr(table_expr) => Some(LuaSemanticDeclId::LuaDecl(LuaDeclId::new(
            analyzer.file_id,
            table_expr.get_position(),
        ))),
        _ => None,
    }
}

fn analyze_xmake_export(analyzer: &mut LuaAnalyzer) -> Option<()> {
    let decl_tree = analyzer
        .db
        .get_decl_index()
        .get_decl_tree(&analyzer.file_id)?;
    let export_decls = decl_tree.get_export_env_decls();
    let mut main_id = None;
    for decl_id in export_decls {
        let decl = decl_tree.get_decl(&decl_id)?;
        if decl.get_name() == "main" {
            main_id = Some(decl_id);
            break;
        }
    }

    if let Some(main_id) = main_id {
        let typ = analyzer
            .db
            .get_type_index()
            .get_type_cache(&main_id.into())?
            .clone();
        let module_info = analyzer
            .db
            .get_module_index_mut()
            .get_module_mut(analyzer.file_id)?;

        module_info.export_type = Some(typ.as_type().clone());
        return Some(());
    }

    let module_info = analyzer
        .db
        .get_module_index_mut()
        .get_module_mut(analyzer.file_id)?;

    module_info.export_type = Some(LuaType::ModuleRef(analyzer.file_id));
    Some(())
}
