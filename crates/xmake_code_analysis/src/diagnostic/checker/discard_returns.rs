use emmylua_parser::{LuaAstNode, LuaCallExprStat};
use rowan::NodeOrToken;

use crate::{
    DiagnosticCode, LuaNoDiscard, LuaSemanticDeclId, LuaType, SemanticDeclLevel, SemanticModel,
};

use super::{Checker, DiagnosticContext};

pub struct DiscardReturnsChecker;

impl Checker for DiscardReturnsChecker {
    const CODES: &[DiagnosticCode] = &[DiagnosticCode::DiscardReturns];

    fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) {
        let root = semantic_model.get_root().clone();
        for call_expr_stat in root.descendants::<LuaCallExprStat>() {
            check_call_expr(context, semantic_model, call_expr_stat);
        }
    }
}

fn check_call_expr(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    call_expr_stat: LuaCallExprStat,
) -> Option<()> {
    let call_expr = call_expr_stat.get_call_expr()?;
    let prefix_node = call_expr.get_prefix_expr()?.syntax().clone();
    let semantic_decl = semantic_model.find_decl(
        NodeOrToken::Node(prefix_node.clone()),
        SemanticDeclLevel::default(),
    )?;

    let signature_id = match semantic_decl {
        LuaSemanticDeclId::LuaDecl(decl_id) => {
            let type_cache = semantic_model
                .get_db()
                .get_type_index()
                .get_type_cache(&decl_id.into());
            if let Some(type_cache) = type_cache {
                if let LuaType::Signature(signature_id) = type_cache.as_type() {
                    signature_id.clone()
                } else {
                    return Some(());
                }
            } else {
                return Some(());
            }
        }
        LuaSemanticDeclId::Member(member_id) => {
            let type_cache = semantic_model
                .get_db()
                .get_type_index()
                .get_type_cache(&member_id.into());
            if let Some(type_cache) = type_cache {
                if let LuaType::Signature(signature_id) = type_cache.as_type() {
                    signature_id.clone()
                } else {
                    return Some(());
                }
            } else {
                return Some(());
            }
        }
        LuaSemanticDeclId::Signature(signature_id) => signature_id,
        _ => return Some(()),
    };

    let signature = semantic_model
        .get_db()
        .get_signature_index()
        .get(&signature_id)?;
    if let Some(nodiscard) = &signature.nodiscard {
        let nodiscard_message = match nodiscard {
            LuaNoDiscard::NoDiscard => "no discard".to_string(),
            LuaNoDiscard::NoDiscardWithMessage(message) => message.to_string(),
        };

        context.add_diagnostic(
            DiagnosticCode::DiscardReturns,
            prefix_node.text_range(),
            nodiscard_message,
            None,
        );
    }

    Some(())
}
