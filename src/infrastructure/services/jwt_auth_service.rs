use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: i32,
    pub exp: i64,
    pub typ: String,
}

#[derive(Default, Debug)]
pub struct JwtAuthService {
    secret: String,
}

impl JwtAuthService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    fn generate_token(
        &self,
        user_id: i32,
        claims: JwtClaims,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub fn generate_access_token(
        &self,
        user_id: i32,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = JwtClaims {
            sub: user_id,
            exp: (OffsetDateTime::now_utc() + Duration::minutes(15)).unix_timestamp(),
            typ: String::from("access"),
        };

        let token = self.generate_token(user_id, claims)?;

        Ok(token)
    }

    pub fn generate_refresh_token(
        &self,
        user_id: i32,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = JwtClaims {
            sub: user_id,
            exp: (OffsetDateTime::now_utc() + Duration::days(7)).unix_timestamp(),
            typ: String::from("refresh"),
        };

        let token = self.generate_token(user_id, claims)?;

        Ok(token)
    }

    pub fn validate_token(&self, token: &str) -> Result<JwtClaims, String> {
        jsonwebtoken::decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map(|data| data.claims)
        .map_err(|e| e.to_string())
    }

    pub fn refresh_access_token(&self, refresh_token: &str) -> Result<String, String> {
        let claims = self.validate_token(refresh_token)?;

        if claims.typ != "refresh" {
            return Err("Invalid type".into());
        }

        self.generate_access_token(claims.sub)
            .map_err(|e| e.to_string())
    }
}
