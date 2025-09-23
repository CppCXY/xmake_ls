mod deps_provider;
mod file_path_provider;
mod import_module_provider;
mod include_module_provider;

use emmylua_parser::{
    LuaAstNode, LuaAstToken, LuaCallArgList, LuaCallExpr, LuaLiteralExpr, LuaStringToken,
};
use xmake_code_analysis::{XmakeFunction, get_xmake_function};

use crate::handlers::completion::completion_builder::CompletionBuilder;

pub fn add_completion(builder: &mut CompletionBuilder) -> Option<()> {
    if builder.is_cancelled() {
        return None;
    }

    let string_token = LuaStringToken::cast(builder.trigger_token.clone())?;
    let call_expr = string_token
        .get_parent::<LuaLiteralExpr>()?
        .get_parent::<LuaCallArgList>()?
        .get_parent::<LuaCallExpr>()?;
    let xmake_function = get_xmake_function(&call_expr)?;
    match xmake_function {
        XmakeFunction::AddFiles => {
            file_path_provider::add_completion(builder, string_token);
        }
        XmakeFunction::Import => {
            import_module_provider::add_completion(builder, string_token);
        }
        XmakeFunction::Includes => {
            include_module_provider::add_completion(builder, string_token);
        }
        XmakeFunction::AddDeps => {
            deps_provider::add_completion(builder, string_token);
        }
        _ => return None,
    }

    builder.stop_here();
    Some(())
}
