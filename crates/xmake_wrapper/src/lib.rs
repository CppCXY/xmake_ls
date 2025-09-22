mod builder;
mod command;
mod error;
mod executor;
mod version;

pub use builder::*;
pub use command::*;
pub use error::*;
pub use executor::*;
use log::debug;
pub use version::XmakeVersion;

use std::{path::PathBuf, process::Stdio};
use tokio::process::Command;

use crate::version::strip_ansi_codes;

/// Main struct of the xmake command wrapper
#[derive(Debug, Clone)]
#[allow(dead_code)] // This is a library module; some methods may be used externally
pub struct XmakeWrapper {
    /// Path to the xmake executable
    pub xmake_env: String,
    /// Working directory
    pub working_dir: Option<PathBuf>,
}

#[allow(dead_code)] // This is a library module; some methods may be used externally
impl XmakeWrapper {
    /// Create a new xmake wrapper instance
    pub fn new() -> Self {
        Self {
            xmake_env: "xmake".to_string(),
            working_dir: None,
        }
    }

    /// Create an instance with a custom xmake path
    pub fn with_path<P: AsRef<str>>(xmake_path: P) -> Self {
        Self {
            xmake_env: xmake_path.as_ref().to_string(),
            working_dir: None,
        }
    }

    /// Set the working directory
    pub fn with_working_dir<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    /// Create a new xmake command builder
    pub fn command(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::new(self.clone())
    }

    /// Execute a xmake command
    pub async fn execute(&self, cmd: &XmakeCommand) -> Result<XmakeOutput, XmakeError> {
        execute_command(self, cmd).await
    }

    /// Check whether xmake is available
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
        // First try to resolve the currently configured xmake_env path directly
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

        // Use which/where command to find xmake in PATH
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

    pub async fn get_xmake_version(&self) -> Result<XmakeVersion, XmakeError> {
        debug!("Getting xmake version from: {}", self.xmake_env);
        let output = Command::new(&self.xmake_env)
            .arg("--version")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if output.status.success() {
            let version_raw = String::from_utf8_lossy(&output.stdout).to_string();
            let version_clean = strip_ansi_codes(&version_raw);
            match XmakeVersion::parse(&version_clean) {
                Some(ver) => Ok(ver),
                None => {
                    log::error!(
                        "Failed to parse xmake version from output: {}",
                        version_clean
                    );
                    Err(XmakeError::VersionParseError {
                        version_str: version_clean,
                    })
                }
            }
        } else {
            log::error!("Failed to get xmake version from: {}", self.xmake_env);
            Err(XmakeError::XmakeNotFound)
        }
    }
}

impl Default for XmakeWrapper {
    fn default() -> Self {
        Self::new()
    }
}
