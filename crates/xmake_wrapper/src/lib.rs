mod builder;
mod command;
mod error;
mod executor;

pub use builder::*;
pub use command::*;
pub use error::*;
pub use executor::*;

use std::path::PathBuf;
use tokio::process::Command;

/// xmake 命令包装器的主要结构
#[derive(Debug, Clone)]
#[allow(dead_code)] // 这是一个库模块，某些方法可能在外部使用
pub struct XmakeWrapper {
    /// xmake 可执行文件路径
    pub xmake_env: String,
    /// 工作目录
    pub working_dir: Option<PathBuf>,
}

#[allow(dead_code)] // 这是一个库模块，某些方法可能在外部使用
impl XmakeWrapper {
    /// 创建新的 xmake 包装器实例
    pub fn new() -> Self {
        Self {
            xmake_env: "xmake".to_string(),
            working_dir: None,
        }
    }

    /// 使用自定义 xmake 路径创建实例
    pub fn with_path<P: AsRef<str>>(xmake_path: P) -> Self {
        Self {
            xmake_env: xmake_path.as_ref().to_string(),
            working_dir: None,
        }
    }

    /// 设置工作目录
    pub fn with_working_dir<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    /// 创建一个新的 xmake 命令
    pub fn command(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::new(self.clone())
    }

    /// 执行 xmake 命令
    pub async fn execute(&self, cmd: &XmakeCommand) -> Result<XmakeOutput, XmakeError> {
        execute_command(self, cmd).await
    }

    /// 检查 xmake 是否可用
    pub async fn check_available(&self) -> bool {
        let result = Command::new(&self.xmake_env)
            .arg("--version")
            .output()
            .await;

        match result {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    pub async fn get_xmake_path(&self) -> Option<PathBuf> {
        // 首先尝试直接解析当前配置的 xmake_env 路径
        let xmake_path = PathBuf::from(&self.xmake_env);
        if xmake_path.is_absolute() && xmake_path.exists() {
            return Some(xmake_path);
        }

        if let Ok(xmake_home) = std::env::var("XMAKE_HOME") {
            let xmake_exe = PathBuf::from(xmake_home).join("xmake");
            if xmake_exe.exists() {
                return Some(xmake_exe);
            }
        }

        // 使用 which/where 命令在 PATH 中查找 xmake
        let which_cmd = if cfg!(windows) { "where" } else { "which" };

        let result = Command::new(which_cmd).arg(&self.xmake_env).output().await;

        match result {
            Ok(output) if output.status.success() => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let first_line = output_str.lines().next()?;
                let path = PathBuf::from(first_line.trim());
                if path.exists() {
                    return Some(path);
                }
            }
            _ => {}
        }

        None
    }

    pub async fn get_xmake_program_dir(&self) -> Option<PathBuf> {
        if let Ok(xmake_program_dir) = std::env::var("XMAKE_PROGRAM_DIR") {
            let xmake_program_dir = PathBuf::from(xmake_program_dir);
            if xmake_program_dir.exists() {
                return Some(xmake_program_dir);
            }
        }

        None
    }
}

impl Default for XmakeWrapper {
    fn default() -> Self {
        Self::new()
    }
}
