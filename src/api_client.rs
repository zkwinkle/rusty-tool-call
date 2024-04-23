use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

use self::{
    api_types::chat_completion::{ChatCompletion, CreateChatCompletion},
    error::ApiClientError,
};

pub mod api_types;
pub mod error;

pub struct ApiClient {
    inner: reqwest::Client,
    access_token: String,
}

const BASE_URL: &str = "https://api.openai.com/v1";

impl ApiClient {
    pub fn new(access_token: String) -> Self {
        Self {
            inner: reqwest::Client::new(),
            access_token,
        }
    }

    async fn post<T, B>(&self, path: &str, body: B) -> Result<T, ApiClientError>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        let response = self
            .inner
            .post(format!("{}{}", BASE_URL, path))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.access_token))
            .body(serde_json::to_value(body)?.to_string())
            .send()
            .await?;

        let bytes = response.bytes().await?;

        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn create_chat_completion(
        &self,
        create: CreateChatCompletion<'_>,
    ) -> Result<ChatCompletion, ApiClientError> {
        let path = "/chat/completions";

        self.post(path, json!(create)).await
    }
}
