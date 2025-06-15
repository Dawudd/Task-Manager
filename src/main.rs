use crate::task_manager::TaskManager;
use std::process::exit;
use colored::Colorize;
use crate::console::{clear_console, read_input, wait, print_tasks_for_today, display_all_tasks, display_completed_tasks, read_task_details};

pub mod task;
mod csv_handler;
mod task_manager;
mod console;



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
        print_tasks_for_today(&task_manager);
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
                        if task_manager.get_task(&task.name()).is_some() {
                            let confirm = read_input("Task with this name already exists. Do you want to overwrite it? (y/n)");
                            if confirm.to_lowercase() != "y" {
                                println!("Task not added.");
                                wait();
                                continue;
                            }
                        }
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
