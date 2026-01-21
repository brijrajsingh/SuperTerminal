---
agent: 'agent'
model: Claude Sonnet 4.5 (copilot)
tools: ['githubRepo', 'search/codebase']
description: 'Generate a new RUST based AI powered CLI application'
---

your goal is to create a new RUST based AI powered CLI application that allows users to write cli commands in natural language and have them converted into executable shell commands.

Requirements:
1. The application should accept natural language input from the user.
2. It should process the input using an AI model to generate the corresponding shell command.
3. The generated shell command should be displayed to the user for confirmation before execution.
4. Upon user confirmation, the application should provide the shell command which user can copy and execute in their terminal.
5. The application should allow default copy of the command to clipboard on confirmation.
6. The application should handle errors gracefully and provide meaningful feedback to the user.
7. Include documentation on how to set up and use the application.
8. Write unit tests to ensure the functionality of the application.



 $tool:rust-gpt-4.1-beast-mode