use serde::{Serialize};

#[derive(Serialize)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub is_done: bool
}

impl Todo {
    pub fn clone(&self) -> Todo {
        Todo {
            id: self.id,
            text: self.text.clone(),
            is_done: self.is_done
        }
    }
}