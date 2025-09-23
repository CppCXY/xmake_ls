use rowan::TextRange;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XmakeTargetOrPackage {
    pub name: String,
    pub is_target: bool,
    pub range: TextRange,
}
