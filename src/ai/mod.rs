/// AI service for translating natural language to shell commands
use crate::config::Config;
use crate::error::{Result, SuperTerminalError};
use async_openai::{
    types::chat::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};

/// AI service for command translation
pub struct AIService {
    client: Client<async_openai::config::OpenAIConfig>,
    config: Config,
}

impl AIService {
    /// Creates a new AI service with the given configuration
    pub fn new(config: Config) -> Result<Self> {
        let openai_config = async_openai::config::OpenAIConfig::new().with_api_key(&config.api_key);

        let client = Client::with_config(openai_config);

        Ok(Self { client, config })
    }

    /// Translates natural language input to a shell command
    pub async fn translate_to_command(&self, natural_language: &str) -> Result<String> {
        if natural_language.trim().is_empty() {
            return Err(SuperTerminalError::InvalidInput(
                "Input cannot be empty".to_string(),
            ));
        }

        let system_message = self.build_system_prompt();
        let user_message = format!(
            "Convert this natural language request to a shell command: {}",
            natural_language
        );

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.config.model)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_message)
                    .build()
                    .map_err(|e| SuperTerminalError::ConfigError(e.to_string()))?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_message)
                    .build()
                    .map_err(|e| SuperTerminalError::ConfigError(e.to_string()))?
                    .into(),
            ])
            .max_tokens(self.config.max_tokens)
            .temperature(self.config.temperature)
            .build()
            .map_err(|e| SuperTerminalError::ConfigError(e.to_string()))?;

        let response = self.client.chat().create(request).await?;

        let command = response
            .choices
            .first()
            .and_then(|choice| choice.message.content.as_ref())
            .map(|content| self.extract_command(content))
            .ok_or_else(|| SuperTerminalError::ConfigError("No response from AI".to_string()))?;

        Ok(command)
    }

    /// Builds the system prompt for the AI
    fn build_system_prompt(&self) -> String {
        r#"You are a helpful assistant that converts natural language requests into shell commands.
Rules:
1. Return ONLY the shell command, nothing else
2. Do not include explanations or markdown formatting
3. Do not include code block markers (```)
4. Return a single command or a pipeline of commands
5. Make commands safe and avoid destructive operations without explicit confirmation
6. For Linux/Unix systems, use standard bash commands
7. If the request is ambiguous, make reasonable assumptions
8. Do not include comments in the command

Examples:
Input: "list all files in the current directory"
Output: ls -la

Input: "find all python files"
Output: find . -name "*.py"

Input: "show disk usage"
Output: df -h

Input: "count lines in all text files"
Output: find . -name "*.txt" -exec wc -l {} +
"#
        .to_string()
    }

    /// Extracts the command from the AI response, removing any markdown or extra text
    fn extract_command(&self, content: &str) -> String {
        let mut content = content.trim();

        // Remove markdown code blocks if present
        if let Some(stripped) = content.strip_prefix("```bash") {
            content = stripped;
        } else if let Some(stripped) = content.strip_prefix("```sh") {
            content = stripped;
        } else if let Some(stripped) = content.strip_prefix("```") {
            content = stripped;
        }

        content = content.trim();

        if let Some(stripped) = content.strip_suffix("```") {
            content = stripped.trim();
        }

        // Take only the first non-empty line
        let command = content
            .lines()
            .map(|line| line.trim())
            .find(|line| !line.is_empty())
            .unwrap_or("");

        command.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_command_plain() {
        let config = Config::default();
        let service = AIService::new(config).unwrap();
        let result = service.extract_command("ls -la");
        assert_eq!(result, "ls -la");
    }

    #[test]
    fn test_extract_command_with_markdown() {
        let config = Config::default();
        let service = AIService::new(config).unwrap();
        let result = service.extract_command("```bash\nls -la\n```");
        assert_eq!(result, "ls -la");
    }

    #[test]
    fn test_extract_command_with_explanation() {
        let config = Config::default();
        let service = AIService::new(config).unwrap();
        let result = service.extract_command("ls -la\nThis lists all files");
        assert_eq!(result, "ls -la");
    }
}
