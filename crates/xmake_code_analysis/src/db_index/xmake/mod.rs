mod xmake_function;

use std::collections::HashMap;

use crate::{FileId, LuaIndex};
pub use xmake_function::*;

#[derive(Debug)]
pub struct LuaXmakeIndex {
    includes_file_ids: HashMap<FileId, Vec<FileId>>,
}

impl LuaXmakeIndex {
    pub fn new() -> Self {
        Self {
            includes_file_ids: HashMap::new(),
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
}

impl LuaIndex for LuaXmakeIndex {
    fn remove(&mut self, file_id: crate::FileId) {
        self.includes_file_ids.remove(&file_id);
    }

    fn clear(&mut self) {
        self.includes_file_ids.clear();
    }
}
