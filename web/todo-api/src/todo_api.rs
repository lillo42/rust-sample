use actix_web::{Responder, web, get, post, put, delete, HttpResponse};
use crate::repositories::{PostgresRepository, Repository};
use crate::entities::{CreateTodoRequest, Todo, UpdateTodoRequest};

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

#[put("/api/v1/todo/{id}")]
pub async fn update(update_todo: web::Json<UpdateTodoRequest>, path: web::Path<i32>) -> impl Responder {
    println!("Updating Todo");
    let postgres = PostgresRepository::new();

    let todo = Todo {
        id: path.into_inner(),
        text: update_todo.text.clone(),
        is_done: update_todo.is_done
    };

    postgres.update(&todo);

    return HttpResponse::Accepted();
}

#[delete("/api/v1/todo/{id}")]
pub async fn delete(path: web::Path<i32>) -> impl Responder {
    println!("Updating Todo");
    let postgres = PostgresRepository::new();

    let todo = Todo {
        id: path.into_inner(),
        text: String::from(""),
        is_done: false
    };

    postgres.delete(&todo);
    return HttpResponse::Accepted();
}