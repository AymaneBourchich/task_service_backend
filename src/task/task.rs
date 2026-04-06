pub struct Task {
    id: u64,
    title: String,
    completed: bool,
}

impl Task {
    pub fn new(id: u64, title: String) -> Self {
        Self {
            id, 
            title, 
            completed: false,
        }
    }
    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}
