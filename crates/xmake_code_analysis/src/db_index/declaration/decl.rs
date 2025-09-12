use crate::FileId;
use crate::{LuaMemberId, LuaSignatureId};
use emmylua_parser::{LuaKind, LuaSyntaxId, LuaSyntaxKind};
use rowan::{TextRange, TextSize};
use smol_str::SmolStr;

use super::decl_id::LuaDeclId;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct LuaDecl {
    name: SmolStr,
    file_id: FileId,
    range: TextRange,
    expr_id: Option<LuaSyntaxId>,
    pub extra: LuaDeclExtra,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum LuaDeclExtra {
    Local {
        kind: LuaKind,
        attrib: Option<LocalAttribute>,
    },
    Param {
        idx: usize,
        signature_id: LuaSignatureId,
        owner_member_id: Option<LuaMemberId>,
    },
    ImplicitSelf {
        kind: LuaKind,
    },
    Global {
        kind: LuaKind,
    },
}

impl LuaDecl {
    pub fn new(
        name: &str,
        file_id: FileId,
        range: TextRange,
        extra: LuaDeclExtra,
        expr_id: Option<LuaSyntaxId>,
    ) -> Self {
        Self {
            name: SmolStr::new(name),
            file_id,
            range,
            expr_id,
            extra,
        }
    }

    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }

    pub fn get_id(&self) -> LuaDeclId {
        LuaDeclId::new(self.file_id, self.range.start())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_position(&self) -> TextSize {
        self.range.start()
    }

    pub fn get_range(&self) -> TextRange {
        self.range
    }

    pub fn get_syntax_id(&self) -> LuaSyntaxId {
        match self.extra {
            LuaDeclExtra::Local { kind, .. } => LuaSyntaxId::new(kind, self.range),
            LuaDeclExtra::Param { .. } => {
                LuaSyntaxId::new(LuaSyntaxKind::ParamName.into(), self.range)
            }
            LuaDeclExtra::ImplicitSelf { kind } => LuaSyntaxId::new(kind, self.range),
            LuaDeclExtra::Global { kind, .. } => LuaSyntaxId::new(kind, self.range),
        }
    }

    pub fn get_value_syntax_id(&self) -> Option<LuaSyntaxId> {
        self.expr_id
    }

    pub fn is_local(&self) -> bool {
        matches!(
            &self.extra,
            LuaDeclExtra::Local { .. } | LuaDeclExtra::Param { .. }
        )
    }

    pub fn is_param(&self) -> bool {
        matches!(&self.extra, LuaDeclExtra::Param { .. })
    }

    pub fn is_global(&self) -> bool {
        matches!(&self.extra, LuaDeclExtra::Global { .. })
    }

    pub fn is_implicit_self(&self) -> bool {
        matches!(&self.extra, LuaDeclExtra::ImplicitSelf { .. })
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum LocalAttribute {
    Const,
    Close,
    IterConst,
}
