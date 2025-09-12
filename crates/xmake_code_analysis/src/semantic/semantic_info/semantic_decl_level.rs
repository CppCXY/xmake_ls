#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SemanticDeclLevel {
    NoTrace,
    Trace(usize),
}

impl SemanticDeclLevel {
    pub fn next_level(&self) -> Option<Self> {
        match self {
            SemanticDeclLevel::NoTrace => None,
            SemanticDeclLevel::Trace(level) => {
                if *level == 0 {
                    return None;
                }

                let new_level = level - 1;
                Some(SemanticDeclLevel::Trace(new_level))
            }
        }
    }

    pub fn reached_limit(&self) -> bool {
        match self {
            SemanticDeclLevel::NoTrace => true,
            SemanticDeclLevel::Trace(level) => *level == 0,
        }
    }
}

impl Default for SemanticDeclLevel {
    fn default() -> Self {
        SemanticDeclLevel::Trace(10)
    }
}
