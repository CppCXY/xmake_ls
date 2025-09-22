use crate::{XmakeCommand, XmakeError, XmakeOutput, XmakeWrapper};
use log::{debug, error, info, warn};
use std::process::Stdio;
use tokio::process::Command;
use tokio::time::{Duration, timeout};

/// Default command timeout in seconds
const DEFAULT_TIMEOUT_SECS: u64 = 300; // 5 minutes

/// Execute an xmake command
pub async fn execute_command(
    wrapper: &XmakeWrapper,
    cmd: &XmakeCommand,
) -> Result<XmakeOutput, XmakeError> {
    execute_command_with_timeout(wrapper, cmd, DEFAULT_TIMEOUT_SECS).await
}

/// Execute a command with timeout
pub async fn execute_command_with_timeout(
    wrapper: &XmakeWrapper,
    cmd: &XmakeCommand,
    timeout_secs: u64,
) -> Result<XmakeOutput, XmakeError> {
    let args = cmd.build_args();
    let command_str = format!("{} {}", wrapper.xmake_env, args.join(" "));

    debug!("Executing xmake command: {}", command_str);

    // Build tokio command
    let mut command = Command::new(&wrapper.xmake_env);
    command
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null());

    // Set working directory
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

    // Set environment variables
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

    // Execute command
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

    // Apply timeout
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
