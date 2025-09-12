use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcSemanticToken {
    /// Enable semantic tokens.
    #[serde(default = "default_true")]
    #[schemars(extend("x-vscode-setting" = true))]
    pub enable: bool,

    /// Render Markdown/RST in documentation. Set `doc.syntax` for this option to have effect.
    #[serde(default)]
    #[schemars(extend("x-vscode-setting" = true))]
    pub render_documentation_markup: bool,
}

impl Default for EmmyrcSemanticToken {
    fn default() -> Self {
        Self {
            enable: default_true(),
            render_documentation_markup: true,
        }
    }
}

fn default_true() -> bool {
    true
}
