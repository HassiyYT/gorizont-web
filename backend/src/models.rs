use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
