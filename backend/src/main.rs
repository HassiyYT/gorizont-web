use actix_web::{web, App, HttpServer, HttpResponse};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema;
mod models;

use models::{User, Post};

#[derive(Deserialize)]
struct RegisterUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct CreatePost {
    title: String,
    body: String,
}

async fn register_user(register: web::Json<RegisterUser>) -> HttpResponse {
    // Implement user registration logic (hash password, save to DB)
    HttpResponse::Ok().json("User registered")
}

async fn login_user(login: web::Json<LoginUser>) -> HttpResponse {
    // Implement user login logic (validate password, generate JWT)
    HttpResponse::Ok().json("User logged in")
}

async fn create_post(post: web::Json<CreatePost>) -> HttpResponse {
    // Implement post creation logic
    HttpResponse::Ok().json("Post created")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
            .route("/posts", web::post().to(create_post))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
