use actix_web::{ web, Responder, HttpResponse, delete };

use crate::AppState;

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