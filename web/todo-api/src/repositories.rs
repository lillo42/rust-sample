use crate::entities::Todo;
use postgres::{Client, NoTls};
use actix_web::web::Data;

pub trait Repository {
    fn get_all(&self) -> Vec<Todo>;

    fn insert(&self, todo: &Todo);

    fn update(&self, todo: &Todo);

    fn delete(&self, todo: &Todo);
}

pub struct PostgresRepository {
    connection: String,
}

impl PostgresRepository {
    pub fn new() -> PostgresRepository {
        PostgresRepository {
            connection: String::from("postgres://postgres:Rust123@localhost/todo")
        }
    }
}

impl Repository for PostgresRepository {
     fn get_all(&self) -> Vec<Todo> {
        let conn = Client::connect(self.connection.as_ref(),  NoTls);
        let mut conn =  match conn {
            Ok(c) => c,
            Err(e) => {
                println!("Error to create connection string [Error: {}]", e);
                return vec![];
            }
        };

        let mut result = vec![];

        let rows = conn.query("SELECT * FROM Todo", &[]);
        let rows = match rows {
            Ok(r) => r,
            Err(e)  => {
                println!("Error execute query {}", e);
                return result
            }
        };

        for row in rows {
            result.push(Todo {
                id: row.get(0),
                text: row.get(1),
                is_done: row.get(2)
            });
        }

        result
    }

    fn insert(&self, todo: &Todo) {
        let conn = Client::connect(self.connection.as_ref(),  NoTls);
        let mut conn =  match conn {
            Ok(c) => c,
            Err(e) => {
                println!("Error to create connection string [Error: {}]", e);
                return;
            }
        };

        match conn.execute("INSERT INTO Todo (id, text, is_done) VALUES ($1, $2, $3)",
                        &[&todo.id, &todo.text, &todo.is_done]) {
            Err(e) => println!("Error to insert {}", e),
            Ok(_) => {}
        };
    }

    fn update(&self, todo: &Todo) {
        let conn = Client::connect(self.connection.as_ref(),  NoTls);
        let mut conn =  match conn {
            Ok(c) => c,
            Err(e) => {
                println!("Error to create connection string [Error: {}]", e);
                return;
            }
        };

        match conn.execute("UPDATE Todo SET text=$1, is_done=$2 WHERE id=$3",
                           &[&todo.text, &todo.is_done, &todo.id]) {
            Err(e) => println!("Error to insert {}", e),
            Ok(_) => {}
        };
    }

    fn delete(&self, todo: &Todo) {
        let conn = Client::connect(self.connection.as_ref(),  NoTls);
        let mut conn =  match conn {
            Ok(c) => c,
            Err(e) => {
                println!("Error to create connection string [Error: {}]", e);
                return;
            }
        };

        match conn.execute("DELETE FROM Todo WHERE id=$1",
                           &[&todo.id]) {
            Err(e) => println!("Error to insert {}", e),
            Ok(_) => {}
        };
    }
}


static mut DATA: Vec<Todo> = vec![];
pub struct InMemoryRepository {

}

impl Repository for InMemoryRepository {
    fn get_all(&self) -> Vec<Todo> {
        let mut result = vec![];
        unsafe {
            DATA.clone_into(&result);
        }
        result
    }

    fn insert(&self, todo: &Todo) {
        unsafe {
            DATA.push(todo.clone());
        }
    }

    fn update(&self, todo: &Todo) {
        unsafe {
            let update = DATA.iter().find(|x| x.id.eq(&todo.id));
            match update {
                Some(e) => {},
                None() => {}

            }
        }
    }

    fn delete(&self, todo: &Todo) {
        unimplemented!()
    }
}