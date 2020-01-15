use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub is_done: bool
}

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub text: String
}

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    pub text: String,
    pub is_done: bool
}
