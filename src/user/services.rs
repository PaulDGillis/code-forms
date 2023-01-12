use actix_web::{ get, post, web::{Json, self}, Responder, HttpResponse };
use argon2::{ password_hash::{ rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString }, Argon2 };

use crate::{AppState, user::models::{User, LoginBody, SignupBody}};

pub fn user_service_config(cfg: &mut web::ServiceConfig) {    
    cfg
        .service(list_users)
        .service(signup)
        .service(login);
}

#[get("/list")]
pub async fn list_users(state: web::Data<AppState>) -> impl Responder {
    if state.is_debug {
        match sqlx::query_as::<_, User>("SELECT username, password_hash FROM users")
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

#[post("/signup")]
pub async fn signup(state: web::Data<AppState>, body: Json<SignupBody>) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(body.password.as_bytes(), &salt).expect("Oops").to_string();

    match sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING username, password_hash"
    )
        .bind(body.username.to_string())
        .bind(password_hash.to_string())
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create user"),
    }
}

#[post("/login")]
pub async fn login(state: web::Data<AppState>, body: Json<LoginBody>) -> impl Responder {
    match sqlx::query_as::<_, User>(
        "SELECT username, password_hash FROM users WHERE username = $1"
    )
        .bind(body.username.to_string())
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