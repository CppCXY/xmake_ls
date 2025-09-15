use emmylua_parser::{LuaAstNode, LuaCallExpr, LuaExpr, LuaIndexKey, LuaLiteralToken};

use crate::{LuaDecl, LuaDeclExtra, XmakeFunction, compilation::analyzer::decl::DeclAnalyzer};

pub fn analyze_xmake_function_call(
    analyzer: &mut DeclAnalyzer,
    call_expr: &LuaCallExpr,
    xmake_function: XmakeFunction,
) -> Option<()> {
    match xmake_function {
        XmakeFunction::Import => {
            analyze_import(analyzer, call_expr);
        }
        _ => {}
    }

    Some(())
}

fn analyze_import(analyzer: &mut DeclAnalyzer, call_expr: &LuaCallExpr) -> Option<()> {
    let arg_list = call_expr.get_args_list()?;
    let args = arg_list.get_args().collect::<Vec<_>>();
    if args.is_empty() {
        return None;
    }

    let first_arg = &args[0];
    let string_token = match first_arg {
        LuaExpr::LiteralExpr(s) => match s.get_literal()? {
            LuaLiteralToken::String(string_token) => string_token,
            _ => return None,
        },
        _ => return None,
    };

    let import_path = string_token.get_value();
    let parts = import_path.split('.').collect::<Vec<_>>();
    if parts.is_empty() {
        return None;
    }

    let mut local_name = parts.last()?.to_string();
    if args.len() >= 2 {
        if let LuaExpr::TableExpr(table_expr) = &args[1] {
            let fields = table_expr.get_fields();
            for field in fields {
                if let Some(index_key) = field.get_field_key() {
                    match index_key {
                        LuaIndexKey::Name(name) => {
                            let field_name = name.get_name_text();
                            if field_name == "alias" {
                                if let Some(value) = field.get_value_expr() {
                                    if let LuaExpr::LiteralExpr(literal) = value {
                                        if let LuaLiteralToken::String(string_token) =
                                            literal.get_literal()?
                                        {
                                            local_name = string_token.get_value();
                                        }
                                    }
                                }
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let decl = LuaDecl::new(
        &local_name,
        analyzer.get_file_id(),
        call_expr.get_range(),
        LuaDeclExtra::Local {
            kind: call_expr.syntax().kind(),
            attrib: None,
        },
        None,
    );

    analyzer.add_decl(decl);

    Some(())
}
