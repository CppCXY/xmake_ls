mod global_id;

use std::collections::HashMap;

pub use global_id::GlobalId;
use rowan::TextSize;

use crate::{FileId, LuaSemanticDeclId, LuaType, XmakeScope};

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

    pub fn resolve_global_decl_id(
        &self,
        db: &DbIndex,
        name: &str,
        file_id: FileId,
        position: TextSize,
    ) -> Option<LuaDeclId> {
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
                    if typ.is_def() || typ.is_ref() {
                        return Some(*decl_id);
                    }

                    if let LuaType::Signature(signature_id) = typ {
                        let semantic_id = LuaSemanticDeclId::Signature(*signature_id);
                        if filter_global_by_scope(db, semantic_id, file_id, position).is_some() {
                            continue;
                        }
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

pub fn filter_global_by_scope(
    db: &DbIndex,
    semantic_id: LuaSemanticDeclId,
    file_id: FileId,
    position: TextSize,
) -> Option<()> {
    let property = db.get_property_index().get_property(&semantic_id)?;
    let xmake_scope = property.scope.clone()?;
    let xmake_targets = db.get_xmake_index().get_targets(file_id)?;
    for xmake_target in xmake_targets {
        if xmake_target.range.contains(position) {
            match (xmake_scope, xmake_target.kind) {
                (XmakeScope::Package, x) if !x.is_package() => return Some(()),
                (XmakeScope::Option, x) if !x.is_option() => return Some(()),
                (XmakeScope::Rule, x) if !x.is_rule() => return Some(()),
                (XmakeScope::Target, x) if !x.is_target() => return Some(()),
                (XmakeScope::Task, x) if !x.is_task() => return Some(()),
                _ => {}
            }
        }
    }

    None
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
