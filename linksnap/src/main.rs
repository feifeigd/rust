mod links;

mod route_handlers;

mod state;
use std::env;

use crate::state::State;

use actix_web::{dev::Service as _, middleware::Logger, App, HttpServer, web};
use futures_util::future::FutureExt;

// use env_logger::Env;
use route_handlers::index;

use log::info;

fn init_env() {
    env::set_var("RUST_LOG", "linksnap=info");
    env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init_from_env(Env::default().default_filter_or("info"));

    env_logger::init();
    info!("Starting http server: 127.0.0.1:8080");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_env();

    let state = State::init();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()) )
            // 注册中间件
            .wrap(Logger::default())
            .wrap_fn(|req, srv| {
                println!("Hi from start. You requested: {}", req.path());
                srv.call(req).map(|res| {
                    println!("Hi from response");
                    res
                })
            })
            .service(index)
    });
    server.bind(("127.0.0.1", 8080))?.run().await
}
