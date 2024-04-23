#![doc = include_str!("../README.md")]

use api_client::{
    api_types::chat_completion::{
        ChatCompletion, CreateChatCompletion, Message,
    },
    ApiClient,
};

/// API Client
pub mod api_client;

struct Chat {
    messages: Vec<Message>,
    client: ApiClient,
}

impl Chat {
    fn append_message(&mut self, msg: Message) { self.messages.push(msg); }

    async fn complete_next_message(&self, agent: &Agent) -> ChatCompletion {
        let mut messages = self.messages.clone();

        messages.push(Message::System {
            content: agent.description.clone(),
            name: Some(agent.name.clone()),
        });

        let tools = &[];

        self.client
            .create_chat_completion(CreateChatCompletion {
                model: "gpt-3.5-turbo".to_owned(),
                messages: &messages,
                tools,
            })
            .await
            .unwrap()
    }
}

struct Agent {
    name: String,
    description: String,
}
