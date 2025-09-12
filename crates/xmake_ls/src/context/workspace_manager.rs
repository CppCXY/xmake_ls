use std::collections::HashSet;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::{path::PathBuf, sync::Arc, time::Duration};

use super::{ClientProxy, FileDiagnostic, StatusBar};
use crate::handlers::{ClientConfig, init_analysis};
use dirs;
use log::info;
use lsp_types::Uri;
use tokio::sync::{Mutex, RwLock};
use tokio_util::sync::CancellationToken;
use wax::Pattern;
use xmake_code_analysis::uri_to_file_path;
use xmake_code_analysis::{XmakeAnalysis, Emmyrc, load_configs};

pub struct WorkspaceManager {
    analysis: Arc<RwLock<XmakeAnalysis>>,
    #[allow(unused)]
    client: Arc<ClientProxy>,
    status_bar: Arc<StatusBar>,
    update_token: Arc<Mutex<Option<Arc<ReindexToken>>>>,
    file_diagnostic: Arc<FileDiagnostic>,
    pub client_config: ClientConfig,
    pub workspace_folders: Vec<PathBuf>,
    pub watcher: Option<notify::RecommendedWatcher>,
    pub current_open_files: HashSet<Uri>,
    pub match_file_pattern: WorkspaceFileMatcher,
    // 原子变量
    pub workspace_initialized: Arc<AtomicBool>,
}

impl WorkspaceManager {
    pub fn new(
        analysis: Arc<RwLock<XmakeAnalysis>>,
        client: Arc<ClientProxy>,
        status_bar: Arc<StatusBar>,
        file_diagnostic: Arc<FileDiagnostic>,
    ) -> Self {
        Self {
            analysis,
            client,
            status_bar,
            client_config: ClientConfig::default(),
            workspace_folders: Vec::new(),
            update_token: Arc::new(Mutex::new(None)),
            file_diagnostic,
            watcher: None,
            current_open_files: HashSet::new(),
            match_file_pattern: WorkspaceFileMatcher::default(),
            workspace_initialized: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn is_workspace_initialized(&self) -> bool {
        self.workspace_initialized
            .load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn set_workspace_initialized(&self) {
        self.workspace_initialized
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub async fn reload_workspace(&self) -> Option<()> {
        let config_root: Option<PathBuf> = match self.workspace_folders.first() {
            Some(root) => Some(PathBuf::from(root)),
            None => None,
        };

        let emmyrc = load_emmy_config(config_root, self.client_config.clone());
        let analysis = self.analysis.clone();
        let workspace_folders = self.workspace_folders.clone();
        let status_bar = self.status_bar.clone();
        let file_diagnostic = self.file_diagnostic.clone();
        init_analysis(
            &analysis,
            &status_bar,
            &file_diagnostic,
            workspace_folders,
            emmyrc,
        )
        .await;

        Some(())
    }

    pub async fn extend_reindex_delay(&self) -> Option<()> {
        let update_token = self.update_token.lock().await;
        if let Some(token) = update_token.as_ref() {
            token.set_resleep().await;
        }

        Some(())
    }

    pub async fn reindex_workspace(&self, delay: Duration) -> Option<()> {
        log::info!("reindex workspace with delay: {:?}", delay);
        let mut update_token = self.update_token.lock().await;
        if let Some(token) = update_token.as_ref() {
            token.cancel();
            log::info!("cancel reindex workspace");
        }

        let cancel_token = Arc::new(ReindexToken::new(delay));
        update_token.replace(cancel_token.clone());
        drop(update_token);
        let analysis = self.analysis.clone();
        let file_diagnostic = self.file_diagnostic.clone();
        tokio::spawn(async move {
            cancel_token.wait_for_reindex().await;
            if cancel_token.is_cancelled() {
                return;
            }

            let mut analysis = analysis.write().await;

            // 在重新索引之前清理不存在的文件
            analysis.cleanup_nonexistent_files();

            analysis.reindex();
            file_diagnostic
                .add_workspace_diagnostic_task(500, true)
                .await;
        });

        Some(())
    }

    pub fn is_workspace_file(&self, uri: &Uri) -> bool {
        if self.workspace_folders.is_empty() {
            return true;
        }

        let Some(file_path) = uri_to_file_path(&uri) else {
            return true;
        };

        let mut file_matched = true;
        for workspace in &self.workspace_folders {
            if let Ok(relative) = file_path.strip_prefix(workspace) {
                file_matched = self.match_file_pattern.is_match(&file_path, relative);

                if file_matched {
                    // If the file matches the include pattern, we can stop checking further.
                    break;
                }
            }
        }

        file_matched
    }
}

pub fn load_emmy_config(config_root: Option<PathBuf>, client_config: ClientConfig) -> Arc<Emmyrc> {
    // Config load priority.
    // * Global `<os-specific home-dir>/.luarc.json`.
    // * Global `<os-specific home-dir>/.emmyrc.json`.
    // * Global `<os-specific config-dir>/emmylua_ls/.luarc.json`.
    // * Global `<os-specific config-dir>/emmylua_ls/.emmyrc.json`.
    // * Environment-specified config at the $EMMYLUALS_CONFIG path.
    // * Local `.luarc.json`.
    // * Local `.emmyrc.json`.
    let luarc_file = ".luarc.json";
    let emmyrc_file = ".emmyrc.json";
    let mut config_files = Vec::new();

    let home_dir = dirs::home_dir();
    match home_dir {
        Some(home_dir) => {
            let global_luarc_path = home_dir.join(luarc_file);
            if global_luarc_path.exists() {
                info!("load config from: {:?}", global_luarc_path);
                config_files.push(global_luarc_path);
            }
            let global_emmyrc_path = home_dir.join(emmyrc_file);
            if global_emmyrc_path.exists() {
                info!("load config from: {:?}", global_emmyrc_path);
                config_files.push(global_emmyrc_path);
            }
        }
        None => {}
    };

    let emmylua_config_dir = "emmylua_ls";
    let config_dir = dirs::config_dir().map(|path| path.join(emmylua_config_dir));
    match config_dir {
        Some(config_dir) => {
            let global_luarc_path = config_dir.join(luarc_file);
            if global_luarc_path.exists() {
                info!("load config from: {:?}", global_luarc_path);
                config_files.push(global_luarc_path);
            }
            let global_emmyrc_path = config_dir.join(emmyrc_file);
            if global_emmyrc_path.exists() {
                info!("load config from: {:?}", global_emmyrc_path);
                config_files.push(global_emmyrc_path);
            }
        }
        None => {}
    };

    std::env::var("EMMYLUALS_CONFIG")
        .inspect(|path| {
            let config_path = std::path::PathBuf::from(path);
            if config_path.exists() {
                info!("load config from: {:?}", config_path);
                config_files.push(config_path);
            }
        })
        .ok();

    if let Some(config_root) = &config_root {
        let luarc_path = config_root.join(luarc_file);
        if luarc_path.exists() {
            info!("load config from: {:?}", luarc_path);
            config_files.push(luarc_path);
        }
        let emmyrc_path = config_root.join(emmyrc_file);
        if emmyrc_path.exists() {
            info!("load config from: {:?}", emmyrc_path);
            config_files.push(emmyrc_path);
        }
    }

    let mut emmyrc = load_configs(config_files, client_config.partial_emmyrcs.clone());
    merge_client_config(client_config, &mut emmyrc);
    if let Some(workspace_root) = &config_root {
        emmyrc.pre_process_emmyrc(workspace_root);
    }

    log::info!("loaded emmyrc complete");
    emmyrc.into()
}

fn merge_client_config(client_config: ClientConfig, emmyrc: &mut Emmyrc) -> Option<()> {
    emmyrc.runtime.extensions.extend(client_config.extensions);
    emmyrc.workspace.ignore_globs.extend(client_config.exclude);
    if client_config.encoding != "utf-8" {
        emmyrc.workspace.encoding = client_config.encoding;
    }

    Some(())
}

#[derive(Debug)]
pub struct ReindexToken {
    cancel_token: CancellationToken,
    time_sleep: Duration,
    need_re_sleep: Mutex<bool>,
}

impl ReindexToken {
    pub fn new(time_sleep: Duration) -> Self {
        Self {
            cancel_token: CancellationToken::new(),
            time_sleep,
            need_re_sleep: Mutex::new(false),
        }
    }

    pub async fn wait_for_reindex(&self) {
        loop {
            tokio::select! {
                _ = tokio::time::sleep(self.time_sleep) => {
                    // 获取锁来安全地访问和修改 need_re_sleep
                    let mut need_re_sleep = self.need_re_sleep.lock().await;
                    if *need_re_sleep {
                        *need_re_sleep = false;
                    } else {
                        break;
                    }
                }
                _ = self.cancel_token.cancelled() => {
                    break;
                }
            }
        }
    }

    pub fn cancel(&self) {
        self.cancel_token.cancel();
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancel_token.is_cancelled()
    }

    pub async fn set_resleep(&self) {
        // 获取锁来安全地修改 need_re_sleep
        let mut need_re_sleep = self.need_re_sleep.lock().await;
        *need_re_sleep = true;
    }
}

#[derive(Debug, Clone)]
pub struct WorkspaceFileMatcher {
    include: Vec<String>,
    exclude: Vec<String>,
    exclude_dir: Vec<PathBuf>,
}

impl WorkspaceFileMatcher {
    pub fn new(include: Vec<String>, exclude: Vec<String>, exclude_dir: Vec<PathBuf>) -> Self {
        Self {
            include,
            exclude,
            exclude_dir,
        }
    }
    pub fn is_match(&self, path: &Path, relative_path: &Path) -> bool {
        if self.exclude_dir.iter().any(|dir| path.starts_with(dir)) {
            return false;
        }

        // let path_str = path.to_string_lossy().to_string().replace("\\", "/");
        let exclude_matcher = wax::any(self.exclude.iter().map(|s| s.as_str()));
        if let Ok(exclude_set) = exclude_matcher {
            if exclude_set.is_match(relative_path) {
                return false;
            }
        } else {
            log::error!("Invalid exclude pattern");
        }

        let include_matcher = wax::any(self.include.iter().map(|s| s.as_str()));
        if let Ok(include_set) = include_matcher {
            return include_set.is_match(relative_path);
        } else {
            log::error!("Invalid include pattern");
        }

        return true;
    }
}

impl Default for WorkspaceFileMatcher {
    fn default() -> Self {
        let include_pattern = vec!["**/*.lua".to_string()];
        Self::new(include_pattern, vec![], vec![])
    }
}
