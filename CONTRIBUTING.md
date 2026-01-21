---
title: Contributing to SuperTerminal
description: Guidelines for contributing to the SuperTerminal project
ms.date: 2026-01-21
ms.topic: how-to
keywords:
  - contributing
  - development
  - guidelines
estimated_reading_time: 8
---

# Contributing to SuperTerminal

Thank you for your interest in contributing to SuperTerminal! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for all contributors.

## Getting Started

### Prerequisites

* Rust 1.70 or higher
* Git
* An OpenAI API key for testing
* Basic familiarity with Rust and command-line applications

### Setting Up Your Development Environment

1. Fork the repository on GitHub
2. Clone your fork locally:

```bash
git clone https://github.com/your-username/superterminal.git
cd superterminal
```

3. Add the upstream repository:

```bash
git remote add upstream https://github.com/original-owner/superterminal.git
```

4. Set up your OpenAI API key:

```bash
# Option 1: Environment variable
export OPENAI_API_KEY='your-test-api-key'

# Option 2: Create a .env file (recommended for development)
cp .env.example .env
# Then edit .env and add your API key
```

5. Build the project:

```bash
cargo build
```

6. Run the tests:

```bash
cargo test
```

## Development Workflow

### Creating a Feature Branch

Always create a new branch for your work:

```bash
git checkout -b feature/your-feature-name
```

Use descriptive branch names:

* `feature/add-command-history` - for new features
* `fix/api-timeout-handling` - for bug fixes
* `docs/improve-readme` - for documentation
* `refactor/simplify-config` - for refactoring

### Making Changes

1. Make your changes following the coding standards below
2. Add tests for new functionality
3. Ensure all tests pass: `cargo test`
4. Format your code: `cargo fmt`
5. Run the linter: `cargo clippy`

### Committing Changes

Write clear, concise commit messages following this format:

```text
Short summary (50 chars or less)

More detailed explanation if needed. Wrap at 72 characters.
Include the motivation for the change and contrast with
previous behavior.

Fixes #issue-number
```

Examples:

```text
Add command history feature

Implements a command history system that stores previously
generated commands. Users can view and reuse past commands
with the --history flag.

Fixes #42
```

### Submitting a Pull Request

1. Push your branch to your fork:

```bash
git push origin feature/your-feature-name
```

2. Open a Pull Request on GitHub
3. Fill out the PR template completely
4. Link any related issues
5. Wait for review and address feedback

## Coding Standards

### Rust Style

Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/):

* Use `cargo fmt` for automatic formatting
* Follow Rust naming conventions (RFC 430)
* Write idiomatic Rust code
* Prefer borrowing over cloning
* Handle errors properly using `Result<T, E>`

### Code Organization

* Keep functions focused and small
* Use modules to organize related functionality
* Document public APIs with rustdoc comments
* Place unit tests in the same file as the code they test
* Place integration tests in the `tests/` directory

### Documentation

All public items must have documentation:

```rust
/// Translates natural language to a shell command.
///
/// Takes a natural language description and uses AI to generate
/// an appropriate shell command.
///
/// # Arguments
///
/// * `natural_language` - The user's natural language request
///
/// # Returns
///
/// Returns the generated shell command as a `String`, or an error
/// if the translation fails.
///
/// # Errors
///
/// Returns `SuperTerminalError::InvalidInput` if the input is empty.
/// Returns `SuperTerminalError::OpenAIError` if the API call fails.
///
/// # Examples
///
/// ```no_run
/// let service = AIService::new(config)?;
/// let command = service.translate_to_command("list files").await?;
/// assert_eq!(command, "ls -la");
/// ```
pub async fn translate_to_command(&self, natural_language: &str) -> Result<String> {
    // Implementation
}
```

### Testing

Write tests for all new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_extraction() {
        let config = Config::default();
        let service = AIService::new(config).unwrap();
        let result = service.extract_command("ls -la");
        assert_eq!(result, "ls -la");
    }

    #[tokio::test]
    async fn test_api_integration() {
        // Integration tests
    }
}
```

### Error Handling

* Use custom error types defined in `src/error/mod.rs`
* Provide helpful error messages
* Use `Result<T, E>` for fallible operations
* Avoid `unwrap()` and `expect()` in library code

Example:

```rust
pub fn load_config() -> Result<Config> {
    let path = config_path()
        .ok_or_else(|| SuperTerminalError::ConfigError(
            "Could not find config directory".to_string()
        ))?;
    
    let content = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)
        .map_err(|e| SuperTerminalError::ConfigError(e.to_string()))?;
    
    Ok(config)
}
```

## Testing Guidelines

### Unit Tests

* Test individual functions and methods
* Mock external dependencies
* Use descriptive test names
* Test edge cases and error conditions

### Integration Tests

* Test complete workflows
* Use real API calls sparingly (they cost money)
* Consider using recorded responses for API tests

### Test Coverage

Aim for high test coverage:

```bash
# Install tarpaulin for coverage reports
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

## Documentation Guidelines

### README

* Keep the README up to date
* Include clear examples
* Document all major features
* Provide troubleshooting tips

### Code Comments

* Explain why, not what
* Keep comments up to date
* Use rustdoc for API documentation
* Include examples in documentation

### Markdown Style

Follow the project's markdown style guide:

* Use ATX-style headings (`#`, `##`, `###`)
* Include YAML frontmatter
* Wrap lines at 100 characters
* Use code blocks with language specifiers

## Pull Request Process

1. **Ensure CI Passes**: All tests and checks must pass
2. **Update Documentation**: Update README and docs as needed
3. **Add Tests**: Include tests for new functionality
4. **Request Review**: Ask for review from maintainers
5. **Address Feedback**: Respond to all review comments
6. **Squash Commits**: Squash fixup commits before merging

## Release Process

Releases follow semantic versioning (SemVer):

* **Major** (1.0.0): Breaking changes
* **Minor** (0.1.0): New features, backward compatible
* **Patch** (0.0.1): Bug fixes, backward compatible

## Getting Help

If you need help:

* Check existing issues and discussions
* Ask in GitHub Discussions
* Reach out to maintainers
* Join the community chat (if available)

## Recognition

Contributors will be:

* Listed in the project's contributors list
* Mentioned in release notes for significant contributions
* Credited in commit history

Thank you for contributing to SuperTerminal!
