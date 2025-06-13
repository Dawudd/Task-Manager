use crate::task_manager::TaskManager;
use std::io;
use crate::task::Task;

pub mod task;
mod csv_handler;
mod task_manager;

fn clear_console() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear console");
    } else {
        std::process::Command::new("clear")
            .status()
            .expect("Failed to clear console");
    }
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn read_task_details() -> Task {
    let name = read_input("Enter task name:");
    let mut task = Task::new(name);

    // Description
    let description = read_input("Enter description (optional):");
    if !description.is_empty() {
        task.set_description(description);
    }

    // Due date
    loop {
        let due_date = read_input("Enter due date (YYYY-MM-DD) (optional):");
        if due_date.is_empty() {
            break;
        }
        match task.set_due_date(due_date) {
            Ok(_) => break,
            Err(e) => println!("Error setting due date: {}", e),
        }
    }

    // Tags
    let tags = read_input("Enter tags (comma-separated):");
    if !tags.is_empty() {
        for tag in tags.split(',') {
            task.add_tag(tag.trim().to_string());
        }
    }
    
    // Priority
    loop {
        let priority = read_input("Enter priority (0-10) (default: 5):");
        if priority.is_empty() {
            break;
        }
        if let Ok(priority) = priority.parse::<u8>() {
            match task.set_priority(priority) {
                Ok(_) => break,
                Err(e) => println!("Error setting priority: {}", e),
            }
        }
    }
    
    task
}

fn display_task(task: &Task) {
    println!("Name: {}", task.name());
    if let Some(description) = task.description() {
        println!("Description: {}", description);
    }
    if let Some(due_date) = task.due_date() {
        println!("Due Date: {}", due_date);
    }
    if !task.tags().is_empty() {
        println!("Tags: {}", task.tags().iter().cloned().collect::<Vec<String>>().join(", "));
    }
    task.print_priority();
    println!("Completed: {}", task.completed());
}

fn main() {
    println!("Hello, world!");
}
