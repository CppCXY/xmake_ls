use emmylua_parser::{LuaCallExpr, LuaExpr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmakeFunction {
    AddFiles,
    Includes,
    Import,
}

pub fn get_xmake_function(call_expr: &LuaCallExpr) -> Option<XmakeFunction> {
    let prefix_expr = call_expr.get_prefix_expr()?;
    let LuaExpr::NameExpr(name_expr) = prefix_expr else {
        return None;
    };

    let name = name_expr.get_name_text()?;
    match name.as_str() {
        "add_files" => Some(XmakeFunction::AddFiles),
        "includes" => Some(XmakeFunction::Includes),
        "import" => Some(XmakeFunction::Import),
        _ => return None,
    }
}
