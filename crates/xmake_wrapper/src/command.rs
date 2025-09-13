use std::collections::HashMap;

/// xmake命令类型枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XmakeCommandType {
    /// 构建项目
    Build,
    /// 清理项目
    Clean,
    /// 配置项目
    Config,
    /// 安装项目
    Install,
    /// 创建新项目
    Create,
    /// 运行项目
    Run,
    /// 显示版本信息
    Version,
    /// 显示帮助信息
    Help,
    /// 显示项目信息
    Show,
    /// 包管理相关命令
    Package,
    /// 测试项目
    Test,
    /// 自定义命令
    Custom(String),
}

impl XmakeCommandType {
    /// 获取命令字符串
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

/// xmake命令结构
#[derive(Debug, Clone)]
pub struct XmakeCommand {
    /// 命令类型
    pub command_type: XmakeCommandType,
    /// 命令参数
    pub args: Vec<String>,
    /// 环境变量
    pub env_vars: HashMap<String, String>,
    /// 是否继承父进程环境变量
    pub inherit_env: bool,
}

impl XmakeCommand {
    /// 创建新的xmake命令
    pub fn new(command_type: XmakeCommandType) -> Self {
        Self {
            command_type,
            args: Vec::new(),
            env_vars: HashMap::new(),
            inherit_env: true,
        }
    }

    /// 添加参数
    pub fn arg<S: Into<String>>(mut self, arg: S) -> Self {
        self.args.push(arg.into());
        self
    }

    /// 添加多个参数
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

    /// 添加环境变量
    pub fn env<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.env_vars.insert(key.into(), value.into());
        self
    }

    /// 设置是否继承父进程环境变量
    pub fn inherit_env(mut self, inherit: bool) -> Self {
        self.inherit_env = inherit;
        self
    }

    /// 构建完整的命令参数列表
    pub fn build_args(&self) -> Vec<String> {
        let mut args = vec![self.command_type.as_str().to_string()];
        args.extend(self.args.clone());
        args
    }
}

/// 命令输出结果
#[derive(Debug, Clone)]
pub struct XmakeOutput {
    /// 退出状态码
    pub status_code: i32,
    /// 标准输出
    pub stdout: String,
    /// 标准错误输出
    pub stderr: String,
    /// 命令是否成功执行
    pub success: bool,
}

impl XmakeOutput {
    /// 创建新的输出结果
    pub fn new(status_code: i32, stdout: String, stderr: String) -> Self {
        Self {
            status_code,
            stdout,
            stderr,
            success: status_code == 0,
        }
    }

    /// 检查命令是否成功
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// 获取输出文本，优先返回stdout，如果为空则返回stderr
    pub fn output_text(&self) -> &str {
        if !self.stdout.is_empty() {
            &self.stdout
        } else {
            &self.stderr
        }
    }
}
