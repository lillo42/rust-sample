use crate::entities::Todo;
extern crate postgres;
use postgres::{Connection, TlsMode};

pub trait Repository {
    fn get_all(&self) -> Vec<Todo>;

    fn insert(&self, todo: &mut Todo);

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

    fn create_connection(&self) -> Connection {
        let connection = Connection::connect(self.connection.as_ref(), TlsMode::None);
        let connection = match connection {
            Ok(client) => {
                println!("Connection open with succes");
                client
            },
            Err(e) => {
                panic!("Error to create connection string [Error: {}]", e);
            }
        };

        connection
    }
}

impl Repository for PostgresRepository {
     fn get_all(&self) -> Vec<Todo> {
        let conn = self.create_connection();

        let mut result = vec![];

        let rows = conn.query("SELECT * FROM Todo", &[]);
        let rows = match rows {
            Ok(r) => r,
            Err(e)  => {
                println!("Error execute query {}", e);
                return result
            }
        };

        for row in rows.iter() {
            result.push(Todo {
                id: row.get(0),
                text: row.get(1),
                is_done: row.get(2)
            });
        }

        result
    }

    fn insert(&self, todo: &mut Todo) {
        let conn =  self.create_connection();
        match conn.query("INSERT INTO Todo (text, is_done) VALUES ($1, $2) RETURNING id;",
                        &[&todo.text, &todo.is_done]) {
            Err(e) => println!("Error to insert {}", e),
            Ok(id) => {
                todo.id = id.get(0).get(0);
            }
        };
    }

    fn update(&self, todo: &Todo) {
        let conn = self.create_connection();

        match conn.execute("UPDATE Todo SET text=$1, is_done=$2 WHERE id=$3",
                           &[&todo.text, &todo.is_done, &todo.id]) {
            Err(e) => println!("Error to insert {}", e),
            Ok(_) => {}
        };
    }

    fn delete(&self, todo: &Todo) {
        let conn =  self.create_connection();
        match conn.execute("DELETE FROM Todo WHERE id=$1",
                           &[&todo.id]) {
            Err(e) => println!("Error to insert {}", e),
            Ok(_) => {}
        };
    }
}