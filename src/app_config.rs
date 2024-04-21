/// Configuration for the tool.
pub struct AppConfig {
    /// OpenAI API key
    pub openai_api_key: &'static str,
}

/// Create AppConfig to use when the tool's binary.
pub fn create_app_config_from_env() -> AppConfig {
    let openai_api_key: &'static str =
        Box::new(read_env("OPENAI_API_KEY")).leak();
    AppConfig { openai_api_key }
}

/// Read an environment variable, panicking if it's missing.
fn read_env(var_name: &str) -> String {
    std::env::var(var_name).unwrap_or_else(|_| {
        panic!("Missing environment variable: {var_name}");
    })
}
