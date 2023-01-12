use actix_web::{web, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;

pub mod user;
use user::services::user_service_config;

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
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                is_debug: _is_debug
            }))
            .service(web::scope("/user").configure(user_service_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}