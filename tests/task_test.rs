use task_service::task::task::Task;

#[test]
fn task_creation_works() {
    let task = Task::new(1, "Learn Rust".to_string());
    assert_eq!(task.id(), 1);
    assert_eq!(task.title(), "Learn Rust");
    assert_eq!(task.is_completed(), false);
}

#[test]
fn task_completion_works() {
    let mut task = Task::new(2, "Write code".to_string());
    task.complete();
    assert!(task.is_completed());
}