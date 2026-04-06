use std::sync::{Arc, Mutex};
use actix_web::{web, App, HttpServer};
use task_service::web::routes::*;
use task_service::task::task_service::TaskService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let service: Arc<Mutex<TaskService>> = Arc::new(Mutex::new(TaskService::new()));

    HttpServer::new({
        let service = service.clone();
        move || {
            App::new()
                .app_data(web::Data::from(service.clone()))
                .service(list_tasks)
                .service(create_task)
                .service(complete_task)
        }
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}