mod users;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// 手动设置路由
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

/*async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello index")
}*/

// This struct represents state
struct AppState {
    app_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // /app/index.html
    // let scope = web::scope("/app").route("/index.html", web::get().to(index));

    let server = HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web 123"),
            }))
            .service(index)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(users::index)
        // .service(scope)
    });

    server.bind(("127.0.0.1", 8080))?.run().await
}
