use emmylua_parser::{LuaAstNode, LuaClosureExpr, LuaIndexKey, LuaSyntaxKind, LuaTableExpr};
use lsp_types::SymbolKind;
use xmake_code_analysis::LuaDeclId;

use super::builder::{DocumentSymbolBuilder, LuaSymbol};

pub fn build_closure_expr_symbol(
    builder: &mut DocumentSymbolBuilder,
    closure: LuaClosureExpr,
) -> Option<()> {
    let parent = closure.syntax().parent()?;
    if !matches!(
        parent.kind().into(),
        LuaSyntaxKind::LocalFuncStat | LuaSyntaxKind::FuncStat
    ) {
        let symbol = LuaSymbol::new(
            "closure".to_string(),
            None,
            SymbolKind::MODULE,
            closure.get_range(),
        );

        builder.add_node_symbol(closure.syntax().clone(), symbol);
    }

    let file_id = builder.get_file_id();
    let param_list = closure.get_params_list()?;
    for param in param_list.get_params() {
        let decl_id = LuaDeclId::new(file_id, param.get_position());
        let decl = builder.get_decl(&decl_id)?;
        let typ = builder.get_type(decl_id.into());
        let desc = builder.get_symbol_kind_and_detail(Some(&typ));
        let symbol = LuaSymbol::new(
            decl.get_name().to_string(),
            desc.1,
            desc.0,
            decl.get_range(),
        );

        builder.add_node_symbol(param.syntax().clone(), symbol);
    }

    Some(())
}

pub fn build_table_symbol(builder: &mut DocumentSymbolBuilder, table: LuaTableExpr) -> Option<()> {
    let symbol = LuaSymbol::new(
        "table".to_string(),
        None,
        SymbolKind::STRUCT,
        table.get_range(),
    );

    builder.add_node_symbol(table.syntax().clone(), symbol);

    if table.is_object() {
        for field in table.get_fields() {
            let key = field.get_field_key()?;
            let str_key = match key {
                LuaIndexKey::String(key) => key.get_value(),
                LuaIndexKey::Name(key) => key.get_name_text().to_string(),
                LuaIndexKey::Integer(i) => i.get_int_value().to_string(),
                _ => continue,
            };

            let symbol = LuaSymbol::new(str_key, None, SymbolKind::FIELD, field.get_range());

            builder.add_node_symbol(field.syntax().clone(), symbol);
        }
    }

    Some(())
}
