mod file_path_provider;

use emmylua_parser::{
    LuaAstNode, LuaAstToken, LuaCallArgList, LuaCallExpr, LuaExpr, LuaStringToken,
};

use crate::handlers::completion::completion_builder::CompletionBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmakeFunction {
    AddFiles,
    Includes,
}

pub fn add_completion(builder: &mut CompletionBuilder) -> Option<()> {
    if builder.is_cancelled() {
        return None;
    }

    let string_token = LuaStringToken::cast(builder.trigger_token.clone())?;
    let xmake_function = get_xmake_function(&string_token)?;
    match xmake_function {
        XmakeFunction::AddFiles | XmakeFunction::Includes => {
            file_path_provider::add_completion(builder, string_token);
        }
    }

    builder.stop_here();
    Some(())
}

fn get_xmake_function(string_token: &LuaStringToken) -> Option<XmakeFunction> {
    let call_expr = string_token
        .get_parent::<LuaCallArgList>()?
        .get_parent::<LuaCallExpr>()?;
    let prefix_expr = call_expr.get_prefix_expr()?;
    let LuaExpr::NameExpr(name_expr) = prefix_expr else {
        return None;
    };

    let name = name_expr.get_name_text()?;
    match name.as_str() {
        "add_files" => Some(XmakeFunction::AddFiles),
        "includes" => Some(XmakeFunction::Includes),
        _ => return None,
    }
}
