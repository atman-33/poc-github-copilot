---
agent: agent
---

# New Command Implementation Prompt

You are an expert Rust developer. Your task is to implement a new command for the Task Manager CLI.

## Context
The project is a CLI task manager written in Rust using `clap`.

## Requirements
1.  Create a new module or function for the command logic.
2.  Update the `Cli` enum in `src/main.rs` (or wherever arguments are defined) to include the new subcommand.
3.  Implement the handler function.
4.  Ensure error handling is done using `anyhow::Result`.

## Input
Command Name: {{ command_name }}
Description: {{ command_description }}
Arguments: {{ command_arguments }}

## Output
Please provide the code changes required to implement this command.
