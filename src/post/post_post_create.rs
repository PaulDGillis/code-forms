use actix_web::{ post, web::{Json, self}, Responder, HttpResponse };
use serde::Deserialize;
use std::ops::Deref;

use crate::{ AppState, post::sql_text_post::TextPost, auth::auth::Authenticated };

#[derive(Deserialize)]
pub struct CreateTextPostBody {
    pub title: String,
    pub content: String
}

#[post("/create")]
pub async fn create(state: web::Data<AppState>, auth: Authenticated, body: Json<CreateTextPostBody>) -> impl Responder {
    match sqlx::query_as!(TextPost, 
        "INSERT INTO text_posts (title, content, username) VALUES ($1, $2, $3) RETURNING id, title, content, username",
        body.title.to_string(),
        body.content.to_string(),
        auth.sub
    )
        .fetch_one(&state.db)
        .await
    {
        Ok(text_post) => HttpResponse::Ok().json(text_post),
        Err(error) => {
            match error.as_database_error() {
                None => HttpResponse::InternalServerError().json("Unable to create post."),
                Some(db_error) => {
                    let error_message = db_error.deref().constraint().unwrap_or("");
                    if "text_posts_username_fkey" == error_message && state.is_debug {
                        HttpResponse::InternalServerError().json("Unable to create post. User doesn't exist")
                    } else {
                        HttpResponse::InternalServerError().json("Unable to create post.")
                    }
                }
            }
        }
    }
}
