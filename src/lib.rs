pub mod task;
pub mod web;
// use crate::task::task_service::TaskService;
// use actix_web::{App, HttpServer};
// use crate::web::routes::*;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let service = TaskService::new();
//     HttpServer::new(move || {
//         App::new()
//             .app_data(actix_web::web::Data::new(service.clone()))
//             .service(list_tasks)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }