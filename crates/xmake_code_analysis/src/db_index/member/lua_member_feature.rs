#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LuaMemberFeature {
    FileFieldDecl,
    FileDefine,
    FileMethodDecl,
    MetaFieldDecl,
    MetaDefine,
    MetaMethodDecl,
}

impl LuaMemberFeature {
    pub fn is_file_decl(&self) -> bool {
        matches!(
            self,
            LuaMemberFeature::FileFieldDecl | LuaMemberFeature::FileMethodDecl
        )
    }

    pub fn is_meta_decl(&self) -> bool {
        matches!(
            self,
            LuaMemberFeature::MetaFieldDecl
                | LuaMemberFeature::MetaMethodDecl
                | LuaMemberFeature::MetaDefine
        )
    }

    pub fn is_field_decl(&self) -> bool {
        matches!(
            self,
            LuaMemberFeature::FileFieldDecl | LuaMemberFeature::MetaFieldDecl
        )
    }

    pub fn is_decl(&self) -> bool {
        matches!(
            self,
            LuaMemberFeature::FileFieldDecl
                | LuaMemberFeature::FileMethodDecl
                | LuaMemberFeature::MetaFieldDecl
                | LuaMemberFeature::MetaMethodDecl
                | LuaMemberFeature::MetaDefine
        )
    }

    pub fn is_file_define(&self) -> bool {
        matches!(self, LuaMemberFeature::FileDefine)
    }
}
