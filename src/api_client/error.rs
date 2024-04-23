#[derive(Debug)]
pub enum ApiClientError {
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
}

impl From<reqwest::Error> for ApiClientError {
    fn from(error: reqwest::Error) -> Self { Self::Reqwest(error) }
}

impl From<serde_json::Error> for ApiClientError {
    fn from(error: serde_json::Error) -> Self { Self::Json(error) }
}
