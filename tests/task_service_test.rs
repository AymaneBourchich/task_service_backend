use task_service::task::task_service::TaskService;
use task_service::task::task::Task;

#[test]
fn create_task_returns_valid_id() {
    let mut service = TaskService::new();

    let id = service.create_task("Learn Rust".to_string());

    assert_eq!(id, 1);

    let task = service.get_task(id).unwrap();
    assert_eq!(task.id(), 1);
    assert_eq!(task.title(), "Learn Rust");
    assert!(!task.is_completed());
}

#[test]
fn complete_task_marks_task_completed() {
    let mut service = TaskService::new();

    let id = service.create_task("Write code".to_string());

    service.complete_task(id).unwrap();

    let task = service.get_task(id).unwrap();
    assert!(task.is_completed());
}

#[test]
fn get_task_returns_none_for_missing_id() {
    let service = TaskService::new();

    let result = service.get_task(42);

    assert!(result.is_none());
}

#[test]
fn list_tasks_returns_all_tasks() {
    let mut service = TaskService::new();

    let id1 = service.create_task("Task 1".to_string());
    let id2 = service.create_task("Task 2".to_string());

    let tasks: Vec<&Task> = service.list_tasks().collect();

    assert_eq!(tasks.len(), 2);

    let titles: Vec<&str> = tasks.iter().map(|t| t.title()).collect();

    assert!(titles.contains(&"Task 1"));
    assert!(titles.contains(&"Task 2"));

    assert!(service.get_task(id1).is_some());
    assert!(service.get_task(id2).is_some());
}