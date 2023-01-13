use actix_web::{web, App, HttpServer };
use actix_web_httpauth::{middleware::HttpAuthentication };
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenvy::dotenv;

pub mod user;
use user::user_service_config;

pub mod post;
use post::post_service_config;

pub mod auth;
use auth::auth::jwt_validate;

pub struct AppState {
    db: Pool<Postgres>,
    is_debug: bool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _is_debug = std::env::var("DEBUG").unwrap_or(String::from("false")).parse() == Ok(true);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error connecting to database.");

    HttpServer::new(move || {
        let auth_validator = HttpAuthentication::bearer(jwt_validate);

        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                is_debug: _is_debug
            }))
            .service(web::scope("/user").configure(user_service_config))
            .service(web::scope("/post").wrap(auth_validator).configure(post_service_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}