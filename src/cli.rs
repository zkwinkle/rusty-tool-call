use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Arguments passed to the tool when called
pub struct Args {
    /// override the OPENAI_API_KEY environment variable
    #[arg(short, long, value_name = "OPENAI_API_KEY")]
    pub key_override: Option<String>,

    /// The .gpt file with the #root and #critic instructions
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}
