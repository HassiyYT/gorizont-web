use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, Validation};
use futures::future::{ok, Ready};

#[derive(Debug)]
struct AuthError;

impl actix_web::ResponseError for AuthError {}

async fn jwt_middleware(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let token = credentials.token();
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    );

    match decoded {
        Ok(token_data) => {
            let user_id = token_data.claims.sub;
            // Fetch user from the database
            let user = get_user_by_id(user_id);
            req.extensions_mut().insert(user);
            Ok(req)
        }
        Err(_) => Err(AuthError.into()),
    }
}

fn get_user_by_id(user_id: Uuid) -> User {
    // Fetch user from the database using user_id
}
