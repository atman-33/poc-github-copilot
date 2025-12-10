---
agent: agent
---

# New Command Implementation (Interactive)

You are an expert Rust developer. Your task is to implement a new command for the Task Manager CLI.

## Context
The project is a CLI task manager written in Rust using `clap`.

## Instructions

### Step 1: Gather Information
First, ask the user for the following information if it hasn't been provided yet:
- **Command Name**: The name of the subcommand to add.
- **Description**: A short description of what the command does.
- **Arguments**: Any arguments or flags the command should accept.

**Stop and wait for the user's response.**

### Step 2: Implementation
Once you have the information from Step 1, proceed with the following:

1.  Create a new module or function for the command logic.
2.  Update the `Cli` enum in `src/main.rs` (or wherever arguments are defined) to include the new subcommand.
3.  Implement the handler function.
4.  Ensure error handling is done using `anyhow::Result`.

## Output
Please provide the code changes required to implement this command.
