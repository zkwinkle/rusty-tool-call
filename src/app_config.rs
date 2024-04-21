use crate::cli::Args;

/// Configuration for the tool.
pub struct AppConfig {
    /// OpenAI API key
    pub openai_api_key: &'static str,
}

/// Create AppConfig to use when the tool's binary.
pub fn create_app_config_from_env_and_args(args: Args) -> AppConfig {
    let openai_api_key = match args.key_override {
        Some(key) => key,
        None => read_env("OPENAI_API_KEY"),
    };
    let openai_api_key: &'static str = Box::new(openai_api_key).leak();
    AppConfig { openai_api_key }
}

/// Read an environment variable, panicking if it's missing.
fn read_env(var_name: &str) -> String {
    std::env::var(var_name).unwrap_or_else(|_| {
        panic!("Missing environment variable: {var_name}");
    })
}
