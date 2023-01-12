use std::fmt;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    pub username: String,
    pub password_hash: String
}

#[derive(Deserialize)]
pub struct LoginBody { 
    pub username: String,
    pub password: String
}

impl fmt::Display for LoginBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.username, self.password)
    }
}

#[derive(Deserialize)]
pub struct SignupBody { 
    pub username: String,
    pub password: String
}