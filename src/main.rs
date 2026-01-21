mod ai;
mod cli;
mod config;
mod error;

use ai::AIService;
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use config::Config;
use dialoguer::{theme::ColorfulTheme, Confirm};
use error::{Result, SuperTerminalError};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    // Handle subcommands
    if let Some(command) = cli.command {
        return handle_command(command).await;
    }

    // Handle natural language query
    if let Some(query) = cli.query {
        return handle_query(&query, cli.yes, cli.verbose).await;
    }

    // If no query or command provided, show help
    Cli::parse_from(["superterminal", "--help"]);
    Ok(())
}

async fn handle_command(command: Commands) -> Result<()> {
    match command {
        Commands::Config {
            model,
            max_tokens,
            temperature,
            show,
        } => {
            let mut config = Config::load_or_default()?;

            if show {
                println!("{}", "Current Configuration:".cyan().bold());
                println!("  Model:         {}", config.model);
                println!("  Max Tokens:    {}", config.max_tokens);
                println!("  Temperature:   {}", config.temperature);
                println!(
                    "  API Key:       {}",
                    if config.api_key.is_empty() {
                        "Not set"
                    } else {
                        "Set"
                    }
                );
                return Ok(());
            }

            let mut updated = false;

            if let Some(m) = model {
                config.model = m;
                updated = true;
            }

            if let Some(mt) = max_tokens {
                config.max_tokens = mt;
                updated = true;
            }

            if let Some(t) = temperature {
                if !(0.0..=2.0).contains(&t) {
                    return Err(SuperTerminalError::InvalidInput(
                        "Temperature must be between 0.0 and 2.0".to_string(),
                    ));
                }
                config.temperature = t;
                updated = true;
            }

            if updated {
                config.save()?;
                println!("{}", "Configuration updated successfully!".green().bold());
            } else {
                println!("{}", "No configuration changes specified.".yellow());
                println!("Use --show to see current configuration.");
            }

            Ok(())
        }
    }
}

async fn handle_query(query: &str, auto_yes: bool, verbose: bool) -> Result<()> {
    if verbose {
        println!("{} {}", "Processing query:".cyan(), query);
    }

    // Load configuration
    let config = Config::load_or_default()?;

    // Create AI service
    let ai_service = AIService::new(config)?;

    // Show loading indicator
    println!("{}", "Translating to shell command...".yellow());

    // Translate to command
    let command = ai_service.translate_to_command(query).await?;

    // Display the generated command
    println!("\n{}", "Generated Command:".green().bold());
    println!("  {}", command.cyan());
    println!();

    // Ask for confirmation unless auto-yes is enabled
    let confirmed = if auto_yes {
        true
    } else {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Copy this command to clipboard?")
            .default(true)
            .interact()
            .map_err(|_| SuperTerminalError::UserCancelled)?
    };

    if confirmed {
        println!("\n{}", "Command ready to use:".green().bold());
        println!("{}", command);
        println!();
        println!(
            "{}",
            "You can now copy and paste this command into your terminal.".bright_black()
        );
    } else {
        println!("{}", "Command not copied.".yellow());
    }

    Ok(())
}
