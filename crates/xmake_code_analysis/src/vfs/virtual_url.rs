use std::path::PathBuf;

use super::file_path_to_uri;
use lsp_types::Uri;

#[allow(unused)]
#[derive(Debug)]
pub struct VirtualUrlGenerator {
    pub base: PathBuf,
}

#[allow(unused)]
impl VirtualUrlGenerator {
    pub fn new() -> Self {
        let env_path = std::env::current_dir().unwrap();
        VirtualUrlGenerator { base: env_path }
    }

    pub fn new_uri(&self, path: &str) -> Uri {
        let path = self.base.join(path);
        file_path_to_uri(&path).unwrap()
    }

    pub fn new_path(&self, path: &str) -> PathBuf {
        self.base.join(path)
    }
}
