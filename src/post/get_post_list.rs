use actix_web::{ get, web, Responder, HttpResponse };

use crate::{AppState, post::sql_text_post::TextPost};

#[get("/list")]
pub async fn list_posts(state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as!(TextPost, "SELECT id, title, content, username FROM text_posts")
        .fetch_all(&state.db)
        .await
    {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::NotFound().json("No posts found."),
    }
}