use emmylua_parser::{
    LuaAssignStat, LuaAstNode, LuaAstToken, LuaForRangeStat, LuaForStat, LuaFuncStat,
    LuaIfClauseStat, LuaIfStat, LuaLocalFuncStat, LuaLocalStat,
};
use lsp_types::SymbolKind;
use xmake_code_analysis::{LuaDeclId, LuaSignatureId, LuaType};

use super::builder::{DocumentSymbolBuilder, LuaSymbol};

pub fn build_local_stat_symbol(
    builder: &mut DocumentSymbolBuilder,
    local_stat: LuaLocalStat,
) -> Option<()> {
    let file_id = builder.get_file_id();
    let local_names: Vec<_> = local_stat.get_local_name_list().collect();
    let simple_local = local_names.len() == 1;

    for local_name in local_names {
        let decl_id = LuaDeclId::new(file_id, local_name.get_position());
        let decl = builder.get_decl(&decl_id)?;
        let typ = builder.get_type(decl_id.into());
        let desc = builder.get_symbol_kind_and_detail(Some(&typ));
        let range = if simple_local {
            local_stat.get_range()
        } else {
            decl.get_range()
        };

        let symbol = LuaSymbol::new(decl.get_name().to_string(), desc.1, desc.0, range);

        builder.add_node_symbol(local_name.syntax().clone(), symbol);
    }

    Some(())
}

pub fn build_assign_stat_symbol(
    builder: &mut DocumentSymbolBuilder,
    assign_stat: LuaAssignStat,
) -> Option<()> {
    let file_id = builder.get_file_id();
    let (vars, _) = assign_stat.get_var_and_expr_list();
    let simple_var = vars.len() == 1;
    for var in vars {
        let decl_id = LuaDeclId::new(file_id, var.get_position());
        let decl = match builder.get_decl(&decl_id) {
            Some(decl) => decl,
            None => continue,
        };
        let range = if simple_var {
            assign_stat.get_range()
        } else {
            decl.get_range()
        };
        let typ = builder.get_type(decl_id.into());
        let desc = builder.get_symbol_kind_and_detail(Some(&typ));
        let symbol = LuaSymbol::new(decl.get_name().to_string(), desc.1, desc.0, range);

        builder.add_node_symbol(var.syntax().clone(), symbol);
    }

    Some(())
}

pub fn build_for_stat_symbol(
    builder: &mut DocumentSymbolBuilder,
    for_stat: LuaForStat,
) -> Option<()> {
    let file_id = builder.get_file_id();
    let for_symbol = LuaSymbol::new(
        "for".to_string(),
        None,
        SymbolKind::MODULE,
        for_stat.get_range(),
    );
    builder.add_node_symbol(for_stat.syntax().clone(), for_symbol);

    let iter_token = for_stat.get_var_name()?;
    let decl_id = LuaDeclId::new(file_id, iter_token.get_position());
    let decl = builder.get_decl(&decl_id)?;
    let typ = builder.get_type(decl_id.into());
    let desc = builder.get_symbol_kind_and_detail(Some(&typ));
    let symbol = LuaSymbol::new(
        decl.get_name().to_string(),
        desc.1,
        desc.0,
        decl.get_range(),
    );

    builder.add_token_symbol(iter_token.syntax().clone(), symbol);
    Some(())
}

pub fn build_for_range_stat_symbol(
    builder: &mut DocumentSymbolBuilder,
    for_range_stat: LuaForRangeStat,
) -> Option<()> {
    let file_id = builder.get_file_id();
    let for_symbol = LuaSymbol::new(
        "for in".to_string(),
        None,
        SymbolKind::MODULE,
        for_range_stat.get_range(),
    );

    builder.add_node_symbol(for_range_stat.syntax().clone(), for_symbol);

    let vars = for_range_stat.get_var_name_list();
    for var in vars {
        let decl_id = LuaDeclId::new(file_id, var.get_position());
        let decl = builder.get_decl(&decl_id)?;
        let typ = builder.get_type(decl_id.into());
        let desc = builder.get_symbol_kind_and_detail(Some(&typ));
        let symbol = LuaSymbol::new(
            decl.get_name().to_string(),
            desc.1,
            desc.0,
            decl.get_range(),
        );

        builder.add_token_symbol(var.syntax().clone(), symbol);
    }

    Some(())
}

pub fn build_local_func_stat_symbol(
    builder: &mut DocumentSymbolBuilder,
    local_func: LuaLocalFuncStat,
) -> Option<()> {
    let file_id = builder.get_file_id();
    let func_name = local_func.get_local_name()?;
    let decl_id = LuaDeclId::new(file_id, func_name.get_position());
    let decl = builder.get_decl(&decl_id)?;
    let typ = builder.get_type(decl_id.into());
    let desc = builder.get_symbol_kind_and_detail(Some(&typ));

    let full_range = local_func.get_range();
    let name_range = decl.get_range();

    let symbol = LuaSymbol::with_selection_range(
        decl.get_name().to_string(),
        desc.1,
        desc.0,
        full_range,
        name_range,
    );

    builder.add_node_symbol(local_func.syntax().clone(), symbol);
    Some(())
}

pub fn build_func_stat_symbol(
    builder: &mut DocumentSymbolBuilder,
    func: LuaFuncStat,
) -> Option<()> {
    let file_id = builder.get_file_id();
    let func_name = func.get_func_name()?;
    let name = func_name.syntax().text().to_string();
    let closure = func.get_closure()?;
    let signature_id = LuaSignatureId::from_closure(file_id, &closure);
    let func_ty = LuaType::Signature(signature_id);
    let desc = builder.get_symbol_kind_and_detail(Some(&func_ty));

    let full_range = func.get_range();
    let name_range = func_name.get_range();

    let symbol = LuaSymbol::with_selection_range(name, desc.1, desc.0, full_range, name_range);

    builder.add_node_symbol(func.syntax().clone(), symbol);
    Some(())
}

pub fn build_if_stat_symbol(builder: &mut DocumentSymbolBuilder, if_stat: LuaIfStat) -> Option<()> {
    let if_symbol = LuaSymbol::new(
        "if".to_string(),
        None,
        SymbolKind::MODULE,
        if_stat.get_range(),
    );

    builder.add_node_symbol(if_stat.syntax().clone(), if_symbol);

    for branch in if_stat.get_all_clause() {
        let name = match &branch {
            LuaIfClauseStat::Else(_) => "else",
            LuaIfClauseStat::ElseIf(_) => "elseif",
        };

        let symbol = LuaSymbol::new(
            name.to_string(),
            None,
            SymbolKind::MODULE,
            branch.get_range(),
        );

        builder.add_node_symbol(branch.syntax().clone(), symbol);
    }

    Some(())
}
