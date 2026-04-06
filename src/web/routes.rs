use actix_web::{ get, post, web, HttpResponse, Responder };
use crate::task::task_service::TaskService;
use std::sync::Mutex;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
}

#[get("/tasks")]
async fn list_tasks(service: web::Data<Mutex<TaskService>>) -> impl Responder {
    let service = service.lock().unwrap();
    let tasks: Vec<_> = service
        .list_tasks()
        .map(|task| task.title())
        .collect();
    HttpResponse::Ok().json(tasks)
}

#[post("/tasks")]
pub async fn create_task(
    service: web::Data<Mutex<TaskService>>,
    payload: web::Json<CreateTaskRequest>
) -> impl Responder {
    let mut service = service.lock().unwrap();
    let id = service.create_task(payload.title.clone());

    HttpResponse::Ok().json(id)
}

#[post("/tasks/{id}/complete")]
pub async fn complete_task(
    service: web::Data<Mutex<TaskService>>,
    path: web::Path<u64>
) -> impl Responder {
    let mut service = service.lock().unwrap();
    match service.complete_task(path.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::NotFound().body(e),
    }

}
