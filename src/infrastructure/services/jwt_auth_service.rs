use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation, errors::Error};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::application::exceptions::AppError;

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

#[derive(Serialize)]
pub struct Tokens {
    #[serde(rename(serialize = "accessToken"))]
    pub access_token: String,
    #[serde(rename(serialize = "refreshToken"))]
    pub refresh_token: String,
}

pub struct AuthorizationHeader<'life> {
    pub token_type: &'life str,
    pub token: &'life str,
}

// #[allow(dead_code, unused_variables)]
impl JwtAuthService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    fn generate_token(&self, claims: JwtClaims) -> Result<String, Error> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub fn generate(&self, user_id: i32) -> Result<Tokens, Error> {
        let access_token = self.generate_access_token(user_id)?;
        let refresh_token = self.generate_refresh_token(user_id)?;

        Ok(Tokens {
            access_token,
            refresh_token,
        })
    }

    pub fn generate_access_token(&self, user_id: i32) -> Result<String, Error> {
        let claims = JwtClaims {
            sub: user_id,
            exp: (OffsetDateTime::now_utc() + Duration::minutes(15)).unix_timestamp(),
            typ: String::from("access"),
        };

        let token = self.generate_token(claims)?;
        Ok(token)
    }

    pub fn generate_refresh_token(&self, user_id: i32) -> Result<String, Error> {
        let claims = JwtClaims {
            sub: user_id,
            exp: (OffsetDateTime::now_utc() + Duration::days(7)).unix_timestamp(),
            typ: String::from("refresh"),
        };

        let token = self.generate_token(claims)?;
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

    pub fn validate_from_authorization<'r>(
        &'r self,
        token: &'r str,
    ) -> Result<AuthorizationHeader<'r>, AppError> {
        let parts: Vec<&str> = token.split(" ").collect();

        let token_type = parts[0];
        if !token_type.eq("Bearer") || token_type.is_empty() {
            return Err(AppError::Unexpected(format!(
                "Invalid authorization type in header"
            )));
        }

        let token = parts[1];

        Ok(AuthorizationHeader {
            token_type: token_type,
            token: token,
        })
    }

    pub fn refresh_access_token(&self, refresh_token: &str) -> Result<String, String> {
        let claims = self.validate_token(refresh_token)?;

        if claims.typ != "refresh" {
            return Err("Invalid token type".into());
        }

        self.generate_access_token(claims.sub)
            .map_err(|e| e.to_string())
    }
}
