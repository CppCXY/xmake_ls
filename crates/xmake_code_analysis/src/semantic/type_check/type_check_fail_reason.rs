#[derive(Debug)]
pub enum TypeCheckFailReason {
    DonotCheck,
    TypeNotMatch,
    TypeRecursion,
    TypeNotMatchWithReason(String),
}

impl TypeCheckFailReason {
    pub fn is_type_not_match(&self) -> bool {
        matches!(
            self,
            TypeCheckFailReason::TypeNotMatch | TypeCheckFailReason::TypeNotMatchWithReason(_)
        )
    }
}
