use serde_json::Value;

use crate::context::ServerContextSnapshot;

use super::CommandSpec;

pub struct FixFormatCommand;

impl CommandSpec for FixFormatCommand {
    const COMMAND: &str = "emmy.fix.format";

    #[allow(unused)]
    async fn handle(context: ServerContextSnapshot, args: Vec<Value>) -> Option<()> {
        Some(())
    }
}
