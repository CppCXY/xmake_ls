use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcHover {
    /// Enable showing documentation on hover.
    #[serde(default = "default_true")]
    #[schemars(extend("x-vscode-setting" = true))]
    pub enable: bool,

    /// The detail number of hover information.
    /// Default is `None`, which means using the default detail level.
    /// You can set it to a number between `1` and `255` to customize
    #[serde(default)]
    pub custom_detail: Option<u8>,
}

impl Default for EmmyrcHover {
    fn default() -> Self {
        Self {
            enable: default_true(),
            custom_detail: None,
        }
    }
}

fn default_true() -> bool {
    true
}
