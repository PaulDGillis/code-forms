use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct TextPost {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub username: String, // Poster
}