use std::collections::HashMap;

use crate::{DbIndex, LuaMemberKey, LuaType};

use super::{
    LuaMemberInfo,
    find_members::{self},
};

pub fn get_member_map(
    db: &DbIndex,
    prefix_type: &LuaType,
) -> Option<HashMap<LuaMemberKey, Vec<LuaMemberInfo>>> {
    let members = find_members::find_members(db, prefix_type)?;

    let mut member_map = HashMap::new();
    for member in members {
        let key = member.key.clone();
        let typ = &member.typ;
        // 通常是泛型实例化推断结果
        if let LuaType::Union(u) = typ {
            if u.into_vec().iter().all(|f| f.is_function()) {
                for (index, f) in u.into_vec().iter().enumerate() {
                    let new_member = LuaMemberInfo {
                        key: key.clone(),
                        typ: f.clone(),
                        property_owner_id: member.property_owner_id.clone(),
                        feature: member.feature.clone(),
                        overload_index: Some(index),
                    };

                    member_map
                        .entry(key.clone())
                        .or_insert_with(Vec::new)
                        .push(new_member);
                }
                continue;
            }
        }
        member_map.entry(key).or_insert_with(Vec::new).push(member);
    }

    Some(member_map)
}
