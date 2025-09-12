use emmylua_parser::{LuaAstNode, LuaComment, LuaDocTag};

use crate::{
    FlowId, FlowNodeKind,
    compilation::analyzer::flow::{bind_analyze::exprs::bind_expr, binder::FlowBinder},
};

pub fn bind_comment(binder: &mut FlowBinder, lua_comment: LuaComment, current: FlowId) -> FlowId {
    let cast_tags = lua_comment.get_doc_tags().filter_map(|it| match it {
        LuaDocTag::Cast(cast) => Some(cast),
        _ => None,
    });

    let mut parent = current;
    for cast in cast_tags {
        let expr = cast.get_key_expr();
        if let Some(expr) = expr {
            bind_expr(binder, expr, current);

            let flow_id = binder.create_node(FlowNodeKind::TagCast(cast.to_ptr()));
            binder.add_antecedent(flow_id, parent);
            parent = flow_id;
        } else {
            // inline cast
            let Some(owner) = lua_comment.get_owner() else {
                continue;
            };

            let flow_id = binder.create_node(FlowNodeKind::TagCast(cast.to_ptr()));
            let bind_flow_id = binder.get_bind_flow(owner.get_syntax_id());
            if let Some(bind_flow) = bind_flow_id {
                binder.add_antecedent(flow_id, bind_flow);
                binder.bind_syntax_node(owner.get_syntax_id(), flow_id);
            } else {
                binder.add_antecedent(flow_id, parent);
                binder.bind_syntax_node(owner.get_syntax_id(), flow_id);
            }
        }
    }

    parent
}
