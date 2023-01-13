use actix_web::{ web, Responder, HttpResponse, delete };

use crate::{AppState, auth::auth::Authenticated};

#[delete("/{id}")]
pub async fn delete(id_path: web::Path<i32>, auth: Authenticated, state: web::Data<AppState>) -> impl Responder {
    let id = id_path.into_inner();
    match sqlx::query!("DELETE FROM text_posts WHERE id = $1 AND username = $2", id, auth.sub)
        .execute(&state.db)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().json("TextPost successfully deleted")
            } else {
                HttpResponse::Ok().json("Failed to delete TextPost")
            }
        },
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete TextPost"),   
    }
}