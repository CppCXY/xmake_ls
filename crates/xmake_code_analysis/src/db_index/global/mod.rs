mod global_id;

use std::collections::HashMap;

pub use global_id::GlobalId;

use crate::FileId;

use super::{DbIndex, LuaDeclId, LuaIndex};

#[derive(Debug)]
pub struct LuaGlobalIndex {
    global_decl: HashMap<GlobalId, Vec<LuaDeclId>>,
}

impl LuaGlobalIndex {
    pub fn new() -> Self {
        Self {
            global_decl: HashMap::new(),
        }
    }

    pub fn add_global_decl(&mut self, name: &str, decl_id: LuaDeclId) {
        let id = GlobalId::new(name);
        self.global_decl
            .entry(id)
            .or_insert_with(Vec::new)
            .push(decl_id);
    }

    pub fn get_all_global_decl_ids(&self) -> Vec<LuaDeclId> {
        let mut decls = Vec::new();
        for (_, v) in &self.global_decl {
            decls.extend(v);
        }

        decls
    }

    pub fn get_global_decl_ids(&self, name: &str) -> Option<&Vec<LuaDeclId>> {
        let id = GlobalId::new(name);
        self.global_decl.get(&id)
    }

    pub fn is_exist_global_decl(&self, name: &str) -> bool {
        let id = GlobalId::new(name);
        self.global_decl.contains_key(&id)
    }

    pub fn resolve_global_decl_id(&self, db: &DbIndex, name: &str) -> Option<LuaDeclId> {
        let decl_ids = self.get_global_decl_ids(name)?;
        if decl_ids.len() == 1 {
            return Some(decl_ids[0]);
        }

        let mut last_valid_decl_id = None;
        for decl_id in decl_ids {
            let decl_type_cache = db.get_type_index().get_type_cache(&decl_id.clone().into());
            match decl_type_cache {
                Some(type_cache) => {
                    let typ = type_cache.as_type();
                    if typ.is_def() || typ.is_ref() || typ.is_function() {
                        return Some(*decl_id);
                    }

                    if type_cache.is_table() {
                        last_valid_decl_id = Some(decl_id)
                    }
                }
                None => {}
            }
        }

        if last_valid_decl_id.is_none() && decl_ids.len() > 0 {
            return Some(decl_ids[0]);
        }

        last_valid_decl_id.cloned()
    }
}

impl LuaIndex for LuaGlobalIndex {
    fn remove(&mut self, file_id: FileId) {
        self.global_decl.retain(|_, v| {
            v.retain(|decl_id| decl_id.file_id != file_id);
            !v.is_empty()
        });
    }

    fn clear(&mut self) {
        self.global_decl.clear();
    }
}
