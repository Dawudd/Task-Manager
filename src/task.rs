pub mod task {
    #[derive(Debug)]
    struct Task {
        name: String,
        description: Option<String>,
        due_date: Option<String>,
        tags: Vec<String>,
        priority: u8,
        completed: bool,
    }

    impl Task {
        pub fn new(name: String) -> Self {
            Task {
                name,
                description: None,
                due_date: None,
                tags: Vec::new(),
                priority: 0,
                completed: false,
            }
        }

        pub fn set_description(&mut self, description: String) {
            self.description = Some(description);
        }
        pub fn set_due_date(&mut self, due_date: String) {
            self.due_date = Some(due_date);
        }

        pub fn add_tag(&mut self, tag: String) {
            self.tags.push(tag);
        }

        pub fn set_priority(&mut self, priority: u8) {
            self.priority = priority;
        }

        pub fn mark_completed(&mut self) {
            self.completed = true;
        }
    }
}