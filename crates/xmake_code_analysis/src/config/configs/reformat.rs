use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcReformat {
    /// Whether to enable external tool formatting.
    #[serde(default)]
    pub external_tool: Option<EmmyrcExternalTool>,

    /// Whether to enable external tool range formatting.
    #[serde(default)]
    pub external_tool_range_format: Option<EmmyrcExternalTool>,

    /// Whether to use the diff algorithm for formatting.
    #[serde(default = "default_false")]
    pub use_diff: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmmyrcExternalTool {
    /// The command to run the external tool.
    #[serde(default)]
    pub program: String,
    /// The arguments to pass to the external tool.
    #[serde(default)]
    pub args: Vec<String>,
    /// The timeout for the external tool in milliseconds.
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

fn default_timeout() -> u64 {
    5000
}

fn default_false() -> bool {
    false
}
