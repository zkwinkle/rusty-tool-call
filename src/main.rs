#[tokio::main(flavor = "current_thread")]
async fn main() {
    let Ok(openai_api_key) = std::env::var("OPENAI_API_KEY") else {
        panic!("Missing OPENAI_API_KEY environment variable");
    };
}
