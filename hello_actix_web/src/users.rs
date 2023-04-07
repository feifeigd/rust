use actix_web::{get, web, Result, Scope};

// http://localhost:8080/users/2222/abddd
#[get("/{user_id}/{friend}")]
pub async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

// 统一前缀 /users
pub fn scope() -> Scope {
    web::scope("/users").service(index)
}
