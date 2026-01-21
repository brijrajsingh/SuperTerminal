---
title: SuperTerminal Quick Start
description: Get started with SuperTerminal in 5 minutes
ms.date: 2026-01-21
ms.topic: quickstart
keywords:
  - quickstart
  - getting started
  - installation
estimated_reading_time: 5
---

# SuperTerminal Quick Start

Get up and running with SuperTerminal in just a few minutes!

## Quick Install

```bash
# Clone and build
git clone https://github.com/yourusername/superterminal.git
cd superterminal
cargo install --path .

# Set your API key
export OPENAI_API_KEY='your-api-key-here'
```

## First Command

Try your first natural language command:

```bash
superterminal "list all files in the current directory"
```

You'll see:

1. The AI translating your request
2. The generated command displayed
3. A prompt asking if you want to use it

## Common Examples

```bash
# Find files
superterminal "find all Python files"

# System info
superterminal "show disk usage"

# Text processing
superterminal "count lines in all text files"

# Git operations
superterminal "show recent commits"
```

## Configuration

Customize the AI settings:

```bash
# View current config
superterminal config --show

# Change model
superterminal config --model gpt-4-turbo

# Adjust creativity (0.0 - 2.0)
superterminal config --temperature 0.5
```

## Tips

* **Be specific**: "find large files" → "find files larger than 100MB"
* **Use `-y` flag**: Skip confirmation for known-safe commands
* **Review commands**: Always verify before executing destructive operations

## Next Steps

* Read the full [README](README.md) for all features
* Check [EXAMPLES.md](EXAMPLES.md) for more use cases
* See [CONTRIBUTING.md](CONTRIBUTING.md) to contribute

## Troubleshooting

### API Key Error

```bash
# Make sure your key is set
echo $OPENAI_API_KEY

# Add to your shell profile for persistence
echo 'export OPENAI_API_KEY="your-key"' >> ~/.bashrc
source ~/.bashrc
```

### Build Issues

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

## Getting Help

* GitHub Issues: Report bugs or request features
* GitHub Discussions: Ask questions and share ideas
* Documentation: Check all `.md` files in the repository

Happy terminal commanding!
