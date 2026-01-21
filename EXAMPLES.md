---
title: SuperTerminal Examples
description: Practical examples of using SuperTerminal for common tasks
ms.date: 2026-01-21
ms.topic: tutorial
keywords:
  - examples
  - usage
  - tutorial
estimated_reading_time: 10
---

# SuperTerminal Examples

This document provides practical examples of using SuperTerminal for common command-line tasks.

## Quick Start

### Basic Workflow

When you run SuperTerminal, the generated command is automatically copied to your clipboard:

```bash
$ superterminal "list all Python files"

Translating to shell command...

Generated Command:
  find . -name "*.py"

Copy this command to clipboard? [Y/n]: y

✓ Command copied to clipboard!
find . -name "*.py"
Paste it anywhere with Ctrl+V (or Cmd+V on Mac)

# Now just press Ctrl+V in your terminal to paste and execute
$ find . -name "*.py"
```

### Skip Confirmation Mode

Use `-y` flag to automatically copy without prompting:

```bash
$ superterminal -y "show disk usage"

Translating to shell command...

Generated Command:
  df -h

✓ Command copied to clipboard!
df -h
Paste it anywhere with Ctrl+V (or Cmd+V on Mac)

# The command is ready in your clipboard - just paste and run
$ df -h
```

## File Operations

### Finding Files

```bash
# Find all Python files
superterminal "find all Python files"
# Generated: find . -name "*.py"

# Find files larger than 100MB
superterminal "find files larger than 100MB"
# Generated: find . -type f -size +100M

# Find files modified in the last 7 days
superterminal "find files modified in the last 7 days"
# Generated: find . -type f -mtime -7
```

### Listing Files

```bash
# List files by size
superterminal "list files sorted by size"
# Generated: ls -lhS

# List only directories
superterminal "list only directories"
# Generated: ls -d */

# List hidden files
superterminal "list hidden files"
# Generated: ls -a
```

### File Content Operations

```bash
# Count lines in all Python files
superterminal "count lines in all Python files"
# Generated: find . -name "*.py" -exec wc -l {} +

# Search for TODO comments
superterminal "search for TODO comments in all source files"
# Generated: grep -r "TODO" --include="*.rs" --include="*.py" .

# Find largest files in current directory
superterminal "show the 10 largest files"
# Generated: du -ah . | sort -rh | head -n 10
```

## System Information

### Resource Usage

```bash
# Check disk usage
superterminal "show disk usage"
# Generated: df -h

# Check memory usage
superterminal "show memory usage"
# Generated: free -h

# Show CPU information
superterminal "display CPU information"
# Generated: lscpu
```

### Process Management

```bash
# List running processes by memory usage
superterminal "show processes using most memory"
# Generated: ps aux --sort=-%mem | head -n 10

# Find processes using a specific port
superterminal "find process using port 8080"
# Generated: lsof -i :8080
```

### Network Information

```bash
# Show network connections
superterminal "display active network connections"
# Generated: netstat -tunap

# Check listening ports
superterminal "show listening ports"
# Generated: ss -tuln
```

## Text Processing

### Searching

```bash
# Search for pattern in files
superterminal "search for 'error' in all log files"
# Generated: grep -r "error" *.log

# Case-insensitive search
superterminal "search for password case-insensitively"
# Generated: grep -ri "password" .

# Count occurrences of a word
superterminal "count how many times 'function' appears"
# Generated: grep -o "function" * | wc -l
```

### Manipulation

```bash
# Sort lines in a file
superterminal "sort lines in file.txt alphabetically"
# Generated: sort file.txt

# Remove duplicate lines
superterminal "remove duplicate lines from file.txt"
# Generated: sort file.txt | uniq

# Replace text in multiple files
superterminal "replace 'old_name' with 'new_name' in all Python files"
# Generated: find . -name "*.py" -exec sed -i 's/old_name/new_name/g' {} +
```

## Git Operations

### Repository Information

```bash
# Show branches by last commit date
superterminal "show git branches sorted by last commit"
# Generated: git branch --sort=-committerdate

# List commits from last week
superterminal "list commits from the last week"
# Generated: git log --since='1 week ago' --oneline

# Find largest files in git history
superterminal "find the 10 largest files in git history"
# Generated: git rev-list --objects --all | git cat-file --batch-check='%(objecttype) %(objectname) %(objectsize) %(rest)' | grep '^blob' | sort -k3 -n -r | head -n 10
```

### Repository Statistics

```bash
# Count commits by author
superterminal "count commits by each author"
# Generated: git shortlog -sn --all

# Show file change statistics
superterminal "show which files have changed the most"
# Generated: git log --pretty=format: --name-only | sort | uniq -c | sort -rg | head -10
```

## Archive Operations

### Compression

```bash
# Create a tar.gz archive
superterminal "create a compressed archive of src directory"
# Generated: tar -czf src.tar.gz src/

# Extract a tar.gz archive
superterminal "extract archive.tar.gz"
# Generated: tar -xzf archive.tar.gz

# Create a zip file
superterminal "zip all Python files"
# Generated: zip python_files.zip *.py
```

## Advanced Examples

### Combining Multiple Operations

```bash
# Find and delete old log files
superterminal "find and delete log files older than 30 days"
# Generated: find . -name "*.log" -mtime +30 -delete

# Backup and compress
superterminal "create a dated backup of the config directory"
# Generated: tar -czf config_backup_$(date +%Y%m%d).tar.gz config/

# Monitor file changes
superterminal "watch for changes in the current directory"
# Generated: watch -n 1 'ls -ltr | tail'
```

### System Maintenance

```bash
# Clean up old files
superterminal "remove cache files older than 7 days"
# Generated: find ~/.cache -type f -mtime +7 -delete

# Check system logs for errors
superterminal "check system logs for errors in the last hour"
# Generated: journalctl --since "1 hour ago" | grep -i error
```

## Using Configuration Options

### Adjusting AI Behavior

```bash
# Use a different model
superterminal config --model gpt-4-turbo
superterminal "complex query requiring advanced reasoning"

# Adjust temperature for more creative responses
superterminal config --temperature 0.8
superterminal "find files with unusual patterns"

# Increase token limit for complex commands
superterminal config --max-tokens 250
superterminal "create a complex pipeline with multiple steps"
```

## Tips and Best Practices

### Be Specific

```bash
# Instead of: "find files"
superterminal "find all TypeScript files in the src directory modified today"

# Instead of: "show usage"
superterminal "show disk usage of each subdirectory sorted by size"
```

### Use Natural Language

```bash
# Natural phrasing works well
superterminal "I want to see all processes using more than 100MB of RAM"

# Questions work too
superterminal "How can I find the size of this directory?"

# Commands can be implied
superterminal "list files, but exclude hidden ones"
```

### Safety First

```bash
# Review destructive commands carefully
superterminal "delete all temporary files" # REVIEW BEFORE EXECUTING!

# Start with dry runs when available
superterminal "show what would be deleted by removing old logs"
```

## Troubleshooting Examples

### When Commands Don't Work as Expected

If the generated command isn't quite right:

1. **Add more details**:

```bash
# Instead of: "compress files"
superterminal "compress all JavaScript files into a single tar.gz archive"
```

2. **Specify the format**:

```bash
# Instead of: "list processes"
superterminal "list processes in a table showing PID, CPU, memory, and command"
```

3. **Adjust AI parameters**:

```bash
# Lower temperature for more deterministic results
superterminal config --temperature 0.2
```

## Integration with Shell

### Adding to Shell Profile

Add this to your `.bashrc` or `.zshrc`:

```bash
# Alias for quick access
alias st='superterminal'

# Function to execute commands directly (use with caution!)
stexec() {
    local cmd=$(superterminal -y "$@")
    echo "Executing: $cmd"
    eval "$cmd"
}
```

Usage:

```bash
st "list files"  # Review and copy
stexec "show date"  # Execute directly
```

> [!WARNING]
> Executing commands directly without review can be dangerous. Only use `stexec` for non-destructive operations.

## Next Steps

* Explore the [README](../README.md) for more features
* Check the [Contributing Guide](../CONTRIBUTING.md) to help improve SuperTerminal
* Report issues or request features on GitHub

Happy command-line productivity!
