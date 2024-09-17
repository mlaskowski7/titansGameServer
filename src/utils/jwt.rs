use std::env;
use actix_web::HttpRequest;
use chrono::Utc;
use dotenvy::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct JwtClaims {
    pub sub: String,
    pub exp: usize,
}

impl JwtClaims {
    pub fn new(sub: String, exp: usize) -> Self {
        JwtClaims {
            sub,
            exp
        }
    }
}
pub fn generate_jwt_token(user_id: i64) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .unwrap()
        .timestamp() as usize;

    let claims = JwtClaims::new(user_id.to_string(), expiration);

    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref())).expect("Failed to create JWT token")
}

pub fn extract_jwt_token(req: &HttpRequest) -> Option<String> {
    req.headers().get("Authorization").and_then(|header| header.to_str().ok()).map(|token| token.trim_start_matches("Bearer ").to_string())
}

pub fn validate_jwt_token(token: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoded = decode::<JwtClaims>(
        token, &DecodingKey::from_secret(jwt_secret.as_ref()), &Validation::default()
    )?;

    Ok(decoded.claims)
}