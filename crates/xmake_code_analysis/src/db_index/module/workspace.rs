use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub struct Workspace {
    pub root: PathBuf,
    pub id: WorkspaceId,
}

impl Workspace {
    pub fn new(root: PathBuf, id: WorkspaceId) -> Self {
        Self { root, id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord)]
pub struct WorkspaceId {
    pub id: u32,
}

#[allow(unused)]
impl WorkspaceId {
    pub const STD: WorkspaceId = WorkspaceId { id: 0 };
    pub const MAIN: WorkspaceId = WorkspaceId { id: 1 };

    pub fn is_library(&self) -> bool {
        self.id > 1
    }

    pub fn is_main(&self) -> bool {
        self.id == 1
    }

    pub fn is_std(&self) -> bool {
        self.id == 0
    }
}

impl PartialOrd for WorkspaceId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.id.cmp(&other.id) {
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            std::cmp::Ordering::Equal => Some(std::cmp::Ordering::Equal),
        }
    }
}

impl fmt::Display for WorkspaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.id {
            0 => write!(f, "std"),
            1 => write!(f, "main"),
            _ => write!(f, "lib{}", self.id - 1),
        }
    }
}
