use std::fmt;

use crate::api_client::api_types::json_schema::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct CreateChatCompletion<'a> {
    pub model: String,
    pub messages: &'a [Message],
    pub tools: &'a [Tool],
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "role")]
pub enum Message {
    System {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    User {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    Assistant {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Box<[ToolCall]>>,
    },
    Tool {
        content: String,
        tool_call_id: String,
    },
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::System { content, name } => {
                write!(
                    f,
                    "System Message{}: {}\n",
                    maybe_print_name(name),
                    content
                )
            }
            Message::User { content, name } => {
                write!(f, "User{}: {}\n", maybe_print_name(name), content)
            }
            Message::Assistant {
                content,
                name,
                tool_calls,
            } => {
                write!(f, "Assistant{}: ", maybe_print_name(name))?;
                match content {
                    Some(content) => write!(f, "{}\n", content),
                    None => Ok(()),
                }?;
                match tool_calls {
                    Some(tool_calls) => {
                        write!(f, "Requesting tool calls: [\n")?;
                        for tool_call in tool_calls.iter() {
                            write!(f, "{},\n", tool_call)?;
                        }
                        write!(f, "]\n")
                    }
                    None => Ok(()),
                }
            }
            Message::Tool {
                content,
                tool_call_id,
            } => {
                write!(f, "Tool Output ({}): {}", tool_call_id, content)
            }
        }
    }
}
fn maybe_print_name(name: &Option<String>) -> String {
    match name {
        Some(name) => format!(" ({})", name),
        None => String::new(),
    }
}

impl Message {
    pub fn content(&self) -> Option<&str> {
        match self {
            Message::System { content, .. } => Some(content),
            Message::User { content, .. } => Some(content),
            Message::Assistant { content, .. } => {
                content.as_ref().map(|s| s.as_str())
            }
            Message::Tool { content, .. } => Some(content),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Tool {
    pub r#type: String,
    pub function: Function,
}

#[derive(Serialize, Debug)]
pub struct Function {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub name: String,
    pub parameters: JsonSchema,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletion {
    pub id: String,
    pub choices: Box<[ChatCompletionChoice]>,
    pub created: usize,
    pub model: String,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionChoice {
    pub finish_reason: FinishReason,
    pub index: usize,
    pub message: ChatCompletionMessage,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionMessage {
    pub content: Option<String>,
    pub tool_calls: Option<Box<[ToolCall]>>,
    pub role: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: ToolCallFunction,
}

impl fmt::Display for ToolCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n\tid: {}\n\tfn: {}({})\n}}",
            self.id, self.function.name, self.function.arguments
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
}
