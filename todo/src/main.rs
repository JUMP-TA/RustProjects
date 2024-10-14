use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

#[derive(Parser)]
#[command(name = "ToDo")]
#[command(about = "A simple command-line to-do list application", long_about = None)]
struct Args {
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
    /// Mark a task as completed
    Complete {
        /// The ID of the task to complete
        id: u32,
    },
}

fn get_tasks_file() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("todo");
    fs::create_dir_all(&path).unwrap();
    path.push("tasks.json");
    path
}

fn load_tasks() -> Vec<Task> {
    let path = get_tasks_file();
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(data) => match serde_json::from_str(&data) {
                Ok(tasks) => tasks,
                Err(_) => {
                    println!("Error parsing tasks file. Starting with an empty task list.");
                    Vec::new()
                }
            },
            Err(_) => {
                println!("Error reading tasks file. Starting with an empty task list.");
                Vec::new()
            }
        }
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let path = get_tasks_file();
    match serde_json::to_string_pretty(tasks) {
        Ok(data) => {
            if let Err(_) = fs::write(&path, data) {
                println!("Error writing tasks file.");
            }
        }
        Err(_) => {
            println!("Error serializing tasks.");
        }
    }
}


fn main() {
    let args = Args::parse();
    let mut tasks = load_tasks();

    match args.command {
        Commands::Add { description } => {
            add_task(&mut tasks, description);
        }
        Commands::List => {
            list_tasks(&tasks);
        }
        Commands::Complete { id } => {
            complete_task(&mut tasks, id);
        }
    }

    save_tasks(&tasks);
}

fn add_task(tasks: &mut Vec<Task>, description: String) {
    let id = if let Some(last_task) = tasks.last() {
        last_task.id + 1
    } else {
        1
    };

    let task = Task {
        id,
        description,
        completed: false,
    };

    tasks.push(task);
    println!("Task added successfully.");
}

fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    for task in tasks {
        if !task.completed {
            println!("[{}] {}: {}", task.id, if task.completed { "x" } else { " " }, task.description);
        }
    }
}

fn complete_task(tasks: &mut Vec<Task>, id: u32) {
    if tasks.is_empty() {
        println!("No tasks to complete.");
        return;
    }

    for task in tasks.iter_mut() {
        if task.id == id {
            if task.completed {
                println!("Task {} is already completed.", id);
            } else {
                task.completed = true;
                println!("Task {} marked as completed.", id);
            }
            return;
        }
    }
    println!("Task {} not found.", id);
}