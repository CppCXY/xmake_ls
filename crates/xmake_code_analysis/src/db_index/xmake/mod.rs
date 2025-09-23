mod target;
mod xmake_function;

use std::collections::HashMap;

use crate::{FileId, LuaIndex};
pub use target::*;
pub use xmake_function::*;

#[derive(Debug)]
pub struct LuaXmakeIndex {
    includes_file_ids: HashMap<FileId, Vec<FileId>>,
    targets_or_packages: HashMap<FileId, Vec<XmakeTarget>>,
}

impl LuaXmakeIndex {
    pub fn new() -> Self {
        Self {
            includes_file_ids: HashMap::new(),
            targets_or_packages: HashMap::new(),
        }
    }

    pub fn add_includes(&mut self, file_id: FileId, include_file_id: FileId) {
        self.includes_file_ids
            .entry(file_id)
            .or_insert_with(Vec::new)
            .push(include_file_id);
    }

    pub fn get_includes(&self, file_id: FileId) -> Option<&Vec<FileId>> {
        self.includes_file_ids.get(&file_id)
    }

    pub fn add_target_or_package(&mut self, file_id: FileId, target: XmakeTarget) {
        self.targets_or_packages
            .entry(file_id)
            .or_insert_with(Vec::new)
            .push(target);
    }

    pub fn get_targets(&self, file_id: FileId) -> Option<&Vec<XmakeTarget>> {
        self.targets_or_packages.get(&file_id)
    }
}

impl LuaIndex for LuaXmakeIndex {
    fn remove(&mut self, file_id: crate::FileId) {
        self.includes_file_ids.remove(&file_id);
        self.targets_or_packages.remove(&file_id);
    }

    fn clear(&mut self) {
        self.includes_file_ids.clear();
        self.targets_or_packages.clear();
    }
}
