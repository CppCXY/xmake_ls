use emmylua_parser::{LuaCallExpr, LuaExpr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmakeFunction {
    AddFiles,
    Includes,
    Import,
    AddDeps,
    Target,
    Package,
    Option,
    Rule,
    Task,
    EndTarget,
    EndPackage,
    EndOption,
    EndRule,
    EndTask,
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
        "add_deps" => Some(XmakeFunction::AddDeps),
        "target" => Some(XmakeFunction::Target),
        "end_target" => Some(XmakeFunction::EndTarget),
        "package" => Some(XmakeFunction::Package),
        "end_package" => Some(XmakeFunction::EndPackage),
        "option" => Some(XmakeFunction::Option),
        "end_option" => Some(XmakeFunction::EndOption),
        "rule" => Some(XmakeFunction::Rule),
        "end_rule" => Some(XmakeFunction::EndRule),
        "task" => Some(XmakeFunction::Task),
        "end_task" => Some(XmakeFunction::EndTask),
        _ => return None,
    }
}
