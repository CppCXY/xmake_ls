use crate::{XmakeCommand, XmakeCommandType, XmakeError, XmakeOutput, XmakeWrapper};

/// xmake命令构建器
#[derive(Debug, Clone)]
pub struct XmakeCommandBuilder {
    wrapper: XmakeWrapper,
    pub(crate) command: XmakeCommand,
}

impl XmakeCommandBuilder {
    /// 创建新的命令构建器
    pub fn new(wrapper: XmakeWrapper) -> Self {
        Self {
            wrapper,
            command: XmakeCommand::new(XmakeCommandType::Build), // 默认命令
        }
    }

    /// 设置命令类型
    pub fn command_type(mut self, cmd_type: XmakeCommandType) -> Self {
        self.command.command_type = cmd_type;
        self
    }

    /// 添加参数
    pub fn arg<S: Into<String>>(mut self, arg: S) -> Self {
        self.command = self.command.arg(arg);
        self
    }

    /// 添加多个参数
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.command = self.command.args(args);
        self
    }

    /// 添加环境变量
    pub fn env<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.command = self.command.env(key, value);
        self
    }

    /// 设置是否继承父进程环境变量
    pub fn inherit_env(mut self, inherit: bool) -> Self {
        self.command = self.command.inherit_env(inherit);
        self
    }

    /// 执行命令
    pub async fn execute(self) -> Result<XmakeOutput, XmakeError> {
        self.wrapper.execute(&self.command).await
    }

    /// 构建项目
    pub fn build() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Build)
    }

    /// 清理项目
    pub fn clean() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Clean)
    }

    /// 配置项目
    pub fn config() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Config)
    }

    /// 安装项目
    pub fn install() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Install)
    }

    /// 创建新项目
    pub fn create() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Create)
    }

    /// 运行项目
    pub fn run() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Run)
    }

    /// 显示版本信息
    pub fn version() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Version)
    }

    /// 显示帮助信息
    pub fn help() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Help)
    }

    /// 显示项目信息
    pub fn show() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Show)
    }

    /// 包管理命令
    pub fn package() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Package)
    }

    /// 测试项目
    pub fn test() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Test)
    }

    /// 自定义命令
    pub fn custom<S: Into<String>>(cmd: S) -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        let cmd = cmd.into();
        move |wrapper| {
            XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Custom(cmd.clone()))
        }
    }
}

/// 便捷的命令构建函数
impl XmakeWrapper {
    /// 构建项目
    pub fn build(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::build()(self.clone())
    }

    /// 清理项目
    pub fn clean(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::clean()(self.clone())
    }

    /// 配置项目
    pub fn config(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::config()(self.clone())
    }

    /// 安装项目
    pub fn install(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::install()(self.clone())
    }

    /// 创建新项目
    pub fn create(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::create()(self.clone())
    }

    /// 运行项目
    pub fn run(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::run()(self.clone())
    }

    /// 显示版本信息
    pub fn version(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::version()(self.clone())
    }

    /// 显示帮助信息
    pub fn help(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::help()(self.clone())
    }

    /// 显示项目信息
    pub fn show(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::show()(self.clone())
    }

    /// 包管理命令
    pub fn package(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::package()(self.clone())
    }

    /// 测试项目
    pub fn test(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::test()(self.clone())
    }

    /// 自定义命令
    pub fn custom<S: Into<String>>(&self, cmd: S) -> XmakeCommandBuilder {
        XmakeCommandBuilder::custom(cmd)(self.clone())
    }
}
