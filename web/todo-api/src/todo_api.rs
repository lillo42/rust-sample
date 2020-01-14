use actix_web::{Responder, web, get};
use crate::repositories::{PostgresRepository, Repository};

#[get("/api/v1/todo")]
pub async fn get_all() -> impl Responder {
    println!("Getting all todo");
    let postgres = PostgresRepository::new();
    web::Json(postgres.get_all())
}