use emmylua_parser::{LuaChunk, LuaExpr, LuaIndexExpr};

use crate::{
    DbIndex, FlowNode, FlowTree, InferFailReason, LuaInferCache,
    semantic::infer::{
        VarRefId,
        narrow::{
            ResultTypeOrContinue, condition_flow::InferConditionFlow, get_single_antecedent,
            get_type_at_flow::get_type_at_flow, narrow_false_or_nil, remove_false_or_nil,
            var_ref_id::get_var_expr_var_ref_id,
        },
    },
};

#[allow(unused)]
pub fn get_type_at_index_expr(
    db: &DbIndex,
    tree: &FlowTree,
    cache: &mut LuaInferCache,
    root: &LuaChunk,
    var_ref_id: &VarRefId,
    flow_node: &FlowNode,
    index_expr: LuaIndexExpr,
    condition_flow: InferConditionFlow,
) -> Result<ResultTypeOrContinue, InferFailReason> {
    let Some(name_var_ref_id) =
        get_var_expr_var_ref_id(db, cache, LuaExpr::IndexExpr(index_expr.clone()))
    else {
        return Ok(ResultTypeOrContinue::Continue);
    };

    if name_var_ref_id != *var_ref_id {
        return Ok(ResultTypeOrContinue::Continue);
    }

    let antecedent_flow_id = get_single_antecedent(tree, flow_node)?;
    let antecedent_type = get_type_at_flow(db, tree, cache, root, var_ref_id, antecedent_flow_id)?;

    let result_type = match condition_flow {
        InferConditionFlow::FalseCondition => narrow_false_or_nil(db, antecedent_type),
        InferConditionFlow::TrueCondition => remove_false_or_nil(antecedent_type),
    };

    Ok(ResultTypeOrContinue::Result(result_type))
}
