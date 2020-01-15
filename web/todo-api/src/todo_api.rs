use actix_web::{Responder, web, get, post};
use crate::repositories::{PostgresRepository, Repository};
use crate::entities::{CreateTodoRequest, Todo};

#[get("/api/v1/todo")]
pub async fn get_all() -> impl Responder {
    println!("Getting all todo");
    let postgres = PostgresRepository::new();
    return web::Json(postgres.get_all());
}

#[post("/api/v1/todo")]
pub async fn add(create_todo: web::Json<CreateTodoRequest>) -> impl Responder {
    println!("Adding Todo");
    let postgres = PostgresRepository::new();

    let mut todo = Todo {
        id: 0,
        text: create_todo.text.clone(),
        is_done: false
    };

    postgres.insert(&mut todo);

    return web::Json(todo);
}