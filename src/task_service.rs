use crate::task::Task;
use crate::task_repository::TaskRepository;

pub struct TaskService {
    repository: TaskRepository,
}

impl TaskService {
    pub fn new() -> Self {
        Self {
            repository: TaskRepository::new(),
        }
    }

    pub fn create_task(&mut self, title: String) -> u64 {
        self.repository.add_task(title)
    }

    pub fn complete_task(&mut self, id: u64) -> Result<(), &'static str> {
        self.repository.complete_task(id)
    }

    pub fn get_task(&self, id: u64) -> Option<&Task> {
        self.repository.get_task(id)
    }

    pub fn list_tasks(&self) -> impl Iterator<Item = &Task> {
        self.repository.all_tasks()
    }
}
