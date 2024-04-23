#![doc = include_str!("../README.md")]
// #![deny(missing_docs)]

use api_client::{
    api_types::chat_completion::{
        ChatCompletion, CreateChatCompletion, Message, Tool,
    },
    error::ApiClientError,
    ApiClient,
};
use tools::CallableTool;

/// API Client
pub mod api_client;

/// Tools callable by the AI agents
pub mod tools;

pub struct Chat {
    messages: Vec<Message>,
    client: ApiClient,
}

impl Chat {
    pub fn new(client: ApiClient) -> Self {
        Self {
            messages: Vec::new(),
            client,
        }
    }

    pub fn messages(&self) -> &[Message] { &self.messages }

    pub fn append_message(&mut self, msg: Message) { self.messages.push(msg); }

    pub async fn complete_next_message(
        &self,
        agent: &Agent<'_>,
        tools: &[&dyn CallableTool],
    ) -> Result<ChatCompletion, ApiClientError> {
        let mut messages = self.messages.clone();

        messages.push(Message::System {
            content: agent.description.to_string(),
            name: None,
        });

        let tools = &tools.iter().map(|&t| t.into()).collect::<Box<[Tool]>>();

        self.client
            .create_chat_completion(CreateChatCompletion {
                model: "gpt-3.5-turbo".to_owned(),
                messages: &messages,
                tools,
            })
            .await
    }
}

pub struct Agent<'a> {
    pub name: &'a str,
    pub description: &'a str,
}
