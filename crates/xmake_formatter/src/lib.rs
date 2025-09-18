mod format;
mod style_ruler;
mod test;

use emmylua_parser::{LuaAst, LuaParser, ParserConfig};

pub fn reformat_xmake_code(code: &str) -> String {
    let tree = LuaParser::parse(code, ParserConfig::default());

    let mut formatter = format::XmakeFormatter::new(LuaAst::LuaChunk(tree.get_chunk_node()));
    style_ruler::apply_styles(&mut formatter);
    let formatted_text = formatter.get_formatted_text();
    formatted_text
}

pub fn reformat_node(node: &LuaAst) -> String {
    let mut formatter = format::XmakeFormatter::new(node.clone());
    style_ruler::apply_styles(&mut formatter);
    let formatted_text = formatter.get_formatted_text();
    formatted_text
}
