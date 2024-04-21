use llm_tool_calling::create_app_config_from_env;

#[tokio::main(flavor = "current_thread")]
async fn main() { let config = create_app_config_from_env(); }
