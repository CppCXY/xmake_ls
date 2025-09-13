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

/// xmake命令包装器的主要结构
#[derive(Debug, Clone)]
#[allow(dead_code)] // 这是一个库模块，某些方法可能在外部使用
pub struct XmakeWrapper {
    /// xmake可执行文件路径
    pub xmake_path: String,
    /// 工作目录
    pub working_dir: Option<PathBuf>,
}

#[allow(dead_code)] // 这是一个库模块，某些方法可能在外部使用
impl XmakeWrapper {
    /// 创建新的xmake包装器实例
    pub fn new() -> Self {
        Self {
            xmake_path: "xmake".to_string(),
            working_dir: None,
        }
    }

    /// 使用自定义xmake路径创建实例
    pub fn with_path<P: AsRef<str>>(xmake_path: P) -> Self {
        Self {
            xmake_path: xmake_path.as_ref().to_string(),
            working_dir: None,
        }
    }

    /// 设置工作目录
    pub fn with_working_dir<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    /// 创建一个新的xmake命令
    pub fn command(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::new(self.clone())
    }

    /// 执行xmake命令
    pub async fn execute(&self, cmd: &XmakeCommand) -> Result<XmakeOutput, XmakeError> {
        execute_command(self, cmd).await
    }

    /// 检查xmake是否可用
    pub async fn check_available(&self) -> bool {
        let result = Command::new(&self.xmake_path)
            .arg("--version")
            .output()
            .await;

        match result {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }
}

impl Default for XmakeWrapper {
    fn default() -> Self {
        Self::new()
    }
}
