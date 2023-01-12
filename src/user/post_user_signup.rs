use actix_web::{ post, web::{Json, self}, Responder, HttpResponse };
use argon2::{ password_hash::{ rand_core::OsRng, PasswordHasher, SaltString }, Argon2 };
use serde::Deserialize;

use crate::{AppState, user::sql_user::User};

#[derive(Deserialize)]
pub struct SignupBody { 
    pub username: String,
    pub password: String
}

#[post("/signup")]
pub async fn signup(state: web::Data<AppState>, body: Json<SignupBody>) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(body.password.as_bytes(), &salt).expect("Oops").to_string();

    match sqlx::query_as!(
        User,
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING username, password_hash",
        body.username.to_string(),
        password_hash.to_string()
    )
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create user"),
    }
}