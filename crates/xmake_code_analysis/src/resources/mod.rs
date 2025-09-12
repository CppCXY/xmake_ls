mod best_resource_path;

use std::path::{Path, PathBuf};

use best_resource_path::get_best_resources_dir;
use include_dir::{Dir, DirEntry, include_dir};

use crate::{LuaFileInfo, load_workspace_files};

static RESOURCE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/resources");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn load_resource_std(
    create_resources_dir: Option<String>,
) -> (PathBuf, Vec<LuaFileInfo>) {
    if let Some(create_resources_dir) = create_resources_dir {
        let resource_path = if create_resources_dir.is_empty() {
            get_best_resources_dir()
        } else {
            PathBuf::from(&create_resources_dir)
        };
        let std_dir = PathBuf::from(&resource_path).join("std");
        let result = load_resource_from_file_system(&resource_path);
        match result {
            Some(files) => {
                return (std_dir, files);
            }
            None => {}
        }
    }

    let resoucres_dir = get_best_resources_dir();
    let std_dir = resoucres_dir.join("std");
    let files = load_resource_from_include_dir();
    let files = files
        .into_iter()
        .filter_map(|file| {
            if file.path.ends_with(".lua") {
                let path = resoucres_dir
                    .join(&file.path)
                    .to_str()
                    .expect("UTF-8 paths")
                    .to_string();
                Some(LuaFileInfo {
                    path,
                    content: file.content,
                })
            } else {
                None
            }
        })
        .collect::<_>();
    (std_dir, files)
}

fn load_resource_from_file_system(resources_dir: &Path) -> Option<Vec<LuaFileInfo>> {
    if check_need_dump_to_file_system() {
        log::info!("Creating resources dir: {:?}", resources_dir);
        let files = load_resource_from_include_dir();
        for file in &files {
            let path = resources_dir.join(&file.path);
            let parent = path.parent().expect("resources not top-level dir");
            if !parent.exists() {
                match std::fs::create_dir_all(parent) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("Failed to create dir: {:?}, {:?}", parent, e);
                        return None;
                    }
                }
            }

            match std::fs::write(&path, &file.content) {
                Ok(_) => {}
                Err(e) => {
                    log::error!("Failed to write file: {:?}, {:?}", path, e);
                    return None;
                }
            }
        }

        let version_path = resources_dir.join("version");
        let content = format!("{}", VERSION);
        match std::fs::write(&version_path, content) {
            Ok(_) => {}
            Err(e) => {
                log::error!("Failed to write file: {:?}, {:?}", version_path, e);
                return None;
            }
        }
    }

    let std_dir = resources_dir.join("std");
    let match_pattern = vec!["**/*.lua".to_string()];
    let files = match load_workspace_files(&std_dir, &match_pattern, &Vec::new(), &Vec::new(), None)
    {
        Ok(files) => files,
        Err(e) => {
            log::error!("Failed to load std lib: {:?}", e);
            vec![]
        }
    };

    return Some(files);
}

fn check_need_dump_to_file_system() -> bool {
    if cfg!(debug_assertions) {
        return true;
    }

    let resoucres_dir = get_best_resources_dir();
    let version_path = resoucres_dir.join("version");

    if !version_path.exists() {
        return true;
    }

    let Ok(content) = std::fs::read_to_string(&version_path) else {
        return true;
    };
    let version = content.trim();
    if version != VERSION {
        return true;
    }

    false
}

fn load_resource_from_include_dir() -> Vec<LuaFileInfo> {
    let mut files = Vec::new();
    walk_resource_dir(&RESOURCE_DIR, &mut files);
    files
}

fn walk_resource_dir(dir: &Dir, files: &mut Vec<LuaFileInfo>) {
    for entry in dir.entries() {
        match entry {
            DirEntry::File(file) => {
                let path = file.path();
                let content = file.contents_utf8().expect("UTF-8 paths");

                files.push(LuaFileInfo {
                    path: path.to_str().expect("UTF-8 paths").to_string(),
                    content: content.to_string(),
                });
            }
            DirEntry::Dir(subdir) => {
                walk_resource_dir(subdir, files);
            }
        }
    }
}
