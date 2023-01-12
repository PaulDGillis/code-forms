use actix_web::{ post, web::{Json, self}, Responder, HttpResponse };
use argon2::{ password_hash::{ PasswordHash, PasswordVerifier }, Argon2 };
use serde::Deserialize;

use crate::{AppState, user::sql_user::User};

#[derive(Deserialize)]
pub struct LoginBody { 
    pub username: String,
    pub password: String
}

#[post("/login")]
pub async fn login(state: web::Data<AppState>, body: Json<LoginBody>) -> impl Responder {
    match sqlx::query_as!(
        User,
        "SELECT username, password_hash FROM users WHERE username = $1",
        body.username.to_string()
    )
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => {
            // User exists
            let parsed_hash = PasswordHash::new(&user.password_hash).expect("Couldn't create password_hash from user db.");
            let assertion = Argon2::default().verify_password(body.password.as_bytes(), &parsed_hash);
            match assertion {
                Ok(_) => HttpResponse::Ok().json(format!("user {} ", user.username)),
                Err(_) => HttpResponse::InternalServerError().json("Failed to validate user.")
            }
        },
        Err(_) => HttpResponse::InternalServerError().json("Failed to validate user."),
    }
}