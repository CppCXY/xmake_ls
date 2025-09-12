use std::collections::HashSet;

use crate::{DbIndex, LuaType, LuaTypeDeclId};

/// 检查子类型关系.
///
/// 假设现在有一个类型定义`---@class C: A, B`, 那么`sub_type_ref_id`为`C`, `super_type_ref_id`可以为`A`或`B`.
pub fn is_sub_type_of(
    db: &DbIndex,
    sub_type_ref_id: &LuaTypeDeclId,
    super_type_ref_id: &LuaTypeDeclId,
) -> bool {
    check_sub_type_of_iterative(db, sub_type_ref_id, super_type_ref_id).unwrap_or(false)
}

fn check_sub_type_of_iterative(
    db: &DbIndex,
    sub_type_ref_id: &LuaTypeDeclId,
    super_type_ref_id: &LuaTypeDeclId,
) -> Option<bool> {
    if sub_type_ref_id == super_type_ref_id {
        return Some(true);
    }

    let type_index = db.get_type_index();
    let mut stack = Vec::with_capacity(4);
    let mut visited = HashSet::with_capacity(4);

    stack.push(sub_type_ref_id);
    while let Some(current_id) = stack.pop() {
        if !visited.insert(current_id) {
            continue;
        }

        let supers_iter = match type_index.get_super_types_iter(&current_id) {
            Some(iter) => iter,
            None => continue,
        };

        for super_type in supers_iter {
            match super_type {
                LuaType::Ref(super_id) => {
                    // TODO: 不相等时可以判断必要字段是否全部匹配, 如果匹配则认为相等
                    if super_id == super_type_ref_id {
                        return Some(true);
                    }
                    if !visited.contains(super_id) {
                        stack.push(super_id);
                    }
                }
                // TODO: 应该检查泛型参数是否匹配
                LuaType::Generic(generic) => {
                    let base_type_id = generic.get_base_type_id_ref();
                    if base_type_id == super_type_ref_id {
                        return Some(true);
                    }
                    if !visited.contains(&base_type_id) {
                        stack.push(base_type_id);
                    }
                }
                _ => {
                    if let Some(base_id) = get_base_type_id(super_type) {
                        if base_id == *super_type_ref_id {
                            return Some(true);
                        }
                    }
                }
            }
        }
    }

    Some(false)
}

pub fn get_base_type_id(typ: &LuaType) -> Option<LuaTypeDeclId> {
    match typ {
        LuaType::Integer | LuaType::IntegerConst(_) | LuaType::DocIntegerConst(_) => {
            Some(LuaTypeDeclId::new("integer"))
        }
        LuaType::Number | LuaType::FloatConst(_) => Some(LuaTypeDeclId::new("number")),
        LuaType::Boolean | LuaType::BooleanConst(_) | LuaType::DocBooleanConst(_) => {
            Some(LuaTypeDeclId::new("boolean"))
        }
        LuaType::String | LuaType::StringConst(_) | LuaType::DocStringConst(_) => {
            Some(LuaTypeDeclId::new("string"))
        }
        LuaType::Table
        | LuaType::TableGeneric(_)
        | LuaType::TableConst(_)
        | LuaType::Tuple(_)
        | LuaType::Array(_) => Some(LuaTypeDeclId::new("table")),
        LuaType::DocFunction(_) | LuaType::Function | LuaType::Signature(_) => {
            Some(LuaTypeDeclId::new("function"))
        }
        LuaType::Thread => Some(LuaTypeDeclId::new("thread")),
        LuaType::Userdata => Some(LuaTypeDeclId::new("userdata")),
        LuaType::Io => Some(LuaTypeDeclId::new("io")),
        LuaType::Global => Some(LuaTypeDeclId::new("global")),
        LuaType::SelfInfer => Some(LuaTypeDeclId::new("self")),
        LuaType::Nil => Some(LuaTypeDeclId::new("nil")),
        _ => None,
    }
}
