use emmylua_parser::{LuaAstNode, LuaExpr, LuaIfStat};

use crate::{DiagnosticCode, SemanticModel};

use super::{Checker, DiagnosticContext};

pub struct UnnecessaryIfChecker;

impl Checker for UnnecessaryIfChecker {
    const CODES: &[DiagnosticCode] = &[DiagnosticCode::UnnecessaryIf];

    fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) {
        let root = semantic_model.get_root().clone();
        for if_statement in root.descendants::<LuaIfStat>() {
            if let Some(condition) = if_statement.get_condition_expr() {
                check_condition(context, semantic_model, condition);
            }
            for clause in if_statement.get_else_if_clause_list() {
                if let Some(condition) = clause.get_condition_expr() {
                    check_condition(context, semantic_model, condition);
                }
            }
        }
    }
}

fn check_condition(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    condition: LuaExpr,
) -> Option<()> {
    let expr_type = semantic_model.infer_expr(condition.clone()).ok()?;

    if expr_type.is_always_truthy() {
        context.add_diagnostic(
            DiagnosticCode::UnnecessaryIf,
            condition.get_range(),
            t!("Unnecessary `if` statement: this condition is always truthy").to_string(),
            None,
        );
    } else if expr_type.is_always_falsy() {
        context.add_diagnostic(
            DiagnosticCode::UnnecessaryIf,
            condition.get_range(),
            t!("Impossible `if` statement: this condition is always falsy").to_string(),
            None,
        );
    }
    Some(())
}
