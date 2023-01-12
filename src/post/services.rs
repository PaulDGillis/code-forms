use actix_web::{ get, post, web::{Json, self}, Responder, HttpResponse, delete };

use crate::{AppState, post::models::{TextPost, CreateTextPostBody}};

pub fn post_service_config(cfg: &mut web::ServiceConfig) {    
    cfg
        .service(list_posts)
        .service(create)
        .service(delete);
}

#[get("/list")]
pub async fn list_posts(state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, TextPost>("SELECT id, title, content, username FROM text_posts")
        .fetch_all(&state.db)
        .await
    {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::NotFound().json("No posts found."),
    }
}

#[post("/create")]
pub async fn create(state: web::Data<AppState>, body: Json<CreateTextPostBody>) -> impl Responder {
    match sqlx::query_as::<_, TextPost>(
        "INSERT INTO text_posts (title, content, username) VALUES ($1, $2, $3) RETURNING id, title, content, username"
    )
        .bind(body.title.to_string())
        .bind(body.content.to_string())
        .bind(body.username.to_string())
        .fetch_one(&state.db)
        .await
    {
        Ok(text_post) => HttpResponse::Ok().json(text_post),
        Err(_) => HttpResponse::InternalServerError().json("Unable to create post.")
    }
}

#[delete("/{id}")]
pub async fn delete(id_path: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let id = id_path.into_inner();
    match sqlx::query!("DELETE FROM text_posts WHERE id = $1", id)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("TextPost successfully deleted"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete TextPost"),   
    }
}