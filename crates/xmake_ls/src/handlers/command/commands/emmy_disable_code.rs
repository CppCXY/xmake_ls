use std::{fs::OpenOptions, io::Write};

use xmake_code_analysis::{DiagnosticCode, FileId, load_configs_raw};
use lsp_types::{Command, Range};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::RwLock;

use crate::context::{ServerContextSnapshot, WorkspaceManager};

use super::CommandSpec;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DisableAction {
    DisableLine,
    DisableFile,
    DisableProject,
}

pub struct DisableCodeCommand;

impl CommandSpec for DisableCodeCommand {
    const COMMAND: &str = "emmy.disable.code";

    async fn handle(context: ServerContextSnapshot, args: Vec<Value>) -> Option<()> {
        let action: DisableAction = serde_json::from_value(args.get(0)?.clone()).ok()?;
        let code: DiagnosticCode = serde_json::from_value(args.get(3)?.clone()).ok()?;

        match action {
            DisableAction::DisableProject => {
                add_disable_project(context.workspace_manager(), code).await;
            }
            _ => {}
        }

        Some(())
    }
}

pub fn make_disable_code_command(
    title: &str,
    action: DisableAction,
    code: DiagnosticCode,
    file_id: FileId,
    range: Range,
) -> Command {
    let args = vec![
        serde_json::to_value(action).unwrap(),
        serde_json::to_value(file_id).unwrap(),
        serde_json::to_value(range).unwrap(),
        serde_json::to_value(code.get_name()).unwrap(),
    ];

    Command {
        title: title.to_string(),
        command: DisableCodeCommand::COMMAND.to_string(),
        arguments: Some(args),
    }
}

async fn add_disable_project(
    workspace_manager: &RwLock<WorkspaceManager>,
    code: DiagnosticCode,
) -> Option<()> {
    let workspace_manager = workspace_manager.read().await;
    let main_workspace = workspace_manager.workspace_folders.get(0)?;
    let emmyrc_path = main_workspace.join(".emmyrc.json");
    let mut emmyrc = load_configs_raw(vec![emmyrc_path.clone()], None);
    drop(workspace_manager);

    emmyrc
        .as_object_mut()?
        .entry("diagnostics")
        .or_insert_with(|| Value::Object(Default::default()))
        .as_object_mut()?
        .entry("disable")
        .or_insert_with(|| Value::Array(Default::default()))
        .as_array_mut()?
        .push(Value::String(code.to_string()));

    let emmyrc_json = serde_json::to_string_pretty(&emmyrc).ok()?;
    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&emmyrc_path)
    {
        if let Err(err) = file.write_all(emmyrc_json.as_bytes()) {
            log::error!("write emmyrc file failed: {:?}", err);
            return None;
        }
    } else {
        log::error!("Failed to open/create emmyrc file: {:?}", emmyrc_path);
        return None;
    }

    Some(())
}
