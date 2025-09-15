mod xmake_function;

use crate::LuaIndex;
pub use xmake_function::*;

#[derive(Debug)]
pub struct LuaXmakeIndex {}

impl LuaXmakeIndex {
    pub fn new() -> Self {
        Self {}
    }
}

impl LuaIndex for LuaXmakeIndex {
    fn remove(&mut self, _file_id: crate::FileId) {}

    fn clear(&mut self) {}
}
