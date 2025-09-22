use std::fmt;

/// Error types for executing xmake commands
#[derive(Debug, Clone)]
pub enum XmakeError {
    /// Command execution failed
    ExecutionFailed {
        command: String,
        status_code: i32,
        stdout: String,
        stderr: String,
    },
    /// IO error
    IoError {
        message: String,
    },
    /// xmake not found or unavailable
    XmakeNotFound,
    /// Invalid working directory
    InvalidWorkingDirectory {
        path: String,
    },
    /// Command timed out
    Timeout {
        command: String,
        timeout_secs: u64,
    },
    VersionParseError {
        version_str: String,
    },
    /// Other errors
    Other {
        message: String,
    },
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
            XmakeError::VersionParseError { version_str } => {
                write!(
                    f,
                    "Failed to parse xmake version from string: {}",
                    version_str
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
