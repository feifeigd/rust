use actix_web::{get, web, Result};

#[get("/users/{user_id}/{friend}")]
pub async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}