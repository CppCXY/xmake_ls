use emmylua_parser::{LuaAssignStat, LuaAstNode, LuaExpr, LuaLocalStat, LuaStat};

use crate::{DiagnosticCode, LuaType, SemanticModel};

use super::{Checker, DiagnosticContext};

pub struct UnbalancedAssignmentsChecker;

impl Checker for UnbalancedAssignmentsChecker {
    const CODES: &[DiagnosticCode] = &[DiagnosticCode::UnbalancedAssignments];

    fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) {
        let root = semantic_model.get_root().clone();
        for stat in root.descendants::<LuaStat>() {
            match stat {
                LuaStat::LocalStat(local) => {
                    check_local_stat(context, semantic_model, &local);
                }
                LuaStat::AssignStat(assign) => {
                    check_assign_stat(context, semantic_model, &assign);
                }
                _ => {}
            }
        }
    }
}

fn check_assign_stat(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    assign: &LuaAssignStat,
) -> Option<()> {
    let (vars, value_exprs) = assign.get_var_and_expr_list();
    check_unbalanced_assignment(context, semantic_model, &vars, &value_exprs)
}

fn check_local_stat(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    local: &LuaLocalStat,
) -> Option<()> {
    let vars = local.get_local_name_list().collect::<Vec<_>>();
    let value_exprs = local.get_value_exprs().collect::<Vec<_>>();
    check_unbalanced_assignment(context, semantic_model, &vars, &value_exprs)
}

fn check_unbalanced_assignment(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    vars: &[impl LuaAstNode],
    value_exprs: &[LuaExpr],
) -> Option<()> {
    let last_value_expr = match value_exprs.last() {
        Some(expr) => expr,
        None => return Some(()),
    };

    if check_last_expr(semantic_model, &last_value_expr).unwrap_or(false) {
        return Some(());
    }

    let value_types = semantic_model.infer_expr_list_types(value_exprs, Some(vars.len()));
    if let Some(last_type) = value_types.last() {
        if check_last(&last_type.0) {
            return Some(());
        }
    }

    let value_len = value_types.len();

    if vars.len() > value_len {
        for var in vars[value_len..].iter() {
            context.add_diagnostic(
                DiagnosticCode::UnbalancedAssignments,
                var.get_range(),
                t!("The value is assigned as `nil` because the number of values is not enough.")
                    .to_string(),
                None,
            );
        }
    }

    Some(())
}

fn check_last(last_type: &LuaType) -> bool {
    match last_type {
        LuaType::Instance(instance) => check_last(&instance.get_base()),
        _ => false,
    }
}

#[allow(unused)]
fn check_last_expr(semantic_model: &SemanticModel, last_expr: &LuaExpr) -> Option<bool> {
    match last_expr {
        // TODO: 为 signature 建立独立规则
        LuaExpr::CallExpr(call_expr) => {
            return Some(true);
            // 目前仅允许 pcall 和 xpcall 禁用检查, 或许我们应该禁用所有函数调用的检查?
            // let decl_id = semantic_model.find_decl(
            //     call_expr.get_prefix_expr()?.syntax().clone().into(),
            //     SemanticDeclLevel::Trace(50),
            // )?;
            // if let LuaSemanticDeclId::LuaDecl(decl_id) = decl_id {
            //     let decl = semantic_model
            //         .get_db()
            //         .get_decl_index()
            //         .get_decl(&decl_id)?;

            //     if semantic_model
            //         .get_db()
            //         .get_module_index()
            //         .is_std(&decl.get_file_id())
            //         && (decl.get_name() == "pcall" || decl.get_name() == "xpcall")
            //     {
            //         return Some(true);
            //     }
            // }
            // None
        }
        _ => None,
    }
}
