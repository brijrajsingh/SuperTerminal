mod ai;
mod cli;
mod config;
mod error;

use ai::AIService;
use arboard::Clipboard;
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use config::Config;
use dialoguer::{theme::ColorfulTheme, Confirm};
use error::{Result, SuperTerminalError};
use std::env;

#[tokio::main]
async fn main() {
    // Load .env file if it exists (for development)
    // This will fail silently if .env doesn't exist, which is fine for production
    let _ = dotenvy::dotenv();

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

/// Detect the current shell type
fn detect_shell() -> String {
    // Try to get shell from SHELL environment variable (works on macOS and Linux)
    if let Ok(shell_path) = env::var("SHELL") {
        if let Some(shell_name) = shell_path.split('/').last() {
            let shell = shell_name.to_string();
            // Return the detected shell
            return shell;
        }
    }

    // Fallback: Try to detect from parent process on Unix-like systems
    #[cfg(unix)]
    {
        // On macOS, default to zsh (macOS 10.15+)
        #[cfg(target_os = "macos")]
        return "zsh".to_string();
        
        // On other Unix systems, default to bash
        #[cfg(not(target_os = "macos"))]
        return "bash".to_string();
    }

    // On Windows, check for PowerShell or CMD
    #[cfg(target_os = "windows")]
    {
        if env::var("PSModulePath").is_ok() {
            return "powershell".to_string();
        }
        return "cmd".to_string();
    }
}

async fn handle_query(query: &str, auto_yes: bool, verbose: bool) -> Result<()> {
    // Detect current shell
    let shell = detect_shell();
    
    if verbose {
        println!("{} {}", "Detected shell:".cyan(), shell.bright_cyan());
        println!("{} {}", "Processing query:".cyan(), query);
    }

    // Load configuration
    let config = Config::load_or_default()?;

    // Create AI service
    let ai_service = AIService::new(config)?;

    // Show loading indicator
    println!("{}", "Translating to shell command...".yellow());

    // Translate to command with shell context
    let command = ai_service.translate_to_command(query, &shell).await?;

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
        // Copy to clipboard
        match Clipboard::new() {
            Ok(mut clipboard) => {
                if let Err(e) = clipboard.set_text(&command) {
                    eprintln!("{} {}", "Warning: Failed to copy to clipboard:".yellow(), e);
                    println!("\n{}", "Command ready to use:".green().bold());
                    println!("{}", command);
                } else {
                    println!("\n{}", "✓ Command copied to clipboard!".green().bold());
                    println!("{}", command);
                    println!(
                        "{}",
                        "Paste it anywhere with Ctrl+V (or Cmd+V on Mac)".bright_black()
                    );
                }
            }
            Err(e) => {
                eprintln!("{} {}", "Warning: Could not access clipboard:".yellow(), e);
                println!("\n{}", "Command ready to use:".green().bold());
                println!("{}", command);
                println!(
                    "{}",
                    "You can now copy and paste this command into your terminal.".bright_black()
                );
            }
        }
    } else {
        println!("{}", "Command not copied.".yellow());
    }

    Ok(())
}
