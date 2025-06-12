use std::fs::File;
use std::io;
use std::path::Path;
use crate::task::Task;

pub struct CsvHandler {
    file_path: String,
}

impl CsvHandler {
    pub fn new(file_path: String) -> CsvHandler {
        CsvHandler {
            file_path
        }
    }
    
    pub fn load_tasks(&self) -> io::Result<Vec<Task>> {
        if (!Path::new(&self.file_path).exists()) {
            return Ok(Vec::new());
        }
        
        let file = File::open(&self.file_path)?;
        let mut csv_reader = csv::Reader::from_reader(file);
        let mut tasks = Vec::new();
        
        for result in csv_reader.records() {
            let record = result?;
            let mut task = Task::new(record[0].to_string());
            
            // Description
            if !record[1].is_empty() {
                task.set_description(record[1].to_string());
            }
            // Due date
            if !record[2].is_empty() {
                if let Err(e) = task.set_due_date(record[2].to_string()) {
                    eprintln!("Error setting due date for task '{}': {}", task.name(), e);
                }
            }
            // Priority
            if let Ok(priority) = record[3].parse::<u8>() {
                if let Err(e) = task.set_priority(priority) {
                    eprintln!("Error setting priority for task '{}': {}", task.name(), e);
                }
            }
            // Status
            if record[4].parse::<bool>().unwrap_or(false) {
                task.mark_completed();
            }
            // Tags
            for tag in record[5].split(',') {
                let tag = tag.trim();
                if !tag.is_empty() {
                    task.add_tag(tag.to_string());
                }
            }
            tasks.push(task);
        }
        Ok(tasks)
    }
}