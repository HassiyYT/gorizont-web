pub mod schema;
mod handlers;
mod middleware;

use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::middleware::auth_middleware::jwt_middleware;
use crate::handlers::auth::register_user;
use crate::handlers::auth::login_user;
use crate::handlers::posts::create_post;
use crate::handlers::posts::get_posts;
use crate::handlers::posts::update_post;
use crate::handlers::posts::delete_post;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(HttpAuthentication::bearer(jwt_middleware))
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
            .route("/posts", web::post().to(create_post))
            .route("/posts", web::get().to(get_posts))
            .route("/posts/{id}", web::put().to(update_post))
            .route("/posts/{id}", web::delete().to(delete_post))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
