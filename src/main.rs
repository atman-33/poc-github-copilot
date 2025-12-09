mod task;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs::{self, OpenOptions};
use std::path::Path;
use task::Task;

#[derive(Parser)]
#[command(name = "task-manager")]
#[command(about = "A simple CLI task manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// The description of the task
        description: String,
    },
    /// List all tasks
    List,
    /// Complete a task
    Complete {
        /// The ID of the task to complete
        id: usize,
    },
}

const DB_FILE: &str = "tasks.json";

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut tasks = load_tasks()?;

    match &cli.command {
        Commands::Add { description } => {
            let id = tasks.len() + 1;
            let new_task = Task::new(id, description.clone());
            tasks.push(new_task);
            println!("Task added: {}", description);
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks found.");
            } else {
                for task in &tasks {
                    let status = if task.completed { "[x]" } else { "[ ]" };
                    println!("{} {}: {}", status, task.id, task.description);
                }
            }
        }
        Commands::Complete { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == *id) {
                task.complete();
                println!("Task {} marked as completed.", id);
            } else {
                println!("Task with ID {} not found.", id);
            }
        }
    }

    save_tasks(&tasks)?;

    Ok(())
}

fn load_tasks() -> Result<Vec<Task>> {
    if !Path::new(DB_FILE).exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(DB_FILE).context("Failed to read tasks file")?;
    let tasks: Vec<Task> = serde_json::from_str(&content).context("Failed to parse tasks")?;
    Ok(tasks)
}

fn save_tasks(tasks: &[Task]) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(DB_FILE)
        .context("Failed to open tasks file for writing")?;
    serde_json::to_writer_pretty(file, tasks).context("Failed to write tasks")?;
    Ok(())
}
