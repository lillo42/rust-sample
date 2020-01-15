use actix_web::{HttpServer, App};

mod entities;
mod repositories;
mod todo_api;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("starting webAPi");

    let bind =HttpServer::new(|| {
        App::new()
            .service(todo_api::get_all)
            .service(todo_api::add)
    })
        .bind("127.0.0.1:5000");

    match bind {
        Ok(b) => {
            println!("Bind with success");
            b.run()
                .await
        }
        Err(e) => {
            println!("Error to bind port {}", e);
            panic!(e);
        }
    }
}
