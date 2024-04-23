use std::{
    collections::HashMap,
    io::{self},
};

use llm_tool_calling::{
    api_client::{
        api_types::{
            chat_completion::{ChatCompletionChoice, FinishReason, Message},
            json_schema::JsonSchema,
        },
        ApiClient,
    },
    tools::CallableTool,
    Agent, Chat,
};
use serde_json::Value;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let Ok(openai_api_key) = std::env::var("OPENAI_API_KEY") else {
        panic!("Missing OPENAI_API_KEY environment variable");
    };

    let api_client = ApiClient::new(openai_api_key);

    let mut chat = Chat::new(api_client);

    for _ in 0..3 {
        for agent in AGENTS {
            loop {
                let completion = &chat
                    .complete_next_message(&agent, &[&CountWords])
                    .await
                    .unwrap()
                    .choices[0];

                handle_completion(&mut chat, completion, &agent);

                println!("-----------------------------------");

                for message in chat.messages() {
                    println!("=> {}\n", message);
                }

                println!("Press [Enter] to continue...");
                let _ = io::stdin().read_line(&mut String::new());

                if matches!(completion.finish_reason, FinishReason::Stop) {
                    break;
                }
            }
        }

        if chat
            .messages()
            .last()
            .unwrap()
            .content()
            .unwrap()
            .find("I'm happy")
            .is_some()
        {
            return;
        };
    }
}

fn handle_completion(
    chat: &mut Chat,
    completion: &ChatCompletionChoice,
    agent: &Agent,
) {
    match completion.finish_reason {
        FinishReason::Stop => chat.append_message(Message::Assistant {
            content: Some(
                completion.message.content.as_ref().unwrap().to_string(),
            ),
            name: Some(agent.name.to_owned()),
            tool_calls: None,
        }),
        FinishReason::ToolCalls => {
            let tool_calls = &completion.message.tool_calls.clone().unwrap();

            chat.append_message(Message::Assistant {
                content: None,
                name: Some(agent.name.to_owned()),
                tool_calls: Some(tool_calls.clone()),
            });

            for tool_call in tool_calls.iter() {
                let tool =
                    TOOLS.iter().find(|t| t.name() == tool_call.function.name);

                let result = tool.unwrap().call(
                    serde_json::from_str(&tool_call.function.arguments)
                        .unwrap(),
                );

                chat.append_message(Message::Tool {
                    content: result.to_string(),
                    tool_call_id: tool_call.id.clone(),
                })
            }
        }
        _ => todo!("Unhandled finish reason"),
    }
}

// Our agents
const AGENTS: [Agent; 2] = [WRITER_AGENT, CRITIC_AGENT];

const CRITIC_AGENT: Agent  = Agent{
    name: "critic",
    description: "provide feedback for copy.
your job is to evaluate the blog posts. Your analysis should be thorough, constructive, and insightful, offering both praise for strengths and suggestions for improvement. Limit feedback to 150 words. Try to be brief and concise. Always return your feedback back to the root. When happy with the result reply with I am happy no more revision needed.",
};

const WRITER_AGENT: Agent = Agent {
    name: "root",
    description: "You're a writer. Your job is to create compelling, insightful, and engaging blog posts that will engage our audience. Limit post to 150 words.
Add title. Add word count [word_count] and [revision #]. Then call critic for review. Stop when critic happy or no more than 3 revision rounds."
};

// Our tools
const TOOLS: [&dyn CallableTool; 1] = [&CountWords];

pub struct CountWords;

impl CallableTool for CountWords {
    fn name(&self) -> &'static str { "count_words" }
    fn description(&self) -> &'static str {
        "Returns the word count, aka the amount of words, in a string."
    }
    fn get_parameters(&self) -> JsonSchema {
        let mut properties = HashMap::new();

        properties.insert("text".to_owned(), JsonSchema::String);

        JsonSchema::Object {
            required: properties.keys().map(|s| (*s).to_owned()).collect(),
            properties,
        }
    }
    fn call(&self, arguments: Value) -> Value {
        let Value::String(text) = &arguments["text"] else {
            panic!("Missing \"text\" argument when calling count_words")
        };

        text.split_whitespace().count().into()
    }
}
