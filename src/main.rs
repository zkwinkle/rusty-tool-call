use clap::Parser;
use llm_tool_calling::app_config::create_app_config_from_env_and_args;
use llm_tool_calling::cli::Args;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();
    let config = create_app_config_from_env_and_args(args);
}
