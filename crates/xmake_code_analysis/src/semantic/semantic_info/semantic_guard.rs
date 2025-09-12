#[derive(Debug, Copy, Clone)]
pub struct SemanticDeclGuard {
    level: i32,
    limit: i32,
}

impl SemanticDeclGuard {
    pub fn new(limit: i32) -> Self {
        Self { level: 0, limit }
    }

    pub fn next_level(&self) -> Option<Self> {
        if self.level > self.limit {
            return None;
        }

        let new_level = self.level + 1;
        Some(Self {
            level: new_level,
            limit: self.limit,
        })
    }

    pub fn reached_limit(&self) -> bool {
        self.level >= self.limit
    }
}

impl Default for SemanticDeclGuard {
    fn default() -> Self {
        Self::new(50)
    }
}
