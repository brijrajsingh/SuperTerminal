![Logo](images/logo.png "SuperTerminal Logo")
---
title: SuperTerminal
description: AI-powered CLI that converts natural language to shell commands
author: Brijraj Singh
ms.date: 2026-01-21
ms.topic: overview
keywords:
  - cli
  - ai
  - shell
  - terminal
  - openai
  - rust
estimated_reading_time: 5
---

# SuperTerminal

I am so lazy and forgetful at times that it's very hard for me to remember all commands upfront. Sometimes its shameful to accept in open coding session that I forgot the basic command. Hence, I decided to make my own SuperTerminal.

SuperTerminal is an AI-powered command-line interface that translates natural language descriptions into executable shell commands. Simply describe what you want to do in plain English, and SuperTerminal will generate the appropriate shell command for you to review and use.

## Features

* **Natural Language to Shell Commands**: Describe what you want in plain English
* **Automatic Clipboard Copy**: Generated commands are automatically copied to your clipboard
* **Interactive Confirmation**: Review generated commands before using them
* **Configurable AI Settings**: Customize model, temperature, and token limits
* **Safe by Design**: Commands are displayed for review, not automatically executed
* **Cross-Platform**: Works on Linux, macOS, and Windows (with WSL)

## Installation

### Prerequisites

* Rust 1.70 or higher
* An OpenAI API key (get one at [platform.openai.com](https://platform.openai.com))

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/superterminal.git
cd superterminal

# Build and install
cargo install --path .
```

## Configuration

### Setting up your API Key

SuperTerminal requires an OpenAI API key. You have two options:

#### Option 1: Environment Variable (Recommended for Production)

Set it as an environment variable:

```bash
export OPENAI_API_KEY='your-api-key-here'
```

For permanent configuration, add this line to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.).

#### Option 2: .env File (Recommended for Development)

Create a `.env` file in the project root:

```bash
echo "OPENAI_API_KEY=your-api-key-here" > .env
```

The `.env` file will be automatically loaded when you run SuperTerminal.

> [!WARNING]
> Never commit your `.env` file to version control. Add it to `.gitignore`.

### Customizing Settings

Configure the AI model and parameters:

```bash
# View current configuration
superterminal config --show

# Set the model (default: gpt-4)
superterminal config --model gpt-4

# Set max tokens (default: 150)
superterminal config --max-tokens 200

# Set temperature (0.0-2.0, default: 0.3)
superterminal config --temperature 0.5
```

Configuration is stored in `~/.config/superterminal/config.json`.

## Usage

### Basic Usage

```bash
superterminal "list all Python files in the current directory"
```

This will:

1. Translate your request to a shell command
2. Display the generated command
3. Ask for your confirmation
4. Copy the command to your clipboard automatically
5. Show the command ready to paste and use

### Skip Confirmation

Use the `-y` flag to automatically approve the command:

```bash
superterminal -y "show disk usage"
```

### Verbose Output

Enable verbose mode to see additional information:

```bash
superterminal -v "find large files"
```

### Example Commands

```bash
# File operations
superterminal "find all files larger than 100MB"
superterminal "count the number of lines in all Python files"
superterminal "list files modified in the last 7 days"

# System information
superterminal "show memory usage"
superterminal "display current network connections"
superterminal "check CPU temperature"

# Text processing
superterminal "search for TODO comments in all source files"
superterminal "find and replace text in all markdown files"
superterminal "sort lines in a file alphabetically"

# Git operations
superterminal "show git branches sorted by last commit date"
superterminal "find the 10 largest files in git history"
superterminal "list commits from the last week"
```

## How It Works

SuperTerminal uses OpenAI's GPT models to translate natural language into shell commands:

1. **Input Processing**: Your natural language query is sent to the AI
2. **Command Generation**: The AI generates an appropriate shell command
3. **Safety Check**: The command is displayed for your review
4. **Clipboard Copy**: Upon confirmation, the command is copied to your clipboard
5. **User Control**: You can paste and execute the command when ready

> [!IMPORTANT]
> SuperTerminal copies commands to your clipboard for easy pasting. It does not automatically execute commands on your system, giving you full control over what runs.

## Architecture

SuperTerminal is built with a modular architecture:

* **CLI Module** ([src/cli/mod.rs](src/cli/mod.rs)): Command-line argument parsing using `clap`
* **AI Service** ([src/ai/mod.rs](src/ai/mod.rs)): OpenAI API integration for command translation
* **Config Module** ([src/config/mod.rs](src/config/mod.rs)): Configuration management
* **Error Handling** ([src/error/mod.rs](src/error/mod.rs)): Custom error types using `thiserror`

## Development

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with cargo
cargo run -- "your command here"
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for compilation errors
cargo check
```

### Testing

SuperTerminal includes comprehensive unit tests:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_extract_command_plain
```

## Security Considerations

* **API Key Safety**: Never commit your API key to version control
* **Command Review**: Always review generated commands before execution
* **Destructive Operations**: The AI is instructed to avoid destructive commands, but always verify
* **Sensitive Data**: Avoid including sensitive information in your queries

## Troubleshooting

### API Key Not Found

```text
Error: API key not found. Please set OPENAI_API_KEY environment variable
```

**Solution**: Set your OpenAI API key as an environment variable:

```bash
export OPENAI_API_KEY='your-api-key-here'
```

### Configuration Error

```text
Error: Configuration error: Could not find config directory
```

**Solution**: Ensure your home directory is accessible and you have write permissions.

### Network Issues

```text
Error: OpenAI API error: connection refused
```

**Solution**: Check your internet connection and verify that the OpenAI API is accessible.

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

* Built with [Rust](https://www.rust-lang.org/)
* CLI parsing with [clap](https://github.com/clap-rs/clap)
* OpenAI integration with [async-openai](https://github.com/64bit/async-openai)
* Inspired by the need for more intuitive command-line interfaces

## Roadmap

* [ ] Shell integration for direct command execution
* [ ] Command history and favorites
* [ ] Support for multiple AI providers
* [ ] Support for desktop LLM model
* [ ] Interactive command builder
* [ ] Syntax highlighting for generated commands
* [ ] Command explanation mode
* [ ] Offline command suggestions

## Contact

For questions, issues, or suggestions:

* Open an issue on [GitHub](https://github.com/yourusername/superterminal/issues)
* Discussions on [GitHub Discussions](https://github.com/yourusername/superterminal/discussions)
