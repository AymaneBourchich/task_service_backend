use actix_web::{get, post, web, HttpResponse, Responder};
use crate::task::task_service::TaskService;

#[get("/tasks")]
async fn list_tasks(service: web::Data<TaskService>) -> impl Responder {
    let tasks: Vec<_> = service.list_tasks().map(|task| task.title()).collect();
    HttpResponse::Ok().json(tasks)
}