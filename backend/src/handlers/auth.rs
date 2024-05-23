use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use uuid::Uuid;
use chrono::Utc;
use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::schema;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

pub async fn register_user(register: web::Json<RegisterUser>, pool: web::Data<Pool>) -> HttpResponse {
    use schema::users::dsl::*;

    let conn = pool.get().expect("Couldn't get db connection from pool");

    let hashed_password = hash(&register.password, DEFAULT_COST).expect("Failed to hash password");

    let new_user = NewUser {
        id: Uuid::new_v4(),
        username: &register.username,
        email: &register.email,
        password: &hashed_password,
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&conn)
        .expect("Error saving new user");

    HttpResponse::Ok().json("User registered")
}

pub async fn login_user(login: web::Json<LoginUser>, pool: web::Data<Pool>) -> HttpResponse {
    use schema::users::dsl::*;

    let conn = pool.get().expect("Couldn't get db connection from pool");

    let user = users
        .filter(email.eq(&login.email))
        .first::<User>(&conn)
        .optional()
        .expect("Error loading user");

    if let Some(user) = user {
        if verify(&login.password, &user.password).expect("Failed to verify password") {
            let expiration = Utc::now()
                .checked_add_signed(chrono::Duration::minutes(60))
                .expect("valid timestamp")
                .timestamp();

            let claims = Claims {
                sub: user.id,
                exp: expiration as usize,
            };

            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
                .expect("Failed to generate token");

            return HttpResponse::Ok().json(token);
        }
    }

    HttpResponse::Unauthorized().json("Invalid credentials")
}
