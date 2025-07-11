use std::collections::HashSet;
use chrono::{NaiveDate, Local};
use colored::Colorize;

#[derive(Clone)]
pub struct Task {
    name: String,
    description: Option<String>,
    due_date: Option<NaiveDate>,
    tags: HashSet<String>,
    priority: u8,
    completed: bool,
}
impl Task {
    pub fn new(name: String) -> Self {
        Task {
            name,
            description: None,
            due_date: None,
            tags: HashSet::new(),
            priority: 5, // Default priority
            completed: false,
        }
    }

    // Getters
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn due_date_as_str(&self) -> Option<String> {
        self.due_date.map(|date| date.format("%Y-%m-%d").to_string())
    }

    pub fn due_date(&self) -> Option<NaiveDate> {
        self.due_date
    }

    pub fn tags(&self) -> &HashSet<String> {
        &self.tags
    }

    pub fn priority(&self) -> u8 {
        self.priority
    }

    pub fn print_priority(&self) {
        print!("[");
        for _ in 0..self.priority {
            print!("{}", "*".bright_yellow().bold());
        }
        let remaining = 10 - self.priority;
        for _ in 0..remaining {
            print!("_");
        }
        print!("]");
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    // Setters
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
    pub fn set_due_date(&mut self, due_date: String) -> Result<(), String> {
        match NaiveDate::parse_from_str(&due_date, "%Y-%m-%d") {
            Ok(date) => {
                self.due_date = Some(date);
                Ok(())
            },
            Err(_) => Err("Invalid date format. Use YYYY-MM-DD.".to_string()),
        }
    }
    pub fn add_tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.remove(tag);
    }
    pub fn set_priority(&mut self, priority: u8) -> Result<(), String> {
        if priority <= 10 {
            self.priority = priority;
            Ok(())
        } else {
            Err("Priority must be between 0 and 10.".to_string())
        }
    }
    pub fn mark_completed(&mut self) {
        self.completed = true;
    }

    pub fn tags_csv(&self) -> String {
        let mut tags: Vec<&str> = self.tags.iter().map(|s| s.as_str()).collect();
        tags.sort();
        tags.join(",")
    }
}