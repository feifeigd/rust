use actix_web::{App, get, HttpResponse, HttpServer, Responder, };

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Actix Web")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let server = HttpServer::new(|| App::new().service(hello));
    server.bind(("127.0.0.1", 8080))?.run().await
}
