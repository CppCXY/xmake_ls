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
    let expr_len = expr_types.len();
    let arg_count = arg_count.unwrap_or(expr_len);
    let mut need_resolve_funcs = match overloads.len() {
        0 => return Err(InferFailReason::None),
        1 => return Ok(Arc::clone(&overloads[0])),
        _ => overloads
            .iter()
            .map(|it| Some(it.clone()))
            .collect::<Vec<_>>(),
    };

    if expr_len == 0 {
        for overload in overloads {
            let param_len = overload.get_params().len();
            if param_len == 0 {
                return Ok(overload.clone());
            }
        }
    }

    let mut best_match_result = need_resolve_funcs[0].clone().unwrap();
    for arg_index in 0..expr_len {
        let mut current_match_result = ParamMatchResult::NotMatch;
        for i in 0..need_resolve_funcs.len() {
            let opt_func = &need_resolve_funcs[i];
            if opt_func.is_none() {
                continue;
            }
            let func = opt_func.as_ref().unwrap();
            let param_len = func.get_params().len();
            if param_len < arg_count && !is_func_last_param_variadic(func) {
                need_resolve_funcs[i] = None;
                continue;
            }

            let colon_define = func.is_colon_define();
            let mut param_index = arg_index;
            match (colon_define, is_colon_call) {
                (true, false) => {
                    if param_index == 0 {
                        continue;
                    }
                    param_index -= 1;
                }
                (false, true) => {
                    param_index += 1;
                }
                _ => {}
            }
            let expr_type = &expr_types[arg_index];
            let param_type = if param_index < param_len {
                let param_info = func.get_params().get(param_index);
                param_info
                    .map(|it| it.1.clone().unwrap_or(LuaType::Any))
                    .unwrap_or(LuaType::Any)
            } else if let Some(last_param_info) = func.get_params().last() {
                if last_param_info.0 == "..." {
                    last_param_info.1.clone().unwrap_or(LuaType::Any)
                } else {
                    need_resolve_funcs[i] = None;
                    continue;
                }
            } else {
                need_resolve_funcs[i] = None;
                continue;
            };

            let match_result = if param_type.is_any() {
                ParamMatchResult::AnyMatch
            } else if check_type_compact(db, &param_type, &expr_type).is_ok() {
                ParamMatchResult::TypeMatch
            } else {
                ParamMatchResult::NotMatch
            };

            if match_result > current_match_result {
                current_match_result = match_result;
                best_match_result = func.clone();
            }

            if match_result == ParamMatchResult::NotMatch {
                need_resolve_funcs[i] = None;
                continue;
            }

            if match_result > ParamMatchResult::AnyMatch
                && arg_index + 1 == expr_len
                && param_index + 1 == func.get_params().len()
            {
                return Ok(func.clone());
            }
        }

        if current_match_result == ParamMatchResult::NotMatch {
            break;
        }
    }

    let mut rest_need_resolve_funcs = need_resolve_funcs
        .iter()
        .filter_map(|it| it.clone())
        .map(|it| Some(it))
        .collect::<Vec<_>>();

    match rest_need_resolve_funcs.len() {
        0 => return Ok(best_match_result),
        1 => return Ok(rest_need_resolve_funcs[0].clone().unwrap()),
        _ => {}
    }

    let start_param_index = expr_len;
    let mut max_param_len = 0;
    for opt_func in &rest_need_resolve_funcs {
        if let Some(func) = opt_func {
            let param_len = func.get_params().len();
            if param_len > max_param_len {
                max_param_len = param_len;
            }
        }
    }

    let rest_len = rest_need_resolve_funcs.len();
    for param_index in start_param_index..max_param_len {
        let mut current_match_result = ParamMatchResult::NotMatch;
        for i in 0..rest_len {
            let opt_func = &rest_need_resolve_funcs[i];
            if opt_func.is_none() {
                continue;
            }
            let func = opt_func.as_ref().unwrap();
            let param_len = func.get_params().len();
            let colon_define = func.is_colon_define();
            let mut param_index = param_index;
            match (colon_define, is_colon_call) {
                (true, false) => {
                    if param_index == 0 {
                        continue;
                    }
                    param_index -= 1;
                }
                (false, true) => {
                    param_index += 1;
                }
                _ => {}
            }
            let param_type = if param_index < param_len {
                let param_info = func.get_params().get(param_index);
                param_info
                    .map(|it| it.1.clone().unwrap_or(LuaType::Any))
                    .unwrap_or(LuaType::Any)
            } else if let Some(last_param_info) = func.get_params().last() {
                if last_param_info.0 == "..." {
                    last_param_info.1.clone().unwrap_or(LuaType::Any)
                } else {
                    return Ok(func.clone());
                }
            } else {
                return Ok(func.clone());
            };

            let match_result = if param_type.is_any() {
                ParamMatchResult::AnyMatch
            } else if param_type.is_nullable() {
                ParamMatchResult::TypeMatch
            } else {
                ParamMatchResult::NotMatch
            };

            if match_result > current_match_result {
                current_match_result = match_result;
                best_match_result = func.clone();
            }

            if match_result == ParamMatchResult::NotMatch {
                rest_need_resolve_funcs[i] = None;
                continue;
            }

            if match_result >= ParamMatchResult::AnyMatch
                && i + 1 == rest_len
                && param_index + 1 == func.get_params().len()
            {
                return Ok(func.clone());
            }
        }

        if current_match_result == ParamMatchResult::NotMatch {
            break;
        }
    }

    Ok(best_match_result)
}

fn is_func_last_param_variadic(func: &LuaFunctionType) -> bool {
    if let Some(last_param) = func.get_params().last() {
        last_param.0 == "..."
    } else {
        false
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum ParamMatchResult {
    NotMatch,
    AnyMatch,
    TypeMatch,
}
