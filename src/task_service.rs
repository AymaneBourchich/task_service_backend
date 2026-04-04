use crate::task::Task;

pub struct TaskService {
    tasks: Vec<Task>,
}
impl TaskService {
    pub fn new() -> TaskService {
        TaskService {tasks: Vec::new()}
    }

    pub fn add_task(&mut self, id: u64, title: String) {
        let task = Task::new(id, title);
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self, id: u64) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id() == id) {
            task.complete();
            true
        } else {
            false
        }
    }

    pub fn list_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}