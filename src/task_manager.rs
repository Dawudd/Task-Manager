use std::collections::HashMap;
use crate::task::Task;

pub struct TaskManager {
    tasks: HashMap<String, Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.name().to_string(), task);
    }

    pub fn remove_task(&mut self, name: &str) -> Option<Task> {
        self.tasks.remove(name)
    }

    pub fn get_task(&self, name: &str) -> Option<&Task> {
        self.tasks.get(name)
    }

    pub fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    pub fn get_all_tasks_with_due_date(&self) -> Vec<&Task> {
        let mut tasks_with_due_date: Vec<&Task> = self.tasks.values()
            .filter(|task| task.due_date().is_some())
            .collect();

        tasks_with_due_date.sort_by_key(|task| task.due_date().unwrap());

        tasks_with_due_date
    }

    pub fn get_all_tasks_without_due_date(&self) -> Vec<&Task> {
        self.tasks.values()
            .filter(|task| task.due_date().is_none())
            .collect()
    }

    pub fn list_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
    
    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }
    
    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }
    
    pub fn list_completed_tasks(&self) -> Vec<&Task> {
        self.tasks.values().filter(|task| task.completed()).collect()
    }
    
    pub fn list_pending_tasks(&self) -> Vec<&Task> {
        self.tasks.values().filter(|task| !task.completed()).collect()
    }
}