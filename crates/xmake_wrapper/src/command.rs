use std::collections::HashMap;

/// Enum of xmake command types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XmakeCommandType {
    /// Build project
    Build,
    /// Clean project
    Clean,
    /// Configure project
    Config,
    /// Install project
    Install,
    /// Create new project
    Create,
    /// Run project
    Run,
    /// Show version info
    Version,
    /// Show help info
    Help,
    /// Show project info
    Show,
    /// Package management command
    Package,
    /// Test project
    Test,
    /// Custom command
    Custom(String),
}

impl XmakeCommandType {
    /// Get the command string
    pub fn as_str(&self) -> &str {
        match self {
            XmakeCommandType::Build => "build",
            XmakeCommandType::Clean => "clean",
            XmakeCommandType::Config => "config",
            XmakeCommandType::Install => "install",
            XmakeCommandType::Create => "create",
            XmakeCommandType::Run => "run",
            XmakeCommandType::Version => "--version",
            XmakeCommandType::Help => "--help",
            XmakeCommandType::Show => "show",
            XmakeCommandType::Package => "require",
            XmakeCommandType::Test => "test",
            XmakeCommandType::Custom(cmd) => cmd,
        }
    }
}

/// xmake command structure
#[derive(Debug, Clone)]
pub struct XmakeCommand {
    /// Command type
    pub command_type: XmakeCommandType,
    /// Command arguments
    pub args: Vec<String>,
    /// Environment variables
    pub env_vars: HashMap<String, String>,
    /// Whether to inherit parent process environment
    pub inherit_env: bool,
}

impl XmakeCommand {
    /// Create a new xmake command
    pub fn new(command_type: XmakeCommandType) -> Self {
        Self {
            command_type,
            args: Vec::new(),
            env_vars: HashMap::new(),
            inherit_env: true,
        }
    }

    /// Add an argument
    pub fn arg<S: Into<String>>(mut self, arg: S) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Add multiple arguments
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for arg in args {
            self.args.push(arg.into());
        }
        self
    }

    /// Add an environment variable
    pub fn env<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.env_vars.insert(key.into(), value.into());
        self
    }

    /// Set whether to inherit parent environment variables
    pub fn inherit_env(mut self, inherit: bool) -> Self {
        self.inherit_env = inherit;
        self
    }

    /// Build the full command argument list
    pub fn build_args(&self) -> Vec<String> {
        let mut args = vec![self.command_type.as_str().to_string()];
        args.extend(self.args.clone());
        args
    }
}

/// Command output result
#[derive(Debug, Clone)]
pub struct XmakeOutput {
    /// Exit status code
    pub status_code: i32,
    /// Standard output
    pub stdout: String,
    /// Standard error output
    pub stderr: String,
    /// Whether the command executed successfully
    pub success: bool,
}

impl XmakeOutput {
    /// Create a new output result
    pub fn new(status_code: i32, stdout: String, stderr: String) -> Self {
        Self {
            status_code,
            stdout,
            stderr,
            success: status_code == 0,
        }
    }

    /// Check if the command succeeded
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Get the output text, prefer stdout; if empty, return stderr
    pub fn output_text(&self) -> &str {
        if !self.stdout.is_empty() {
            &self.stdout
        } else {
            &self.stderr
        }
    }
}
