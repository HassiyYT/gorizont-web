use actix_web::{web, App, HttpServer, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::middleware::auth_middleware::jwt_middleware;
use crate::handlers::auth;
use crate::handlers::posts;
use crate::schema::users;
use crate::schema::posts;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod handlers;
mod middleware;
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
            //.wrap(HttpAuthentication::bearer(jwt_middleware))
            .wrap(actix_web::HttpAuthentication::bearer(middleware::auth_middleware::jwt_middleware))
            .route("/register", web::post().to(handlers::auth::register_user))
            .route("/login", web::post().to(handlers::auth::login_user))
            .route("/posts", web::post().to(handlers::posts::create_post))
            .route("/posts", web::get().to(handlers::posts::get_posts))
            .route("/posts/{id}", web::put().to(handlers::posts::update_post))
            .route("/posts/{id}", web::delete().to(handlers::posts::delete_post))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
