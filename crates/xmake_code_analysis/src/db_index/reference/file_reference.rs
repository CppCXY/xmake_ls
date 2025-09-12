use rowan::TextRange;
use std::collections::HashMap;

use crate::db_index::LuaDeclId;

#[derive(Debug)]
pub struct FileReference {
    decl_references: HashMap<LuaDeclId, DeclReference>,
    references_to_decl: HashMap<TextRange, LuaDeclId>,
}

impl FileReference {
    pub fn new() -> Self {
        Self {
            decl_references: HashMap::new(),
            references_to_decl: HashMap::new(),
        }
    }

    pub fn add_decl_reference(&mut self, decl_id: LuaDeclId, range: TextRange, is_write: bool) {
        if self.references_to_decl.contains_key(&range) {
            return;
        }

        self.references_to_decl.insert(range, decl_id);
        let decl_ref = DeclReferenceCell { range, is_write };

        self.decl_references
            .entry(decl_id)
            .or_insert_with(DeclReference::new)
            .add_cell(decl_ref);
    }

    pub fn get_decl_references(&self, decl_id: &LuaDeclId) -> Option<&DeclReference> {
        self.decl_references.get(decl_id)
    }

    pub fn get_decl_references_map(&self) -> &HashMap<LuaDeclId, DeclReference> {
        &self.decl_references
    }

    pub fn get_decl_id(&self, range: &TextRange) -> Option<LuaDeclId> {
        self.references_to_decl.get(range).copied()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DeclReferenceCell {
    pub range: TextRange,
    pub is_write: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeclReference {
    pub cells: Vec<DeclReferenceCell>,
    pub mutable: bool,
}

impl DeclReference {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            mutable: false,
        }
    }

    pub fn add_cell(&mut self, cell: DeclReferenceCell) {
        if cell.is_write {
            self.mutable = true;
        }

        self.cells.push(cell);
    }
}
