use crate::{XmakeCommand, XmakeCommandType, XmakeError, XmakeOutput, XmakeWrapper};

/// xmake command builder
#[derive(Debug, Clone)]
pub struct XmakeCommandBuilder {
    wrapper: XmakeWrapper,
    pub(crate) command: XmakeCommand,
}

impl XmakeCommandBuilder {
    /// Create a new command builder
    pub fn new(wrapper: XmakeWrapper) -> Self {
        Self {
            wrapper,
            command: XmakeCommand::new(XmakeCommandType::Build), // default command
        }
    }

    /// Set command type
    pub fn command_type(mut self, cmd_type: XmakeCommandType) -> Self {
        self.command.command_type = cmd_type;
        self
    }

    /// Add an argument
    pub fn arg<S: Into<String>>(mut self, arg: S) -> Self {
        self.command = self.command.arg(arg);
        self
    }

    /// Add multiple arguments
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.command = self.command.args(args);
        self
    }

    /// Add an environment variable
    pub fn env<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.command = self.command.env(key, value);
        self
    }

    /// Set whether to inherit parent environment variables
    pub fn inherit_env(mut self, inherit: bool) -> Self {
        self.command = self.command.inherit_env(inherit);
        self
    }

    /// Execute the command
    pub async fn execute(self) -> Result<XmakeOutput, XmakeError> {
        self.wrapper.execute(&self.command).await
    }

    /// Build project
    pub fn build() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Build)
    }

    /// Clean project
    pub fn clean() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Clean)
    }

    /// Configure project
    pub fn config() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Config)
    }

    /// Install project
    pub fn install() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Install)
    }

    /// Create new project
    pub fn create() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Create)
    }

    /// Run project
    pub fn run() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Run)
    }

    /// Show version info
    pub fn version() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Version)
    }

    /// Show help info
    pub fn help() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Help)
    }

    /// Show project info
    pub fn show() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Show)
    }

    /// Package management command
    pub fn package() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Package)
    }

    /// Test project
    pub fn test() -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        |wrapper| XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Test)
    }

    /// Custom command
    pub fn custom<S: Into<String>>(cmd: S) -> impl Fn(XmakeWrapper) -> XmakeCommandBuilder {
        let cmd = cmd.into();
        move |wrapper| {
            XmakeCommandBuilder::new(wrapper).command_type(XmakeCommandType::Custom(cmd.clone()))
        }
    }
}

/// Convenience command builder helpers
impl XmakeWrapper {
    /// Build project
    pub fn build(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::build()(self.clone())
    }

    /// Clean project
    pub fn clean(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::clean()(self.clone())
    }

    /// Configure project
    pub fn config(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::config()(self.clone())
    }

    /// Install project
    pub fn install(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::install()(self.clone())
    }

    /// Create new project
    pub fn create(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::create()(self.clone())
    }

    /// Run project
    pub fn run(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::run()(self.clone())
    }

    /// Show version info
    pub fn version(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::version()(self.clone())
    }

    /// Show help info
    pub fn help(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::help()(self.clone())
    }

    /// Show project info
    pub fn show(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::show()(self.clone())
    }

    /// Package management command
    pub fn package(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::package()(self.clone())
    }

    /// Test project
    pub fn test(&self) -> XmakeCommandBuilder {
        XmakeCommandBuilder::test()(self.clone())
    }

    /// Custom command
    pub fn custom<S: Into<String>>(&self, cmd: S) -> XmakeCommandBuilder {
        XmakeCommandBuilder::custom(cmd)(self.clone())
    }
}
