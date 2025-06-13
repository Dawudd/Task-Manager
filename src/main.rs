use std::collections::HashSet;
use crate::task_manager::TaskManager;
use std::io;
use std::process::exit;
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

fn read_task_details() -> Result<Task, String> {
    let name = read_input("Enter task name:");
    if name.is_empty() {
        return Err(String::from("Task name is empty"));
    }
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

    Ok(task)
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

fn display_completed_tasks(task_manager: &TaskManager) {
    let completed_tasks = task_manager.list_completed_tasks();
    if completed_tasks.is_empty() {
        println!("No completed tasks.");
        wait();
        return;
    }
    println!("Completed Tasks:");
    for (i, task) in completed_tasks.iter().enumerate() {
        println!("{}. {}", i + 1, task.name());
    }

    println!("\nTotal completed tasks: {}", completed_tasks.len());
    wait();
}

fn edit_task(task_manager: &mut TaskManager, name: &str) {
    if let Some(task) = task_manager.get_task(name) {
        let task = task.clone();
        clear_console();

        println!("Editing task: {}\n(Skip fields to keep current values)", task.name());

        // Name
        let new_name_input = read_input(&format!("Enter new name [{}]: ", task.name()));
        let new_name = if new_name_input.is_empty() {
            task.name().to_string()
        } else {
            new_name_input
        };

        // Description
        let current_description = task.description().unwrap_or("");
        let description_input = read_input(&format!("Enter new description [{}]: ", current_description));
        let description = if description_input.is_empty() {
            current_description.to_string()
        } else {
            description_input
        };

        // Due date
        let current_due_date = task.due_date_as_str().unwrap_or("".to_string());
        let due_date_input = read_input(&format!("Enter new due date [{}]: ", current_due_date));
        let due_date = if due_date_input.is_empty() {
            current_due_date
        } else {
            due_date_input
        };

        // Tags
        let tags_input = read_input(&format!("Enter new tags [{}]: ", task.tags_csv()));
        let tags = if tags_input.is_empty() {
            task.tags().clone()
        } else {
            tags_input.split(',')
                .map(|tag| tag.trim().to_string())
                .filter(|t| !t.is_empty())
                .collect()
        };

        // Priority
        let priority_input = read_input(&format!("Enter new priority [{}]: ", task.priority()));
        let priority: u8 = if priority_input.is_empty() {
            task.priority()
        } else {
            match priority_input.parse::<u8>() {
                Ok(p) => p,
                Err(_) => {
                    println!("Invalid priority value.");
                    return;
                }
            }
        };

        // Create updated task
        let mut new_task = Task::new(new_name);
        new_task.set_description(description);

        if !due_date.is_empty() {
            if let Err(e) = new_task.set_due_date(due_date) {
                println!("Invalid due date: {}", e);
                return;
            }
        }

        for tag in tags {
            new_task.add_tag(tag);
        }

        if let Err(e) = new_task.set_priority(priority) {
            println!("Invalid priority: {}", e);
            return;
        }

        // Remove old task and add updated task
        task_manager.remove_task(name);
        task_manager.add_task(new_task);

        println!("Task updated successfully.");
    } else {
        println!("Task not found.");
    }
}


fn display_all_tasks(task_manager: &mut TaskManager) {
    let tasks_with_due_date = task_manager.get_all_tasks_with_due_date();
    let tasks_without_due_date = task_manager.get_all_tasks_without_due_date();

    if !tasks_with_due_date.is_empty() {
        println!("Tasks with due date:");
        for (i, name) in tasks_with_due_date.iter().enumerate() {
            println!("{}. {} {}", i + 1, name.due_date().unwrap(), name.name());
        }
    }

    if !tasks_without_due_date.is_empty() {
        println!("\nTasks without due date:");
        for (i, name) in tasks_without_due_date.iter().enumerate() {
            println!("{}. {}", i + 1 + tasks_with_due_date.len(), name.name());
        }
    }

    if task_manager.task_count() == 0 {
        println!("No tasks available.");
        wait();
        return;
    } else {
        println!("\nTotal tasks: {}", task_manager.task_count());
    }

    let choice = read_input("\nEnter task number to view details");
    if !choice.is_empty() {
        if let Ok(index) = choice.parse::<usize>() {
            let selected_name = if index <= tasks_with_due_date.len() {
                tasks_with_due_date.get(index - 1).map(|task| task.name())
            } else {
                tasks_without_due_date.get(index - 1 - tasks_with_due_date.len()).map(|task| task.name())
            };
            if let Some(task_name) = selected_name {
                if let Some(task) = task_manager.get_task(&task_name).cloned() {
                    clear_console();
                    display_task(&task);
                    let action = read_input("\nActions: [E]dit, [C]omplete, [D]elete");
                    match action.to_uppercase().as_str() {
                        "E" => {
                            edit_task(task_manager, &task.name());
                        }
                        "C" => {
                            task_manager.mark_task_completed(&task.name());
                            println!("Task marked as completed.");
                        }
                        "D" => {
                            task_manager.remove_task(&task.name());
                            println!("Task deleted successfully.");
                        }
                        _ => println!("Invalid action."),
                    }
                }
            }
        }
    }
}

fn wait() {
    read_input("");
}

fn main() {
    let mut task_manager = TaskManager::new();
    let csv_handler = csv_handler::CsvHandler::new("tasks.csv".to_string());

    // Loading tasks
    if let Ok(tasks) = csv_handler.load_tasks() {
        for task in tasks {
            task_manager.add_task(task);
        }
    } else {
        println!("Error loading tasks from file. Starting with an empty task list.");
        wait();
    }

    loop {
        clear_console();
        println!("Task Manager");
        println!("(1) List and manage tasks");
        println!("(2) Add a new task");
        println!("(3) View completed tasks");
        println!("(4) Exit and save tasks");
        
        let choice = read_input("Choose an option:");

        match choice.as_str() {
            "1" => {
                clear_console();
                display_all_tasks(&mut task_manager);
            }
            "2" => {
                clear_console();
                match read_task_details() {
                    Ok(task) => {
                        task_manager.add_task(task);
                        println!("Task added successfully.");
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }

                wait();
            }
            "3" => {
                clear_console();
                display_completed_tasks(&task_manager);
            }
            "4" => {
                clear_console();
                if let Err(e) = csv_handler.save_tasks(&task_manager.get_all_tasks()) {
                    println!("Error saving tasks: {}", e);
                } else {
                    println!("Tasks saved successfully.");
                }
                exit(0);
            }
            _ => {
                //
            }
        }
    }
}
