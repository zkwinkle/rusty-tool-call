use crate::api_client::api_types::json_schema::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CreateChatCompletion<'a> {
    pub model: String,
    pub messages: &'a [Message],
    pub tools: &'a [Tool],
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "role")]
pub enum Message {
    System {
        content: String,
        name: Option<String>,
    },
    User {
        content: String,
        name: Option<String>,
    },
    Assistant {
        content: String,
        name: Option<String>,
    },
    Tool {
        content: String,
        tool_call_id: String,
    },
}

#[derive(Serialize)]
pub struct Tool {
    r#type: String,
    function: Function,
}

#[derive(Serialize)]
pub struct Function {
    description: Option<String>,
    name: String,
    parameters: Box<[JsonSchema]>,
}

#[derive(Deserialize)]
pub struct ChatCompletion {
    id: String,
    choices: Box<[ChatCompletionChoice]>,
}

#[derive(Deserialize)]
pub struct ChatCompletionChoice {
    finish_reason: FinishReason,
    index: usize,
    message: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
}
