use actix_web::{ get, web, Responder, HttpResponse };

use crate::{AppState, user::sql_user::User};

#[get("/list")]
pub async fn list_users(state: web::Data<AppState>) -> impl Responder {
    if state.is_debug {
        match sqlx::query_as!(User, "SELECT username, password_hash FROM users")
            .fetch_all(&state.db)
            .await
        {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(_) => HttpResponse::NotFound().json("No users found."),
        }
    } else {
        HttpResponse::NotFound().json("List users is disabled in production.")
    }
}