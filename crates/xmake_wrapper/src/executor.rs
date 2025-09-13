use crate::{XmakeCommand, XmakeError, XmakeOutput, XmakeWrapper};
use log::{debug, error, info, warn};
use std::process::Stdio;
use tokio::process::Command;
use tokio::time::{Duration, timeout};

/// 默认命令超时时间（秒）
const DEFAULT_TIMEOUT_SECS: u64 = 300; // 5分钟

/// 执行xmake命令
pub async fn execute_command(
    wrapper: &XmakeWrapper,
    cmd: &XmakeCommand,
) -> Result<XmakeOutput, XmakeError> {
    execute_command_with_timeout(wrapper, cmd, DEFAULT_TIMEOUT_SECS).await
}

/// 带超时的命令执行
pub async fn execute_command_with_timeout(
    wrapper: &XmakeWrapper,
    cmd: &XmakeCommand,
    timeout_secs: u64,
) -> Result<XmakeOutput, XmakeError> {
    let args = cmd.build_args();
    let command_str = format!("{} {}", wrapper.xmake_path, args.join(" "));

    debug!("Executing xmake command: {}", command_str);

    // 构建tokio命令
    let mut command = Command::new(&wrapper.xmake_path);
    command
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null());

    // 设置工作目录
    if let Some(working_dir) = &wrapper.working_dir {
        if !working_dir.exists() {
            let error_msg = format!(
                "Working directory does not exist: {}",
                working_dir.display()
            );
            error!("{}", error_msg);
            return Err(XmakeError::InvalidWorkingDirectory {
                path: working_dir.display().to_string(),
            });
        }
        debug!("Setting working directory to: {}", working_dir.display());
        command.current_dir(working_dir);
    }

    // 设置环境变量
    if cmd.inherit_env {
        for (key, value) in &cmd.env_vars {
            debug!("Setting environment variable: {}={}", key, value);
            command.env(key, value);
        }
    } else {
        debug!("Clearing environment variables");
        command.env_clear();
        for (key, value) in &cmd.env_vars {
            debug!("Setting environment variable: {}={}", key, value);
            command.env(key, value);
        }
    }

    // 执行命令
    let command_str_clone = command_str.clone();
    let execution_future = async {
        info!("Starting xmake command execution: {}", command_str_clone);
        let output = command.output().await?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let status_code = output.status.code().unwrap_or(-1);

        let result = XmakeOutput::new(status_code, stdout, stderr);

        if result.is_success() {
            info!(
                "xmake command completed successfully: {}",
                command_str_clone
            );
            debug!("Command output: {}", result.stdout);
        } else {
            warn!(
                "xmake command failed with status {}: {}",
                status_code, command_str_clone
            );
            debug!("Command stdout: {}", result.stdout);
            debug!("Command stderr: {}", result.stderr);
            return Err(XmakeError::ExecutionFailed {
                command: command_str_clone,
                status_code,
                stdout: result.stdout.clone(),
                stderr: result.stderr.clone(),
            });
        }

        Ok(result)
    };

    // 应用超时
    match timeout(Duration::from_secs(timeout_secs), execution_future).await {
        Ok(result) => result,
        Err(_) => {
            error!(
                "xmake command timed out after {} seconds: {}",
                timeout_secs, command_str
            );
            Err(XmakeError::Timeout {
                command: command_str,
                timeout_secs,
            })
        }
    }
}

/// 检查xmake是否可用
pub async fn check_xmake_available(xmake_path: &str) -> bool {
    debug!("Checking if xmake is available at: {}", xmake_path);
    let result = Command::new(xmake_path)
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await;

    match result {
        Ok(output) => {
            let available = output.status.success();
            if available {
                info!("xmake is available at: {}", xmake_path);
            } else {
                warn!("xmake not available at: {}", xmake_path);
            }
            available
        }
        Err(e) => {
            warn!(
                "Failed to check xmake availability at {}: {}",
                xmake_path, e
            );
            false
        }
    }
}

/// 获取xmake版本信息
pub async fn get_xmake_version(xmake_path: &str) -> Result<String, XmakeError> {
    debug!("Getting xmake version from: {}", xmake_path);
    let output = Command::new(xmake_path)
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout).to_string();
        let version_trimmed = version.trim().to_string();
        info!("xmake version: {}", version_trimmed);
        Ok(version_trimmed)
    } else {
        error!("Failed to get xmake version from: {}", xmake_path);
        Err(XmakeError::XmakeNotFound)
    }
}
