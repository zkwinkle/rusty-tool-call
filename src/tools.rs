use crate::api_client::api_types::chat_completion::{Function, Tool};
use crate::api_client::api_types::json_schema::JsonSchema;

pub trait CallableTool {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn get_parameters(&self) -> JsonSchema;
    fn call(&self, arguments: serde_json::Value) -> serde_json::Value;
}

impl From<&dyn CallableTool> for Tool {
    fn from(tool: &dyn CallableTool) -> Self {
        Tool {
            r#type: "function".to_owned(),
            function: Function {
                description: Some(tool.description().to_owned()),
                name: tool.name().to_owned(),
                parameters: tool.get_parameters(),
            },
        }
    }
}
