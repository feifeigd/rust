mod links;

mod route_handlers;

mod state;
use crate::state::State;

use actix_web::{App, HttpServer};
use route_handlers::index;

use std::env;

use log::info;

fn init_env() {
    env::set_var("RUST_LOG", "linksnap=info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    info!("Starting http server: 127.0.0.1:8080");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_env();

    let server = HttpServer::new(|| App::new().app_data(State::init()).service(index));
    server.bind(("127.0.0.1", 8080))?.run().await
}
