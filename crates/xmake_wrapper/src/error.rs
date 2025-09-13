use std::fmt;

/// xmake命令执行错误类型
#[derive(Debug, Clone)]
pub enum XmakeError {
    /// 命令执行失败
    ExecutionFailed {
        command: String,
        status_code: i32,
        stdout: String,
        stderr: String,
    },
    /// IO错误
    IoError { message: String },
    /// xmake未找到或不可用
    XmakeNotFound,
    /// 无效的工作目录
    InvalidWorkingDirectory { path: String },
    /// 命令超时
    Timeout { command: String, timeout_secs: u64 },
    /// 其他错误
    Other { message: String },
}

impl fmt::Display for XmakeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XmakeError::ExecutionFailed {
                command,
                status_code,
                stdout: _,
                stderr,
            } => {
                write!(
                    f,
                    "Command '{}' failed with status code {}: {}",
                    command, status_code, stderr
                )
            }
            XmakeError::IoError { message } => {
                write!(f, "IO error: {}", message)
            }
            XmakeError::XmakeNotFound => {
                write!(
                    f,
                    "xmake command not found. Please ensure xmake is installed and in PATH"
                )
            }
            XmakeError::InvalidWorkingDirectory { path } => {
                write!(f, "Invalid working directory: {}", path)
            }
            XmakeError::Timeout {
                command,
                timeout_secs,
            } => {
                write!(
                    f,
                    "Command '{}' timed out after {} seconds",
                    command, timeout_secs
                )
            }
            XmakeError::Other { message } => {
                write!(f, "Error: {}", message)
            }
        }
    }
}

impl std::error::Error for XmakeError {}

impl From<std::io::Error> for XmakeError {
    fn from(error: std::io::Error) -> Self {
        XmakeError::IoError {
            message: error.to_string(),
        }
    }
}

impl From<tokio::time::error::Elapsed> for XmakeError {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        XmakeError::Other {
            message: "Operation timed out".to_string(),
        }
    }
}

/// xmake执行结果类型别名
pub type XmakeResult<T> = Result<T, XmakeError>;
