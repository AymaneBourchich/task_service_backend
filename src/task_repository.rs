use crate::task::Task;
use std::collections::HashMap;

pub struct TaskRepository {
    tasks: HashMap<u64, Task>,
    next_id: u64,
}

impl TaskRepository {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, title: String) -> u64{
        let id = self.next_id;
        self.next_id += 1;

        self.tasks.insert(id, Task::new(id, title));
        id

    }

    pub fn get_task(&self, id: u64) -> Option<&Task> {
        self.tasks.get(&id)
    }

    pub fn complete_task(&mut self, id: u64) -> Result<(), &'static str> {
        match self.tasks.get_mut(&id) {
            Some(task) => {
                task.complete();
                Ok(())
            }
            None => Err("Task not found"),
        }
    }
    pub fn all_tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.values()
    }
}
