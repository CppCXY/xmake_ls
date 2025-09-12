use std::sync::Arc;

use crate::{
    InferFailReason, check_type_compact,
    db_index::{DbIndex, LuaFunctionType, LuaType},
    semantic::infer::InferCallFuncResult,
};

pub fn resolve_signature_by_args(
    db: &DbIndex,
    overloads: &[Arc<LuaFunctionType>],
    expr_types: &[LuaType],
    is_colon_call: bool,
    arg_count: Option<usize>,
) -> InferCallFuncResult {
    let arg_count = arg_count.unwrap_or(0);
    let mut opt_funcs = Vec::with_capacity(overloads.len());

    for func in overloads {
        let params = func.get_params();
        if params.len() < arg_count {
            continue;
        }
        let mut total_weight = 0; // 总权重

        let mut fake_expr_len = expr_types.len();
        let jump_param;
        if is_colon_call && !func.is_colon_define() {
            jump_param = 1;
            fake_expr_len += 1;
        } else {
            jump_param = 0;
        };

        // 如果在不计算可空参数的情况下, 参数数量完全匹配, 则认为其权重更高
        if params.len() == fake_expr_len {
            total_weight += params.len() as i32 + 1;
        }

        // 冒号定义且冒号调用
        if is_colon_call && func.is_colon_define() {
            total_weight += 100;
        }

        // 检查每个参数的匹配情况
        for (i, param) in params.iter().enumerate() {
            if i == 0 && jump_param > 0 {
                // 非冒号定义但是冒号调用, 直接认为匹配
                total_weight += 100;
                continue;
            }
            let param_type = param.1.as_ref().unwrap_or(&LuaType::Any);
            let expr_idx = i - jump_param;

            if expr_idx >= expr_types.len() {
                // 没有传入参数, 但参数是可空类型
                if param_type.is_nullable() {
                    total_weight += 1;
                    fake_expr_len += 1;
                }
                continue;
            }

            let expr_type = &expr_types[expr_idx];
            if *param_type == LuaType::Any || check_type_compact(db, param_type, expr_type).is_ok()
            {
                total_weight += 100; // 类型完全匹配
            }
        }
        // 如果参数数量完全匹配, 则认为其权重更高
        if params.len() == fake_expr_len {
            total_weight += 50000;
        }

        opt_funcs.push((func, total_weight));
    }

    // 按权重降序排序
    opt_funcs.sort_by(|a, b| b.1.cmp(&a.1));
    // 返回权重最高的签名，若无则取最后一个重载作为默认
    opt_funcs
        .first()
        .filter(|(_, weight)| *weight > i32::MIN) // 确保不是无效签名
        .map(|(func, _)| Arc::clone(func))
        .or_else(|| overloads.last().cloned())
        .ok_or(InferFailReason::None)
}
