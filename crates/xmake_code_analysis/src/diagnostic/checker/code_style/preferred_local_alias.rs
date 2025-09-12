use std::collections::{HashMap, HashSet};

use emmylua_parser::{
    LuaAst, LuaAstNode, LuaAstToken, LuaExpr, LuaIndexExpr, LuaIndexKey, LuaLocalStat,
    LuaSyntaxKind,
};
use rowan::{NodeOrToken, TextRange};

use crate::{
    DiagnosticCode, LuaDeclId, LuaSemanticDeclId, SemanticDeclLevel, SemanticModel,
    diagnostic::checker::{Checker, DiagnosticContext},
};

pub struct PreferredLocalAliasChecker;

impl Checker for PreferredLocalAliasChecker {
    const CODES: &[DiagnosticCode] = &[DiagnosticCode::PreferredLocalAlias];

    fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) {
        let mut local_alias_set = LocalAliasSet::new();
        let root = semantic_model.get_root().clone();
        for walk in root.walk_descendants::<LuaAst>() {
            match walk {
                rowan::WalkEvent::Enter(node) => {
                    if is_scope(&node) {
                        local_alias_set.push();
                    }

                    match node {
                        LuaAst::LuaLocalStat(local_stat) => {
                            collect_local_alias(&mut local_alias_set, semantic_model, &local_stat);
                        }
                        LuaAst::LuaIndexExpr(index_expr) => {
                            check_index_expr_preference(
                                context,
                                &mut local_alias_set,
                                semantic_model,
                                &index_expr,
                            );
                        }
                        _ => {}
                    }
                }
                rowan::WalkEvent::Leave(node) => {
                    if is_scope(&node) {
                        local_alias_set.pop();
                    }
                }
            }
        }
    }
}

fn is_scope(node: &LuaAst) -> bool {
    matches!(
        node.syntax().kind().into(),
        LuaSyntaxKind::Chunk | LuaSyntaxKind::Block | LuaSyntaxKind::ClosureExpr
    )
}

fn collect_local_alias(
    local_alias_set: &mut LocalAliasSet,
    semantic_model: &SemanticModel,
    local_stat: &LuaLocalStat,
) -> Option<()> {
    let local_list = local_stat.get_local_name_list().collect::<Vec<_>>();
    let value_expr = local_stat.get_value_exprs().collect::<Vec<_>>();
    let min_len = local_list.len().min(value_expr.len());
    for i in 0..min_len {
        let local_name = &local_list[i];
        let value_expr = &value_expr[i];
        if is_only_dot_index_expr(value_expr).unwrap_or(false) {
            let decl_id = LuaDeclId::new(semantic_model.get_file_id(), local_name.get_position());
            let decl_refs = semantic_model
                .get_db()
                .get_reference_index()
                .get_decl_references(&semantic_model.get_file_id(), &decl_id);
            if let Some(decl_refs) = decl_refs {
                if decl_refs.mutable {
                    continue;
                }
            }

            let name = match value_expr {
                LuaExpr::IndexExpr(index_expr) => {
                    let index_key = index_expr.get_index_key()?;
                    match index_key {
                        LuaIndexKey::Name(name_token) => name_token.get_name_text().to_string(),
                        _ => continue,
                    }
                }
                _ => continue,
            };
            let node_or_token = NodeOrToken::Node(value_expr.syntax().clone());
            if let Some(semantic_id) =
                semantic_model.find_decl(node_or_token, SemanticDeclLevel::NoTrace)
            {
                let name_token = local_name.get_name_token()?;
                let preferred_name = name_token.get_name_text();

                local_alias_set.insert(name, preferred_name.to_string(), semantic_id);
                local_alias_set.add_disable_check(value_expr.get_range());
            }
        }
    }

    Some(())
}

fn is_only_dot_index_expr(expr: &LuaExpr) -> Option<bool> {
    let mut index_expr = match expr {
        LuaExpr::IndexExpr(index_expr) => index_expr.clone(),
        _ => return Some(false),
    };

    loop {
        let index_token = index_expr.get_index_token()?;
        if !index_token.is_dot() {
            return Some(false);
        }
        match index_expr.get_prefix_expr() {
            Some(LuaExpr::NameExpr(_)) => return Some(true),
            Some(LuaExpr::IndexExpr(prefix_index_expr)) => {
                index_expr = prefix_index_expr;
            }
            _ => return Some(false),
        }
    }
}

#[derive(Debug)]
struct LocalAliasSet {
    local_alias_stack: Vec<HashMap<String, LocalAliasInfo>>,
    disable_check: HashSet<TextRange>,
}

#[derive(Debug)]
struct LocalAliasInfo {
    pub decl_id: LuaSemanticDeclId,
    pub preferred_name: String,
    pub invalid: bool,
}

impl LocalAliasSet {
    fn new() -> Self {
        LocalAliasSet {
            local_alias_stack: vec![HashMap::new()],
            disable_check: HashSet::new(),
        }
    }

    fn push(&mut self) {
        self.local_alias_stack.push(HashMap::new());
    }

    fn pop(&mut self) {
        self.local_alias_stack.pop();
    }

    fn insert(&mut self, name: String, preferred_name: String, decl_id: LuaSemanticDeclId) {
        if let Some(map) = self.local_alias_stack.last_mut() {
            map.insert(
                name,
                LocalAliasInfo {
                    decl_id,
                    preferred_name,
                    invalid: false,
                },
            );
        }
    }

    fn get(&mut self, name: &str) -> Option<&mut LocalAliasInfo> {
        for map in self.local_alias_stack.iter_mut().rev() {
            if let Some(item) = map.get_mut(name) {
                return Some(item);
            }
        }
        None
    }

    fn add_disable_check(&mut self, range: TextRange) {
        self.disable_check.insert(range);
    }

    fn is_disable_check(&self, range: &TextRange) -> bool {
        self.disable_check.contains(range)
    }
}

fn check_index_expr_preference(
    context: &mut DiagnosticContext,
    local_alias_set: &mut LocalAliasSet,
    semantic_model: &SemanticModel,
    index_expr: &LuaIndexExpr,
) -> Option<()> {
    if local_alias_set.is_disable_check(&index_expr.get_range()) {
        return Some(());
    }

    let expr = LuaExpr::IndexExpr(index_expr.clone());
    if !is_only_dot_index_expr(&expr).unwrap_or(false) {
        return Some(());
    }

    let parent = index_expr.get_parent::<LuaAst>()?;
    let mut mutable_index = false;
    match parent {
        LuaAst::LuaAssignStat(assign_stat) => {
            let eq = assign_stat.get_assign_op()?;
            if eq.get_position() > index_expr.get_position() {
                mutable_index = true;
            }
        }
        LuaAst::LuaFuncStat(_) => {
            mutable_index = true;
        }
        _ => {}
    }

    let index_key = index_expr.get_index_key()?;
    let name = match index_key {
        LuaIndexKey::Name(name_token) => name_token.get_name_text().to_string(),
        _ => return Some(()),
    };

    let alias_info = local_alias_set.get(&name)?;
    if alias_info.invalid {
        return Some(());
    }

    if semantic_model.is_reference_to(
        index_expr.syntax().clone(),
        alias_info.decl_id.clone(),
        SemanticDeclLevel::NoTrace,
    ) {
        if mutable_index {
            alias_info.invalid = true;
            return Some(());
        }

        context.add_diagnostic(
            DiagnosticCode::PreferredLocalAlias,
            index_expr.get_range(),
            t!(
                "Prefer use local alias variable '%{name}'",
                name = alias_info.preferred_name
            )
            .to_string(),
            None,
        );
    }

    Some(())
}
