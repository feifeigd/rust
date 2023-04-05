use actix_web::{get, Responder, web};

use crate::state::State;

#[get("/")]
pub async fn index() -> impl Responder {
    "Welcome to Linksnap API server"
}

#[get("/links")]
pub async fn links(data: web::Data<State>) -> impl Responder {
    "abc"
}