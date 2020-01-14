use actix_web::{Responder, HttpResponse, HttpServer, App, web, get};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello Word")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello 2")
}

#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hello there")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(index3)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
