# PoC GitHub Copilot Configuration Sample

This project is a sample project designed to effectively utilize GitHub Copilot's customization features (Agents, Instructions, Prompts).
It includes a simple CLI task management tool implemented in Rust as an implementation example.

## Project Purpose

The purpose is to demonstrate and learn how to improve the development experience using the following GitHub Copilot configuration files:

- **Agents**: `.github/agents/*.agent.md` - Definitions of custom AI agents with specific roles or knowledge.
- **Instructions**: `.github/instructions/*.instructions.md` - Instructions for Copilot regarding project or language-specific rules.
- **Prompts**: `.github/prompts/*.prompts.md` - Definitions of reusable prompt templates.

## Directory Structure

GitHub Copilot configuration files are placed in specific locations under the `.github` directory.

```
.
├── .github/
│   ├── agents/         # Custom agent definitions (*.agent.md)
│   ├── instructions/   # Instruction files (*.instructions.md)
│   └── prompts/        # Prompt templates (*.prompts.md)
├── src/
│   ├── main.rs         # CLI application entry point
│   └── task.rs         # Task data structure definition
├── Cargo.toml          # Rust project configuration
└── README.md           # This file
```

## Implementation Sample: CLI Task Manager

A simple task management tool written in Rust. It allows adding, listing, and completing tasks.

### Requirements

- Rust (cargo)

### Usage

```bash
# Add a task
cargo run -- add "Read GitHub Copilot documentation"

# List tasks
cargo run -- list

# Complete a task (specify ID)
cargo run -- complete 1
```

## GitHub Copilot Configuration Details

### Instructions
The `.github/instructions` folder defines coding conventions and behaviors for specific file types.
Example: In `rust.instructions.md`, you can describe rules such as recommending the use of `anyhow` or `clap` when generating Rust code.

### Agents
The `.github/agents` folder defines agents with specific domain knowledge or behaviors.
It is intended to be used by invoking them like `@agent_name` in the chat interface.

### Prompts
The `.github/prompts` folder stores frequently used complex instructions as templates.
This makes it easier to obtain consistent quality responses.
