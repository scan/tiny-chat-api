use chrono::{prelude::*, Duration};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand::{self, distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::{env, ops::Add};

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    iat: i64,
    exp: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manager {
    secret: String,
}

impl Manager {
    pub fn new() -> Self {
        let secret: String = env::var("AUTH_SECRET_BASE").unwrap_or_else(|_| {
            log::warn!(
                "AUTH_SECRET_BASE not set, using a random string. This will break after a restart!"
            );

            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(16)
                .collect::<String>()
        });

        Manager { secret }
    }

    pub fn token_for_user(&self, user: &str) -> anyhow::Result<String> {
        let claims = Claims {
            sub: user.to_owned(),
            iss: "tiny-chat".to_owned(),
            iat: Utc::now().timestamp(),
            exp: Utc::now().add(Duration::days(360)).timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?;

        return Ok(token);
    }

    pub fn username_from_token(&self, token: &str) -> anyhow::Result<String> {
        let token_contents = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;

        return Ok(token_contents.claims.sub);
    }
}
