/// Command-line interface for SuperTerminal
use clap::{Parser, Subcommand};

/// SuperTerminal - AI-powered CLI that converts natural language to shell commands
#[derive(Parser, Debug)]
#[command(
    name = "superterminal",
    version,
    about = "AI-powered CLI that converts natural language to shell commands",
    long_about = "SuperTerminal allows you to describe what you want to do in natural language, \
                  and it will convert your request into executable shell commands. \
                  The command is displayed for your review before you choose to execute it."
)]
pub struct Cli {
    /// Natural language description of what you want to do
    #[arg(
        value_name = "QUERY",
        help = "Natural language description of the command you want to run"
    )]
    pub query: Option<String>,

    /// Automatically approve and copy the command without confirmation
    #[arg(short, long, help = "Skip confirmation and automatically copy command")]
    pub yes: bool,

    /// Verbose output
    #[arg(short, long, help = "Enable verbose output")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configure SuperTerminal settings
    Config {
        /// Set the OpenAI model to use
        #[arg(long)]
        model: Option<String>,

        /// Set the maximum tokens for responses
        #[arg(long)]
        max_tokens: Option<u32>,

        /// Set the temperature (0.0 to 2.0)
        #[arg(long)]
        temperature: Option<f32>,

        /// Show current configuration
        #[arg(long)]
        show: bool,
    },
}
